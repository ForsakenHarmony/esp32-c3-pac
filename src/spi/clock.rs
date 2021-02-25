#[doc = "Reader of register CLOCK"]
pub type R = crate::R<u32, super::CLOCK>;
#[doc = "Writer for register CLOCK"]
pub type W = crate::W<u32, super::CLOCK>;
#[doc = "Register CLOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::CLOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLK_EQU_SYSCLK`"]
pub type CLK_EQU_SYSCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_EQU_SYSCLK`"]
pub struct CLK_EQU_SYSCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EQU_SYSCLK_W<'a> {
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
#[doc = "Reader of field `CLKDIV_PRE`"]
pub type CLKDIV_PRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKDIV_PRE`"]
pub struct CLKDIV_PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_PRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
#[doc = "Reader of field `CLKCNT_N`"]
pub type CLKCNT_N_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKCNT_N`"]
pub struct CLKCNT_N_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKCNT_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `CLKCNT_H`"]
pub type CLKCNT_H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKCNT_H`"]
pub struct CLKCNT_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKCNT_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `CLKCNT_L`"]
pub type CLKCNT_L_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKCNT_L`"]
pub struct CLKCNT_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKCNT_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn clk_equ_sysclk(&self) -> CLK_EQU_SYSCLK_R {
        CLK_EQU_SYSCLK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn clkdiv_pre(&self) -> CLKDIV_PRE_R {
        CLKDIV_PRE_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn clkcnt_n(&self) -> CLKCNT_N_R {
        CLKCNT_N_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn clkcnt_h(&self) -> CLKCNT_H_R {
        CLKCNT_H_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn clkcnt_l(&self) -> CLKCNT_L_R {
        CLKCNT_L_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn clk_equ_sysclk(&mut self) -> CLK_EQU_SYSCLK_W {
        CLK_EQU_SYSCLK_W { w: self }
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn clkdiv_pre(&mut self) -> CLKDIV_PRE_W {
        CLKDIV_PRE_W { w: self }
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn clkcnt_n(&mut self) -> CLKCNT_N_W {
        CLKCNT_N_W { w: self }
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn clkcnt_h(&mut self) -> CLKCNT_H_W {
        CLKCNT_H_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn clkcnt_l(&mut self) -> CLKCNT_L_W {
        CLKCNT_L_W { w: self }
    }
}
