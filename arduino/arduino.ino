// #define LV_CONF_INCLUDE_SIMPLE 1
#include <Arduino.h>
#include <lvgl.h>
#include <ESP_Panel_Library.h>
#include <ESP_IOExpander_Library.h>

#define TP_RST 1
#define LCD_BL 2
#define LCD_RST 3
#define SD_CS 4
#define USB_SEL 5

#define I2C_MASTER_NUM 1
#define I2C_MASTER_SDA_IO 8
#define I2C_MASTER_SCL_IO 9

#define LVGL_TICK_PERIOD_MS     (2)
#define LVGL_TASK_MAX_DELAY_MS  (500)
#define LVGL_TASK_MIN_DELAY_MS  (1)
#define LVGL_TASK_STACK_SIZE    (4 * 1024)
#define LVGL_TASK_PRIORITY      (2)
#define LVGL_BUF_HEIGHT         (40)
#define LVGL_BUF_SIZE           (ESP_PANEL_LCD_H_RES * LVGL_BUF_HEIGHT)

ESP_Panel *panel = NULL;
SemaphoreHandle_t lvgl_mux = NULL; // LVGL mutex

void log_printf(lv_log_level_t level, const char * buf) {
  Serial.print(buf);
}

/* Display flushing */
void lvgl_port_disp_flush(lv_display_t *disp, const lv_area_t *area, uint8_t *color_p) {
  // Serial.print("flush display: ");
  // Serial.print(area->x1);Serial.print(" ");
  // Serial.print(area->y1);Serial.print(" ");
  // Serial.print(area->x2);Serial.print(" ");
  // Serial.print(area->y2);Serial.print(" ");
  // Serial.println();
  panel->getLcd()->drawBitmap(area->x1, area->y1, area->x2 + 1, area->y2 + 1, color_p);
  lv_display_flush_ready(disp);
}

uint32_t lvgl_port_tick_get() { return millis(); }

void lvgl_port_lock(int timeout_ms) {
  const TickType_t timeout_ticks = (timeout_ms < 0) ? portMAX_DELAY : pdMS_TO_TICKS(timeout_ms);
  xSemaphoreTakeRecursive(lvgl_mux, timeout_ticks);
}

void lvgl_port_unlock(void) {
  xSemaphoreGiveRecursive(lvgl_mux);
}

void lvgl_port_task(void *arg) {
  Serial.println("Starting LVGL task");

  uint32_t task_delay_ms = LVGL_TASK_MAX_DELAY_MS;
  while (1) {
    // Lock the mutex due to the LVGL APIs are not thread-safe
    lvgl_port_lock(-1);
    task_delay_ms = lv_timer_handler();
    // Release the mutex
    lvgl_port_unlock();
    if (task_delay_ms > LVGL_TASK_MAX_DELAY_MS) {
      task_delay_ms = LVGL_TASK_MAX_DELAY_MS;
    } else if (task_delay_ms < LVGL_TASK_MIN_DELAY_MS) {
      task_delay_ms = LVGL_TASK_MIN_DELAY_MS;
    }
    delay(task_delay_ms);
  }
}

/* Read the touchpad */
void lvgl_port_tp_read(lv_indev_t * indev, lv_indev_data_t * data) {
  panel->getLcdTouch()->readData();

  bool touched = panel->getLcdTouch()->getTouchState();
  if(!touched) {
    data->state = LV_INDEV_STATE_REL;
  } else {
    TouchPoint point = panel->getLcdTouch()->getPoint();

    data->state = LV_INDEV_STATE_PR;
    /*Set the coordinates*/
    data->point.x = point.x;
    data->point.y = point.y;

    Serial.printf("Touch point: x %d, y %d\n", point.x, point.y);
  }
}

