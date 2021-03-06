#[doc = "Reader of register ASSIST_DEBUG_CORE_0_AREA_SP"]
pub type R = crate::R<u32, super::ASSIST_DEBUG_CORE_0_AREA_SP>;
#[doc = "Reader of field `ASSIST_DEBUG_CORE_0_AREA_SP`"]
pub type ASSIST_DEBUG_CORE_0_AREA_SP_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn assist_debug_core_0_area_sp(&self) -> ASSIST_DEBUG_CORE_0_AREA_SP_R {
        ASSIST_DEBUG_CORE_0_AREA_SP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
