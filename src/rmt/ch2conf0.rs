#[doc = "Reader of register CH2CONF0"]
pub type R = crate::R<u32, super::CH2CONF0>;
#[doc = "Writer for register CH2CONF0"]
pub type W = crate::W<u32, super::CH2CONF0>;
#[doc = "Register CH2CONF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CH2CONF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CARRIER_OUT_LV_CH2`"]
pub type CARRIER_OUT_LV_CH2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARRIER_OUT_LV_CH2`"]
pub struct CARRIER_OUT_LV_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_OUT_LV_CH2_W<'a> {
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
#[doc = "Reader of field `CARRIER_EN_CH2`"]
pub type CARRIER_EN_CH2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARRIER_EN_CH2`"]
pub struct CARRIER_EN_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_EN_CH2_W<'a> {
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
#[doc = "Reader of field `MEM_SIZE_CH2`"]
pub type MEM_SIZE_CH2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_SIZE_CH2`"]
pub struct MEM_SIZE_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SIZE_CH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | (((value as u32) & 0x07) << 23);
        self.w
    }
}
#[doc = "Reader of field `IDLE_THRES_CH2`"]
pub type IDLE_THRES_CH2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IDLE_THRES_CH2`"]
pub struct IDLE_THRES_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_THRES_CH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 8)) | (((value as u32) & 0x7fff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DIV_CNT_CH2`"]
pub type DIV_CNT_CH2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_CNT_CH2`"]
pub struct DIV_CNT_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_CNT_CH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn carrier_out_lv_ch2(&self) -> CARRIER_OUT_LV_CH2_R {
        CARRIER_OUT_LV_CH2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn carrier_en_ch2(&self) -> CARRIER_EN_CH2_R {
        CARRIER_EN_CH2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn mem_size_ch2(&self) -> MEM_SIZE_CH2_R {
        MEM_SIZE_CH2_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 8:22"]
    #[inline(always)]
    pub fn idle_thres_ch2(&self) -> IDLE_THRES_CH2_R {
        IDLE_THRES_CH2_R::new(((self.bits >> 8) & 0x7fff) as u16)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn div_cnt_ch2(&self) -> DIV_CNT_CH2_R {
        DIV_CNT_CH2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn carrier_out_lv_ch2(&mut self) -> CARRIER_OUT_LV_CH2_W {
        CARRIER_OUT_LV_CH2_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn carrier_en_ch2(&mut self) -> CARRIER_EN_CH2_W {
        CARRIER_EN_CH2_W { w: self }
    }
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn mem_size_ch2(&mut self) -> MEM_SIZE_CH2_W {
        MEM_SIZE_CH2_W { w: self }
    }
    #[doc = "Bits 8:22"]
    #[inline(always)]
    pub fn idle_thres_ch2(&mut self) -> IDLE_THRES_CH2_W {
        IDLE_THRES_CH2_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn div_cnt_ch2(&mut self) -> DIV_CNT_CH2_W {
        DIV_CNT_CH2_W { w: self }
    }
}
