#[doc = "Reader of register RD_REPEAT_DATA1"]
pub type R = crate::R<u32, super::RD_REPEAT_DATA1>;
#[doc = "Reader of field `KEY_PURPOSE_1`"]
pub type KEY_PURPOSE_1_R = crate::R<u8, u8>;
#[doc = "Reader of field `KEY_PURPOSE_0`"]
pub type KEY_PURPOSE_0_R = crate::R<u8, u8>;
#[doc = "Reader of field `SECURE_BOOT_KEY_REVOKE2`"]
pub type SECURE_BOOT_KEY_REVOKE2_R = crate::R<bool, bool>;
#[doc = "Reader of field `SECURE_BOOT_KEY_REVOKE1`"]
pub type SECURE_BOOT_KEY_REVOKE1_R = crate::R<bool, bool>;
#[doc = "Reader of field `SECURE_BOOT_KEY_REVOKE0`"]
pub type SECURE_BOOT_KEY_REVOKE0_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_BOOT_CRYPT_CNT`"]
pub type SPI_BOOT_CRYPT_CNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `WDT_DELAY_SEL`"]
pub type WDT_DELAY_SEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `RPT4_RESERVED2`"]
pub type RPT4_RESERVED2_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn key_purpose_1(&self) -> KEY_PURPOSE_1_R {
        KEY_PURPOSE_1_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn key_purpose_0(&self) -> KEY_PURPOSE_0_R {
        KEY_PURPOSE_0_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn secure_boot_key_revoke2(&self) -> SECURE_BOOT_KEY_REVOKE2_R {
        SECURE_BOOT_KEY_REVOKE2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn secure_boot_key_revoke1(&self) -> SECURE_BOOT_KEY_REVOKE1_R {
        SECURE_BOOT_KEY_REVOKE1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn secure_boot_key_revoke0(&self) -> SECURE_BOOT_KEY_REVOKE0_R {
        SECURE_BOOT_KEY_REVOKE0_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn spi_boot_crypt_cnt(&self) -> SPI_BOOT_CRYPT_CNT_R {
        SPI_BOOT_CRYPT_CNT_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn wdt_delay_sel(&self) -> WDT_DELAY_SEL_R {
        WDT_DELAY_SEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rpt4_reserved2(&self) -> RPT4_RESERVED2_R {
        RPT4_RESERVED2_R::new((self.bits & 0xffff) as u16)
    }
}
