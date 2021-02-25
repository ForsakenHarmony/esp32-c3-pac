#[doc = "Reader of register FIB_SEL"]
pub type R = crate::R<u32, super::FIB_SEL>;
#[doc = "Writer for register FIB_SEL"]
pub type W = crate::W<u32, super::FIB_SEL>;
#[doc = "Register FIB_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::FIB_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIB_SEL`"]
pub type FIB_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIB_SEL`"]
pub struct FIB_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FIB_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn fib_sel(&self) -> FIB_SEL_R {
        FIB_SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn fib_sel(&mut self) -> FIB_SEL_W {
        FIB_SEL_W { w: self }
    }
}
