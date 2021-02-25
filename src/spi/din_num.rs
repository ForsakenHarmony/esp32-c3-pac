#[doc = "Reader of register DIN_NUM"]
pub type R = crate::R<u32, super::DIN_NUM>;
#[doc = "Writer for register DIN_NUM"]
pub type W = crate::W<u32, super::DIN_NUM>;
#[doc = "Register DIN_NUM `reset()`'s with value 0"]
impl crate::ResetValue for super::DIN_NUM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIN3_NUM`"]
pub type DIN3_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIN3_NUM`"]
pub struct DIN3_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN3_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `DIN2_NUM`"]
pub type DIN2_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIN2_NUM`"]
pub struct DIN2_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN2_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `DIN1_NUM`"]
pub type DIN1_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIN1_NUM`"]
pub struct DIN1_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN1_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `DIN0_NUM`"]
pub type DIN0_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIN0_NUM`"]
pub struct DIN0_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN0_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn din3_num(&self) -> DIN3_NUM_R {
        DIN3_NUM_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn din2_num(&self) -> DIN2_NUM_R {
        DIN2_NUM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn din1_num(&self) -> DIN1_NUM_R {
        DIN1_NUM_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn din0_num(&self) -> DIN0_NUM_R {
        DIN0_NUM_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn din3_num(&mut self) -> DIN3_NUM_W {
        DIN3_NUM_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn din2_num(&mut self) -> DIN2_NUM_W {
        DIN2_NUM_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn din1_num(&mut self) -> DIN1_NUM_W {
        DIN1_NUM_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn din0_num(&mut self) -> DIN0_NUM_W {
        DIN0_NUM_W { w: self }
    }
}
