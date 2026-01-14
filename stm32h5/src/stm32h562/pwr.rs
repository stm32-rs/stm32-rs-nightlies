#[repr(C)]
#[derive(Debug)]
///Register block
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
    ///0x00 - PWR power mode control register
    #[inline(always)]
    pub const fn pmcr(&self) -> &PMCR {
        &self.pmcr
    }
    ///0x04 - PWR status register
    #[inline(always)]
    pub const fn pmsr(&self) -> &PMSR {
        &self.pmsr
    }
    ///0x10 - PWR voltage scaling control register
    #[inline(always)]
    pub const fn voscr(&self) -> &VOSCR {
        &self.voscr
    }
    ///0x14 - PWR voltage scaling status register
    #[inline(always)]
    pub const fn vossr(&self) -> &VOSSR {
        &self.vossr
    }
    ///0x20 - PWR Backup domain control register
    #[inline(always)]
    pub const fn bdcr(&self) -> &BDCR {
        &self.bdcr
    }
    ///0x24 - PWR Backup domain control register
    #[inline(always)]
    pub const fn dbpcr(&self) -> &DBPCR {
        &self.dbpcr
    }
    ///0x28 - PWR Backup domain status register
    #[inline(always)]
    pub const fn bdsr(&self) -> &BDSR {
        &self.bdsr
    }
    ///0x2c - PWR USB Type-C power delivery register
    #[inline(always)]
    pub const fn ucpdr(&self) -> &UCPDR {
        &self.ucpdr
    }
    ///0x30 - PWR supply configuration control register
    #[inline(always)]
    pub const fn sccr(&self) -> &SCCR {
        &self.sccr
    }
    ///0x34 - PWR voltage monitor control register
    #[inline(always)]
    pub const fn vmcr(&self) -> &VMCR {
        &self.vmcr
    }
    ///0x38 - PWR USB supply control register
    #[inline(always)]
    pub const fn usbscr(&self) -> &USBSCR {
        &self.usbscr
    }
    ///0x3c - PWR voltage monitor status register
    #[inline(always)]
    pub const fn vmsr(&self) -> &VMSR {
        &self.vmsr
    }
    ///0x40 - PWR wakeup status clear register
    #[inline(always)]
    pub const fn wuscr(&self) -> &WUSCR {
        &self.wuscr
    }
    ///0x44 - PWR wakeup status register
    #[inline(always)]
    pub const fn wusr(&self) -> &WUSR {
        &self.wusr
    }
    ///0x48 - PWR wakeup configuration register
    #[inline(always)]
    pub const fn wucr(&self) -> &WUCR {
        &self.wucr
    }
    ///0x50 - PWR I/O retention register
    #[inline(always)]
    pub const fn ioretr(&self) -> &IORETR {
        &self.ioretr
    }
    ///0x100 - PWR security configuration register
    #[inline(always)]
    pub const fn seccfgr(&self) -> &SECCFGR {
        &self.seccfgr
    }
    ///0x104 - PWR privilege configuration register
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
}
/**PMCR (rw) register accessor: PWR power mode control register

You can [`read`](crate::Reg::read) this register and get [`pmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#PWR:PMCR)

For information about available fields see [`mod@pmcr`] module*/
pub type PMCR = crate::Reg<pmcr::PMCRrs>;
///PWR power mode control register
pub mod pmcr;
/**PMSR (r) register accessor: PWR status register

You can [`read`](crate::Reg::read) this register and get [`pmsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#PWR:PMSR)

For information about available fields see [`mod@pmsr`] module*/
pub type PMSR = crate::Reg<pmsr::PMSRrs>;
///PWR status register
pub mod pmsr;
/**VOSCR (rw) register accessor: PWR voltage scaling control register

You can [`read`](crate::Reg::read) this register and get [`voscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`voscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#PWR:VOSCR)

For information about available fields see [`mod@voscr`] module*/
pub type VOSCR = crate::Reg<voscr::VOSCRrs>;
///PWR voltage scaling control register
pub mod voscr;
/**VOSSR (r) register accessor: PWR voltage scaling status register

You can [`read`](crate::Reg::read) this register and get [`vossr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#PWR:VOSSR)

For information about available fields see [`mod@vossr`] module*/
pub type VOSSR = crate::Reg<vossr::VOSSRrs>;
///PWR voltage scaling status register
pub mod vossr;
/**BDCR (rw) register accessor: PWR Backup domain control register

You can [`read`](crate::Reg::read) this register and get [`bdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#PWR:BDCR)

For information about available fields see [`mod@bdcr`] module*/
pub type BDCR = crate::Reg<bdcr::BDCRrs>;
///PWR Backup domain control register
pub mod bdcr;
/**DBPCR (rw) register accessor: PWR Backup domain control register

You can [`read`](crate::Reg::read) this register and get [`dbpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#PWR:DBPCR)

For information about available fields see [`mod@dbpcr`] module*/
pub type DBPCR = crate::Reg<dbpcr::DBPCRrs>;
///PWR Backup domain control register
pub mod dbpcr;
/**BDSR (r) register accessor: PWR Backup domain status register

You can [`read`](crate::Reg::read) this register and get [`bdsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#PWR:BDSR)

For information about available fields see [`mod@bdsr`] module*/
pub type BDSR = crate::Reg<bdsr::BDSRrs>;
///PWR Backup domain status register
pub mod bdsr;
/**UCPDR (rw) register accessor: PWR USB Type-C power delivery register

You can [`read`](crate::Reg::read) this register and get [`ucpdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucpdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#PWR:UCPDR)

For information about available fields see [`mod@ucpdr`] module*/
pub type UCPDR = crate::Reg<ucpdr::UCPDRrs>;
///PWR USB Type-C power delivery register
pub mod ucpdr;
/**SCCR (rw) register accessor: PWR supply configuration control register

You can [`read`](crate::Reg::read) this register and get [`sccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#PWR:SCCR)

For information about available fields see [`mod@sccr`] module*/
pub type SCCR = crate::Reg<sccr::SCCRrs>;
///PWR supply configuration control register
pub mod sccr;
/**VMCR (rw) register accessor: PWR voltage monitor control register

You can [`read`](crate::Reg::read) this register and get [`vmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#PWR:VMCR)

For information about available fields see [`mod@vmcr`] module*/
pub type VMCR = crate::Reg<vmcr::VMCRrs>;
///PWR voltage monitor control register
pub mod vmcr;
/**USBSCR (rw) register accessor: PWR USB supply control register

You can [`read`](crate::Reg::read) this register and get [`usbscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#PWR:USBSCR)

For information about available fields see [`mod@usbscr`] module*/
pub type USBSCR = crate::Reg<usbscr::USBSCRrs>;
///PWR USB supply control register
pub mod usbscr;
/**VMSR (r) register accessor: PWR voltage monitor status register

You can [`read`](crate::Reg::read) this register and get [`vmsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#PWR:VMSR)

For information about available fields see [`mod@vmsr`] module*/
pub type VMSR = crate::Reg<vmsr::VMSRrs>;
///PWR voltage monitor status register
pub mod vmsr;
/**WUSCR (w) register accessor: PWR wakeup status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wuscr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#PWR:WUSCR)

For information about available fields see [`mod@wuscr`] module*/
pub type WUSCR = crate::Reg<wuscr::WUSCRrs>;
///PWR wakeup status clear register
pub mod wuscr;
/**WUSR (r) register accessor: PWR wakeup status register

You can [`read`](crate::Reg::read) this register and get [`wusr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#PWR:WUSR)

For information about available fields see [`mod@wusr`] module*/
pub type WUSR = crate::Reg<wusr::WUSRrs>;
///PWR wakeup status register
pub mod wusr;
/**WUCR (rw) register accessor: PWR wakeup configuration register

You can [`read`](crate::Reg::read) this register and get [`wucr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wucr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#PWR:WUCR)

For information about available fields see [`mod@wucr`] module*/
pub type WUCR = crate::Reg<wucr::WUCRrs>;
///PWR wakeup configuration register
pub mod wucr;
/**IORETR (rw) register accessor: PWR I/O retention register

You can [`read`](crate::Reg::read) this register and get [`ioretr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioretr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#PWR:IORETR)

For information about available fields see [`mod@ioretr`] module*/
pub type IORETR = crate::Reg<ioretr::IORETRrs>;
///PWR I/O retention register
pub mod ioretr;
/**SECCFGR (rw) register accessor: PWR security configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#PWR:SECCFGR)

For information about available fields see [`mod@seccfgr`] module*/
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
///PWR security configuration register
pub mod seccfgr;
/**PRIVCFGR (rw) register accessor: PWR privilege configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#PWR:PRIVCFGR)

For information about available fields see [`mod@privcfgr`] module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///PWR privilege configuration register
pub mod privcfgr;
