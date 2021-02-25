#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EFUSE_PGM_DATA0"]
    pub pgm_data0: PGM_DATA0,
    #[doc = "0x04 - EFUSE_PGM_DATA1"]
    pub pgm_data1: PGM_DATA1,
    #[doc = "0x08 - EFUSE_PGM_DATA2"]
    pub pgm_data2: PGM_DATA2,
    #[doc = "0x0c - EFUSE_PGM_DATA3"]
    pub pgm_data3: PGM_DATA3,
    #[doc = "0x10 - EFUSE_PGM_DATA4"]
    pub pgm_data4: PGM_DATA4,
    #[doc = "0x14 - EFUSE_PGM_DATA5"]
    pub pgm_data5: PGM_DATA5,
    #[doc = "0x18 - EFUSE_PGM_DATA6"]
    pub pgm_data6: PGM_DATA6,
    #[doc = "0x1c - EFUSE_PGM_DATA7"]
    pub pgm_data7: PGM_DATA7,
    #[doc = "0x20 - EFUSE_PGM_CHECK_VALUE0"]
    pub pgm_check_value0: PGM_CHECK_VALUE0,
    #[doc = "0x24 - EFUSE_PGM_CHECK_VALUE1"]
    pub pgm_check_value1: PGM_CHECK_VALUE1,
    #[doc = "0x28 - EFUSE_PGM_CHECK_VALUE2"]
    pub pgm_check_value2: PGM_CHECK_VALUE2,
    #[doc = "0x2c - EFUSE_RD_WR_DIS"]
    pub rd_wr_dis: RD_WR_DIS,
    #[doc = "0x30 - EFUSE_RD_REPEAT_DATA0"]
    pub rd_repeat_data0: RD_REPEAT_DATA0,
    #[doc = "0x34 - EFUSE_RD_REPEAT_DATA1"]
    pub rd_repeat_data1: RD_REPEAT_DATA1,
    #[doc = "0x38 - EFUSE_RD_REPEAT_DATA2"]
    pub rd_repeat_data2: RD_REPEAT_DATA2,
    #[doc = "0x3c - EFUSE_RD_REPEAT_DATA3"]
    pub rd_repeat_data3: RD_REPEAT_DATA3,
    #[doc = "0x40 - EFUSE_RD_REPEAT_DATA4"]
    pub rd_repeat_data4: RD_REPEAT_DATA4,
    #[doc = "0x44 - EFUSE_RD_MAC_SPI_SYS_0"]
    pub rd_mac_spi_sys_0: RD_MAC_SPI_SYS_0,
    #[doc = "0x48 - EFUSE_RD_MAC_SPI_SYS_1"]
    pub rd_mac_spi_sys_1: RD_MAC_SPI_SYS_1,
    #[doc = "0x4c - EFUSE_RD_MAC_SPI_SYS_2"]
    pub rd_mac_spi_sys_2: RD_MAC_SPI_SYS_2,
    #[doc = "0x50 - EFUSE_RD_MAC_SPI_SYS_3"]
    pub rd_mac_spi_sys_3: RD_MAC_SPI_SYS_3,
    #[doc = "0x54 - EFUSE_RD_MAC_SPI_SYS_4"]
    pub rd_mac_spi_sys_4: RD_MAC_SPI_SYS_4,
    #[doc = "0x58 - EFUSE_RD_MAC_SPI_SYS_5"]
    pub rd_mac_spi_sys_5: RD_MAC_SPI_SYS_5,
    #[doc = "0x5c - EFUSE_RD_SYS_PART1_DATA0"]
    pub rd_sys_part1_data0: RD_SYS_PART1_DATA0,
    #[doc = "0x60 - EFUSE_RD_SYS_PART1_DATA1"]
    pub rd_sys_part1_data1: RD_SYS_PART1_DATA1,
    #[doc = "0x64 - EFUSE_RD_SYS_PART1_DATA2"]
    pub rd_sys_part1_data2: RD_SYS_PART1_DATA2,
    #[doc = "0x68 - EFUSE_RD_SYS_PART1_DATA3"]
    pub rd_sys_part1_data3: RD_SYS_PART1_DATA3,
    #[doc = "0x6c - EFUSE_RD_SYS_PART1_DATA4"]
    pub rd_sys_part1_data4: RD_SYS_PART1_DATA4,
    #[doc = "0x70 - EFUSE_RD_SYS_PART1_DATA5"]
    pub rd_sys_part1_data5: RD_SYS_PART1_DATA5,
    #[doc = "0x74 - EFUSE_RD_SYS_PART1_DATA6"]
    pub rd_sys_part1_data6: RD_SYS_PART1_DATA6,
    #[doc = "0x78 - EFUSE_RD_SYS_PART1_DATA7"]
    pub rd_sys_part1_data7: RD_SYS_PART1_DATA7,
    #[doc = "0x7c - EFUSE_RD_USR_DATA0"]
    pub rd_usr_data0: RD_USR_DATA0,
    #[doc = "0x80 - EFUSE_RD_USR_DATA1"]
    pub rd_usr_data1: RD_USR_DATA1,
    #[doc = "0x84 - EFUSE_RD_USR_DATA2"]
    pub rd_usr_data2: RD_USR_DATA2,
    #[doc = "0x88 - EFUSE_RD_USR_DATA3"]
    pub rd_usr_data3: RD_USR_DATA3,
    #[doc = "0x8c - EFUSE_RD_USR_DATA4"]
    pub rd_usr_data4: RD_USR_DATA4,
    #[doc = "0x90 - EFUSE_RD_USR_DATA5"]
    pub rd_usr_data5: RD_USR_DATA5,
    #[doc = "0x94 - EFUSE_RD_USR_DATA6"]
    pub rd_usr_data6: RD_USR_DATA6,
    #[doc = "0x98 - EFUSE_RD_USR_DATA7"]
    pub rd_usr_data7: RD_USR_DATA7,
    #[doc = "0x9c - EFUSE_RD_KEY0_DATA0"]
    pub rd_key0_data0: RD_KEY0_DATA0,
    #[doc = "0xa0 - EFUSE_RD_KEY0_DATA1"]
    pub rd_key0_data1: RD_KEY0_DATA1,
    #[doc = "0xa4 - EFUSE_RD_KEY0_DATA2"]
    pub rd_key0_data2: RD_KEY0_DATA2,
    #[doc = "0xa8 - EFUSE_RD_KEY0_DATA3"]
    pub rd_key0_data3: RD_KEY0_DATA3,
    #[doc = "0xac - EFUSE_RD_KEY0_DATA4"]
    pub rd_key0_data4: RD_KEY0_DATA4,
    #[doc = "0xb0 - EFUSE_RD_KEY0_DATA5"]
    pub rd_key0_data5: RD_KEY0_DATA5,
    #[doc = "0xb4 - EFUSE_RD_KEY0_DATA6"]
    pub rd_key0_data6: RD_KEY0_DATA6,
    #[doc = "0xb8 - EFUSE_RD_KEY0_DATA7"]
    pub rd_key0_data7: RD_KEY0_DATA7,
    #[doc = "0xbc - EFUSE_RD_KEY1_DATA0"]
    pub rd_key1_data0: RD_KEY1_DATA0,
    #[doc = "0xc0 - EFUSE_RD_KEY1_DATA1"]
    pub rd_key1_data1: RD_KEY1_DATA1,
    #[doc = "0xc4 - EFUSE_RD_KEY1_DATA2"]
    pub rd_key1_data2: RD_KEY1_DATA2,
    #[doc = "0xc8 - EFUSE_RD_KEY1_DATA3"]
    pub rd_key1_data3: RD_KEY1_DATA3,
    #[doc = "0xcc - EFUSE_RD_KEY1_DATA4"]
    pub rd_key1_data4: RD_KEY1_DATA4,
    #[doc = "0xd0 - EFUSE_RD_KEY1_DATA5"]
    pub rd_key1_data5: RD_KEY1_DATA5,
    #[doc = "0xd4 - EFUSE_RD_KEY1_DATA6"]
    pub rd_key1_data6: RD_KEY1_DATA6,
    #[doc = "0xd8 - EFUSE_RD_KEY1_DATA7"]
    pub rd_key1_data7: RD_KEY1_DATA7,
    #[doc = "0xdc - EFUSE_RD_KEY2_DATA0"]
    pub rd_key2_data0: RD_KEY2_DATA0,
    #[doc = "0xe0 - EFUSE_RD_KEY2_DATA1"]
    pub rd_key2_data1: RD_KEY2_DATA1,
    #[doc = "0xe4 - EFUSE_RD_KEY2_DATA2"]
    pub rd_key2_data2: RD_KEY2_DATA2,
    #[doc = "0xe8 - EFUSE_RD_KEY2_DATA3"]
    pub rd_key2_data3: RD_KEY2_DATA3,
    #[doc = "0xec - EFUSE_RD_KEY2_DATA4"]
    pub rd_key2_data4: RD_KEY2_DATA4,
    #[doc = "0xf0 - EFUSE_RD_KEY2_DATA5"]
    pub rd_key2_data5: RD_KEY2_DATA5,
    #[doc = "0xf4 - EFUSE_RD_KEY2_DATA6"]
    pub rd_key2_data6: RD_KEY2_DATA6,
    #[doc = "0xf8 - EFUSE_RD_KEY2_DATA7"]
    pub rd_key2_data7: RD_KEY2_DATA7,
    #[doc = "0xfc - EFUSE_RD_KEY3_DATA0"]
    pub rd_key3_data0: RD_KEY3_DATA0,
    #[doc = "0x100 - EFUSE_RD_KEY3_DATA1"]
    pub rd_key3_data1: RD_KEY3_DATA1,
    #[doc = "0x104 - EFUSE_RD_KEY3_DATA2"]
    pub rd_key3_data2: RD_KEY3_DATA2,
    #[doc = "0x108 - EFUSE_RD_KEY3_DATA3"]
    pub rd_key3_data3: RD_KEY3_DATA3,
    #[doc = "0x10c - EFUSE_RD_KEY3_DATA4"]
    pub rd_key3_data4: RD_KEY3_DATA4,
    #[doc = "0x110 - EFUSE_RD_KEY3_DATA5"]
    pub rd_key3_data5: RD_KEY3_DATA5,
    #[doc = "0x114 - EFUSE_RD_KEY3_DATA6"]
    pub rd_key3_data6: RD_KEY3_DATA6,
    #[doc = "0x118 - EFUSE_RD_KEY3_DATA7"]
    pub rd_key3_data7: RD_KEY3_DATA7,
    #[doc = "0x11c - EFUSE_RD_KEY4_DATA0"]
    pub rd_key4_data0: RD_KEY4_DATA0,
    #[doc = "0x120 - EFUSE_RD_KEY4_DATA1"]
    pub rd_key4_data1: RD_KEY4_DATA1,
    #[doc = "0x124 - EFUSE_RD_KEY4_DATA2"]
    pub rd_key4_data2: RD_KEY4_DATA2,
    #[doc = "0x128 - EFUSE_RD_KEY4_DATA3"]
    pub rd_key4_data3: RD_KEY4_DATA3,
    #[doc = "0x12c - EFUSE_RD_KEY4_DATA4"]
    pub rd_key4_data4: RD_KEY4_DATA4,
    #[doc = "0x130 - EFUSE_RD_KEY4_DATA5"]
    pub rd_key4_data5: RD_KEY4_DATA5,
    #[doc = "0x134 - EFUSE_RD_KEY4_DATA6"]
    pub rd_key4_data6: RD_KEY4_DATA6,
    #[doc = "0x138 - EFUSE_RD_KEY4_DATA7"]
    pub rd_key4_data7: RD_KEY4_DATA7,
    #[doc = "0x13c - EFUSE_RD_KEY5_DATA0"]
    pub rd_key5_data0: RD_KEY5_DATA0,
    #[doc = "0x140 - EFUSE_RD_KEY5_DATA1"]
    pub rd_key5_data1: RD_KEY5_DATA1,
    #[doc = "0x144 - EFUSE_RD_KEY5_DATA2"]
    pub rd_key5_data2: RD_KEY5_DATA2,
    #[doc = "0x148 - EFUSE_RD_KEY5_DATA3"]
    pub rd_key5_data3: RD_KEY5_DATA3,
    #[doc = "0x14c - EFUSE_RD_KEY5_DATA4"]
    pub rd_key5_data4: RD_KEY5_DATA4,
    #[doc = "0x150 - EFUSE_RD_KEY5_DATA5"]
    pub rd_key5_data5: RD_KEY5_DATA5,
    #[doc = "0x154 - EFUSE_RD_KEY5_DATA6"]
    pub rd_key5_data6: RD_KEY5_DATA6,
    #[doc = "0x158 - EFUSE_RD_KEY5_DATA7"]
    pub rd_key5_data7: RD_KEY5_DATA7,
    #[doc = "0x15c - EFUSE_RD_SYS_PART2_DATA0"]
    pub rd_sys_part2_data0: RD_SYS_PART2_DATA0,
    #[doc = "0x160 - EFUSE_RD_SYS_PART2_DATA1"]
    pub rd_sys_part2_data1: RD_SYS_PART2_DATA1,
    #[doc = "0x164 - EFUSE_RD_SYS_PART2_DATA2"]
    pub rd_sys_part2_data2: RD_SYS_PART2_DATA2,
    #[doc = "0x168 - EFUSE_RD_SYS_PART2_DATA3"]
    pub rd_sys_part2_data3: RD_SYS_PART2_DATA3,
    #[doc = "0x16c - EFUSE_RD_SYS_PART2_DATA4"]
    pub rd_sys_part2_data4: RD_SYS_PART2_DATA4,
    #[doc = "0x170 - EFUSE_RD_SYS_PART2_DATA5"]
    pub rd_sys_part2_data5: RD_SYS_PART2_DATA5,
    #[doc = "0x174 - EFUSE_RD_SYS_PART2_DATA6"]
    pub rd_sys_part2_data6: RD_SYS_PART2_DATA6,
    #[doc = "0x178 - EFUSE_RD_SYS_PART2_DATA7"]
    pub rd_sys_part2_data7: RD_SYS_PART2_DATA7,
    #[doc = "0x17c - EFUSE_RD_REPEAT_ERR0"]
    pub rd_repeat_err0: RD_REPEAT_ERR0,
    #[doc = "0x180 - EFUSE_RD_REPEAT_ERR1"]
    pub rd_repeat_err1: RD_REPEAT_ERR1,
    #[doc = "0x184 - EFUSE_RD_REPEAT_ERR2"]
    pub rd_repeat_err2: RD_REPEAT_ERR2,
    #[doc = "0x188 - EFUSE_RD_REPEAT_ERR3"]
    pub rd_repeat_err3: RD_REPEAT_ERR3,
    _reserved99: [u8; 4usize],
    #[doc = "0x190 - EFUSE_RD_REPEAT_ERR4"]
    pub rd_repeat_err4: RD_REPEAT_ERR4,
    _reserved100: [u8; 44usize],
    #[doc = "0x1c0 - EFUSE_RD_RS_ERR0"]
    pub rd_rs_err0: RD_RS_ERR0,
    #[doc = "0x1c4 - EFUSE_RD_RS_ERR1"]
    pub rd_rs_err1: RD_RS_ERR1,
    #[doc = "0x1c8 - EFUSE_CLK"]
    pub clk: CLK,
    #[doc = "0x1cc - EFUSE_CONF"]
    pub conf: CONF,
    #[doc = "0x1d0 - EFUSE_STATUS"]
    pub status: STATUS,
    #[doc = "0x1d4 - EFUSE_CMD"]
    pub cmd: CMD,
    #[doc = "0x1d8 - EFUSE_INT_RAW"]
    pub int_raw: INT_RAW,
    #[doc = "0x1dc - EFUSE_INT_ST"]
    pub int_st: INT_ST,
    #[doc = "0x1e0 - EFUSE_INT_ENA"]
    pub int_ena: INT_ENA,
    #[doc = "0x1e4 - EFUSE_INT_CLR"]
    pub int_clr: INT_CLR,
    #[doc = "0x1e8 - EFUSE_DAC_CONF"]
    pub dac_conf: DAC_CONF,
    #[doc = "0x1ec - EFUSE_RD_TIM_CONF"]
    pub rd_tim_conf: RD_TIM_CONF,
    #[doc = "0x1f0 - EFUSE_WR_TIM_CONF1"]
    pub wr_tim_conf1: WR_TIM_CONF1,
    #[doc = "0x1f4 - EFUSE_WR_TIM_CONF2"]
    pub wr_tim_conf2: WR_TIM_CONF2,
    _reserved114: [u8; 4usize],
    #[doc = "0x1fc - EFUSE_DATE"]
    pub date: DATE,
}
#[doc = "EFUSE_PGM_DATA0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgm_data0](pgm_data0) module"]
pub type PGM_DATA0 = crate::Reg<u32, _PGM_DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PGM_DATA0;
#[doc = "`read()` method returns [pgm_data0::R](pgm_data0::R) reader structure"]
impl crate::Readable for PGM_DATA0 {}
#[doc = "`write(|w| ..)` method takes [pgm_data0::W](pgm_data0::W) writer structure"]
impl crate::Writable for PGM_DATA0 {}
#[doc = "EFUSE_PGM_DATA0"]
pub mod pgm_data0;
#[doc = "EFUSE_PGM_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgm_data1](pgm_data1) module"]
pub type PGM_DATA1 = crate::Reg<u32, _PGM_DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PGM_DATA1;
#[doc = "`read()` method returns [pgm_data1::R](pgm_data1::R) reader structure"]
impl crate::Readable for PGM_DATA1 {}
#[doc = "`write(|w| ..)` method takes [pgm_data1::W](pgm_data1::W) writer structure"]
impl crate::Writable for PGM_DATA1 {}
#[doc = "EFUSE_PGM_DATA1"]
pub mod pgm_data1;
#[doc = "EFUSE_PGM_DATA2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgm_data2](pgm_data2) module"]
pub type PGM_DATA2 = crate::Reg<u32, _PGM_DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PGM_DATA2;
#[doc = "`read()` method returns [pgm_data2::R](pgm_data2::R) reader structure"]
impl crate::Readable for PGM_DATA2 {}
#[doc = "`write(|w| ..)` method takes [pgm_data2::W](pgm_data2::W) writer structure"]
impl crate::Writable for PGM_DATA2 {}
#[doc = "EFUSE_PGM_DATA2"]
pub mod pgm_data2;
#[doc = "EFUSE_PGM_DATA3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgm_data3](pgm_data3) module"]
pub type PGM_DATA3 = crate::Reg<u32, _PGM_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PGM_DATA3;
#[doc = "`read()` method returns [pgm_data3::R](pgm_data3::R) reader structure"]
impl crate::Readable for PGM_DATA3 {}
#[doc = "`write(|w| ..)` method takes [pgm_data3::W](pgm_data3::W) writer structure"]
impl crate::Writable for PGM_DATA3 {}
#[doc = "EFUSE_PGM_DATA3"]
pub mod pgm_data3;
#[doc = "EFUSE_PGM_DATA4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgm_data4](pgm_data4) module"]
pub type PGM_DATA4 = crate::Reg<u32, _PGM_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PGM_DATA4;
#[doc = "`read()` method returns [pgm_data4::R](pgm_data4::R) reader structure"]
impl crate::Readable for PGM_DATA4 {}
#[doc = "`write(|w| ..)` method takes [pgm_data4::W](pgm_data4::W) writer structure"]
impl crate::Writable for PGM_DATA4 {}
#[doc = "EFUSE_PGM_DATA4"]
pub mod pgm_data4;
#[doc = "EFUSE_PGM_DATA5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgm_data5](pgm_data5) module"]
pub type PGM_DATA5 = crate::Reg<u32, _PGM_DATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PGM_DATA5;
#[doc = "`read()` method returns [pgm_data5::R](pgm_data5::R) reader structure"]
impl crate::Readable for PGM_DATA5 {}
#[doc = "EFUSE_PGM_DATA5"]
pub mod pgm_data5;
#[doc = "EFUSE_PGM_DATA6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgm_data6](pgm_data6) module"]
pub type PGM_DATA6 = crate::Reg<u32, _PGM_DATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PGM_DATA6;
#[doc = "`read()` method returns [pgm_data6::R](pgm_data6::R) reader structure"]
impl crate::Readable for PGM_DATA6 {}
#[doc = "`write(|w| ..)` method takes [pgm_data6::W](pgm_data6::W) writer structure"]
impl crate::Writable for PGM_DATA6 {}
#[doc = "EFUSE_PGM_DATA6"]
pub mod pgm_data6;
#[doc = "EFUSE_PGM_DATA7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgm_data7](pgm_data7) module"]
pub type PGM_DATA7 = crate::Reg<u32, _PGM_DATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PGM_DATA7;
#[doc = "`read()` method returns [pgm_data7::R](pgm_data7::R) reader structure"]
impl crate::Readable for PGM_DATA7 {}
#[doc = "`write(|w| ..)` method takes [pgm_data7::W](pgm_data7::W) writer structure"]
impl crate::Writable for PGM_DATA7 {}
#[doc = "EFUSE_PGM_DATA7"]
pub mod pgm_data7;
#[doc = "EFUSE_PGM_CHECK_VALUE0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgm_check_value0](pgm_check_value0) module"]
pub type PGM_CHECK_VALUE0 = crate::Reg<u32, _PGM_CHECK_VALUE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PGM_CHECK_VALUE0;
#[doc = "`read()` method returns [pgm_check_value0::R](pgm_check_value0::R) reader structure"]
impl crate::Readable for PGM_CHECK_VALUE0 {}
#[doc = "`write(|w| ..)` method takes [pgm_check_value0::W](pgm_check_value0::W) writer structure"]
impl crate::Writable for PGM_CHECK_VALUE0 {}
#[doc = "EFUSE_PGM_CHECK_VALUE0"]
pub mod pgm_check_value0;
#[doc = "EFUSE_PGM_CHECK_VALUE1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgm_check_value1](pgm_check_value1) module"]
pub type PGM_CHECK_VALUE1 = crate::Reg<u32, _PGM_CHECK_VALUE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PGM_CHECK_VALUE1;
#[doc = "`read()` method returns [pgm_check_value1::R](pgm_check_value1::R) reader structure"]
impl crate::Readable for PGM_CHECK_VALUE1 {}
#[doc = "`write(|w| ..)` method takes [pgm_check_value1::W](pgm_check_value1::W) writer structure"]
impl crate::Writable for PGM_CHECK_VALUE1 {}
#[doc = "EFUSE_PGM_CHECK_VALUE1"]
pub mod pgm_check_value1;
#[doc = "EFUSE_PGM_CHECK_VALUE2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgm_check_value2](pgm_check_value2) module"]
pub type PGM_CHECK_VALUE2 = crate::Reg<u32, _PGM_CHECK_VALUE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PGM_CHECK_VALUE2;
#[doc = "`read()` method returns [pgm_check_value2::R](pgm_check_value2::R) reader structure"]
impl crate::Readable for PGM_CHECK_VALUE2 {}
#[doc = "`write(|w| ..)` method takes [pgm_check_value2::W](pgm_check_value2::W) writer structure"]
impl crate::Writable for PGM_CHECK_VALUE2 {}
#[doc = "EFUSE_PGM_CHECK_VALUE2"]
pub mod pgm_check_value2;
#[doc = "EFUSE_RD_WR_DIS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_wr_dis](rd_wr_dis) module"]
pub type RD_WR_DIS = crate::Reg<u32, _RD_WR_DIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_WR_DIS;
#[doc = "`read()` method returns [rd_wr_dis::R](rd_wr_dis::R) reader structure"]
impl crate::Readable for RD_WR_DIS {}
#[doc = "EFUSE_RD_WR_DIS"]
pub mod rd_wr_dis;
#[doc = "EFUSE_RD_REPEAT_DATA0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_data0](rd_repeat_data0) module"]
pub type RD_REPEAT_DATA0 = crate::Reg<u32, _RD_REPEAT_DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_REPEAT_DATA0;
#[doc = "`read()` method returns [rd_repeat_data0::R](rd_repeat_data0::R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA0 {}
#[doc = "EFUSE_RD_REPEAT_DATA0"]
pub mod rd_repeat_data0;
#[doc = "EFUSE_RD_REPEAT_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_data1](rd_repeat_data1) module"]
pub type RD_REPEAT_DATA1 = crate::Reg<u32, _RD_REPEAT_DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_REPEAT_DATA1;
#[doc = "`read()` method returns [rd_repeat_data1::R](rd_repeat_data1::R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA1 {}
#[doc = "EFUSE_RD_REPEAT_DATA1"]
pub mod rd_repeat_data1;
#[doc = "EFUSE_RD_REPEAT_DATA2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_data2](rd_repeat_data2) module"]
pub type RD_REPEAT_DATA2 = crate::Reg<u32, _RD_REPEAT_DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_REPEAT_DATA2;
#[doc = "`read()` method returns [rd_repeat_data2::R](rd_repeat_data2::R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA2 {}
#[doc = "EFUSE_RD_REPEAT_DATA2"]
pub mod rd_repeat_data2;
#[doc = "EFUSE_RD_REPEAT_DATA3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_data3](rd_repeat_data3) module"]
pub type RD_REPEAT_DATA3 = crate::Reg<u32, _RD_REPEAT_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_REPEAT_DATA3;
#[doc = "`read()` method returns [rd_repeat_data3::R](rd_repeat_data3::R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA3 {}
#[doc = "EFUSE_RD_REPEAT_DATA3"]
pub mod rd_repeat_data3;
#[doc = "EFUSE_RD_REPEAT_DATA4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_data4](rd_repeat_data4) module"]
pub type RD_REPEAT_DATA4 = crate::Reg<u32, _RD_REPEAT_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_REPEAT_DATA4;
#[doc = "`read()` method returns [rd_repeat_data4::R](rd_repeat_data4::R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA4 {}
#[doc = "EFUSE_RD_REPEAT_DATA4"]
pub mod rd_repeat_data4;
#[doc = "EFUSE_RD_MAC_SPI_SYS_0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_mac_spi_sys_0](rd_mac_spi_sys_0) module"]
pub type RD_MAC_SPI_SYS_0 = crate::Reg<u32, _RD_MAC_SPI_SYS_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_MAC_SPI_SYS_0;
#[doc = "`read()` method returns [rd_mac_spi_sys_0::R](rd_mac_spi_sys_0::R) reader structure"]
impl crate::Readable for RD_MAC_SPI_SYS_0 {}
#[doc = "EFUSE_RD_MAC_SPI_SYS_0"]
pub mod rd_mac_spi_sys_0;
#[doc = "EFUSE_RD_MAC_SPI_SYS_1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_mac_spi_sys_1](rd_mac_spi_sys_1) module"]
pub type RD_MAC_SPI_SYS_1 = crate::Reg<u32, _RD_MAC_SPI_SYS_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_MAC_SPI_SYS_1;
#[doc = "`read()` method returns [rd_mac_spi_sys_1::R](rd_mac_spi_sys_1::R) reader structure"]
impl crate::Readable for RD_MAC_SPI_SYS_1 {}
#[doc = "EFUSE_RD_MAC_SPI_SYS_1"]
pub mod rd_mac_spi_sys_1;
#[doc = "EFUSE_RD_MAC_SPI_SYS_2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_mac_spi_sys_2](rd_mac_spi_sys_2) module"]
pub type RD_MAC_SPI_SYS_2 = crate::Reg<u32, _RD_MAC_SPI_SYS_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_MAC_SPI_SYS_2;
#[doc = "`read()` method returns [rd_mac_spi_sys_2::R](rd_mac_spi_sys_2::R) reader structure"]
impl crate::Readable for RD_MAC_SPI_SYS_2 {}
#[doc = "EFUSE_RD_MAC_SPI_SYS_2"]
pub mod rd_mac_spi_sys_2;
#[doc = "EFUSE_RD_MAC_SPI_SYS_3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_mac_spi_sys_3](rd_mac_spi_sys_3) module"]
pub type RD_MAC_SPI_SYS_3 = crate::Reg<u32, _RD_MAC_SPI_SYS_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_MAC_SPI_SYS_3;
#[doc = "`read()` method returns [rd_mac_spi_sys_3::R](rd_mac_spi_sys_3::R) reader structure"]
impl crate::Readable for RD_MAC_SPI_SYS_3 {}
#[doc = "EFUSE_RD_MAC_SPI_SYS_3"]
pub mod rd_mac_spi_sys_3;
#[doc = "EFUSE_RD_MAC_SPI_SYS_4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_mac_spi_sys_4](rd_mac_spi_sys_4) module"]
pub type RD_MAC_SPI_SYS_4 = crate::Reg<u32, _RD_MAC_SPI_SYS_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_MAC_SPI_SYS_4;
#[doc = "`read()` method returns [rd_mac_spi_sys_4::R](rd_mac_spi_sys_4::R) reader structure"]
impl crate::Readable for RD_MAC_SPI_SYS_4 {}
#[doc = "EFUSE_RD_MAC_SPI_SYS_4"]
pub mod rd_mac_spi_sys_4;
#[doc = "EFUSE_RD_MAC_SPI_SYS_5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_mac_spi_sys_5](rd_mac_spi_sys_5) module"]
pub type RD_MAC_SPI_SYS_5 = crate::Reg<u32, _RD_MAC_SPI_SYS_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_MAC_SPI_SYS_5;
#[doc = "`read()` method returns [rd_mac_spi_sys_5::R](rd_mac_spi_sys_5::R) reader structure"]
impl crate::Readable for RD_MAC_SPI_SYS_5 {}
#[doc = "EFUSE_RD_MAC_SPI_SYS_5"]
pub mod rd_mac_spi_sys_5;
#[doc = "EFUSE_RD_SYS_PART1_DATA0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_sys_part1_data0](rd_sys_part1_data0) module"]
pub type RD_SYS_PART1_DATA0 = crate::Reg<u32, _RD_SYS_PART1_DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_SYS_PART1_DATA0;
#[doc = "`read()` method returns [rd_sys_part1_data0::R](rd_sys_part1_data0::R) reader structure"]
impl crate::Readable for RD_SYS_PART1_DATA0 {}
#[doc = "EFUSE_RD_SYS_PART1_DATA0"]
pub mod rd_sys_part1_data0;
#[doc = "EFUSE_RD_SYS_PART1_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_sys_part1_data1](rd_sys_part1_data1) module"]
pub type RD_SYS_PART1_DATA1 = crate::Reg<u32, _RD_SYS_PART1_DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_SYS_PART1_DATA1;
#[doc = "`read()` method returns [rd_sys_part1_data1::R](rd_sys_part1_data1::R) reader structure"]
impl crate::Readable for RD_SYS_PART1_DATA1 {}
#[doc = "EFUSE_RD_SYS_PART1_DATA1"]
pub mod rd_sys_part1_data1;
#[doc = "EFUSE_RD_SYS_PART1_DATA2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_sys_part1_data2](rd_sys_part1_data2) module"]
pub type RD_SYS_PART1_DATA2 = crate::Reg<u32, _RD_SYS_PART1_DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_SYS_PART1_DATA2;
#[doc = "`read()` method returns [rd_sys_part1_data2::R](rd_sys_part1_data2::R) reader structure"]
impl crate::Readable for RD_SYS_PART1_DATA2 {}
#[doc = "EFUSE_RD_SYS_PART1_DATA2"]
pub mod rd_sys_part1_data2;
#[doc = "EFUSE_RD_SYS_PART1_DATA3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_sys_part1_data3](rd_sys_part1_data3) module"]
pub type RD_SYS_PART1_DATA3 = crate::Reg<u32, _RD_SYS_PART1_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_SYS_PART1_DATA3;
#[doc = "`read()` method returns [rd_sys_part1_data3::R](rd_sys_part1_data3::R) reader structure"]
impl crate::Readable for RD_SYS_PART1_DATA3 {}
#[doc = "EFUSE_RD_SYS_PART1_DATA3"]
pub mod rd_sys_part1_data3;
#[doc = "EFUSE_RD_SYS_PART1_DATA4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_sys_part1_data4](rd_sys_part1_data4) module"]
pub type RD_SYS_PART1_DATA4 = crate::Reg<u32, _RD_SYS_PART1_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_SYS_PART1_DATA4;
#[doc = "`read()` method returns [rd_sys_part1_data4::R](rd_sys_part1_data4::R) reader structure"]
impl crate::Readable for RD_SYS_PART1_DATA4 {}
#[doc = "EFUSE_RD_SYS_PART1_DATA4"]
pub mod rd_sys_part1_data4;
#[doc = "EFUSE_RD_SYS_PART1_DATA5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_sys_part1_data5](rd_sys_part1_data5) module"]
pub type RD_SYS_PART1_DATA5 = crate::Reg<u32, _RD_SYS_PART1_DATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_SYS_PART1_DATA5;
#[doc = "`read()` method returns [rd_sys_part1_data5::R](rd_sys_part1_data5::R) reader structure"]
impl crate::Readable for RD_SYS_PART1_DATA5 {}
#[doc = "EFUSE_RD_SYS_PART1_DATA5"]
pub mod rd_sys_part1_data5;
#[doc = "EFUSE_RD_SYS_PART1_DATA6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_sys_part1_data6](rd_sys_part1_data6) module"]
pub type RD_SYS_PART1_DATA6 = crate::Reg<u32, _RD_SYS_PART1_DATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_SYS_PART1_DATA6;
#[doc = "`read()` method returns [rd_sys_part1_data6::R](rd_sys_part1_data6::R) reader structure"]
impl crate::Readable for RD_SYS_PART1_DATA6 {}
#[doc = "EFUSE_RD_SYS_PART1_DATA6"]
pub mod rd_sys_part1_data6;
#[doc = "EFUSE_RD_SYS_PART1_DATA7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_sys_part1_data7](rd_sys_part1_data7) module"]
pub type RD_SYS_PART1_DATA7 = crate::Reg<u32, _RD_SYS_PART1_DATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_SYS_PART1_DATA7;
#[doc = "`read()` method returns [rd_sys_part1_data7::R](rd_sys_part1_data7::R) reader structure"]
impl crate::Readable for RD_SYS_PART1_DATA7 {}
#[doc = "EFUSE_RD_SYS_PART1_DATA7"]
pub mod rd_sys_part1_data7;
#[doc = "EFUSE_RD_USR_DATA0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_usr_data0](rd_usr_data0) module"]
pub type RD_USR_DATA0 = crate::Reg<u32, _RD_USR_DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_USR_DATA0;
#[doc = "`read()` method returns [rd_usr_data0::R](rd_usr_data0::R) reader structure"]
impl crate::Readable for RD_USR_DATA0 {}
#[doc = "EFUSE_RD_USR_DATA0"]
pub mod rd_usr_data0;
#[doc = "EFUSE_RD_USR_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_usr_data1](rd_usr_data1) module"]
pub type RD_USR_DATA1 = crate::Reg<u32, _RD_USR_DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_USR_DATA1;
#[doc = "`read()` method returns [rd_usr_data1::R](rd_usr_data1::R) reader structure"]
impl crate::Readable for RD_USR_DATA1 {}
#[doc = "EFUSE_RD_USR_DATA1"]
pub mod rd_usr_data1;
#[doc = "EFUSE_RD_USR_DATA2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_usr_data2](rd_usr_data2) module"]
pub type RD_USR_DATA2 = crate::Reg<u32, _RD_USR_DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_USR_DATA2;
#[doc = "`read()` method returns [rd_usr_data2::R](rd_usr_data2::R) reader structure"]
impl crate::Readable for RD_USR_DATA2 {}
#[doc = "EFUSE_RD_USR_DATA2"]
pub mod rd_usr_data2;
#[doc = "EFUSE_RD_USR_DATA3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_usr_data3](rd_usr_data3) module"]
pub type RD_USR_DATA3 = crate::Reg<u32, _RD_USR_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_USR_DATA3;
#[doc = "`read()` method returns [rd_usr_data3::R](rd_usr_data3::R) reader structure"]
impl crate::Readable for RD_USR_DATA3 {}
#[doc = "EFUSE_RD_USR_DATA3"]
pub mod rd_usr_data3;
#[doc = "EFUSE_RD_USR_DATA4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_usr_data4](rd_usr_data4) module"]
pub type RD_USR_DATA4 = crate::Reg<u32, _RD_USR_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_USR_DATA4;
#[doc = "`read()` method returns [rd_usr_data4::R](rd_usr_data4::R) reader structure"]
impl crate::Readable for RD_USR_DATA4 {}
#[doc = "EFUSE_RD_USR_DATA4"]
pub mod rd_usr_data4;
#[doc = "EFUSE_RD_USR_DATA5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_usr_data5](rd_usr_data5) module"]
pub type RD_USR_DATA5 = crate::Reg<u32, _RD_USR_DATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_USR_DATA5;
#[doc = "`read()` method returns [rd_usr_data5::R](rd_usr_data5::R) reader structure"]
impl crate::Readable for RD_USR_DATA5 {}
#[doc = "EFUSE_RD_USR_DATA5"]
pub mod rd_usr_data5;
#[doc = "EFUSE_RD_USR_DATA6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_usr_data6](rd_usr_data6) module"]
pub type RD_USR_DATA6 = crate::Reg<u32, _RD_USR_DATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_USR_DATA6;
#[doc = "`read()` method returns [rd_usr_data6::R](rd_usr_data6::R) reader structure"]
impl crate::Readable for RD_USR_DATA6 {}
#[doc = "EFUSE_RD_USR_DATA6"]
pub mod rd_usr_data6;
#[doc = "EFUSE_RD_USR_DATA7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_usr_data7](rd_usr_data7) module"]
pub type RD_USR_DATA7 = crate::Reg<u32, _RD_USR_DATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_USR_DATA7;
#[doc = "`read()` method returns [rd_usr_data7::R](rd_usr_data7::R) reader structure"]
impl crate::Readable for RD_USR_DATA7 {}
#[doc = "EFUSE_RD_USR_DATA7"]
pub mod rd_usr_data7;
#[doc = "EFUSE_RD_KEY0_DATA0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key0_data0](rd_key0_data0) module"]
pub type RD_KEY0_DATA0 = crate::Reg<u32, _RD_KEY0_DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY0_DATA0;
#[doc = "`read()` method returns [rd_key0_data0::R](rd_key0_data0::R) reader structure"]
impl crate::Readable for RD_KEY0_DATA0 {}
#[doc = "EFUSE_RD_KEY0_DATA0"]
pub mod rd_key0_data0;
#[doc = "EFUSE_RD_KEY0_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key0_data1](rd_key0_data1) module"]
pub type RD_KEY0_DATA1 = crate::Reg<u32, _RD_KEY0_DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY0_DATA1;
#[doc = "`read()` method returns [rd_key0_data1::R](rd_key0_data1::R) reader structure"]
impl crate::Readable for RD_KEY0_DATA1 {}
#[doc = "EFUSE_RD_KEY0_DATA1"]
pub mod rd_key0_data1;
#[doc = "EFUSE_RD_KEY0_DATA2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key0_data2](rd_key0_data2) module"]
pub type RD_KEY0_DATA2 = crate::Reg<u32, _RD_KEY0_DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY0_DATA2;
#[doc = "`read()` method returns [rd_key0_data2::R](rd_key0_data2::R) reader structure"]
impl crate::Readable for RD_KEY0_DATA2 {}
#[doc = "EFUSE_RD_KEY0_DATA2"]
pub mod rd_key0_data2;
#[doc = "EFUSE_RD_KEY0_DATA3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key0_data3](rd_key0_data3) module"]
pub type RD_KEY0_DATA3 = crate::Reg<u32, _RD_KEY0_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY0_DATA3;
#[doc = "`read()` method returns [rd_key0_data3::R](rd_key0_data3::R) reader structure"]
impl crate::Readable for RD_KEY0_DATA3 {}
#[doc = "EFUSE_RD_KEY0_DATA3"]
pub mod rd_key0_data3;
#[doc = "EFUSE_RD_KEY0_DATA4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key0_data4](rd_key0_data4) module"]
pub type RD_KEY0_DATA4 = crate::Reg<u32, _RD_KEY0_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY0_DATA4;
#[doc = "`read()` method returns [rd_key0_data4::R](rd_key0_data4::R) reader structure"]
impl crate::Readable for RD_KEY0_DATA4 {}
#[doc = "EFUSE_RD_KEY0_DATA4"]
pub mod rd_key0_data4;
#[doc = "EFUSE_RD_KEY0_DATA5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key0_data5](rd_key0_data5) module"]
pub type RD_KEY0_DATA5 = crate::Reg<u32, _RD_KEY0_DATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY0_DATA5;
#[doc = "`read()` method returns [rd_key0_data5::R](rd_key0_data5::R) reader structure"]
impl crate::Readable for RD_KEY0_DATA5 {}
#[doc = "EFUSE_RD_KEY0_DATA5"]
pub mod rd_key0_data5;
#[doc = "EFUSE_RD_KEY0_DATA6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key0_data6](rd_key0_data6) module"]
pub type RD_KEY0_DATA6 = crate::Reg<u32, _RD_KEY0_DATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY0_DATA6;
#[doc = "`read()` method returns [rd_key0_data6::R](rd_key0_data6::R) reader structure"]
impl crate::Readable for RD_KEY0_DATA6 {}
#[doc = "EFUSE_RD_KEY0_DATA6"]
pub mod rd_key0_data6;
#[doc = "EFUSE_RD_KEY0_DATA7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key0_data7](rd_key0_data7) module"]
pub type RD_KEY0_DATA7 = crate::Reg<u32, _RD_KEY0_DATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY0_DATA7;
#[doc = "`read()` method returns [rd_key0_data7::R](rd_key0_data7::R) reader structure"]
impl crate::Readable for RD_KEY0_DATA7 {}
#[doc = "EFUSE_RD_KEY0_DATA7"]
pub mod rd_key0_data7;
#[doc = "EFUSE_RD_KEY1_DATA0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key1_data0](rd_key1_data0) module"]
pub type RD_KEY1_DATA0 = crate::Reg<u32, _RD_KEY1_DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY1_DATA0;
#[doc = "`read()` method returns [rd_key1_data0::R](rd_key1_data0::R) reader structure"]
impl crate::Readable for RD_KEY1_DATA0 {}
#[doc = "EFUSE_RD_KEY1_DATA0"]
pub mod rd_key1_data0;
#[doc = "EFUSE_RD_KEY1_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key1_data1](rd_key1_data1) module"]
pub type RD_KEY1_DATA1 = crate::Reg<u32, _RD_KEY1_DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY1_DATA1;
#[doc = "`read()` method returns [rd_key1_data1::R](rd_key1_data1::R) reader structure"]
impl crate::Readable for RD_KEY1_DATA1 {}
#[doc = "EFUSE_RD_KEY1_DATA1"]
pub mod rd_key1_data1;
#[doc = "EFUSE_RD_KEY1_DATA2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key1_data2](rd_key1_data2) module"]
pub type RD_KEY1_DATA2 = crate::Reg<u32, _RD_KEY1_DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY1_DATA2;
#[doc = "`read()` method returns [rd_key1_data2::R](rd_key1_data2::R) reader structure"]
impl crate::Readable for RD_KEY1_DATA2 {}
#[doc = "EFUSE_RD_KEY1_DATA2"]
pub mod rd_key1_data2;
#[doc = "EFUSE_RD_KEY1_DATA3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key1_data3](rd_key1_data3) module"]
pub type RD_KEY1_DATA3 = crate::Reg<u32, _RD_KEY1_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY1_DATA3;
#[doc = "`read()` method returns [rd_key1_data3::R](rd_key1_data3::R) reader structure"]
impl crate::Readable for RD_KEY1_DATA3 {}
#[doc = "EFUSE_RD_KEY1_DATA3"]
pub mod rd_key1_data3;
#[doc = "EFUSE_RD_KEY1_DATA4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key1_data4](rd_key1_data4) module"]
pub type RD_KEY1_DATA4 = crate::Reg<u32, _RD_KEY1_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY1_DATA4;
#[doc = "`read()` method returns [rd_key1_data4::R](rd_key1_data4::R) reader structure"]
impl crate::Readable for RD_KEY1_DATA4 {}
#[doc = "EFUSE_RD_KEY1_DATA4"]
pub mod rd_key1_data4;
#[doc = "EFUSE_RD_KEY1_DATA5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key1_data5](rd_key1_data5) module"]
pub type RD_KEY1_DATA5 = crate::Reg<u32, _RD_KEY1_DATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY1_DATA5;
#[doc = "`read()` method returns [rd_key1_data5::R](rd_key1_data5::R) reader structure"]
impl crate::Readable for RD_KEY1_DATA5 {}
#[doc = "EFUSE_RD_KEY1_DATA5"]
pub mod rd_key1_data5;
#[doc = "EFUSE_RD_KEY1_DATA6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key1_data6](rd_key1_data6) module"]
pub type RD_KEY1_DATA6 = crate::Reg<u32, _RD_KEY1_DATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY1_DATA6;
#[doc = "`read()` method returns [rd_key1_data6::R](rd_key1_data6::R) reader structure"]
impl crate::Readable for RD_KEY1_DATA6 {}
#[doc = "EFUSE_RD_KEY1_DATA6"]
pub mod rd_key1_data6;
#[doc = "EFUSE_RD_KEY1_DATA7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key1_data7](rd_key1_data7) module"]
pub type RD_KEY1_DATA7 = crate::Reg<u32, _RD_KEY1_DATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY1_DATA7;
#[doc = "`read()` method returns [rd_key1_data7::R](rd_key1_data7::R) reader structure"]
impl crate::Readable for RD_KEY1_DATA7 {}
#[doc = "EFUSE_RD_KEY1_DATA7"]
pub mod rd_key1_data7;
#[doc = "EFUSE_RD_KEY2_DATA0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key2_data0](rd_key2_data0) module"]
pub type RD_KEY2_DATA0 = crate::Reg<u32, _RD_KEY2_DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY2_DATA0;
#[doc = "`read()` method returns [rd_key2_data0::R](rd_key2_data0::R) reader structure"]
impl crate::Readable for RD_KEY2_DATA0 {}
#[doc = "EFUSE_RD_KEY2_DATA0"]
pub mod rd_key2_data0;
#[doc = "EFUSE_RD_KEY2_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key2_data1](rd_key2_data1) module"]
pub type RD_KEY2_DATA1 = crate::Reg<u32, _RD_KEY2_DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY2_DATA1;
#[doc = "`read()` method returns [rd_key2_data1::R](rd_key2_data1::R) reader structure"]
impl crate::Readable for RD_KEY2_DATA1 {}
#[doc = "EFUSE_RD_KEY2_DATA1"]
pub mod rd_key2_data1;
#[doc = "EFUSE_RD_KEY2_DATA2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key2_data2](rd_key2_data2) module"]
pub type RD_KEY2_DATA2 = crate::Reg<u32, _RD_KEY2_DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY2_DATA2;
#[doc = "`read()` method returns [rd_key2_data2::R](rd_key2_data2::R) reader structure"]
impl crate::Readable for RD_KEY2_DATA2 {}
#[doc = "EFUSE_RD_KEY2_DATA2"]
pub mod rd_key2_data2;
#[doc = "EFUSE_RD_KEY2_DATA3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key2_data3](rd_key2_data3) module"]
pub type RD_KEY2_DATA3 = crate::Reg<u32, _RD_KEY2_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY2_DATA3;
#[doc = "`read()` method returns [rd_key2_data3::R](rd_key2_data3::R) reader structure"]
impl crate::Readable for RD_KEY2_DATA3 {}
#[doc = "EFUSE_RD_KEY2_DATA3"]
pub mod rd_key2_data3;
#[doc = "EFUSE_RD_KEY2_DATA4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key2_data4](rd_key2_data4) module"]
pub type RD_KEY2_DATA4 = crate::Reg<u32, _RD_KEY2_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY2_DATA4;
#[doc = "`read()` method returns [rd_key2_data4::R](rd_key2_data4::R) reader structure"]
impl crate::Readable for RD_KEY2_DATA4 {}
#[doc = "EFUSE_RD_KEY2_DATA4"]
pub mod rd_key2_data4;
#[doc = "EFUSE_RD_KEY2_DATA5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key2_data5](rd_key2_data5) module"]
pub type RD_KEY2_DATA5 = crate::Reg<u32, _RD_KEY2_DATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY2_DATA5;
#[doc = "`read()` method returns [rd_key2_data5::R](rd_key2_data5::R) reader structure"]
impl crate::Readable for RD_KEY2_DATA5 {}
#[doc = "EFUSE_RD_KEY2_DATA5"]
pub mod rd_key2_data5;
#[doc = "EFUSE_RD_KEY2_DATA6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key2_data6](rd_key2_data6) module"]
pub type RD_KEY2_DATA6 = crate::Reg<u32, _RD_KEY2_DATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY2_DATA6;
#[doc = "`read()` method returns [rd_key2_data6::R](rd_key2_data6::R) reader structure"]
impl crate::Readable for RD_KEY2_DATA6 {}
#[doc = "EFUSE_RD_KEY2_DATA6"]
pub mod rd_key2_data6;
#[doc = "EFUSE_RD_KEY2_DATA7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key2_data7](rd_key2_data7) module"]
pub type RD_KEY2_DATA7 = crate::Reg<u32, _RD_KEY2_DATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY2_DATA7;
#[doc = "`read()` method returns [rd_key2_data7::R](rd_key2_data7::R) reader structure"]
impl crate::Readable for RD_KEY2_DATA7 {}
#[doc = "EFUSE_RD_KEY2_DATA7"]
pub mod rd_key2_data7;
#[doc = "EFUSE_RD_KEY3_DATA0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key3_data0](rd_key3_data0) module"]
pub type RD_KEY3_DATA0 = crate::Reg<u32, _RD_KEY3_DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY3_DATA0;
#[doc = "`read()` method returns [rd_key3_data0::R](rd_key3_data0::R) reader structure"]
impl crate::Readable for RD_KEY3_DATA0 {}
#[doc = "EFUSE_RD_KEY3_DATA0"]
pub mod rd_key3_data0;
#[doc = "EFUSE_RD_KEY3_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key3_data1](rd_key3_data1) module"]
pub type RD_KEY3_DATA1 = crate::Reg<u32, _RD_KEY3_DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY3_DATA1;
#[doc = "`read()` method returns [rd_key3_data1::R](rd_key3_data1::R) reader structure"]
impl crate::Readable for RD_KEY3_DATA1 {}
#[doc = "EFUSE_RD_KEY3_DATA1"]
pub mod rd_key3_data1;
#[doc = "EFUSE_RD_KEY3_DATA2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key3_data2](rd_key3_data2) module"]
pub type RD_KEY3_DATA2 = crate::Reg<u32, _RD_KEY3_DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY3_DATA2;
#[doc = "`read()` method returns [rd_key3_data2::R](rd_key3_data2::R) reader structure"]
impl crate::Readable for RD_KEY3_DATA2 {}
#[doc = "EFUSE_RD_KEY3_DATA2"]
pub mod rd_key3_data2;
#[doc = "EFUSE_RD_KEY3_DATA3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key3_data3](rd_key3_data3) module"]
pub type RD_KEY3_DATA3 = crate::Reg<u32, _RD_KEY3_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY3_DATA3;
#[doc = "`read()` method returns [rd_key3_data3::R](rd_key3_data3::R) reader structure"]
impl crate::Readable for RD_KEY3_DATA3 {}
#[doc = "EFUSE_RD_KEY3_DATA3"]
pub mod rd_key3_data3;
#[doc = "EFUSE_RD_KEY3_DATA4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key3_data4](rd_key3_data4) module"]
pub type RD_KEY3_DATA4 = crate::Reg<u32, _RD_KEY3_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY3_DATA4;
#[doc = "`read()` method returns [rd_key3_data4::R](rd_key3_data4::R) reader structure"]
impl crate::Readable for RD_KEY3_DATA4 {}
#[doc = "EFUSE_RD_KEY3_DATA4"]
pub mod rd_key3_data4;
#[doc = "EFUSE_RD_KEY3_DATA5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key3_data5](rd_key3_data5) module"]
pub type RD_KEY3_DATA5 = crate::Reg<u32, _RD_KEY3_DATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY3_DATA5;
#[doc = "`read()` method returns [rd_key3_data5::R](rd_key3_data5::R) reader structure"]
impl crate::Readable for RD_KEY3_DATA5 {}
#[doc = "EFUSE_RD_KEY3_DATA5"]
pub mod rd_key3_data5;
#[doc = "EFUSE_RD_KEY3_DATA6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key3_data6](rd_key3_data6) module"]
pub type RD_KEY3_DATA6 = crate::Reg<u32, _RD_KEY3_DATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY3_DATA6;
#[doc = "`read()` method returns [rd_key3_data6::R](rd_key3_data6::R) reader structure"]
impl crate::Readable for RD_KEY3_DATA6 {}
#[doc = "EFUSE_RD_KEY3_DATA6"]
pub mod rd_key3_data6;
#[doc = "EFUSE_RD_KEY3_DATA7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key3_data7](rd_key3_data7) module"]
pub type RD_KEY3_DATA7 = crate::Reg<u32, _RD_KEY3_DATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY3_DATA7;
#[doc = "`read()` method returns [rd_key3_data7::R](rd_key3_data7::R) reader structure"]
impl crate::Readable for RD_KEY3_DATA7 {}
#[doc = "EFUSE_RD_KEY3_DATA7"]
pub mod rd_key3_data7;
#[doc = "EFUSE_RD_KEY4_DATA0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key4_data0](rd_key4_data0) module"]
pub type RD_KEY4_DATA0 = crate::Reg<u32, _RD_KEY4_DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY4_DATA0;
#[doc = "`read()` method returns [rd_key4_data0::R](rd_key4_data0::R) reader structure"]
impl crate::Readable for RD_KEY4_DATA0 {}
#[doc = "EFUSE_RD_KEY4_DATA0"]
pub mod rd_key4_data0;
#[doc = "EFUSE_RD_KEY4_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key4_data1](rd_key4_data1) module"]
pub type RD_KEY4_DATA1 = crate::Reg<u32, _RD_KEY4_DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY4_DATA1;
#[doc = "`read()` method returns [rd_key4_data1::R](rd_key4_data1::R) reader structure"]
impl crate::Readable for RD_KEY4_DATA1 {}
#[doc = "EFUSE_RD_KEY4_DATA1"]
pub mod rd_key4_data1;
#[doc = "EFUSE_RD_KEY4_DATA2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key4_data2](rd_key4_data2) module"]
pub type RD_KEY4_DATA2 = crate::Reg<u32, _RD_KEY4_DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY4_DATA2;
#[doc = "`read()` method returns [rd_key4_data2::R](rd_key4_data2::R) reader structure"]
impl crate::Readable for RD_KEY4_DATA2 {}
#[doc = "EFUSE_RD_KEY4_DATA2"]
pub mod rd_key4_data2;
#[doc = "EFUSE_RD_KEY4_DATA3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key4_data3](rd_key4_data3) module"]
pub type RD_KEY4_DATA3 = crate::Reg<u32, _RD_KEY4_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY4_DATA3;
#[doc = "`read()` method returns [rd_key4_data3::R](rd_key4_data3::R) reader structure"]
impl crate::Readable for RD_KEY4_DATA3 {}
#[doc = "EFUSE_RD_KEY4_DATA3"]
pub mod rd_key4_data3;
#[doc = "EFUSE_RD_KEY4_DATA4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key4_data4](rd_key4_data4) module"]
pub type RD_KEY4_DATA4 = crate::Reg<u32, _RD_KEY4_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY4_DATA4;
#[doc = "`read()` method returns [rd_key4_data4::R](rd_key4_data4::R) reader structure"]
impl crate::Readable for RD_KEY4_DATA4 {}
#[doc = "EFUSE_RD_KEY4_DATA4"]
pub mod rd_key4_data4;
#[doc = "EFUSE_RD_KEY4_DATA5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key4_data5](rd_key4_data5) module"]
pub type RD_KEY4_DATA5 = crate::Reg<u32, _RD_KEY4_DATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY4_DATA5;
#[doc = "`read()` method returns [rd_key4_data5::R](rd_key4_data5::R) reader structure"]
impl crate::Readable for RD_KEY4_DATA5 {}
#[doc = "EFUSE_RD_KEY4_DATA5"]
pub mod rd_key4_data5;
#[doc = "EFUSE_RD_KEY4_DATA6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key4_data6](rd_key4_data6) module"]
pub type RD_KEY4_DATA6 = crate::Reg<u32, _RD_KEY4_DATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY4_DATA6;
#[doc = "`read()` method returns [rd_key4_data6::R](rd_key4_data6::R) reader structure"]
impl crate::Readable for RD_KEY4_DATA6 {}
#[doc = "EFUSE_RD_KEY4_DATA6"]
pub mod rd_key4_data6;
#[doc = "EFUSE_RD_KEY4_DATA7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key4_data7](rd_key4_data7) module"]
pub type RD_KEY4_DATA7 = crate::Reg<u32, _RD_KEY4_DATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY4_DATA7;
#[doc = "`read()` method returns [rd_key4_data7::R](rd_key4_data7::R) reader structure"]
impl crate::Readable for RD_KEY4_DATA7 {}
#[doc = "EFUSE_RD_KEY4_DATA7"]
pub mod rd_key4_data7;
#[doc = "EFUSE_RD_KEY5_DATA0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key5_data0](rd_key5_data0) module"]
pub type RD_KEY5_DATA0 = crate::Reg<u32, _RD_KEY5_DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY5_DATA0;
#[doc = "`read()` method returns [rd_key5_data0::R](rd_key5_data0::R) reader structure"]
impl crate::Readable for RD_KEY5_DATA0 {}
#[doc = "EFUSE_RD_KEY5_DATA0"]
pub mod rd_key5_data0;
#[doc = "EFUSE_RD_KEY5_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key5_data1](rd_key5_data1) module"]
pub type RD_KEY5_DATA1 = crate::Reg<u32, _RD_KEY5_DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY5_DATA1;
#[doc = "`read()` method returns [rd_key5_data1::R](rd_key5_data1::R) reader structure"]
impl crate::Readable for RD_KEY5_DATA1 {}
#[doc = "EFUSE_RD_KEY5_DATA1"]
pub mod rd_key5_data1;
#[doc = "EFUSE_RD_KEY5_DATA2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key5_data2](rd_key5_data2) module"]
pub type RD_KEY5_DATA2 = crate::Reg<u32, _RD_KEY5_DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY5_DATA2;
#[doc = "`read()` method returns [rd_key5_data2::R](rd_key5_data2::R) reader structure"]
impl crate::Readable for RD_KEY5_DATA2 {}
#[doc = "EFUSE_RD_KEY5_DATA2"]
pub mod rd_key5_data2;
#[doc = "EFUSE_RD_KEY5_DATA3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key5_data3](rd_key5_data3) module"]
pub type RD_KEY5_DATA3 = crate::Reg<u32, _RD_KEY5_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY5_DATA3;
#[doc = "`read()` method returns [rd_key5_data3::R](rd_key5_data3::R) reader structure"]
impl crate::Readable for RD_KEY5_DATA3 {}
#[doc = "EFUSE_RD_KEY5_DATA3"]
pub mod rd_key5_data3;
#[doc = "EFUSE_RD_KEY5_DATA4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key5_data4](rd_key5_data4) module"]
pub type RD_KEY5_DATA4 = crate::Reg<u32, _RD_KEY5_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY5_DATA4;
#[doc = "`read()` method returns [rd_key5_data4::R](rd_key5_data4::R) reader structure"]
impl crate::Readable for RD_KEY5_DATA4 {}
#[doc = "EFUSE_RD_KEY5_DATA4"]
pub mod rd_key5_data4;
#[doc = "EFUSE_RD_KEY5_DATA5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key5_data5](rd_key5_data5) module"]
pub type RD_KEY5_DATA5 = crate::Reg<u32, _RD_KEY5_DATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY5_DATA5;
#[doc = "`read()` method returns [rd_key5_data5::R](rd_key5_data5::R) reader structure"]
impl crate::Readable for RD_KEY5_DATA5 {}
#[doc = "EFUSE_RD_KEY5_DATA5"]
pub mod rd_key5_data5;
#[doc = "EFUSE_RD_KEY5_DATA6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key5_data6](rd_key5_data6) module"]
pub type RD_KEY5_DATA6 = crate::Reg<u32, _RD_KEY5_DATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY5_DATA6;
#[doc = "`read()` method returns [rd_key5_data6::R](rd_key5_data6::R) reader structure"]
impl crate::Readable for RD_KEY5_DATA6 {}
#[doc = "EFUSE_RD_KEY5_DATA6"]
pub mod rd_key5_data6;
#[doc = "EFUSE_RD_KEY5_DATA7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key5_data7](rd_key5_data7) module"]
pub type RD_KEY5_DATA7 = crate::Reg<u32, _RD_KEY5_DATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_KEY5_DATA7;
#[doc = "`read()` method returns [rd_key5_data7::R](rd_key5_data7::R) reader structure"]
impl crate::Readable for RD_KEY5_DATA7 {}
#[doc = "EFUSE_RD_KEY5_DATA7"]
pub mod rd_key5_data7;
#[doc = "EFUSE_RD_SYS_PART2_DATA0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_sys_part2_data0](rd_sys_part2_data0) module"]
pub type RD_SYS_PART2_DATA0 = crate::Reg<u32, _RD_SYS_PART2_DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_SYS_PART2_DATA0;
#[doc = "`read()` method returns [rd_sys_part2_data0::R](rd_sys_part2_data0::R) reader structure"]
impl crate::Readable for RD_SYS_PART2_DATA0 {}
#[doc = "EFUSE_RD_SYS_PART2_DATA0"]
pub mod rd_sys_part2_data0;
#[doc = "EFUSE_RD_SYS_PART2_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_sys_part2_data1](rd_sys_part2_data1) module"]
pub type RD_SYS_PART2_DATA1 = crate::Reg<u32, _RD_SYS_PART2_DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_SYS_PART2_DATA1;
#[doc = "`read()` method returns [rd_sys_part2_data1::R](rd_sys_part2_data1::R) reader structure"]
impl crate::Readable for RD_SYS_PART2_DATA1 {}
#[doc = "EFUSE_RD_SYS_PART2_DATA1"]
pub mod rd_sys_part2_data1;
#[doc = "EFUSE_RD_SYS_PART2_DATA2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_sys_part2_data2](rd_sys_part2_data2) module"]
pub type RD_SYS_PART2_DATA2 = crate::Reg<u32, _RD_SYS_PART2_DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_SYS_PART2_DATA2;
#[doc = "`read()` method returns [rd_sys_part2_data2::R](rd_sys_part2_data2::R) reader structure"]
impl crate::Readable for RD_SYS_PART2_DATA2 {}
#[doc = "EFUSE_RD_SYS_PART2_DATA2"]
pub mod rd_sys_part2_data2;
#[doc = "EFUSE_RD_SYS_PART2_DATA3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_sys_part2_data3](rd_sys_part2_data3) module"]
pub type RD_SYS_PART2_DATA3 = crate::Reg<u32, _RD_SYS_PART2_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_SYS_PART2_DATA3;
#[doc = "`read()` method returns [rd_sys_part2_data3::R](rd_sys_part2_data3::R) reader structure"]
impl crate::Readable for RD_SYS_PART2_DATA3 {}
#[doc = "EFUSE_RD_SYS_PART2_DATA3"]
pub mod rd_sys_part2_data3;
#[doc = "EFUSE_RD_SYS_PART2_DATA4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_sys_part2_data4](rd_sys_part2_data4) module"]
pub type RD_SYS_PART2_DATA4 = crate::Reg<u32, _RD_SYS_PART2_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_SYS_PART2_DATA4;
#[doc = "`read()` method returns [rd_sys_part2_data4::R](rd_sys_part2_data4::R) reader structure"]
impl crate::Readable for RD_SYS_PART2_DATA4 {}
#[doc = "EFUSE_RD_SYS_PART2_DATA4"]
pub mod rd_sys_part2_data4;
#[doc = "EFUSE_RD_SYS_PART2_DATA5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_sys_part2_data5](rd_sys_part2_data5) module"]
pub type RD_SYS_PART2_DATA5 = crate::Reg<u32, _RD_SYS_PART2_DATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_SYS_PART2_DATA5;
#[doc = "`read()` method returns [rd_sys_part2_data5::R](rd_sys_part2_data5::R) reader structure"]
impl crate::Readable for RD_SYS_PART2_DATA5 {}
#[doc = "EFUSE_RD_SYS_PART2_DATA5"]
pub mod rd_sys_part2_data5;
#[doc = "EFUSE_RD_SYS_PART2_DATA6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_sys_part2_data6](rd_sys_part2_data6) module"]
pub type RD_SYS_PART2_DATA6 = crate::Reg<u32, _RD_SYS_PART2_DATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_SYS_PART2_DATA6;
#[doc = "`read()` method returns [rd_sys_part2_data6::R](rd_sys_part2_data6::R) reader structure"]
impl crate::Readable for RD_SYS_PART2_DATA6 {}
#[doc = "EFUSE_RD_SYS_PART2_DATA6"]
pub mod rd_sys_part2_data6;
#[doc = "EFUSE_RD_SYS_PART2_DATA7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_sys_part2_data7](rd_sys_part2_data7) module"]
pub type RD_SYS_PART2_DATA7 = crate::Reg<u32, _RD_SYS_PART2_DATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_SYS_PART2_DATA7;
#[doc = "`read()` method returns [rd_sys_part2_data7::R](rd_sys_part2_data7::R) reader structure"]
impl crate::Readable for RD_SYS_PART2_DATA7 {}
#[doc = "EFUSE_RD_SYS_PART2_DATA7"]
pub mod rd_sys_part2_data7;
#[doc = "EFUSE_RD_REPEAT_ERR0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_err0](rd_repeat_err0) module"]
pub type RD_REPEAT_ERR0 = crate::Reg<u32, _RD_REPEAT_ERR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_REPEAT_ERR0;
#[doc = "`read()` method returns [rd_repeat_err0::R](rd_repeat_err0::R) reader structure"]
impl crate::Readable for RD_REPEAT_ERR0 {}
#[doc = "EFUSE_RD_REPEAT_ERR0"]
pub mod rd_repeat_err0;
#[doc = "EFUSE_RD_REPEAT_ERR1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_err1](rd_repeat_err1) module"]
pub type RD_REPEAT_ERR1 = crate::Reg<u32, _RD_REPEAT_ERR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_REPEAT_ERR1;
#[doc = "`read()` method returns [rd_repeat_err1::R](rd_repeat_err1::R) reader structure"]
impl crate::Readable for RD_REPEAT_ERR1 {}
#[doc = "EFUSE_RD_REPEAT_ERR1"]
pub mod rd_repeat_err1;
#[doc = "EFUSE_RD_REPEAT_ERR2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_err2](rd_repeat_err2) module"]
pub type RD_REPEAT_ERR2 = crate::Reg<u32, _RD_REPEAT_ERR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_REPEAT_ERR2;
#[doc = "`read()` method returns [rd_repeat_err2::R](rd_repeat_err2::R) reader structure"]
impl crate::Readable for RD_REPEAT_ERR2 {}
#[doc = "EFUSE_RD_REPEAT_ERR2"]
pub mod rd_repeat_err2;
#[doc = "EFUSE_RD_REPEAT_ERR3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_err3](rd_repeat_err3) module"]
pub type RD_REPEAT_ERR3 = crate::Reg<u32, _RD_REPEAT_ERR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_REPEAT_ERR3;
#[doc = "`read()` method returns [rd_repeat_err3::R](rd_repeat_err3::R) reader structure"]
impl crate::Readable for RD_REPEAT_ERR3 {}
#[doc = "EFUSE_RD_REPEAT_ERR3"]
pub mod rd_repeat_err3;
#[doc = "EFUSE_RD_REPEAT_ERR4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_err4](rd_repeat_err4) module"]
pub type RD_REPEAT_ERR4 = crate::Reg<u32, _RD_REPEAT_ERR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_REPEAT_ERR4;
#[doc = "`read()` method returns [rd_repeat_err4::R](rd_repeat_err4::R) reader structure"]
impl crate::Readable for RD_REPEAT_ERR4 {}
#[doc = "EFUSE_RD_REPEAT_ERR4"]
pub mod rd_repeat_err4;
#[doc = "EFUSE_RD_RS_ERR0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_rs_err0](rd_rs_err0) module"]
pub type RD_RS_ERR0 = crate::Reg<u32, _RD_RS_ERR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_RS_ERR0;
#[doc = "`read()` method returns [rd_rs_err0::R](rd_rs_err0::R) reader structure"]
impl crate::Readable for RD_RS_ERR0 {}
#[doc = "EFUSE_RD_RS_ERR0"]
pub mod rd_rs_err0;
#[doc = "EFUSE_RD_RS_ERR1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_rs_err1](rd_rs_err1) module"]
pub type RD_RS_ERR1 = crate::Reg<u32, _RD_RS_ERR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_RS_ERR1;
#[doc = "`read()` method returns [rd_rs_err1::R](rd_rs_err1::R) reader structure"]
impl crate::Readable for RD_RS_ERR1 {}
#[doc = "EFUSE_RD_RS_ERR1"]
pub mod rd_rs_err1;
#[doc = "EFUSE_CLK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk](clk) module"]
pub type CLK = crate::Reg<u32, _CLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK;
#[doc = "`read()` method returns [clk::R](clk::R) reader structure"]
impl crate::Readable for CLK {}
#[doc = "`write(|w| ..)` method takes [clk::W](clk::W) writer structure"]
impl crate::Writable for CLK {}
#[doc = "EFUSE_CLK"]
pub mod clk;
#[doc = "EFUSE_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf](conf) module"]
pub type CONF = crate::Reg<u32, _CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONF;
#[doc = "`read()` method returns [conf::R](conf::R) reader structure"]
impl crate::Readable for CONF {}
#[doc = "`write(|w| ..)` method takes [conf::W](conf::W) writer structure"]
impl crate::Writable for CONF {}
#[doc = "EFUSE_CONF"]
pub mod conf;
#[doc = "EFUSE_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "EFUSE_STATUS"]
pub mod status;
#[doc = "EFUSE_CMD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`read()` method returns [cmd::R](cmd::R) reader structure"]
impl crate::Readable for CMD {}
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "EFUSE_CMD"]
pub mod cmd;
#[doc = "EFUSE_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](int_raw) module"]
pub type INT_RAW = crate::Reg<u32, _INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_RAW;
#[doc = "`read()` method returns [int_raw::R](int_raw::R) reader structure"]
impl crate::Readable for INT_RAW {}
#[doc = "EFUSE_INT_RAW"]
pub mod int_raw;
#[doc = "EFUSE_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](int_st) module"]
pub type INT_ST = crate::Reg<u32, _INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ST;
#[doc = "`read()` method returns [int_st::R](int_st::R) reader structure"]
impl crate::Readable for INT_ST {}
#[doc = "EFUSE_INT_ST"]
pub mod int_st;
#[doc = "EFUSE_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](int_ena) module"]
pub type INT_ENA = crate::Reg<u32, _INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ENA;
#[doc = "`read()` method returns [int_ena::R](int_ena::R) reader structure"]
impl crate::Readable for INT_ENA {}
#[doc = "`write(|w| ..)` method takes [int_ena::W](int_ena::W) writer structure"]
impl crate::Writable for INT_ENA {}
#[doc = "EFUSE_INT_ENA"]
pub mod int_ena;
#[doc = "EFUSE_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](int_clr) module"]
pub type INT_CLR = crate::Reg<u32, _INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CLR;
#[doc = "`write(|w| ..)` method takes [int_clr::W](int_clr::W) writer structure"]
impl crate::Writable for INT_CLR {}
#[doc = "EFUSE_INT_CLR"]
pub mod int_clr;
#[doc = "EFUSE_DAC_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_conf](dac_conf) module"]
pub type DAC_CONF = crate::Reg<u32, _DAC_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_CONF;
#[doc = "`read()` method returns [dac_conf::R](dac_conf::R) reader structure"]
impl crate::Readable for DAC_CONF {}
#[doc = "`write(|w| ..)` method takes [dac_conf::W](dac_conf::W) writer structure"]
impl crate::Writable for DAC_CONF {}
#[doc = "EFUSE_DAC_CONF"]
pub mod dac_conf;
#[doc = "EFUSE_RD_TIM_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_tim_conf](rd_tim_conf) module"]
pub type RD_TIM_CONF = crate::Reg<u32, _RD_TIM_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_TIM_CONF;
#[doc = "`read()` method returns [rd_tim_conf::R](rd_tim_conf::R) reader structure"]
impl crate::Readable for RD_TIM_CONF {}
#[doc = "`write(|w| ..)` method takes [rd_tim_conf::W](rd_tim_conf::W) writer structure"]
impl crate::Writable for RD_TIM_CONF {}
#[doc = "EFUSE_RD_TIM_CONF"]
pub mod rd_tim_conf;
#[doc = "EFUSE_WR_TIM_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_tim_conf1](wr_tim_conf1) module"]
pub type WR_TIM_CONF1 = crate::Reg<u32, _WR_TIM_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WR_TIM_CONF1;
#[doc = "`read()` method returns [wr_tim_conf1::R](wr_tim_conf1::R) reader structure"]
impl crate::Readable for WR_TIM_CONF1 {}
#[doc = "`write(|w| ..)` method takes [wr_tim_conf1::W](wr_tim_conf1::W) writer structure"]
impl crate::Writable for WR_TIM_CONF1 {}
#[doc = "EFUSE_WR_TIM_CONF1"]
pub mod wr_tim_conf1;
#[doc = "EFUSE_WR_TIM_CONF2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_tim_conf2](wr_tim_conf2) module"]
pub type WR_TIM_CONF2 = crate::Reg<u32, _WR_TIM_CONF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WR_TIM_CONF2;
#[doc = "`read()` method returns [wr_tim_conf2::R](wr_tim_conf2::R) reader structure"]
impl crate::Readable for WR_TIM_CONF2 {}
#[doc = "`write(|w| ..)` method takes [wr_tim_conf2::W](wr_tim_conf2::W) writer structure"]
impl crate::Writable for WR_TIM_CONF2 {}
#[doc = "EFUSE_WR_TIM_CONF2"]
pub mod wr_tim_conf2;
#[doc = "EFUSE_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "EFUSE_DATE"]
pub mod date;
