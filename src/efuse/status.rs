#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `REPEAT_ERR_CNT`"]
pub type REPEAT_ERR_CNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `OTP_VDDQ_IS_SW`"]
pub type OTP_VDDQ_IS_SW_R = crate::R<bool, bool>;
#[doc = "Reader of field `OTP_PGENB_SW`"]
pub type OTP_PGENB_SW_R = crate::R<bool, bool>;
#[doc = "Reader of field `OTP_CSB_SW`"]
pub type OTP_CSB_SW_R = crate::R<bool, bool>;
#[doc = "Reader of field `OTP_STROBE_SW`"]
pub type OTP_STROBE_SW_R = crate::R<bool, bool>;
#[doc = "Reader of field `OTP_VDDQ_C_SYNC2`"]
pub type OTP_VDDQ_C_SYNC2_R = crate::R<bool, bool>;
#[doc = "Reader of field `OTP_LOAD_SW`"]
pub type OTP_LOAD_SW_R = crate::R<bool, bool>;
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 10:17"]
    #[inline(always)]
    pub fn repeat_err_cnt(&self) -> REPEAT_ERR_CNT_R {
        REPEAT_ERR_CNT_R::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn otp_vddq_is_sw(&self) -> OTP_VDDQ_IS_SW_R {
        OTP_VDDQ_IS_SW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn otp_pgenb_sw(&self) -> OTP_PGENB_SW_R {
        OTP_PGENB_SW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn otp_csb_sw(&self) -> OTP_CSB_SW_R {
        OTP_CSB_SW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn otp_strobe_sw(&self) -> OTP_STROBE_SW_R {
        OTP_STROBE_SW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn otp_vddq_c_sync2(&self) -> OTP_VDDQ_C_SYNC2_R {
        OTP_VDDQ_C_SYNC2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn otp_load_sw(&self) -> OTP_LOAD_SW_R {
        OTP_LOAD_SW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 0x0f) as u8)
    }
}
