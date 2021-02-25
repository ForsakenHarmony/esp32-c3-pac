#[doc = "Reader of register RD_RS_ERR1"]
pub type R = crate::R<u32, super::RD_RS_ERR1>;
#[doc = "Reader of field `SYS_PART2_FAIL`"]
pub type SYS_PART2_FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYS_PART2_ERR_NUM`"]
pub type SYS_PART2_ERR_NUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `KEY5_FAIL`"]
pub type KEY5_FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `KEY5_ERR_NUM`"]
pub type KEY5_ERR_NUM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sys_part2_fail(&self) -> SYS_PART2_FAIL_R {
        SYS_PART2_FAIL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn sys_part2_err_num(&self) -> SYS_PART2_ERR_NUM_R {
        SYS_PART2_ERR_NUM_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn key5_fail(&self) -> KEY5_FAIL_R {
        KEY5_FAIL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn key5_err_num(&self) -> KEY5_ERR_NUM_R {
        KEY5_ERR_NUM_R::new((self.bits & 0x07) as u8)
    }
}
