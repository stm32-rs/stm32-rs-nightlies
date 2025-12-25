#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    power: POWER,
    clkcr: CLKCR,
    arg: ARG,
    cmd: CMD,
    respcmd: RESPCMD,
    resp1: RESP1,
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
    ///0x00 - power control register
    #[inline(always)]
    pub const fn power(&self) -> &POWER {
        &self.power
    }
    ///0x04 - SDI clock control register
    #[inline(always)]
    pub const fn clkcr(&self) -> &CLKCR {
        &self.clkcr
    }
    ///0x08 - argument register
    #[inline(always)]
    pub const fn arg(&self) -> &ARG {
        &self.arg
    }
    ///0x0c - command register
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    ///0x10 - command response register
    #[inline(always)]
    pub const fn respcmd(&self) -> &RESPCMD {
        &self.respcmd
    }
    ///0x14 - response 1..4 register
    #[inline(always)]
    pub const fn resp1(&self) -> &RESP1 {
        &self.resp1
    }
    ///0x18 - response 1..4 register
    #[inline(always)]
    pub const fn resp2(&self) -> &RESP2 {
        &self.resp2
    }
    ///0x1c - response 1..4 register
    #[inline(always)]
    pub const fn resp3(&self) -> &RESP3 {
        &self.resp3
    }
    ///0x20 - response 1..4 register
    #[inline(always)]
    pub const fn resp4(&self) -> &RESP4 {
        &self.resp4
    }
    ///0x24 - data timer register
    #[inline(always)]
    pub const fn dtimer(&self) -> &DTIMER {
        &self.dtimer
    }
    ///0x28 - data length register
    #[inline(always)]
    pub const fn dlen(&self) -> &DLEN {
        &self.dlen
    }
    ///0x2c - data control register
    #[inline(always)]
    pub const fn dctrl(&self) -> &DCTRL {
        &self.dctrl
    }
    ///0x30 - data counter register
    #[inline(always)]
    pub const fn dcount(&self) -> &DCOUNT {
        &self.dcount
    }
    ///0x34 - status register
    #[inline(always)]
    pub const fn sta(&self) -> &STA {
        &self.sta
    }
    ///0x38 - interrupt clear register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x3c - mask register
    #[inline(always)]
    pub const fn mask(&self) -> &MASK {
        &self.mask
    }
    ///0x48 - FIFO counter register
    #[inline(always)]
    pub const fn fifocnt(&self) -> &FIFOCNT {
        &self.fifocnt
    }
    ///0x80 - data FIFO register
    #[inline(always)]
    pub const fn fifo(&self) -> &FIFO {
        &self.fifo
    }
}
/**POWER (rw) register accessor: power control register

You can [`read`](crate::Reg::read) this register and get [`power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#SDIO:POWER)

For information about available fields see [`mod@power`] module*/
pub type POWER = crate::Reg<power::POWERrs>;
///power control register
pub mod power;
/**CLKCR (rw) register accessor: SDI clock control register

You can [`read`](crate::Reg::read) this register and get [`clkcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#SDIO:CLKCR)

For information about available fields see [`mod@clkcr`] module*/
pub type CLKCR = crate::Reg<clkcr::CLKCRrs>;
///SDI clock control register
pub mod clkcr;
/**ARG (rw) register accessor: argument register

You can [`read`](crate::Reg::read) this register and get [`arg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#SDIO:ARG)

For information about available fields see [`mod@arg`] module*/
pub type ARG = crate::Reg<arg::ARGrs>;
///argument register
pub mod arg;
/**CMD (rw) register accessor: command register

You can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#SDIO:CMD)

For information about available fields see [`mod@cmd`] module*/
pub type CMD = crate::Reg<cmd::CMDrs>;
///command register
pub mod cmd;
/**RESPCMD (r) register accessor: command response register

You can [`read`](crate::Reg::read) this register and get [`respcmd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#SDIO:RESPCMD)

For information about available fields see [`mod@respcmd`] module*/
pub type RESPCMD = crate::Reg<respcmd::RESPCMDrs>;
///command response register
pub mod respcmd;
/**RESP1 (r) register accessor: response 1..4 register

You can [`read`](crate::Reg::read) this register and get [`resp1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#SDIO:RESP1)

For information about available fields see [`mod@resp1`] module*/
pub type RESP1 = crate::Reg<resp1::RESP1rs>;
///response 1..4 register
pub mod resp1;
/**RESP2 (r) register accessor: response 1..4 register

You can [`read`](crate::Reg::read) this register and get [`resp2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#SDIO:RESP2)

For information about available fields see [`mod@resp2`] module*/
pub type RESP2 = crate::Reg<resp2::RESP2rs>;
///response 1..4 register
pub mod resp2;
/**RESP3 (r) register accessor: response 1..4 register

You can [`read`](crate::Reg::read) this register and get [`resp3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#SDIO:RESP3)

For information about available fields see [`mod@resp3`] module*/
pub type RESP3 = crate::Reg<resp3::RESP3rs>;
///response 1..4 register
pub mod resp3;
/**RESP4 (r) register accessor: response 1..4 register

You can [`read`](crate::Reg::read) this register and get [`resp4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#SDIO:RESP4)

For information about available fields see [`mod@resp4`] module*/
pub type RESP4 = crate::Reg<resp4::RESP4rs>;
///response 1..4 register
pub mod resp4;
/**DTIMER (rw) register accessor: data timer register

You can [`read`](crate::Reg::read) this register and get [`dtimer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtimer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#SDIO:DTIMER)

For information about available fields see [`mod@dtimer`] module*/
pub type DTIMER = crate::Reg<dtimer::DTIMERrs>;
///data timer register
pub mod dtimer;
/**DLEN (rw) register accessor: data length register

You can [`read`](crate::Reg::read) this register and get [`dlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#SDIO:DLEN)

For information about available fields see [`mod@dlen`] module*/
pub type DLEN = crate::Reg<dlen::DLENrs>;
///data length register
pub mod dlen;
/**DCTRL (rw) register accessor: data control register

You can [`read`](crate::Reg::read) this register and get [`dctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#SDIO:DCTRL)

For information about available fields see [`mod@dctrl`] module*/
pub type DCTRL = crate::Reg<dctrl::DCTRLrs>;
///data control register
pub mod dctrl;
/**DCOUNT (r) register accessor: data counter register

You can [`read`](crate::Reg::read) this register and get [`dcount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#SDIO:DCOUNT)

For information about available fields see [`mod@dcount`] module*/
pub type DCOUNT = crate::Reg<dcount::DCOUNTrs>;
///data counter register
pub mod dcount;
/**STA (r) register accessor: status register

You can [`read`](crate::Reg::read) this register and get [`sta::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#SDIO:STA)

For information about available fields see [`mod@sta`] module*/
pub type STA = crate::Reg<sta::STArs>;
///status register
pub mod sta;
/**ICR (rw) register accessor: interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#SDIO:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///interrupt clear register
pub mod icr;
/**MASK (rw) register accessor: mask register

You can [`read`](crate::Reg::read) this register and get [`mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#SDIO:MASK)

For information about available fields see [`mod@mask`] module*/
pub type MASK = crate::Reg<mask::MASKrs>;
///mask register
pub mod mask;
/**FIFOCNT (r) register accessor: FIFO counter register

You can [`read`](crate::Reg::read) this register and get [`fifocnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#SDIO:FIFOCNT)

For information about available fields see [`mod@fifocnt`] module*/
pub type FIFOCNT = crate::Reg<fifocnt::FIFOCNTrs>;
///FIFO counter register
pub mod fifocnt;
/**FIFO (rw) register accessor: data FIFO register

You can [`read`](crate::Reg::read) this register and get [`fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#SDIO:FIFO)

For information about available fields see [`mod@fifo`] module*/
pub type FIFO = crate::Reg<fifo::FIFOrs>;
///data FIFO register
pub mod fifo;
