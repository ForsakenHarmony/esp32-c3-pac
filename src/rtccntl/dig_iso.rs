#[doc = "Reader of register DIG_ISO"]
pub type R = crate::R<u32, super::DIG_ISO>;
#[doc = "Writer for register DIG_ISO"]
pub type W = crate::W<u32, super::DIG_ISO>;
#[doc = "Register DIG_ISO `reset()`'s with value 0"]
impl crate::ResetValue for super::DIG_ISO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DG_WRAP_FORCE_NOISO`"]
pub type DG_WRAP_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DG_WRAP_FORCE_NOISO`"]
pub struct DG_WRAP_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `DG_WRAP_FORCE_ISO`"]
pub type DG_WRAP_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DG_WRAP_FORCE_ISO`"]
pub struct DG_WRAP_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `WIFI_FORCE_NOISO`"]
pub type WIFI_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WIFI_FORCE_NOISO`"]
pub struct WIFI_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `WIFI_FORCE_ISO`"]
pub type WIFI_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WIFI_FORCE_ISO`"]
pub struct WIFI_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `CPU_TOP_FORCE_NOISO`"]
pub type CPU_TOP_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPU_TOP_FORCE_NOISO`"]
pub struct CPU_TOP_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_TOP_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `CPU_TOP_FORCE_ISO`"]
pub type CPU_TOP_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPU_TOP_FORCE_ISO`"]
pub struct CPU_TOP_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_TOP_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `DG_PERI_FORCE_NOISO`"]
pub type DG_PERI_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DG_PERI_FORCE_NOISO`"]
pub struct DG_PERI_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PERI_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `DG_PERI_FORCE_ISO`"]
pub type DG_PERI_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DG_PERI_FORCE_ISO`"]
pub struct DG_PERI_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PERI_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `BT_FORCE_NOISO`"]
pub type BT_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BT_FORCE_NOISO`"]
pub struct BT_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> BT_FORCE_NOISO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `BT_FORCE_ISO`"]
pub type BT_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BT_FORCE_ISO`"]
pub struct BT_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> BT_FORCE_ISO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `DG_PAD_FORCE_HOLD`"]
pub type DG_PAD_FORCE_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DG_PAD_FORCE_HOLD`"]
pub struct DG_PAD_FORCE_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PAD_FORCE_HOLD_W<'a> {
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
#[doc = "Reader of field `DG_PAD_FORCE_UNHOLD`"]
pub type DG_PAD_FORCE_UNHOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DG_PAD_FORCE_UNHOLD`"]
pub struct DG_PAD_FORCE_UNHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PAD_FORCE_UNHOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `DG_PAD_FORCE_ISO`"]
pub type DG_PAD_FORCE_ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DG_PAD_FORCE_ISO`"]
pub struct DG_PAD_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PAD_FORCE_ISO_W<'a> {
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
#[doc = "Reader of field `DG_PAD_FORCE_NOISO`"]
pub type DG_PAD_FORCE_NOISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DG_PAD_FORCE_NOISO`"]
pub struct DG_PAD_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PAD_FORCE_NOISO_W<'a> {
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
#[doc = "Reader of field `DG_PAD_AUTOHOLD_EN`"]
pub type DG_PAD_AUTOHOLD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DG_PAD_AUTOHOLD_EN`"]
pub struct DG_PAD_AUTOHOLD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PAD_AUTOHOLD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Write proxy for field `CLR_DG_PAD_AUTOHOLD`"]
pub struct CLR_DG_PAD_AUTOHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_DG_PAD_AUTOHOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `DG_PAD_AUTOHOLD`"]
pub type DG_PAD_AUTOHOLD_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIG_ISO_FORCE_ON`"]
pub type DIG_ISO_FORCE_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIG_ISO_FORCE_ON`"]
pub struct DIG_ISO_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_ISO_FORCE_ON_W<'a> {
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
#[doc = "Reader of field `DIG_ISO_FORCE_OFF`"]
pub type DIG_ISO_FORCE_OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIG_ISO_FORCE_OFF`"]
pub struct DIG_ISO_FORCE_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_ISO_FORCE_OFF_W<'a> {
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
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dg_wrap_force_noiso(&self) -> DG_WRAP_FORCE_NOISO_R {
        DG_WRAP_FORCE_NOISO_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dg_wrap_force_iso(&self) -> DG_WRAP_FORCE_ISO_R {
        DG_WRAP_FORCE_ISO_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn wifi_force_noiso(&self) -> WIFI_FORCE_NOISO_R {
        WIFI_FORCE_NOISO_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn wifi_force_iso(&self) -> WIFI_FORCE_ISO_R {
        WIFI_FORCE_ISO_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cpu_top_force_noiso(&self) -> CPU_TOP_FORCE_NOISO_R {
        CPU_TOP_FORCE_NOISO_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cpu_top_force_iso(&self) -> CPU_TOP_FORCE_ISO_R {
        CPU_TOP_FORCE_ISO_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dg_peri_force_noiso(&self) -> DG_PERI_FORCE_NOISO_R {
        DG_PERI_FORCE_NOISO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dg_peri_force_iso(&self) -> DG_PERI_FORCE_ISO_R {
        DG_PERI_FORCE_ISO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn bt_force_noiso(&self) -> BT_FORCE_NOISO_R {
        BT_FORCE_NOISO_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn bt_force_iso(&self) -> BT_FORCE_ISO_R {
        BT_FORCE_ISO_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dg_pad_force_hold(&self) -> DG_PAD_FORCE_HOLD_R {
        DG_PAD_FORCE_HOLD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dg_pad_force_unhold(&self) -> DG_PAD_FORCE_UNHOLD_R {
        DG_PAD_FORCE_UNHOLD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dg_pad_force_iso(&self) -> DG_PAD_FORCE_ISO_R {
        DG_PAD_FORCE_ISO_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dg_pad_force_noiso(&self) -> DG_PAD_FORCE_NOISO_R {
        DG_PAD_FORCE_NOISO_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dg_pad_autohold_en(&self) -> DG_PAD_AUTOHOLD_EN_R {
        DG_PAD_AUTOHOLD_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dg_pad_autohold(&self) -> DG_PAD_AUTOHOLD_R {
        DG_PAD_AUTOHOLD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dig_iso_force_on(&self) -> DIG_ISO_FORCE_ON_R {
        DIG_ISO_FORCE_ON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dig_iso_force_off(&self) -> DIG_ISO_FORCE_OFF_R {
        DIG_ISO_FORCE_OFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dg_wrap_force_noiso(&mut self) -> DG_WRAP_FORCE_NOISO_W {
        DG_WRAP_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dg_wrap_force_iso(&mut self) -> DG_WRAP_FORCE_ISO_W {
        DG_WRAP_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn wifi_force_noiso(&mut self) -> WIFI_FORCE_NOISO_W {
        WIFI_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn wifi_force_iso(&mut self) -> WIFI_FORCE_ISO_W {
        WIFI_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cpu_top_force_noiso(&mut self) -> CPU_TOP_FORCE_NOISO_W {
        CPU_TOP_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cpu_top_force_iso(&mut self) -> CPU_TOP_FORCE_ISO_W {
        CPU_TOP_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dg_peri_force_noiso(&mut self) -> DG_PERI_FORCE_NOISO_W {
        DG_PERI_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dg_peri_force_iso(&mut self) -> DG_PERI_FORCE_ISO_W {
        DG_PERI_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn bt_force_noiso(&mut self) -> BT_FORCE_NOISO_W {
        BT_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn bt_force_iso(&mut self) -> BT_FORCE_ISO_W {
        BT_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dg_pad_force_hold(&mut self) -> DG_PAD_FORCE_HOLD_W {
        DG_PAD_FORCE_HOLD_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dg_pad_force_unhold(&mut self) -> DG_PAD_FORCE_UNHOLD_W {
        DG_PAD_FORCE_UNHOLD_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dg_pad_force_iso(&mut self) -> DG_PAD_FORCE_ISO_W {
        DG_PAD_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dg_pad_force_noiso(&mut self) -> DG_PAD_FORCE_NOISO_W {
        DG_PAD_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dg_pad_autohold_en(&mut self) -> DG_PAD_AUTOHOLD_EN_W {
        DG_PAD_AUTOHOLD_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clr_dg_pad_autohold(&mut self) -> CLR_DG_PAD_AUTOHOLD_W {
        CLR_DG_PAD_AUTOHOLD_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dig_iso_force_on(&mut self) -> DIG_ISO_FORCE_ON_W {
        DIG_ISO_FORCE_ON_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dig_iso_force_off(&mut self) -> DIG_ISO_FORCE_OFF_W {
        DIG_ISO_FORCE_OFF_W { w: self }
    }
}
