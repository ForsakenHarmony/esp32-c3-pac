#[doc = "Reader of register FSM_STATUS"]
pub type R = crate::R<u32, super::FSM_STATUS>;
#[doc = "Reader of field `ST_UTX_OUT`"]
pub type ST_UTX_OUT_R = crate::R<u8, u8>;
#[doc = "Reader of field `ST_URX_OUT`"]
pub type ST_URX_OUT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn st_utx_out(&self) -> ST_UTX_OUT_R {
        ST_UTX_OUT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn st_urx_out(&self) -> ST_URX_OUT_R {
        ST_URX_OUT_R::new((self.bits & 0x0f) as u8)
    }
}
