#[doc = "Reader of register DMA_CONF"]
pub type R = crate::R<u32, super::DMA_CONF>;
#[doc = "Writer for register DMA_CONF"]
pub type W = crate::W<u32, super::DMA_CONF>;
#[doc = "Register DMA_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DMA_AFIFO_RST`"]
pub struct DMA_AFIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_AFIFO_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Write proxy for field `BUF_AFIFO_RST`"]
pub struct BUF_AFIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_AFIFO_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Write proxy for field `RX_AFIFO_RST`"]
pub struct RX_AFIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_AFIFO_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `DMA_TX_ENA`"]
pub type DMA_TX_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_TX_ENA`"]
pub struct DMA_TX_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_TX_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `DMA_RX_ENA`"]
pub type DMA_RX_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_RX_ENA`"]
pub struct DMA_RX_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_RX_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `RX_EOF_EN`"]
pub type RX_EOF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_EOF_EN`"]
pub struct RX_EOF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_EOF_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `SLV_TX_SEG_TRANS_CLR_EN`"]
pub type SLV_TX_SEG_TRANS_CLR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_TX_SEG_TRANS_CLR_EN`"]
pub struct SLV_TX_SEG_TRANS_CLR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_TX_SEG_TRANS_CLR_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `SLV_RX_SEG_TRANS_CLR_EN`"]
pub type SLV_RX_SEG_TRANS_CLR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_RX_SEG_TRANS_CLR_EN`"]
pub struct SLV_RX_SEG_TRANS_CLR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RX_SEG_TRANS_CLR_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `DMA_SLV_SEG_TRANS_EN`"]
pub type DMA_SLV_SEG_TRANS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_SLV_SEG_TRANS_EN`"]
pub struct DMA_SLV_SEG_TRANS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SLV_SEG_TRANS_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dma_tx_ena(&self) -> DMA_TX_ENA_R {
        DMA_TX_ENA_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dma_rx_ena(&self) -> DMA_RX_ENA_R {
        DMA_RX_ENA_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rx_eof_en(&self) -> RX_EOF_EN_R {
        RX_EOF_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slv_tx_seg_trans_clr_en(&self) -> SLV_TX_SEG_TRANS_CLR_EN_R {
        SLV_TX_SEG_TRANS_CLR_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slv_rx_seg_trans_clr_en(&self) -> SLV_RX_SEG_TRANS_CLR_EN_R {
        SLV_RX_SEG_TRANS_CLR_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn dma_slv_seg_trans_en(&self) -> DMA_SLV_SEG_TRANS_EN_R {
        DMA_SLV_SEG_TRANS_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dma_afifo_rst(&mut self) -> DMA_AFIFO_RST_W {
        DMA_AFIFO_RST_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn buf_afifo_rst(&mut self) -> BUF_AFIFO_RST_W {
        BUF_AFIFO_RST_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rx_afifo_rst(&mut self) -> RX_AFIFO_RST_W {
        RX_AFIFO_RST_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dma_tx_ena(&mut self) -> DMA_TX_ENA_W {
        DMA_TX_ENA_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dma_rx_ena(&mut self) -> DMA_RX_ENA_W {
        DMA_RX_ENA_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rx_eof_en(&mut self) -> RX_EOF_EN_W {
        RX_EOF_EN_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slv_tx_seg_trans_clr_en(&mut self) -> SLV_TX_SEG_TRANS_CLR_EN_W {
        SLV_TX_SEG_TRANS_CLR_EN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slv_rx_seg_trans_clr_en(&mut self) -> SLV_RX_SEG_TRANS_CLR_EN_W {
        SLV_RX_SEG_TRANS_CLR_EN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn dma_slv_seg_trans_en(&mut self) -> DMA_SLV_SEG_TRANS_EN_W {
        DMA_SLV_SEG_TRANS_EN_W { w: self }
    }
}
