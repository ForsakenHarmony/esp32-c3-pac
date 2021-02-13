#[doc = "Reader of register LEDC_INT_ENA"]
pub type R = crate::R<u32, super::LEDC_INT_ENA>;
#[doc = "Writer for register LEDC_INT_ENA"]
pub type W = crate::W<u32, super::LEDC_INT_ENA>;
#[doc = "Register LEDC_INT_ENA `reset()`'s with value 0"]
impl crate::ResetValue for super::LEDC_INT_ENA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEDC_OVF_CNT_LSCH5_INT_ENA`"]
pub type LEDC_OVF_CNT_LSCH5_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_OVF_CNT_LSCH5_INT_ENA`"]
pub struct LEDC_OVF_CNT_LSCH5_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_OVF_CNT_LSCH5_INT_ENA_W<'a> {
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
#[doc = "Reader of field `LEDC_OVF_CNT_LSCH4_INT_ENA`"]
pub type LEDC_OVF_CNT_LSCH4_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_OVF_CNT_LSCH4_INT_ENA`"]
pub struct LEDC_OVF_CNT_LSCH4_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_OVF_CNT_LSCH4_INT_ENA_W<'a> {
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
#[doc = "Reader of field `LEDC_OVF_CNT_LSCH3_INT_ENA`"]
pub type LEDC_OVF_CNT_LSCH3_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_OVF_CNT_LSCH3_INT_ENA`"]
pub struct LEDC_OVF_CNT_LSCH3_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_OVF_CNT_LSCH3_INT_ENA_W<'a> {
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
#[doc = "Reader of field `LEDC_OVF_CNT_LSCH2_INT_ENA`"]
pub type LEDC_OVF_CNT_LSCH2_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_OVF_CNT_LSCH2_INT_ENA`"]
pub struct LEDC_OVF_CNT_LSCH2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_OVF_CNT_LSCH2_INT_ENA_W<'a> {
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
#[doc = "Reader of field `LEDC_OVF_CNT_LSCH1_INT_ENA`"]
pub type LEDC_OVF_CNT_LSCH1_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_OVF_CNT_LSCH1_INT_ENA`"]
pub struct LEDC_OVF_CNT_LSCH1_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_OVF_CNT_LSCH1_INT_ENA_W<'a> {
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
#[doc = "Reader of field `LEDC_OVF_CNT_LSCH0_INT_ENA`"]
pub type LEDC_OVF_CNT_LSCH0_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_OVF_CNT_LSCH0_INT_ENA`"]
pub struct LEDC_OVF_CNT_LSCH0_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_OVF_CNT_LSCH0_INT_ENA_W<'a> {
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
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_LSCH5_INT_ENA`"]
pub type LEDC_DUTY_CHNG_END_LSCH5_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_LSCH5_INT_ENA`"]
pub struct LEDC_DUTY_CHNG_END_LSCH5_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_LSCH5_INT_ENA_W<'a> {
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
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_LSCH4_INT_ENA`"]
pub type LEDC_DUTY_CHNG_END_LSCH4_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_LSCH4_INT_ENA`"]
pub struct LEDC_DUTY_CHNG_END_LSCH4_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_LSCH4_INT_ENA_W<'a> {
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
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_LSCH3_INT_ENA`"]
pub type LEDC_DUTY_CHNG_END_LSCH3_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_LSCH3_INT_ENA`"]
pub struct LEDC_DUTY_CHNG_END_LSCH3_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_LSCH3_INT_ENA_W<'a> {
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
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_LSCH2_INT_ENA`"]
pub type LEDC_DUTY_CHNG_END_LSCH2_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_LSCH2_INT_ENA`"]
pub struct LEDC_DUTY_CHNG_END_LSCH2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_LSCH2_INT_ENA_W<'a> {
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
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_LSCH1_INT_ENA`"]
pub type LEDC_DUTY_CHNG_END_LSCH1_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_LSCH1_INT_ENA`"]
pub struct LEDC_DUTY_CHNG_END_LSCH1_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_LSCH1_INT_ENA_W<'a> {
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
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_LSCH0_INT_ENA`"]
pub type LEDC_DUTY_CHNG_END_LSCH0_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_LSCH0_INT_ENA`"]
pub struct LEDC_DUTY_CHNG_END_LSCH0_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_LSCH0_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `LEDC_LSTIMER3_OVF_INT_ENA`"]
pub type LEDC_LSTIMER3_OVF_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_LSTIMER3_OVF_INT_ENA`"]
pub struct LEDC_LSTIMER3_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_LSTIMER3_OVF_INT_ENA_W<'a> {
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
#[doc = "Reader of field `LEDC_LSTIMER2_OVF_INT_ENA`"]
pub type LEDC_LSTIMER2_OVF_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_LSTIMER2_OVF_INT_ENA`"]
pub struct LEDC_LSTIMER2_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_LSTIMER2_OVF_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `LEDC_LSTIMER1_OVF_INT_ENA`"]
pub type LEDC_LSTIMER1_OVF_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_LSTIMER1_OVF_INT_ENA`"]
pub struct LEDC_LSTIMER1_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_LSTIMER1_OVF_INT_ENA_W<'a> {
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
#[doc = "Reader of field `LEDC_LSTIMER0_OVF_INT_ENA`"]
pub type LEDC_LSTIMER0_OVF_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_LSTIMER0_OVF_INT_ENA`"]
pub struct LEDC_LSTIMER0_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_LSTIMER0_OVF_INT_ENA_W<'a> {
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
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_lsch5_int_ena(&self) -> LEDC_OVF_CNT_LSCH5_INT_ENA_R {
        LEDC_OVF_CNT_LSCH5_INT_ENA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_lsch4_int_ena(&self) -> LEDC_OVF_CNT_LSCH4_INT_ENA_R {
        LEDC_OVF_CNT_LSCH4_INT_ENA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_lsch3_int_ena(&self) -> LEDC_OVF_CNT_LSCH3_INT_ENA_R {
        LEDC_OVF_CNT_LSCH3_INT_ENA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_lsch2_int_ena(&self) -> LEDC_OVF_CNT_LSCH2_INT_ENA_R {
        LEDC_OVF_CNT_LSCH2_INT_ENA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_lsch1_int_ena(&self) -> LEDC_OVF_CNT_LSCH1_INT_ENA_R {
        LEDC_OVF_CNT_LSCH1_INT_ENA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_lsch0_int_ena(&self) -> LEDC_OVF_CNT_LSCH0_INT_ENA_R {
        LEDC_OVF_CNT_LSCH0_INT_ENA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch5_int_ena(&self) -> LEDC_DUTY_CHNG_END_LSCH5_INT_ENA_R {
        LEDC_DUTY_CHNG_END_LSCH5_INT_ENA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch4_int_ena(&self) -> LEDC_DUTY_CHNG_END_LSCH4_INT_ENA_R {
        LEDC_DUTY_CHNG_END_LSCH4_INT_ENA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch3_int_ena(&self) -> LEDC_DUTY_CHNG_END_LSCH3_INT_ENA_R {
        LEDC_DUTY_CHNG_END_LSCH3_INT_ENA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch2_int_ena(&self) -> LEDC_DUTY_CHNG_END_LSCH2_INT_ENA_R {
        LEDC_DUTY_CHNG_END_LSCH2_INT_ENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch1_int_ena(&self) -> LEDC_DUTY_CHNG_END_LSCH1_INT_ENA_R {
        LEDC_DUTY_CHNG_END_LSCH1_INT_ENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch0_int_ena(&self) -> LEDC_DUTY_CHNG_END_LSCH0_INT_ENA_R {
        LEDC_DUTY_CHNG_END_LSCH0_INT_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ledc_lstimer3_ovf_int_ena(&self) -> LEDC_LSTIMER3_OVF_INT_ENA_R {
        LEDC_LSTIMER3_OVF_INT_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ledc_lstimer2_ovf_int_ena(&self) -> LEDC_LSTIMER2_OVF_INT_ENA_R {
        LEDC_LSTIMER2_OVF_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ledc_lstimer1_ovf_int_ena(&self) -> LEDC_LSTIMER1_OVF_INT_ENA_R {
        LEDC_LSTIMER1_OVF_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ledc_lstimer0_ovf_int_ena(&self) -> LEDC_LSTIMER0_OVF_INT_ENA_R {
        LEDC_LSTIMER0_OVF_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_lsch5_int_ena(&mut self) -> LEDC_OVF_CNT_LSCH5_INT_ENA_W {
        LEDC_OVF_CNT_LSCH5_INT_ENA_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_lsch4_int_ena(&mut self) -> LEDC_OVF_CNT_LSCH4_INT_ENA_W {
        LEDC_OVF_CNT_LSCH4_INT_ENA_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_lsch3_int_ena(&mut self) -> LEDC_OVF_CNT_LSCH3_INT_ENA_W {
        LEDC_OVF_CNT_LSCH3_INT_ENA_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_lsch2_int_ena(&mut self) -> LEDC_OVF_CNT_LSCH2_INT_ENA_W {
        LEDC_OVF_CNT_LSCH2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_lsch1_int_ena(&mut self) -> LEDC_OVF_CNT_LSCH1_INT_ENA_W {
        LEDC_OVF_CNT_LSCH1_INT_ENA_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_lsch0_int_ena(&mut self) -> LEDC_OVF_CNT_LSCH0_INT_ENA_W {
        LEDC_OVF_CNT_LSCH0_INT_ENA_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch5_int_ena(&mut self) -> LEDC_DUTY_CHNG_END_LSCH5_INT_ENA_W {
        LEDC_DUTY_CHNG_END_LSCH5_INT_ENA_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch4_int_ena(&mut self) -> LEDC_DUTY_CHNG_END_LSCH4_INT_ENA_W {
        LEDC_DUTY_CHNG_END_LSCH4_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch3_int_ena(&mut self) -> LEDC_DUTY_CHNG_END_LSCH3_INT_ENA_W {
        LEDC_DUTY_CHNG_END_LSCH3_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch2_int_ena(&mut self) -> LEDC_DUTY_CHNG_END_LSCH2_INT_ENA_W {
        LEDC_DUTY_CHNG_END_LSCH2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch1_int_ena(&mut self) -> LEDC_DUTY_CHNG_END_LSCH1_INT_ENA_W {
        LEDC_DUTY_CHNG_END_LSCH1_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch0_int_ena(&mut self) -> LEDC_DUTY_CHNG_END_LSCH0_INT_ENA_W {
        LEDC_DUTY_CHNG_END_LSCH0_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ledc_lstimer3_ovf_int_ena(&mut self) -> LEDC_LSTIMER3_OVF_INT_ENA_W {
        LEDC_LSTIMER3_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ledc_lstimer2_ovf_int_ena(&mut self) -> LEDC_LSTIMER2_OVF_INT_ENA_W {
        LEDC_LSTIMER2_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ledc_lstimer1_ovf_int_ena(&mut self) -> LEDC_LSTIMER1_OVF_INT_ENA_W {
        LEDC_LSTIMER1_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ledc_lstimer0_ovf_int_ena(&mut self) -> LEDC_LSTIMER0_OVF_INT_ENA_W {
        LEDC_LSTIMER0_OVF_INT_ENA_W { w: self }
    }
}
