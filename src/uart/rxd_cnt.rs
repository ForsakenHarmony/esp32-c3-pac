#[doc = "Reader of register RXD_CNT"]
pub type R = crate::R<u32, super::RXD_CNT>;
#[doc = "Reader of field `RXD_EDGE_CNT`"]
pub type RXD_EDGE_CNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rxd_edge_cnt(&self) -> RXD_EDGE_CNT_R {
        RXD_EDGE_CNT_R::new((self.bits & 0x03ff) as u16)
    }
}
