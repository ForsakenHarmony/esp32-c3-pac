#[doc = "Reader of register INT_ST"]
pub type R = crate::R<u32, super::INT_ST>;
#[doc = "Reader of field `PGM_DONE_INT_ST`"]
pub type PGM_DONE_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `READ_DONE_INT_ST`"]
pub type READ_DONE_INT_ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pgm_done_int_st(&self) -> PGM_DONE_INT_ST_R {
        PGM_DONE_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn read_done_int_st(&self) -> READ_DONE_INT_ST_R {
        READ_DONE_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
