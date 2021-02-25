#[doc = "Reader of register RD_MAC_SPI_SYS_0"]
pub type R = crate::R<u32, super::RD_MAC_SPI_SYS_0>;
#[doc = "Reader of field `MAC_0`"]
pub type MAC_0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mac_0(&self) -> MAC_0_R {
        MAC_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
