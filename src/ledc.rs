#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LEDC_LSCH0_CONF0"]
    pub lsch0_conf0: LSCH0_CONF0,
    #[doc = "0x04 - LEDC_LSCH0_HPOINT"]
    pub lsch0_hpoint: LSCH0_HPOINT,
    #[doc = "0x08 - LEDC_LSCH0_DUTY"]
    pub lsch0_duty: LSCH0_DUTY,
    #[doc = "0x0c - LEDC_LSCH0_CONF1"]
    pub lsch0_conf1: LSCH0_CONF1,
    #[doc = "0x10 - LEDC_LSCH0_DUTY_R"]
    pub lsch0_duty_r: LSCH0_DUTY_R,
    #[doc = "0x14 - LEDC_LSCH1_CONF0"]
    pub lsch1_conf0: LSCH1_CONF0,
    #[doc = "0x18 - LEDC_LSCH1_HPOINT"]
    pub lsch1_hpoint: LSCH1_HPOINT,
    #[doc = "0x1c - LEDC_LSCH1_DUTY"]
    pub lsch1_duty: LSCH1_DUTY,
    #[doc = "0x20 - LEDC_LSCH1_CONF1"]
    pub lsch1_conf1: LSCH1_CONF1,
    #[doc = "0x24 - LEDC_LSCH1_DUTY_R"]
    pub lsch1_duty_r: LSCH1_DUTY_R,
    #[doc = "0x28 - LEDC_LSCH2_CONF0"]
    pub lsch2_conf0: LSCH2_CONF0,
    #[doc = "0x2c - LEDC_LSCH2_HPOINT"]
    pub lsch2_hpoint: LSCH2_HPOINT,
    #[doc = "0x30 - LEDC_LSCH2_DUTY"]
    pub lsch2_duty: LSCH2_DUTY,
    #[doc = "0x34 - LEDC_LSCH2_CONF1"]
    pub lsch2_conf1: LSCH2_CONF1,
    #[doc = "0x38 - LEDC_LSCH2_DUTY_R"]
    pub lsch2_duty_r: LSCH2_DUTY_R,
    #[doc = "0x3c - LEDC_LSCH3_CONF0"]
    pub lsch3_conf0: LSCH3_CONF0,
    #[doc = "0x40 - LEDC_LSCH3_HPOINT"]
    pub lsch3_hpoint: LSCH3_HPOINT,
    #[doc = "0x44 - LEDC_LSCH3_DUTY"]
    pub lsch3_duty: LSCH3_DUTY,
    #[doc = "0x48 - LEDC_LSCH3_CONF1"]
    pub lsch3_conf1: LSCH3_CONF1,
    #[doc = "0x4c - LEDC_LSCH3_DUTY_R"]
    pub lsch3_duty_r: LSCH3_DUTY_R,
    #[doc = "0x50 - LEDC_LSCH4_CONF0"]
    pub lsch4_conf0: LSCH4_CONF0,
    #[doc = "0x54 - LEDC_LSCH4_HPOINT"]
    pub lsch4_hpoint: LSCH4_HPOINT,
    #[doc = "0x58 - LEDC_LSCH4_DUTY"]
    pub lsch4_duty: LSCH4_DUTY,
    #[doc = "0x5c - LEDC_LSCH4_CONF1"]
    pub lsch4_conf1: LSCH4_CONF1,
    #[doc = "0x60 - LEDC_LSCH4_DUTY_R"]
    pub lsch4_duty_r: LSCH4_DUTY_R,
    #[doc = "0x64 - LEDC_LSCH5_CONF0"]
    pub lsch5_conf0: LSCH5_CONF0,
    #[doc = "0x68 - LEDC_LSCH5_HPOINT"]
    pub lsch5_hpoint: LSCH5_HPOINT,
    #[doc = "0x6c - LEDC_LSCH5_DUTY"]
    pub lsch5_duty: LSCH5_DUTY,
    #[doc = "0x70 - LEDC_LSCH5_CONF1"]
    pub lsch5_conf1: LSCH5_CONF1,
    #[doc = "0x74 - LEDC_LSCH5_DUTY_R"]
    pub lsch5_duty_r: LSCH5_DUTY_R,
    _reserved30: [u8; 40usize],
    #[doc = "0xa0 - LEDC_LSTIMER0_CONF"]
    pub lstimer0_conf: LSTIMER0_CONF,
    #[doc = "0xa4 - LEDC_LSTIMER0_VALUE"]
    pub lstimer0_value: LSTIMER0_VALUE,
    #[doc = "0xa8 - LEDC_LSTIMER1_CONF"]
    pub lstimer1_conf: LSTIMER1_CONF,
    #[doc = "0xac - LEDC_LSTIMER1_VALUE"]
    pub lstimer1_value: LSTIMER1_VALUE,
    #[doc = "0xb0 - LEDC_LSTIMER2_CONF"]
    pub lstimer2_conf: LSTIMER2_CONF,
    #[doc = "0xb4 - LEDC_LSTIMER2_VALUE"]
    pub lstimer2_value: LSTIMER2_VALUE,
    #[doc = "0xb8 - LEDC_LSTIMER3_CONF"]
    pub lstimer3_conf: LSTIMER3_CONF,
    #[doc = "0xbc - LEDC_LSTIMER3_VALUE"]
    pub lstimer3_value: LSTIMER3_VALUE,
    #[doc = "0xc0 - LEDC_INT_RAW"]
    pub int_raw: INT_RAW,
    #[doc = "0xc4 - LEDC_INT_ST"]
    pub int_st: INT_ST,
    #[doc = "0xc8 - LEDC_INT_ENA"]
    pub int_ena: INT_ENA,
    #[doc = "0xcc - LEDC_INT_CLR"]
    pub int_clr: INT_CLR,
    #[doc = "0xd0 - LEDC_CONF"]
    pub conf: CONF,
    _reserved43: [u8; 40usize],
    #[doc = "0xfc - LEDC_DATE"]
    pub date: DATE,
}
#[doc = "LEDC_LSCH0_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch0_conf0](lsch0_conf0) module"]
pub type LSCH0_CONF0 = crate::Reg<u32, _LSCH0_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH0_CONF0;
#[doc = "`read()` method returns [lsch0_conf0::R](lsch0_conf0::R) reader structure"]
impl crate::Readable for LSCH0_CONF0 {}
#[doc = "`write(|w| ..)` method takes [lsch0_conf0::W](lsch0_conf0::W) writer structure"]
impl crate::Writable for LSCH0_CONF0 {}
#[doc = "LEDC_LSCH0_CONF0"]
pub mod lsch0_conf0;
#[doc = "LEDC_LSCH0_HPOINT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch0_hpoint](lsch0_hpoint) module"]
pub type LSCH0_HPOINT = crate::Reg<u32, _LSCH0_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH0_HPOINT;
#[doc = "`read()` method returns [lsch0_hpoint::R](lsch0_hpoint::R) reader structure"]
impl crate::Readable for LSCH0_HPOINT {}
#[doc = "`write(|w| ..)` method takes [lsch0_hpoint::W](lsch0_hpoint::W) writer structure"]
impl crate::Writable for LSCH0_HPOINT {}
#[doc = "LEDC_LSCH0_HPOINT"]
pub mod lsch0_hpoint;
#[doc = "LEDC_LSCH0_DUTY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch0_duty](lsch0_duty) module"]
pub type LSCH0_DUTY = crate::Reg<u32, _LSCH0_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH0_DUTY;
#[doc = "`read()` method returns [lsch0_duty::R](lsch0_duty::R) reader structure"]
impl crate::Readable for LSCH0_DUTY {}
#[doc = "`write(|w| ..)` method takes [lsch0_duty::W](lsch0_duty::W) writer structure"]
impl crate::Writable for LSCH0_DUTY {}
#[doc = "LEDC_LSCH0_DUTY"]
pub mod lsch0_duty;
#[doc = "LEDC_LSCH0_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch0_conf1](lsch0_conf1) module"]
pub type LSCH0_CONF1 = crate::Reg<u32, _LSCH0_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH0_CONF1;
#[doc = "`read()` method returns [lsch0_conf1::R](lsch0_conf1::R) reader structure"]
impl crate::Readable for LSCH0_CONF1 {}
#[doc = "`write(|w| ..)` method takes [lsch0_conf1::W](lsch0_conf1::W) writer structure"]
impl crate::Writable for LSCH0_CONF1 {}
#[doc = "LEDC_LSCH0_CONF1"]
pub mod lsch0_conf1;
#[doc = "LEDC_LSCH0_DUTY_R\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch0_duty_r](lsch0_duty_r) module"]
pub type LSCH0_DUTY_R = crate::Reg<u32, _LSCH0_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH0_DUTY_R;
#[doc = "`read()` method returns [lsch0_duty_r::R](lsch0_duty_r::R) reader structure"]
impl crate::Readable for LSCH0_DUTY_R {}
#[doc = "LEDC_LSCH0_DUTY_R"]
pub mod lsch0_duty_r;
#[doc = "LEDC_LSCH1_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch1_conf0](lsch1_conf0) module"]
pub type LSCH1_CONF0 = crate::Reg<u32, _LSCH1_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH1_CONF0;
#[doc = "`read()` method returns [lsch1_conf0::R](lsch1_conf0::R) reader structure"]
impl crate::Readable for LSCH1_CONF0 {}
#[doc = "`write(|w| ..)` method takes [lsch1_conf0::W](lsch1_conf0::W) writer structure"]
impl crate::Writable for LSCH1_CONF0 {}
#[doc = "LEDC_LSCH1_CONF0"]
pub mod lsch1_conf0;
#[doc = "LEDC_LSCH1_HPOINT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch1_hpoint](lsch1_hpoint) module"]
pub type LSCH1_HPOINT = crate::Reg<u32, _LSCH1_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH1_HPOINT;
#[doc = "`read()` method returns [lsch1_hpoint::R](lsch1_hpoint::R) reader structure"]
impl crate::Readable for LSCH1_HPOINT {}
#[doc = "`write(|w| ..)` method takes [lsch1_hpoint::W](lsch1_hpoint::W) writer structure"]
impl crate::Writable for LSCH1_HPOINT {}
#[doc = "LEDC_LSCH1_HPOINT"]
pub mod lsch1_hpoint;
#[doc = "LEDC_LSCH1_DUTY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch1_duty](lsch1_duty) module"]
pub type LSCH1_DUTY = crate::Reg<u32, _LSCH1_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH1_DUTY;
#[doc = "`read()` method returns [lsch1_duty::R](lsch1_duty::R) reader structure"]
impl crate::Readable for LSCH1_DUTY {}
#[doc = "`write(|w| ..)` method takes [lsch1_duty::W](lsch1_duty::W) writer structure"]
impl crate::Writable for LSCH1_DUTY {}
#[doc = "LEDC_LSCH1_DUTY"]
pub mod lsch1_duty;
#[doc = "LEDC_LSCH1_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch1_conf1](lsch1_conf1) module"]
pub type LSCH1_CONF1 = crate::Reg<u32, _LSCH1_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH1_CONF1;
#[doc = "`read()` method returns [lsch1_conf1::R](lsch1_conf1::R) reader structure"]
impl crate::Readable for LSCH1_CONF1 {}
#[doc = "`write(|w| ..)` method takes [lsch1_conf1::W](lsch1_conf1::W) writer structure"]
impl crate::Writable for LSCH1_CONF1 {}
#[doc = "LEDC_LSCH1_CONF1"]
pub mod lsch1_conf1;
#[doc = "LEDC_LSCH1_DUTY_R\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch1_duty_r](lsch1_duty_r) module"]
pub type LSCH1_DUTY_R = crate::Reg<u32, _LSCH1_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH1_DUTY_R;
#[doc = "`read()` method returns [lsch1_duty_r::R](lsch1_duty_r::R) reader structure"]
impl crate::Readable for LSCH1_DUTY_R {}
#[doc = "LEDC_LSCH1_DUTY_R"]
pub mod lsch1_duty_r;
#[doc = "LEDC_LSCH2_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch2_conf0](lsch2_conf0) module"]
pub type LSCH2_CONF0 = crate::Reg<u32, _LSCH2_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH2_CONF0;
#[doc = "`read()` method returns [lsch2_conf0::R](lsch2_conf0::R) reader structure"]
impl crate::Readable for LSCH2_CONF0 {}
#[doc = "`write(|w| ..)` method takes [lsch2_conf0::W](lsch2_conf0::W) writer structure"]
impl crate::Writable for LSCH2_CONF0 {}
#[doc = "LEDC_LSCH2_CONF0"]
pub mod lsch2_conf0;
#[doc = "LEDC_LSCH2_HPOINT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch2_hpoint](lsch2_hpoint) module"]
pub type LSCH2_HPOINT = crate::Reg<u32, _LSCH2_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH2_HPOINT;
#[doc = "`read()` method returns [lsch2_hpoint::R](lsch2_hpoint::R) reader structure"]
impl crate::Readable for LSCH2_HPOINT {}
#[doc = "`write(|w| ..)` method takes [lsch2_hpoint::W](lsch2_hpoint::W) writer structure"]
impl crate::Writable for LSCH2_HPOINT {}
#[doc = "LEDC_LSCH2_HPOINT"]
pub mod lsch2_hpoint;
#[doc = "LEDC_LSCH2_DUTY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch2_duty](lsch2_duty) module"]
pub type LSCH2_DUTY = crate::Reg<u32, _LSCH2_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH2_DUTY;
#[doc = "`read()` method returns [lsch2_duty::R](lsch2_duty::R) reader structure"]
impl crate::Readable for LSCH2_DUTY {}
#[doc = "`write(|w| ..)` method takes [lsch2_duty::W](lsch2_duty::W) writer structure"]
impl crate::Writable for LSCH2_DUTY {}
#[doc = "LEDC_LSCH2_DUTY"]
pub mod lsch2_duty;
#[doc = "LEDC_LSCH2_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch2_conf1](lsch2_conf1) module"]
pub type LSCH2_CONF1 = crate::Reg<u32, _LSCH2_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH2_CONF1;
#[doc = "`read()` method returns [lsch2_conf1::R](lsch2_conf1::R) reader structure"]
impl crate::Readable for LSCH2_CONF1 {}
#[doc = "`write(|w| ..)` method takes [lsch2_conf1::W](lsch2_conf1::W) writer structure"]
impl crate::Writable for LSCH2_CONF1 {}
#[doc = "LEDC_LSCH2_CONF1"]
pub mod lsch2_conf1;
#[doc = "LEDC_LSCH2_DUTY_R\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch2_duty_r](lsch2_duty_r) module"]
pub type LSCH2_DUTY_R = crate::Reg<u32, _LSCH2_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH2_DUTY_R;
#[doc = "`read()` method returns [lsch2_duty_r::R](lsch2_duty_r::R) reader structure"]
impl crate::Readable for LSCH2_DUTY_R {}
#[doc = "LEDC_LSCH2_DUTY_R"]
pub mod lsch2_duty_r;
#[doc = "LEDC_LSCH3_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch3_conf0](lsch3_conf0) module"]
pub type LSCH3_CONF0 = crate::Reg<u32, _LSCH3_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH3_CONF0;
#[doc = "`read()` method returns [lsch3_conf0::R](lsch3_conf0::R) reader structure"]
impl crate::Readable for LSCH3_CONF0 {}
#[doc = "`write(|w| ..)` method takes [lsch3_conf0::W](lsch3_conf0::W) writer structure"]
impl crate::Writable for LSCH3_CONF0 {}
#[doc = "LEDC_LSCH3_CONF0"]
pub mod lsch3_conf0;
#[doc = "LEDC_LSCH3_HPOINT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch3_hpoint](lsch3_hpoint) module"]
pub type LSCH3_HPOINT = crate::Reg<u32, _LSCH3_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH3_HPOINT;
#[doc = "`read()` method returns [lsch3_hpoint::R](lsch3_hpoint::R) reader structure"]
impl crate::Readable for LSCH3_HPOINT {}
#[doc = "`write(|w| ..)` method takes [lsch3_hpoint::W](lsch3_hpoint::W) writer structure"]
impl crate::Writable for LSCH3_HPOINT {}
#[doc = "LEDC_LSCH3_HPOINT"]
pub mod lsch3_hpoint;
#[doc = "LEDC_LSCH3_DUTY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch3_duty](lsch3_duty) module"]
pub type LSCH3_DUTY = crate::Reg<u32, _LSCH3_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH3_DUTY;
#[doc = "`read()` method returns [lsch3_duty::R](lsch3_duty::R) reader structure"]
impl crate::Readable for LSCH3_DUTY {}
#[doc = "`write(|w| ..)` method takes [lsch3_duty::W](lsch3_duty::W) writer structure"]
impl crate::Writable for LSCH3_DUTY {}
#[doc = "LEDC_LSCH3_DUTY"]
pub mod lsch3_duty;
#[doc = "LEDC_LSCH3_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch3_conf1](lsch3_conf1) module"]
pub type LSCH3_CONF1 = crate::Reg<u32, _LSCH3_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH3_CONF1;
#[doc = "`read()` method returns [lsch3_conf1::R](lsch3_conf1::R) reader structure"]
impl crate::Readable for LSCH3_CONF1 {}
#[doc = "`write(|w| ..)` method takes [lsch3_conf1::W](lsch3_conf1::W) writer structure"]
impl crate::Writable for LSCH3_CONF1 {}
#[doc = "LEDC_LSCH3_CONF1"]
pub mod lsch3_conf1;
#[doc = "LEDC_LSCH3_DUTY_R\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch3_duty_r](lsch3_duty_r) module"]
pub type LSCH3_DUTY_R = crate::Reg<u32, _LSCH3_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH3_DUTY_R;
#[doc = "`read()` method returns [lsch3_duty_r::R](lsch3_duty_r::R) reader structure"]
impl crate::Readable for LSCH3_DUTY_R {}
#[doc = "LEDC_LSCH3_DUTY_R"]
pub mod lsch3_duty_r;
#[doc = "LEDC_LSCH4_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch4_conf0](lsch4_conf0) module"]
pub type LSCH4_CONF0 = crate::Reg<u32, _LSCH4_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH4_CONF0;
#[doc = "`read()` method returns [lsch4_conf0::R](lsch4_conf0::R) reader structure"]
impl crate::Readable for LSCH4_CONF0 {}
#[doc = "`write(|w| ..)` method takes [lsch4_conf0::W](lsch4_conf0::W) writer structure"]
impl crate::Writable for LSCH4_CONF0 {}
#[doc = "LEDC_LSCH4_CONF0"]
pub mod lsch4_conf0;
#[doc = "LEDC_LSCH4_HPOINT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch4_hpoint](lsch4_hpoint) module"]
pub type LSCH4_HPOINT = crate::Reg<u32, _LSCH4_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH4_HPOINT;
#[doc = "`read()` method returns [lsch4_hpoint::R](lsch4_hpoint::R) reader structure"]
impl crate::Readable for LSCH4_HPOINT {}
#[doc = "`write(|w| ..)` method takes [lsch4_hpoint::W](lsch4_hpoint::W) writer structure"]
impl crate::Writable for LSCH4_HPOINT {}
#[doc = "LEDC_LSCH4_HPOINT"]
pub mod lsch4_hpoint;
#[doc = "LEDC_LSCH4_DUTY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch4_duty](lsch4_duty) module"]
pub type LSCH4_DUTY = crate::Reg<u32, _LSCH4_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH4_DUTY;
#[doc = "`read()` method returns [lsch4_duty::R](lsch4_duty::R) reader structure"]
impl crate::Readable for LSCH4_DUTY {}
#[doc = "`write(|w| ..)` method takes [lsch4_duty::W](lsch4_duty::W) writer structure"]
impl crate::Writable for LSCH4_DUTY {}
#[doc = "LEDC_LSCH4_DUTY"]
pub mod lsch4_duty;
#[doc = "LEDC_LSCH4_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch4_conf1](lsch4_conf1) module"]
pub type LSCH4_CONF1 = crate::Reg<u32, _LSCH4_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH4_CONF1;
#[doc = "`read()` method returns [lsch4_conf1::R](lsch4_conf1::R) reader structure"]
impl crate::Readable for LSCH4_CONF1 {}
#[doc = "`write(|w| ..)` method takes [lsch4_conf1::W](lsch4_conf1::W) writer structure"]
impl crate::Writable for LSCH4_CONF1 {}
#[doc = "LEDC_LSCH4_CONF1"]
pub mod lsch4_conf1;
#[doc = "LEDC_LSCH4_DUTY_R\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch4_duty_r](lsch4_duty_r) module"]
pub type LSCH4_DUTY_R = crate::Reg<u32, _LSCH4_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH4_DUTY_R;
#[doc = "`read()` method returns [lsch4_duty_r::R](lsch4_duty_r::R) reader structure"]
impl crate::Readable for LSCH4_DUTY_R {}
#[doc = "LEDC_LSCH4_DUTY_R"]
pub mod lsch4_duty_r;
#[doc = "LEDC_LSCH5_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch5_conf0](lsch5_conf0) module"]
pub type LSCH5_CONF0 = crate::Reg<u32, _LSCH5_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH5_CONF0;
#[doc = "`read()` method returns [lsch5_conf0::R](lsch5_conf0::R) reader structure"]
impl crate::Readable for LSCH5_CONF0 {}
#[doc = "`write(|w| ..)` method takes [lsch5_conf0::W](lsch5_conf0::W) writer structure"]
impl crate::Writable for LSCH5_CONF0 {}
#[doc = "LEDC_LSCH5_CONF0"]
pub mod lsch5_conf0;
#[doc = "LEDC_LSCH5_HPOINT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch5_hpoint](lsch5_hpoint) module"]
pub type LSCH5_HPOINT = crate::Reg<u32, _LSCH5_HPOINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH5_HPOINT;
#[doc = "`read()` method returns [lsch5_hpoint::R](lsch5_hpoint::R) reader structure"]
impl crate::Readable for LSCH5_HPOINT {}
#[doc = "`write(|w| ..)` method takes [lsch5_hpoint::W](lsch5_hpoint::W) writer structure"]
impl crate::Writable for LSCH5_HPOINT {}
#[doc = "LEDC_LSCH5_HPOINT"]
pub mod lsch5_hpoint;
#[doc = "LEDC_LSCH5_DUTY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch5_duty](lsch5_duty) module"]
pub type LSCH5_DUTY = crate::Reg<u32, _LSCH5_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH5_DUTY;
#[doc = "`read()` method returns [lsch5_duty::R](lsch5_duty::R) reader structure"]
impl crate::Readable for LSCH5_DUTY {}
#[doc = "`write(|w| ..)` method takes [lsch5_duty::W](lsch5_duty::W) writer structure"]
impl crate::Writable for LSCH5_DUTY {}
#[doc = "LEDC_LSCH5_DUTY"]
pub mod lsch5_duty;
#[doc = "LEDC_LSCH5_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch5_conf1](lsch5_conf1) module"]
pub type LSCH5_CONF1 = crate::Reg<u32, _LSCH5_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH5_CONF1;
#[doc = "`read()` method returns [lsch5_conf1::R](lsch5_conf1::R) reader structure"]
impl crate::Readable for LSCH5_CONF1 {}
#[doc = "`write(|w| ..)` method takes [lsch5_conf1::W](lsch5_conf1::W) writer structure"]
impl crate::Writable for LSCH5_CONF1 {}
#[doc = "LEDC_LSCH5_CONF1"]
pub mod lsch5_conf1;
#[doc = "LEDC_LSCH5_DUTY_R\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch5_duty_r](lsch5_duty_r) module"]
pub type LSCH5_DUTY_R = crate::Reg<u32, _LSCH5_DUTY_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSCH5_DUTY_R;
#[doc = "`read()` method returns [lsch5_duty_r::R](lsch5_duty_r::R) reader structure"]
impl crate::Readable for LSCH5_DUTY_R {}
#[doc = "LEDC_LSCH5_DUTY_R"]
pub mod lsch5_duty_r;
#[doc = "LEDC_LSTIMER0_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lstimer0_conf](lstimer0_conf) module"]
pub type LSTIMER0_CONF = crate::Reg<u32, _LSTIMER0_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSTIMER0_CONF;
#[doc = "`read()` method returns [lstimer0_conf::R](lstimer0_conf::R) reader structure"]
impl crate::Readable for LSTIMER0_CONF {}
#[doc = "`write(|w| ..)` method takes [lstimer0_conf::W](lstimer0_conf::W) writer structure"]
impl crate::Writable for LSTIMER0_CONF {}
#[doc = "LEDC_LSTIMER0_CONF"]
pub mod lstimer0_conf;
#[doc = "LEDC_LSTIMER0_VALUE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lstimer0_value](lstimer0_value) module"]
pub type LSTIMER0_VALUE = crate::Reg<u32, _LSTIMER0_VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSTIMER0_VALUE;
#[doc = "`read()` method returns [lstimer0_value::R](lstimer0_value::R) reader structure"]
impl crate::Readable for LSTIMER0_VALUE {}
#[doc = "LEDC_LSTIMER0_VALUE"]
pub mod lstimer0_value;
#[doc = "LEDC_LSTIMER1_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lstimer1_conf](lstimer1_conf) module"]
pub type LSTIMER1_CONF = crate::Reg<u32, _LSTIMER1_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSTIMER1_CONF;
#[doc = "`read()` method returns [lstimer1_conf::R](lstimer1_conf::R) reader structure"]
impl crate::Readable for LSTIMER1_CONF {}
#[doc = "`write(|w| ..)` method takes [lstimer1_conf::W](lstimer1_conf::W) writer structure"]
impl crate::Writable for LSTIMER1_CONF {}
#[doc = "LEDC_LSTIMER1_CONF"]
pub mod lstimer1_conf;
#[doc = "LEDC_LSTIMER1_VALUE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lstimer1_value](lstimer1_value) module"]
pub type LSTIMER1_VALUE = crate::Reg<u32, _LSTIMER1_VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSTIMER1_VALUE;
#[doc = "`read()` method returns [lstimer1_value::R](lstimer1_value::R) reader structure"]
impl crate::Readable for LSTIMER1_VALUE {}
#[doc = "LEDC_LSTIMER1_VALUE"]
pub mod lstimer1_value;
#[doc = "LEDC_LSTIMER2_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lstimer2_conf](lstimer2_conf) module"]
pub type LSTIMER2_CONF = crate::Reg<u32, _LSTIMER2_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSTIMER2_CONF;
#[doc = "`read()` method returns [lstimer2_conf::R](lstimer2_conf::R) reader structure"]
impl crate::Readable for LSTIMER2_CONF {}
#[doc = "`write(|w| ..)` method takes [lstimer2_conf::W](lstimer2_conf::W) writer structure"]
impl crate::Writable for LSTIMER2_CONF {}
#[doc = "LEDC_LSTIMER2_CONF"]
pub mod lstimer2_conf;
#[doc = "LEDC_LSTIMER2_VALUE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lstimer2_value](lstimer2_value) module"]
pub type LSTIMER2_VALUE = crate::Reg<u32, _LSTIMER2_VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSTIMER2_VALUE;
#[doc = "`read()` method returns [lstimer2_value::R](lstimer2_value::R) reader structure"]
impl crate::Readable for LSTIMER2_VALUE {}
#[doc = "LEDC_LSTIMER2_VALUE"]
pub mod lstimer2_value;
#[doc = "LEDC_LSTIMER3_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lstimer3_conf](lstimer3_conf) module"]
pub type LSTIMER3_CONF = crate::Reg<u32, _LSTIMER3_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSTIMER3_CONF;
#[doc = "`read()` method returns [lstimer3_conf::R](lstimer3_conf::R) reader structure"]
impl crate::Readable for LSTIMER3_CONF {}
#[doc = "`write(|w| ..)` method takes [lstimer3_conf::W](lstimer3_conf::W) writer structure"]
impl crate::Writable for LSTIMER3_CONF {}
#[doc = "LEDC_LSTIMER3_CONF"]
pub mod lstimer3_conf;
#[doc = "LEDC_LSTIMER3_VALUE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lstimer3_value](lstimer3_value) module"]
pub type LSTIMER3_VALUE = crate::Reg<u32, _LSTIMER3_VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSTIMER3_VALUE;
#[doc = "`read()` method returns [lstimer3_value::R](lstimer3_value::R) reader structure"]
impl crate::Readable for LSTIMER3_VALUE {}
#[doc = "LEDC_LSTIMER3_VALUE"]
pub mod lstimer3_value;
#[doc = "LEDC_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](int_raw) module"]
pub type INT_RAW = crate::Reg<u32, _INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_RAW;
#[doc = "`read()` method returns [int_raw::R](int_raw::R) reader structure"]
impl crate::Readable for INT_RAW {}
#[doc = "LEDC_INT_RAW"]
pub mod int_raw;
#[doc = "LEDC_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](int_st) module"]
pub type INT_ST = crate::Reg<u32, _INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ST;
#[doc = "`read()` method returns [int_st::R](int_st::R) reader structure"]
impl crate::Readable for INT_ST {}
#[doc = "LEDC_INT_ST"]
pub mod int_st;
#[doc = "LEDC_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](int_ena) module"]
pub type INT_ENA = crate::Reg<u32, _INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ENA;
#[doc = "`read()` method returns [int_ena::R](int_ena::R) reader structure"]
impl crate::Readable for INT_ENA {}
#[doc = "`write(|w| ..)` method takes [int_ena::W](int_ena::W) writer structure"]
impl crate::Writable for INT_ENA {}
#[doc = "LEDC_INT_ENA"]
pub mod int_ena;
#[doc = "LEDC_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](int_clr) module"]
pub type INT_CLR = crate::Reg<u32, _INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CLR;
#[doc = "`write(|w| ..)` method takes [int_clr::W](int_clr::W) writer structure"]
impl crate::Writable for INT_CLR {}
#[doc = "LEDC_INT_CLR"]
pub mod int_clr;
#[doc = "LEDC_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf](conf) module"]
pub type CONF = crate::Reg<u32, _CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONF;
#[doc = "`read()` method returns [conf::R](conf::R) reader structure"]
impl crate::Readable for CONF {}
#[doc = "`write(|w| ..)` method takes [conf::W](conf::W) writer structure"]
impl crate::Writable for CONF {}
#[doc = "LEDC_CONF"]
pub mod conf;
#[doc = "LEDC_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "LEDC_DATE"]
pub mod date;
