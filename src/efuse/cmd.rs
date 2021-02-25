#[doc = "Reader of register CMD"]
pub type R = crate::R<u32, super::CMD>;
#[doc = "Writer for register CMD"]
pub type W = crate::W<u32, super::CMD>;
#[doc = "Register CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLK_NUM`"]
pub type BLK_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BLK_NUM`"]
pub struct BLK_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> BLK_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Reader of field `PGM_CMD`"]
pub type PGM_CMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGM_CMD`"]
pub struct PGM_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_CMD_W<'a> {
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
#[doc = "Reader of field `READ_CMD`"]
pub type READ_CMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `READ_CMD`"]
pub struct READ_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_CMD_W<'a> {
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
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn blk_num(&self) -> BLK_NUM_R {
        BLK_NUM_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pgm_cmd(&self) -> PGM_CMD_R {
        PGM_CMD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn read_cmd(&self) -> READ_CMD_R {
        READ_CMD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn blk_num(&mut self) -> BLK_NUM_W {
        BLK_NUM_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pgm_cmd(&mut self) -> PGM_CMD_W {
        PGM_CMD_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn read_cmd(&mut self) -> READ_CMD_W {
        READ_CMD_W { w: self }
    }
}
