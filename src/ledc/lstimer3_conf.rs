#[doc = "Reader of register LSTIMER3_CONF"]
pub type R = crate::R<u32, super::LSTIMER3_CONF>;
#[doc = "Writer for register LSTIMER3_CONF"]
pub type W = crate::W<u32, super::LSTIMER3_CONF>;
#[doc = "Register LSTIMER3_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::LSTIMER3_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `LSTIMER3_PARA_UP`"]
pub struct LSTIMER3_PARA_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER3_PARA_UP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `TICK_SEL_LSTIMER3`"]
pub type TICK_SEL_LSTIMER3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TICK_SEL_LSTIMER3`"]
pub struct TICK_SEL_LSTIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> TICK_SEL_LSTIMER3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `LSTIMER3_RST`"]
pub type LSTIMER3_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSTIMER3_RST`"]
pub struct LSTIMER3_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER3_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `LSTIMER3_PAUSE`"]
pub type LSTIMER3_PAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSTIMER3_PAUSE`"]
pub struct LSTIMER3_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER3_PAUSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `CLK_DIV_LSTIMER3`"]
pub type CLK_DIV_LSTIMER3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CLK_DIV_LSTIMER3`"]
pub struct CLK_DIV_LSTIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DIV_LSTIMER3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 4)) | (((value as u32) & 0x0003_ffff) << 4);
        self.w
    }
}
#[doc = "Reader of field `LSTIMER3_DUTY_RES`"]
pub type LSTIMER3_DUTY_RES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LSTIMER3_DUTY_RES`"]
pub struct LSTIMER3_DUTY_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER3_DUTY_RES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tick_sel_lstimer3(&self) -> TICK_SEL_LSTIMER3_R {
        TICK_SEL_LSTIMER3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn lstimer3_rst(&self) -> LSTIMER3_RST_R {
        LSTIMER3_RST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn lstimer3_pause(&self) -> LSTIMER3_PAUSE_R {
        LSTIMER3_PAUSE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 4:21"]
    #[inline(always)]
    pub fn clk_div_lstimer3(&self) -> CLK_DIV_LSTIMER3_R {
        CLK_DIV_LSTIMER3_R::new(((self.bits >> 4) & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn lstimer3_duty_res(&self) -> LSTIMER3_DUTY_RES_R {
        LSTIMER3_DUTY_RES_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn lstimer3_para_up(&mut self) -> LSTIMER3_PARA_UP_W {
        LSTIMER3_PARA_UP_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tick_sel_lstimer3(&mut self) -> TICK_SEL_LSTIMER3_W {
        TICK_SEL_LSTIMER3_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn lstimer3_rst(&mut self) -> LSTIMER3_RST_W {
        LSTIMER3_RST_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn lstimer3_pause(&mut self) -> LSTIMER3_PAUSE_W {
        LSTIMER3_PAUSE_W { w: self }
    }
    #[doc = "Bits 4:21"]
    #[inline(always)]
    pub fn clk_div_lstimer3(&mut self) -> CLK_DIV_LSTIMER3_W {
        CLK_DIV_LSTIMER3_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn lstimer3_duty_res(&mut self) -> LSTIMER3_DUTY_RES_W {
        LSTIMER3_DUTY_RES_W { w: self }
    }
}
