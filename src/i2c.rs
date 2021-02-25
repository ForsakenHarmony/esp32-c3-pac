#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C_SCL_LOW_PERIOD"]
    pub scl_low_period: SCL_LOW_PERIOD,
    #[doc = "0x04 - I2C_CTR"]
    pub ctr: CTR,
    #[doc = "0x08 - I2C_SR"]
    pub sr: SR,
    #[doc = "0x0c - I2C_TO"]
    pub to: TO,
    #[doc = "0x10 - I2C_SLAVE_ADDR"]
    pub slave_addr: SLAVE_ADDR,
    #[doc = "0x14 - I2C_FIFO_ST"]
    pub fifo_st: FIFO_ST,
    #[doc = "0x18 - I2C_FIFO_CONF"]
    pub fifo_conf: FIFO_CONF,
    #[doc = "0x1c - I2C_DATA"]
    pub data: DATA,
    #[doc = "0x20 - I2C_INT_RAW"]
    pub int_raw: INT_RAW,
    #[doc = "0x24 - I2C_INT_CLR"]
    pub int_clr: INT_CLR,
    #[doc = "0x28 - I2C_INT_ENA"]
    pub int_ena: INT_ENA,
    #[doc = "0x2c - I2C_INT_STATUS"]
    pub int_status: INT_STATUS,
    #[doc = "0x30 - I2C_SDA_HOLD"]
    pub sda_hold: SDA_HOLD,
    #[doc = "0x34 - I2C_SDA_SAMPLE"]
    pub sda_sample: SDA_SAMPLE,
    #[doc = "0x38 - I2C_SCL_HIGH_PERIOD"]
    pub scl_high_period: SCL_HIGH_PERIOD,
    _reserved15: [u8; 4usize],
    #[doc = "0x40 - I2C_SCL_START_HOLD"]
    pub scl_start_hold: SCL_START_HOLD,
    #[doc = "0x44 - I2C_SCL_RSTART_SETUP"]
    pub scl_rstart_setup: SCL_RSTART_SETUP,
    #[doc = "0x48 - I2C_SCL_STOP_HOLD"]
    pub scl_stop_hold: SCL_STOP_HOLD,
    #[doc = "0x4c - I2C_SCL_STOP_SETUP"]
    pub scl_stop_setup: SCL_STOP_SETUP,
    #[doc = "0x50 - I2C_FILTER_CFG"]
    pub filter_cfg: FILTER_CFG,
    #[doc = "0x54 - I2C_CLK_CONF"]
    pub clk_conf: CLK_CONF,
    #[doc = "0x58 - I2C_COMD0"]
    pub comd0: COMD0,
    #[doc = "0x5c - I2C_COMD1"]
    pub comd1: COMD1,
    #[doc = "0x60 - I2C_COMD2"]
    pub comd2: COMD2,
    #[doc = "0x64 - I2C_COMD3"]
    pub comd3: COMD3,
    #[doc = "0x68 - I2C_COMD4"]
    pub comd4: COMD4,
    #[doc = "0x6c - I2C_COMD5"]
    pub comd5: COMD5,
    #[doc = "0x70 - I2C_COMD6"]
    pub comd6: COMD6,
    #[doc = "0x74 - I2C_COMD7"]
    pub comd7: COMD7,
    #[doc = "0x78 - I2C_SCL_ST_TIME_OUT"]
    pub scl_st_time_out: SCL_ST_TIME_OUT,
    #[doc = "0x7c - I2C_SCL_MAIN_ST_TIME_OUT"]
    pub scl_main_st_time_out: SCL_MAIN_ST_TIME_OUT,
    #[doc = "0x80 - I2C_SCL_SP_CONF"]
    pub scl_sp_conf: SCL_SP_CONF,
    #[doc = "0x84 - I2C_SCL_STRETCH_CONF"]
    pub scl_stretch_conf: SCL_STRETCH_CONF,
    _reserved33: [u8; 112usize],
    #[doc = "0xf8 - I2C_DATE"]
    pub date: DATE,
}
#[doc = "I2C_SCL_LOW_PERIOD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_low_period](scl_low_period) module"]
pub type SCL_LOW_PERIOD = crate::Reg<u32, _SCL_LOW_PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_LOW_PERIOD;
#[doc = "`read()` method returns [scl_low_period::R](scl_low_period::R) reader structure"]
impl crate::Readable for SCL_LOW_PERIOD {}
#[doc = "`write(|w| ..)` method takes [scl_low_period::W](scl_low_period::W) writer structure"]
impl crate::Writable for SCL_LOW_PERIOD {}
#[doc = "I2C_SCL_LOW_PERIOD"]
pub mod scl_low_period;
#[doc = "I2C_CTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr](ctr) module"]
pub type CTR = crate::Reg<u32, _CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTR;
#[doc = "`read()` method returns [ctr::R](ctr::R) reader structure"]
impl crate::Readable for CTR {}
#[doc = "`write(|w| ..)` method takes [ctr::W](ctr::W) writer structure"]
impl crate::Writable for CTR {}
#[doc = "I2C_CTR"]
pub mod ctr;
#[doc = "I2C_SR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "I2C_SR"]
pub mod sr;
#[doc = "I2C_TO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [to](to) module"]
pub type TO = crate::Reg<u32, _TO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TO;
#[doc = "`read()` method returns [to::R](to::R) reader structure"]
impl crate::Readable for TO {}
#[doc = "`write(|w| ..)` method takes [to::W](to::W) writer structure"]
impl crate::Writable for TO {}
#[doc = "I2C_TO"]
pub mod to;
#[doc = "I2C_SLAVE_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave_addr](slave_addr) module"]
pub type SLAVE_ADDR = crate::Reg<u32, _SLAVE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLAVE_ADDR;
#[doc = "`read()` method returns [slave_addr::R](slave_addr::R) reader structure"]
impl crate::Readable for SLAVE_ADDR {}
#[doc = "`write(|w| ..)` method takes [slave_addr::W](slave_addr::W) writer structure"]
impl crate::Writable for SLAVE_ADDR {}
#[doc = "I2C_SLAVE_ADDR"]
pub mod slave_addr;
#[doc = "I2C_FIFO_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_st](fifo_st) module"]
pub type FIFO_ST = crate::Reg<u32, _FIFO_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_ST;
#[doc = "`read()` method returns [fifo_st::R](fifo_st::R) reader structure"]
impl crate::Readable for FIFO_ST {}
#[doc = "I2C_FIFO_ST"]
pub mod fifo_st;
#[doc = "I2C_FIFO_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_conf](fifo_conf) module"]
pub type FIFO_CONF = crate::Reg<u32, _FIFO_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_CONF;
#[doc = "`read()` method returns [fifo_conf::R](fifo_conf::R) reader structure"]
impl crate::Readable for FIFO_CONF {}
#[doc = "`write(|w| ..)` method takes [fifo_conf::W](fifo_conf::W) writer structure"]
impl crate::Writable for FIFO_CONF {}
#[doc = "I2C_FIFO_CONF"]
pub mod fifo_conf;
#[doc = "I2C_DATA\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "I2C_DATA"]
pub mod data;
#[doc = "I2C_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](int_raw) module"]
pub type INT_RAW = crate::Reg<u32, _INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_RAW;
#[doc = "`read()` method returns [int_raw::R](int_raw::R) reader structure"]
impl crate::Readable for INT_RAW {}
#[doc = "`write(|w| ..)` method takes [int_raw::W](int_raw::W) writer structure"]
impl crate::Writable for INT_RAW {}
#[doc = "I2C_INT_RAW"]
pub mod int_raw;
#[doc = "I2C_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](int_clr) module"]
pub type INT_CLR = crate::Reg<u32, _INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CLR;
#[doc = "`write(|w| ..)` method takes [int_clr::W](int_clr::W) writer structure"]
impl crate::Writable for INT_CLR {}
#[doc = "I2C_INT_CLR"]
pub mod int_clr;
#[doc = "I2C_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](int_ena) module"]
pub type INT_ENA = crate::Reg<u32, _INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ENA;
#[doc = "`read()` method returns [int_ena::R](int_ena::R) reader structure"]
impl crate::Readable for INT_ENA {}
#[doc = "`write(|w| ..)` method takes [int_ena::W](int_ena::W) writer structure"]
impl crate::Writable for INT_ENA {}
#[doc = "I2C_INT_ENA"]
pub mod int_ena;
#[doc = "I2C_INT_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_status](int_status) module"]
pub type INT_STATUS = crate::Reg<u32, _INT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_STATUS;
#[doc = "`read()` method returns [int_status::R](int_status::R) reader structure"]
impl crate::Readable for INT_STATUS {}
#[doc = "I2C_INT_STATUS"]
pub mod int_status;
#[doc = "I2C_SDA_HOLD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sda_hold](sda_hold) module"]
pub type SDA_HOLD = crate::Reg<u32, _SDA_HOLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDA_HOLD;
#[doc = "`read()` method returns [sda_hold::R](sda_hold::R) reader structure"]
impl crate::Readable for SDA_HOLD {}
#[doc = "`write(|w| ..)` method takes [sda_hold::W](sda_hold::W) writer structure"]
impl crate::Writable for SDA_HOLD {}
#[doc = "I2C_SDA_HOLD"]
pub mod sda_hold;
#[doc = "I2C_SDA_SAMPLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sda_sample](sda_sample) module"]
pub type SDA_SAMPLE = crate::Reg<u32, _SDA_SAMPLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDA_SAMPLE;
#[doc = "`read()` method returns [sda_sample::R](sda_sample::R) reader structure"]
impl crate::Readable for SDA_SAMPLE {}
#[doc = "`write(|w| ..)` method takes [sda_sample::W](sda_sample::W) writer structure"]
impl crate::Writable for SDA_SAMPLE {}
#[doc = "I2C_SDA_SAMPLE"]
pub mod sda_sample;
#[doc = "I2C_SCL_HIGH_PERIOD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_high_period](scl_high_period) module"]
pub type SCL_HIGH_PERIOD = crate::Reg<u32, _SCL_HIGH_PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_HIGH_PERIOD;
#[doc = "`read()` method returns [scl_high_period::R](scl_high_period::R) reader structure"]
impl crate::Readable for SCL_HIGH_PERIOD {}
#[doc = "`write(|w| ..)` method takes [scl_high_period::W](scl_high_period::W) writer structure"]
impl crate::Writable for SCL_HIGH_PERIOD {}
#[doc = "I2C_SCL_HIGH_PERIOD"]
pub mod scl_high_period;
#[doc = "I2C_SCL_START_HOLD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_start_hold](scl_start_hold) module"]
pub type SCL_START_HOLD = crate::Reg<u32, _SCL_START_HOLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_START_HOLD;
#[doc = "`read()` method returns [scl_start_hold::R](scl_start_hold::R) reader structure"]
impl crate::Readable for SCL_START_HOLD {}
#[doc = "`write(|w| ..)` method takes [scl_start_hold::W](scl_start_hold::W) writer structure"]
impl crate::Writable for SCL_START_HOLD {}
#[doc = "I2C_SCL_START_HOLD"]
pub mod scl_start_hold;
#[doc = "I2C_SCL_RSTART_SETUP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_rstart_setup](scl_rstart_setup) module"]
pub type SCL_RSTART_SETUP = crate::Reg<u32, _SCL_RSTART_SETUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_RSTART_SETUP;
#[doc = "`read()` method returns [scl_rstart_setup::R](scl_rstart_setup::R) reader structure"]
impl crate::Readable for SCL_RSTART_SETUP {}
#[doc = "`write(|w| ..)` method takes [scl_rstart_setup::W](scl_rstart_setup::W) writer structure"]
impl crate::Writable for SCL_RSTART_SETUP {}
#[doc = "I2C_SCL_RSTART_SETUP"]
pub mod scl_rstart_setup;
#[doc = "I2C_SCL_STOP_HOLD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_stop_hold](scl_stop_hold) module"]
pub type SCL_STOP_HOLD = crate::Reg<u32, _SCL_STOP_HOLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_STOP_HOLD;
#[doc = "`read()` method returns [scl_stop_hold::R](scl_stop_hold::R) reader structure"]
impl crate::Readable for SCL_STOP_HOLD {}
#[doc = "`write(|w| ..)` method takes [scl_stop_hold::W](scl_stop_hold::W) writer structure"]
impl crate::Writable for SCL_STOP_HOLD {}
#[doc = "I2C_SCL_STOP_HOLD"]
pub mod scl_stop_hold;
#[doc = "I2C_SCL_STOP_SETUP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_stop_setup](scl_stop_setup) module"]
pub type SCL_STOP_SETUP = crate::Reg<u32, _SCL_STOP_SETUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_STOP_SETUP;
#[doc = "`read()` method returns [scl_stop_setup::R](scl_stop_setup::R) reader structure"]
impl crate::Readable for SCL_STOP_SETUP {}
#[doc = "`write(|w| ..)` method takes [scl_stop_setup::W](scl_stop_setup::W) writer structure"]
impl crate::Writable for SCL_STOP_SETUP {}
#[doc = "I2C_SCL_STOP_SETUP"]
pub mod scl_stop_setup;
#[doc = "I2C_FILTER_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filter_cfg](filter_cfg) module"]
pub type FILTER_CFG = crate::Reg<u32, _FILTER_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FILTER_CFG;
#[doc = "`read()` method returns [filter_cfg::R](filter_cfg::R) reader structure"]
impl crate::Readable for FILTER_CFG {}
#[doc = "`write(|w| ..)` method takes [filter_cfg::W](filter_cfg::W) writer structure"]
impl crate::Writable for FILTER_CFG {}
#[doc = "I2C_FILTER_CFG"]
pub mod filter_cfg;
#[doc = "I2C_CLK_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_conf](clk_conf) module"]
pub type CLK_CONF = crate::Reg<u32, _CLK_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_CONF;
#[doc = "`read()` method returns [clk_conf::R](clk_conf::R) reader structure"]
impl crate::Readable for CLK_CONF {}
#[doc = "`write(|w| ..)` method takes [clk_conf::W](clk_conf::W) writer structure"]
impl crate::Writable for CLK_CONF {}
#[doc = "I2C_CLK_CONF"]
pub mod clk_conf;
#[doc = "I2C_COMD0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comd0](comd0) module"]
pub type COMD0 = crate::Reg<u32, _COMD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMD0;
#[doc = "`read()` method returns [comd0::R](comd0::R) reader structure"]
impl crate::Readable for COMD0 {}
#[doc = "`write(|w| ..)` method takes [comd0::W](comd0::W) writer structure"]
impl crate::Writable for COMD0 {}
#[doc = "I2C_COMD0"]
pub mod comd0;
#[doc = "I2C_COMD1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comd1](comd1) module"]
pub type COMD1 = crate::Reg<u32, _COMD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMD1;
#[doc = "`read()` method returns [comd1::R](comd1::R) reader structure"]
impl crate::Readable for COMD1 {}
#[doc = "`write(|w| ..)` method takes [comd1::W](comd1::W) writer structure"]
impl crate::Writable for COMD1 {}
#[doc = "I2C_COMD1"]
pub mod comd1;
#[doc = "I2C_COMD2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comd2](comd2) module"]
pub type COMD2 = crate::Reg<u32, _COMD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMD2;
#[doc = "`read()` method returns [comd2::R](comd2::R) reader structure"]
impl crate::Readable for COMD2 {}
#[doc = "`write(|w| ..)` method takes [comd2::W](comd2::W) writer structure"]
impl crate::Writable for COMD2 {}
#[doc = "I2C_COMD2"]
pub mod comd2;
#[doc = "I2C_COMD3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comd3](comd3) module"]
pub type COMD3 = crate::Reg<u32, _COMD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMD3;
#[doc = "`read()` method returns [comd3::R](comd3::R) reader structure"]
impl crate::Readable for COMD3 {}
#[doc = "`write(|w| ..)` method takes [comd3::W](comd3::W) writer structure"]
impl crate::Writable for COMD3 {}
#[doc = "I2C_COMD3"]
pub mod comd3;
#[doc = "I2C_COMD4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comd4](comd4) module"]
pub type COMD4 = crate::Reg<u32, _COMD4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMD4;
#[doc = "`read()` method returns [comd4::R](comd4::R) reader structure"]
impl crate::Readable for COMD4 {}
#[doc = "`write(|w| ..)` method takes [comd4::W](comd4::W) writer structure"]
impl crate::Writable for COMD4 {}
#[doc = "I2C_COMD4"]
pub mod comd4;
#[doc = "I2C_COMD5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comd5](comd5) module"]
pub type COMD5 = crate::Reg<u32, _COMD5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMD5;
#[doc = "`read()` method returns [comd5::R](comd5::R) reader structure"]
impl crate::Readable for COMD5 {}
#[doc = "`write(|w| ..)` method takes [comd5::W](comd5::W) writer structure"]
impl crate::Writable for COMD5 {}
#[doc = "I2C_COMD5"]
pub mod comd5;
#[doc = "I2C_COMD6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comd6](comd6) module"]
pub type COMD6 = crate::Reg<u32, _COMD6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMD6;
#[doc = "`read()` method returns [comd6::R](comd6::R) reader structure"]
impl crate::Readable for COMD6 {}
#[doc = "`write(|w| ..)` method takes [comd6::W](comd6::W) writer structure"]
impl crate::Writable for COMD6 {}
#[doc = "I2C_COMD6"]
pub mod comd6;
#[doc = "I2C_COMD7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comd7](comd7) module"]
pub type COMD7 = crate::Reg<u32, _COMD7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMD7;
#[doc = "`read()` method returns [comd7::R](comd7::R) reader structure"]
impl crate::Readable for COMD7 {}
#[doc = "`write(|w| ..)` method takes [comd7::W](comd7::W) writer structure"]
impl crate::Writable for COMD7 {}
#[doc = "I2C_COMD7"]
pub mod comd7;
#[doc = "I2C_SCL_ST_TIME_OUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_st_time_out](scl_st_time_out) module"]
pub type SCL_ST_TIME_OUT = crate::Reg<u32, _SCL_ST_TIME_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_ST_TIME_OUT;
#[doc = "`read()` method returns [scl_st_time_out::R](scl_st_time_out::R) reader structure"]
impl crate::Readable for SCL_ST_TIME_OUT {}
#[doc = "`write(|w| ..)` method takes [scl_st_time_out::W](scl_st_time_out::W) writer structure"]
impl crate::Writable for SCL_ST_TIME_OUT {}
#[doc = "I2C_SCL_ST_TIME_OUT"]
pub mod scl_st_time_out;
#[doc = "I2C_SCL_MAIN_ST_TIME_OUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_main_st_time_out](scl_main_st_time_out) module"]
pub type SCL_MAIN_ST_TIME_OUT = crate::Reg<u32, _SCL_MAIN_ST_TIME_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_MAIN_ST_TIME_OUT;
#[doc = "`read()` method returns [scl_main_st_time_out::R](scl_main_st_time_out::R) reader structure"]
impl crate::Readable for SCL_MAIN_ST_TIME_OUT {}
#[doc = "`write(|w| ..)` method takes [scl_main_st_time_out::W](scl_main_st_time_out::W) writer structure"]
impl crate::Writable for SCL_MAIN_ST_TIME_OUT {}
#[doc = "I2C_SCL_MAIN_ST_TIME_OUT"]
pub mod scl_main_st_time_out;
#[doc = "I2C_SCL_SP_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_sp_conf](scl_sp_conf) module"]
pub type SCL_SP_CONF = crate::Reg<u32, _SCL_SP_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_SP_CONF;
#[doc = "`read()` method returns [scl_sp_conf::R](scl_sp_conf::R) reader structure"]
impl crate::Readable for SCL_SP_CONF {}
#[doc = "`write(|w| ..)` method takes [scl_sp_conf::W](scl_sp_conf::W) writer structure"]
impl crate::Writable for SCL_SP_CONF {}
#[doc = "I2C_SCL_SP_CONF"]
pub mod scl_sp_conf;
#[doc = "I2C_SCL_STRETCH_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_stretch_conf](scl_stretch_conf) module"]
pub type SCL_STRETCH_CONF = crate::Reg<u32, _SCL_STRETCH_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_STRETCH_CONF;
#[doc = "`read()` method returns [scl_stretch_conf::R](scl_stretch_conf::R) reader structure"]
impl crate::Readable for SCL_STRETCH_CONF {}
#[doc = "`write(|w| ..)` method takes [scl_stretch_conf::W](scl_stretch_conf::W) writer structure"]
impl crate::Writable for SCL_STRETCH_CONF {}
#[doc = "I2C_SCL_STRETCH_CONF"]
pub mod scl_stretch_conf;
#[doc = "I2C_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "I2C_DATE"]
pub mod date;
