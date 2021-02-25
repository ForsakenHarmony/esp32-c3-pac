#[doc = "Reader of register RD_RS_ERR0"]
pub type R = crate::R<u32, super::RD_RS_ERR0>;
#[doc = "Reader of field `KEY4_FAIL`"]
pub type KEY4_FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `KEY4_ERR_NUM`"]
pub type KEY4_ERR_NUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `KEY3_FAIL`"]
pub type KEY3_FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `KEY3_ERR_NUM`"]
pub type KEY3_ERR_NUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `KEY2_FAIL`"]
pub type KEY2_FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `KEY2_ERR_NUM`"]
pub type KEY2_ERR_NUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `KEY1_FAIL`"]
pub type KEY1_FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `KEY1_ERR_NUM`"]
pub type KEY1_ERR_NUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `KEY0_FAIL`"]
pub type KEY0_FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `KEY0_ERR_NUM`"]
pub type KEY0_ERR_NUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `USR_DATA_FAIL`"]
pub type USR_DATA_FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `USR_DATA_ERR_NUM`"]
pub type USR_DATA_ERR_NUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `SYS_PART1_FAIL`"]
pub type SYS_PART1_FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYS_PART1_NUM`"]
pub type SYS_PART1_NUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAC_SPI_8M_FAIL`"]
pub type MAC_SPI_8M_FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `MAC_SPI_8M_ERR_NUM`"]
pub type MAC_SPI_8M_ERR_NUM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn key4_fail(&self) -> KEY4_FAIL_R {
        KEY4_FAIL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn key4_err_num(&self) -> KEY4_ERR_NUM_R {
        KEY4_ERR_NUM_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn key3_fail(&self) -> KEY3_FAIL_R {
        KEY3_FAIL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn key3_err_num(&self) -> KEY3_ERR_NUM_R {
        KEY3_ERR_NUM_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn key2_fail(&self) -> KEY2_FAIL_R {
        KEY2_FAIL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn key2_err_num(&self) -> KEY2_ERR_NUM_R {
        KEY2_ERR_NUM_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn key1_fail(&self) -> KEY1_FAIL_R {
        KEY1_FAIL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn key1_err_num(&self) -> KEY1_ERR_NUM_R {
        KEY1_ERR_NUM_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn key0_fail(&self) -> KEY0_FAIL_R {
        KEY0_FAIL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn key0_err_num(&self) -> KEY0_ERR_NUM_R {
        KEY0_ERR_NUM_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn usr_data_fail(&self) -> USR_DATA_FAIL_R {
        USR_DATA_FAIL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn usr_data_err_num(&self) -> USR_DATA_ERR_NUM_R {
        USR_DATA_ERR_NUM_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sys_part1_fail(&self) -> SYS_PART1_FAIL_R {
        SYS_PART1_FAIL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn sys_part1_num(&self) -> SYS_PART1_NUM_R {
        SYS_PART1_NUM_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn mac_spi_8m_fail(&self) -> MAC_SPI_8M_FAIL_R {
        MAC_SPI_8M_FAIL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn mac_spi_8m_err_num(&self) -> MAC_SPI_8M_ERR_NUM_R {
        MAC_SPI_8M_ERR_NUM_R::new((self.bits & 0x07) as u8)
    }
}
