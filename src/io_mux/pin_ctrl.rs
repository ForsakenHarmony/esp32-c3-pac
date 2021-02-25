#[doc = "Reader of register PIN_CTRL"]
pub type R = crate::R<u32, super::PIN_CTRL>;
#[doc = "Writer for register PIN_CTRL"]
pub type W = crate::W<u32, super::PIN_CTRL>;
#[doc = "Register PIN_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::PIN_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PIN_CTRL_CLK3`"]
pub type PIN_CTRL_CLK3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PIN_CTRL_CLK3`"]
pub struct PIN_CTRL_CLK3_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_CTRL_CLK3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `PIN_CTRL_CLK2`"]
pub type PIN_CTRL_CLK2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PIN_CTRL_CLK2`"]
pub struct PIN_CTRL_CLK2_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_CTRL_CLK2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `PIN_CTRL_CLK1`"]
pub type PIN_CTRL_CLK1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PIN_CTRL_CLK1`"]
pub struct PIN_CTRL_CLK1_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_CTRL_CLK1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pin_ctrl_clk3(&self) -> PIN_CTRL_CLK3_R {
        PIN_CTRL_CLK3_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn pin_ctrl_clk2(&self) -> PIN_CTRL_CLK2_R {
        PIN_CTRL_CLK2_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn pin_ctrl_clk1(&self) -> PIN_CTRL_CLK1_R {
        PIN_CTRL_CLK1_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pin_ctrl_clk3(&mut self) -> PIN_CTRL_CLK3_W {
        PIN_CTRL_CLK3_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn pin_ctrl_clk2(&mut self) -> PIN_CTRL_CLK2_W {
        PIN_CTRL_CLK2_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn pin_ctrl_clk1(&mut self) -> PIN_CTRL_CLK1_W {
        PIN_CTRL_CLK1_W { w: self }
    }
}
