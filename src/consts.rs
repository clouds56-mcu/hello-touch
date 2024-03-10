use esp_idf_svc::sys::{
  esp_lcd_rgb_panel_config_t,
  esp_lcd_rgb_panel_config_t__bindgen_ty_1,
  esp_lcd_rgb_timing_t,
  esp_lcd_rgb_timing_t__bindgen_ty_1,
  soc_periph_lcd_clk_src_t_LCD_CLK_SRC_PLL160M,
};

pub fn lcd_config() -> esp_lcd_rgb_panel_config_t {
  esp_lcd_rgb_panel_config_t {
    clk_src: soc_periph_lcd_clk_src_t_LCD_CLK_SRC_PLL160M,
    timings: esp_lcd_rgb_timing_t {
      pclk_hz: 14 * 1000 * 1000,
      h_res: 800,
      v_res: 400,
      hsync_pulse_width: 10,
      hsync_back_porch: 10,
      hsync_front_porch: 20,
      vsync_pulse_width: 10,
      vsync_back_porch: 10,
      vsync_front_porch: 10,
      flags: {
        let mut value = esp_lcd_rgb_timing_t__bindgen_ty_1::default();
        value.set_pclk_active_neg(0);
        value
      },
    },
    data_width: 16,
    // bits_per_pixel: todo!(),
    // num_fbs: todo!(),
    // bounce_buffer_size_px: todo!(),
    // sram_trans_align: todo!(),
    psram_trans_align: 64,
    hsync_gpio_num: pin!(46),
    vsync_gpio_num: pin!(3),
    de_gpio_num: pin!(5),
    pclk_gpio_num: pin!(7),
    disp_gpio_num: pin!(none),
    data_gpio_nums: [
      pin!(14), // ESP_PANEL_LCD_RGB_IO_DATA0
      pin!(38), // ESP_PANEL_LCD_RGB_IO_DATA1
      pin!(18), // ESP_PANEL_LCD_RGB_IO_DATA2
      pin!(17), // ESP_PANEL_LCD_RGB_IO_DATA3
      pin!(10), // ESP_PANEL_LCD_RGB_IO_DATA4
      pin!(39), // ESP_PANEL_LCD_RGB_IO_DATA5
      pin!(0),  // ESP_PANEL_LCD_RGB_IO_DATA6
      pin!(45), // ESP_PANEL_LCD_RGB_IO_DATA7
      pin!(48), // ESP_PANEL_LCD_RGB_IO_DATA8
      pin!(47), // ESP_PANEL_LCD_RGB_IO_DATA9
      pin!(21), // ESP_PANEL_LCD_RGB_IO_DATA10
      pin!(1),  // ESP_PANEL_LCD_RGB_IO_DATA11
      pin!(2),  // ESP_PANEL_LCD_RGB_IO_DATA12
      pin!(42), // ESP_PANEL_LCD_RGB_IO_DATA13
      pin!(41), // ESP_PANEL_LCD_RGB_IO_DATA14
      pin!(40), // ESP_PANEL_LCD_RGB_IO_DATA15
    ],
    flags: {
      let mut value = esp_lcd_rgb_panel_config_t__bindgen_ty_1::default();
      value.set_fb_in_psram(1);
      value
    },
    ..esp_lcd_rgb_panel_config_t::default()
  }
}
