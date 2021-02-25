#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI_CMD"]
    pub cmd: CMD,
    #[doc = "0x04 - SPI_ADDR"]
    pub addr: ADDR,
    #[doc = "0x08 - SPI_CTRL"]
    pub ctrl: CTRL,
    #[doc = "0x0c - SPI_CLOCK"]
    pub clock: CLOCK,
    #[doc = "0x10 - SPI_USER"]
    pub user: USER,
    #[doc = "0x14 - SPI_USER1"]
    pub user1: USER1,
    #[doc = "0x18 - SPI_USER2"]
    pub user2: USER2,
    #[doc = "0x1c - SPI_MS_DLEN"]
    pub ms_dlen: MS_DLEN,
    #[doc = "0x20 - SPI_MISC"]
    pub misc: MISC,
    #[doc = "0x24 - SPI_DIN_MODE"]
    pub din_mode: DIN_MODE,
    #[doc = "0x28 - SPI_DIN_NUM"]
    pub din_num: DIN_NUM,
    #[doc = "0x2c - SPI_DOUT_MODE"]
    pub dout_mode: DOUT_MODE,
    #[doc = "0x30 - SPI_DMA_CONF"]
    pub dma_conf: DMA_CONF,
    #[doc = "0x34 - SPI_DMA_INT_ENA"]
    pub dma_int_ena: DMA_INT_ENA,
    #[doc = "0x38 - SPI_DMA_INT_CLR"]
    pub dma_int_clr: DMA_INT_CLR,
    #[doc = "0x3c - SPI_DMA_INT_RAW"]
    pub dma_int_raw: DMA_INT_RAW,
    #[doc = "0x40 - SPI_DMA_INT_ST"]
    pub dma_int_st: DMA_INT_ST,
    _reserved17: [u8; 84usize],
    #[doc = "0x98 - SPI_W0"]
    pub w: [W; 16],
    _reserved18: [u8; 8usize],
    #[doc = "0xe0 - SPI_SLAVE"]
    pub slave: SLAVE,
    #[doc = "0xe4 - SPI_SLAVE1"]
    pub slave1: SLAVE1,
    #[doc = "0xe8 - SPI_CLK_GATE"]
    pub clk_gate: CLK_GATE,
    _reserved21: [u8; 4usize],
    #[doc = "0xf0 - SPI_DATE"]
    pub date: DATE,
}
#[doc = "SPI_CMD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`read()` method returns [cmd::R](cmd::R) reader structure"]
impl crate::Readable for CMD {}
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "SPI_CMD"]
pub mod cmd;
#[doc = "SPI_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](addr) module"]
pub type ADDR = crate::Reg<u32, _ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR;
#[doc = "`read()` method returns [addr::R](addr::R) reader structure"]
impl crate::Readable for ADDR {}
#[doc = "`write(|w| ..)` method takes [addr::W](addr::W) writer structure"]
impl crate::Writable for ADDR {}
#[doc = "SPI_ADDR"]
pub mod addr;
#[doc = "SPI_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "SPI_CTRL"]
pub mod ctrl;
#[doc = "SPI_CLOCK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock](clock) module"]
pub type CLOCK = crate::Reg<u32, _CLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCK;
#[doc = "`read()` method returns [clock::R](clock::R) reader structure"]
impl crate::Readable for CLOCK {}
#[doc = "`write(|w| ..)` method takes [clock::W](clock::W) writer structure"]
impl crate::Writable for CLOCK {}
#[doc = "SPI_CLOCK"]
pub mod clock;
#[doc = "SPI_USER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user](user) module"]
pub type USER = crate::Reg<u32, _USER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER;
#[doc = "`read()` method returns [user::R](user::R) reader structure"]
impl crate::Readable for USER {}
#[doc = "`write(|w| ..)` method takes [user::W](user::W) writer structure"]
impl crate::Writable for USER {}
#[doc = "SPI_USER"]
pub mod user;
#[doc = "SPI_USER1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user1](user1) module"]
pub type USER1 = crate::Reg<u32, _USER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER1;
#[doc = "`read()` method returns [user1::R](user1::R) reader structure"]
impl crate::Readable for USER1 {}
#[doc = "`write(|w| ..)` method takes [user1::W](user1::W) writer structure"]
impl crate::Writable for USER1 {}
#[doc = "SPI_USER1"]
pub mod user1;
#[doc = "SPI_USER2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user2](user2) module"]
pub type USER2 = crate::Reg<u32, _USER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER2;
#[doc = "`read()` method returns [user2::R](user2::R) reader structure"]
impl crate::Readable for USER2 {}
#[doc = "`write(|w| ..)` method takes [user2::W](user2::W) writer structure"]
impl crate::Writable for USER2 {}
#[doc = "SPI_USER2"]
pub mod user2;
#[doc = "SPI_MS_DLEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ms_dlen](ms_dlen) module"]
pub type MS_DLEN = crate::Reg<u32, _MS_DLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MS_DLEN;
#[doc = "`read()` method returns [ms_dlen::R](ms_dlen::R) reader structure"]
impl crate::Readable for MS_DLEN {}
#[doc = "`write(|w| ..)` method takes [ms_dlen::W](ms_dlen::W) writer structure"]
impl crate::Writable for MS_DLEN {}
#[doc = "SPI_MS_DLEN"]
pub mod ms_dlen;
#[doc = "SPI_MISC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc](misc) module"]
pub type MISC = crate::Reg<u32, _MISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC;
#[doc = "`read()` method returns [misc::R](misc::R) reader structure"]
impl crate::Readable for MISC {}
#[doc = "`write(|w| ..)` method takes [misc::W](misc::W) writer structure"]
impl crate::Writable for MISC {}
#[doc = "SPI_MISC"]
pub mod misc;
#[doc = "SPI_DIN_MODE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [din_mode](din_mode) module"]
pub type DIN_MODE = crate::Reg<u32, _DIN_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIN_MODE;
#[doc = "`read()` method returns [din_mode::R](din_mode::R) reader structure"]
impl crate::Readable for DIN_MODE {}
#[doc = "`write(|w| ..)` method takes [din_mode::W](din_mode::W) writer structure"]
impl crate::Writable for DIN_MODE {}
#[doc = "SPI_DIN_MODE"]
pub mod din_mode;
#[doc = "SPI_DIN_NUM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [din_num](din_num) module"]
pub type DIN_NUM = crate::Reg<u32, _DIN_NUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIN_NUM;
#[doc = "`read()` method returns [din_num::R](din_num::R) reader structure"]
impl crate::Readable for DIN_NUM {}
#[doc = "`write(|w| ..)` method takes [din_num::W](din_num::W) writer structure"]
impl crate::Writable for DIN_NUM {}
#[doc = "SPI_DIN_NUM"]
pub mod din_num;
#[doc = "SPI_DOUT_MODE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout_mode](dout_mode) module"]
pub type DOUT_MODE = crate::Reg<u32, _DOUT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT_MODE;
#[doc = "`read()` method returns [dout_mode::R](dout_mode::R) reader structure"]
impl crate::Readable for DOUT_MODE {}
#[doc = "`write(|w| ..)` method takes [dout_mode::W](dout_mode::W) writer structure"]
impl crate::Writable for DOUT_MODE {}
#[doc = "SPI_DOUT_MODE"]
pub mod dout_mode;
#[doc = "SPI_DMA_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_conf](dma_conf) module"]
pub type DMA_CONF = crate::Reg<u32, _DMA_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CONF;
#[doc = "`read()` method returns [dma_conf::R](dma_conf::R) reader structure"]
impl crate::Readable for DMA_CONF {}
#[doc = "`write(|w| ..)` method takes [dma_conf::W](dma_conf::W) writer structure"]
impl crate::Writable for DMA_CONF {}
#[doc = "SPI_DMA_CONF"]
pub mod dma_conf;
#[doc = "SPI_DMA_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_ena](dma_int_ena) module"]
pub type DMA_INT_ENA = crate::Reg<u32, _DMA_INT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT_ENA;
#[doc = "`read()` method returns [dma_int_ena::R](dma_int_ena::R) reader structure"]
impl crate::Readable for DMA_INT_ENA {}
#[doc = "`write(|w| ..)` method takes [dma_int_ena::W](dma_int_ena::W) writer structure"]
impl crate::Writable for DMA_INT_ENA {}
#[doc = "SPI_DMA_INT_ENA"]
pub mod dma_int_ena;
#[doc = "SPI_DMA_INT_CLR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_clr](dma_int_clr) module"]
pub type DMA_INT_CLR = crate::Reg<u32, _DMA_INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT_CLR;
#[doc = "`write(|w| ..)` method takes [dma_int_clr::W](dma_int_clr::W) writer structure"]
impl crate::Writable for DMA_INT_CLR {}
#[doc = "SPI_DMA_INT_CLR"]
pub mod dma_int_clr;
#[doc = "SPI_DMA_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_raw](dma_int_raw) module"]
pub type DMA_INT_RAW = crate::Reg<u32, _DMA_INT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT_RAW;
#[doc = "`read()` method returns [dma_int_raw::R](dma_int_raw::R) reader structure"]
impl crate::Readable for DMA_INT_RAW {}
#[doc = "`write(|w| ..)` method takes [dma_int_raw::W](dma_int_raw::W) writer structure"]
impl crate::Writable for DMA_INT_RAW {}
#[doc = "SPI_DMA_INT_RAW"]
pub mod dma_int_raw;
#[doc = "SPI_DMA_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_st](dma_int_st) module"]
pub type DMA_INT_ST = crate::Reg<u32, _DMA_INT_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INT_ST;
#[doc = "`read()` method returns [dma_int_st::R](dma_int_st::R) reader structure"]
impl crate::Readable for DMA_INT_ST {}
#[doc = "SPI_DMA_INT_ST"]
pub mod dma_int_st;
#[doc = "SPI_W0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w](w) module"]
pub type W = crate::Reg<u32, _W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W;
#[doc = "`read()` method returns [w::R](w::R) reader structure"]
impl crate::Readable for W {}
#[doc = "`write(|w| ..)` method takes [w::W](w::W) writer structure"]
impl crate::Writable for W {}
#[doc = "SPI_W0"]
pub mod w;
#[doc = "SPI_SLAVE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave](slave) module"]
pub type SLAVE = crate::Reg<u32, _SLAVE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLAVE;
#[doc = "`read()` method returns [slave::R](slave::R) reader structure"]
impl crate::Readable for SLAVE {}
#[doc = "`write(|w| ..)` method takes [slave::W](slave::W) writer structure"]
impl crate::Writable for SLAVE {}
#[doc = "SPI_SLAVE"]
pub mod slave;
#[doc = "SPI_SLAVE1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave1](slave1) module"]
pub type SLAVE1 = crate::Reg<u32, _SLAVE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLAVE1;
#[doc = "`read()` method returns [slave1::R](slave1::R) reader structure"]
impl crate::Readable for SLAVE1 {}
#[doc = "`write(|w| ..)` method takes [slave1::W](slave1::W) writer structure"]
impl crate::Writable for SLAVE1 {}
#[doc = "SPI_SLAVE1"]
pub mod slave1;
#[doc = "SPI_CLK_GATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_gate](clk_gate) module"]
pub type CLK_GATE = crate::Reg<u32, _CLK_GATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_GATE;
#[doc = "`read()` method returns [clk_gate::R](clk_gate::R) reader structure"]
impl crate::Readable for CLK_GATE {}
#[doc = "`write(|w| ..)` method takes [clk_gate::W](clk_gate::W) writer structure"]
impl crate::Writable for CLK_GATE {}
#[doc = "SPI_CLK_GATE"]
pub mod clk_gate;
#[doc = "SPI_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "SPI_DATE"]
pub mod date;
