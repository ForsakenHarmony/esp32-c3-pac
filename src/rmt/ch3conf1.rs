#[doc = "Reader of register CH3CONF1"]
pub type R = crate::R<u32, super::CH3CONF1>;
#[doc = "Writer for register CH3CONF1"]
pub type W = crate::W<u32, super::CH3CONF1>;
#[doc = "Register CH3CONF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CH3CONF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CONF_UPDATE_CH3`"]
pub struct CONF_UPDATE_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> CONF_UPDATE_CH3_W<'a> {
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
#[doc = "Write proxy for field `AFIFO_RST_CH3`"]
pub struct AFIFO_RST_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> AFIFO_RST_CH3_W<'a> {
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
#[doc = "Reader of field `MEM_RX_WRAP_EN_CH3`"]
pub type MEM_RX_WRAP_EN_CH3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEM_RX_WRAP_EN_CH3`"]
pub struct MEM_RX_WRAP_EN_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_RX_WRAP_EN_CH3_W<'a> {
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
#[doc = "Reader of field `RX_FILTER_THRES_CH3`"]
pub type RX_FILTER_THRES_CH3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_FILTER_THRES_CH3`"]
pub struct RX_FILTER_THRES_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FILTER_THRES_CH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 5)) | (((value as u32) & 0xff) << 5);
        self.w
    }
}
#[doc = "Reader of field `RX_FILTER_EN_CH3`"]
pub type RX_FILTER_EN_CH3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_FILTER_EN_CH3`"]
pub struct RX_FILTER_EN_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FILTER_EN_CH3_W<'a> {
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
#[doc = "Reader of field `MEM_OWNER_CH3`"]
pub type MEM_OWNER_CH3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEM_OWNER_CH3`"]
pub struct MEM_OWNER_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_OWNER_CH3_W<'a> {
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
#[doc = "Write proxy for field `APB_MEM_RST_CH3`"]
pub struct APB_MEM_RST_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_MEM_RST_CH3_W<'a> {
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
#[doc = "Write proxy for field `MEM_WR_RST_CH3`"]
pub struct MEM_WR_RST_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_WR_RST_CH3_W<'a> {
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
#[doc = "Reader of field `RX_EN_CH3`"]
pub type RX_EN_CH3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_EN_CH3`"]
pub struct RX_EN_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_EN_CH3_W<'a> {
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
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn mem_rx_wrap_en_ch3(&self) -> MEM_RX_WRAP_EN_CH3_R {
        MEM_RX_WRAP_EN_CH3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 5:12"]
    #[inline(always)]
    pub fn rx_filter_thres_ch3(&self) -> RX_FILTER_THRES_CH3_R {
        RX_FILTER_THRES_CH3_R::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rx_filter_en_ch3(&self) -> RX_FILTER_EN_CH3_R {
        RX_FILTER_EN_CH3_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn mem_owner_ch3(&self) -> MEM_OWNER_CH3_R {
        MEM_OWNER_CH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_en_ch3(&self) -> RX_EN_CH3_R {
        RX_EN_CH3_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn conf_update_ch3(&mut self) -> CONF_UPDATE_CH3_W {
        CONF_UPDATE_CH3_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn afifo_rst_ch3(&mut self) -> AFIFO_RST_CH3_W {
        AFIFO_RST_CH3_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn mem_rx_wrap_en_ch3(&mut self) -> MEM_RX_WRAP_EN_CH3_W {
        MEM_RX_WRAP_EN_CH3_W { w: self }
    }
    #[doc = "Bits 5:12"]
    #[inline(always)]
    pub fn rx_filter_thres_ch3(&mut self) -> RX_FILTER_THRES_CH3_W {
        RX_FILTER_THRES_CH3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rx_filter_en_ch3(&mut self) -> RX_FILTER_EN_CH3_W {
        RX_FILTER_EN_CH3_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn mem_owner_ch3(&mut self) -> MEM_OWNER_CH3_W {
        MEM_OWNER_CH3_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn apb_mem_rst_ch3(&mut self) -> APB_MEM_RST_CH3_W {
        APB_MEM_RST_CH3_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mem_wr_rst_ch3(&mut self) -> MEM_WR_RST_CH3_W {
        MEM_WR_RST_CH3_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_en_ch3(&mut self) -> RX_EN_CH3_W {
        RX_EN_CH3_W { w: self }
    }
}
