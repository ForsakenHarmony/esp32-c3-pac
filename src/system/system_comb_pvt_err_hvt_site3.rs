#[doc = "Reader of register SYSTEM_COMB_PVT_ERR_HVT_SITE3"]
pub type R = crate::R<u32, super::SYSTEM_COMB_PVT_ERR_HVT_SITE3>;
#[doc = "Reader of field `SYSTEM_COMB_TIMING_ERR_CNT_HVT_SITE3`"]
pub type SYSTEM_COMB_TIMING_ERR_CNT_HVT_SITE3_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn system_comb_timing_err_cnt_hvt_site3(&self) -> SYSTEM_COMB_TIMING_ERR_CNT_HVT_SITE3_R {
        SYSTEM_COMB_TIMING_ERR_CNT_HVT_SITE3_R::new((self.bits & 0xffff) as u16)
    }
}
