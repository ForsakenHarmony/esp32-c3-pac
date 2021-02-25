#[doc = "Reader of register RX_TIMING"]
pub type R = crate::R<u32, super::RX_TIMING>;
#[doc = "Writer for register RX_TIMING"]
pub type W = crate::W<u32, super::RX_TIMING>;
#[doc = "Register RX_TIMING `reset()`'s with value 0"]
impl crate::ResetValue for super::RX_TIMING {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_BCK_IN_DM`"]
pub type RX_BCK_IN_DM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_BCK_IN_DM`"]
pub struct RX_BCK_IN_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BCK_IN_DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `RX_WS_IN_DM`"]
pub type RX_WS_IN_DM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_WS_IN_DM`"]
pub struct RX_WS_IN_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_WS_IN_DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `RX_BCK_OUT_DM`"]
pub type RX_BCK_OUT_DM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_BCK_OUT_DM`"]
pub struct RX_BCK_OUT_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BCK_OUT_DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `RX_WS_OUT_DM`"]
pub type RX_WS_OUT_DM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_WS_OUT_DM`"]
pub struct RX_WS_OUT_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_WS_OUT_DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `RX_SD_IN_DM`"]
pub type RX_SD_IN_DM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_SD_IN_DM`"]
pub struct RX_SD_IN_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SD_IN_DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn rx_bck_in_dm(&self) -> RX_BCK_IN_DM_R {
        RX_BCK_IN_DM_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rx_ws_in_dm(&self) -> RX_WS_IN_DM_R {
        RX_WS_IN_DM_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn rx_bck_out_dm(&self) -> RX_BCK_OUT_DM_R {
        RX_BCK_OUT_DM_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rx_ws_out_dm(&self) -> RX_WS_OUT_DM_R {
        RX_WS_OUT_DM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rx_sd_in_dm(&self) -> RX_SD_IN_DM_R {
        RX_SD_IN_DM_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn rx_bck_in_dm(&mut self) -> RX_BCK_IN_DM_W {
        RX_BCK_IN_DM_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rx_ws_in_dm(&mut self) -> RX_WS_IN_DM_W {
        RX_WS_IN_DM_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn rx_bck_out_dm(&mut self) -> RX_BCK_OUT_DM_W {
        RX_BCK_OUT_DM_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rx_ws_out_dm(&mut self) -> RX_WS_OUT_DM_W {
        RX_WS_OUT_DM_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rx_sd_in_dm(&mut self) -> RX_SD_IN_DM_W {
        RX_SD_IN_DM_W { w: self }
    }
}
