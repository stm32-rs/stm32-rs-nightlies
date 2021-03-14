#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Bits 1:0 = PWRCTRL: Power supply control bits"]
    pub power: POWER,
    #[doc = "0x04 - SDI clock control register (SDIO_CLKCR)"]
    pub clkcr: CLKCR,
    #[doc = "0x08 - Bits 31:0 = : Command argument"]
    pub arg: ARG,
    #[doc = "0x0c - SDIO command register (SDIO_CMD)"]
    pub cmd: CMD,
    #[doc = "0x10 - SDIO command register"]
    pub respcmd: RESPCMD,
    #[doc = "0x14 - Bits 31:0 = CARDSTATUS1"]
    pub respi1: RESPI1,
    #[doc = "0x18 - Bits 31:0 = CARDSTATUS2"]
    pub resp2: RESP2,
    #[doc = "0x1c - Bits 31:0 = CARDSTATUS3"]
    pub resp3: RESP3,
    #[doc = "0x20 - Bits 31:0 = CARDSTATUS4"]
    pub resp4: RESP4,
    #[doc = "0x24 - Bits 31:0 = DATATIME: Data timeout period"]
    pub dtimer: DTIMER,
    #[doc = "0x28 - Bits 24:0 = DATALENGTH: Data length value"]
    pub dlen: DLEN,
    #[doc = "0x2c - SDIO data control register (SDIO_DCTRL)"]
    pub dctrl: DCTRL,
    #[doc = "0x30 - Bits 24:0 = DATACOUNT: Data count value"]
    pub dcount: DCOUNT,
    #[doc = "0x34 - SDIO status register (SDIO_STA)"]
    pub sta: STA,
    #[doc = "0x38 - SDIO interrupt clear register (SDIO_ICR)"]
    pub icr: ICR,
    #[doc = "0x3c - SDIO mask register (SDIO_MASK)"]
    pub mask: MASK,
    _reserved16: [u8; 8usize],
    #[doc = "0x48 - Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO"]
    pub fifocnt: FIFOCNT,
    _reserved17: [u8; 52usize],
    #[doc = "0x80 - bits 31:0 = FIFOData: Receive and transmit FIFO data"]
    pub fifo: FIFO,
}
#[doc = "Bits 1:0 = PWRCTRL: Power supply control bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power](power) module"]
pub type POWER = crate::Reg<u32, _POWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POWER;
#[doc = "`read()` method returns [power::R](power::R) reader structure"]
impl crate::Readable for POWER {}
#[doc = "`write(|w| ..)` method takes [power::W](power::W) writer structure"]
impl crate::Writable for POWER {}
#[doc = "Bits 1:0 = PWRCTRL: Power supply control bits"]
pub mod power;
#[doc = "SDI clock control register (SDIO_CLKCR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkcr](clkcr) module"]
pub type CLKCR = crate::Reg<u32, _CLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKCR;
#[doc = "`read()` method returns [clkcr::R](clkcr::R) reader structure"]
impl crate::Readable for CLKCR {}
#[doc = "`write(|w| ..)` method takes [clkcr::W](clkcr::W) writer structure"]
impl crate::Writable for CLKCR {}
#[doc = "SDI clock control register (SDIO_CLKCR)"]
pub mod clkcr;
#[doc = "Bits 31:0 = : Command argument\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arg](arg) module"]
pub type ARG = crate::Reg<u32, _ARG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARG;
#[doc = "`read()` method returns [arg::R](arg::R) reader structure"]
impl crate::Readable for ARG {}
#[doc = "`write(|w| ..)` method takes [arg::W](arg::W) writer structure"]
impl crate::Writable for ARG {}
#[doc = "Bits 31:0 = : Command argument"]
pub mod arg;
#[doc = "SDIO command register (SDIO_CMD)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`read()` method returns [cmd::R](cmd::R) reader structure"]
impl crate::Readable for CMD {}
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "SDIO command register (SDIO_CMD)"]
pub mod cmd;
#[doc = "SDIO command register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [respcmd](respcmd) module"]
pub type RESPCMD = crate::Reg<u32, _RESPCMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESPCMD;
#[doc = "`read()` method returns [respcmd::R](respcmd::R) reader structure"]
impl crate::Readable for RESPCMD {}
#[doc = "SDIO command register"]
pub mod respcmd;
#[doc = "Bits 31:0 = CARDSTATUS1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [respi1](respi1) module"]
pub type RESPI1 = crate::Reg<u32, _RESPI1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESPI1;
#[doc = "`read()` method returns [respi1::R](respi1::R) reader structure"]
impl crate::Readable for RESPI1 {}
#[doc = "Bits 31:0 = CARDSTATUS1"]
pub mod respi1;
#[doc = "Bits 31:0 = CARDSTATUS2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp2](resp2) module"]
pub type RESP2 = crate::Reg<u32, _RESP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP2;
#[doc = "`read()` method returns [resp2::R](resp2::R) reader structure"]
impl crate::Readable for RESP2 {}
#[doc = "Bits 31:0 = CARDSTATUS2"]
pub mod resp2;
#[doc = "Bits 31:0 = CARDSTATUS3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp3](resp3) module"]
pub type RESP3 = crate::Reg<u32, _RESP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP3;
#[doc = "`read()` method returns [resp3::R](resp3::R) reader structure"]
impl crate::Readable for RESP3 {}
#[doc = "Bits 31:0 = CARDSTATUS3"]
pub mod resp3;
#[doc = "Bits 31:0 = CARDSTATUS4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp4](resp4) module"]
pub type RESP4 = crate::Reg<u32, _RESP4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP4;
#[doc = "`read()` method returns [resp4::R](resp4::R) reader structure"]
impl crate::Readable for RESP4 {}
#[doc = "Bits 31:0 = CARDSTATUS4"]
pub mod resp4;
#[doc = "Bits 31:0 = DATATIME: Data timeout period\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtimer](dtimer) module"]
pub type DTIMER = crate::Reg<u32, _DTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTIMER;
#[doc = "`read()` method returns [dtimer::R](dtimer::R) reader structure"]
impl crate::Readable for DTIMER {}
#[doc = "`write(|w| ..)` method takes [dtimer::W](dtimer::W) writer structure"]
impl crate::Writable for DTIMER {}
#[doc = "Bits 31:0 = DATATIME: Data timeout period"]
pub mod dtimer;
#[doc = "Bits 24:0 = DATALENGTH: Data length value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlen](dlen) module"]
pub type DLEN = crate::Reg<u32, _DLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLEN;
#[doc = "`read()` method returns [dlen::R](dlen::R) reader structure"]
impl crate::Readable for DLEN {}
#[doc = "`write(|w| ..)` method takes [dlen::W](dlen::W) writer structure"]
impl crate::Writable for DLEN {}
#[doc = "Bits 24:0 = DATALENGTH: Data length value"]
pub mod dlen;
#[doc = "SDIO data control register (SDIO_DCTRL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dctrl](dctrl) module"]
pub type DCTRL = crate::Reg<u32, _DCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCTRL;
#[doc = "`read()` method returns [dctrl::R](dctrl::R) reader structure"]
impl crate::Readable for DCTRL {}
#[doc = "`write(|w| ..)` method takes [dctrl::W](dctrl::W) writer structure"]
impl crate::Writable for DCTRL {}
#[doc = "SDIO data control register (SDIO_DCTRL)"]
pub mod dctrl;
#[doc = "Bits 24:0 = DATACOUNT: Data count value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcount](dcount) module"]
pub type DCOUNT = crate::Reg<u32, _DCOUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCOUNT;
#[doc = "`read()` method returns [dcount::R](dcount::R) reader structure"]
impl crate::Readable for DCOUNT {}
#[doc = "Bits 24:0 = DATACOUNT: Data count value"]
pub mod dcount;
#[doc = "SDIO status register (SDIO_STA)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sta](sta) module"]
pub type STA = crate::Reg<u32, _STA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STA;
#[doc = "`read()` method returns [sta::R](sta::R) reader structure"]
impl crate::Readable for STA {}
#[doc = "SDIO status register (SDIO_STA)"]
pub mod sta;
#[doc = "SDIO interrupt clear register (SDIO_ICR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`read()` method returns [icr::R](icr::R) reader structure"]
impl crate::Readable for ICR {}
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "SDIO interrupt clear register (SDIO_ICR)"]
pub mod icr;
#[doc = "SDIO mask register (SDIO_MASK)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask](mask) module"]
pub type MASK = crate::Reg<u32, _MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK;
#[doc = "`read()` method returns [mask::R](mask::R) reader structure"]
impl crate::Readable for MASK {}
#[doc = "`write(|w| ..)` method takes [mask::W](mask::W) writer structure"]
impl crate::Writable for MASK {}
#[doc = "SDIO mask register (SDIO_MASK)"]
pub mod mask;
#[doc = "Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifocnt](fifocnt) module"]
pub type FIFOCNT = crate::Reg<u32, _FIFOCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOCNT;
#[doc = "`read()` method returns [fifocnt::R](fifocnt::R) reader structure"]
impl crate::Readable for FIFOCNT {}
#[doc = "Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO"]
pub mod fifocnt;
#[doc = "bits 31:0 = FIFOData: Receive and transmit FIFO data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo](fifo) module"]
pub type FIFO = crate::Reg<u32, _FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO;
#[doc = "`read()` method returns [fifo::R](fifo::R) reader structure"]
impl crate::Readable for FIFO {}
#[doc = "`write(|w| ..)` method takes [fifo::W](fifo::W) writer structure"]
impl crate::Writable for FIFO {}
#[doc = "bits 31:0 = FIFOData: Receive and transmit FIFO data"]
pub mod fifo;
