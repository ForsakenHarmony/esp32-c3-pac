#[doc = "Reader of register RD_SYS_PART1_DATA1"]
pub type R = crate::R<u32, super::RD_SYS_PART1_DATA1>;
#[doc = "Reader of field `SYS_DATA_PART1_1`"]
pub type SYS_DATA_PART1_1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sys_data_part1_1(&self) -> SYS_DATA_PART1_1_R {
        SYS_DATA_PART1_1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
