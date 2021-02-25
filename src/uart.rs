#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART_FIFO"]
    pub fifo: FIFO,
    #[doc = "0x04 - UART_INT_RAW"]
    pub int_raw: INT_RAW,
    #[doc = "0x08 - UART_INT_ST"]
    pub int_st: INT_ST,
    #[doc = "0x0c - UART_INT_ENA"]
    pub int_ena: INT_ENA,
    #[doc = "0x10 - UART_INT_CLR"]
    pub int_clr: INT_CLR,
    #[doc = "0x14 - UART_CLKDIV"]
    pub clkdiv: CLKDIV,
    #[doc = "0x18 - UART_RX_FILT"]
    pub rx_filt: RX_FILT,
    #[doc = "0x1c - UART_STATUS"]
    pub status: STATUS,
    #[doc = "0x20 - UART_CONF0"]
    pub conf0: CONF0,
    #[doc = "0x24 - UART_CONF1"]
    pub conf1: CONF1,
    #[doc = "0x28 - UART_LOWPULSE"]
    pub lowpulse: LOWPULSE,
    #[doc = "0x2c - UART_HIGHPULSE"]
    pub highpulse: HIGHPULSE,
    #[doc = "0x30 - UART_RXD_CNT"]
    pub rxd_cnt: RXD_CNT,
    #[doc = "0x34 - UART_FLOW_CONF"]
    pub flow_conf: FLOW_CONF,
    #[doc = "0x38 - UART_SLEEP_CONF"]
    pub sleep_conf: SLEEP_CONF,
    #[doc = "0x3c - UART_SWFC_CONF0"]
    pub swfc_conf0: SWFC_CONF0,
    #[doc = "0x40 - UART_SWFC_CONF1"]
    pub swfc_conf1: SWFC_CONF1,
    #[doc = "0x44 - UART_TXBRK_CONF"]
    pub txbrk_conf: TXBRK_CONF,
    #[doc = "0x48 - UART_IDLE_CONF"]
    pub idle_conf: IDLE_CONF,
    #[doc = "0x4c - UART_RS485_CONF"]
    pub rs485_conf: RS485_CONF,
    #[doc = "0x50 - UART_AT_CMD_PRECNT"]
    pub at_cmd_precnt: AT_CMD_PRECNT,
    #[doc = "0x54 - UART_AT_CMD_POSTCNT"]
    pub at_cmd_postcnt: AT_CMD_POSTCNT,
    #[doc = "0x58 - UART_AT_CMD_GAPTOUT"]
    pub at_cmd_gaptout: AT_CMD_GAPTOUT,
    #[doc = "0x5c - UART_AT_CMD_CHAR"]
    pub at_cmd_char: AT_CMD_CHAR,
    #[doc = "0x60 - UART_MEM_CONF"]
    pub mem_conf: MEM_CONF,
    #[doc = "0x64 - UART_MEM_TX_STATUS"]
    pub mem_tx_status: MEM_TX_STATUS,
    #[doc = "0x68 - UART_MEM_RX_STATUS"]
    pub mem_rx_status: MEM_RX_STATUS,
    #[doc = "0x6c - UART_FSM_STATUS"]
    pub fsm_status: FSM_STATUS,
    #[doc = "0x70 - UART_POSPULSE"]
    pub pospulse: POSPULSE,
    #[doc = "0x74 - UART_NEGPULSE"]
    pub negpulse: NEGPULSE,
    #[doc = "0x78 - UART_CLK_CONF"]
    pub clk_conf: CLK_CONF,
    #[doc = "0x7c - UART_DATE"]
    pub date: DATE,
    #[doc = "0x80 - UART_ID"]
    pub id: ID,
}
#[doc = "UART_FIFO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo](fifo) module"]
pub type FIFO = crate::Reg<u32, _FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO;
#[doc = "`read()` method returns [fifo::R](fifo::R) reader structure"]
impl crate::Readable for FIFO {}
#[doc = "UART_FIFO"]
pub mod fifo;
#[doc = "UART_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](int_raw) module"]
pub type INT_RAW = crate::Reg<u32, _INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_RAW;
#[doc = "`read()` method returns [int_raw::R](int_raw::R) reader structure"]
impl crate::Readable for INT_RAW {}
#[doc = "`write(|w| ..)` method takes [int_raw::W](int_raw::W) writer structure"]
impl crate::Writable for INT_RAW {}
#[doc = "UART_INT_RAW"]
pub mod int_raw;
#[doc = "UART_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](int_st) module"]
pub type INT_ST = crate::Reg<u32, _INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ST;
#[doc = "`read()` method returns [int_st::R](int_st::R) reader structure"]
impl crate::Readable for INT_ST {}
#[doc = "UART_INT_ST"]
pub mod int_st;
#[doc = "UART_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](int_ena) module"]
pub type INT_ENA = crate::Reg<u32, _INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_ENA;
#[doc = "`read()` method returns [int_ena::R](int_ena::R) reader structure"]
impl crate::Readable for INT_ENA {}
#[doc = "`write(|w| ..)` method takes [int_ena::W](int_ena::W) writer structure"]
impl crate::Writable for INT_ENA {}
#[doc = "UART_INT_ENA"]
pub mod int_ena;
#[doc = "UART_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](int_clr) module"]
pub type INT_CLR = crate::Reg<u32, _INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CLR;
#[doc = "`write(|w| ..)` method takes [int_clr::W](int_clr::W) writer structure"]
impl crate::Writable for INT_CLR {}
#[doc = "UART_INT_CLR"]
pub mod int_clr;
#[doc = "UART_CLKDIV\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdiv](clkdiv) module"]
pub type CLKDIV = crate::Reg<u32, _CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKDIV;
#[doc = "`read()` method returns [clkdiv::R](clkdiv::R) reader structure"]
impl crate::Readable for CLKDIV {}
#[doc = "`write(|w| ..)` method takes [clkdiv::W](clkdiv::W) writer structure"]
impl crate::Writable for CLKDIV {}
#[doc = "UART_CLKDIV"]
pub mod clkdiv;
#[doc = "UART_RX_FILT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_filt](rx_filt) module"]
pub type RX_FILT = crate::Reg<u32, _RX_FILT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_FILT;
#[doc = "`read()` method returns [rx_filt::R](rx_filt::R) reader structure"]
impl crate::Readable for RX_FILT {}
#[doc = "`write(|w| ..)` method takes [rx_filt::W](rx_filt::W) writer structure"]
impl crate::Writable for RX_FILT {}
#[doc = "UART_RX_FILT"]
pub mod rx_filt;
#[doc = "UART_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "UART_STATUS"]
pub mod status;
#[doc = "UART_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf0](conf0) module"]
pub type CONF0 = crate::Reg<u32, _CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONF0;
#[doc = "`read()` method returns [conf0::R](conf0::R) reader structure"]
impl crate::Readable for CONF0 {}
#[doc = "`write(|w| ..)` method takes [conf0::W](conf0::W) writer structure"]
impl crate::Writable for CONF0 {}
#[doc = "UART_CONF0"]
pub mod conf0;
#[doc = "UART_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf1](conf1) module"]
pub type CONF1 = crate::Reg<u32, _CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONF1;
#[doc = "`read()` method returns [conf1::R](conf1::R) reader structure"]
impl crate::Readable for CONF1 {}
#[doc = "`write(|w| ..)` method takes [conf1::W](conf1::W) writer structure"]
impl crate::Writable for CONF1 {}
#[doc = "UART_CONF1"]
pub mod conf1;
#[doc = "UART_LOWPULSE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lowpulse](lowpulse) module"]
pub type LOWPULSE = crate::Reg<u32, _LOWPULSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOWPULSE;
#[doc = "`read()` method returns [lowpulse::R](lowpulse::R) reader structure"]
impl crate::Readable for LOWPULSE {}
#[doc = "UART_LOWPULSE"]
pub mod lowpulse;
#[doc = "UART_HIGHPULSE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [highpulse](highpulse) module"]
pub type HIGHPULSE = crate::Reg<u32, _HIGHPULSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIGHPULSE;
#[doc = "`read()` method returns [highpulse::R](highpulse::R) reader structure"]
impl crate::Readable for HIGHPULSE {}
#[doc = "UART_HIGHPULSE"]
pub mod highpulse;
#[doc = "UART_RXD_CNT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxd_cnt](rxd_cnt) module"]
pub type RXD_CNT = crate::Reg<u32, _RXD_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXD_CNT;
#[doc = "`read()` method returns [rxd_cnt::R](rxd_cnt::R) reader structure"]
impl crate::Readable for RXD_CNT {}
#[doc = "UART_RXD_CNT"]
pub mod rxd_cnt;
#[doc = "UART_FLOW_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flow_conf](flow_conf) module"]
pub type FLOW_CONF = crate::Reg<u32, _FLOW_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLOW_CONF;
#[doc = "`read()` method returns [flow_conf::R](flow_conf::R) reader structure"]
impl crate::Readable for FLOW_CONF {}
#[doc = "`write(|w| ..)` method takes [flow_conf::W](flow_conf::W) writer structure"]
impl crate::Writable for FLOW_CONF {}
#[doc = "UART_FLOW_CONF"]
pub mod flow_conf;
#[doc = "UART_SLEEP_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sleep_conf](sleep_conf) module"]
pub type SLEEP_CONF = crate::Reg<u32, _SLEEP_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLEEP_CONF;
#[doc = "`read()` method returns [sleep_conf::R](sleep_conf::R) reader structure"]
impl crate::Readable for SLEEP_CONF {}
#[doc = "`write(|w| ..)` method takes [sleep_conf::W](sleep_conf::W) writer structure"]
impl crate::Writable for SLEEP_CONF {}
#[doc = "UART_SLEEP_CONF"]
pub mod sleep_conf;
#[doc = "UART_SWFC_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swfc_conf0](swfc_conf0) module"]
pub type SWFC_CONF0 = crate::Reg<u32, _SWFC_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWFC_CONF0;
#[doc = "`read()` method returns [swfc_conf0::R](swfc_conf0::R) reader structure"]
impl crate::Readable for SWFC_CONF0 {}
#[doc = "`write(|w| ..)` method takes [swfc_conf0::W](swfc_conf0::W) writer structure"]
impl crate::Writable for SWFC_CONF0 {}
#[doc = "UART_SWFC_CONF0"]
pub mod swfc_conf0;
#[doc = "UART_SWFC_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swfc_conf1](swfc_conf1) module"]
pub type SWFC_CONF1 = crate::Reg<u32, _SWFC_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWFC_CONF1;
#[doc = "`read()` method returns [swfc_conf1::R](swfc_conf1::R) reader structure"]
impl crate::Readable for SWFC_CONF1 {}
#[doc = "`write(|w| ..)` method takes [swfc_conf1::W](swfc_conf1::W) writer structure"]
impl crate::Writable for SWFC_CONF1 {}
#[doc = "UART_SWFC_CONF1"]
pub mod swfc_conf1;
#[doc = "UART_TXBRK_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbrk_conf](txbrk_conf) module"]
pub type TXBRK_CONF = crate::Reg<u32, _TXBRK_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBRK_CONF;
#[doc = "`read()` method returns [txbrk_conf::R](txbrk_conf::R) reader structure"]
impl crate::Readable for TXBRK_CONF {}
#[doc = "`write(|w| ..)` method takes [txbrk_conf::W](txbrk_conf::W) writer structure"]
impl crate::Writable for TXBRK_CONF {}
#[doc = "UART_TXBRK_CONF"]
pub mod txbrk_conf;
#[doc = "UART_IDLE_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idle_conf](idle_conf) module"]
pub type IDLE_CONF = crate::Reg<u32, _IDLE_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDLE_CONF;
#[doc = "`read()` method returns [idle_conf::R](idle_conf::R) reader structure"]
impl crate::Readable for IDLE_CONF {}
#[doc = "`write(|w| ..)` method takes [idle_conf::W](idle_conf::W) writer structure"]
impl crate::Writable for IDLE_CONF {}
#[doc = "UART_IDLE_CONF"]
pub mod idle_conf;
#[doc = "UART_RS485_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rs485_conf](rs485_conf) module"]
pub type RS485_CONF = crate::Reg<u32, _RS485_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RS485_CONF;
#[doc = "`read()` method returns [rs485_conf::R](rs485_conf::R) reader structure"]
impl crate::Readable for RS485_CONF {}
#[doc = "`write(|w| ..)` method takes [rs485_conf::W](rs485_conf::W) writer structure"]
impl crate::Writable for RS485_CONF {}
#[doc = "UART_RS485_CONF"]
pub mod rs485_conf;
#[doc = "UART_AT_CMD_PRECNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [at_cmd_precnt](at_cmd_precnt) module"]
pub type AT_CMD_PRECNT = crate::Reg<u32, _AT_CMD_PRECNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AT_CMD_PRECNT;
#[doc = "`read()` method returns [at_cmd_precnt::R](at_cmd_precnt::R) reader structure"]
impl crate::Readable for AT_CMD_PRECNT {}
#[doc = "`write(|w| ..)` method takes [at_cmd_precnt::W](at_cmd_precnt::W) writer structure"]
impl crate::Writable for AT_CMD_PRECNT {}
#[doc = "UART_AT_CMD_PRECNT"]
pub mod at_cmd_precnt;
#[doc = "UART_AT_CMD_POSTCNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [at_cmd_postcnt](at_cmd_postcnt) module"]
pub type AT_CMD_POSTCNT = crate::Reg<u32, _AT_CMD_POSTCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AT_CMD_POSTCNT;
#[doc = "`read()` method returns [at_cmd_postcnt::R](at_cmd_postcnt::R) reader structure"]
impl crate::Readable for AT_CMD_POSTCNT {}
#[doc = "`write(|w| ..)` method takes [at_cmd_postcnt::W](at_cmd_postcnt::W) writer structure"]
impl crate::Writable for AT_CMD_POSTCNT {}
#[doc = "UART_AT_CMD_POSTCNT"]
pub mod at_cmd_postcnt;
#[doc = "UART_AT_CMD_GAPTOUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [at_cmd_gaptout](at_cmd_gaptout) module"]
pub type AT_CMD_GAPTOUT = crate::Reg<u32, _AT_CMD_GAPTOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AT_CMD_GAPTOUT;
#[doc = "`read()` method returns [at_cmd_gaptout::R](at_cmd_gaptout::R) reader structure"]
impl crate::Readable for AT_CMD_GAPTOUT {}
#[doc = "`write(|w| ..)` method takes [at_cmd_gaptout::W](at_cmd_gaptout::W) writer structure"]
impl crate::Writable for AT_CMD_GAPTOUT {}
#[doc = "UART_AT_CMD_GAPTOUT"]
pub mod at_cmd_gaptout;
#[doc = "UART_AT_CMD_CHAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [at_cmd_char](at_cmd_char) module"]
pub type AT_CMD_CHAR = crate::Reg<u32, _AT_CMD_CHAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AT_CMD_CHAR;
#[doc = "`read()` method returns [at_cmd_char::R](at_cmd_char::R) reader structure"]
impl crate::Readable for AT_CMD_CHAR {}
#[doc = "`write(|w| ..)` method takes [at_cmd_char::W](at_cmd_char::W) writer structure"]
impl crate::Writable for AT_CMD_CHAR {}
#[doc = "UART_AT_CMD_CHAR"]
pub mod at_cmd_char;
#[doc = "UART_MEM_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_conf](mem_conf) module"]
pub type MEM_CONF = crate::Reg<u32, _MEM_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_CONF;
#[doc = "`read()` method returns [mem_conf::R](mem_conf::R) reader structure"]
impl crate::Readable for MEM_CONF {}
#[doc = "`write(|w| ..)` method takes [mem_conf::W](mem_conf::W) writer structure"]
impl crate::Writable for MEM_CONF {}
#[doc = "UART_MEM_CONF"]
pub mod mem_conf;
#[doc = "UART_MEM_TX_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_tx_status](mem_tx_status) module"]
pub type MEM_TX_STATUS = crate::Reg<u32, _MEM_TX_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_TX_STATUS;
#[doc = "`read()` method returns [mem_tx_status::R](mem_tx_status::R) reader structure"]
impl crate::Readable for MEM_TX_STATUS {}
#[doc = "UART_MEM_TX_STATUS"]
pub mod mem_tx_status;
#[doc = "UART_MEM_RX_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_rx_status](mem_rx_status) module"]
pub type MEM_RX_STATUS = crate::Reg<u32, _MEM_RX_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_RX_STATUS;
#[doc = "`read()` method returns [mem_rx_status::R](mem_rx_status::R) reader structure"]
impl crate::Readable for MEM_RX_STATUS {}
#[doc = "UART_MEM_RX_STATUS"]
pub mod mem_rx_status;
#[doc = "UART_FSM_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_status](fsm_status) module"]
pub type FSM_STATUS = crate::Reg<u32, _FSM_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_STATUS;
#[doc = "`read()` method returns [fsm_status::R](fsm_status::R) reader structure"]
impl crate::Readable for FSM_STATUS {}
#[doc = "UART_FSM_STATUS"]
pub mod fsm_status;
#[doc = "UART_POSPULSE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pospulse](pospulse) module"]
pub type POSPULSE = crate::Reg<u32, _POSPULSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POSPULSE;
#[doc = "`read()` method returns [pospulse::R](pospulse::R) reader structure"]
impl crate::Readable for POSPULSE {}
#[doc = "UART_POSPULSE"]
pub mod pospulse;
#[doc = "UART_NEGPULSE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [negpulse](negpulse) module"]
pub type NEGPULSE = crate::Reg<u32, _NEGPULSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NEGPULSE;
#[doc = "`read()` method returns [negpulse::R](negpulse::R) reader structure"]
impl crate::Readable for NEGPULSE {}
#[doc = "UART_NEGPULSE"]
pub mod negpulse;
#[doc = "UART_CLK_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_conf](clk_conf) module"]
pub type CLK_CONF = crate::Reg<u32, _CLK_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_CONF;
#[doc = "`read()` method returns [clk_conf::R](clk_conf::R) reader structure"]
impl crate::Readable for CLK_CONF {}
#[doc = "`write(|w| ..)` method takes [clk_conf::W](clk_conf::W) writer structure"]
impl crate::Writable for CLK_CONF {}
#[doc = "UART_CLK_CONF"]
pub mod clk_conf;
#[doc = "UART_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "UART_DATE"]
pub mod date;
#[doc = "UART_ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id](id) module"]
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "`write(|w| ..)` method takes [id::W](id::W) writer structure"]
impl crate::Writable for ID {}
#[doc = "UART_ID"]
pub mod id;
