#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO_SIGMADELTA0"]
    pub sigmadelta0: SIGMADELTA0,
    #[doc = "0x04 - GPIO_SIGMADELTA1"]
    pub sigmadelta1: SIGMADELTA1,
    #[doc = "0x08 - GPIO_SIGMADELTA2"]
    pub sigmadelta2: SIGMADELTA2,
    #[doc = "0x0c - GPIO_SIGMADELTA3"]
    pub sigmadelta3: SIGMADELTA3,
    _reserved4: [u8; 16usize],
    #[doc = "0x20 - GPIO_SIGMADELTA_CG"]
    pub sigmadelta_cg: SIGMADELTA_CG,
    #[doc = "0x24 - GPIO_SIGMADELTA_MISC"]
    pub sigmadelta_misc: SIGMADELTA_MISC,
    #[doc = "0x28 - GPIO_SIGMADELTA_VERSION"]
    pub sigmadelta_version: SIGMADELTA_VERSION,
}
#[doc = "GPIO_SIGMADELTA0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigmadelta0](sigmadelta0) module"]
pub type SIGMADELTA0 = crate::Reg<u32, _SIGMADELTA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIGMADELTA0;
#[doc = "`read()` method returns [sigmadelta0::R](sigmadelta0::R) reader structure"]
impl crate::Readable for SIGMADELTA0 {}
#[doc = "`write(|w| ..)` method takes [sigmadelta0::W](sigmadelta0::W) writer structure"]
impl crate::Writable for SIGMADELTA0 {}
#[doc = "GPIO_SIGMADELTA0"]
pub mod sigmadelta0;
#[doc = "GPIO_SIGMADELTA1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigmadelta1](sigmadelta1) module"]
pub type SIGMADELTA1 = crate::Reg<u32, _SIGMADELTA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIGMADELTA1;
#[doc = "`read()` method returns [sigmadelta1::R](sigmadelta1::R) reader structure"]
impl crate::Readable for SIGMADELTA1 {}
#[doc = "`write(|w| ..)` method takes [sigmadelta1::W](sigmadelta1::W) writer structure"]
impl crate::Writable for SIGMADELTA1 {}
#[doc = "GPIO_SIGMADELTA1"]
pub mod sigmadelta1;
#[doc = "GPIO_SIGMADELTA2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigmadelta2](sigmadelta2) module"]
pub type SIGMADELTA2 = crate::Reg<u32, _SIGMADELTA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIGMADELTA2;
#[doc = "`read()` method returns [sigmadelta2::R](sigmadelta2::R) reader structure"]
impl crate::Readable for SIGMADELTA2 {}
#[doc = "`write(|w| ..)` method takes [sigmadelta2::W](sigmadelta2::W) writer structure"]
impl crate::Writable for SIGMADELTA2 {}
#[doc = "GPIO_SIGMADELTA2"]
pub mod sigmadelta2;
#[doc = "GPIO_SIGMADELTA3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigmadelta3](sigmadelta3) module"]
pub type SIGMADELTA3 = crate::Reg<u32, _SIGMADELTA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIGMADELTA3;
#[doc = "`read()` method returns [sigmadelta3::R](sigmadelta3::R) reader structure"]
impl crate::Readable for SIGMADELTA3 {}
#[doc = "`write(|w| ..)` method takes [sigmadelta3::W](sigmadelta3::W) writer structure"]
impl crate::Writable for SIGMADELTA3 {}
#[doc = "GPIO_SIGMADELTA3"]
pub mod sigmadelta3;
#[doc = "GPIO_SIGMADELTA_CG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigmadelta_cg](sigmadelta_cg) module"]
pub type SIGMADELTA_CG = crate::Reg<u32, _SIGMADELTA_CG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIGMADELTA_CG;
#[doc = "`read()` method returns [sigmadelta_cg::R](sigmadelta_cg::R) reader structure"]
impl crate::Readable for SIGMADELTA_CG {}
#[doc = "`write(|w| ..)` method takes [sigmadelta_cg::W](sigmadelta_cg::W) writer structure"]
impl crate::Writable for SIGMADELTA_CG {}
#[doc = "GPIO_SIGMADELTA_CG"]
pub mod sigmadelta_cg;
#[doc = "GPIO_SIGMADELTA_MISC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigmadelta_misc](sigmadelta_misc) module"]
pub type SIGMADELTA_MISC = crate::Reg<u32, _SIGMADELTA_MISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIGMADELTA_MISC;
#[doc = "`read()` method returns [sigmadelta_misc::R](sigmadelta_misc::R) reader structure"]
impl crate::Readable for SIGMADELTA_MISC {}
#[doc = "`write(|w| ..)` method takes [sigmadelta_misc::W](sigmadelta_misc::W) writer structure"]
impl crate::Writable for SIGMADELTA_MISC {}
#[doc = "GPIO_SIGMADELTA_MISC"]
pub mod sigmadelta_misc;
#[doc = "GPIO_SIGMADELTA_VERSION\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigmadelta_version](sigmadelta_version) module"]
pub type SIGMADELTA_VERSION = crate::Reg<u32, _SIGMADELTA_VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIGMADELTA_VERSION;
#[doc = "`read()` method returns [sigmadelta_version::R](sigmadelta_version::R) reader structure"]
impl crate::Readable for SIGMADELTA_VERSION {}
#[doc = "`write(|w| ..)` method takes [sigmadelta_version::W](sigmadelta_version::W) writer structure"]
impl crate::Writable for SIGMADELTA_VERSION {}
#[doc = "GPIO_SIGMADELTA_VERSION"]
pub mod sigmadelta_version;
