#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfgr: CFGR,
    oar: OAR,
    pres: PRES,
    esr: ESR,
    csr: CSR,
    txd: TXD,
    rxd: RXD,
}
impl RegisterBlock {
    #[doc = "0x00 - configuration register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    #[doc = "0x04 - CEC own address register"]
    #[inline(always)]
    pub const fn oar(&self) -> &OAR {
        &self.oar
    }
    #[doc = "0x08 - Rx Data Register"]
    #[inline(always)]
    pub const fn pres(&self) -> &PRES {
        &self.pres
    }
    #[doc = "0x0c - CEC error status register"]
    #[inline(always)]
    pub const fn esr(&self) -> &ESR {
        &self.esr
    }
    #[doc = "0x10 - CEC control and status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    #[doc = "0x14 - CEC Tx data register"]
    #[inline(always)]
    pub const fn txd(&self) -> &TXD {
        &self.txd
    }
    #[doc = "0x18 - CEC Rx data register"]
    #[inline(always)]
    pub const fn rxd(&self) -> &RXD {
        &self.rxd
    }
}
#[doc = "CFGR (rw) register accessor: configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`]
module"]
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
#[doc = "configuration register"]
pub mod cfgr;
#[doc = "OAR (rw) register accessor: CEC own address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oar`]
module"]
pub type OAR = crate::Reg<oar::OARrs>;
#[doc = "CEC own address register"]
pub mod oar;
#[doc = "PRES (rw) register accessor: Rx Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pres::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pres::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pres`]
module"]
pub type PRES = crate::Reg<pres::PRESrs>;
#[doc = "Rx Data Register"]
pub mod pres;
#[doc = "ESR (r) register accessor: CEC error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esr`]
module"]
pub type ESR = crate::Reg<esr::ESRrs>;
#[doc = "CEC error status register"]
pub mod esr;
#[doc = "CSR (rw) register accessor: CEC control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSRrs>;
#[doc = "CEC control and status register"]
pub mod csr;
#[doc = "TXD (rw) register accessor: CEC Tx data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txd`]
module"]
pub type TXD = crate::Reg<txd::TXDrs>;
#[doc = "CEC Tx data register"]
pub mod txd;
#[doc = "RXD (r) register accessor: CEC Rx data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxd`]
module"]
pub type RXD = crate::Reg<rxd::RXDrs>;
#[doc = "CEC Rx data register"]
pub mod rxd;
