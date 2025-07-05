#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ipgr1: IPGR1,
    ipgr2: IPGR2,
    ipgr3: IPGR3,
    _reserved3: [u8; 0x10],
    ipgr8: IPGR8,
    ipc1r1: IPC1R1,
    ipc1r2: IPC1R2,
    ipc1r3: IPC1R3,
    _reserved7: [u8; 0xd8],
    prcr: PRCR,
    prescr: PRESCR,
    presur: PRESUR,
    _reserved10: [u8; 0xe4],
    prier: PRIER,
    prsr: PRSR,
    prfcr: PRFCR,
    _reserved13: [u8; 0x04],
    cmcr: CMCR,
    cmfrcr: CMFRCR,
    _reserved15: [u8; 0x01e4],
    cmier: CMIER,
    cmsr1: CMSR1,
    cmsr2: CMSR2,
    cmfcr: CMFCR,
    _reserved19: [u8; 0x04],
    p0fscr: P0FSCR,
    _reserved20: [u8; 0xf8],
    p0fctcr: P0FCTCR,
    p0scstr: P0SCSTR,
    p0scszr: P0SCSZR,
    _reserved23: [u8; 0xa4],
    p0dccntr: P0DCCNTR,
    p0dclmtr: P0DCLMTR,
    _reserved25: [u8; 0x08],
    p0ppcr: P0PPCR,
    p0ppm0ar1: P0PPM0AR1,
    p0ppm0ar2: P0PPM0AR2,
    _reserved28: [u8; 0x28],
    p0ier: P0IER,
    p0sr: P0SR,
    p0fcr: P0FCR,
    _reserved31: [u8; 0x0100],
    p0cfctcr: P0CFCTCR,
    p0cscstr: P0CSCSTR,
    p0cscszr: P0CSCSZR,
    _reserved34: [u8; 0xb4],
    p0cppcr: P0CPPCR,
    p0cppm0ar1: P0CPPM0AR1,
    p0cppm0ar2: P0CPPM0AR2,
}
impl RegisterBlock {
    ///0x00 - DCMIPP IP-Plug global register 1
    #[inline(always)]
    pub const fn ipgr1(&self) -> &IPGR1 {
        &self.ipgr1
    }
    ///0x04 - DCMIPP IP-Plug global register 2
    #[inline(always)]
    pub const fn ipgr2(&self) -> &IPGR2 {
        &self.ipgr2
    }
    ///0x08 - DCMIPP IP-Plug global register 3
    #[inline(always)]
    pub const fn ipgr3(&self) -> &IPGR3 {
        &self.ipgr3
    }
    ///0x1c - DCMIPP IP-Plug identification register
    #[inline(always)]
    pub const fn ipgr8(&self) -> &IPGR8 {
        &self.ipgr8
    }
    ///0x20 - DCMIPP IP-Plug Clientx register 1
    #[inline(always)]
    pub const fn ipc1r1(&self) -> &IPC1R1 {
        &self.ipc1r1
    }
    ///0x24 - DCMIPP IP-Plug Clientx register 2
    #[inline(always)]
    pub const fn ipc1r2(&self) -> &IPC1R2 {
        &self.ipc1r2
    }
    ///0x28 - DCMIPP IP-Plug Clientx register 3
    #[inline(always)]
    pub const fn ipc1r3(&self) -> &IPC1R3 {
        &self.ipc1r3
    }
    ///0x104 - DCMIPP parallel interface control register
    #[inline(always)]
    pub const fn prcr(&self) -> &PRCR {
        &self.prcr
    }
    ///0x108 - DCMIPP parallel interface embedded synchronization code register
    #[inline(always)]
    pub const fn prescr(&self) -> &PRESCR {
        &self.prescr
    }
    ///0x10c - DCMIPP parallel interface embedded synchronization unmask register
    #[inline(always)]
    pub const fn presur(&self) -> &PRESUR {
        &self.presur
    }
    ///0x1f4 - DCMIPP parallel interface interrupt enable register
    #[inline(always)]
    pub const fn prier(&self) -> &PRIER {
        &self.prier
    }
    ///0x1f8 - DCMIPP parallel interface status register
    #[inline(always)]
    pub const fn prsr(&self) -> &PRSR {
        &self.prsr
    }
    ///0x1fc - DCMIPP parallel interface interrupt clear register
    #[inline(always)]
    pub const fn prfcr(&self) -> &PRFCR {
        &self.prfcr
    }
    ///0x204 - DCMIPP common configuration register
    #[inline(always)]
    pub const fn cmcr(&self) -> &CMCR {
        &self.cmcr
    }
    ///0x208 - DCMIPP common frame counter register
    #[inline(always)]
    pub const fn cmfrcr(&self) -> &CMFRCR {
        &self.cmfrcr
    }
    ///0x3f0 - DCMIPP common interrupt enable register
    #[inline(always)]
    pub const fn cmier(&self) -> &CMIER {
        &self.cmier
    }
    ///0x3f4 - DCMIPP common status register 1
    #[inline(always)]
    pub const fn cmsr1(&self) -> &CMSR1 {
        &self.cmsr1
    }
    ///0x3f8 - DCMIPP common status register 2
    #[inline(always)]
    pub const fn cmsr2(&self) -> &CMSR2 {
        &self.cmsr2
    }
    ///0x3fc - DCMIPP common interrupt clear register
    #[inline(always)]
    pub const fn cmfcr(&self) -> &CMFCR {
        &self.cmfcr
    }
    ///0x404 - DCMIPP Pipe0 flow selection configuration register
    #[inline(always)]
    pub const fn p0fscr(&self) -> &P0FSCR {
        &self.p0fscr
    }
    ///0x500 - DCMIPP Pipe0 flow control configuration register
    #[inline(always)]
    pub const fn p0fctcr(&self) -> &P0FCTCR {
        &self.p0fctcr
    }
    ///0x504 - DCMIPP Pipe0 stat/crop start register
    #[inline(always)]
    pub const fn p0scstr(&self) -> &P0SCSTR {
        &self.p0scstr
    }
    ///0x508 - DCMIPP Pipe0 stat/crop size register
    #[inline(always)]
    pub const fn p0scszr(&self) -> &P0SCSZR {
        &self.p0scszr
    }
    ///0x5b0 - DCMIPP Pipe0 dump counter register
    #[inline(always)]
    pub const fn p0dccntr(&self) -> &P0DCCNTR {
        &self.p0dccntr
    }
    ///0x5b4 - DCMIPP Pipe0 dump limit register
    #[inline(always)]
    pub const fn p0dclmtr(&self) -> &P0DCLMTR {
        &self.p0dclmtr
    }
    ///0x5c0 - DCMIPP Pipe0 pixel packer configuration register
    #[inline(always)]
    pub const fn p0ppcr(&self) -> &P0PPCR {
        &self.p0ppcr
    }
    ///0x5c4 - DCMIPP Pipe0 pixel packer Memory0 address register 1
    #[inline(always)]
    pub const fn p0ppm0ar1(&self) -> &P0PPM0AR1 {
        &self.p0ppm0ar1
    }
    ///0x5c8 - DCMIPP Pipe0 pixel packer Memory0 address register 2
    #[inline(always)]
    pub const fn p0ppm0ar2(&self) -> &P0PPM0AR2 {
        &self.p0ppm0ar2
    }
    ///0x5f4 - DCMIPP Pipe0 interrupt enable register
    #[inline(always)]
    pub const fn p0ier(&self) -> &P0IER {
        &self.p0ier
    }
    ///0x5f8 - DCMIPP Pipe0 status register
    #[inline(always)]
    pub const fn p0sr(&self) -> &P0SR {
        &self.p0sr
    }
    ///0x5fc - DCMIPP Pipe0 interrupt clear register
    #[inline(always)]
    pub const fn p0fcr(&self) -> &P0FCR {
        &self.p0fcr
    }
    ///0x700 - DCMIPP Pipe0 current flow control configuration register
    #[inline(always)]
    pub const fn p0cfctcr(&self) -> &P0CFCTCR {
        &self.p0cfctcr
    }
    ///0x704 - DCMIPP Pipe0 current stat/crop start register
    #[inline(always)]
    pub const fn p0cscstr(&self) -> &P0CSCSTR {
        &self.p0cscstr
    }
    ///0x708 - DCMIPP Pipe0 current stat/crop size register
    #[inline(always)]
    pub const fn p0cscszr(&self) -> &P0CSCSZR {
        &self.p0cscszr
    }
    ///0x7c0 - DCMIPP Pipe0 current pixel packer configuration register
    #[inline(always)]
    pub const fn p0cppcr(&self) -> &P0CPPCR {
        &self.p0cppcr
    }
    ///0x7c4 - DCMIPP Pipe0 current pixel packer Memory0 address register 1
    #[inline(always)]
    pub const fn p0cppm0ar1(&self) -> &P0CPPM0AR1 {
        &self.p0cppm0ar1
    }
    ///0x7c8 - DCMIPP Pipe0 current pixel packer Memory0 address register 2
    #[inline(always)]
    pub const fn p0cppm0ar2(&self) -> &P0CPPM0AR2 {
        &self.p0cppm0ar2
    }
}
/**IPGR1 (rw) register accessor: DCMIPP IP-Plug global register 1

You can [`read`](crate::Reg::read) this register and get [`ipgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:IPGR1)

For information about available fields see [`mod@ipgr1`] module*/
pub type IPGR1 = crate::Reg<ipgr1::IPGR1rs>;
///DCMIPP IP-Plug global register 1
pub mod ipgr1;
/**IPGR2 (rw) register accessor: DCMIPP IP-Plug global register 2

You can [`read`](crate::Reg::read) this register and get [`ipgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:IPGR2)

For information about available fields see [`mod@ipgr2`] module*/
pub type IPGR2 = crate::Reg<ipgr2::IPGR2rs>;
///DCMIPP IP-Plug global register 2
pub mod ipgr2;
/**IPGR3 (r) register accessor: DCMIPP IP-Plug global register 3

You can [`read`](crate::Reg::read) this register and get [`ipgr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:IPGR3)

For information about available fields see [`mod@ipgr3`] module*/
pub type IPGR3 = crate::Reg<ipgr3::IPGR3rs>;
///DCMIPP IP-Plug global register 3
pub mod ipgr3;
/**IPGR8 (r) register accessor: DCMIPP IP-Plug identification register

You can [`read`](crate::Reg::read) this register and get [`ipgr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:IPGR8)

For information about available fields see [`mod@ipgr8`] module*/
pub type IPGR8 = crate::Reg<ipgr8::IPGR8rs>;
///DCMIPP IP-Plug identification register
pub mod ipgr8;
/**IPC1R1 (rw) register accessor: DCMIPP IP-Plug Clientx register 1

You can [`read`](crate::Reg::read) this register and get [`ipc1r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc1r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:IPC1R1)

For information about available fields see [`mod@ipc1r1`] module*/
pub type IPC1R1 = crate::Reg<ipc1r1::IPC1R1rs>;
///DCMIPP IP-Plug Clientx register 1
pub mod ipc1r1;
/**IPC1R2 (rw) register accessor: DCMIPP IP-Plug Clientx register 2

You can [`read`](crate::Reg::read) this register and get [`ipc1r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc1r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:IPC1R2)

For information about available fields see [`mod@ipc1r2`] module*/
pub type IPC1R2 = crate::Reg<ipc1r2::IPC1R2rs>;
///DCMIPP IP-Plug Clientx register 2
pub mod ipc1r2;
/**IPC1R3 (rw) register accessor: DCMIPP IP-Plug Clientx register 3

You can [`read`](crate::Reg::read) this register and get [`ipc1r3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc1r3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:IPC1R3)

For information about available fields see [`mod@ipc1r3`] module*/
pub type IPC1R3 = crate::Reg<ipc1r3::IPC1R3rs>;
///DCMIPP IP-Plug Clientx register 3
pub mod ipc1r3;
/**PRCR (rw) register accessor: DCMIPP parallel interface control register

You can [`read`](crate::Reg::read) this register and get [`prcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:PRCR)

For information about available fields see [`mod@prcr`] module*/
pub type PRCR = crate::Reg<prcr::PRCRrs>;
///DCMIPP parallel interface control register
pub mod prcr;
/**PRESCR (rw) register accessor: DCMIPP parallel interface embedded synchronization code register

You can [`read`](crate::Reg::read) this register and get [`prescr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prescr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:PRESCR)

For information about available fields see [`mod@prescr`] module*/
pub type PRESCR = crate::Reg<prescr::PRESCRrs>;
///DCMIPP parallel interface embedded synchronization code register
pub mod prescr;
/**PRESUR (rw) register accessor: DCMIPP parallel interface embedded synchronization unmask register

You can [`read`](crate::Reg::read) this register and get [`presur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:PRESUR)

For information about available fields see [`mod@presur`] module*/
pub type PRESUR = crate::Reg<presur::PRESURrs>;
///DCMIPP parallel interface embedded synchronization unmask register
pub mod presur;
/**PRIER (rw) register accessor: DCMIPP parallel interface interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`prier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:PRIER)

For information about available fields see [`mod@prier`] module*/
pub type PRIER = crate::Reg<prier::PRIERrs>;
///DCMIPP parallel interface interrupt enable register
pub mod prier;
/**PRSR (r) register accessor: DCMIPP parallel interface status register

You can [`read`](crate::Reg::read) this register and get [`prsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:PRSR)

For information about available fields see [`mod@prsr`] module*/
pub type PRSR = crate::Reg<prsr::PRSRrs>;
///DCMIPP parallel interface status register
pub mod prsr;
/**PRFCR (w) register accessor: DCMIPP parallel interface interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prfcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:PRFCR)

For information about available fields see [`mod@prfcr`] module*/
pub type PRFCR = crate::Reg<prfcr::PRFCRrs>;
///DCMIPP parallel interface interrupt clear register
pub mod prfcr;
/**CMCR (w) register accessor: DCMIPP common configuration register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:CMCR)

For information about available fields see [`mod@cmcr`] module*/
pub type CMCR = crate::Reg<cmcr::CMCRrs>;
///DCMIPP common configuration register
pub mod cmcr;
/**CMFRCR (r) register accessor: DCMIPP common frame counter register

You can [`read`](crate::Reg::read) this register and get [`cmfrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:CMFRCR)

For information about available fields see [`mod@cmfrcr`] module*/
pub type CMFRCR = crate::Reg<cmfrcr::CMFRCRrs>;
///DCMIPP common frame counter register
pub mod cmfrcr;
/**CMIER (rw) register accessor: DCMIPP common interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cmier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:CMIER)

For information about available fields see [`mod@cmier`] module*/
pub type CMIER = crate::Reg<cmier::CMIERrs>;
///DCMIPP common interrupt enable register
pub mod cmier;
/**CMSR1 (r) register accessor: DCMIPP common status register 1

You can [`read`](crate::Reg::read) this register and get [`cmsr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:CMSR1)

For information about available fields see [`mod@cmsr1`] module*/
pub type CMSR1 = crate::Reg<cmsr1::CMSR1rs>;
///DCMIPP common status register 1
pub mod cmsr1;
/**CMSR2 (r) register accessor: DCMIPP common status register 2

You can [`read`](crate::Reg::read) this register and get [`cmsr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:CMSR2)

For information about available fields see [`mod@cmsr2`] module*/
pub type CMSR2 = crate::Reg<cmsr2::CMSR2rs>;
///DCMIPP common status register 2
pub mod cmsr2;
/**CMFCR (w) register accessor: DCMIPP common interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmfcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:CMFCR)

For information about available fields see [`mod@cmfcr`] module*/
pub type CMFCR = crate::Reg<cmfcr::CMFCRrs>;
///DCMIPP common interrupt clear register
pub mod cmfcr;
/**P0FSCR (rw) register accessor: DCMIPP Pipe0 flow selection configuration register

You can [`read`](crate::Reg::read) this register and get [`p0fscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0fscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:P0FSCR)

For information about available fields see [`mod@p0fscr`] module*/
pub type P0FSCR = crate::Reg<p0fscr::P0FSCRrs>;
///DCMIPP Pipe0 flow selection configuration register
pub mod p0fscr;
/**P0FCTCR (rw) register accessor: DCMIPP Pipe0 flow control configuration register

You can [`read`](crate::Reg::read) this register and get [`p0fctcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0fctcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:P0FCTCR)

For information about available fields see [`mod@p0fctcr`] module*/
pub type P0FCTCR = crate::Reg<p0fctcr::P0FCTCRrs>;
///DCMIPP Pipe0 flow control configuration register
pub mod p0fctcr;
/**P0SCSTR (rw) register accessor: DCMIPP Pipe0 stat/crop start register

You can [`read`](crate::Reg::read) this register and get [`p0scstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0scstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:P0SCSTR)

For information about available fields see [`mod@p0scstr`] module*/
pub type P0SCSTR = crate::Reg<p0scstr::P0SCSTRrs>;
///DCMIPP Pipe0 stat/crop start register
pub mod p0scstr;
/**P0SCSZR (rw) register accessor: DCMIPP Pipe0 stat/crop size register

You can [`read`](crate::Reg::read) this register and get [`p0scszr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0scszr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:P0SCSZR)

For information about available fields see [`mod@p0scszr`] module*/
pub type P0SCSZR = crate::Reg<p0scszr::P0SCSZRrs>;
///DCMIPP Pipe0 stat/crop size register
pub mod p0scszr;
/**P0DCCNTR (r) register accessor: DCMIPP Pipe0 dump counter register

You can [`read`](crate::Reg::read) this register and get [`p0dccntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:P0DCCNTR)

For information about available fields see [`mod@p0dccntr`] module*/
pub type P0DCCNTR = crate::Reg<p0dccntr::P0DCCNTRrs>;
///DCMIPP Pipe0 dump counter register
pub mod p0dccntr;
/**P0DCLMTR (rw) register accessor: DCMIPP Pipe0 dump limit register

You can [`read`](crate::Reg::read) this register and get [`p0dclmtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0dclmtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:P0DCLMTR)

For information about available fields see [`mod@p0dclmtr`] module*/
pub type P0DCLMTR = crate::Reg<p0dclmtr::P0DCLMTRrs>;
///DCMIPP Pipe0 dump limit register
pub mod p0dclmtr;
/**P0PPCR (rw) register accessor: DCMIPP Pipe0 pixel packer configuration register

You can [`read`](crate::Reg::read) this register and get [`p0ppcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0ppcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:P0PPCR)

For information about available fields see [`mod@p0ppcr`] module*/
pub type P0PPCR = crate::Reg<p0ppcr::P0PPCRrs>;
///DCMIPP Pipe0 pixel packer configuration register
pub mod p0ppcr;
/**P0PPM0AR1 (rw) register accessor: DCMIPP Pipe0 pixel packer Memory0 address register 1

You can [`read`](crate::Reg::read) this register and get [`p0ppm0ar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0ppm0ar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:P0PPM0AR1)

For information about available fields see [`mod@p0ppm0ar1`] module*/
pub type P0PPM0AR1 = crate::Reg<p0ppm0ar1::P0PPM0AR1rs>;
///DCMIPP Pipe0 pixel packer Memory0 address register 1
pub mod p0ppm0ar1;
/**P0PPM0AR2 (rw) register accessor: DCMIPP Pipe0 pixel packer Memory0 address register 2

You can [`read`](crate::Reg::read) this register and get [`p0ppm0ar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0ppm0ar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:P0PPM0AR2)

For information about available fields see [`mod@p0ppm0ar2`] module*/
pub type P0PPM0AR2 = crate::Reg<p0ppm0ar2::P0PPM0AR2rs>;
///DCMIPP Pipe0 pixel packer Memory0 address register 2
pub mod p0ppm0ar2;
/**P0IER (rw) register accessor: DCMIPP Pipe0 interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`p0ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:P0IER)

For information about available fields see [`mod@p0ier`] module*/
pub type P0IER = crate::Reg<p0ier::P0IERrs>;
///DCMIPP Pipe0 interrupt enable register
pub mod p0ier;
/**P0SR (r) register accessor: DCMIPP Pipe0 status register

You can [`read`](crate::Reg::read) this register and get [`p0sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:P0SR)

For information about available fields see [`mod@p0sr`] module*/
pub type P0SR = crate::Reg<p0sr::P0SRrs>;
///DCMIPP Pipe0 status register
pub mod p0sr;
/**P0FCR (w) register accessor: DCMIPP Pipe0 interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:P0FCR)

For information about available fields see [`mod@p0fcr`] module*/
pub type P0FCR = crate::Reg<p0fcr::P0FCRrs>;
///DCMIPP Pipe0 interrupt clear register
pub mod p0fcr;
/**P0CFCTCR (r) register accessor: DCMIPP Pipe0 current flow control configuration register

You can [`read`](crate::Reg::read) this register and get [`p0cfctcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:P0CFCTCR)

For information about available fields see [`mod@p0cfctcr`] module*/
pub type P0CFCTCR = crate::Reg<p0cfctcr::P0CFCTCRrs>;
///DCMIPP Pipe0 current flow control configuration register
pub mod p0cfctcr;
/**P0CSCSTR (r) register accessor: DCMIPP Pipe0 current stat/crop start register

You can [`read`](crate::Reg::read) this register and get [`p0cscstr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:P0CSCSTR)

For information about available fields see [`mod@p0cscstr`] module*/
pub type P0CSCSTR = crate::Reg<p0cscstr::P0CSCSTRrs>;
///DCMIPP Pipe0 current stat/crop start register
pub mod p0cscstr;
/**P0CSCSZR (r) register accessor: DCMIPP Pipe0 current stat/crop size register

You can [`read`](crate::Reg::read) this register and get [`p0cscszr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:P0CSCSZR)

For information about available fields see [`mod@p0cscszr`] module*/
pub type P0CSCSZR = crate::Reg<p0cscszr::P0CSCSZRrs>;
///DCMIPP Pipe0 current stat/crop size register
pub mod p0cscszr;
/**P0CPPCR (r) register accessor: DCMIPP Pipe0 current pixel packer configuration register

You can [`read`](crate::Reg::read) this register and get [`p0cppcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:P0CPPCR)

For information about available fields see [`mod@p0cppcr`] module*/
pub type P0CPPCR = crate::Reg<p0cppcr::P0CPPCRrs>;
///DCMIPP Pipe0 current pixel packer configuration register
pub mod p0cppcr;
/**P0CPPM0AR1 (r) register accessor: DCMIPP Pipe0 current pixel packer Memory0 address register 1

You can [`read`](crate::Reg::read) this register and get [`p0cppm0ar1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:P0CPPM0AR1)

For information about available fields see [`mod@p0cppm0ar1`] module*/
pub type P0CPPM0AR1 = crate::Reg<p0cppm0ar1::P0CPPM0AR1rs>;
///DCMIPP Pipe0 current pixel packer Memory0 address register 1
pub mod p0cppm0ar1;
/**P0CPPM0AR2 (r) register accessor: DCMIPP Pipe0 current pixel packer Memory0 address register 2

You can [`read`](crate::Reg::read) this register and get [`p0cppm0ar2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:P0CPPM0AR2)

For information about available fields see [`mod@p0cppm0ar2`] module*/
pub type P0CPPM0AR2 = crate::Reg<p0cppm0ar2::P0CPPM0AR2rs>;
///DCMIPP Pipe0 current pixel packer Memory0 address register 2
pub mod p0cppm0ar2;
