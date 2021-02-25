#[doc = "Reader of register STATE"]
pub type R = crate::R<u32, super::STATE>;
#[doc = "Reader of field `TX_IDLE`"]
pub type TX_IDLE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_idle(&self) -> TX_IDLE_R {
        TX_IDLE_R::new((self.bits & 0x01) != 0)
    }
}
