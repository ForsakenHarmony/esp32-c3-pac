#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC_I2C_SCL_LOW_PERIOD"]
    pub scl_low_period: SCL_LOW_PERIOD,
    #[doc = "0x04 - RTC_I2C_CTRL"]
    pub ctrl: CTRL,
    #[doc = "0x08 - RTC_I2C_STATUS"]
    pub status: STATUS,
    #[doc = "0x0c - RTC_I2C_TIMEOUT"]
    pub timeout: TIMEOUT,
    #[doc = "0x10 - RTC_I2C_SLAVE_ADDR"]
    pub slave_addr: SLAVE_ADDR,
    #[doc = "0x14 - RTC_I2C_SCL_HIGH"]
    pub scl_high: SCL_HIGH,
    #[doc = "0x18 - RTC_I2C_SDA_DUTY"]
    pub sda_duty: SDA_DUTY,
    #[doc = "0x1c - RTC_I2C_SCL_START_PERIOD"]
    pub scl_start_period: SCL_START_PERIOD,
    #[doc = "0x20 - RTC_I2C_SCL_STOP_PERIOD"]
    pub scl_stop_period: SCL_STOP_PERIOD,
    #[doc = "0x24 - RTC_I2C_INT_CLR"]
    pub int_clr: INT_CLR,
    #[doc = "0x28 - RTC_I2C_INT_RAW"]
    pub int_raw: INT_RAW,
    #[doc = "0x2c - RTC_I2C_INT_ST"]
    pub int_st: INT_ST,
    #[doc = "0x30 - RTC_I2C_INT_ENA"]
    pub int_ena: INT_ENA,
    #[doc = "0x34 - RTC_I2C_DATA"]
    pub data: DATA,
    #[doc = "0x38 - RTC_I2C_CMD0"]
    pub cmd0: CMD0,
    #[doc = "0x3c - RTC_I2C_CMD1"]
    pub cmd1: CMD1,
    #[doc = "0x40 - RTC_I2C_CMD2"]
    pub cmd2: CMD2,
    #[doc = "0x44 - RTC_I2C_CMD3"]
    pub cmd3: CMD3,
    #[doc = "0x48 - RTC_I2C_CMD4"]
    pub cmd4: CMD4,
    #[doc = "0x4c - RTC_I2C_CMD5"]
    pub cmd5: CMD5,
    #[doc = "0x50 - RTC_I2C_CMD6"]
    pub cmd6: CMD6,
    #[doc = "0x54 - RTC_I2C_CMD7"]
    pub cmd7: CMD7,
    #[doc = "0x58 - RTC_I2C_CMD8"]
    pub cmd8: CMD8,
    #[doc = "0x5c - RTC_I2C_CMD9"]
    pub cmd9: CMD9,
    #[doc = "0x60 - RTC_I2C_CMD10"]
    pub cmd10: CMD10,
    #[doc = "0x64 - RTC_I2C_CMD11"]
    pub cmd11: CMD11,
    #[doc = "0x68 - RTC_I2C_CMD12"]
    pub cmd12: CMD12,
    #[doc = "0x6c - RTC_I2C_CMD13"]
    pub cmd13: CMD13,
    #[doc = "0x70 - RTC_I2C_CMD14"]
    pub cmd14: CMD14,
    #[doc = "0x74 - RTC_I2C_CMD15"]
    pub cmd15: CMD15,
    _reserved30: [u8; 132usize],
    #[doc = "0xfc - RTC_I2C_DATE"]
    pub date: DATE,
}
#[doc = "RTC_I2C_SCL_LOW_PERIOD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_low_period](scl_low_period) module"]
pub type SCL_LOW_PERIOD = crate::Reg<u32, _SCL_LOW_PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_LOW_PERIOD;
#[doc = "`read()` method returns [scl_low_period::R](scl_low_period::R) reader structure"]
impl crate::Readable for SCL_LOW_PERIOD {}
#[doc = "`write(|w| ..)` method takes [scl_low_period::W](scl_low_period::W) writer structure"]
impl crate::Writable for SCL_LOW_PERIOD {}
#[doc = "RTC_I2C_SCL_LOW_PERIOD"]
pub mod scl_low_period;
#[doc = "RTC_I2C_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "RTC_I2C_CTRL"]
pub mod ctrl;
#[doc = "RTC_I2C_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "RTC_I2C_STATUS"]
pub mod status;
#[doc = "RTC_I2C_TIMEOUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timeout](timeout) module"]
pub type TIMEOUT = crate::Reg<u32, _TIMEOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMEOUT;
#[doc = "`read()` method returns [timeout::R](timeout::R) reader structure"]
impl crate::Readable for TIMEOUT {}
#[doc = "`write(|w| ..)` method takes [timeout::W](timeout::W) writer structure"]
impl crate::Writable for TIMEOUT {}
#[doc = "RTC_I2C_TIMEOUT"]
pub mod timeout;
#[doc = "RTC_I2C_SLAVE_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave_addr](slave_addr) module"]
pub type SLAVE_ADDR = crate::Reg<u32, _SLAVE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLAVE_ADDR;
#[doc = "`read()` method returns [slave_addr::R](slave_addr::R) reader structure"]
impl crate::Readable for SLAVE_ADDR {}
#[doc = "`write(|w| ..)` method takes [slave_addr::W](slave_addr::W) writer structure"]
impl crate::Writable for SLAVE_ADDR {}
#[doc = "RTC_I2C_SLAVE_ADDR"]
pub mod slave_addr;
#[doc = "RTC_I2C_SCL_HIGH\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_high](scl_high) module"]
pub type SCL_HIGH = crate::Reg<u32, _SCL_HIGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_HIGH;
#[doc = "`read()` method returns [scl_high::R](scl_high::R) reader structure"]
impl crate::Readable for SCL_HIGH {}
#[doc = "`write(|w| ..)` method takes [scl_high::W](scl_high::W) writer structure"]
impl crate::Writable for SCL_HIGH {}
#[doc = "RTC_I2C_SCL_HIGH"]
pub mod scl_high;
#[doc = "RTC_I2C_SDA_DUTY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sda_duty](sda_duty) module"]
pub type SDA_DUTY = crate::Reg<u32, _SDA_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDA_DUTY;
#[doc = "`read()` method returns [sda_duty::R](sda_duty::R) reader structure"]
impl crate::Readable for SDA_DUTY {}
#[doc = "`write(|w| ..)` method takes [sda_duty::W](sda_duty::W) writer structure"]
impl crate::Writable for SDA_DUTY {}
#[doc = "RTC_I2C_SDA_DUTY"]
pub mod sda_duty;
#[doc = "RTC_I2C_SCL_START_PERIOD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_start_period](scl_start_period) module"]
pub type SCL_START_PERIOD = crate::Reg<u32, _SCL_START_PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_START_PERIOD;
#[doc = "`read()` method returns [scl_start_period::R](scl_start_period::R) reader structure"]
impl crate::Readable for SCL_START_PERIOD {}
#[doc = "`write(|w| ..)` method takes [scl_start_period::W](scl_start_period::W) writer structure"]
impl crate::Writable for SCL_START_PERIOD {}
#[doc = "RTC_I2C_SCL_START_PERIOD"]
pub mod scl_start_period;
#[doc = "RTC_I2C_SCL_STOP_PERIOD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_stop_period](scl_stop_period) module"]
pub type SCL_STOP_PERIOD = crate::Reg<u32, _SCL_STOP_PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCL_STOP_PERIOD;
#[doc = "`read()` method returns [scl_stop_period::R](scl_stop_period::R) reader structure"]
impl crate::Readable for SCL_STOP_PERIOD {}
#[doc = "`write(|w| ..)` method takes [scl_stop_period::W](scl_stop_period::W) writer structure"]
impl crate::Writable for SCL_STOP_PERIOD {}
#[doc = "RTC_I2C_SCL_STOP_PERIOD"]
pub mod scl_stop_period;
#[doc = "RTC_I2C_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](int_clr) module"]
pub type INT_CLR = crate::Reg<u32, _INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CLR;
#[doc = "`write(|w| ..)` method takes [int_clr::W](int_clr::W) writer structure"]
impl crate::Writable for INT_CLR {}
#[doc = "RTC_I2C_INT_CLR"]
pub mod int_clr;
#[doc = "RTC_I2C_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](int_raw) module"]
pub type INT_RAW = crate::Reg<u32, _INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_RAW;
#[doc = "`read()` method returns [int_raw::R](int_raw::R) reader structure"]
impl crate::Readable for INT_RAW {}
#[doc = "RTC_I2C_INT_RAW"]
pub mod int_raw;
#[doc = "RTC_I2C_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](int_st) module"]
pub type INT_ST = crate::Reg<u32, _INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ST;
#[doc = "`read()` method returns [int_st::R](int_st::R) reader structure"]
impl crate::Readable for INT_ST {}
#[doc = "RTC_I2C_INT_ST"]
pub mod int_st;
#[doc = "RTC_I2C_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](int_ena) module"]
pub type INT_ENA = crate::Reg<u32, _INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ENA;
#[doc = "`read()` method returns [int_ena::R](int_ena::R) reader structure"]
impl crate::Readable for INT_ENA {}
#[doc = "`write(|w| ..)` method takes [int_ena::W](int_ena::W) writer structure"]
impl crate::Writable for INT_ENA {}
#[doc = "RTC_I2C_INT_ENA"]
pub mod int_ena;
#[doc = "RTC_I2C_DATA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "RTC_I2C_DATA"]
pub mod data;
#[doc = "RTC_I2C_CMD0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd0](cmd0) module"]
pub type CMD0 = crate::Reg<u32, _CMD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD0;
#[doc = "`read()` method returns [cmd0::R](cmd0::R) reader structure"]
impl crate::Readable for CMD0 {}
#[doc = "`write(|w| ..)` method takes [cmd0::W](cmd0::W) writer structure"]
impl crate::Writable for CMD0 {}
#[doc = "RTC_I2C_CMD0"]
pub mod cmd0;
#[doc = "RTC_I2C_CMD1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd1](cmd1) module"]
pub type CMD1 = crate::Reg<u32, _CMD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD1;
#[doc = "`read()` method returns [cmd1::R](cmd1::R) reader structure"]
impl crate::Readable for CMD1 {}
#[doc = "`write(|w| ..)` method takes [cmd1::W](cmd1::W) writer structure"]
impl crate::Writable for CMD1 {}
#[doc = "RTC_I2C_CMD1"]
pub mod cmd1;
#[doc = "RTC_I2C_CMD2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd2](cmd2) module"]
pub type CMD2 = crate::Reg<u32, _CMD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD2;
#[doc = "`read()` method returns [cmd2::R](cmd2::R) reader structure"]
impl crate::Readable for CMD2 {}
#[doc = "`write(|w| ..)` method takes [cmd2::W](cmd2::W) writer structure"]
impl crate::Writable for CMD2 {}
#[doc = "RTC_I2C_CMD2"]
pub mod cmd2;
#[doc = "RTC_I2C_CMD3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd3](cmd3) module"]
pub type CMD3 = crate::Reg<u32, _CMD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD3;
#[doc = "`read()` method returns [cmd3::R](cmd3::R) reader structure"]
impl crate::Readable for CMD3 {}
#[doc = "`write(|w| ..)` method takes [cmd3::W](cmd3::W) writer structure"]
impl crate::Writable for CMD3 {}
#[doc = "RTC_I2C_CMD3"]
pub mod cmd3;
#[doc = "RTC_I2C_CMD4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd4](cmd4) module"]
pub type CMD4 = crate::Reg<u32, _CMD4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD4;
#[doc = "`read()` method returns [cmd4::R](cmd4::R) reader structure"]
impl crate::Readable for CMD4 {}
#[doc = "`write(|w| ..)` method takes [cmd4::W](cmd4::W) writer structure"]
impl crate::Writable for CMD4 {}
#[doc = "RTC_I2C_CMD4"]
pub mod cmd4;
#[doc = "RTC_I2C_CMD5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd5](cmd5) module"]
pub type CMD5 = crate::Reg<u32, _CMD5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD5;
#[doc = "`read()` method returns [cmd5::R](cmd5::R) reader structure"]
impl crate::Readable for CMD5 {}
#[doc = "`write(|w| ..)` method takes [cmd5::W](cmd5::W) writer structure"]
impl crate::Writable for CMD5 {}
#[doc = "RTC_I2C_CMD5"]
pub mod cmd5;
#[doc = "RTC_I2C_CMD6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd6](cmd6) module"]
pub type CMD6 = crate::Reg<u32, _CMD6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD6;
#[doc = "`read()` method returns [cmd6::R](cmd6::R) reader structure"]
impl crate::Readable for CMD6 {}
#[doc = "`write(|w| ..)` method takes [cmd6::W](cmd6::W) writer structure"]
impl crate::Writable for CMD6 {}
#[doc = "RTC_I2C_CMD6"]
pub mod cmd6;
#[doc = "RTC_I2C_CMD7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd7](cmd7) module"]
pub type CMD7 = crate::Reg<u32, _CMD7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD7;
#[doc = "`read()` method returns [cmd7::R](cmd7::R) reader structure"]
impl crate::Readable for CMD7 {}
#[doc = "`write(|w| ..)` method takes [cmd7::W](cmd7::W) writer structure"]
impl crate::Writable for CMD7 {}
#[doc = "RTC_I2C_CMD7"]
pub mod cmd7;
#[doc = "RTC_I2C_CMD8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd8](cmd8) module"]
pub type CMD8 = crate::Reg<u32, _CMD8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD8;
#[doc = "`read()` method returns [cmd8::R](cmd8::R) reader structure"]
impl crate::Readable for CMD8 {}
#[doc = "`write(|w| ..)` method takes [cmd8::W](cmd8::W) writer structure"]
impl crate::Writable for CMD8 {}
#[doc = "RTC_I2C_CMD8"]
pub mod cmd8;
#[doc = "RTC_I2C_CMD9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd9](cmd9) module"]
pub type CMD9 = crate::Reg<u32, _CMD9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD9;
#[doc = "`read()` method returns [cmd9::R](cmd9::R) reader structure"]
impl crate::Readable for CMD9 {}
#[doc = "`write(|w| ..)` method takes [cmd9::W](cmd9::W) writer structure"]
impl crate::Writable for CMD9 {}
#[doc = "RTC_I2C_CMD9"]
pub mod cmd9;
#[doc = "RTC_I2C_CMD10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd10](cmd10) module"]
pub type CMD10 = crate::Reg<u32, _CMD10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD10;
#[doc = "`read()` method returns [cmd10::R](cmd10::R) reader structure"]
impl crate::Readable for CMD10 {}
#[doc = "`write(|w| ..)` method takes [cmd10::W](cmd10::W) writer structure"]
impl crate::Writable for CMD10 {}
#[doc = "RTC_I2C_CMD10"]
pub mod cmd10;
#[doc = "RTC_I2C_CMD11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd11](cmd11) module"]
pub type CMD11 = crate::Reg<u32, _CMD11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD11;
#[doc = "`read()` method returns [cmd11::R](cmd11::R) reader structure"]
impl crate::Readable for CMD11 {}
#[doc = "`write(|w| ..)` method takes [cmd11::W](cmd11::W) writer structure"]
impl crate::Writable for CMD11 {}
#[doc = "RTC_I2C_CMD11"]
pub mod cmd11;
#[doc = "RTC_I2C_CMD12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd12](cmd12) module"]
pub type CMD12 = crate::Reg<u32, _CMD12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD12;
#[doc = "`read()` method returns [cmd12::R](cmd12::R) reader structure"]
impl crate::Readable for CMD12 {}
#[doc = "`write(|w| ..)` method takes [cmd12::W](cmd12::W) writer structure"]
impl crate::Writable for CMD12 {}
#[doc = "RTC_I2C_CMD12"]
pub mod cmd12;
#[doc = "RTC_I2C_CMD13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd13](cmd13) module"]
pub type CMD13 = crate::Reg<u32, _CMD13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD13;
#[doc = "`read()` method returns [cmd13::R](cmd13::R) reader structure"]
impl crate::Readable for CMD13 {}
#[doc = "`write(|w| ..)` method takes [cmd13::W](cmd13::W) writer structure"]
impl crate::Writable for CMD13 {}
#[doc = "RTC_I2C_CMD13"]
pub mod cmd13;
#[doc = "RTC_I2C_CMD14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd14](cmd14) module"]
pub type CMD14 = crate::Reg<u32, _CMD14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD14;
#[doc = "`read()` method returns [cmd14::R](cmd14::R) reader structure"]
impl crate::Readable for CMD14 {}
#[doc = "`write(|w| ..)` method takes [cmd14::W](cmd14::W) writer structure"]
impl crate::Writable for CMD14 {}
#[doc = "RTC_I2C_CMD14"]
pub mod cmd14;
#[doc = "RTC_I2C_CMD15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd15](cmd15) module"]
pub type CMD15 = crate::Reg<u32, _CMD15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD15;
#[doc = "`read()` method returns [cmd15::R](cmd15::R) reader structure"]
impl crate::Readable for CMD15 {}
#[doc = "`write(|w| ..)` method takes [cmd15::W](cmd15::W) writer structure"]
impl crate::Writable for CMD15 {}
#[doc = "RTC_I2C_CMD15"]
pub mod cmd15;
#[doc = "RTC_I2C_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "RTC_I2C_DATE"]
pub mod date;
