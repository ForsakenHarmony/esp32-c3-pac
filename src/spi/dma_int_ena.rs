#[doc = "Reader of register DMA_INT_ENA"]
pub type R = crate::R<u32, super::DMA_INT_ENA>;
#[doc = "Writer for register DMA_INT_ENA"]
pub type W = crate::W<u32, super::DMA_INT_ENA>;
#[doc = "Register DMA_INT_ENA `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_INT_ENA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APP1_INT_ENA`"]
pub type APP1_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP1_INT_ENA`"]
pub struct APP1_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> APP1_INT_ENA_W<'a> {
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
#[doc = "Reader of field `APP2_INT_ENA`"]
pub type APP2_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP2_INT_ENA`"]
pub struct APP2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> APP2_INT_ENA_W<'a> {
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
#[doc = "Reader of field `MST_TX_AFIFO_REMPTY_ERR_INT_ENA`"]
pub type MST_TX_AFIFO_REMPTY_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MST_TX_AFIFO_REMPTY_ERR_INT_ENA`"]
pub struct MST_TX_AFIFO_REMPTY_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> MST_TX_AFIFO_REMPTY_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `MST_RX_AFIFO_WFULL_ERR_INT_ENA`"]
pub type MST_RX_AFIFO_WFULL_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MST_RX_AFIFO_WFULL_ERR_INT_ENA`"]
pub struct MST_RX_AFIFO_WFULL_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> MST_RX_AFIFO_WFULL_ERR_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `SLV_CMD_ERR_INT_ENA`"]
pub type SLV_CMD_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_CMD_ERR_INT_ENA`"]
pub struct SLV_CMD_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMD_ERR_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SLV_BUF_ADDR_ERR_INT_ENA`"]
pub type SLV_BUF_ADDR_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_BUF_ADDR_ERR_INT_ENA`"]
pub struct SLV_BUF_ADDR_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_BUF_ADDR_ERR_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `SEG_MAGIC_ERR_INT_ENA`"]
pub type SEG_MAGIC_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEG_MAGIC_ERR_INT_ENA`"]
pub struct SEG_MAGIC_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SEG_MAGIC_ERR_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `DMA_SEG_TRANS_DONE_INT_ENA`"]
pub type DMA_SEG_TRANS_DONE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_SEG_TRANS_DONE_INT_ENA`"]
pub struct DMA_SEG_TRANS_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SEG_TRANS_DONE_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `TRANS_DONE_INT_ENA`"]
pub type TRANS_DONE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRANS_DONE_INT_ENA`"]
pub struct TRANS_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_DONE_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `SLV_WR_BUF_DONE_INT_ENA`"]
pub type SLV_WR_BUF_DONE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_WR_BUF_DONE_INT_ENA`"]
pub struct SLV_WR_BUF_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WR_BUF_DONE_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `SLV_RD_BUF_DONE_INT_ENA`"]
pub type SLV_RD_BUF_DONE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_RD_BUF_DONE_INT_ENA`"]
pub struct SLV_RD_BUF_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RD_BUF_DONE_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SLV_WR_DMA_DONE_INT_ENA`"]
pub type SLV_WR_DMA_DONE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_WR_DMA_DONE_INT_ENA`"]
pub struct SLV_WR_DMA_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WR_DMA_DONE_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SLV_RD_DMA_DONE_INT_ENA`"]
pub type SLV_RD_DMA_DONE_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_RD_DMA_DONE_INT_ENA`"]
pub struct SLV_RD_DMA_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RD_DMA_DONE_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SLV_CMDA_INT_ENA`"]
pub type SLV_CMDA_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_CMDA_INT_ENA`"]
pub struct SLV_CMDA_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMDA_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `SLV_CMD9_INT_ENA`"]
pub type SLV_CMD9_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_CMD9_INT_ENA`"]
pub struct SLV_CMD9_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMD9_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SLV_CMD8_INT_ENA`"]
pub type SLV_CMD8_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_CMD8_INT_ENA`"]
pub struct SLV_CMD8_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMD8_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `SLV_CMD7_INT_ENA`"]
pub type SLV_CMD7_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_CMD7_INT_ENA`"]
pub struct SLV_CMD7_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMD7_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SLV_EN_QPI_INT_ENA`"]
pub type SLV_EN_QPI_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_EN_QPI_INT_ENA`"]
pub struct SLV_EN_QPI_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_EN_QPI_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SLV_EX_QPI_INT_ENA`"]
pub type SLV_EX_QPI_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_EX_QPI_INT_ENA`"]
pub struct SLV_EX_QPI_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_EX_QPI_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `DMA_OUTFIFO_EMPTY_ERR_INT_ENA`"]
pub type DMA_OUTFIFO_EMPTY_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_OUTFIFO_EMPTY_ERR_INT_ENA`"]
pub struct DMA_OUTFIFO_EMPTY_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUTFIFO_EMPTY_ERR_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DMA_INFIFO_FULL_ERR_INT_ENA`"]
pub type DMA_INFIFO_FULL_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_INFIFO_FULL_ERR_INT_ENA`"]
pub struct DMA_INFIFO_FULL_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_INFIFO_FULL_ERR_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn app1_int_ena(&self) -> APP1_INT_ENA_R {
        APP1_INT_ENA_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn app2_int_ena(&self) -> APP2_INT_ENA_R {
        APP2_INT_ENA_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn mst_tx_afifo_rempty_err_int_ena(&self) -> MST_TX_AFIFO_REMPTY_ERR_INT_ENA_R {
        MST_TX_AFIFO_REMPTY_ERR_INT_ENA_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn mst_rx_afifo_wfull_err_int_ena(&self) -> MST_RX_AFIFO_WFULL_ERR_INT_ENA_R {
        MST_RX_AFIFO_WFULL_ERR_INT_ENA_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slv_cmd_err_int_ena(&self) -> SLV_CMD_ERR_INT_ENA_R {
        SLV_CMD_ERR_INT_ENA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slv_buf_addr_err_int_ena(&self) -> SLV_BUF_ADDR_ERR_INT_ENA_R {
        SLV_BUF_ADDR_ERR_INT_ENA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn seg_magic_err_int_ena(&self) -> SEG_MAGIC_ERR_INT_ENA_R {
        SEG_MAGIC_ERR_INT_ENA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dma_seg_trans_done_int_ena(&self) -> DMA_SEG_TRANS_DONE_INT_ENA_R {
        DMA_SEG_TRANS_DONE_INT_ENA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn trans_done_int_ena(&self) -> TRANS_DONE_INT_ENA_R {
        TRANS_DONE_INT_ENA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slv_wr_buf_done_int_ena(&self) -> SLV_WR_BUF_DONE_INT_ENA_R {
        SLV_WR_BUF_DONE_INT_ENA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slv_rd_buf_done_int_ena(&self) -> SLV_RD_BUF_DONE_INT_ENA_R {
        SLV_RD_BUF_DONE_INT_ENA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slv_wr_dma_done_int_ena(&self) -> SLV_WR_DMA_DONE_INT_ENA_R {
        SLV_WR_DMA_DONE_INT_ENA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slv_rd_dma_done_int_ena(&self) -> SLV_RD_DMA_DONE_INT_ENA_R {
        SLV_RD_DMA_DONE_INT_ENA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn slv_cmda_int_ena(&self) -> SLV_CMDA_INT_ENA_R {
        SLV_CMDA_INT_ENA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slv_cmd9_int_ena(&self) -> SLV_CMD9_INT_ENA_R {
        SLV_CMD9_INT_ENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slv_cmd8_int_ena(&self) -> SLV_CMD8_INT_ENA_R {
        SLV_CMD8_INT_ENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slv_cmd7_int_ena(&self) -> SLV_CMD7_INT_ENA_R {
        SLV_CMD7_INT_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slv_en_qpi_int_ena(&self) -> SLV_EN_QPI_INT_ENA_R {
        SLV_EN_QPI_INT_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slv_ex_qpi_int_ena(&self) -> SLV_EX_QPI_INT_ENA_R {
        SLV_EX_QPI_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_outfifo_empty_err_int_ena(&self) -> DMA_OUTFIFO_EMPTY_ERR_INT_ENA_R {
        DMA_OUTFIFO_EMPTY_ERR_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_infifo_full_err_int_ena(&self) -> DMA_INFIFO_FULL_ERR_INT_ENA_R {
        DMA_INFIFO_FULL_ERR_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn app1_int_ena(&mut self) -> APP1_INT_ENA_W {
        APP1_INT_ENA_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn app2_int_ena(&mut self) -> APP2_INT_ENA_W {
        APP2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn mst_tx_afifo_rempty_err_int_ena(&mut self) -> MST_TX_AFIFO_REMPTY_ERR_INT_ENA_W {
        MST_TX_AFIFO_REMPTY_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn mst_rx_afifo_wfull_err_int_ena(&mut self) -> MST_RX_AFIFO_WFULL_ERR_INT_ENA_W {
        MST_RX_AFIFO_WFULL_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slv_cmd_err_int_ena(&mut self) -> SLV_CMD_ERR_INT_ENA_W {
        SLV_CMD_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slv_buf_addr_err_int_ena(&mut self) -> SLV_BUF_ADDR_ERR_INT_ENA_W {
        SLV_BUF_ADDR_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn seg_magic_err_int_ena(&mut self) -> SEG_MAGIC_ERR_INT_ENA_W {
        SEG_MAGIC_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dma_seg_trans_done_int_ena(&mut self) -> DMA_SEG_TRANS_DONE_INT_ENA_W {
        DMA_SEG_TRANS_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn trans_done_int_ena(&mut self) -> TRANS_DONE_INT_ENA_W {
        TRANS_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slv_wr_buf_done_int_ena(&mut self) -> SLV_WR_BUF_DONE_INT_ENA_W {
        SLV_WR_BUF_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slv_rd_buf_done_int_ena(&mut self) -> SLV_RD_BUF_DONE_INT_ENA_W {
        SLV_RD_BUF_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slv_wr_dma_done_int_ena(&mut self) -> SLV_WR_DMA_DONE_INT_ENA_W {
        SLV_WR_DMA_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slv_rd_dma_done_int_ena(&mut self) -> SLV_RD_DMA_DONE_INT_ENA_W {
        SLV_RD_DMA_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn slv_cmda_int_ena(&mut self) -> SLV_CMDA_INT_ENA_W {
        SLV_CMDA_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slv_cmd9_int_ena(&mut self) -> SLV_CMD9_INT_ENA_W {
        SLV_CMD9_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slv_cmd8_int_ena(&mut self) -> SLV_CMD8_INT_ENA_W {
        SLV_CMD8_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slv_cmd7_int_ena(&mut self) -> SLV_CMD7_INT_ENA_W {
        SLV_CMD7_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slv_en_qpi_int_ena(&mut self) -> SLV_EN_QPI_INT_ENA_W {
        SLV_EN_QPI_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slv_ex_qpi_int_ena(&mut self) -> SLV_EX_QPI_INT_ENA_W {
        SLV_EX_QPI_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_outfifo_empty_err_int_ena(&mut self) -> DMA_OUTFIFO_EMPTY_ERR_INT_ENA_W {
        DMA_OUTFIFO_EMPTY_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_infifo_full_err_int_ena(&mut self) -> DMA_INFIFO_FULL_ERR_INT_ENA_W {
        DMA_INFIFO_FULL_ERR_INT_ENA_W { w: self }
    }
}
