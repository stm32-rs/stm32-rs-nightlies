#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mcr: MCR,
    msr: MSR,
    tsr: TSR,
    rfr: [RFR; 2],
    ier: IER,
    esr: ESR,
    btr: BTR,
    _reserved7: [u8; 0x0160],
    tx: [TX; 3],
    rx: [RX; 2],
    _reserved9: [u8; 0x30],
    fmr: FMR,
    fm1r: FM1R,
    _reserved11: [u8; 0x04],
    fs1r: FS1R,
    _reserved12: [u8; 0x04],
    ffa1r: FFA1R,
    _reserved13: [u8; 0x04],
    fa1r: FA1R,
    _reserved14: [u8; 0x20],
    fb: [FB; 28],
}
impl RegisterBlock {
    #[doc = "0x00 - master control register"]
    #[inline(always)]
    pub const fn mcr(&self) -> &MCR {
        &self.mcr
    }
    #[doc = "0x04 - master status register"]
    #[inline(always)]
    pub const fn msr(&self) -> &MSR {
        &self.msr
    }
    #[doc = "0x08 - transmit status register"]
    #[inline(always)]
    pub const fn tsr(&self) -> &TSR {
        &self.tsr
    }
    #[doc = "0x0c..0x14 - receive FIFO %s register"]
    #[inline(always)]
    pub const fn rfr(&self, n: usize) -> &RFR {
        &self.rfr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x14 - receive FIFO %s register"]
    #[inline(always)]
    pub fn rfr_iter(&self) -> impl Iterator<Item = &RFR> {
        self.rfr.iter()
    }
    #[doc = "0x0c - receive FIFO 0 register"]
    #[inline(always)]
    pub const fn rf0r(&self) -> &RFR {
        self.rfr(0)
    }
    #[doc = "0x10 - receive FIFO 1 register"]
    #[inline(always)]
    pub const fn rf1r(&self) -> &RFR {
        self.rfr(1)
    }
    #[doc = "0x14 - interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x18 - interrupt enable register"]
    #[inline(always)]
    pub const fn esr(&self) -> &ESR {
        &self.esr
    }
    #[doc = "0x1c - bit timing register"]
    #[inline(always)]
    pub const fn btr(&self) -> &BTR {
        &self.btr
    }
    #[doc = "0x180..0x1b0 - CAN Transmit cluster"]
    #[inline(always)]
    pub const fn tx(&self, n: usize) -> &TX {
        &self.tx[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x180..0x1b0 - CAN Transmit cluster"]
    #[inline(always)]
    pub fn tx_iter(&self) -> impl Iterator<Item = &TX> {
        self.tx.iter()
    }
    #[doc = "0x1b0..0x1d0 - CAN Receive cluster"]
    #[inline(always)]
    pub const fn rx(&self, n: usize) -> &RX {
        &self.rx[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1b0..0x1d0 - CAN Receive cluster"]
    #[inline(always)]
    pub fn rx_iter(&self) -> impl Iterator<Item = &RX> {
        self.rx.iter()
    }
    #[doc = "0x200 - filter master register"]
    #[inline(always)]
    pub const fn fmr(&self) -> &FMR {
        &self.fmr
    }
    #[doc = "0x204 - filter mode register"]
    #[inline(always)]
    pub const fn fm1r(&self) -> &FM1R {
        &self.fm1r
    }
    #[doc = "0x20c - filter scale register"]
    #[inline(always)]
    pub const fn fs1r(&self) -> &FS1R {
        &self.fs1r
    }
    #[doc = "0x214 - filter FIFO assignment register"]
    #[inline(always)]
    pub const fn ffa1r(&self) -> &FFA1R {
        &self.ffa1r
    }
    #[doc = "0x21c - filter activation register"]
    #[inline(always)]
    pub const fn fa1r(&self) -> &FA1R {
        &self.fa1r
    }
    #[doc = "0x240..0x320 - CAN Filter Bank cluster"]
    #[inline(always)]
    pub const fn fb(&self, n: usize) -> &FB {
        &self.fb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x240..0x320 - CAN Filter Bank cluster"]
    #[inline(always)]
    pub fn fb_iter(&self) -> impl Iterator<Item = &FB> {
        self.fb.iter()
    }
}
#[doc = "MCR (rw) register accessor: master control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
pub type MCR = crate::Reg<mcr::MCRrs>;
#[doc = "master control register"]
pub mod mcr;
#[doc = "MSR (rw) register accessor: master status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr`]
module"]
pub type MSR = crate::Reg<msr::MSRrs>;
#[doc = "master status register"]
pub mod msr;
#[doc = "TSR (rw) register accessor: transmit status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsr`]
module"]
pub type TSR = crate::Reg<tsr::TSRrs>;
#[doc = "transmit status register"]
pub mod tsr;
#[doc = "RFR (rw) register accessor: receive FIFO %s register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfr`]
module"]
pub type RFR = crate::Reg<rfr::RFRrs>;
#[doc = "receive FIFO %s register"]
pub mod rfr;
#[doc = "IER (rw) register accessor: interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IERrs>;
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "ESR (rw) register accessor: interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`esr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esr`]
module"]
pub type ESR = crate::Reg<esr::ESRrs>;
#[doc = "interrupt enable register"]
pub mod esr;
#[doc = "BTR (rw) register accessor: bit timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btr`]
module"]
pub type BTR = crate::Reg<btr::BTRrs>;
#[doc = "bit timing register"]
pub mod btr;
#[doc = "CAN Transmit cluster"]
pub use self::tx::TX;
#[doc = r"Cluster"]
#[doc = "CAN Transmit cluster"]
pub mod tx;
#[doc = "CAN Receive cluster"]
pub use self::rx::RX;
#[doc = r"Cluster"]
#[doc = "CAN Receive cluster"]
pub mod rx;
#[doc = "FMR (rw) register accessor: filter master register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmr`]
module"]
pub type FMR = crate::Reg<fmr::FMRrs>;
#[doc = "filter master register"]
pub mod fmr;
#[doc = "FM1R (rw) register accessor: filter mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fm1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fm1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fm1r`]
module"]
pub type FM1R = crate::Reg<fm1r::FM1Rrs>;
#[doc = "filter mode register"]
pub mod fm1r;
#[doc = "FS1R (rw) register accessor: filter scale register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs1r`]
module"]
pub type FS1R = crate::Reg<fs1r::FS1Rrs>;
#[doc = "filter scale register"]
pub mod fs1r;
#[doc = "FFA1R (rw) register accessor: filter FIFO assignment register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ffa1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ffa1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffa1r`]
module"]
pub type FFA1R = crate::Reg<ffa1r::FFA1Rrs>;
#[doc = "filter FIFO assignment register"]
pub mod ffa1r;
#[doc = "FA1R (rw) register accessor: filter activation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fa1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fa1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fa1r`]
module"]
pub type FA1R = crate::Reg<fa1r::FA1Rrs>;
#[doc = "filter activation register"]
pub mod fa1r;
#[doc = "CAN Filter Bank cluster"]
pub use self::fb::FB;
#[doc = r"Cluster"]
#[doc = "CAN Filter Bank cluster"]
pub mod fb;
