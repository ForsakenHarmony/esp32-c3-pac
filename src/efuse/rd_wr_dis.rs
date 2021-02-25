#[doc = "Reader of register RD_WR_DIS"]
pub type R = crate::R<u32, super::RD_WR_DIS>;
#[doc = "Reader of field `WR_DIS`"]
pub type WR_DIS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn wr_dis(&self) -> WR_DIS_R {
        WR_DIS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
