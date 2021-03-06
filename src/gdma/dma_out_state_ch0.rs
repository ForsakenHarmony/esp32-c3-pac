#[doc = "Reader of register DMA_OUT_STATE_CH0"]
pub type R = crate::R<u32, super::DMA_OUT_STATE_CH0>;
#[doc = "Reader of field `DMA_OUT_STATE_CH0`"]
pub type DMA_OUT_STATE_CH0_R = crate::R<u8, u8>;
#[doc = "Reader of field `DMA_OUT_DSCR_STATE_CH0`"]
pub type DMA_OUT_DSCR_STATE_CH0_R = crate::R<u8, u8>;
#[doc = "Reader of field `DMA_OUTLINK_DSCR_ADDR_CH0`"]
pub type DMA_OUTLINK_DSCR_ADDR_CH0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn dma_out_state_ch0(&self) -> DMA_OUT_STATE_CH0_R {
        DMA_OUT_STATE_CH0_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn dma_out_dscr_state_ch0(&self) -> DMA_OUT_DSCR_STATE_CH0_R {
        DMA_OUT_DSCR_STATE_CH0_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn dma_outlink_dscr_addr_ch0(&self) -> DMA_OUTLINK_DSCR_ADDR_CH0_R {
        DMA_OUTLINK_DSCR_ADDR_CH0_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
