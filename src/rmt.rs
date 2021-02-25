#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - RMT_CH0CONF0"]
    pub ch0conf0: CH0CONF0,
    #[doc = "0x14 - RMT_CH1CONF0"]
    pub ch1conf0: CH1CONF0,
    #[doc = "0x18 - RMT_CH2CONF0"]
    pub ch2conf0: CH2CONF0,
    #[doc = "0x1c - RMT_CH2CONF1"]
    pub ch2conf1: CH2CONF1,
    #[doc = "0x20 - RMT_CH3CONF0"]
    pub ch3conf0: CH3CONF0,
    #[doc = "0x24 - RMT_CH3CONF1"]
    pub ch3conf1: CH3CONF1,
    #[doc = "0x28 - RMT_CH0STATUS"]
    pub ch0status: CH0STATUS,
    #[doc = "0x2c - RMT_CH1STATUS"]
    pub ch1status: CH1STATUS,
    #[doc = "0x30 - RMT_CH2STATUS"]
    pub ch2status: CH2STATUS,
    #[doc = "0x34 - RMT_CH3STATUS"]
    pub ch3status: CH3STATUS,
    #[doc = "0x38 - RMT_INT_RAW"]
    pub int_raw: INT_RAW,
    #[doc = "0x3c - RMT_INT_ST"]
    pub int_st: INT_ST,
    #[doc = "0x40 - RMT_INT_ENA"]
    pub int_ena: INT_ENA,
    #[doc = "0x44 - RMT_INT_CLR"]
    pub int_clr: INT_CLR,
    #[doc = "0x48 - RMT_CH0CARRIER_DUTY"]
    pub ch0carrier_duty: CH0CARRIER_DUTY,
    #[doc = "0x4c - RMT_CH1CARRIER_DUTY"]
    pub ch1carrier_duty: CH1CARRIER_DUTY,
    #[doc = "0x50 - RMT_CH2_RX_CARRIER_RM"]
    pub ch2_rx_carrier_rm: CH2_RX_CARRIER_RM,
    #[doc = "0x54 - RMT_CH3_RX_CARRIER_RM"]
    pub ch3_rx_carrier_rm: CH3_RX_CARRIER_RM,
    #[doc = "0x58 - RMT_CH0_TX_LIM"]
    pub ch0_tx_lim: CH0_TX_LIM,
    #[doc = "0x5c - RMT_CH1_TX_LIM"]
    pub ch1_tx_lim: CH1_TX_LIM,
    #[doc = "0x60 - RMT_CH2_RX_LIM"]
    pub ch2_rx_lim: CH2_RX_LIM,
    #[doc = "0x64 - RMT_CH3_RX_LIM"]
    pub ch3_rx_lim: CH3_RX_LIM,
    #[doc = "0x68 - RMT_SYS_CONF"]
    pub sys_conf: SYS_CONF,
    #[doc = "0x6c - RMT_TX_SIM"]
    pub tx_sim: TX_SIM,
    #[doc = "0x70 - RMT_REF_CNT_RST"]
    pub ref_cnt_rst: REF_CNT_RST,
    _reserved25: [u8; 88usize],
    #[doc = "0xcc - RMT_DATE"]
    pub date: DATE,
}
#[doc = "RMT_CH0CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0conf0](ch0conf0) module"]
pub type CH0CONF0 = crate::Reg<u32, _CH0CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0CONF0;
#[doc = "`read()` method returns [ch0conf0::R](ch0conf0::R) reader structure"]
impl crate::Readable for CH0CONF0 {}
#[doc = "`write(|w| ..)` method takes [ch0conf0::W](ch0conf0::W) writer structure"]
impl crate::Writable for CH0CONF0 {}
#[doc = "RMT_CH0CONF0"]
pub mod ch0conf0;
#[doc = "RMT_CH1CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1conf0](ch1conf0) module"]
pub type CH1CONF0 = crate::Reg<u32, _CH1CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1CONF0;
#[doc = "`read()` method returns [ch1conf0::R](ch1conf0::R) reader structure"]
impl crate::Readable for CH1CONF0 {}
#[doc = "`write(|w| ..)` method takes [ch1conf0::W](ch1conf0::W) writer structure"]
impl crate::Writable for CH1CONF0 {}
#[doc = "RMT_CH1CONF0"]
pub mod ch1conf0;
#[doc = "RMT_CH2CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2conf0](ch2conf0) module"]
pub type CH2CONF0 = crate::Reg<u32, _CH2CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2CONF0;
#[doc = "`read()` method returns [ch2conf0::R](ch2conf0::R) reader structure"]
impl crate::Readable for CH2CONF0 {}
#[doc = "`write(|w| ..)` method takes [ch2conf0::W](ch2conf0::W) writer structure"]
impl crate::Writable for CH2CONF0 {}
#[doc = "RMT_CH2CONF0"]
pub mod ch2conf0;
#[doc = "RMT_CH2CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2conf1](ch2conf1) module"]
pub type CH2CONF1 = crate::Reg<u32, _CH2CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2CONF1;
#[doc = "`read()` method returns [ch2conf1::R](ch2conf1::R) reader structure"]
impl crate::Readable for CH2CONF1 {}
#[doc = "`write(|w| ..)` method takes [ch2conf1::W](ch2conf1::W) writer structure"]
impl crate::Writable for CH2CONF1 {}
#[doc = "RMT_CH2CONF1"]
pub mod ch2conf1;
#[doc = "RMT_CH3CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3conf0](ch3conf0) module"]
pub type CH3CONF0 = crate::Reg<u32, _CH3CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3CONF0;
#[doc = "`read()` method returns [ch3conf0::R](ch3conf0::R) reader structure"]
impl crate::Readable for CH3CONF0 {}
#[doc = "`write(|w| ..)` method takes [ch3conf0::W](ch3conf0::W) writer structure"]
impl crate::Writable for CH3CONF0 {}
#[doc = "RMT_CH3CONF0"]
pub mod ch3conf0;
#[doc = "RMT_CH3CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3conf1](ch3conf1) module"]
pub type CH3CONF1 = crate::Reg<u32, _CH3CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3CONF1;
#[doc = "`read()` method returns [ch3conf1::R](ch3conf1::R) reader structure"]
impl crate::Readable for CH3CONF1 {}
#[doc = "`write(|w| ..)` method takes [ch3conf1::W](ch3conf1::W) writer structure"]
impl crate::Writable for CH3CONF1 {}
#[doc = "RMT_CH3CONF1"]
pub mod ch3conf1;
#[doc = "RMT_CH0STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0status](ch0status) module"]
pub type CH0STATUS = crate::Reg<u32, _CH0STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0STATUS;
#[doc = "`read()` method returns [ch0status::R](ch0status::R) reader structure"]
impl crate::Readable for CH0STATUS {}
#[doc = "RMT_CH0STATUS"]
pub mod ch0status;
#[doc = "RMT_CH1STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1status](ch1status) module"]
pub type CH1STATUS = crate::Reg<u32, _CH1STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1STATUS;
#[doc = "`read()` method returns [ch1status::R](ch1status::R) reader structure"]
impl crate::Readable for CH1STATUS {}
#[doc = "RMT_CH1STATUS"]
pub mod ch1status;
#[doc = "RMT_CH2STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2status](ch2status) module"]
pub type CH2STATUS = crate::Reg<u32, _CH2STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2STATUS;
#[doc = "`read()` method returns [ch2status::R](ch2status::R) reader structure"]
impl crate::Readable for CH2STATUS {}
#[doc = "RMT_CH2STATUS"]
pub mod ch2status;
#[doc = "RMT_CH3STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3status](ch3status) module"]
pub type CH3STATUS = crate::Reg<u32, _CH3STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3STATUS;
#[doc = "`read()` method returns [ch3status::R](ch3status::R) reader structure"]
impl crate::Readable for CH3STATUS {}
#[doc = "RMT_CH3STATUS"]
pub mod ch3status;
#[doc = "RMT_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](int_raw) module"]
pub type INT_RAW = crate::Reg<u32, _INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_RAW;
#[doc = "`read()` method returns [int_raw::R](int_raw::R) reader structure"]
impl crate::Readable for INT_RAW {}
#[doc = "`write(|w| ..)` method takes [int_raw::W](int_raw::W) writer structure"]
impl crate::Writable for INT_RAW {}
#[doc = "RMT_INT_RAW"]
pub mod int_raw;
#[doc = "RMT_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](int_st) module"]
pub type INT_ST = crate::Reg<u32, _INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ST;
#[doc = "`read()` method returns [int_st::R](int_st::R) reader structure"]
impl crate::Readable for INT_ST {}
#[doc = "RMT_INT_ST"]
pub mod int_st;
#[doc = "RMT_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](int_ena) module"]
pub type INT_ENA = crate::Reg<u32, _INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ENA;
#[doc = "`read()` method returns [int_ena::R](int_ena::R) reader structure"]
impl crate::Readable for INT_ENA {}
#[doc = "`write(|w| ..)` method takes [int_ena::W](int_ena::W) writer structure"]
impl crate::Writable for INT_ENA {}
#[doc = "RMT_INT_ENA"]
pub mod int_ena;
#[doc = "RMT_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](int_clr) module"]
pub type INT_CLR = crate::Reg<u32, _INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CLR;
#[doc = "`write(|w| ..)` method takes [int_clr::W](int_clr::W) writer structure"]
impl crate::Writable for INT_CLR {}
#[doc = "RMT_INT_CLR"]
pub mod int_clr;
#[doc = "RMT_CH0CARRIER_DUTY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0carrier_duty](ch0carrier_duty) module"]
pub type CH0CARRIER_DUTY = crate::Reg<u32, _CH0CARRIER_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0CARRIER_DUTY;
#[doc = "`read()` method returns [ch0carrier_duty::R](ch0carrier_duty::R) reader structure"]
impl crate::Readable for CH0CARRIER_DUTY {}
#[doc = "`write(|w| ..)` method takes [ch0carrier_duty::W](ch0carrier_duty::W) writer structure"]
impl crate::Writable for CH0CARRIER_DUTY {}
#[doc = "RMT_CH0CARRIER_DUTY"]
pub mod ch0carrier_duty;
#[doc = "RMT_CH1CARRIER_DUTY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1carrier_duty](ch1carrier_duty) module"]
pub type CH1CARRIER_DUTY = crate::Reg<u32, _CH1CARRIER_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1CARRIER_DUTY;
#[doc = "`read()` method returns [ch1carrier_duty::R](ch1carrier_duty::R) reader structure"]
impl crate::Readable for CH1CARRIER_DUTY {}
#[doc = "`write(|w| ..)` method takes [ch1carrier_duty::W](ch1carrier_duty::W) writer structure"]
impl crate::Writable for CH1CARRIER_DUTY {}
#[doc = "RMT_CH1CARRIER_DUTY"]
pub mod ch1carrier_duty;
#[doc = "RMT_CH2_RX_CARRIER_RM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_rx_carrier_rm](ch2_rx_carrier_rm) module"]
pub type CH2_RX_CARRIER_RM = crate::Reg<u32, _CH2_RX_CARRIER_RM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_RX_CARRIER_RM;
#[doc = "`read()` method returns [ch2_rx_carrier_rm::R](ch2_rx_carrier_rm::R) reader structure"]
impl crate::Readable for CH2_RX_CARRIER_RM {}
#[doc = "`write(|w| ..)` method takes [ch2_rx_carrier_rm::W](ch2_rx_carrier_rm::W) writer structure"]
impl crate::Writable for CH2_RX_CARRIER_RM {}
#[doc = "RMT_CH2_RX_CARRIER_RM"]
pub mod ch2_rx_carrier_rm;
#[doc = "RMT_CH3_RX_CARRIER_RM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_rx_carrier_rm](ch3_rx_carrier_rm) module"]
pub type CH3_RX_CARRIER_RM = crate::Reg<u32, _CH3_RX_CARRIER_RM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_RX_CARRIER_RM;
#[doc = "`read()` method returns [ch3_rx_carrier_rm::R](ch3_rx_carrier_rm::R) reader structure"]
impl crate::Readable for CH3_RX_CARRIER_RM {}
#[doc = "`write(|w| ..)` method takes [ch3_rx_carrier_rm::W](ch3_rx_carrier_rm::W) writer structure"]
impl crate::Writable for CH3_RX_CARRIER_RM {}
#[doc = "RMT_CH3_RX_CARRIER_RM"]
pub mod ch3_rx_carrier_rm;
#[doc = "RMT_CH0_TX_LIM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_tx_lim](ch0_tx_lim) module"]
pub type CH0_TX_LIM = crate::Reg<u32, _CH0_TX_LIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_TX_LIM;
#[doc = "`read()` method returns [ch0_tx_lim::R](ch0_tx_lim::R) reader structure"]
impl crate::Readable for CH0_TX_LIM {}
#[doc = "`write(|w| ..)` method takes [ch0_tx_lim::W](ch0_tx_lim::W) writer structure"]
impl crate::Writable for CH0_TX_LIM {}
#[doc = "RMT_CH0_TX_LIM"]
pub mod ch0_tx_lim;
#[doc = "RMT_CH1_TX_LIM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_tx_lim](ch1_tx_lim) module"]
pub type CH1_TX_LIM = crate::Reg<u32, _CH1_TX_LIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_TX_LIM;
#[doc = "`read()` method returns [ch1_tx_lim::R](ch1_tx_lim::R) reader structure"]
impl crate::Readable for CH1_TX_LIM {}
#[doc = "`write(|w| ..)` method takes [ch1_tx_lim::W](ch1_tx_lim::W) writer structure"]
impl crate::Writable for CH1_TX_LIM {}
#[doc = "RMT_CH1_TX_LIM"]
pub mod ch1_tx_lim;
#[doc = "RMT_CH2_RX_LIM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_rx_lim](ch2_rx_lim) module"]
pub type CH2_RX_LIM = crate::Reg<u32, _CH2_RX_LIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_RX_LIM;
#[doc = "`read()` method returns [ch2_rx_lim::R](ch2_rx_lim::R) reader structure"]
impl crate::Readable for CH2_RX_LIM {}
#[doc = "`write(|w| ..)` method takes [ch2_rx_lim::W](ch2_rx_lim::W) writer structure"]
impl crate::Writable for CH2_RX_LIM {}
#[doc = "RMT_CH2_RX_LIM"]
pub mod ch2_rx_lim;
#[doc = "RMT_CH3_RX_LIM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_rx_lim](ch3_rx_lim) module"]
pub type CH3_RX_LIM = crate::Reg<u32, _CH3_RX_LIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_RX_LIM;
#[doc = "`read()` method returns [ch3_rx_lim::R](ch3_rx_lim::R) reader structure"]
impl crate::Readable for CH3_RX_LIM {}
#[doc = "`write(|w| ..)` method takes [ch3_rx_lim::W](ch3_rx_lim::W) writer structure"]
impl crate::Writable for CH3_RX_LIM {}
#[doc = "RMT_CH3_RX_LIM"]
pub mod ch3_rx_lim;
#[doc = "RMT_SYS_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_conf](sys_conf) module"]
pub type SYS_CONF = crate::Reg<u32, _SYS_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_CONF;
#[doc = "`read()` method returns [sys_conf::R](sys_conf::R) reader structure"]
impl crate::Readable for SYS_CONF {}
#[doc = "`write(|w| ..)` method takes [sys_conf::W](sys_conf::W) writer structure"]
impl crate::Writable for SYS_CONF {}
#[doc = "RMT_SYS_CONF"]
pub mod sys_conf;
#[doc = "RMT_TX_SIM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_sim](tx_sim) module"]
pub type TX_SIM = crate::Reg<u32, _TX_SIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_SIM;
#[doc = "`read()` method returns [tx_sim::R](tx_sim::R) reader structure"]
impl crate::Readable for TX_SIM {}
#[doc = "`write(|w| ..)` method takes [tx_sim::W](tx_sim::W) writer structure"]
impl crate::Writable for TX_SIM {}
#[doc = "RMT_TX_SIM"]
pub mod tx_sim;
#[doc = "RMT_REF_CNT_RST\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_cnt_rst](ref_cnt_rst) module"]
pub type REF_CNT_RST = crate::Reg<u32, _REF_CNT_RST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REF_CNT_RST;
#[doc = "`write(|w| ..)` method takes [ref_cnt_rst::W](ref_cnt_rst::W) writer structure"]
impl crate::Writable for REF_CNT_RST {}
#[doc = "RMT_REF_CNT_RST"]
pub mod ref_cnt_rst;
#[doc = "RMT_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "RMT_DATE"]
pub mod date;
