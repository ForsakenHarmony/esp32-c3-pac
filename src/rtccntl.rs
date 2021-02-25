#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC_CNTL_OPTIONS0"]
    pub options0: OPTIONS0,
    #[doc = "0x04 - RTC_CNTL_SLP_TIMER0"]
    pub slp_timer0: SLP_TIMER0,
    #[doc = "0x08 - RTC_CNTL_SLP_TIMER1"]
    pub slp_timer1: SLP_TIMER1,
    #[doc = "0x0c - RTC_CNTL_TIME_UPDATE"]
    pub time_update: TIME_UPDATE,
    #[doc = "0x10 - RTC_CNTL_TIME_LOW0"]
    pub time_low0: TIME_LOW0,
    #[doc = "0x14 - RTC_CNTL_TIME_HIGH0"]
    pub time_high0: TIME_HIGH0,
    #[doc = "0x18 - RTC_CNTL_STATE0"]
    pub state0: STATE0,
    #[doc = "0x1c - RTC_CNTL_TIMER1"]
    pub timer1: TIMER1,
    #[doc = "0x20 - RTC_CNTL_TIMER2"]
    pub timer2: TIMER2,
    #[doc = "0x24 - RTC_CNTL_TIMER3"]
    pub timer3: TIMER3,
    #[doc = "0x28 - RTC_CNTL_TIMER4"]
    pub timer4: TIMER4,
    #[doc = "0x2c - RTC_CNTL_TIMER5"]
    pub timer5: TIMER5,
    #[doc = "0x30 - RTC_CNTL_TIMER6"]
    pub timer6: TIMER6,
    #[doc = "0x34 - RTC_CNTL_ANA_CONF"]
    pub ana_conf: ANA_CONF,
    #[doc = "0x38 - RTC_CNTL_RESET_STATE"]
    pub reset_state: RESET_STATE,
    #[doc = "0x3c - RTC_CNTL_WAKEUP_STATE"]
    pub wakeup_state: WAKEUP_STATE,
    #[doc = "0x40 - RTC_CNTL_INT_ENA"]
    pub int_ena: INT_ENA,
    #[doc = "0x44 - RTC_CNTL_INT_RAW"]
    pub int_raw: INT_RAW,
    #[doc = "0x48 - RTC_CNTL_INT_ST"]
    pub int_st: INT_ST,
    #[doc = "0x4c - RTC_CNTL_INT_CLR"]
    pub int_clr: INT_CLR,
    #[doc = "0x50 - RTC_CNTL_STORE0"]
    pub store0: STORE0,
    #[doc = "0x54 - RTC_CNTL_STORE1"]
    pub store1: STORE1,
    #[doc = "0x58 - RTC_CNTL_STORE2"]
    pub store2: STORE2,
    #[doc = "0x5c - RTC_CNTL_STORE3"]
    pub store3: STORE3,
    #[doc = "0x60 - RTC_CNTL_EXT_XTL_CONF"]
    pub ext_xtl_conf: EXT_XTL_CONF,
    #[doc = "0x64 - RTC_CNTL_EXT_WAKEUP_CONF"]
    pub ext_wakeup_conf: EXT_WAKEUP_CONF,
    #[doc = "0x68 - RTC_CNTL_SLP_REJECT_CONF"]
    pub slp_reject_conf: SLP_REJECT_CONF,
    #[doc = "0x6c - RTC_CNTL_CPU_PERIOD_CONF"]
    pub cpu_period_conf: CPU_PERIOD_CONF,
    #[doc = "0x70 - RTC_CNTL_CLK_CONF"]
    pub clk_conf: CLK_CONF,
    #[doc = "0x74 - RTC_CNTL_SLOW_CLK_CONF"]
    pub slow_clk_conf: SLOW_CLK_CONF,
    #[doc = "0x78 - RTC_CNTL_SDIO_CONF"]
    pub sdio_conf: SDIO_CONF,
    #[doc = "0x7c - RTC_CNTL_BIAS_CONF"]
    pub bias_conf: BIAS_CONF,
    #[doc = "0x80 - RTC_CNTL"]
    pub rtc_cntl: RTC_CNTL,
    #[doc = "0x84 - RTC_CNTL_PWC"]
    pub pwc: PWC,
    #[doc = "0x88 - RTC_CNTL_DIG_PWC"]
    pub dig_pwc: DIG_PWC,
    #[doc = "0x8c - RTC_CNTL_DIG_ISO"]
    pub dig_iso: DIG_ISO,
    #[doc = "0x90 - RTC_CNTL_WDTCONFIG0"]
    pub wdtconfig0: WDTCONFIG0,
    #[doc = "0x94 - RTC_CNTL_WDTCONFIG1"]
    pub wdtconfig1: WDTCONFIG1,
    #[doc = "0x98 - RTC_CNTL_WDTCONFIG2"]
    pub wdtconfig2: WDTCONFIG2,
    #[doc = "0x9c - RTC_CNTL_WDTCONFIG3"]
    pub wdtconfig3: WDTCONFIG3,
    #[doc = "0xa0 - RTC_CNTL_WDTCONFIG4"]
    pub wdtconfig4: WDTCONFIG4,
    #[doc = "0xa4 - RTC_CNTL_WDTFEED"]
    pub wdtfeed: WDTFEED,
    #[doc = "0xa8 - RTC_CNTL_WDTWPROTECT"]
    pub wdtwprotect: WDTWPROTECT,
    #[doc = "0xac - RTC_CNTL_SWD_CONF"]
    pub swd_conf: SWD_CONF,
    #[doc = "0xb0 - RTC_CNTL_SWD_WPROTECT"]
    pub swd_wprotect: SWD_WPROTECT,
    #[doc = "0xb4 - RTC_CNTL_SW_CPU_STALL"]
    pub sw_cpu_stall: SW_CPU_STALL,
    #[doc = "0xb8 - RTC_CNTL_STORE4"]
    pub store4: STORE4,
    #[doc = "0xbc - RTC_CNTL_STORE5"]
    pub store5: STORE5,
    #[doc = "0xc0 - RTC_CNTL_STORE6"]
    pub store6: STORE6,
    #[doc = "0xc4 - RTC_CNTL_STORE7"]
    pub store7: STORE7,
    #[doc = "0xc8 - RTC_CNTL_LOW_POWER_ST"]
    pub low_power_st: LOW_POWER_ST,
    #[doc = "0xcc - RTC_CNTL_DIAG0"]
    pub diag0: DIAG0,
    #[doc = "0xd0 - RTC_CNTL_PAD_HOLD"]
    pub pad_hold: PAD_HOLD,
    #[doc = "0xd4 - RTC_CNTL_DIG_PAD_HOLD"]
    pub dig_pad_hold: DIG_PAD_HOLD,
    #[doc = "0xd8 - RTC_CNTL_BROWN_OUT"]
    pub brown_out: BROWN_OUT,
    #[doc = "0xdc - RTC_CNTL_TIME_LOW1"]
    pub time_low1: TIME_LOW1,
    #[doc = "0xe0 - RTC_CNTL_TIME_HIGH1"]
    pub time_high1: TIME_HIGH1,
    #[doc = "0xe4 - RTC_CNTL_XTAL32K_CLK_FACTOR"]
    pub xtal32k_clk_factor: XTAL32K_CLK_FACTOR,
    #[doc = "0xe8 - RTC_CNTL_XTAL32K_CONF"]
    pub xtal32k_conf: XTAL32K_CONF,
    #[doc = "0xec - RTC_CNTL_USB_CONF"]
    pub usb_conf: USB_CONF,
    #[doc = "0xf0 - RTC_CNTL_SLP_REJECT_CAUSE"]
    pub slp_reject_cause: SLP_REJECT_CAUSE,
    #[doc = "0xf4 - RTC_CNTL_OPTION1"]
    pub option1: OPTION1,
    #[doc = "0xf8 - RTC_CNTL_SLP_WAKEUP_CAUSE"]
    pub slp_wakeup_cause: SLP_WAKEUP_CAUSE,
    #[doc = "0xfc - RTC_CNTL_ULP_CP_TIMER_1"]
    pub ulp_cp_timer_1: ULP_CP_TIMER_1,
    #[doc = "0x100 - RTC_CNTL_INT_ENA_W1TS"]
    pub int_ena_w1ts: INT_ENA_W1TS,
    #[doc = "0x104 - RTC_CNTL_INT_ENA_W1TC"]
    pub int_ena_w1tc: INT_ENA_W1TC,
    #[doc = "0x108 - RTC_CNTL_RETENTION_CTRL"]
    pub retention_ctrl: RETENTION_CTRL,
    #[doc = "0x10c - RTC_CNTL_FIB_SEL"]
    pub fib_sel: FIB_SEL,
    #[doc = "0x110 - RTC_CNTL_GPIO_WAKEUP"]
    pub gpio_wakeup: GPIO_WAKEUP,
    #[doc = "0x114 - RTC_CNTL_DBG_SEL"]
    pub dbg_sel: DBG_SEL,
    #[doc = "0x118 - RTC_CNTL_DBG_MAP"]
    pub dbg_map: DBG_MAP,
    #[doc = "0x11c - RTC_CNTL_SENSOR_CTRL"]
    pub sensor_ctrl: SENSOR_CTRL,
    #[doc = "0x120 - RTC_CNTL_DBG_SAR_SEL"]
    pub dbg_sar_sel: DBG_SAR_SEL,
    #[doc = "0x124 - RTC_CNTL_PG_CTRL"]
    pub pg_ctrl: PG_CTRL,
    _reserved74: [u8; 212usize],
    #[doc = "0x1fc - RTC_CNTL_DATE"]
    pub date: DATE,
}
#[doc = "RTC_CNTL_OPTIONS0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [options0](options0) module"]
pub type OPTIONS0 = crate::Reg<u32, _OPTIONS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTIONS0;
#[doc = "`read()` method returns [options0::R](options0::R) reader structure"]
impl crate::Readable for OPTIONS0 {}
#[doc = "`write(|w| ..)` method takes [options0::W](options0::W) writer structure"]
impl crate::Writable for OPTIONS0 {}
#[doc = "RTC_CNTL_OPTIONS0"]
pub mod options0;
#[doc = "RTC_CNTL_SLP_TIMER0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_timer0](slp_timer0) module"]
pub type SLP_TIMER0 = crate::Reg<u32, _SLP_TIMER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLP_TIMER0;
#[doc = "`read()` method returns [slp_timer0::R](slp_timer0::R) reader structure"]
impl crate::Readable for SLP_TIMER0 {}
#[doc = "`write(|w| ..)` method takes [slp_timer0::W](slp_timer0::W) writer structure"]
impl crate::Writable for SLP_TIMER0 {}
#[doc = "RTC_CNTL_SLP_TIMER0"]
pub mod slp_timer0;
#[doc = "RTC_CNTL_SLP_TIMER1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_timer1](slp_timer1) module"]
pub type SLP_TIMER1 = crate::Reg<u32, _SLP_TIMER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLP_TIMER1;
#[doc = "`read()` method returns [slp_timer1::R](slp_timer1::R) reader structure"]
impl crate::Readable for SLP_TIMER1 {}
#[doc = "`write(|w| ..)` method takes [slp_timer1::W](slp_timer1::W) writer structure"]
impl crate::Writable for SLP_TIMER1 {}
#[doc = "RTC_CNTL_SLP_TIMER1"]
pub mod slp_timer1;
#[doc = "RTC_CNTL_TIME_UPDATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time_update](time_update) module"]
pub type TIME_UPDATE = crate::Reg<u32, _TIME_UPDATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIME_UPDATE;
#[doc = "`read()` method returns [time_update::R](time_update::R) reader structure"]
impl crate::Readable for TIME_UPDATE {}
#[doc = "`write(|w| ..)` method takes [time_update::W](time_update::W) writer structure"]
impl crate::Writable for TIME_UPDATE {}
#[doc = "RTC_CNTL_TIME_UPDATE"]
pub mod time_update;
#[doc = "RTC_CNTL_TIME_LOW0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time_low0](time_low0) module"]
pub type TIME_LOW0 = crate::Reg<u32, _TIME_LOW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIME_LOW0;
#[doc = "`read()` method returns [time_low0::R](time_low0::R) reader structure"]
impl crate::Readable for TIME_LOW0 {}
#[doc = "RTC_CNTL_TIME_LOW0"]
pub mod time_low0;
#[doc = "RTC_CNTL_TIME_HIGH0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time_high0](time_high0) module"]
pub type TIME_HIGH0 = crate::Reg<u32, _TIME_HIGH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIME_HIGH0;
#[doc = "`read()` method returns [time_high0::R](time_high0::R) reader structure"]
impl crate::Readable for TIME_HIGH0 {}
#[doc = "RTC_CNTL_TIME_HIGH0"]
pub mod time_high0;
#[doc = "RTC_CNTL_STATE0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state0](state0) module"]
pub type STATE0 = crate::Reg<u32, _STATE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATE0;
#[doc = "`read()` method returns [state0::R](state0::R) reader structure"]
impl crate::Readable for STATE0 {}
#[doc = "`write(|w| ..)` method takes [state0::W](state0::W) writer structure"]
impl crate::Writable for STATE0 {}
#[doc = "RTC_CNTL_STATE0"]
pub mod state0;
#[doc = "RTC_CNTL_TIMER1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1](timer1) module"]
pub type TIMER1 = crate::Reg<u32, _TIMER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER1;
#[doc = "`read()` method returns [timer1::R](timer1::R) reader structure"]
impl crate::Readable for TIMER1 {}
#[doc = "`write(|w| ..)` method takes [timer1::W](timer1::W) writer structure"]
impl crate::Writable for TIMER1 {}
#[doc = "RTC_CNTL_TIMER1"]
pub mod timer1;
#[doc = "RTC_CNTL_TIMER2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer2](timer2) module"]
pub type TIMER2 = crate::Reg<u32, _TIMER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER2;
#[doc = "`read()` method returns [timer2::R](timer2::R) reader structure"]
impl crate::Readable for TIMER2 {}
#[doc = "`write(|w| ..)` method takes [timer2::W](timer2::W) writer structure"]
impl crate::Writable for TIMER2 {}
#[doc = "RTC_CNTL_TIMER2"]
pub mod timer2;
#[doc = "RTC_CNTL_TIMER3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer3](timer3) module"]
pub type TIMER3 = crate::Reg<u32, _TIMER3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER3;
#[doc = "`read()` method returns [timer3::R](timer3::R) reader structure"]
impl crate::Readable for TIMER3 {}
#[doc = "`write(|w| ..)` method takes [timer3::W](timer3::W) writer structure"]
impl crate::Writable for TIMER3 {}
#[doc = "RTC_CNTL_TIMER3"]
pub mod timer3;
#[doc = "RTC_CNTL_TIMER4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer4](timer4) module"]
pub type TIMER4 = crate::Reg<u32, _TIMER4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER4;
#[doc = "`read()` method returns [timer4::R](timer4::R) reader structure"]
impl crate::Readable for TIMER4 {}
#[doc = "`write(|w| ..)` method takes [timer4::W](timer4::W) writer structure"]
impl crate::Writable for TIMER4 {}
#[doc = "RTC_CNTL_TIMER4"]
pub mod timer4;
#[doc = "RTC_CNTL_TIMER5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer5](timer5) module"]
pub type TIMER5 = crate::Reg<u32, _TIMER5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER5;
#[doc = "`read()` method returns [timer5::R](timer5::R) reader structure"]
impl crate::Readable for TIMER5 {}
#[doc = "`write(|w| ..)` method takes [timer5::W](timer5::W) writer structure"]
impl crate::Writable for TIMER5 {}
#[doc = "RTC_CNTL_TIMER5"]
pub mod timer5;
#[doc = "RTC_CNTL_TIMER6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer6](timer6) module"]
pub type TIMER6 = crate::Reg<u32, _TIMER6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER6;
#[doc = "`read()` method returns [timer6::R](timer6::R) reader structure"]
impl crate::Readable for TIMER6 {}
#[doc = "`write(|w| ..)` method takes [timer6::W](timer6::W) writer structure"]
impl crate::Writable for TIMER6 {}
#[doc = "RTC_CNTL_TIMER6"]
pub mod timer6;
#[doc = "RTC_CNTL_ANA_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_conf](ana_conf) module"]
pub type ANA_CONF = crate::Reg<u32, _ANA_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANA_CONF;
#[doc = "`read()` method returns [ana_conf::R](ana_conf::R) reader structure"]
impl crate::Readable for ANA_CONF {}
#[doc = "`write(|w| ..)` method takes [ana_conf::W](ana_conf::W) writer structure"]
impl crate::Writable for ANA_CONF {}
#[doc = "RTC_CNTL_ANA_CONF"]
pub mod ana_conf;
#[doc = "RTC_CNTL_RESET_STATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_state](reset_state) module"]
pub type RESET_STATE = crate::Reg<u32, _RESET_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESET_STATE;
#[doc = "`read()` method returns [reset_state::R](reset_state::R) reader structure"]
impl crate::Readable for RESET_STATE {}
#[doc = "`write(|w| ..)` method takes [reset_state::W](reset_state::W) writer structure"]
impl crate::Writable for RESET_STATE {}
#[doc = "RTC_CNTL_RESET_STATE"]
pub mod reset_state;
#[doc = "RTC_CNTL_WAKEUP_STATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakeup_state](wakeup_state) module"]
pub type WAKEUP_STATE = crate::Reg<u32, _WAKEUP_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKEUP_STATE;
#[doc = "`read()` method returns [wakeup_state::R](wakeup_state::R) reader structure"]
impl crate::Readable for WAKEUP_STATE {}
#[doc = "`write(|w| ..)` method takes [wakeup_state::W](wakeup_state::W) writer structure"]
impl crate::Writable for WAKEUP_STATE {}
#[doc = "RTC_CNTL_WAKEUP_STATE"]
pub mod wakeup_state;
#[doc = "RTC_CNTL_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](int_ena) module"]
pub type INT_ENA = crate::Reg<u32, _INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ENA;
#[doc = "`read()` method returns [int_ena::R](int_ena::R) reader structure"]
impl crate::Readable for INT_ENA {}
#[doc = "`write(|w| ..)` method takes [int_ena::W](int_ena::W) writer structure"]
impl crate::Writable for INT_ENA {}
#[doc = "RTC_CNTL_INT_ENA"]
pub mod int_ena;
#[doc = "RTC_CNTL_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](int_raw) module"]
pub type INT_RAW = crate::Reg<u32, _INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_RAW;
#[doc = "`read()` method returns [int_raw::R](int_raw::R) reader structure"]
impl crate::Readable for INT_RAW {}
#[doc = "RTC_CNTL_INT_RAW"]
pub mod int_raw;
#[doc = "RTC_CNTL_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](int_st) module"]
pub type INT_ST = crate::Reg<u32, _INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ST;
#[doc = "`read()` method returns [int_st::R](int_st::R) reader structure"]
impl crate::Readable for INT_ST {}
#[doc = "RTC_CNTL_INT_ST"]
pub mod int_st;
#[doc = "RTC_CNTL_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](int_clr) module"]
pub type INT_CLR = crate::Reg<u32, _INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CLR;
#[doc = "`write(|w| ..)` method takes [int_clr::W](int_clr::W) writer structure"]
impl crate::Writable for INT_CLR {}
#[doc = "RTC_CNTL_INT_CLR"]
pub mod int_clr;
#[doc = "RTC_CNTL_STORE0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store0](store0) module"]
pub type STORE0 = crate::Reg<u32, _STORE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STORE0;
#[doc = "`read()` method returns [store0::R](store0::R) reader structure"]
impl crate::Readable for STORE0 {}
#[doc = "`write(|w| ..)` method takes [store0::W](store0::W) writer structure"]
impl crate::Writable for STORE0 {}
#[doc = "RTC_CNTL_STORE0"]
pub mod store0;
#[doc = "RTC_CNTL_STORE1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store1](store1) module"]
pub type STORE1 = crate::Reg<u32, _STORE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STORE1;
#[doc = "`read()` method returns [store1::R](store1::R) reader structure"]
impl crate::Readable for STORE1 {}
#[doc = "`write(|w| ..)` method takes [store1::W](store1::W) writer structure"]
impl crate::Writable for STORE1 {}
#[doc = "RTC_CNTL_STORE1"]
pub mod store1;
#[doc = "RTC_CNTL_STORE2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store2](store2) module"]
pub type STORE2 = crate::Reg<u32, _STORE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STORE2;
#[doc = "`read()` method returns [store2::R](store2::R) reader structure"]
impl crate::Readable for STORE2 {}
#[doc = "`write(|w| ..)` method takes [store2::W](store2::W) writer structure"]
impl crate::Writable for STORE2 {}
#[doc = "RTC_CNTL_STORE2"]
pub mod store2;
#[doc = "RTC_CNTL_STORE3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store3](store3) module"]
pub type STORE3 = crate::Reg<u32, _STORE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STORE3;
#[doc = "`read()` method returns [store3::R](store3::R) reader structure"]
impl crate::Readable for STORE3 {}
#[doc = "`write(|w| ..)` method takes [store3::W](store3::W) writer structure"]
impl crate::Writable for STORE3 {}
#[doc = "RTC_CNTL_STORE3"]
pub mod store3;
#[doc = "RTC_CNTL_EXT_XTL_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_xtl_conf](ext_xtl_conf) module"]
pub type EXT_XTL_CONF = crate::Reg<u32, _EXT_XTL_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_XTL_CONF;
#[doc = "`read()` method returns [ext_xtl_conf::R](ext_xtl_conf::R) reader structure"]
impl crate::Readable for EXT_XTL_CONF {}
#[doc = "`write(|w| ..)` method takes [ext_xtl_conf::W](ext_xtl_conf::W) writer structure"]
impl crate::Writable for EXT_XTL_CONF {}
#[doc = "RTC_CNTL_EXT_XTL_CONF"]
pub mod ext_xtl_conf;
#[doc = "RTC_CNTL_EXT_WAKEUP_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_wakeup_conf](ext_wakeup_conf) module"]
pub type EXT_WAKEUP_CONF = crate::Reg<u32, _EXT_WAKEUP_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_WAKEUP_CONF;
#[doc = "`read()` method returns [ext_wakeup_conf::R](ext_wakeup_conf::R) reader structure"]
impl crate::Readable for EXT_WAKEUP_CONF {}
#[doc = "`write(|w| ..)` method takes [ext_wakeup_conf::W](ext_wakeup_conf::W) writer structure"]
impl crate::Writable for EXT_WAKEUP_CONF {}
#[doc = "RTC_CNTL_EXT_WAKEUP_CONF"]
pub mod ext_wakeup_conf;
#[doc = "RTC_CNTL_SLP_REJECT_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_reject_conf](slp_reject_conf) module"]
pub type SLP_REJECT_CONF = crate::Reg<u32, _SLP_REJECT_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLP_REJECT_CONF;
#[doc = "`read()` method returns [slp_reject_conf::R](slp_reject_conf::R) reader structure"]
impl crate::Readable for SLP_REJECT_CONF {}
#[doc = "`write(|w| ..)` method takes [slp_reject_conf::W](slp_reject_conf::W) writer structure"]
impl crate::Writable for SLP_REJECT_CONF {}
#[doc = "RTC_CNTL_SLP_REJECT_CONF"]
pub mod slp_reject_conf;
#[doc = "RTC_CNTL_CPU_PERIOD_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_period_conf](cpu_period_conf) module"]
pub type CPU_PERIOD_CONF = crate::Reg<u32, _CPU_PERIOD_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPU_PERIOD_CONF;
#[doc = "`read()` method returns [cpu_period_conf::R](cpu_period_conf::R) reader structure"]
impl crate::Readable for CPU_PERIOD_CONF {}
#[doc = "`write(|w| ..)` method takes [cpu_period_conf::W](cpu_period_conf::W) writer structure"]
impl crate::Writable for CPU_PERIOD_CONF {}
#[doc = "RTC_CNTL_CPU_PERIOD_CONF"]
pub mod cpu_period_conf;
#[doc = "RTC_CNTL_CLK_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_conf](clk_conf) module"]
pub type CLK_CONF = crate::Reg<u32, _CLK_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_CONF;
#[doc = "`read()` method returns [clk_conf::R](clk_conf::R) reader structure"]
impl crate::Readable for CLK_CONF {}
#[doc = "`write(|w| ..)` method takes [clk_conf::W](clk_conf::W) writer structure"]
impl crate::Writable for CLK_CONF {}
#[doc = "RTC_CNTL_CLK_CONF"]
pub mod clk_conf;
#[doc = "RTC_CNTL_SLOW_CLK_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slow_clk_conf](slow_clk_conf) module"]
pub type SLOW_CLK_CONF = crate::Reg<u32, _SLOW_CLK_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLOW_CLK_CONF;
#[doc = "`read()` method returns [slow_clk_conf::R](slow_clk_conf::R) reader structure"]
impl crate::Readable for SLOW_CLK_CONF {}
#[doc = "`write(|w| ..)` method takes [slow_clk_conf::W](slow_clk_conf::W) writer structure"]
impl crate::Writable for SLOW_CLK_CONF {}
#[doc = "RTC_CNTL_SLOW_CLK_CONF"]
pub mod slow_clk_conf;
#[doc = "RTC_CNTL_SDIO_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_conf](sdio_conf) module"]
pub type SDIO_CONF = crate::Reg<u32, _SDIO_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDIO_CONF;
#[doc = "`read()` method returns [sdio_conf::R](sdio_conf::R) reader structure"]
impl crate::Readable for SDIO_CONF {}
#[doc = "`write(|w| ..)` method takes [sdio_conf::W](sdio_conf::W) writer structure"]
impl crate::Writable for SDIO_CONF {}
#[doc = "RTC_CNTL_SDIO_CONF"]
pub mod sdio_conf;
#[doc = "RTC_CNTL_BIAS_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bias_conf](bias_conf) module"]
pub type BIAS_CONF = crate::Reg<u32, _BIAS_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BIAS_CONF;
#[doc = "`read()` method returns [bias_conf::R](bias_conf::R) reader structure"]
impl crate::Readable for BIAS_CONF {}
#[doc = "`write(|w| ..)` method takes [bias_conf::W](bias_conf::W) writer structure"]
impl crate::Writable for BIAS_CONF {}
#[doc = "RTC_CNTL_BIAS_CONF"]
pub mod bias_conf;
#[doc = "RTC_CNTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl](rtc_cntl) module"]
pub type RTC_CNTL = crate::Reg<u32, _RTC_CNTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNTL;
#[doc = "`read()` method returns [rtc_cntl::R](rtc_cntl::R) reader structure"]
impl crate::Readable for RTC_CNTL {}
#[doc = "`write(|w| ..)` method takes [rtc_cntl::W](rtc_cntl::W) writer structure"]
impl crate::Writable for RTC_CNTL {}
#[doc = "RTC_CNTL"]
pub mod rtc_cntl;
#[doc = "RTC_CNTL_PWC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwc](pwc) module"]
pub type PWC = crate::Reg<u32, _PWC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWC;
#[doc = "`read()` method returns [pwc::R](pwc::R) reader structure"]
impl crate::Readable for PWC {}
#[doc = "`write(|w| ..)` method takes [pwc::W](pwc::W) writer structure"]
impl crate::Writable for PWC {}
#[doc = "RTC_CNTL_PWC"]
pub mod pwc;
#[doc = "RTC_CNTL_DIG_PWC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_pwc](dig_pwc) module"]
pub type DIG_PWC = crate::Reg<u32, _DIG_PWC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIG_PWC;
#[doc = "`read()` method returns [dig_pwc::R](dig_pwc::R) reader structure"]
impl crate::Readable for DIG_PWC {}
#[doc = "`write(|w| ..)` method takes [dig_pwc::W](dig_pwc::W) writer structure"]
impl crate::Writable for DIG_PWC {}
#[doc = "RTC_CNTL_DIG_PWC"]
pub mod dig_pwc;
#[doc = "RTC_CNTL_DIG_ISO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_iso](dig_iso) module"]
pub type DIG_ISO = crate::Reg<u32, _DIG_ISO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIG_ISO;
#[doc = "`read()` method returns [dig_iso::R](dig_iso::R) reader structure"]
impl crate::Readable for DIG_ISO {}
#[doc = "`write(|w| ..)` method takes [dig_iso::W](dig_iso::W) writer structure"]
impl crate::Writable for DIG_ISO {}
#[doc = "RTC_CNTL_DIG_ISO"]
pub mod dig_iso;
#[doc = "RTC_CNTL_WDTCONFIG0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconfig0](wdtconfig0) module"]
pub type WDTCONFIG0 = crate::Reg<u32, _WDTCONFIG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONFIG0;
#[doc = "`read()` method returns [wdtconfig0::R](wdtconfig0::R) reader structure"]
impl crate::Readable for WDTCONFIG0 {}
#[doc = "`write(|w| ..)` method takes [wdtconfig0::W](wdtconfig0::W) writer structure"]
impl crate::Writable for WDTCONFIG0 {}
#[doc = "RTC_CNTL_WDTCONFIG0"]
pub mod wdtconfig0;
#[doc = "RTC_CNTL_WDTCONFIG1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconfig1](wdtconfig1) module"]
pub type WDTCONFIG1 = crate::Reg<u32, _WDTCONFIG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONFIG1;
#[doc = "`read()` method returns [wdtconfig1::R](wdtconfig1::R) reader structure"]
impl crate::Readable for WDTCONFIG1 {}
#[doc = "`write(|w| ..)` method takes [wdtconfig1::W](wdtconfig1::W) writer structure"]
impl crate::Writable for WDTCONFIG1 {}
#[doc = "RTC_CNTL_WDTCONFIG1"]
pub mod wdtconfig1;
#[doc = "RTC_CNTL_WDTCONFIG2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconfig2](wdtconfig2) module"]
pub type WDTCONFIG2 = crate::Reg<u32, _WDTCONFIG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONFIG2;
#[doc = "`read()` method returns [wdtconfig2::R](wdtconfig2::R) reader structure"]
impl crate::Readable for WDTCONFIG2 {}
#[doc = "`write(|w| ..)` method takes [wdtconfig2::W](wdtconfig2::W) writer structure"]
impl crate::Writable for WDTCONFIG2 {}
#[doc = "RTC_CNTL_WDTCONFIG2"]
pub mod wdtconfig2;
#[doc = "RTC_CNTL_WDTCONFIG3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconfig3](wdtconfig3) module"]
pub type WDTCONFIG3 = crate::Reg<u32, _WDTCONFIG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONFIG3;
#[doc = "`read()` method returns [wdtconfig3::R](wdtconfig3::R) reader structure"]
impl crate::Readable for WDTCONFIG3 {}
#[doc = "`write(|w| ..)` method takes [wdtconfig3::W](wdtconfig3::W) writer structure"]
impl crate::Writable for WDTCONFIG3 {}
#[doc = "RTC_CNTL_WDTCONFIG3"]
pub mod wdtconfig3;
#[doc = "RTC_CNTL_WDTCONFIG4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconfig4](wdtconfig4) module"]
pub type WDTCONFIG4 = crate::Reg<u32, _WDTCONFIG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONFIG4;
#[doc = "`read()` method returns [wdtconfig4::R](wdtconfig4::R) reader structure"]
impl crate::Readable for WDTCONFIG4 {}
#[doc = "`write(|w| ..)` method takes [wdtconfig4::W](wdtconfig4::W) writer structure"]
impl crate::Writable for WDTCONFIG4 {}
#[doc = "RTC_CNTL_WDTCONFIG4"]
pub mod wdtconfig4;
#[doc = "RTC_CNTL_WDTFEED\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtfeed](wdtfeed) module"]
pub type WDTFEED = crate::Reg<u32, _WDTFEED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTFEED;
#[doc = "`write(|w| ..)` method takes [wdtfeed::W](wdtfeed::W) writer structure"]
impl crate::Writable for WDTFEED {}
#[doc = "RTC_CNTL_WDTFEED"]
pub mod wdtfeed;
#[doc = "RTC_CNTL_WDTWPROTECT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtwprotect](wdtwprotect) module"]
pub type WDTWPROTECT = crate::Reg<u32, _WDTWPROTECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTWPROTECT;
#[doc = "`read()` method returns [wdtwprotect::R](wdtwprotect::R) reader structure"]
impl crate::Readable for WDTWPROTECT {}
#[doc = "`write(|w| ..)` method takes [wdtwprotect::W](wdtwprotect::W) writer structure"]
impl crate::Writable for WDTWPROTECT {}
#[doc = "RTC_CNTL_WDTWPROTECT"]
pub mod wdtwprotect;
#[doc = "RTC_CNTL_SWD_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swd_conf](swd_conf) module"]
pub type SWD_CONF = crate::Reg<u32, _SWD_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWD_CONF;
#[doc = "`read()` method returns [swd_conf::R](swd_conf::R) reader structure"]
impl crate::Readable for SWD_CONF {}
#[doc = "`write(|w| ..)` method takes [swd_conf::W](swd_conf::W) writer structure"]
impl crate::Writable for SWD_CONF {}
#[doc = "RTC_CNTL_SWD_CONF"]
pub mod swd_conf;
#[doc = "RTC_CNTL_SWD_WPROTECT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swd_wprotect](swd_wprotect) module"]
pub type SWD_WPROTECT = crate::Reg<u32, _SWD_WPROTECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWD_WPROTECT;
#[doc = "`read()` method returns [swd_wprotect::R](swd_wprotect::R) reader structure"]
impl crate::Readable for SWD_WPROTECT {}
#[doc = "`write(|w| ..)` method takes [swd_wprotect::W](swd_wprotect::W) writer structure"]
impl crate::Writable for SWD_WPROTECT {}
#[doc = "RTC_CNTL_SWD_WPROTECT"]
pub mod swd_wprotect;
#[doc = "RTC_CNTL_SW_CPU_STALL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_cpu_stall](sw_cpu_stall) module"]
pub type SW_CPU_STALL = crate::Reg<u32, _SW_CPU_STALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_CPU_STALL;
#[doc = "`read()` method returns [sw_cpu_stall::R](sw_cpu_stall::R) reader structure"]
impl crate::Readable for SW_CPU_STALL {}
#[doc = "`write(|w| ..)` method takes [sw_cpu_stall::W](sw_cpu_stall::W) writer structure"]
impl crate::Writable for SW_CPU_STALL {}
#[doc = "RTC_CNTL_SW_CPU_STALL"]
pub mod sw_cpu_stall;
#[doc = "RTC_CNTL_STORE4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store4](store4) module"]
pub type STORE4 = crate::Reg<u32, _STORE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STORE4;
#[doc = "`read()` method returns [store4::R](store4::R) reader structure"]
impl crate::Readable for STORE4 {}
#[doc = "`write(|w| ..)` method takes [store4::W](store4::W) writer structure"]
impl crate::Writable for STORE4 {}
#[doc = "RTC_CNTL_STORE4"]
pub mod store4;
#[doc = "RTC_CNTL_STORE5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store5](store5) module"]
pub type STORE5 = crate::Reg<u32, _STORE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STORE5;
#[doc = "`read()` method returns [store5::R](store5::R) reader structure"]
impl crate::Readable for STORE5 {}
#[doc = "`write(|w| ..)` method takes [store5::W](store5::W) writer structure"]
impl crate::Writable for STORE5 {}
#[doc = "RTC_CNTL_STORE5"]
pub mod store5;
#[doc = "RTC_CNTL_STORE6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store6](store6) module"]
pub type STORE6 = crate::Reg<u32, _STORE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STORE6;
#[doc = "`read()` method returns [store6::R](store6::R) reader structure"]
impl crate::Readable for STORE6 {}
#[doc = "`write(|w| ..)` method takes [store6::W](store6::W) writer structure"]
impl crate::Writable for STORE6 {}
#[doc = "RTC_CNTL_STORE6"]
pub mod store6;
#[doc = "RTC_CNTL_STORE7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store7](store7) module"]
pub type STORE7 = crate::Reg<u32, _STORE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STORE7;
#[doc = "`read()` method returns [store7::R](store7::R) reader structure"]
impl crate::Readable for STORE7 {}
#[doc = "`write(|w| ..)` method takes [store7::W](store7::W) writer structure"]
impl crate::Writable for STORE7 {}
#[doc = "RTC_CNTL_STORE7"]
pub mod store7;
#[doc = "RTC_CNTL_LOW_POWER_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [low_power_st](low_power_st) module"]
pub type LOW_POWER_ST = crate::Reg<u32, _LOW_POWER_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOW_POWER_ST;
#[doc = "`read()` method returns [low_power_st::R](low_power_st::R) reader structure"]
impl crate::Readable for LOW_POWER_ST {}
#[doc = "RTC_CNTL_LOW_POWER_ST"]
pub mod low_power_st;
#[doc = "RTC_CNTL_DIAG0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diag0](diag0) module"]
pub type DIAG0 = crate::Reg<u32, _DIAG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIAG0;
#[doc = "`read()` method returns [diag0::R](diag0::R) reader structure"]
impl crate::Readable for DIAG0 {}
#[doc = "RTC_CNTL_DIAG0"]
pub mod diag0;
#[doc = "RTC_CNTL_PAD_HOLD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_hold](pad_hold) module"]
pub type PAD_HOLD = crate::Reg<u32, _PAD_HOLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAD_HOLD;
#[doc = "`read()` method returns [pad_hold::R](pad_hold::R) reader structure"]
impl crate::Readable for PAD_HOLD {}
#[doc = "`write(|w| ..)` method takes [pad_hold::W](pad_hold::W) writer structure"]
impl crate::Writable for PAD_HOLD {}
#[doc = "RTC_CNTL_PAD_HOLD"]
pub mod pad_hold;
#[doc = "RTC_CNTL_DIG_PAD_HOLD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_pad_hold](dig_pad_hold) module"]
pub type DIG_PAD_HOLD = crate::Reg<u32, _DIG_PAD_HOLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIG_PAD_HOLD;
#[doc = "`read()` method returns [dig_pad_hold::R](dig_pad_hold::R) reader structure"]
impl crate::Readable for DIG_PAD_HOLD {}
#[doc = "`write(|w| ..)` method takes [dig_pad_hold::W](dig_pad_hold::W) writer structure"]
impl crate::Writable for DIG_PAD_HOLD {}
#[doc = "RTC_CNTL_DIG_PAD_HOLD"]
pub mod dig_pad_hold;
#[doc = "RTC_CNTL_BROWN_OUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brown_out](brown_out) module"]
pub type BROWN_OUT = crate::Reg<u32, _BROWN_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BROWN_OUT;
#[doc = "`read()` method returns [brown_out::R](brown_out::R) reader structure"]
impl crate::Readable for BROWN_OUT {}
#[doc = "`write(|w| ..)` method takes [brown_out::W](brown_out::W) writer structure"]
impl crate::Writable for BROWN_OUT {}
#[doc = "RTC_CNTL_BROWN_OUT"]
pub mod brown_out;
#[doc = "RTC_CNTL_TIME_LOW1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time_low1](time_low1) module"]
pub type TIME_LOW1 = crate::Reg<u32, _TIME_LOW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIME_LOW1;
#[doc = "`read()` method returns [time_low1::R](time_low1::R) reader structure"]
impl crate::Readable for TIME_LOW1 {}
#[doc = "RTC_CNTL_TIME_LOW1"]
pub mod time_low1;
#[doc = "RTC_CNTL_TIME_HIGH1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time_high1](time_high1) module"]
pub type TIME_HIGH1 = crate::Reg<u32, _TIME_HIGH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIME_HIGH1;
#[doc = "`read()` method returns [time_high1::R](time_high1::R) reader structure"]
impl crate::Readable for TIME_HIGH1 {}
#[doc = "RTC_CNTL_TIME_HIGH1"]
pub mod time_high1;
#[doc = "RTC_CNTL_XTAL32K_CLK_FACTOR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal32k_clk_factor](xtal32k_clk_factor) module"]
pub type XTAL32K_CLK_FACTOR = crate::Reg<u32, _XTAL32K_CLK_FACTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XTAL32K_CLK_FACTOR;
#[doc = "`read()` method returns [xtal32k_clk_factor::R](xtal32k_clk_factor::R) reader structure"]
impl crate::Readable for XTAL32K_CLK_FACTOR {}
#[doc = "`write(|w| ..)` method takes [xtal32k_clk_factor::W](xtal32k_clk_factor::W) writer structure"]
impl crate::Writable for XTAL32K_CLK_FACTOR {}
#[doc = "RTC_CNTL_XTAL32K_CLK_FACTOR"]
pub mod xtal32k_clk_factor;
#[doc = "RTC_CNTL_XTAL32K_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal32k_conf](xtal32k_conf) module"]
pub type XTAL32K_CONF = crate::Reg<u32, _XTAL32K_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XTAL32K_CONF;
#[doc = "`read()` method returns [xtal32k_conf::R](xtal32k_conf::R) reader structure"]
impl crate::Readable for XTAL32K_CONF {}
#[doc = "`write(|w| ..)` method takes [xtal32k_conf::W](xtal32k_conf::W) writer structure"]
impl crate::Writable for XTAL32K_CONF {}
#[doc = "RTC_CNTL_XTAL32K_CONF"]
pub mod xtal32k_conf;
#[doc = "RTC_CNTL_USB_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_conf](usb_conf) module"]
pub type USB_CONF = crate::Reg<u32, _USB_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_CONF;
#[doc = "`read()` method returns [usb_conf::R](usb_conf::R) reader structure"]
impl crate::Readable for USB_CONF {}
#[doc = "`write(|w| ..)` method takes [usb_conf::W](usb_conf::W) writer structure"]
impl crate::Writable for USB_CONF {}
#[doc = "RTC_CNTL_USB_CONF"]
pub mod usb_conf;
#[doc = "RTC_CNTL_SLP_REJECT_CAUSE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_reject_cause](slp_reject_cause) module"]
pub type SLP_REJECT_CAUSE = crate::Reg<u32, _SLP_REJECT_CAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLP_REJECT_CAUSE;
#[doc = "`read()` method returns [slp_reject_cause::R](slp_reject_cause::R) reader structure"]
impl crate::Readable for SLP_REJECT_CAUSE {}
#[doc = "RTC_CNTL_SLP_REJECT_CAUSE"]
pub mod slp_reject_cause;
#[doc = "RTC_CNTL_OPTION1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [option1](option1) module"]
pub type OPTION1 = crate::Reg<u32, _OPTION1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTION1;
#[doc = "`read()` method returns [option1::R](option1::R) reader structure"]
impl crate::Readable for OPTION1 {}
#[doc = "`write(|w| ..)` method takes [option1::W](option1::W) writer structure"]
impl crate::Writable for OPTION1 {}
#[doc = "RTC_CNTL_OPTION1"]
pub mod option1;
#[doc = "RTC_CNTL_SLP_WAKEUP_CAUSE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_wakeup_cause](slp_wakeup_cause) module"]
pub type SLP_WAKEUP_CAUSE = crate::Reg<u32, _SLP_WAKEUP_CAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLP_WAKEUP_CAUSE;
#[doc = "`read()` method returns [slp_wakeup_cause::R](slp_wakeup_cause::R) reader structure"]
impl crate::Readable for SLP_WAKEUP_CAUSE {}
#[doc = "RTC_CNTL_SLP_WAKEUP_CAUSE"]
pub mod slp_wakeup_cause;
#[doc = "RTC_CNTL_ULP_CP_TIMER_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ulp_cp_timer_1](ulp_cp_timer_1) module"]
pub type ULP_CP_TIMER_1 = crate::Reg<u32, _ULP_CP_TIMER_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ULP_CP_TIMER_1;
#[doc = "`read()` method returns [ulp_cp_timer_1::R](ulp_cp_timer_1::R) reader structure"]
impl crate::Readable for ULP_CP_TIMER_1 {}
#[doc = "`write(|w| ..)` method takes [ulp_cp_timer_1::W](ulp_cp_timer_1::W) writer structure"]
impl crate::Writable for ULP_CP_TIMER_1 {}
#[doc = "RTC_CNTL_ULP_CP_TIMER_1"]
pub mod ulp_cp_timer_1;
#[doc = "RTC_CNTL_INT_ENA_W1TS\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena_w1ts](int_ena_w1ts) module"]
pub type INT_ENA_W1TS = crate::Reg<u32, _INT_ENA_W1TS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ENA_W1TS;
#[doc = "`write(|w| ..)` method takes [int_ena_w1ts::W](int_ena_w1ts::W) writer structure"]
impl crate::Writable for INT_ENA_W1TS {}
#[doc = "RTC_CNTL_INT_ENA_W1TS"]
pub mod int_ena_w1ts;
#[doc = "RTC_CNTL_INT_ENA_W1TC\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena_w1tc](int_ena_w1tc) module"]
pub type INT_ENA_W1TC = crate::Reg<u32, _INT_ENA_W1TC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ENA_W1TC;
#[doc = "`write(|w| ..)` method takes [int_ena_w1tc::W](int_ena_w1tc::W) writer structure"]
impl crate::Writable for INT_ENA_W1TC {}
#[doc = "RTC_CNTL_INT_ENA_W1TC"]
pub mod int_ena_w1tc;
#[doc = "RTC_CNTL_RETENTION_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retention_ctrl](retention_ctrl) module"]
pub type RETENTION_CTRL = crate::Reg<u32, _RETENTION_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RETENTION_CTRL;
#[doc = "`read()` method returns [retention_ctrl::R](retention_ctrl::R) reader structure"]
impl crate::Readable for RETENTION_CTRL {}
#[doc = "`write(|w| ..)` method takes [retention_ctrl::W](retention_ctrl::W) writer structure"]
impl crate::Writable for RETENTION_CTRL {}
#[doc = "RTC_CNTL_RETENTION_CTRL"]
pub mod retention_ctrl;
#[doc = "RTC_CNTL_FIB_SEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fib_sel](fib_sel) module"]
pub type FIB_SEL = crate::Reg<u32, _FIB_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIB_SEL;
#[doc = "`read()` method returns [fib_sel::R](fib_sel::R) reader structure"]
impl crate::Readable for FIB_SEL {}
#[doc = "`write(|w| ..)` method takes [fib_sel::W](fib_sel::W) writer structure"]
impl crate::Writable for FIB_SEL {}
#[doc = "RTC_CNTL_FIB_SEL"]
pub mod fib_sel;
#[doc = "RTC_CNTL_GPIO_WAKEUP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_wakeup](gpio_wakeup) module"]
pub type GPIO_WAKEUP = crate::Reg<u32, _GPIO_WAKEUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_WAKEUP;
#[doc = "`read()` method returns [gpio_wakeup::R](gpio_wakeup::R) reader structure"]
impl crate::Readable for GPIO_WAKEUP {}
#[doc = "`write(|w| ..)` method takes [gpio_wakeup::W](gpio_wakeup::W) writer structure"]
impl crate::Writable for GPIO_WAKEUP {}
#[doc = "RTC_CNTL_GPIO_WAKEUP"]
pub mod gpio_wakeup;
#[doc = "RTC_CNTL_DBG_SEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_sel](dbg_sel) module"]
pub type DBG_SEL = crate::Reg<u32, _DBG_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBG_SEL;
#[doc = "`read()` method returns [dbg_sel::R](dbg_sel::R) reader structure"]
impl crate::Readable for DBG_SEL {}
#[doc = "`write(|w| ..)` method takes [dbg_sel::W](dbg_sel::W) writer structure"]
impl crate::Writable for DBG_SEL {}
#[doc = "RTC_CNTL_DBG_SEL"]
pub mod dbg_sel;
#[doc = "RTC_CNTL_DBG_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_map](dbg_map) module"]
pub type DBG_MAP = crate::Reg<u32, _DBG_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBG_MAP;
#[doc = "`read()` method returns [dbg_map::R](dbg_map::R) reader structure"]
impl crate::Readable for DBG_MAP {}
#[doc = "`write(|w| ..)` method takes [dbg_map::W](dbg_map::W) writer structure"]
impl crate::Writable for DBG_MAP {}
#[doc = "RTC_CNTL_DBG_MAP"]
pub mod dbg_map;
#[doc = "RTC_CNTL_SENSOR_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensor_ctrl](sensor_ctrl) module"]
pub type SENSOR_CTRL = crate::Reg<u32, _SENSOR_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSOR_CTRL;
#[doc = "`read()` method returns [sensor_ctrl::R](sensor_ctrl::R) reader structure"]
impl crate::Readable for SENSOR_CTRL {}
#[doc = "`write(|w| ..)` method takes [sensor_ctrl::W](sensor_ctrl::W) writer structure"]
impl crate::Writable for SENSOR_CTRL {}
#[doc = "RTC_CNTL_SENSOR_CTRL"]
pub mod sensor_ctrl;
#[doc = "RTC_CNTL_DBG_SAR_SEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_sar_sel](dbg_sar_sel) module"]
pub type DBG_SAR_SEL = crate::Reg<u32, _DBG_SAR_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBG_SAR_SEL;
#[doc = "`read()` method returns [dbg_sar_sel::R](dbg_sar_sel::R) reader structure"]
impl crate::Readable for DBG_SAR_SEL {}
#[doc = "`write(|w| ..)` method takes [dbg_sar_sel::W](dbg_sar_sel::W) writer structure"]
impl crate::Writable for DBG_SAR_SEL {}
#[doc = "RTC_CNTL_DBG_SAR_SEL"]
pub mod dbg_sar_sel;
#[doc = "RTC_CNTL_PG_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg_ctrl](pg_ctrl) module"]
pub type PG_CTRL = crate::Reg<u32, _PG_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PG_CTRL;
#[doc = "`read()` method returns [pg_ctrl::R](pg_ctrl::R) reader structure"]
impl crate::Readable for PG_CTRL {}
#[doc = "`write(|w| ..)` method takes [pg_ctrl::W](pg_ctrl::W) writer structure"]
impl crate::Writable for PG_CTRL {}
#[doc = "RTC_CNTL_PG_CTRL"]
pub mod pg_ctrl;
#[doc = "RTC_CNTL_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "RTC_CNTL_DATE"]
pub mod date;
