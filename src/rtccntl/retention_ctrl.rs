#[doc = "Reader of register RETENTION_CTRL"]
pub type R = crate::R<u32, super::RETENTION_CTRL>;
#[doc = "Writer for register RETENTION_CTRL"]
pub type W = crate::W<u32, super::RETENTION_CTRL>;
#[doc = "Register RETENTION_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::RETENTION_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RETENTION_WAIT`"]
pub type RETENTION_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RETENTION_WAIT`"]
pub struct RETENTION_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RETENTION_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
#[doc = "Reader of field `RETENTION_EN`"]
pub type RETENTION_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RETENTION_EN`"]
pub struct RETENTION_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RETENTION_EN_W<'a> {
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
#[doc = "Reader of field `RETENTION_CLKOFF_WAIT`"]
pub type RETENTION_CLKOFF_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RETENTION_CLKOFF_WAIT`"]
pub struct RETENTION_CLKOFF_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RETENTION_CLKOFF_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 22)) | (((value as u32) & 0x0f) << 22);
        self.w
    }
}
#[doc = "Reader of field `RETENTION_DONE_WAIT`"]
pub type RETENTION_DONE_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RETENTION_DONE_WAIT`"]
pub struct RETENTION_DONE_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RETENTION_DONE_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Reader of field `RETENTION_CLK_SEL`"]
pub type RETENTION_CLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RETENTION_CLK_SEL`"]
pub struct RETENTION_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RETENTION_CLK_SEL_W<'a> {
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
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn retention_wait(&self) -> RETENTION_WAIT_R {
        RETENTION_WAIT_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn retention_en(&self) -> RETENTION_EN_R {
        RETENTION_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn retention_clkoff_wait(&self) -> RETENTION_CLKOFF_WAIT_R {
        RETENTION_CLKOFF_WAIT_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn retention_done_wait(&self) -> RETENTION_DONE_WAIT_R {
        RETENTION_DONE_WAIT_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn retention_clk_sel(&self) -> RETENTION_CLK_SEL_R {
        RETENTION_CLK_SEL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn retention_wait(&mut self) -> RETENTION_WAIT_W {
        RETENTION_WAIT_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn retention_en(&mut self) -> RETENTION_EN_W {
        RETENTION_EN_W { w: self }
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn retention_clkoff_wait(&mut self) -> RETENTION_CLKOFF_WAIT_W {
        RETENTION_CLKOFF_WAIT_W { w: self }
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn retention_done_wait(&mut self) -> RETENTION_DONE_WAIT_W {
        RETENTION_DONE_WAIT_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn retention_clk_sel(&mut self) -> RETENTION_CLK_SEL_W {
        RETENTION_CLK_SEL_W { w: self }
    }
}
