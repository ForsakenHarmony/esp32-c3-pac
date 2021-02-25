#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO_BT_SELECT"]
    pub bt_select: BT_SELECT,
    #[doc = "0x04 - GPIO_OUT"]
    pub out: OUT,
    #[doc = "0x08 - GPIO_OUT_W1TS"]
    pub out_w1ts: OUT_W1TS,
    #[doc = "0x0c - GPIO_OUT_W1TC"]
    pub out_w1tc: OUT_W1TC,
    _reserved4: [u8; 12usize],
    #[doc = "0x1c - GPIO_SDIO_SELECT"]
    pub sdio_select: SDIO_SELECT,
    #[doc = "0x20 - GPIO_ENABLE"]
    pub enable: ENABLE,
    #[doc = "0x24 - GPIO_ENABLE_W1TS"]
    pub enable_w1ts: ENABLE_W1TS,
    #[doc = "0x28 - GPIO_ENABLE_W1TC"]
    pub enable_w1tc: ENABLE_W1TC,
    _reserved8: [u8; 12usize],
    #[doc = "0x38 - GPIO_STRAP"]
    pub strap: STRAP,
    #[doc = "0x3c - GPIO_IN"]
    pub in_: IN,
    _reserved10: [u8; 4usize],
    #[doc = "0x44 - GPIO_STATUS"]
    pub status: STATUS,
    #[doc = "0x48 - GPIO_STATUS_W1TS"]
    pub status_w1ts: STATUS_W1TS,
    #[doc = "0x4c - GPIO_STATUS_W1TC"]
    pub status_w1tc: STATUS_W1TC,
    _reserved13: [u8; 12usize],
    #[doc = "0x5c - GPIO_PCPU_INT"]
    pub pcpu_int: PCPU_INT,
    #[doc = "0x60 - GPIO_PCPU_NMI_INT"]
    pub pcpu_nmi_int: PCPU_NMI_INT,
    #[doc = "0x64 - GPIO_CPUSDIO_INT"]
    pub cpusdio_int: CPUSDIO_INT,
    _reserved16: [u8; 12usize],
    #[doc = "0x74 - GPIO_PIN0"]
    pub pin: [PIN; 26],
    _reserved17: [u8; 112usize],
    #[doc = "0x14c - GPIO_STATUS_NEXT"]
    pub status_next: STATUS_NEXT,
    _reserved18: [u8; 4usize],
    #[doc = "0x154 - GPIO_FUNC0_IN_SEL_CFG"]
    pub func_in_sel_cfg: [FUNC_IN_SEL_CFG; 128],
    _reserved19: [u8; 512usize],
    #[doc = "0x554 - GPIO_FUNC0_OUT_SEL_CFG"]
    pub func_out_sel_cfg: [FUNC_OUT_SEL_CFG; 26],
    _reserved20: [u8; 112usize],
    #[doc = "0x62c - GPIO_CLOCK_GATE"]
    pub clock_gate: CLOCK_GATE,
    _reserved21: [u8; 204usize],
    #[doc = "0x6fc - GPIO_DATE"]
    pub date: DATE,
}
#[doc = "GPIO_BT_SELECT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bt_select](bt_select) module"]
pub type BT_SELECT = crate::Reg<u32, _BT_SELECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BT_SELECT;
#[doc = "`read()` method returns [bt_select::R](bt_select::R) reader structure"]
impl crate::Readable for BT_SELECT {}
#[doc = "`write(|w| ..)` method takes [bt_select::W](bt_select::W) writer structure"]
impl crate::Writable for BT_SELECT {}
#[doc = "GPIO_BT_SELECT"]
pub mod bt_select;
#[doc = "GPIO_OUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out](out) module"]
pub type OUT = crate::Reg<u32, _OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT;
#[doc = "`read()` method returns [out::R](out::R) reader structure"]
impl crate::Readable for OUT {}
#[doc = "`write(|w| ..)` method takes [out::W](out::W) writer structure"]
impl crate::Writable for OUT {}
#[doc = "GPIO_OUT"]
pub mod out;
#[doc = "GPIO_OUT_W1TS\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_w1ts](out_w1ts) module"]
pub type OUT_W1TS = crate::Reg<u32, _OUT_W1TS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_W1TS;
#[doc = "`write(|w| ..)` method takes [out_w1ts::W](out_w1ts::W) writer structure"]
impl crate::Writable for OUT_W1TS {}
#[doc = "GPIO_OUT_W1TS"]
pub mod out_w1ts;
#[doc = "GPIO_OUT_W1TC\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_w1tc](out_w1tc) module"]
pub type OUT_W1TC = crate::Reg<u32, _OUT_W1TC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_W1TC;
#[doc = "`write(|w| ..)` method takes [out_w1tc::W](out_w1tc::W) writer structure"]
impl crate::Writable for OUT_W1TC {}
#[doc = "GPIO_OUT_W1TC"]
pub mod out_w1tc;
#[doc = "GPIO_SDIO_SELECT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_select](sdio_select) module"]
pub type SDIO_SELECT = crate::Reg<u32, _SDIO_SELECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDIO_SELECT;
#[doc = "`read()` method returns [sdio_select::R](sdio_select::R) reader structure"]
impl crate::Readable for SDIO_SELECT {}
#[doc = "`write(|w| ..)` method takes [sdio_select::W](sdio_select::W) writer structure"]
impl crate::Writable for SDIO_SELECT {}
#[doc = "GPIO_SDIO_SELECT"]
pub mod sdio_select;
#[doc = "GPIO_ENABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "GPIO_ENABLE"]
pub mod enable;
#[doc = "GPIO_ENABLE_W1TS\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable_w1ts](enable_w1ts) module"]
pub type ENABLE_W1TS = crate::Reg<u32, _ENABLE_W1TS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE_W1TS;
#[doc = "`write(|w| ..)` method takes [enable_w1ts::W](enable_w1ts::W) writer structure"]
impl crate::Writable for ENABLE_W1TS {}
#[doc = "GPIO_ENABLE_W1TS"]
pub mod enable_w1ts;
#[doc = "GPIO_ENABLE_W1TC\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable_w1tc](enable_w1tc) module"]
pub type ENABLE_W1TC = crate::Reg<u32, _ENABLE_W1TC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE_W1TC;
#[doc = "`write(|w| ..)` method takes [enable_w1tc::W](enable_w1tc::W) writer structure"]
impl crate::Writable for ENABLE_W1TC {}
#[doc = "GPIO_ENABLE_W1TC"]
pub mod enable_w1tc;
#[doc = "GPIO_STRAP\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [strap](strap) module"]
pub type STRAP = crate::Reg<u32, _STRAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STRAP;
#[doc = "`read()` method returns [strap::R](strap::R) reader structure"]
impl crate::Readable for STRAP {}
#[doc = "GPIO_STRAP"]
pub mod strap;
#[doc = "GPIO_IN\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_](in_) module"]
pub type IN = crate::Reg<u32, _IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IN;
#[doc = "`read()` method returns [in_::R](in_::R) reader structure"]
impl crate::Readable for IN {}
#[doc = "GPIO_IN"]
pub mod in_;
#[doc = "GPIO_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "GPIO_STATUS"]
pub mod status;
#[doc = "GPIO_STATUS_W1TS\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status_w1ts](status_w1ts) module"]
pub type STATUS_W1TS = crate::Reg<u32, _STATUS_W1TS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS_W1TS;
#[doc = "`write(|w| ..)` method takes [status_w1ts::W](status_w1ts::W) writer structure"]
impl crate::Writable for STATUS_W1TS {}
#[doc = "GPIO_STATUS_W1TS"]
pub mod status_w1ts;
#[doc = "GPIO_STATUS_W1TC\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status_w1tc](status_w1tc) module"]
pub type STATUS_W1TC = crate::Reg<u32, _STATUS_W1TC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS_W1TC;
#[doc = "`write(|w| ..)` method takes [status_w1tc::W](status_w1tc::W) writer structure"]
impl crate::Writable for STATUS_W1TC {}
#[doc = "GPIO_STATUS_W1TC"]
pub mod status_w1tc;
#[doc = "GPIO_PCPU_INT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcpu_int](pcpu_int) module"]
pub type PCPU_INT = crate::Reg<u32, _PCPU_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCPU_INT;
#[doc = "`read()` method returns [pcpu_int::R](pcpu_int::R) reader structure"]
impl crate::Readable for PCPU_INT {}
#[doc = "GPIO_PCPU_INT"]
pub mod pcpu_int;
#[doc = "GPIO_PCPU_NMI_INT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcpu_nmi_int](pcpu_nmi_int) module"]
pub type PCPU_NMI_INT = crate::Reg<u32, _PCPU_NMI_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCPU_NMI_INT;
#[doc = "`read()` method returns [pcpu_nmi_int::R](pcpu_nmi_int::R) reader structure"]
impl crate::Readable for PCPU_NMI_INT {}
#[doc = "GPIO_PCPU_NMI_INT"]
pub mod pcpu_nmi_int;
#[doc = "GPIO_CPUSDIO_INT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpusdio_int](cpusdio_int) module"]
pub type CPUSDIO_INT = crate::Reg<u32, _CPUSDIO_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUSDIO_INT;
#[doc = "`read()` method returns [cpusdio_int::R](cpusdio_int::R) reader structure"]
impl crate::Readable for CPUSDIO_INT {}
#[doc = "GPIO_CPUSDIO_INT"]
pub mod cpusdio_int;
#[doc = "GPIO_PIN0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin](pin) module"]
pub type PIN = crate::Reg<u32, _PIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN;
#[doc = "`read()` method returns [pin::R](pin::R) reader structure"]
impl crate::Readable for PIN {}
#[doc = "`write(|w| ..)` method takes [pin::W](pin::W) writer structure"]
impl crate::Writable for PIN {}
#[doc = "GPIO_PIN0"]
pub mod pin;
#[doc = "GPIO_STATUS_NEXT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status_next](status_next) module"]
pub type STATUS_NEXT = crate::Reg<u32, _STATUS_NEXT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS_NEXT;
#[doc = "`read()` method returns [status_next::R](status_next::R) reader structure"]
impl crate::Readable for STATUS_NEXT {}
#[doc = "GPIO_STATUS_NEXT"]
pub mod status_next;
#[doc = "GPIO_FUNC0_IN_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [func_in_sel_cfg](func_in_sel_cfg) module"]
pub type FUNC_IN_SEL_CFG = crate::Reg<u32, _FUNC_IN_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC_IN_SEL_CFG;
#[doc = "`read()` method returns [func_in_sel_cfg::R](func_in_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC_IN_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func_in_sel_cfg::W](func_in_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC_IN_SEL_CFG {}
#[doc = "GPIO_FUNC0_IN_SEL_CFG"]
pub mod func_in_sel_cfg;
#[doc = "GPIO_FUNC0_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [func_out_sel_cfg](func_out_sel_cfg) module"]
pub type FUNC_OUT_SEL_CFG = crate::Reg<u32, _FUNC_OUT_SEL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNC_OUT_SEL_CFG;
#[doc = "`read()` method returns [func_out_sel_cfg::R](func_out_sel_cfg::R) reader structure"]
impl crate::Readable for FUNC_OUT_SEL_CFG {}
#[doc = "`write(|w| ..)` method takes [func_out_sel_cfg::W](func_out_sel_cfg::W) writer structure"]
impl crate::Writable for FUNC_OUT_SEL_CFG {}
#[doc = "GPIO_FUNC0_OUT_SEL_CFG"]
pub mod func_out_sel_cfg;
#[doc = "GPIO_CLOCK_GATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_gate](clock_gate) module"]
pub type CLOCK_GATE = crate::Reg<u32, _CLOCK_GATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCK_GATE;
#[doc = "`read()` method returns [clock_gate::R](clock_gate::R) reader structure"]
impl crate::Readable for CLOCK_GATE {}
#[doc = "`write(|w| ..)` method takes [clock_gate::W](clock_gate::W) writer structure"]
impl crate::Writable for CLOCK_GATE {}
#[doc = "GPIO_CLOCK_GATE"]
pub mod clock_gate;
#[doc = "GPIO_DATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "GPIO_DATE"]
pub mod date;
