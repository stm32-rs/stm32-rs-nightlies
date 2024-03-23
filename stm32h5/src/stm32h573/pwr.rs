#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pmcr: PMCR,
    pmsr: PMSR,
    _reserved2: [u8; 0x08],
    voscr: VOSCR,
    vossr: VOSSR,
    _reserved4: [u8; 0x08],
    bdcr: BDCR,
    dbpcr: DBPCR,
    bdsr: BDSR,
    ucpdr: UCPDR,
    sccr: SCCR,
    vmcr: VMCR,
    usbscr: USBSCR,
    vmsr: VMSR,
    wuscr: WUSCR,
    wusr: WUSR,
    wucr: WUCR,
    _reserved15: [u8; 0x04],
    ioretr: IORETR,
    _reserved16: [u8; 0xac],
    seccfgr: SECCFGR,
    privcfgr: PRIVCFGR,
}
impl RegisterBlock {
    #[doc = "0x00 - PWR power mode control register"]
    #[inline(always)]
    pub const fn pmcr(&self) -> &PMCR {
        &self.pmcr
    }
    #[doc = "0x04 - PWR status register"]
    #[inline(always)]
    pub const fn pmsr(&self) -> &PMSR {
        &self.pmsr
    }
    #[doc = "0x10 - PWR voltage scaling control register"]
    #[inline(always)]
    pub const fn voscr(&self) -> &VOSCR {
        &self.voscr
    }
    #[doc = "0x14 - PWR voltage scaling status register"]
    #[inline(always)]
    pub const fn vossr(&self) -> &VOSSR {
        &self.vossr
    }
    #[doc = "0x20 - PWR Backup domain control register"]
    #[inline(always)]
    pub const fn bdcr(&self) -> &BDCR {
        &self.bdcr
    }
    #[doc = "0x24 - PWR Backup domain control register"]
    #[inline(always)]
    pub const fn dbpcr(&self) -> &DBPCR {
        &self.dbpcr
    }
    #[doc = "0x28 - PWR Backup domain status register"]
    #[inline(always)]
    pub const fn bdsr(&self) -> &BDSR {
        &self.bdsr
    }
    #[doc = "0x2c - PWR USB Type-C power delivery register"]
    #[inline(always)]
    pub const fn ucpdr(&self) -> &UCPDR {
        &self.ucpdr
    }
    #[doc = "0x30 - PWR supply configuration control register"]
    #[inline(always)]
    pub const fn sccr(&self) -> &SCCR {
        &self.sccr
    }
    #[doc = "0x34 - PWR voltage monitor control register"]
    #[inline(always)]
    pub const fn vmcr(&self) -> &VMCR {
        &self.vmcr
    }
    #[doc = "0x38 - PWR USB supply control register"]
    #[inline(always)]
    pub const fn usbscr(&self) -> &USBSCR {
        &self.usbscr
    }
    #[doc = "0x3c - PWR voltage monitor status register"]
    #[inline(always)]
    pub const fn vmsr(&self) -> &VMSR {
        &self.vmsr
    }
    #[doc = "0x40 - PWR wakeup status clear register"]
    #[inline(always)]
    pub const fn wuscr(&self) -> &WUSCR {
        &self.wuscr
    }
    #[doc = "0x44 - PWR wakeup status register"]
    #[inline(always)]
    pub const fn wusr(&self) -> &WUSR {
        &self.wusr
    }
    #[doc = "0x48 - PWR wakeup configuration register"]
    #[inline(always)]
    pub const fn wucr(&self) -> &WUCR {
        &self.wucr
    }
    #[doc = "0x50 - PWR I/O retention register"]
    #[inline(always)]
    pub const fn ioretr(&self) -> &IORETR {
        &self.ioretr
    }
    #[doc = "0x100 - PWR security configuration register"]
    #[inline(always)]
    pub const fn seccfgr(&self) -> &SECCFGR {
        &self.seccfgr
    }
    #[doc = "0x104 - PWR privilege configuration register"]
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
}
#[doc = "PMCR (rw) register accessor: PWR power mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmcr`]
module"]
pub type PMCR = crate::Reg<pmcr::PMCRrs>;
#[doc = "PWR power mode control register"]
pub mod pmcr;
#[doc = "PMSR (r) register accessor: PWR status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmsr`]
module"]
pub type PMSR = crate::Reg<pmsr::PMSRrs>;
#[doc = "PWR status register"]
pub mod pmsr;
#[doc = "VOSCR (rw) register accessor: PWR voltage scaling control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`voscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`voscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@voscr`]
module"]
pub type VOSCR = crate::Reg<voscr::VOSCRrs>;
#[doc = "PWR voltage scaling control register"]
pub mod voscr;
#[doc = "VOSSR (r) register accessor: PWR voltage scaling status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vossr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vossr`]
module"]
pub type VOSSR = crate::Reg<vossr::VOSSRrs>;
#[doc = "PWR voltage scaling status register"]
pub mod vossr;
#[doc = "BDCR (rw) register accessor: PWR Backup domain control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdcr`]
module"]
pub type BDCR = crate::Reg<bdcr::BDCRrs>;
#[doc = "PWR Backup domain control register"]
pub mod bdcr;
#[doc = "DBPCR (rw) register accessor: PWR Backup domain control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbpcr`]
module"]
pub type DBPCR = crate::Reg<dbpcr::DBPCRrs>;
#[doc = "PWR Backup domain control register"]
pub mod dbpcr;
#[doc = "BDSR (r) register accessor: PWR Backup domain status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdsr`]
module"]
pub type BDSR = crate::Reg<bdsr::BDSRrs>;
#[doc = "PWR Backup domain status register"]
pub mod bdsr;
#[doc = "UCPDR (rw) register accessor: PWR USB Type-C power delivery register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucpdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucpdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucpdr`]
module"]
pub type UCPDR = crate::Reg<ucpdr::UCPDRrs>;
#[doc = "PWR USB Type-C power delivery register"]
pub mod ucpdr;
#[doc = "SCCR (rw) register accessor: PWR supply configuration control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccr`]
module"]
pub type SCCR = crate::Reg<sccr::SCCRrs>;
#[doc = "PWR supply configuration control register"]
pub mod sccr;
#[doc = "VMCR (rw) register accessor: PWR voltage monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vmcr`]
module"]
pub type VMCR = crate::Reg<vmcr::VMCRrs>;
#[doc = "PWR voltage monitor control register"]
pub mod vmcr;
#[doc = "USBSCR (rw) register accessor: PWR USB supply control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbscr`]
module"]
pub type USBSCR = crate::Reg<usbscr::USBSCRrs>;
#[doc = "PWR USB supply control register"]
pub mod usbscr;
#[doc = "VMSR (r) register accessor: PWR voltage monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vmsr`]
module"]
pub type VMSR = crate::Reg<vmsr::VMSRrs>;
#[doc = "PWR voltage monitor status register"]
pub mod vmsr;
#[doc = "WUSCR (w) register accessor: PWR wakeup status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wuscr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wuscr`]
module"]
pub type WUSCR = crate::Reg<wuscr::WUSCRrs>;
#[doc = "PWR wakeup status clear register"]
pub mod wuscr;
#[doc = "WUSR (r) register accessor: PWR wakeup status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wusr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wusr`]
module"]
pub type WUSR = crate::Reg<wusr::WUSRrs>;
#[doc = "PWR wakeup status register"]
pub mod wusr;
#[doc = "WUCR (rw) register accessor: PWR wakeup configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wucr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wucr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wucr`]
module"]
pub type WUCR = crate::Reg<wucr::WUCRrs>;
#[doc = "PWR wakeup configuration register"]
pub mod wucr;
#[doc = "IORETR (rw) register accessor: PWR I/O retention register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioretr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioretr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioretr`]
module"]
pub type IORETR = crate::Reg<ioretr::IORETRrs>;
#[doc = "PWR I/O retention register"]
pub mod ioretr;
#[doc = "SECCFGR (rw) register accessor: PWR security configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seccfgr`]
module"]
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
#[doc = "PWR security configuration register"]
pub mod seccfgr;
#[doc = "PRIVCFGR (rw) register accessor: PWR privilege configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@privcfgr`]
module"]
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
#[doc = "PWR privilege configuration register"]
pub mod privcfgr;
