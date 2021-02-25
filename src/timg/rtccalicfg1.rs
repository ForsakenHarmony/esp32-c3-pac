#[doc = "Reader of register RTCCALICFG1"]
pub type R = crate::R<u32, super::RTCCALICFG1>;
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<u32, u32>;
#[doc = "Reader of field `CYCLING_DATA_VLD`"]
pub type CYCLING_DATA_VLD_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 7:31"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cycling_data_vld(&self) -> CYCLING_DATA_VLD_R {
        CYCLING_DATA_VLD_R::new((self.bits & 0x01) != 0)
    }
}
