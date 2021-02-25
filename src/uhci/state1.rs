#[doc = "Reader of register STATE1"]
pub type R = crate::R<u32, super::STATE1>;
#[doc = "Reader of field `ENCODE_STATE`"]
pub type ENCODE_STATE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn encode_state(&self) -> ENCODE_STATE_R {
        ENCODE_STATE_R::new((self.bits & 0x07) as u8)
    }
}
