#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - configures clock source and clock output pins"]
    pub pin_ctrl: PIN_CTRL,
    #[doc = "0x04 - configures IO_MUX for GPIO0"]
    pub gpio0: GPIO0,
    #[doc = "0x08 - configures IO_MUX for GPIO1"]
    pub gpio1: GPIO1,
    #[doc = "0x0c - configures IO_MUX for GPIO2"]
    pub gpio2: GPIO2,
    #[doc = "0x10 - configures IO_MUX for GPIO3"]
    pub gpio3: GPIO3,
    #[doc = "0x14 - configures IO_MUX for MTMS"]
    pub mtms: MTMS,
    #[doc = "0x18 - configures IO_MUX for MTDI"]
    pub mtdi: MTDI,
    #[doc = "0x1c - configures IO_MUX for MTCK"]
    pub mtck: MTCK,
    #[doc = "0x20 - configures IO_MUX for MTDO"]
    pub mtdo: MTDO,
    #[doc = "0x24 - configures IO_MUX for GPIO8"]
    pub gpio8: GPIO8,
    #[doc = "0x28 - configures IO_MUX for GPIO9"]
    pub gpio9: GPIO9,
    #[doc = "0x2c - configures IO_MUX for GPIO10"]
    pub gpio10: GPIO10,
    #[doc = "0x30 - configures IO_MUX for VDD_SPI"]
    pub vdd_spi: VDD_SPI,
    #[doc = "0x34 - configures IO_MUX for SPIHD"]
    pub spihd: SPIHD,
    #[doc = "0x38 - configures IO_MUX for SPIWP"]
    pub spiwp: SPIWP,
    #[doc = "0x3c - configures IO_MUX for SPICS0"]
    pub spics0: SPICS0,
    #[doc = "0x40 - configures IO_MUX for SPICLK"]
    pub spiclk: SPICLK,
    #[doc = "0x44 - configures IO_MUX for SPID"]
    pub spid: SPID,
    #[doc = "0x48 - configures IO_MUX for SPIQ"]
    pub spiq: SPIQ,
    #[doc = "0x4c - configures IO_MUX for GPIO18"]
    pub gpio18: GPIO18,
    #[doc = "0x50 - configures IO_MUX for GPIO19"]
    pub gpio19: GPIO19,
    #[doc = "0x54 - configures IO_MUX for U0RXD"]
    pub u0rxd: U0RXD,
    #[doc = "0x58 - configures IO_MUX for U0TXD"]
    pub u0txd: U0TXD,
}
#[doc = "configures clock source and clock output pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin_ctrl](pin_ctrl) module"]
pub type PIN_CTRL = crate::Reg<u32, _PIN_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN_CTRL;
#[doc = "`read()` method returns [pin_ctrl::R](pin_ctrl::R) reader structure"]
impl crate::Readable for PIN_CTRL {}
#[doc = "`write(|w| ..)` method takes [pin_ctrl::W](pin_ctrl::W) writer structure"]
impl crate::Writable for PIN_CTRL {}
#[doc = "configures clock source and clock output pins"]
pub mod pin_ctrl;
#[doc = "configures IO_MUX for GPIO0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio0](gpio0) module"]
pub type GPIO0 = crate::Reg<u32, _GPIO0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO0;
#[doc = "`read()` method returns [gpio0::R](gpio0::R) reader structure"]
impl crate::Readable for GPIO0 {}
#[doc = "`write(|w| ..)` method takes [gpio0::W](gpio0::W) writer structure"]
impl crate::Writable for GPIO0 {}
#[doc = "configures IO_MUX for GPIO0"]
pub mod gpio0;
#[doc = "configures IO_MUX for GPIO1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio1](gpio1) module"]
pub type GPIO1 = crate::Reg<u32, _GPIO1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO1;
#[doc = "`read()` method returns [gpio1::R](gpio1::R) reader structure"]
impl crate::Readable for GPIO1 {}
#[doc = "`write(|w| ..)` method takes [gpio1::W](gpio1::W) writer structure"]
impl crate::Writable for GPIO1 {}
#[doc = "configures IO_MUX for GPIO1"]
pub mod gpio1;
#[doc = "configures IO_MUX for GPIO2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio2](gpio2) module"]
pub type GPIO2 = crate::Reg<u32, _GPIO2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO2;
#[doc = "`read()` method returns [gpio2::R](gpio2::R) reader structure"]
impl crate::Readable for GPIO2 {}
#[doc = "`write(|w| ..)` method takes [gpio2::W](gpio2::W) writer structure"]
impl crate::Writable for GPIO2 {}
#[doc = "configures IO_MUX for GPIO2"]
pub mod gpio2;
#[doc = "configures IO_MUX for GPIO3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio3](gpio3) module"]
pub type GPIO3 = crate::Reg<u32, _GPIO3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO3;
#[doc = "`read()` method returns [gpio3::R](gpio3::R) reader structure"]
impl crate::Readable for GPIO3 {}
#[doc = "`write(|w| ..)` method takes [gpio3::W](gpio3::W) writer structure"]
impl crate::Writable for GPIO3 {}
#[doc = "configures IO_MUX for GPIO3"]
pub mod gpio3;
#[doc = "configures IO_MUX for MTMS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtms](mtms) module"]
pub type MTMS = crate::Reg<u32, _MTMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTMS;
#[doc = "`read()` method returns [mtms::R](mtms::R) reader structure"]
impl crate::Readable for MTMS {}
#[doc = "`write(|w| ..)` method takes [mtms::W](mtms::W) writer structure"]
impl crate::Writable for MTMS {}
#[doc = "configures IO_MUX for MTMS"]
pub mod mtms;
#[doc = "configures IO_MUX for MTDI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtdi](mtdi) module"]
pub type MTDI = crate::Reg<u32, _MTDI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTDI;
#[doc = "`read()` method returns [mtdi::R](mtdi::R) reader structure"]
impl crate::Readable for MTDI {}
#[doc = "`write(|w| ..)` method takes [mtdi::W](mtdi::W) writer structure"]
impl crate::Writable for MTDI {}
#[doc = "configures IO_MUX for MTDI"]
pub mod mtdi;
#[doc = "configures IO_MUX for MTCK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtck](mtck) module"]
pub type MTCK = crate::Reg<u32, _MTCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTCK;
#[doc = "`read()` method returns [mtck::R](mtck::R) reader structure"]
impl crate::Readable for MTCK {}
#[doc = "`write(|w| ..)` method takes [mtck::W](mtck::W) writer structure"]
impl crate::Writable for MTCK {}
#[doc = "configures IO_MUX for MTCK"]
pub mod mtck;
#[doc = "configures IO_MUX for MTDO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtdo](mtdo) module"]
pub type MTDO = crate::Reg<u32, _MTDO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTDO;
#[doc = "`read()` method returns [mtdo::R](mtdo::R) reader structure"]
impl crate::Readable for MTDO {}
#[doc = "`write(|w| ..)` method takes [mtdo::W](mtdo::W) writer structure"]
impl crate::Writable for MTDO {}
#[doc = "configures IO_MUX for MTDO"]
pub mod mtdo;
#[doc = "configures IO_MUX for GPIO8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio8](gpio8) module"]
pub type GPIO8 = crate::Reg<u32, _GPIO8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO8;
#[doc = "`read()` method returns [gpio8::R](gpio8::R) reader structure"]
impl crate::Readable for GPIO8 {}
#[doc = "`write(|w| ..)` method takes [gpio8::W](gpio8::W) writer structure"]
impl crate::Writable for GPIO8 {}
#[doc = "configures IO_MUX for GPIO8"]
pub mod gpio8;
#[doc = "configures IO_MUX for GPIO9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio9](gpio9) module"]
pub type GPIO9 = crate::Reg<u32, _GPIO9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO9;
#[doc = "`read()` method returns [gpio9::R](gpio9::R) reader structure"]
impl crate::Readable for GPIO9 {}
#[doc = "`write(|w| ..)` method takes [gpio9::W](gpio9::W) writer structure"]
impl crate::Writable for GPIO9 {}
#[doc = "configures IO_MUX for GPIO9"]
pub mod gpio9;
#[doc = "configures IO_MUX for GPIO10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio10](gpio10) module"]
pub type GPIO10 = crate::Reg<u32, _GPIO10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO10;
#[doc = "`read()` method returns [gpio10::R](gpio10::R) reader structure"]
impl crate::Readable for GPIO10 {}
#[doc = "`write(|w| ..)` method takes [gpio10::W](gpio10::W) writer structure"]
impl crate::Writable for GPIO10 {}
#[doc = "configures IO_MUX for GPIO10"]
pub mod gpio10;
#[doc = "configures IO_MUX for VDD_SPI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdd_spi](vdd_spi) module"]
pub type VDD_SPI = crate::Reg<u32, _VDD_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VDD_SPI;
#[doc = "`read()` method returns [vdd_spi::R](vdd_spi::R) reader structure"]
impl crate::Readable for VDD_SPI {}
#[doc = "`write(|w| ..)` method takes [vdd_spi::W](vdd_spi::W) writer structure"]
impl crate::Writable for VDD_SPI {}
#[doc = "configures IO_MUX for VDD_SPI"]
pub mod vdd_spi;
#[doc = "configures IO_MUX for SPIHD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spihd](spihd) module"]
pub type SPIHD = crate::Reg<u32, _SPIHD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPIHD;
#[doc = "`read()` method returns [spihd::R](spihd::R) reader structure"]
impl crate::Readable for SPIHD {}
#[doc = "`write(|w| ..)` method takes [spihd::W](spihd::W) writer structure"]
impl crate::Writable for SPIHD {}
#[doc = "configures IO_MUX for SPIHD"]
pub mod spihd;
#[doc = "configures IO_MUX for SPIWP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spiwp](spiwp) module"]
pub type SPIWP = crate::Reg<u32, _SPIWP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPIWP;
#[doc = "`read()` method returns [spiwp::R](spiwp::R) reader structure"]
impl crate::Readable for SPIWP {}
#[doc = "`write(|w| ..)` method takes [spiwp::W](spiwp::W) writer structure"]
impl crate::Writable for SPIWP {}
#[doc = "configures IO_MUX for SPIWP"]
pub mod spiwp;
#[doc = "configures IO_MUX for SPICS0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spics0](spics0) module"]
pub type SPICS0 = crate::Reg<u32, _SPICS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPICS0;
#[doc = "`read()` method returns [spics0::R](spics0::R) reader structure"]
impl crate::Readable for SPICS0 {}
#[doc = "`write(|w| ..)` method takes [spics0::W](spics0::W) writer structure"]
impl crate::Writable for SPICS0 {}
#[doc = "configures IO_MUX for SPICS0"]
pub mod spics0;
#[doc = "configures IO_MUX for SPICLK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spiclk](spiclk) module"]
pub type SPICLK = crate::Reg<u32, _SPICLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPICLK;
#[doc = "`read()` method returns [spiclk::R](spiclk::R) reader structure"]
impl crate::Readable for SPICLK {}
#[doc = "`write(|w| ..)` method takes [spiclk::W](spiclk::W) writer structure"]
impl crate::Writable for SPICLK {}
#[doc = "configures IO_MUX for SPICLK"]
pub mod spiclk;
#[doc = "configures IO_MUX for SPID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spid](spid) module"]
pub type SPID = crate::Reg<u32, _SPID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPID;
#[doc = "`read()` method returns [spid::R](spid::R) reader structure"]
impl crate::Readable for SPID {}
#[doc = "`write(|w| ..)` method takes [spid::W](spid::W) writer structure"]
impl crate::Writable for SPID {}
#[doc = "configures IO_MUX for SPID"]
pub mod spid;
#[doc = "configures IO_MUX for SPIQ\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spiq](spiq) module"]
pub type SPIQ = crate::Reg<u32, _SPIQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPIQ;
#[doc = "`read()` method returns [spiq::R](spiq::R) reader structure"]
impl crate::Readable for SPIQ {}
#[doc = "`write(|w| ..)` method takes [spiq::W](spiq::W) writer structure"]
impl crate::Writable for SPIQ {}
#[doc = "configures IO_MUX for SPIQ"]
pub mod spiq;
#[doc = "configures IO_MUX for GPIO18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio18](gpio18) module"]
pub type GPIO18 = crate::Reg<u32, _GPIO18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO18;
#[doc = "`read()` method returns [gpio18::R](gpio18::R) reader structure"]
impl crate::Readable for GPIO18 {}
#[doc = "`write(|w| ..)` method takes [gpio18::W](gpio18::W) writer structure"]
impl crate::Writable for GPIO18 {}
#[doc = "configures IO_MUX for GPIO18"]
pub mod gpio18;
#[doc = "configures IO_MUX for GPIO19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio19](gpio19) module"]
pub type GPIO19 = crate::Reg<u32, _GPIO19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO19;
#[doc = "`read()` method returns [gpio19::R](gpio19::R) reader structure"]
impl crate::Readable for GPIO19 {}
#[doc = "`write(|w| ..)` method takes [gpio19::W](gpio19::W) writer structure"]
impl crate::Writable for GPIO19 {}
#[doc = "configures IO_MUX for GPIO19"]
pub mod gpio19;
#[doc = "configures IO_MUX for U0RXD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u0rxd](u0rxd) module"]
pub type U0RXD = crate::Reg<u32, _U0RXD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U0RXD;
#[doc = "`read()` method returns [u0rxd::R](u0rxd::R) reader structure"]
impl crate::Readable for U0RXD {}
#[doc = "`write(|w| ..)` method takes [u0rxd::W](u0rxd::W) writer structure"]
impl crate::Writable for U0RXD {}
#[doc = "configures IO_MUX for U0RXD"]
pub mod u0rxd;
#[doc = "configures IO_MUX for U0TXD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u0txd](u0txd) module"]
pub type U0TXD = crate::Reg<u32, _U0TXD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U0TXD;
#[doc = "`read()` method returns [u0txd::R](u0txd::R) reader structure"]
impl crate::Readable for U0TXD {}
#[doc = "`write(|w| ..)` method takes [u0txd::W](u0txd::W) writer structure"]
impl crate::Writable for U0TXD {}
#[doc = "configures IO_MUX for U0TXD"]
pub mod u0txd;
