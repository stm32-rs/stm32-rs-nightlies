#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    power: POWER,
    clkcr: CLKCR,
    arg: ARG,
    cmd: CMD,
    respcmd: RESPCMD,
    resp: [RESP; 4],
    dtimer: DTIMER,
    dlen: DLEN,
    dctrl: DCTRL,
    dcount: DCOUNT,
    sta: STA,
    icr: ICR,
    mask: MASK,
    _reserved13: [u8; 0x08],
    fifocnt: FIFOCNT,
    _reserved14: [u8; 0x34],
    fifo: FIFO,
}
impl RegisterBlock {
    #[doc = "0x00 - power control register"]
    #[inline(always)]
    pub const fn power(&self) -> &POWER {
        &self.power
    }
    #[doc = "0x04 - SDI clock control register"]
    #[inline(always)]
    pub const fn clkcr(&self) -> &CLKCR {
        &self.clkcr
    }
    #[doc = "0x08 - argument register"]
    #[inline(always)]
    pub const fn arg(&self) -> &ARG {
        &self.arg
    }
    #[doc = "0x0c - command register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x10 - command response register"]
    #[inline(always)]
    pub const fn respcmd(&self) -> &RESPCMD {
        &self.respcmd
    }
    #[doc = "0x14..0x24 - SDIO response %s register"]
    #[inline(always)]
    pub const fn resp(&self, n: usize) -> &RESP {
        &self.resp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x14..0x24 - SDIO response %s register"]
    #[inline(always)]
    pub fn resp_iter(&self) -> impl Iterator<Item = &RESP> {
        self.resp.iter()
    }
    #[doc = "0x14 - SDIO response 1 register"]
    #[inline(always)]
    pub const fn resp1(&self) -> &RESP {
        self.resp(0)
    }
    #[doc = "0x18 - SDIO response 2 register"]
    #[inline(always)]
    pub const fn resp2(&self) -> &RESP {
        self.resp(1)
    }
    #[doc = "0x1c - SDIO response 3 register"]
    #[inline(always)]
    pub const fn resp3(&self) -> &RESP {
        self.resp(2)
    }
    #[doc = "0x20 - SDIO response 4 register"]
    #[inline(always)]
    pub const fn resp4(&self) -> &RESP {
        self.resp(3)
    }
    #[doc = "0x24 - data timer register"]
    #[inline(always)]
    pub const fn dtimer(&self) -> &DTIMER {
        &self.dtimer
    }
    #[doc = "0x28 - data length register"]
    #[inline(always)]
    pub const fn dlen(&self) -> &DLEN {
        &self.dlen
    }
    #[doc = "0x2c - data control register"]
    #[inline(always)]
    pub const fn dctrl(&self) -> &DCTRL {
        &self.dctrl
    }
    #[doc = "0x30 - data counter register"]
    #[inline(always)]
    pub const fn dcount(&self) -> &DCOUNT {
        &self.dcount
    }
    #[doc = "0x34 - status register"]
    #[inline(always)]
    pub const fn sta(&self) -> &STA {
        &self.sta
    }
    #[doc = "0x38 - interrupt clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x3c - mask register"]
    #[inline(always)]
    pub const fn mask(&self) -> &MASK {
        &self.mask
    }
    #[doc = "0x48 - FIFO counter register"]
    #[inline(always)]
    pub const fn fifocnt(&self) -> &FIFOCNT {
        &self.fifocnt
    }
    #[doc = "0x80 - data FIFO register"]
    #[inline(always)]
    pub const fn fifo(&self) -> &FIFO {
        &self.fifo
    }
}
#[doc = "POWER (rw) register accessor: power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power`]
module"]
pub type POWER = crate::Reg<power::POWERrs>;
#[doc = "power control register"]
pub mod power;
#[doc = "CLKCR (rw) register accessor: SDI clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcr`]
module"]
pub type CLKCR = crate::Reg<clkcr::CLKCRrs>;
#[doc = "SDI clock control register"]
pub mod clkcr;
#[doc = "ARG (rw) register accessor: argument register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arg`]
module"]
pub type ARG = crate::Reg<arg::ARGrs>;
#[doc = "argument register"]
pub mod arg;
#[doc = "CMD (rw) register accessor: command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMDrs>;
#[doc = "command register"]
pub mod cmd;
#[doc = "RESPCMD (r) register accessor: command response register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`respcmd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@respcmd`]
module"]
pub type RESPCMD = crate::Reg<respcmd::RESPCMDrs>;
#[doc = "command response register"]
pub mod respcmd;
#[doc = "RESP (r) register accessor: SDIO response %s register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp`]
module"]
pub type RESP = crate::Reg<resp::RESPrs>;
#[doc = "SDIO response %s register"]
pub mod resp;
#[doc = "DTIMER (rw) register accessor: data timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtimer`]
module"]
pub type DTIMER = crate::Reg<dtimer::DTIMERrs>;
#[doc = "data timer register"]
pub mod dtimer;
#[doc = "DLEN (rw) register accessor: data length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlen`]
module"]
pub type DLEN = crate::Reg<dlen::DLENrs>;
#[doc = "data length register"]
pub mod dlen;
#[doc = "DCTRL (rw) register accessor: data control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctrl`]
module"]
pub type DCTRL = crate::Reg<dctrl::DCTRLrs>;
#[doc = "data control register"]
pub mod dctrl;
#[doc = "DCOUNT (r) register accessor: data counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcount::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcount`]
module"]
pub type DCOUNT = crate::Reg<dcount::DCOUNTrs>;
#[doc = "data counter register"]
pub mod dcount;
#[doc = "STA (r) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sta::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sta`]
module"]
pub type STA = crate::Reg<sta::STArs>;
#[doc = "status register"]
pub mod sta;
#[doc = "ICR (rw) register accessor: interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "interrupt clear register"]
pub mod icr;
#[doc = "MASK (rw) register accessor: mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask`]
module"]
pub type MASK = crate::Reg<mask::MASKrs>;
#[doc = "mask register"]
pub mod mask;
#[doc = "FIFOCNT (r) register accessor: FIFO counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifocnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifocnt`]
module"]
pub type FIFOCNT = crate::Reg<fifocnt::FIFOCNTrs>;
#[doc = "FIFO counter register"]
pub mod fifocnt;
#[doc = "FIFO (rw) register accessor: data FIFO register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo`]
module"]
pub type FIFO = crate::Reg<fifo::FIFOrs>;
#[doc = "data FIFO register"]
pub mod fifo;
