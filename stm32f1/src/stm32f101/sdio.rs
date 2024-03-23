#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    power: POWER,
    clkcr: CLKCR,
    arg: ARG,
    cmd: CMD,
    respcmd: RESPCMD,
    respi1: RESPI1,
    resp2: RESP2,
    resp3: RESP3,
    resp4: RESP4,
    dtimer: DTIMER,
    dlen: DLEN,
    dctrl: DCTRL,
    dcount: DCOUNT,
    sta: STA,
    icr: ICR,
    mask: MASK,
    _reserved16: [u8; 0x08],
    fifocnt: FIFOCNT,
    _reserved17: [u8; 0x34],
    fifo: FIFO,
}
impl RegisterBlock {
    #[doc = "0x00 - Bits 1:0 = PWRCTRL: Power supply control bits"]
    #[inline(always)]
    pub const fn power(&self) -> &POWER {
        &self.power
    }
    #[doc = "0x04 - SDI clock control register (SDIO_CLKCR)"]
    #[inline(always)]
    pub const fn clkcr(&self) -> &CLKCR {
        &self.clkcr
    }
    #[doc = "0x08 - Bits 31:0 = : Command argument"]
    #[inline(always)]
    pub const fn arg(&self) -> &ARG {
        &self.arg
    }
    #[doc = "0x0c - SDIO command register (SDIO_CMD)"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x10 - SDIO command register"]
    #[inline(always)]
    pub const fn respcmd(&self) -> &RESPCMD {
        &self.respcmd
    }
    #[doc = "0x14 - Bits 31:0 = CARDSTATUS1"]
    #[inline(always)]
    pub const fn respi1(&self) -> &RESPI1 {
        &self.respi1
    }
    #[doc = "0x18 - Bits 31:0 = CARDSTATUS2"]
    #[inline(always)]
    pub const fn resp2(&self) -> &RESP2 {
        &self.resp2
    }
    #[doc = "0x1c - Bits 31:0 = CARDSTATUS3"]
    #[inline(always)]
    pub const fn resp3(&self) -> &RESP3 {
        &self.resp3
    }
    #[doc = "0x20 - Bits 31:0 = CARDSTATUS4"]
    #[inline(always)]
    pub const fn resp4(&self) -> &RESP4 {
        &self.resp4
    }
    #[doc = "0x24 - Bits 31:0 = DATATIME: Data timeout period"]
    #[inline(always)]
    pub const fn dtimer(&self) -> &DTIMER {
        &self.dtimer
    }
    #[doc = "0x28 - Bits 24:0 = DATALENGTH: Data length value"]
    #[inline(always)]
    pub const fn dlen(&self) -> &DLEN {
        &self.dlen
    }
    #[doc = "0x2c - SDIO data control register (SDIO_DCTRL)"]
    #[inline(always)]
    pub const fn dctrl(&self) -> &DCTRL {
        &self.dctrl
    }
    #[doc = "0x30 - Bits 24:0 = DATACOUNT: Data count value"]
    #[inline(always)]
    pub const fn dcount(&self) -> &DCOUNT {
        &self.dcount
    }
    #[doc = "0x34 - SDIO status register (SDIO_STA)"]
    #[inline(always)]
    pub const fn sta(&self) -> &STA {
        &self.sta
    }
    #[doc = "0x38 - SDIO interrupt clear register (SDIO_ICR)"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x3c - SDIO mask register (SDIO_MASK)"]
    #[inline(always)]
    pub const fn mask(&self) -> &MASK {
        &self.mask
    }
    #[doc = "0x48 - Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO"]
    #[inline(always)]
    pub const fn fifocnt(&self) -> &FIFOCNT {
        &self.fifocnt
    }
    #[doc = "0x80 - bits 31:0 = FIFOData: Receive and transmit FIFO data"]
    #[inline(always)]
    pub const fn fifo(&self) -> &FIFO {
        &self.fifo
    }
}
#[doc = "POWER (rw) register accessor: Bits 1:0 = PWRCTRL: Power supply control bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power`]
module"]
pub type POWER = crate::Reg<power::POWERrs>;
#[doc = "Bits 1:0 = PWRCTRL: Power supply control bits"]
pub mod power;
#[doc = "CLKCR (rw) register accessor: SDI clock control register (SDIO_CLKCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcr`]
module"]
pub type CLKCR = crate::Reg<clkcr::CLKCRrs>;
#[doc = "SDI clock control register (SDIO_CLKCR)"]
pub mod clkcr;
#[doc = "ARG (rw) register accessor: Bits 31:0 = : Command argument\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arg`]
module"]
pub type ARG = crate::Reg<arg::ARGrs>;
#[doc = "Bits 31:0 = : Command argument"]
pub mod arg;
#[doc = "CMD (rw) register accessor: SDIO command register (SDIO_CMD)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMDrs>;
#[doc = "SDIO command register (SDIO_CMD)"]
pub mod cmd;
#[doc = "RESPCMD (r) register accessor: SDIO command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`respcmd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@respcmd`]
module"]
pub type RESPCMD = crate::Reg<respcmd::RESPCMDrs>;
#[doc = "SDIO command register"]
pub mod respcmd;
#[doc = "RESPI1 (r) register accessor: Bits 31:0 = CARDSTATUS1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`respi1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@respi1`]
module"]
pub type RESPI1 = crate::Reg<respi1::RESPI1rs>;
#[doc = "Bits 31:0 = CARDSTATUS1"]
pub mod respi1;
#[doc = "RESP2 (r) register accessor: Bits 31:0 = CARDSTATUS2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp2`]
module"]
pub type RESP2 = crate::Reg<resp2::RESP2rs>;
#[doc = "Bits 31:0 = CARDSTATUS2"]
pub mod resp2;
#[doc = "RESP3 (r) register accessor: Bits 31:0 = CARDSTATUS3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp3`]
module"]
pub type RESP3 = crate::Reg<resp3::RESP3rs>;
#[doc = "Bits 31:0 = CARDSTATUS3"]
pub mod resp3;
#[doc = "RESP4 (r) register accessor: Bits 31:0 = CARDSTATUS4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp4`]
module"]
pub type RESP4 = crate::Reg<resp4::RESP4rs>;
#[doc = "Bits 31:0 = CARDSTATUS4"]
pub mod resp4;
#[doc = "DTIMER (rw) register accessor: Bits 31:0 = DATATIME: Data timeout period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtimer`]
module"]
pub type DTIMER = crate::Reg<dtimer::DTIMERrs>;
#[doc = "Bits 31:0 = DATATIME: Data timeout period"]
pub mod dtimer;
#[doc = "DLEN (rw) register accessor: Bits 24:0 = DATALENGTH: Data length value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlen`]
module"]
pub type DLEN = crate::Reg<dlen::DLENrs>;
#[doc = "Bits 24:0 = DATALENGTH: Data length value"]
pub mod dlen;
#[doc = "DCTRL (rw) register accessor: SDIO data control register (SDIO_DCTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctrl`]
module"]
pub type DCTRL = crate::Reg<dctrl::DCTRLrs>;
#[doc = "SDIO data control register (SDIO_DCTRL)"]
pub mod dctrl;
#[doc = "DCOUNT (r) register accessor: Bits 24:0 = DATACOUNT: Data count value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcount::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcount`]
module"]
pub type DCOUNT = crate::Reg<dcount::DCOUNTrs>;
#[doc = "Bits 24:0 = DATACOUNT: Data count value"]
pub mod dcount;
#[doc = "STA (r) register accessor: SDIO status register (SDIO_STA)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sta::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sta`]
module"]
pub type STA = crate::Reg<sta::STArs>;
#[doc = "SDIO status register (SDIO_STA)"]
pub mod sta;
#[doc = "ICR (rw) register accessor: SDIO interrupt clear register (SDIO_ICR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "SDIO interrupt clear register (SDIO_ICR)"]
pub mod icr;
#[doc = "MASK (rw) register accessor: SDIO mask register (SDIO_MASK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask`]
module"]
pub type MASK = crate::Reg<mask::MASKrs>;
#[doc = "SDIO mask register (SDIO_MASK)"]
pub mod mask;
#[doc = "FIFOCNT (r) register accessor: Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifocnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifocnt`]
module"]
pub type FIFOCNT = crate::Reg<fifocnt::FIFOCNTrs>;
#[doc = "Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO"]
pub mod fifocnt;
#[doc = "FIFO (rw) register accessor: bits 31:0 = FIFOData: Receive and transmit FIFO data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo`]
module"]
pub type FIFO = crate::Reg<fifo::FIFOrs>;
#[doc = "bits 31:0 = FIFOData: Receive and transmit FIFO data"]
pub mod fifo;