void setup() {
  Serial.begin(115200); /* prepare for possible serial debug */

  String LVGL_Arduino = "Hello LVGL! ";
  LVGL_Arduino += String('V') + lv_version_major() + "." + lv_version_minor() + "." + lv_version_patch();

  Serial.println(LVGL_Arduino);
  Serial.println("I am ESP32_Display_Panel");
  panel = new ESP_Panel();

  assert(ESP_PANEL_LCD_BUS_TYPE == ESP_PANEL_BUS_TYPE_RGB);
  /* Initialize LVGL core */
  lv_init();
  lv_log_register_print_cb(log_printf);

  /* Initialize LVGL buffers */
  /* Using double buffers is more faster than single buffer */
  /* Using internal SRAM is more fast than PSRAM (Note: Memory allocated using `malloc` may be located in PSRAM.) */
  uint8_t *buf = (uint8_t *)heap_caps_calloc(LVGL_BUF_SIZE, sizeof(lv_color16_t), MALLOC_CAP_INTERNAL);
  assert(buf);

  static lv_draw_buf_t draw_buf;
  lv_draw_buf_init(&draw_buf, ESP_PANEL_LCD_H_RES, LVGL_BUF_HEIGHT, LV_COLOR_FORMAT_RGB565, 0, buf, LVGL_BUF_SIZE*sizeof(lv_color16_t));

  static lv_display_t* display = lv_display_create(ESP_PANEL_LCD_H_RES, ESP_PANEL_LCD_V_RES);
  assert(display);
  lv_display_set_color_format(display, LV_COLOR_FORMAT_RGB565);
  lv_display_set_flush_cb(display, lvgl_port_disp_flush);
  lv_display_set_draw_buffers(display, &draw_buf, NULL);
  // lv_display_set_buffers(display, &buf, NULL, LVGL_BUF_SIZE, LV_DISPLAY_RENDER_MODE_PARTIAL);
  lv_display_set_default(display);

  lv_tick_set_cb(lvgl_port_tick_get);
  // esp_register_freertos_tick_hook();

  /* Initialize the input device */
  static lv_indev_t* indev = lv_indev_create();
  lv_indev_set_type(indev, LV_INDEV_TYPE_POINTER);
  lv_indev_set_read_cb(indev, lvgl_port_tp_read);
  lv_indev_set_display(indev, display);

  /**
   * These development boards require the use of an IO expander to configure the screen,
   * so it needs to be initialized in advance and registered with the panel for use.
   */
  Serial.println("Initialize IO expander");
  /* Initialize IO expander */
  ESP_IOExpander *expander = new ESP_IOExpander_CH422G(I2C_MASTER_NUM, ESP_IO_EXPANDER_I2C_CH422G_ADDRESS_000, I2C_MASTER_SCL_IO, I2C_MASTER_SDA_IO);
  // ESP_IOExpander *expander = new ESP_IOExpander_CH422G(I2C_MASTER_NUM, ESP_IO_EXPANDER_I2C_CH422G_ADDRESS_000);
  expander->init();
  expander->begin();
  expander->multiPinMode(TP_RST | LCD_BL | LCD_RST | SD_CS | USB_SEL, OUTPUT);
  expander->multiDigitalWrite(TP_RST | LCD_BL | LCD_RST | SD_CS, HIGH);
  // expander->multiPinMode((1<<TP_RST) | (1<<LCD_BL) | (1<<LCD_RST) | (1<<SD_CS) | (1<<USB_SEL), OUTPUT);
  // expander->multiDigitalWrite((1<<TP_RST) | (1<<LCD_BL) | (1<<LCD_RST) | (1<<SD_CS), HIGH);
  // Turn off backlight
  // expander->digitalWrite(USB_SEL, LOW);
  expander->digitalWrite(USB_SEL, LOW);
  /* Add into panel */
  panel->addIOExpander(expander);

  /* Initialize bus and device of panel */
  panel->init();
  /* Start panel */
  panel->begin();

  /* Create a task to run the LVGL task periodically */
  lvgl_mux = xSemaphoreCreateRecursiveMutex();
  xTaskCreate(lvgl_port_task, "lvgl", LVGL_TASK_STACK_SIZE, NULL, LVGL_TASK_PRIORITY, NULL);

  Serial.println("lvgl inited");

  /* Lock the mutex due to the LVGL APIs are not thread-safe */
  lvgl_port_lock(-1);
  /* Create simple label */
  lv_obj_set_style_bg_color(lv_scr_act(), LV_COLOR_MAKE(0, 0, 255), LV_STATE_DEFAULT);
  lv_obj_t *label = lv_label_create(lv_scr_act());
  lv_label_set_text(label, LVGL_Arduino.c_str());
  lv_obj_align(label, LV_ALIGN_CENTER, 0, 0);

  // TODO: why removing this line cause panic?
  lv_refr_now(display);
  /* Release the mutex */
  lvgl_port_unlock();

  Serial.println("Setup done");
  delay(3000);
  Serial.println("Hello, world!");
}

void loop() {
  static u_long last_tick = 0;
  ulong new_tick = millis();

  if (new_tick > last_tick) {
    lvgl_port_lock(-1);
    lv_tick_inc(new_tick - last_tick);
    lvgl_port_unlock();
    last_tick = new_tick;
  }
  delay(100);
}
