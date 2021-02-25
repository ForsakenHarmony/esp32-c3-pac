#[doc = "Reader of register DMA_INT_ST"]
pub type R = crate::R<u32, super::DMA_INT_ST>;
#[doc = "Reader of field `APP1_INT_ST`"]
pub type APP1_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `APP2_INT_ST`"]
pub type APP2_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `MST_TX_AFIFO_REMPTY_ERR_INT_ST`"]
pub type MST_TX_AFIFO_REMPTY_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `MST_RX_AFIFO_WFULL_ERR_INT_ST`"]
pub type MST_RX_AFIFO_WFULL_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLV_CMD_ERR_INT_ST`"]
pub type SLV_CMD_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLV_BUF_ADDR_ERR_INT_ST`"]
pub type SLV_BUF_ADDR_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEG_MAGIC_ERR_INT_ST`"]
pub type SEG_MAGIC_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_SEG_TRANS_DONE_INT_ST`"]
pub type DMA_SEG_TRANS_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRANS_DONE_INT_ST`"]
pub type TRANS_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLV_WR_BUF_DONE_INT_ST`"]
pub type SLV_WR_BUF_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLV_RD_BUF_DONE_INT_ST`"]
pub type SLV_RD_BUF_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLV_WR_DMA_DONE_INT_ST`"]
pub type SLV_WR_DMA_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLV_RD_DMA_DONE_INT_ST`"]
pub type SLV_RD_DMA_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLV_CMDA_INT_ST`"]
pub type SLV_CMDA_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLV_CMD9_INT_ST`"]
pub type SLV_CMD9_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLV_CMD8_INT_ST`"]
pub type SLV_CMD8_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLV_CMD7_INT_ST`"]
pub type SLV_CMD7_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLV_EN_QPI_INT_ST`"]
pub type SLV_EN_QPI_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLV_EX_QPI_INT_ST`"]
pub type SLV_EX_QPI_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_OUTFIFO_EMPTY_ERR_INT_ST`"]
pub type DMA_OUTFIFO_EMPTY_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_INFIFO_FULL_ERR_INT_ST`"]
pub type DMA_INFIFO_FULL_ERR_INT_ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn app1_int_st(&self) -> APP1_INT_ST_R {
        APP1_INT_ST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn app2_int_st(&self) -> APP2_INT_ST_R {
        APP2_INT_ST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn mst_tx_afifo_rempty_err_int_st(&self) -> MST_TX_AFIFO_REMPTY_ERR_INT_ST_R {
        MST_TX_AFIFO_REMPTY_ERR_INT_ST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn mst_rx_afifo_wfull_err_int_st(&self) -> MST_RX_AFIFO_WFULL_ERR_INT_ST_R {
        MST_RX_AFIFO_WFULL_ERR_INT_ST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slv_cmd_err_int_st(&self) -> SLV_CMD_ERR_INT_ST_R {
        SLV_CMD_ERR_INT_ST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slv_buf_addr_err_int_st(&self) -> SLV_BUF_ADDR_ERR_INT_ST_R {
        SLV_BUF_ADDR_ERR_INT_ST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn seg_magic_err_int_st(&self) -> SEG_MAGIC_ERR_INT_ST_R {
        SEG_MAGIC_ERR_INT_ST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dma_seg_trans_done_int_st(&self) -> DMA_SEG_TRANS_DONE_INT_ST_R {
        DMA_SEG_TRANS_DONE_INT_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn trans_done_int_st(&self) -> TRANS_DONE_INT_ST_R {
        TRANS_DONE_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slv_wr_buf_done_int_st(&self) -> SLV_WR_BUF_DONE_INT_ST_R {
        SLV_WR_BUF_DONE_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slv_rd_buf_done_int_st(&self) -> SLV_RD_BUF_DONE_INT_ST_R {
        SLV_RD_BUF_DONE_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slv_wr_dma_done_int_st(&self) -> SLV_WR_DMA_DONE_INT_ST_R {
        SLV_WR_DMA_DONE_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slv_rd_dma_done_int_st(&self) -> SLV_RD_DMA_DONE_INT_ST_R {
        SLV_RD_DMA_DONE_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn slv_cmda_int_st(&self) -> SLV_CMDA_INT_ST_R {
        SLV_CMDA_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slv_cmd9_int_st(&self) -> SLV_CMD9_INT_ST_R {
        SLV_CMD9_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slv_cmd8_int_st(&self) -> SLV_CMD8_INT_ST_R {
        SLV_CMD8_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slv_cmd7_int_st(&self) -> SLV_CMD7_INT_ST_R {
        SLV_CMD7_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slv_en_qpi_int_st(&self) -> SLV_EN_QPI_INT_ST_R {
        SLV_EN_QPI_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slv_ex_qpi_int_st(&self) -> SLV_EX_QPI_INT_ST_R {
        SLV_EX_QPI_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_outfifo_empty_err_int_st(&self) -> DMA_OUTFIFO_EMPTY_ERR_INT_ST_R {
        DMA_OUTFIFO_EMPTY_ERR_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_infifo_full_err_int_st(&self) -> DMA_INFIFO_FULL_ERR_INT_ST_R {
        DMA_INFIFO_FULL_ERR_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
