#[doc = "Writer for register DMA_INT_CLR"]
pub type W = crate::W<u32, super::DMA_INT_CLR>;
#[doc = "Register DMA_INT_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_INT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `APP1_INT_CLR`"]
pub struct APP1_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> APP1_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `APP2_INT_CLR`"]
pub struct APP2_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> APP2_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `MST_TX_AFIFO_REMPTY_ERR_INT_CLR`"]
pub struct MST_TX_AFIFO_REMPTY_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> MST_TX_AFIFO_REMPTY_ERR_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `MST_RX_AFIFO_WFULL_ERR_INT_CLR`"]
pub struct MST_RX_AFIFO_WFULL_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> MST_RX_AFIFO_WFULL_ERR_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SLV_CMD_ERR_INT_CLR`"]
pub struct SLV_CMD_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMD_ERR_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SLV_BUF_ADDR_ERR_INT_CLR`"]
pub struct SLV_BUF_ADDR_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_BUF_ADDR_ERR_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SEG_MAGIC_ERR_INT_CLR`"]
pub struct SEG_MAGIC_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SEG_MAGIC_ERR_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `DMA_SEG_TRANS_DONE_INT_CLR`"]
pub struct DMA_SEG_TRANS_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SEG_TRANS_DONE_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `TRANS_DONE_INT_CLR`"]
pub struct TRANS_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_DONE_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SLV_WR_BUF_DONE_INT_CLR`"]
pub struct SLV_WR_BUF_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WR_BUF_DONE_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SLV_RD_BUF_DONE_INT_CLR`"]
pub struct SLV_RD_BUF_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RD_BUF_DONE_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SLV_WR_DMA_DONE_INT_CLR`"]
pub struct SLV_WR_DMA_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WR_DMA_DONE_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SLV_RD_DMA_DONE_INT_CLR`"]
pub struct SLV_RD_DMA_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RD_DMA_DONE_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SLV_CMDA_INT_CLR`"]
pub struct SLV_CMDA_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMDA_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SLV_CMD9_INT_CLR`"]
pub struct SLV_CMD9_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMD9_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SLV_CMD8_INT_CLR`"]
pub struct SLV_CMD8_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMD8_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SLV_CMD7_INT_CLR`"]
pub struct SLV_CMD7_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMD7_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SLV_EN_QPI_INT_CLR`"]
pub struct SLV_EN_QPI_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_EN_QPI_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `SLV_EX_QPI_INT_CLR`"]
pub struct SLV_EX_QPI_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_EX_QPI_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `DMA_OUTFIFO_EMPTY_ERR_INT_CLR`"]
pub struct DMA_OUTFIFO_EMPTY_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUTFIFO_EMPTY_ERR_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `DMA_INFIFO_FULL_ERR_INT_CLR`"]
pub struct DMA_INFIFO_FULL_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_INFIFO_FULL_ERR_INT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn app1_int_clr(&mut self) -> APP1_INT_CLR_W {
        APP1_INT_CLR_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn app2_int_clr(&mut self) -> APP2_INT_CLR_W {
        APP2_INT_CLR_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn mst_tx_afifo_rempty_err_int_clr(&mut self) -> MST_TX_AFIFO_REMPTY_ERR_INT_CLR_W {
        MST_TX_AFIFO_REMPTY_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn mst_rx_afifo_wfull_err_int_clr(&mut self) -> MST_RX_AFIFO_WFULL_ERR_INT_CLR_W {
        MST_RX_AFIFO_WFULL_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slv_cmd_err_int_clr(&mut self) -> SLV_CMD_ERR_INT_CLR_W {
        SLV_CMD_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slv_buf_addr_err_int_clr(&mut self) -> SLV_BUF_ADDR_ERR_INT_CLR_W {
        SLV_BUF_ADDR_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn seg_magic_err_int_clr(&mut self) -> SEG_MAGIC_ERR_INT_CLR_W {
        SEG_MAGIC_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dma_seg_trans_done_int_clr(&mut self) -> DMA_SEG_TRANS_DONE_INT_CLR_W {
        DMA_SEG_TRANS_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn trans_done_int_clr(&mut self) -> TRANS_DONE_INT_CLR_W {
        TRANS_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slv_wr_buf_done_int_clr(&mut self) -> SLV_WR_BUF_DONE_INT_CLR_W {
        SLV_WR_BUF_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slv_rd_buf_done_int_clr(&mut self) -> SLV_RD_BUF_DONE_INT_CLR_W {
        SLV_RD_BUF_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slv_wr_dma_done_int_clr(&mut self) -> SLV_WR_DMA_DONE_INT_CLR_W {
        SLV_WR_DMA_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slv_rd_dma_done_int_clr(&mut self) -> SLV_RD_DMA_DONE_INT_CLR_W {
        SLV_RD_DMA_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn slv_cmda_int_clr(&mut self) -> SLV_CMDA_INT_CLR_W {
        SLV_CMDA_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slv_cmd9_int_clr(&mut self) -> SLV_CMD9_INT_CLR_W {
        SLV_CMD9_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slv_cmd8_int_clr(&mut self) -> SLV_CMD8_INT_CLR_W {
        SLV_CMD8_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slv_cmd7_int_clr(&mut self) -> SLV_CMD7_INT_CLR_W {
        SLV_CMD7_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slv_en_qpi_int_clr(&mut self) -> SLV_EN_QPI_INT_CLR_W {
        SLV_EN_QPI_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slv_ex_qpi_int_clr(&mut self) -> SLV_EX_QPI_INT_CLR_W {
        SLV_EX_QPI_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_outfifo_empty_err_int_clr(&mut self) -> DMA_OUTFIFO_EMPTY_ERR_INT_CLR_W {
        DMA_OUTFIFO_EMPTY_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_infifo_full_err_int_clr(&mut self) -> DMA_INFIFO_FULL_ERR_INT_CLR_W {
        DMA_INFIFO_FULL_ERR_INT_CLR_W { w: self }
    }
}
