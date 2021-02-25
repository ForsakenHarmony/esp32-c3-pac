#[doc = "Reader of register ID"]
pub type R = crate::R<u32, super::ID>;
#[doc = "Writer for register ID"]
pub type W = crate::W<u32, super::ID>;
#[doc = "Register ID `reset()`'s with value 0"]
impl crate::ResetValue for super::ID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UPDATE`"]
pub type UPDATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPDATE`"]
pub struct UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDATE_W<'a> {
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
#[doc = "Reader of field `HIGH_SPEED`"]
pub type HIGH_SPEED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HIGH_SPEED`"]
pub struct HIGH_SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGH_SPEED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ID`"]
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | ((value as u32) & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn high_speed(&self) -> HIGH_SPEED_R {
        HIGH_SPEED_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn update(&mut self) -> UPDATE_W {
        UPDATE_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn high_speed(&mut self) -> HIGH_SPEED_W {
        HIGH_SPEED_W { w: self }
    }
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
}
