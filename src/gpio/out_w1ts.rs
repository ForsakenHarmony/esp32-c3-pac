#[doc = "Writer for register OUT_W1TS"]
pub type W = crate::W<u32, super::OUT_W1TS>;
#[doc = "Register OUT_W1TS `reset()`'s with value 0"]
impl crate::ResetValue for super::OUT_W1TS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `OUT_W1TS`"]
pub struct OUT_W1TS_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_W1TS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | ((value as u32) & 0x03ff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn out_w1ts(&mut self) -> OUT_W1TS_W {
        OUT_W1TS_W { w: self }
    }
}
