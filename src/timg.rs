#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIMG_T0CONFIG"]
    pub t0config: T0CONFIG,
    #[doc = "0x04 - TIMG_T0LO"]
    pub t0lo: T0LO,
    #[doc = "0x08 - TIMG_T0HI"]
    pub t0hi: T0HI,
    #[doc = "0x0c - TIMG_T0UPDATE"]
    pub t0update: T0UPDATE,
    #[doc = "0x10 - TIMG_T0ALARMLO"]
    pub t0alarmlo: T0ALARMLO,
    #[doc = "0x14 - TIMG_T0ALARMHI"]
    pub t0alarmhi: T0ALARMHI,
    #[doc = "0x18 - TIMG_T0LOADLO"]
    pub t0loadlo: T0LOADLO,
    #[doc = "0x1c - TIMG_T0LOADHI"]
    pub t0loadhi: T0LOADHI,
    #[doc = "0x20 - TIMG_T0LOAD"]
    pub t0load: T0LOAD,
    _reserved9: [u8; 36usize],
    #[doc = "0x48 - TIMG_WDTCONFIG0"]
    pub wdtconfig0: WDTCONFIG0,
    #[doc = "0x4c - TIMG_WDTCONFIG1"]
    pub wdtconfig1: WDTCONFIG1,
    #[doc = "0x50 - TIMG_WDTCONFIG2"]
    pub wdtconfig2: WDTCONFIG2,
    #[doc = "0x54 - TIMG_WDTCONFIG3"]
    pub wdtconfig3: WDTCONFIG3,
    #[doc = "0x58 - TIMG_WDTCONFIG4"]
    pub wdtconfig4: WDTCONFIG4,
    #[doc = "0x5c - TIMG_WDTCONFIG5"]
    pub wdtconfig5: WDTCONFIG5,
    #[doc = "0x60 - TIMG_WDTFEED"]
    pub wdtfeed: WDTFEED,
    #[doc = "0x64 - TIMG_WDTWPROTECT"]
    pub wdtwprotect: WDTWPROTECT,
    #[doc = "0x68 - TIMG_RTCCALICFG"]
    pub rtccalicfg: RTCCALICFG,
    #[doc = "0x6c - TIMG_RTCCALICFG1"]
    pub rtccalicfg1: RTCCALICFG1,
    #[doc = "0x70 - TIMG_INT_ENA_TIMERS"]
    pub int_ena_timers: INT_ENA_TIMERS,
    #[doc = "0x74 - TIMG_INT_RAW_TIMERS"]
    pub int_raw_timers: INT_RAW_TIMERS,
    #[doc = "0x78 - TIMG_INT_ST_TIMERS"]
    pub int_st_timers: INT_ST_TIMERS,
    #[doc = "0x7c - TIMG_INT_CLR_TIMERS"]
    pub int_clr_timers: INT_CLR_TIMERS,
    #[doc = "0x80 - TIMG_RTCCALICFG2"]
    pub rtccalicfg2: RTCCALICFG2,
    _reserved24: [u8; 116usize],
    #[doc = "0xf8 - TIMG_NTIMERS_DATE"]
    pub ntimers_date: NTIMERS_DATE,
    #[doc = "0xfc - TIMG_CLK"]
    pub clk: CLK,
}
#[doc = "TIMG_T0CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0config](t0config) module"]
pub type T0CONFIG = crate::Reg<u32, _T0CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0CONFIG;
#[doc = "`read()` method returns [t0config::R](t0config::R) reader structure"]
impl crate::Readable for T0CONFIG {}
#[doc = "`write(|w| ..)` method takes [t0config::W](t0config::W) writer structure"]
impl crate::Writable for T0CONFIG {}
#[doc = "TIMG_T0CONFIG"]
pub mod t0config;
#[doc = "TIMG_T0LO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0lo](t0lo) module"]
pub type T0LO = crate::Reg<u32, _T0LO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0LO;
#[doc = "`read()` method returns [t0lo::R](t0lo::R) reader structure"]
impl crate::Readable for T0LO {}
#[doc = "TIMG_T0LO"]
pub mod t0lo;
#[doc = "TIMG_T0HI\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0hi](t0hi) module"]
pub type T0HI = crate::Reg<u32, _T0HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0HI;
#[doc = "`read()` method returns [t0hi::R](t0hi::R) reader structure"]
impl crate::Readable for T0HI {}
#[doc = "TIMG_T0HI"]
pub mod t0hi;
#[doc = "TIMG_T0UPDATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0update](t0update) module"]
pub type T0UPDATE = crate::Reg<u32, _T0UPDATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0UPDATE;
#[doc = "`read()` method returns [t0update::R](t0update::R) reader structure"]
impl crate::Readable for T0UPDATE {}
#[doc = "`write(|w| ..)` method takes [t0update::W](t0update::W) writer structure"]
impl crate::Writable for T0UPDATE {}
#[doc = "TIMG_T0UPDATE"]
pub mod t0update;
#[doc = "TIMG_T0ALARMLO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0alarmlo](t0alarmlo) module"]
pub type T0ALARMLO = crate::Reg<u32, _T0ALARMLO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0ALARMLO;
#[doc = "`read()` method returns [t0alarmlo::R](t0alarmlo::R) reader structure"]
impl crate::Readable for T0ALARMLO {}
#[doc = "`write(|w| ..)` method takes [t0alarmlo::W](t0alarmlo::W) writer structure"]
impl crate::Writable for T0ALARMLO {}
#[doc = "TIMG_T0ALARMLO"]
pub mod t0alarmlo;
#[doc = "TIMG_T0ALARMHI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0alarmhi](t0alarmhi) module"]
pub type T0ALARMHI = crate::Reg<u32, _T0ALARMHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0ALARMHI;
#[doc = "`read()` method returns [t0alarmhi::R](t0alarmhi::R) reader structure"]
impl crate::Readable for T0ALARMHI {}
#[doc = "`write(|w| ..)` method takes [t0alarmhi::W](t0alarmhi::W) writer structure"]
impl crate::Writable for T0ALARMHI {}
#[doc = "TIMG_T0ALARMHI"]
pub mod t0alarmhi;
#[doc = "TIMG_T0LOADLO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0loadlo](t0loadlo) module"]
pub type T0LOADLO = crate::Reg<u32, _T0LOADLO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0LOADLO;
#[doc = "`read()` method returns [t0loadlo::R](t0loadlo::R) reader structure"]
impl crate::Readable for T0LOADLO {}
#[doc = "`write(|w| ..)` method takes [t0loadlo::W](t0loadlo::W) writer structure"]
impl crate::Writable for T0LOADLO {}
#[doc = "TIMG_T0LOADLO"]
pub mod t0loadlo;
#[doc = "TIMG_T0LOADHI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0loadhi](t0loadhi) module"]
pub type T0LOADHI = crate::Reg<u32, _T0LOADHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0LOADHI;
#[doc = "`read()` method returns [t0loadhi::R](t0loadhi::R) reader structure"]
impl crate::Readable for T0LOADHI {}
#[doc = "`write(|w| ..)` method takes [t0loadhi::W](t0loadhi::W) writer structure"]
impl crate::Writable for T0LOADHI {}
#[doc = "TIMG_T0LOADHI"]
pub mod t0loadhi;
#[doc = "TIMG_T0LOAD\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0load](t0load) module"]
pub type T0LOAD = crate::Reg<u32, _T0LOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0LOAD;
#[doc = "`write(|w| ..)` method takes [t0load::W](t0load::W) writer structure"]
impl crate::Writable for T0LOAD {}
#[doc = "TIMG_T0LOAD"]
pub mod t0load;
#[doc = "TIMG_WDTCONFIG0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconfig0](wdtconfig0) module"]
pub type WDTCONFIG0 = crate::Reg<u32, _WDTCONFIG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONFIG0;
#[doc = "`read()` method returns [wdtconfig0::R](wdtconfig0::R) reader structure"]
impl crate::Readable for WDTCONFIG0 {}
#[doc = "`write(|w| ..)` method takes [wdtconfig0::W](wdtconfig0::W) writer structure"]
impl crate::Writable for WDTCONFIG0 {}
#[doc = "TIMG_WDTCONFIG0"]
pub mod wdtconfig0;
#[doc = "TIMG_WDTCONFIG1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconfig1](wdtconfig1) module"]
pub type WDTCONFIG1 = crate::Reg<u32, _WDTCONFIG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONFIG1;
#[doc = "`read()` method returns [wdtconfig1::R](wdtconfig1::R) reader structure"]
impl crate::Readable for WDTCONFIG1 {}
#[doc = "`write(|w| ..)` method takes [wdtconfig1::W](wdtconfig1::W) writer structure"]
impl crate::Writable for WDTCONFIG1 {}
#[doc = "TIMG_WDTCONFIG1"]
pub mod wdtconfig1;
#[doc = "TIMG_WDTCONFIG2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconfig2](wdtconfig2) module"]
pub type WDTCONFIG2 = crate::Reg<u32, _WDTCONFIG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONFIG2;
#[doc = "`read()` method returns [wdtconfig2::R](wdtconfig2::R) reader structure"]
impl crate::Readable for WDTCONFIG2 {}
#[doc = "`write(|w| ..)` method takes [wdtconfig2::W](wdtconfig2::W) writer structure"]
impl crate::Writable for WDTCONFIG2 {}
#[doc = "TIMG_WDTCONFIG2"]
pub mod wdtconfig2;
#[doc = "TIMG_WDTCONFIG3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconfig3](wdtconfig3) module"]
pub type WDTCONFIG3 = crate::Reg<u32, _WDTCONFIG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONFIG3;
#[doc = "`read()` method returns [wdtconfig3::R](wdtconfig3::R) reader structure"]
impl crate::Readable for WDTCONFIG3 {}
#[doc = "`write(|w| ..)` method takes [wdtconfig3::W](wdtconfig3::W) writer structure"]
impl crate::Writable for WDTCONFIG3 {}
#[doc = "TIMG_WDTCONFIG3"]
pub mod wdtconfig3;
#[doc = "TIMG_WDTCONFIG4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconfig4](wdtconfig4) module"]
pub type WDTCONFIG4 = crate::Reg<u32, _WDTCONFIG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONFIG4;
#[doc = "`read()` method returns [wdtconfig4::R](wdtconfig4::R) reader structure"]
impl crate::Readable for WDTCONFIG4 {}
#[doc = "`write(|w| ..)` method takes [wdtconfig4::W](wdtconfig4::W) writer structure"]
impl crate::Writable for WDTCONFIG4 {}
#[doc = "TIMG_WDTCONFIG4"]
pub mod wdtconfig4;
#[doc = "TIMG_WDTCONFIG5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconfig5](wdtconfig5) module"]
pub type WDTCONFIG5 = crate::Reg<u32, _WDTCONFIG5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONFIG5;
#[doc = "`read()` method returns [wdtconfig5::R](wdtconfig5::R) reader structure"]
impl crate::Readable for WDTCONFIG5 {}
#[doc = "`write(|w| ..)` method takes [wdtconfig5::W](wdtconfig5::W) writer structure"]
impl crate::Writable for WDTCONFIG5 {}
#[doc = "TIMG_WDTCONFIG5"]
pub mod wdtconfig5;
#[doc = "TIMG_WDTFEED\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtfeed](wdtfeed) module"]
pub type WDTFEED = crate::Reg<u32, _WDTFEED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTFEED;
#[doc = "`write(|w| ..)` method takes [wdtfeed::W](wdtfeed::W) writer structure"]
impl crate::Writable for WDTFEED {}
#[doc = "TIMG_WDTFEED"]
pub mod wdtfeed;
#[doc = "TIMG_WDTWPROTECT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtwprotect](wdtwprotect) module"]
pub type WDTWPROTECT = crate::Reg<u32, _WDTWPROTECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTWPROTECT;
#[doc = "`read()` method returns [wdtwprotect::R](wdtwprotect::R) reader structure"]
impl crate::Readable for WDTWPROTECT {}
#[doc = "`write(|w| ..)` method takes [wdtwprotect::W](wdtwprotect::W) writer structure"]
impl crate::Writable for WDTWPROTECT {}
#[doc = "TIMG_WDTWPROTECT"]
pub mod wdtwprotect;
#[doc = "TIMG_RTCCALICFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccalicfg](rtccalicfg) module"]
pub type RTCCALICFG = crate::Reg<u32, _RTCCALICFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCALICFG;
#[doc = "`read()` method returns [rtccalicfg::R](rtccalicfg::R) reader structure"]
impl crate::Readable for RTCCALICFG {}
#[doc = "`write(|w| ..)` method takes [rtccalicfg::W](rtccalicfg::W) writer structure"]
impl crate::Writable for RTCCALICFG {}
#[doc = "TIMG_RTCCALICFG"]
pub mod rtccalicfg;
#[doc = "TIMG_RTCCALICFG1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccalicfg1](rtccalicfg1) module"]
pub type RTCCALICFG1 = crate::Reg<u32, _RTCCALICFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCALICFG1;
#[doc = "`read()` method returns [rtccalicfg1::R](rtccalicfg1::R) reader structure"]
impl crate::Readable for RTCCALICFG1 {}
#[doc = "TIMG_RTCCALICFG1"]
pub mod rtccalicfg1;
#[doc = "TIMG_INT_ENA_TIMERS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena_timers](int_ena_timers) module"]
pub type INT_ENA_TIMERS = crate::Reg<u32, _INT_ENA_TIMERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ENA_TIMERS;
#[doc = "`read()` method returns [int_ena_timers::R](int_ena_timers::R) reader structure"]
impl crate::Readable for INT_ENA_TIMERS {}
#[doc = "`write(|w| ..)` method takes [int_ena_timers::W](int_ena_timers::W) writer structure"]
impl crate::Writable for INT_ENA_TIMERS {}
#[doc = "TIMG_INT_ENA_TIMERS"]
pub mod int_ena_timers;
#[doc = "TIMG_INT_RAW_TIMERS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw_timers](int_raw_timers) module"]
pub type INT_RAW_TIMERS = crate::Reg<u32, _INT_RAW_TIMERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_RAW_TIMERS;
#[doc = "`read()` method returns [int_raw_timers::R](int_raw_timers::R) reader structure"]
impl crate::Readable for INT_RAW_TIMERS {}
#[doc = "`write(|w| ..)` method takes [int_raw_timers::W](int_raw_timers::W) writer structure"]
impl crate::Writable for INT_RAW_TIMERS {}
#[doc = "TIMG_INT_RAW_TIMERS"]
pub mod int_raw_timers;
#[doc = "TIMG_INT_ST_TIMERS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st_timers](int_st_timers) module"]
pub type INT_ST_TIMERS = crate::Reg<u32, _INT_ST_TIMERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ST_TIMERS;
#[doc = "`read()` method returns [int_st_timers::R](int_st_timers::R) reader structure"]
impl crate::Readable for INT_ST_TIMERS {}
#[doc = "TIMG_INT_ST_TIMERS"]
pub mod int_st_timers;
#[doc = "TIMG_INT_CLR_TIMERS\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr_timers](int_clr_timers) module"]
pub type INT_CLR_TIMERS = crate::Reg<u32, _INT_CLR_TIMERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CLR_TIMERS;
#[doc = "`write(|w| ..)` method takes [int_clr_timers::W](int_clr_timers::W) writer structure"]
impl crate::Writable for INT_CLR_TIMERS {}
#[doc = "TIMG_INT_CLR_TIMERS"]
pub mod int_clr_timers;
#[doc = "TIMG_RTCCALICFG2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccalicfg2](rtccalicfg2) module"]
pub type RTCCALICFG2 = crate::Reg<u32, _RTCCALICFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCALICFG2;
#[doc = "`read()` method returns [rtccalicfg2::R](rtccalicfg2::R) reader structure"]
impl crate::Readable for RTCCALICFG2 {}
#[doc = "`write(|w| ..)` method takes [rtccalicfg2::W](rtccalicfg2::W) writer structure"]
impl crate::Writable for RTCCALICFG2 {}
#[doc = "TIMG_RTCCALICFG2"]
pub mod rtccalicfg2;
#[doc = "TIMG_NTIMERS_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ntimers_date](ntimers_date) module"]
pub type NTIMERS_DATE = crate::Reg<u32, _NTIMERS_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NTIMERS_DATE;
#[doc = "`read()` method returns [ntimers_date::R](ntimers_date::R) reader structure"]
impl crate::Readable for NTIMERS_DATE {}
#[doc = "`write(|w| ..)` method takes [ntimers_date::W](ntimers_date::W) writer structure"]
impl crate::Writable for NTIMERS_DATE {}
#[doc = "TIMG_NTIMERS_DATE"]
pub mod ntimers_date;
#[doc = "TIMG_CLK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk](clk) module"]
pub type CLK = crate::Reg<u32, _CLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK;
#[doc = "`read()` method returns [clk::R](clk::R) reader structure"]
impl crate::Readable for CLK {}
#[doc = "`write(|w| ..)` method takes [clk::W](clk::W) writer structure"]
impl crate::Writable for CLK {}
#[doc = "TIMG_CLK"]
pub mod clk;
