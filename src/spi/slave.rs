#[doc = "Reader of register SLAVE"]
pub type R = crate::R<u32, super::SLAVE>;
#[doc = "Writer for register SLAVE"]
pub type W = crate::W<u32, super::SLAVE>;
#[doc = "Register SLAVE `reset()`'s with value 0"]
impl crate::ResetValue for super::SLAVE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USR_CONF`"]
pub type USR_CONF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USR_CONF`"]
pub struct USR_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_CONF_W<'a> {
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
#[doc = "Write proxy for field `SOFT_RESET`"]
pub struct SOFT_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_RESET_W<'a> {
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
#[doc = "Reader of field `SLAVE_MODE`"]
pub type SLAVE_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAVE_MODE`"]
pub struct SLAVE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `DMA_SEG_MAGIC_VALUE`"]
pub type DMA_SEG_MAGIC_VALUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMA_SEG_MAGIC_VALUE`"]
pub struct DMA_SEG_MAGIC_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SEG_MAGIC_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 22)) | (((value as u32) & 0x0f) << 22);
        self.w
    }
}
#[doc = "Reader of field `SLV_WRBUF_BITLEN_EN`"]
pub type SLV_WRBUF_BITLEN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_WRBUF_BITLEN_EN`"]
pub struct SLV_WRBUF_BITLEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WRBUF_BITLEN_EN_W<'a> {
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
#[doc = "Reader of field `SLV_RDBUF_BITLEN_EN`"]
pub type SLV_RDBUF_BITLEN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_RDBUF_BITLEN_EN`"]
pub struct SLV_RDBUF_BITLEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RDBUF_BITLEN_EN_W<'a> {
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
#[doc = "Reader of field `SLV_WRDMA_BITLEN_EN`"]
pub type SLV_WRDMA_BITLEN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_WRDMA_BITLEN_EN`"]
pub struct SLV_WRDMA_BITLEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WRDMA_BITLEN_EN_W<'a> {
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
#[doc = "Reader of field `SLV_RDDMA_BITLEN_EN`"]
pub type SLV_RDDMA_BITLEN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_RDDMA_BITLEN_EN`"]
pub struct SLV_RDDMA_BITLEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RDDMA_BITLEN_EN_W<'a> {
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
#[doc = "Reader of field `RSCK_DATA_OUT`"]
pub type RSCK_DATA_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSCK_DATA_OUT`"]
pub struct RSCK_DATA_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSCK_DATA_OUT_W<'a> {
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
#[doc = "Reader of field `CLK_MODE_13`"]
pub type CLK_MODE_13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_MODE_13`"]
pub struct CLK_MODE_13_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_MODE_13_W<'a> {
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
#[doc = "Reader of field `CLK_MODE`"]
pub type CLK_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLK_MODE`"]
pub struct CLK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn usr_conf(&self) -> USR_CONF_R {
        USR_CONF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn slave_mode(&self) -> SLAVE_MODE_R {
        SLAVE_MODE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn dma_seg_magic_value(&self) -> DMA_SEG_MAGIC_VALUE_R {
        DMA_SEG_MAGIC_VALUE_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slv_wrbuf_bitlen_en(&self) -> SLV_WRBUF_BITLEN_EN_R {
        SLV_WRBUF_BITLEN_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slv_rdbuf_bitlen_en(&self) -> SLV_RDBUF_BITLEN_EN_R {
        SLV_RDBUF_BITLEN_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slv_wrdma_bitlen_en(&self) -> SLV_WRDMA_BITLEN_EN_R {
        SLV_WRDMA_BITLEN_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slv_rddma_bitlen_en(&self) -> SLV_RDDMA_BITLEN_EN_R {
        SLV_RDDMA_BITLEN_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rsck_data_out(&self) -> RSCK_DATA_OUT_R {
        RSCK_DATA_OUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk_mode_13(&self) -> CLK_MODE_13_R {
        CLK_MODE_13_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn clk_mode(&self) -> CLK_MODE_R {
        CLK_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn usr_conf(&mut self) -> USR_CONF_W {
        USR_CONF_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn soft_reset(&mut self) -> SOFT_RESET_W {
        SOFT_RESET_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn slave_mode(&mut self) -> SLAVE_MODE_W {
        SLAVE_MODE_W { w: self }
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn dma_seg_magic_value(&mut self) -> DMA_SEG_MAGIC_VALUE_W {
        DMA_SEG_MAGIC_VALUE_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slv_wrbuf_bitlen_en(&mut self) -> SLV_WRBUF_BITLEN_EN_W {
        SLV_WRBUF_BITLEN_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slv_rdbuf_bitlen_en(&mut self) -> SLV_RDBUF_BITLEN_EN_W {
        SLV_RDBUF_BITLEN_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slv_wrdma_bitlen_en(&mut self) -> SLV_WRDMA_BITLEN_EN_W {
        SLV_WRDMA_BITLEN_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slv_rddma_bitlen_en(&mut self) -> SLV_RDDMA_BITLEN_EN_W {
        SLV_RDDMA_BITLEN_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rsck_data_out(&mut self) -> RSCK_DATA_OUT_W {
        RSCK_DATA_OUT_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk_mode_13(&mut self) -> CLK_MODE_13_W {
        CLK_MODE_13_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn clk_mode(&mut self) -> CLK_MODE_W {
        CLK_MODE_W { w: self }
    }
}
