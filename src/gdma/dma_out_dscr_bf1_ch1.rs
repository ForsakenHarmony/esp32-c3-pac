#[doc = "Reader of register DMA_OUT_DSCR_BF1_CH1"]
pub type R = crate::R<u32, super::DMA_OUT_DSCR_BF1_CH1>;
#[doc = "Reader of field `DMA_OUTLINK_DSCR_BF1_CH1`"]
pub type DMA_OUTLINK_DSCR_BF1_CH1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dma_outlink_dscr_bf1_ch1(&self) -> DMA_OUTLINK_DSCR_BF1_CH1_R {
        DMA_OUTLINK_DSCR_BF1_CH1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
