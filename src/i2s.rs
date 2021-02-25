#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 12usize],
    #[doc = "0x0c - I2S_INT_RAW"]
    pub int_raw: INT_RAW,
    #[doc = "0x10 - I2S_INT_ST"]
    pub int_st: INT_ST,
    #[doc = "0x14 - I2S_INT_ENA"]
    pub int_ena: INT_ENA,
    #[doc = "0x18 - I2S_INT_CLR"]
    pub int_clr: INT_CLR,
    _reserved4: [u8; 4usize],
    #[doc = "0x20 - I2S_RX_CONF"]
    pub rx_conf: RX_CONF,
    #[doc = "0x24 - I2S_TX_CONF"]
    pub tx_conf: TX_CONF,
    #[doc = "0x28 - I2S_RX_CONF1"]
    pub rx_conf1: RX_CONF1,
    #[doc = "0x2c - I2S_TX_CONF1"]
    pub tx_conf1: TX_CONF1,
    #[doc = "0x30 - I2S_RX_CLKM_CONF"]
    pub rx_clkm_conf: RX_CLKM_CONF,
    #[doc = "0x34 - I2S_TX_CLKM_CONF"]
    pub tx_clkm_conf: TX_CLKM_CONF,
    #[doc = "0x38 - I2S_RX_CLKM_DIV_CONF"]
    pub rx_clkm_div_conf: RX_CLKM_DIV_CONF,
    #[doc = "0x3c - I2S_TX_CLKM_DIV_CONF"]
    pub tx_clkm_div_conf: TX_CLKM_DIV_CONF,
    #[doc = "0x40 - I2S_TX_PCM2PDM_CONF"]
    pub tx_pcm2pdm_conf: TX_PCM2PDM_CONF,
    #[doc = "0x44 - I2S_TX_PCM2PDM_CONF1"]
    pub tx_pcm2pdm_conf1: TX_PCM2PDM_CONF1,
    _reserved14: [u8; 8usize],
    #[doc = "0x50 - I2S_RX_TDM_CTRL"]
    pub rx_tdm_ctrl: RX_TDM_CTRL,
    #[doc = "0x54 - I2S_TX_TDM_CTRL"]
    pub tx_tdm_ctrl: TX_TDM_CTRL,
    #[doc = "0x58 - I2S_RX_TIMING"]
    pub rx_timing: RX_TIMING,
    #[doc = "0x5c - I2S_TX_TIMING"]
    pub tx_timing: TX_TIMING,
    #[doc = "0x60 - I2S_LC_HUNG_CONF"]
    pub lc_hung_conf: LC_HUNG_CONF,
    #[doc = "0x64 - I2S_RXEOF_NUM"]
    pub rxeof_num: RXEOF_NUM,
    #[doc = "0x68 - I2S_CONF_SIGLE_DATA"]
    pub conf_sigle_data: CONF_SIGLE_DATA,
    #[doc = "0x6c - I2S_STATE"]
    pub state: STATE,
    _reserved22: [u8; 16usize],
    #[doc = "0x80 - I2S_DATE"]
    pub date: DATE,
}
#[doc = "I2S_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](int_raw) module"]
pub type INT_RAW = crate::Reg<u32, _INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_RAW;
#[doc = "`read()` method returns [int_raw::R](int_raw::R) reader structure"]
impl crate::Readable for INT_RAW {}
#[doc = "I2S_INT_RAW"]
pub mod int_raw;
#[doc = "I2S_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](int_st) module"]
pub type INT_ST = crate::Reg<u32, _INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ST;
#[doc = "`read()` method returns [int_st::R](int_st::R) reader structure"]
impl crate::Readable for INT_ST {}
#[doc = "I2S_INT_ST"]
pub mod int_st;
#[doc = "I2S_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](int_ena) module"]
pub type INT_ENA = crate::Reg<u32, _INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ENA;
#[doc = "`read()` method returns [int_ena::R](int_ena::R) reader structure"]
impl crate::Readable for INT_ENA {}
#[doc = "`write(|w| ..)` method takes [int_ena::W](int_ena::W) writer structure"]
impl crate::Writable for INT_ENA {}
#[doc = "I2S_INT_ENA"]
pub mod int_ena;
#[doc = "I2S_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](int_clr) module"]
pub type INT_CLR = crate::Reg<u32, _INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CLR;
#[doc = "`write(|w| ..)` method takes [int_clr::W](int_clr::W) writer structure"]
impl crate::Writable for INT_CLR {}
#[doc = "I2S_INT_CLR"]
pub mod int_clr;
#[doc = "I2S_RX_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_conf](rx_conf) module"]
pub type RX_CONF = crate::Reg<u32, _RX_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_CONF;
#[doc = "`read()` method returns [rx_conf::R](rx_conf::R) reader structure"]
impl crate::Readable for RX_CONF {}
#[doc = "`write(|w| ..)` method takes [rx_conf::W](rx_conf::W) writer structure"]
impl crate::Writable for RX_CONF {}
#[doc = "I2S_RX_CONF"]
pub mod rx_conf;
#[doc = "I2S_TX_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_conf](tx_conf) module"]
pub type TX_CONF = crate::Reg<u32, _TX_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_CONF;
#[doc = "`read()` method returns [tx_conf::R](tx_conf::R) reader structure"]
impl crate::Readable for TX_CONF {}
#[doc = "`write(|w| ..)` method takes [tx_conf::W](tx_conf::W) writer structure"]
impl crate::Writable for TX_CONF {}
#[doc = "I2S_TX_CONF"]
pub mod tx_conf;
#[doc = "I2S_RX_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_conf1](rx_conf1) module"]
pub type RX_CONF1 = crate::Reg<u32, _RX_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_CONF1;
#[doc = "`read()` method returns [rx_conf1::R](rx_conf1::R) reader structure"]
impl crate::Readable for RX_CONF1 {}
#[doc = "`write(|w| ..)` method takes [rx_conf1::W](rx_conf1::W) writer structure"]
impl crate::Writable for RX_CONF1 {}
#[doc = "I2S_RX_CONF1"]
pub mod rx_conf1;
#[doc = "I2S_TX_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_conf1](tx_conf1) module"]
pub type TX_CONF1 = crate::Reg<u32, _TX_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_CONF1;
#[doc = "`read()` method returns [tx_conf1::R](tx_conf1::R) reader structure"]
impl crate::Readable for TX_CONF1 {}
#[doc = "`write(|w| ..)` method takes [tx_conf1::W](tx_conf1::W) writer structure"]
impl crate::Writable for TX_CONF1 {}
#[doc = "I2S_TX_CONF1"]
pub mod tx_conf1;
#[doc = "I2S_RX_CLKM_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_clkm_conf](rx_clkm_conf) module"]
pub type RX_CLKM_CONF = crate::Reg<u32, _RX_CLKM_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_CLKM_CONF;
#[doc = "`read()` method returns [rx_clkm_conf::R](rx_clkm_conf::R) reader structure"]
impl crate::Readable for RX_CLKM_CONF {}
#[doc = "`write(|w| ..)` method takes [rx_clkm_conf::W](rx_clkm_conf::W) writer structure"]
impl crate::Writable for RX_CLKM_CONF {}
#[doc = "I2S_RX_CLKM_CONF"]
pub mod rx_clkm_conf;
#[doc = "I2S_TX_CLKM_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_clkm_conf](tx_clkm_conf) module"]
pub type TX_CLKM_CONF = crate::Reg<u32, _TX_CLKM_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_CLKM_CONF;
#[doc = "`read()` method returns [tx_clkm_conf::R](tx_clkm_conf::R) reader structure"]
impl crate::Readable for TX_CLKM_CONF {}
#[doc = "`write(|w| ..)` method takes [tx_clkm_conf::W](tx_clkm_conf::W) writer structure"]
impl crate::Writable for TX_CLKM_CONF {}
#[doc = "I2S_TX_CLKM_CONF"]
pub mod tx_clkm_conf;
#[doc = "I2S_RX_CLKM_DIV_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_clkm_div_conf](rx_clkm_div_conf) module"]
pub type RX_CLKM_DIV_CONF = crate::Reg<u32, _RX_CLKM_DIV_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_CLKM_DIV_CONF;
#[doc = "`read()` method returns [rx_clkm_div_conf::R](rx_clkm_div_conf::R) reader structure"]
impl crate::Readable for RX_CLKM_DIV_CONF {}
#[doc = "`write(|w| ..)` method takes [rx_clkm_div_conf::W](rx_clkm_div_conf::W) writer structure"]
impl crate::Writable for RX_CLKM_DIV_CONF {}
#[doc = "I2S_RX_CLKM_DIV_CONF"]
pub mod rx_clkm_div_conf;
#[doc = "I2S_TX_CLKM_DIV_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_clkm_div_conf](tx_clkm_div_conf) module"]
pub type TX_CLKM_DIV_CONF = crate::Reg<u32, _TX_CLKM_DIV_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_CLKM_DIV_CONF;
#[doc = "`read()` method returns [tx_clkm_div_conf::R](tx_clkm_div_conf::R) reader structure"]
impl crate::Readable for TX_CLKM_DIV_CONF {}
#[doc = "`write(|w| ..)` method takes [tx_clkm_div_conf::W](tx_clkm_div_conf::W) writer structure"]
impl crate::Writable for TX_CLKM_DIV_CONF {}
#[doc = "I2S_TX_CLKM_DIV_CONF"]
pub mod tx_clkm_div_conf;
#[doc = "I2S_TX_PCM2PDM_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_pcm2pdm_conf](tx_pcm2pdm_conf) module"]
pub type TX_PCM2PDM_CONF = crate::Reg<u32, _TX_PCM2PDM_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_PCM2PDM_CONF;
#[doc = "`read()` method returns [tx_pcm2pdm_conf::R](tx_pcm2pdm_conf::R) reader structure"]
impl crate::Readable for TX_PCM2PDM_CONF {}
#[doc = "`write(|w| ..)` method takes [tx_pcm2pdm_conf::W](tx_pcm2pdm_conf::W) writer structure"]
impl crate::Writable for TX_PCM2PDM_CONF {}
#[doc = "I2S_TX_PCM2PDM_CONF"]
pub mod tx_pcm2pdm_conf;
#[doc = "I2S_TX_PCM2PDM_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_pcm2pdm_conf1](tx_pcm2pdm_conf1) module"]
pub type TX_PCM2PDM_CONF1 = crate::Reg<u32, _TX_PCM2PDM_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_PCM2PDM_CONF1;
#[doc = "`read()` method returns [tx_pcm2pdm_conf1::R](tx_pcm2pdm_conf1::R) reader structure"]
impl crate::Readable for TX_PCM2PDM_CONF1 {}
#[doc = "`write(|w| ..)` method takes [tx_pcm2pdm_conf1::W](tx_pcm2pdm_conf1::W) writer structure"]
impl crate::Writable for TX_PCM2PDM_CONF1 {}
#[doc = "I2S_TX_PCM2PDM_CONF1"]
pub mod tx_pcm2pdm_conf1;
#[doc = "I2S_RX_TDM_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_tdm_ctrl](rx_tdm_ctrl) module"]
pub type RX_TDM_CTRL = crate::Reg<u32, _RX_TDM_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_TDM_CTRL;
#[doc = "`read()` method returns [rx_tdm_ctrl::R](rx_tdm_ctrl::R) reader structure"]
impl crate::Readable for RX_TDM_CTRL {}
#[doc = "`write(|w| ..)` method takes [rx_tdm_ctrl::W](rx_tdm_ctrl::W) writer structure"]
impl crate::Writable for RX_TDM_CTRL {}
#[doc = "I2S_RX_TDM_CTRL"]
pub mod rx_tdm_ctrl;
#[doc = "I2S_TX_TDM_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_tdm_ctrl](tx_tdm_ctrl) module"]
pub type TX_TDM_CTRL = crate::Reg<u32, _TX_TDM_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_TDM_CTRL;
#[doc = "`read()` method returns [tx_tdm_ctrl::R](tx_tdm_ctrl::R) reader structure"]
impl crate::Readable for TX_TDM_CTRL {}
#[doc = "`write(|w| ..)` method takes [tx_tdm_ctrl::W](tx_tdm_ctrl::W) writer structure"]
impl crate::Writable for TX_TDM_CTRL {}
#[doc = "I2S_TX_TDM_CTRL"]
pub mod tx_tdm_ctrl;
#[doc = "I2S_RX_TIMING\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_timing](rx_timing) module"]
pub type RX_TIMING = crate::Reg<u32, _RX_TIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_TIMING;
#[doc = "`read()` method returns [rx_timing::R](rx_timing::R) reader structure"]
impl crate::Readable for RX_TIMING {}
#[doc = "`write(|w| ..)` method takes [rx_timing::W](rx_timing::W) writer structure"]
impl crate::Writable for RX_TIMING {}
#[doc = "I2S_RX_TIMING"]
pub mod rx_timing;
#[doc = "I2S_TX_TIMING\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_timing](tx_timing) module"]
pub type TX_TIMING = crate::Reg<u32, _TX_TIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_TIMING;
#[doc = "`read()` method returns [tx_timing::R](tx_timing::R) reader structure"]
impl crate::Readable for TX_TIMING {}
#[doc = "`write(|w| ..)` method takes [tx_timing::W](tx_timing::W) writer structure"]
impl crate::Writable for TX_TIMING {}
#[doc = "I2S_TX_TIMING"]
pub mod tx_timing;
#[doc = "I2S_LC_HUNG_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lc_hung_conf](lc_hung_conf) module"]
pub type LC_HUNG_CONF = crate::Reg<u32, _LC_HUNG_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LC_HUNG_CONF;
#[doc = "`read()` method returns [lc_hung_conf::R](lc_hung_conf::R) reader structure"]
impl crate::Readable for LC_HUNG_CONF {}
#[doc = "`write(|w| ..)` method takes [lc_hung_conf::W](lc_hung_conf::W) writer structure"]
impl crate::Writable for LC_HUNG_CONF {}
#[doc = "I2S_LC_HUNG_CONF"]
pub mod lc_hung_conf;
#[doc = "I2S_RXEOF_NUM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxeof_num](rxeof_num) module"]
pub type RXEOF_NUM = crate::Reg<u32, _RXEOF_NUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXEOF_NUM;
#[doc = "`read()` method returns [rxeof_num::R](rxeof_num::R) reader structure"]
impl crate::Readable for RXEOF_NUM {}
#[doc = "`write(|w| ..)` method takes [rxeof_num::W](rxeof_num::W) writer structure"]
impl crate::Writable for RXEOF_NUM {}
#[doc = "I2S_RXEOF_NUM"]
pub mod rxeof_num;
#[doc = "I2S_CONF_SIGLE_DATA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf_sigle_data](conf_sigle_data) module"]
pub type CONF_SIGLE_DATA = crate::Reg<u32, _CONF_SIGLE_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONF_SIGLE_DATA;
#[doc = "`read()` method returns [conf_sigle_data::R](conf_sigle_data::R) reader structure"]
impl crate::Readable for CONF_SIGLE_DATA {}
#[doc = "`write(|w| ..)` method takes [conf_sigle_data::W](conf_sigle_data::W) writer structure"]
impl crate::Writable for CONF_SIGLE_DATA {}
#[doc = "I2S_CONF_SIGLE_DATA"]
pub mod conf_sigle_data;
#[doc = "I2S_STATE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state](state) module"]
pub type STATE = crate::Reg<u32, _STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATE;
#[doc = "`read()` method returns [state::R](state::R) reader structure"]
impl crate::Readable for STATE {}
#[doc = "I2S_STATE"]
pub mod state;
#[doc = "I2S_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "I2S_DATE"]
pub mod date;
