#[doc = "Reader of register USER"]
pub type R = crate::R<u32, super::USER>;
#[doc = "Writer for register USER"]
pub type W = crate::W<u32, super::USER>;
#[doc = "Register USER `reset()`'s with value 0"]
impl crate::ResetValue for super::USER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USR_COMMAND`"]
pub type USR_COMMAND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USR_COMMAND`"]
pub struct USR_COMMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_COMMAND_W<'a> {
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
#[doc = "Reader of field `USR_ADDR`"]
pub type USR_ADDR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USR_ADDR`"]
pub struct USR_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_ADDR_W<'a> {
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
#[doc = "Reader of field `USR_DUMMY`"]
pub type USR_DUMMY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USR_DUMMY`"]
pub struct USR_DUMMY_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_DUMMY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `USR_MISO`"]
pub type USR_MISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USR_MISO`"]
pub struct USR_MISO_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_MISO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `USR_MOSI`"]
pub type USR_MOSI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USR_MOSI`"]
pub struct USR_MOSI_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_MOSI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `USR_DUMMY_IDLE`"]
pub type USR_DUMMY_IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USR_DUMMY_IDLE`"]
pub struct USR_DUMMY_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_DUMMY_IDLE_W<'a> {
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
#[doc = "Reader of field `USR_MOSI_HIGHPART`"]
pub type USR_MOSI_HIGHPART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USR_MOSI_HIGHPART`"]
pub struct USR_MOSI_HIGHPART_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_MOSI_HIGHPART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `USR_MISO_HIGHPART`"]
pub type USR_MISO_HIGHPART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USR_MISO_HIGHPART`"]
pub struct USR_MISO_HIGHPART_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_MISO_HIGHPART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `SIO`"]
pub type SIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIO`"]
pub struct SIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `USR_CONF_NXT`"]
pub type USR_CONF_NXT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USR_CONF_NXT`"]
pub struct USR_CONF_NXT_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_CONF_NXT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `FWRITE_QUAD`"]
pub type FWRITE_QUAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FWRITE_QUAD`"]
pub struct FWRITE_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> FWRITE_QUAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `FWRITE_DUAL`"]
pub type FWRITE_DUAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FWRITE_DUAL`"]
pub struct FWRITE_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FWRITE_DUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `CK_OUT_EDGE`"]
pub type CK_OUT_EDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CK_OUT_EDGE`"]
pub struct CK_OUT_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> CK_OUT_EDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `RSCK_I_EDGE`"]
pub type RSCK_I_EDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSCK_I_EDGE`"]
pub struct RSCK_I_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSCK_I_EDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `CS_SETUP`"]
pub type CS_SETUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CS_SETUP`"]
pub struct CS_SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_SETUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `CS_HOLD`"]
pub type CS_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CS_HOLD`"]
pub struct CS_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `TSCK_I_EDGE`"]
pub type TSCK_I_EDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSCK_I_EDGE`"]
pub struct TSCK_I_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCK_I_EDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `QPI_MODE`"]
pub type QPI_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QPI_MODE`"]
pub struct QPI_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> QPI_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DOUTDIN`"]
pub type DOUTDIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUTDIN`"]
pub struct DOUTDIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUTDIN_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn usr_command(&self) -> USR_COMMAND_R {
        USR_COMMAND_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn usr_addr(&self) -> USR_ADDR_R {
        USR_ADDR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn usr_dummy(&self) -> USR_DUMMY_R {
        USR_DUMMY_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn usr_miso(&self) -> USR_MISO_R {
        USR_MISO_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn usr_mosi(&self) -> USR_MOSI_R {
        USR_MOSI_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn usr_dummy_idle(&self) -> USR_DUMMY_IDLE_R {
        USR_DUMMY_IDLE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn usr_mosi_highpart(&self) -> USR_MOSI_HIGHPART_R {
        USR_MOSI_HIGHPART_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn usr_miso_highpart(&self) -> USR_MISO_HIGHPART_R {
        USR_MISO_HIGHPART_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sio(&self) -> SIO_R {
        SIO_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn usr_conf_nxt(&self) -> USR_CONF_NXT_R {
        USR_CONF_NXT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn fwrite_quad(&self) -> FWRITE_QUAD_R {
        FWRITE_QUAD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn fwrite_dual(&self) -> FWRITE_DUAL_R {
        FWRITE_DUAL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ck_out_edge(&self) -> CK_OUT_EDGE_R {
        CK_OUT_EDGE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rsck_i_edge(&self) -> RSCK_I_EDGE_R {
        RSCK_I_EDGE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cs_setup(&self) -> CS_SETUP_R {
        CS_SETUP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cs_hold(&self) -> CS_HOLD_R {
        CS_HOLD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tsck_i_edge(&self) -> TSCK_I_EDGE_R {
        TSCK_I_EDGE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn qpi_mode(&self) -> QPI_MODE_R {
        QPI_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn doutdin(&self) -> DOUTDIN_R {
        DOUTDIN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn usr_command(&mut self) -> USR_COMMAND_W {
        USR_COMMAND_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn usr_addr(&mut self) -> USR_ADDR_W {
        USR_ADDR_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn usr_dummy(&mut self) -> USR_DUMMY_W {
        USR_DUMMY_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn usr_miso(&mut self) -> USR_MISO_W {
        USR_MISO_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn usr_mosi(&mut self) -> USR_MOSI_W {
        USR_MOSI_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn usr_dummy_idle(&mut self) -> USR_DUMMY_IDLE_W {
        USR_DUMMY_IDLE_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn usr_mosi_highpart(&mut self) -> USR_MOSI_HIGHPART_W {
        USR_MOSI_HIGHPART_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn usr_miso_highpart(&mut self) -> USR_MISO_HIGHPART_W {
        USR_MISO_HIGHPART_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sio(&mut self) -> SIO_W {
        SIO_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn usr_conf_nxt(&mut self) -> USR_CONF_NXT_W {
        USR_CONF_NXT_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn fwrite_quad(&mut self) -> FWRITE_QUAD_W {
        FWRITE_QUAD_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn fwrite_dual(&mut self) -> FWRITE_DUAL_W {
        FWRITE_DUAL_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ck_out_edge(&mut self) -> CK_OUT_EDGE_W {
        CK_OUT_EDGE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rsck_i_edge(&mut self) -> RSCK_I_EDGE_W {
        RSCK_I_EDGE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cs_setup(&mut self) -> CS_SETUP_W {
        CS_SETUP_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cs_hold(&mut self) -> CS_HOLD_W {
        CS_HOLD_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tsck_i_edge(&mut self) -> TSCK_I_EDGE_W {
        TSCK_I_EDGE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn qpi_mode(&mut self) -> QPI_MODE_W {
        QPI_MODE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn doutdin(&mut self) -> DOUTDIN_W {
        DOUTDIN_W { w: self }
    }
}
