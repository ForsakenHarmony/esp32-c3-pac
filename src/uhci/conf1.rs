#[doc = "Reader of register CONF1"]
pub type R = crate::R<u32, super::CONF1>;
#[doc = "Writer for register CONF1"]
pub type W = crate::W<u32, super::CONF1>;
#[doc = "Register CONF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SW_START`"]
pub type SW_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_START`"]
pub struct SW_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_START_W<'a> {
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
#[doc = "Reader of field `WAIT_SW_START`"]
pub type WAIT_SW_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAIT_SW_START`"]
pub struct WAIT_SW_START_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_SW_START_W<'a> {
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
#[doc = "Reader of field `TX_ACK_NUM_RE`"]
pub type TX_ACK_NUM_RE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_ACK_NUM_RE`"]
pub struct TX_ACK_NUM_RE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ACK_NUM_RE_W<'a> {
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
#[doc = "Reader of field `TX_CHECK_SUM_RE`"]
pub type TX_CHECK_SUM_RE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_CHECK_SUM_RE`"]
pub struct TX_CHECK_SUM_RE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CHECK_SUM_RE_W<'a> {
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
#[doc = "Reader of field `SAVE_HEAD`"]
pub type SAVE_HEAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAVE_HEAD`"]
pub struct SAVE_HEAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SAVE_HEAD_W<'a> {
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
#[doc = "Reader of field `CRC_DISABLE`"]
pub type CRC_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRC_DISABLE`"]
pub struct CRC_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_DISABLE_W<'a> {
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
#[doc = "Reader of field `CHECK_SEQ_EN`"]
pub type CHECK_SEQ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHECK_SEQ_EN`"]
pub struct CHECK_SEQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHECK_SEQ_EN_W<'a> {
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
#[doc = "Reader of field `CHECK_SUM_EN`"]
pub type CHECK_SUM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHECK_SUM_EN`"]
pub struct CHECK_SUM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHECK_SUM_EN_W<'a> {
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
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sw_start(&self) -> SW_START_R {
        SW_START_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn wait_sw_start(&self) -> WAIT_SW_START_R {
        WAIT_SW_START_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_ack_num_re(&self) -> TX_ACK_NUM_RE_R {
        TX_ACK_NUM_RE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_check_sum_re(&self) -> TX_CHECK_SUM_RE_R {
        TX_CHECK_SUM_RE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn save_head(&self) -> SAVE_HEAD_R {
        SAVE_HEAD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn crc_disable(&self) -> CRC_DISABLE_R {
        CRC_DISABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn check_seq_en(&self) -> CHECK_SEQ_EN_R {
        CHECK_SEQ_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn check_sum_en(&self) -> CHECK_SUM_EN_R {
        CHECK_SUM_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sw_start(&mut self) -> SW_START_W {
        SW_START_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn wait_sw_start(&mut self) -> WAIT_SW_START_W {
        WAIT_SW_START_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_ack_num_re(&mut self) -> TX_ACK_NUM_RE_W {
        TX_ACK_NUM_RE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_check_sum_re(&mut self) -> TX_CHECK_SUM_RE_W {
        TX_CHECK_SUM_RE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn save_head(&mut self) -> SAVE_HEAD_W {
        SAVE_HEAD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn crc_disable(&mut self) -> CRC_DISABLE_W {
        CRC_DISABLE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn check_seq_en(&mut self) -> CHECK_SEQ_EN_W {
        CHECK_SEQ_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn check_sum_en(&mut self) -> CHECK_SUM_EN_W {
        CHECK_SUM_EN_W { w: self }
    }
}
