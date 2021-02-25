#[doc = "Reader of register SLAVE1"]
pub type R = crate::R<u32, super::SLAVE1>;
#[doc = "Writer for register SLAVE1"]
pub type W = crate::W<u32, super::SLAVE1>;
#[doc = "Register SLAVE1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SLAVE1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLV_LAST_ADDR`"]
pub type SLV_LAST_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLV_LAST_ADDR`"]
pub struct SLV_LAST_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_LAST_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
#[doc = "Reader of field `SLV_LAST_COMMAND`"]
pub type SLV_LAST_COMMAND_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLV_LAST_COMMAND`"]
pub struct SLV_LAST_COMMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_LAST_COMMAND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 18)) | (((value as u32) & 0xff) << 18);
        self.w
    }
}
#[doc = "Reader of field `SLV_DATA_BITLEN`"]
pub type SLV_DATA_BITLEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SLV_DATA_BITLEN`"]
pub struct SLV_DATA_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_DATA_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn slv_last_addr(&self) -> SLV_LAST_ADDR_R {
        SLV_LAST_ADDR_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bits 18:25"]
    #[inline(always)]
    pub fn slv_last_command(&self) -> SLV_LAST_COMMAND_R {
        SLV_LAST_COMMAND_R::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn slv_data_bitlen(&self) -> SLV_DATA_BITLEN_R {
        SLV_DATA_BITLEN_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn slv_last_addr(&mut self) -> SLV_LAST_ADDR_W {
        SLV_LAST_ADDR_W { w: self }
    }
    #[doc = "Bits 18:25"]
    #[inline(always)]
    pub fn slv_last_command(&mut self) -> SLV_LAST_COMMAND_W {
        SLV_LAST_COMMAND_W { w: self }
    }
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn slv_data_bitlen(&mut self) -> SLV_DATA_BITLEN_W {
        SLV_DATA_BITLEN_W { w: self }
    }
}
