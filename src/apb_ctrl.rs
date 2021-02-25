#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - APB_CTRL_SYSCLK_CONF"]
    pub sysclk_conf: SYSCLK_CONF,
    #[doc = "0x04 - APB_CTRL_TICK_CONF"]
    pub tick_conf: TICK_CONF,
    #[doc = "0x08 - APB_CTRL_CLK_OUT_EN"]
    pub clk_out_en: CLK_OUT_EN,
    #[doc = "0x0c - APB_CTRL_WIFI_BB_CFG"]
    pub wifi_bb_cfg: WIFI_BB_CFG,
    #[doc = "0x10 - APB_CTRL_WIFI_BB_CFG_2"]
    pub wifi_bb_cfg_2: WIFI_BB_CFG_2,
    #[doc = "0x14 - APB_CTRL_WIFI_CLK_EN"]
    pub wifi_clk_en: WIFI_CLK_EN,
    #[doc = "0x18 - APB_CTRL_WIFI_RST_EN"]
    pub wifi_rst_en: WIFI_RST_EN,
    #[doc = "0x1c - APB_CTRL_HOST_INF_SEL"]
    pub host_inf_sel: HOST_INF_SEL,
    #[doc = "0x20 - APB_CTRL_EXT_MEM_PMS_LOCK"]
    pub ext_mem_pms_lock: EXT_MEM_PMS_LOCK,
    _reserved9: [u8; 4usize],
    #[doc = "0x28 - APB_CTRL_FLASH_ACE0_ATTR"]
    pub flash_ace0_attr: FLASH_ACE0_ATTR,
    #[doc = "0x2c - APB_CTRL_FLASH_ACE1_ATTR"]
    pub flash_ace1_attr: FLASH_ACE1_ATTR,
    #[doc = "0x30 - APB_CTRL_FLASH_ACE2_ATTR"]
    pub flash_ace2_attr: FLASH_ACE2_ATTR,
    #[doc = "0x34 - APB_CTRL_FLASH_ACE3_ATTR"]
    pub flash_ace3_attr: FLASH_ACE3_ATTR,
    #[doc = "0x38 - APB_CTRL_FLASH_ACE0_ADDR"]
    pub flash_ace0_addr: FLASH_ACE0_ADDR,
    #[doc = "0x3c - APB_CTRL_FLASH_ACE1_ADDR"]
    pub flash_ace1_addr: FLASH_ACE1_ADDR,
    #[doc = "0x40 - APB_CTRL_FLASH_ACE2_ADDR"]
    pub flash_ace2_addr: FLASH_ACE2_ADDR,
    #[doc = "0x44 - APB_CTRL_FLASH_ACE3_ADDR"]
    pub flash_ace3_addr: FLASH_ACE3_ADDR,
    #[doc = "0x48 - APB_CTRL_FLASH_ACE0_SIZE"]
    pub flash_ace0_size: FLASH_ACE0_SIZE,
    #[doc = "0x4c - APB_CTRL_FLASH_ACE1_SIZE"]
    pub flash_ace1_size: FLASH_ACE1_SIZE,
    #[doc = "0x50 - APB_CTRL_FLASH_ACE2_SIZE"]
    pub flash_ace2_size: FLASH_ACE2_SIZE,
    #[doc = "0x54 - APB_CTRL_FLASH_ACE3_SIZE"]
    pub flash_ace3_size: FLASH_ACE3_SIZE,
    _reserved21: [u8; 48usize],
    #[doc = "0x88 - APB_CTRL_SPI_MEM_PMS_CTRL"]
    pub spi_mem_pms_ctrl: SPI_MEM_PMS_CTRL,
    #[doc = "0x8c - APB_CTRL_SPI_MEM_REJECT_ADDR"]
    pub spi_mem_reject_addr: SPI_MEM_REJECT_ADDR,
    #[doc = "0x90 - APB_CTRL_SDIO_CTRL"]
    pub sdio_ctrl: SDIO_CTRL,
    #[doc = "0x94 - APB_CTRL_REDCY_SIG0"]
    pub redcy_sig0: REDCY_SIG0,
    #[doc = "0x98 - APB_CTRL_REDCY_SIG1"]
    pub redcy_sig1: REDCY_SIG1,
    #[doc = "0x9c - APB_CTRL_FRONT_END_MEM_PD"]
    pub front_end_mem_pd: FRONT_END_MEM_PD,
    #[doc = "0xa0 - APB_CTRL_RETENTION_CTRL"]
    pub retention_ctrl: RETENTION_CTRL,
    #[doc = "0xa4 - APB_CTRL_CLKGATE_FORCE_ON"]
    pub clkgate_force_on: CLKGATE_FORCE_ON,
    #[doc = "0xa8 - APB_CTRL_MEM_POWER_DOWN"]
    pub mem_power_down: MEM_POWER_DOWN,
    #[doc = "0xac - APB_CTRL_MEM_POWER_UP"]
    pub mem_power_up: MEM_POWER_UP,
    #[doc = "0xb0 - APB_CTRL_RND_DATA"]
    pub rnd_data: RND_DATA,
    #[doc = "0xb4 - APB_CTRL_PERI_BACKUP_CONFIG"]
    pub peri_backup_config: PERI_BACKUP_CONFIG,
    #[doc = "0xb8 - APB_CTRL_PERI_BACKUP_APB_ADDR"]
    pub peri_backup_apb_addr: PERI_BACKUP_APB_ADDR,
    #[doc = "0xbc - APB_CTRL_PERI_BACKUP_MEM_ADDR"]
    pub peri_backup_mem_addr: PERI_BACKUP_MEM_ADDR,
    #[doc = "0xc0 - APB_CTRL_PERI_BACKUP_INT_RAW"]
    pub peri_backup_int_raw: PERI_BACKUP_INT_RAW,
    #[doc = "0xc4 - APB_CTRL_PERI_BACKUP_INT_ST"]
    pub peri_backup_int_st: PERI_BACKUP_INT_ST,
    #[doc = "0xc8 - APB_CTRL_PERI_BACKUP_INT_ENA"]
    pub peri_backup_int_ena: PERI_BACKUP_INT_ENA,
    _reserved38: [u8; 4usize],
    #[doc = "0xd0 - APB_CTRL_PERI_BACKUP_INT_CLR"]
    pub peri_backup_int_clr: PERI_BACKUP_INT_CLR,
    _reserved39: [u8; 808usize],
    #[doc = "0x3fc - APB_CTRL_DATE"]
    pub date: DATE,
}
#[doc = "APB_CTRL_SYSCLK_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysclk_conf](sysclk_conf) module"]
pub type SYSCLK_CONF = crate::Reg<u32, _SYSCLK_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCLK_CONF;
#[doc = "`read()` method returns [sysclk_conf::R](sysclk_conf::R) reader structure"]
impl crate::Readable for SYSCLK_CONF {}
#[doc = "`write(|w| ..)` method takes [sysclk_conf::W](sysclk_conf::W) writer structure"]
impl crate::Writable for SYSCLK_CONF {}
#[doc = "APB_CTRL_SYSCLK_CONF"]
pub mod sysclk_conf;
#[doc = "APB_CTRL_TICK_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tick_conf](tick_conf) module"]
pub type TICK_CONF = crate::Reg<u32, _TICK_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TICK_CONF;
#[doc = "`read()` method returns [tick_conf::R](tick_conf::R) reader structure"]
impl crate::Readable for TICK_CONF {}
#[doc = "`write(|w| ..)` method takes [tick_conf::W](tick_conf::W) writer structure"]
impl crate::Writable for TICK_CONF {}
#[doc = "APB_CTRL_TICK_CONF"]
pub mod tick_conf;
#[doc = "APB_CTRL_CLK_OUT_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_out_en](clk_out_en) module"]
pub type CLK_OUT_EN = crate::Reg<u32, _CLK_OUT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_OUT_EN;
#[doc = "`read()` method returns [clk_out_en::R](clk_out_en::R) reader structure"]
impl crate::Readable for CLK_OUT_EN {}
#[doc = "`write(|w| ..)` method takes [clk_out_en::W](clk_out_en::W) writer structure"]
impl crate::Writable for CLK_OUT_EN {}
#[doc = "APB_CTRL_CLK_OUT_EN"]
pub mod clk_out_en;
#[doc = "APB_CTRL_WIFI_BB_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_bb_cfg](wifi_bb_cfg) module"]
pub type WIFI_BB_CFG = crate::Reg<u32, _WIFI_BB_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WIFI_BB_CFG;
#[doc = "`read()` method returns [wifi_bb_cfg::R](wifi_bb_cfg::R) reader structure"]
impl crate::Readable for WIFI_BB_CFG {}
#[doc = "`write(|w| ..)` method takes [wifi_bb_cfg::W](wifi_bb_cfg::W) writer structure"]
impl crate::Writable for WIFI_BB_CFG {}
#[doc = "APB_CTRL_WIFI_BB_CFG"]
pub mod wifi_bb_cfg;
#[doc = "APB_CTRL_WIFI_BB_CFG_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_bb_cfg_2](wifi_bb_cfg_2) module"]
pub type WIFI_BB_CFG_2 = crate::Reg<u32, _WIFI_BB_CFG_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WIFI_BB_CFG_2;
#[doc = "`read()` method returns [wifi_bb_cfg_2::R](wifi_bb_cfg_2::R) reader structure"]
impl crate::Readable for WIFI_BB_CFG_2 {}
#[doc = "`write(|w| ..)` method takes [wifi_bb_cfg_2::W](wifi_bb_cfg_2::W) writer structure"]
impl crate::Writable for WIFI_BB_CFG_2 {}
#[doc = "APB_CTRL_WIFI_BB_CFG_2"]
pub mod wifi_bb_cfg_2;
#[doc = "APB_CTRL_WIFI_CLK_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_clk_en](wifi_clk_en) module"]
pub type WIFI_CLK_EN = crate::Reg<u32, _WIFI_CLK_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WIFI_CLK_EN;
#[doc = "`read()` method returns [wifi_clk_en::R](wifi_clk_en::R) reader structure"]
impl crate::Readable for WIFI_CLK_EN {}
#[doc = "`write(|w| ..)` method takes [wifi_clk_en::W](wifi_clk_en::W) writer structure"]
impl crate::Writable for WIFI_CLK_EN {}
#[doc = "APB_CTRL_WIFI_CLK_EN"]
pub mod wifi_clk_en;
#[doc = "APB_CTRL_WIFI_RST_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_rst_en](wifi_rst_en) module"]
pub type WIFI_RST_EN = crate::Reg<u32, _WIFI_RST_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WIFI_RST_EN;
#[doc = "`read()` method returns [wifi_rst_en::R](wifi_rst_en::R) reader structure"]
impl crate::Readable for WIFI_RST_EN {}
#[doc = "`write(|w| ..)` method takes [wifi_rst_en::W](wifi_rst_en::W) writer structure"]
impl crate::Writable for WIFI_RST_EN {}
#[doc = "APB_CTRL_WIFI_RST_EN"]
pub mod wifi_rst_en;
#[doc = "APB_CTRL_HOST_INF_SEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_inf_sel](host_inf_sel) module"]
pub type HOST_INF_SEL = crate::Reg<u32, _HOST_INF_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_INF_SEL;
#[doc = "`read()` method returns [host_inf_sel::R](host_inf_sel::R) reader structure"]
impl crate::Readable for HOST_INF_SEL {}
#[doc = "`write(|w| ..)` method takes [host_inf_sel::W](host_inf_sel::W) writer structure"]
impl crate::Writable for HOST_INF_SEL {}
#[doc = "APB_CTRL_HOST_INF_SEL"]
pub mod host_inf_sel;
#[doc = "APB_CTRL_EXT_MEM_PMS_LOCK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_mem_pms_lock](ext_mem_pms_lock) module"]
pub type EXT_MEM_PMS_LOCK = crate::Reg<u32, _EXT_MEM_PMS_LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_MEM_PMS_LOCK;
#[doc = "`read()` method returns [ext_mem_pms_lock::R](ext_mem_pms_lock::R) reader structure"]
impl crate::Readable for EXT_MEM_PMS_LOCK {}
#[doc = "`write(|w| ..)` method takes [ext_mem_pms_lock::W](ext_mem_pms_lock::W) writer structure"]
impl crate::Writable for EXT_MEM_PMS_LOCK {}
#[doc = "APB_CTRL_EXT_MEM_PMS_LOCK"]
pub mod ext_mem_pms_lock;
#[doc = "APB_CTRL_FLASH_ACE0_ATTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ace0_attr](flash_ace0_attr) module"]
pub type FLASH_ACE0_ATTR = crate::Reg<u32, _FLASH_ACE0_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_ACE0_ATTR;
#[doc = "`read()` method returns [flash_ace0_attr::R](flash_ace0_attr::R) reader structure"]
impl crate::Readable for FLASH_ACE0_ATTR {}
#[doc = "`write(|w| ..)` method takes [flash_ace0_attr::W](flash_ace0_attr::W) writer structure"]
impl crate::Writable for FLASH_ACE0_ATTR {}
#[doc = "APB_CTRL_FLASH_ACE0_ATTR"]
pub mod flash_ace0_attr;
#[doc = "APB_CTRL_FLASH_ACE1_ATTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ace1_attr](flash_ace1_attr) module"]
pub type FLASH_ACE1_ATTR = crate::Reg<u32, _FLASH_ACE1_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_ACE1_ATTR;
#[doc = "`read()` method returns [flash_ace1_attr::R](flash_ace1_attr::R) reader structure"]
impl crate::Readable for FLASH_ACE1_ATTR {}
#[doc = "`write(|w| ..)` method takes [flash_ace1_attr::W](flash_ace1_attr::W) writer structure"]
impl crate::Writable for FLASH_ACE1_ATTR {}
#[doc = "APB_CTRL_FLASH_ACE1_ATTR"]
pub mod flash_ace1_attr;
#[doc = "APB_CTRL_FLASH_ACE2_ATTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ace2_attr](flash_ace2_attr) module"]
pub type FLASH_ACE2_ATTR = crate::Reg<u32, _FLASH_ACE2_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_ACE2_ATTR;
#[doc = "`read()` method returns [flash_ace2_attr::R](flash_ace2_attr::R) reader structure"]
impl crate::Readable for FLASH_ACE2_ATTR {}
#[doc = "`write(|w| ..)` method takes [flash_ace2_attr::W](flash_ace2_attr::W) writer structure"]
impl crate::Writable for FLASH_ACE2_ATTR {}
#[doc = "APB_CTRL_FLASH_ACE2_ATTR"]
pub mod flash_ace2_attr;
#[doc = "APB_CTRL_FLASH_ACE3_ATTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ace3_attr](flash_ace3_attr) module"]
pub type FLASH_ACE3_ATTR = crate::Reg<u32, _FLASH_ACE3_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_ACE3_ATTR;
#[doc = "`read()` method returns [flash_ace3_attr::R](flash_ace3_attr::R) reader structure"]
impl crate::Readable for FLASH_ACE3_ATTR {}
#[doc = "`write(|w| ..)` method takes [flash_ace3_attr::W](flash_ace3_attr::W) writer structure"]
impl crate::Writable for FLASH_ACE3_ATTR {}
#[doc = "APB_CTRL_FLASH_ACE3_ATTR"]
pub mod flash_ace3_attr;
#[doc = "APB_CTRL_FLASH_ACE0_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ace0_addr](flash_ace0_addr) module"]
pub type FLASH_ACE0_ADDR = crate::Reg<u32, _FLASH_ACE0_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_ACE0_ADDR;
#[doc = "`read()` method returns [flash_ace0_addr::R](flash_ace0_addr::R) reader structure"]
impl crate::Readable for FLASH_ACE0_ADDR {}
#[doc = "`write(|w| ..)` method takes [flash_ace0_addr::W](flash_ace0_addr::W) writer structure"]
impl crate::Writable for FLASH_ACE0_ADDR {}
#[doc = "APB_CTRL_FLASH_ACE0_ADDR"]
pub mod flash_ace0_addr;
#[doc = "APB_CTRL_FLASH_ACE1_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ace1_addr](flash_ace1_addr) module"]
pub type FLASH_ACE1_ADDR = crate::Reg<u32, _FLASH_ACE1_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_ACE1_ADDR;
#[doc = "`read()` method returns [flash_ace1_addr::R](flash_ace1_addr::R) reader structure"]
impl crate::Readable for FLASH_ACE1_ADDR {}
#[doc = "`write(|w| ..)` method takes [flash_ace1_addr::W](flash_ace1_addr::W) writer structure"]
impl crate::Writable for FLASH_ACE1_ADDR {}
#[doc = "APB_CTRL_FLASH_ACE1_ADDR"]
pub mod flash_ace1_addr;
#[doc = "APB_CTRL_FLASH_ACE2_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ace2_addr](flash_ace2_addr) module"]
pub type FLASH_ACE2_ADDR = crate::Reg<u32, _FLASH_ACE2_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_ACE2_ADDR;
#[doc = "`read()` method returns [flash_ace2_addr::R](flash_ace2_addr::R) reader structure"]
impl crate::Readable for FLASH_ACE2_ADDR {}
#[doc = "`write(|w| ..)` method takes [flash_ace2_addr::W](flash_ace2_addr::W) writer structure"]
impl crate::Writable for FLASH_ACE2_ADDR {}
#[doc = "APB_CTRL_FLASH_ACE2_ADDR"]
pub mod flash_ace2_addr;
#[doc = "APB_CTRL_FLASH_ACE3_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ace3_addr](flash_ace3_addr) module"]
pub type FLASH_ACE3_ADDR = crate::Reg<u32, _FLASH_ACE3_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_ACE3_ADDR;
#[doc = "`read()` method returns [flash_ace3_addr::R](flash_ace3_addr::R) reader structure"]
impl crate::Readable for FLASH_ACE3_ADDR {}
#[doc = "`write(|w| ..)` method takes [flash_ace3_addr::W](flash_ace3_addr::W) writer structure"]
impl crate::Writable for FLASH_ACE3_ADDR {}
#[doc = "APB_CTRL_FLASH_ACE3_ADDR"]
pub mod flash_ace3_addr;
#[doc = "APB_CTRL_FLASH_ACE0_SIZE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ace0_size](flash_ace0_size) module"]
pub type FLASH_ACE0_SIZE = crate::Reg<u32, _FLASH_ACE0_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_ACE0_SIZE;
#[doc = "`read()` method returns [flash_ace0_size::R](flash_ace0_size::R) reader structure"]
impl crate::Readable for FLASH_ACE0_SIZE {}
#[doc = "`write(|w| ..)` method takes [flash_ace0_size::W](flash_ace0_size::W) writer structure"]
impl crate::Writable for FLASH_ACE0_SIZE {}
#[doc = "APB_CTRL_FLASH_ACE0_SIZE"]
pub mod flash_ace0_size;
#[doc = "APB_CTRL_FLASH_ACE1_SIZE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ace1_size](flash_ace1_size) module"]
pub type FLASH_ACE1_SIZE = crate::Reg<u32, _FLASH_ACE1_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_ACE1_SIZE;
#[doc = "`read()` method returns [flash_ace1_size::R](flash_ace1_size::R) reader structure"]
impl crate::Readable for FLASH_ACE1_SIZE {}
#[doc = "`write(|w| ..)` method takes [flash_ace1_size::W](flash_ace1_size::W) writer structure"]
impl crate::Writable for FLASH_ACE1_SIZE {}
#[doc = "APB_CTRL_FLASH_ACE1_SIZE"]
pub mod flash_ace1_size;
#[doc = "APB_CTRL_FLASH_ACE2_SIZE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ace2_size](flash_ace2_size) module"]
pub type FLASH_ACE2_SIZE = crate::Reg<u32, _FLASH_ACE2_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_ACE2_SIZE;
#[doc = "`read()` method returns [flash_ace2_size::R](flash_ace2_size::R) reader structure"]
impl crate::Readable for FLASH_ACE2_SIZE {}
#[doc = "`write(|w| ..)` method takes [flash_ace2_size::W](flash_ace2_size::W) writer structure"]
impl crate::Writable for FLASH_ACE2_SIZE {}
#[doc = "APB_CTRL_FLASH_ACE2_SIZE"]
pub mod flash_ace2_size;
#[doc = "APB_CTRL_FLASH_ACE3_SIZE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ace3_size](flash_ace3_size) module"]
pub type FLASH_ACE3_SIZE = crate::Reg<u32, _FLASH_ACE3_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_ACE3_SIZE;
#[doc = "`read()` method returns [flash_ace3_size::R](flash_ace3_size::R) reader structure"]
impl crate::Readable for FLASH_ACE3_SIZE {}
#[doc = "`write(|w| ..)` method takes [flash_ace3_size::W](flash_ace3_size::W) writer structure"]
impl crate::Writable for FLASH_ACE3_SIZE {}
#[doc = "APB_CTRL_FLASH_ACE3_SIZE"]
pub mod flash_ace3_size;
#[doc = "APB_CTRL_SPI_MEM_PMS_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_pms_ctrl](spi_mem_pms_ctrl) module"]
pub type SPI_MEM_PMS_CTRL = crate::Reg<u32, _SPI_MEM_PMS_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_PMS_CTRL;
#[doc = "`read()` method returns [spi_mem_pms_ctrl::R](spi_mem_pms_ctrl::R) reader structure"]
impl crate::Readable for SPI_MEM_PMS_CTRL {}
#[doc = "`write(|w| ..)` method takes [spi_mem_pms_ctrl::W](spi_mem_pms_ctrl::W) writer structure"]
impl crate::Writable for SPI_MEM_PMS_CTRL {}
#[doc = "APB_CTRL_SPI_MEM_PMS_CTRL"]
pub mod spi_mem_pms_ctrl;
#[doc = "APB_CTRL_SPI_MEM_REJECT_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_reject_addr](spi_mem_reject_addr) module"]
pub type SPI_MEM_REJECT_ADDR = crate::Reg<u32, _SPI_MEM_REJECT_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MEM_REJECT_ADDR;
#[doc = "`read()` method returns [spi_mem_reject_addr::R](spi_mem_reject_addr::R) reader structure"]
impl crate::Readable for SPI_MEM_REJECT_ADDR {}
#[doc = "APB_CTRL_SPI_MEM_REJECT_ADDR"]
pub mod spi_mem_reject_addr;
#[doc = "APB_CTRL_SDIO_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_ctrl](sdio_ctrl) module"]
pub type SDIO_CTRL = crate::Reg<u32, _SDIO_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDIO_CTRL;
#[doc = "`read()` method returns [sdio_ctrl::R](sdio_ctrl::R) reader structure"]
impl crate::Readable for SDIO_CTRL {}
#[doc = "`write(|w| ..)` method takes [sdio_ctrl::W](sdio_ctrl::W) writer structure"]
impl crate::Writable for SDIO_CTRL {}
#[doc = "APB_CTRL_SDIO_CTRL"]
pub mod sdio_ctrl;
#[doc = "APB_CTRL_REDCY_SIG0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [redcy_sig0](redcy_sig0) module"]
pub type REDCY_SIG0 = crate::Reg<u32, _REDCY_SIG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REDCY_SIG0;
#[doc = "`read()` method returns [redcy_sig0::R](redcy_sig0::R) reader structure"]
impl crate::Readable for REDCY_SIG0 {}
#[doc = "`write(|w| ..)` method takes [redcy_sig0::W](redcy_sig0::W) writer structure"]
impl crate::Writable for REDCY_SIG0 {}
#[doc = "APB_CTRL_REDCY_SIG0"]
pub mod redcy_sig0;
#[doc = "APB_CTRL_REDCY_SIG1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [redcy_sig1](redcy_sig1) module"]
pub type REDCY_SIG1 = crate::Reg<u32, _REDCY_SIG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REDCY_SIG1;
#[doc = "`read()` method returns [redcy_sig1::R](redcy_sig1::R) reader structure"]
impl crate::Readable for REDCY_SIG1 {}
#[doc = "`write(|w| ..)` method takes [redcy_sig1::W](redcy_sig1::W) writer structure"]
impl crate::Writable for REDCY_SIG1 {}
#[doc = "APB_CTRL_REDCY_SIG1"]
pub mod redcy_sig1;
#[doc = "APB_CTRL_FRONT_END_MEM_PD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [front_end_mem_pd](front_end_mem_pd) module"]
pub type FRONT_END_MEM_PD = crate::Reg<u32, _FRONT_END_MEM_PD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRONT_END_MEM_PD;
#[doc = "`read()` method returns [front_end_mem_pd::R](front_end_mem_pd::R) reader structure"]
impl crate::Readable for FRONT_END_MEM_PD {}
#[doc = "`write(|w| ..)` method takes [front_end_mem_pd::W](front_end_mem_pd::W) writer structure"]
impl crate::Writable for FRONT_END_MEM_PD {}
#[doc = "APB_CTRL_FRONT_END_MEM_PD"]
pub mod front_end_mem_pd;
#[doc = "APB_CTRL_RETENTION_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retention_ctrl](retention_ctrl) module"]
pub type RETENTION_CTRL = crate::Reg<u32, _RETENTION_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RETENTION_CTRL;
#[doc = "`read()` method returns [retention_ctrl::R](retention_ctrl::R) reader structure"]
impl crate::Readable for RETENTION_CTRL {}
#[doc = "`write(|w| ..)` method takes [retention_ctrl::W](retention_ctrl::W) writer structure"]
impl crate::Writable for RETENTION_CTRL {}
#[doc = "APB_CTRL_RETENTION_CTRL"]
pub mod retention_ctrl;
#[doc = "APB_CTRL_CLKGATE_FORCE_ON\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkgate_force_on](clkgate_force_on) module"]
pub type CLKGATE_FORCE_ON = crate::Reg<u32, _CLKGATE_FORCE_ON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKGATE_FORCE_ON;
#[doc = "`read()` method returns [clkgate_force_on::R](clkgate_force_on::R) reader structure"]
impl crate::Readable for CLKGATE_FORCE_ON {}
#[doc = "`write(|w| ..)` method takes [clkgate_force_on::W](clkgate_force_on::W) writer structure"]
impl crate::Writable for CLKGATE_FORCE_ON {}
#[doc = "APB_CTRL_CLKGATE_FORCE_ON"]
pub mod clkgate_force_on;
#[doc = "APB_CTRL_MEM_POWER_DOWN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_power_down](mem_power_down) module"]
pub type MEM_POWER_DOWN = crate::Reg<u32, _MEM_POWER_DOWN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_POWER_DOWN;
#[doc = "`read()` method returns [mem_power_down::R](mem_power_down::R) reader structure"]
impl crate::Readable for MEM_POWER_DOWN {}
#[doc = "`write(|w| ..)` method takes [mem_power_down::W](mem_power_down::W) writer structure"]
impl crate::Writable for MEM_POWER_DOWN {}
#[doc = "APB_CTRL_MEM_POWER_DOWN"]
pub mod mem_power_down;
#[doc = "APB_CTRL_MEM_POWER_UP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_power_up](mem_power_up) module"]
pub type MEM_POWER_UP = crate::Reg<u32, _MEM_POWER_UP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_POWER_UP;
#[doc = "`read()` method returns [mem_power_up::R](mem_power_up::R) reader structure"]
impl crate::Readable for MEM_POWER_UP {}
#[doc = "`write(|w| ..)` method takes [mem_power_up::W](mem_power_up::W) writer structure"]
impl crate::Writable for MEM_POWER_UP {}
#[doc = "APB_CTRL_MEM_POWER_UP"]
pub mod mem_power_up;
#[doc = "APB_CTRL_RND_DATA\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rnd_data](rnd_data) module"]
pub type RND_DATA = crate::Reg<u32, _RND_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RND_DATA;
#[doc = "`read()` method returns [rnd_data::R](rnd_data::R) reader structure"]
impl crate::Readable for RND_DATA {}
#[doc = "APB_CTRL_RND_DATA"]
pub mod rnd_data;
#[doc = "APB_CTRL_PERI_BACKUP_CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peri_backup_config](peri_backup_config) module"]
pub type PERI_BACKUP_CONFIG = crate::Reg<u32, _PERI_BACKUP_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERI_BACKUP_CONFIG;
#[doc = "`read()` method returns [peri_backup_config::R](peri_backup_config::R) reader structure"]
impl crate::Readable for PERI_BACKUP_CONFIG {}
#[doc = "`write(|w| ..)` method takes [peri_backup_config::W](peri_backup_config::W) writer structure"]
impl crate::Writable for PERI_BACKUP_CONFIG {}
#[doc = "APB_CTRL_PERI_BACKUP_CONFIG"]
pub mod peri_backup_config;
#[doc = "APB_CTRL_PERI_BACKUP_APB_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peri_backup_apb_addr](peri_backup_apb_addr) module"]
pub type PERI_BACKUP_APB_ADDR = crate::Reg<u32, _PERI_BACKUP_APB_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERI_BACKUP_APB_ADDR;
#[doc = "`read()` method returns [peri_backup_apb_addr::R](peri_backup_apb_addr::R) reader structure"]
impl crate::Readable for PERI_BACKUP_APB_ADDR {}
#[doc = "`write(|w| ..)` method takes [peri_backup_apb_addr::W](peri_backup_apb_addr::W) writer structure"]
impl crate::Writable for PERI_BACKUP_APB_ADDR {}
#[doc = "APB_CTRL_PERI_BACKUP_APB_ADDR"]
pub mod peri_backup_apb_addr;
#[doc = "APB_CTRL_PERI_BACKUP_MEM_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peri_backup_mem_addr](peri_backup_mem_addr) module"]
pub type PERI_BACKUP_MEM_ADDR = crate::Reg<u32, _PERI_BACKUP_MEM_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERI_BACKUP_MEM_ADDR;
#[doc = "`read()` method returns [peri_backup_mem_addr::R](peri_backup_mem_addr::R) reader structure"]
impl crate::Readable for PERI_BACKUP_MEM_ADDR {}
#[doc = "`write(|w| ..)` method takes [peri_backup_mem_addr::W](peri_backup_mem_addr::W) writer structure"]
impl crate::Writable for PERI_BACKUP_MEM_ADDR {}
#[doc = "APB_CTRL_PERI_BACKUP_MEM_ADDR"]
pub mod peri_backup_mem_addr;
#[doc = "APB_CTRL_PERI_BACKUP_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peri_backup_int_raw](peri_backup_int_raw) module"]
pub type PERI_BACKUP_INT_RAW = crate::Reg<u32, _PERI_BACKUP_INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERI_BACKUP_INT_RAW;
#[doc = "`read()` method returns [peri_backup_int_raw::R](peri_backup_int_raw::R) reader structure"]
impl crate::Readable for PERI_BACKUP_INT_RAW {}
#[doc = "APB_CTRL_PERI_BACKUP_INT_RAW"]
pub mod peri_backup_int_raw;
#[doc = "APB_CTRL_PERI_BACKUP_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peri_backup_int_st](peri_backup_int_st) module"]
pub type PERI_BACKUP_INT_ST = crate::Reg<u32, _PERI_BACKUP_INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERI_BACKUP_INT_ST;
#[doc = "`read()` method returns [peri_backup_int_st::R](peri_backup_int_st::R) reader structure"]
impl crate::Readable for PERI_BACKUP_INT_ST {}
#[doc = "APB_CTRL_PERI_BACKUP_INT_ST"]
pub mod peri_backup_int_st;
#[doc = "APB_CTRL_PERI_BACKUP_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peri_backup_int_ena](peri_backup_int_ena) module"]
pub type PERI_BACKUP_INT_ENA = crate::Reg<u32, _PERI_BACKUP_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERI_BACKUP_INT_ENA;
#[doc = "`read()` method returns [peri_backup_int_ena::R](peri_backup_int_ena::R) reader structure"]
impl crate::Readable for PERI_BACKUP_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [peri_backup_int_ena::W](peri_backup_int_ena::W) writer structure"]
impl crate::Writable for PERI_BACKUP_INT_ENA {}
#[doc = "APB_CTRL_PERI_BACKUP_INT_ENA"]
pub mod peri_backup_int_ena;
#[doc = "APB_CTRL_PERI_BACKUP_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peri_backup_int_clr](peri_backup_int_clr) module"]
pub type PERI_BACKUP_INT_CLR = crate::Reg<u32, _PERI_BACKUP_INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERI_BACKUP_INT_CLR;
#[doc = "`write(|w| ..)` method takes [peri_backup_int_clr::W](peri_backup_int_clr::W) writer structure"]
impl crate::Writable for PERI_BACKUP_INT_CLR {}
#[doc = "APB_CTRL_PERI_BACKUP_INT_CLR"]
pub mod peri_backup_int_clr;
#[doc = "APB_CTRL_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "APB_CTRL_DATE"]
pub mod date;
