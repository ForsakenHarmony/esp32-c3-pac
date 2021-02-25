#[doc = "Reader of register USER1"]
pub type R = crate::R<u32, super::USER1>;
#[doc = "Writer for register USER1"]
pub type W = crate::W<u32, super::USER1>;
#[doc = "Register USER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::USER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USR_ADDR_BITLEN`"]
pub type USR_ADDR_BITLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USR_ADDR_BITLEN`"]
pub struct USR_ADDR_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_ADDR_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
#[doc = "Reader of field `CS_HOLD_TIME`"]
pub type CS_HOLD_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CS_HOLD_TIME`"]
pub struct CS_HOLD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_HOLD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 22)) | (((value as u32) & 0x1f) << 22);
        self.w
    }
}
#[doc = "Reader of field `CS_SETUP_TIME`"]
pub type CS_SETUP_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CS_SETUP_TIME`"]
pub struct CS_SETUP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_SETUP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | (((value as u32) & 0x1f) << 17);
        self.w
    }
}
#[doc = "Reader of field `MST_WFULL_ERR_END_EN`"]
pub type MST_WFULL_ERR_END_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MST_WFULL_ERR_END_EN`"]
pub struct MST_WFULL_ERR_END_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MST_WFULL_ERR_END_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `USR_DUMMY_CYCLELEN`"]
pub type USR_DUMMY_CYCLELEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USR_DUMMY_CYCLELEN`"]
pub struct USR_DUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_DUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn usr_addr_bitlen(&self) -> USR_ADDR_BITLEN_R {
        USR_ADDR_BITLEN_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26"]
    #[inline(always)]
    pub fn cs_hold_time(&self) -> CS_HOLD_TIME_R {
        CS_HOLD_TIME_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn cs_setup_time(&self) -> CS_SETUP_TIME_R {
        CS_SETUP_TIME_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mst_wfull_err_end_en(&self) -> MST_WFULL_ERR_END_EN_R {
        MST_WFULL_ERR_END_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn usr_dummy_cyclelen(&self) -> USR_DUMMY_CYCLELEN_R {
        USR_DUMMY_CYCLELEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn usr_addr_bitlen(&mut self) -> USR_ADDR_BITLEN_W {
        USR_ADDR_BITLEN_W { w: self }
    }
    #[doc = "Bits 22:26"]
    #[inline(always)]
    pub fn cs_hold_time(&mut self) -> CS_HOLD_TIME_W {
        CS_HOLD_TIME_W { w: self }
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn cs_setup_time(&mut self) -> CS_SETUP_TIME_W {
        CS_SETUP_TIME_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mst_wfull_err_end_en(&mut self) -> MST_WFULL_ERR_END_EN_W {
        MST_WFULL_ERR_END_EN_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn usr_dummy_cyclelen(&mut self) -> USR_DUMMY_CYCLELEN_W {
        USR_DUMMY_CYCLELEN_W { w: self }
    }
}
