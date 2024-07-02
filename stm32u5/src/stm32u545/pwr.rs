#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    pwr_cr1: PWR_CR1,
    pwr_cr2: PWR_CR2,
    pwr_cr3: PWR_CR3,
    pwr_vosr: PWR_VOSR,
    pwr_svmcr: PWR_SVMCR,
    pwr_wucr1: PWR_WUCR1,
    pwr_wucr2: PWR_WUCR2,
    pwr_wucr3: PWR_WUCR3,
    pwr_bdcr1: PWR_BDCR1,
    pwr_bdcr2: PWR_BDCR2,
    pwr_dbpr: PWR_DBPR,
    pwr_ucpdr: PWR_UCPDR,
    pwr_seccfgr: PWR_SECCFGR,
    pwr_privcfgr: PWR_PRIVCFGR,
    pwr_sr: PWR_SR,
    pwr_svmsr: PWR_SVMSR,
    pwr_bdsr: PWR_BDSR,
    pwr_wusr: PWR_WUSR,
    pwr_wuscr: PWR_WUSCR,
    pwr_apcr: PWR_APCR,
    pwr_pucra: PWR_PUCRA,
    pwr_pdcra: PWR_PDCRA,
    pwr_pucrb: PWR_PUCRB,
    pwr_pdcrb: PWR_PDCRB,
    pwr_pucrc: PWR_PUCRC,
    pwr_pdcrc: PWR_PDCRC,
    pwr_pucrd: PWR_PUCRD,
    pwr_pdcrd: PWR_PDCRD,
    pwr_pucre: PWR_PUCRE,
    pwr_pdcre: PWR_PDCRE,
    pwr_pucrf: PWR_PUCRF,
    pwr_pdcrf: PWR_PDCRF,
    pwr_pucrg: PWR_PUCRG,
    pwr_pdcrg: PWR_PDCRG,
    pwr_pucrh: PWR_PUCRH,
    pwr_pdcrh: PWR_PDCRH,
    pwr_pucri: PWR_PUCRI,
    pwr_pdcri: PWR_PDCRI,
    pwr_pucrj: PWR_PUCRJ,
    pwr_pdcrj: PWR_PDCRJ,
    _reserved40: [u8; 0x08],
    pwr_cr4: PWR_CR4,
}
impl RegisterBlock {
    ///0x00 - PWR control register 1
    #[inline(always)]
    pub const fn pwr_cr1(&self) -> &PWR_CR1 {
        &self.pwr_cr1
    }
    ///0x04 - PWR control register 2
    #[inline(always)]
    pub const fn pwr_cr2(&self) -> &PWR_CR2 {
        &self.pwr_cr2
    }
    ///0x08 - PWR control register 3
    #[inline(always)]
    pub const fn pwr_cr3(&self) -> &PWR_CR3 {
        &self.pwr_cr3
    }
    ///0x0c - PWR voltage scaling register
    #[inline(always)]
    pub const fn pwr_vosr(&self) -> &PWR_VOSR {
        &self.pwr_vosr
    }
    ///0x10 - PWR supply voltage monitoring control register
    #[inline(always)]
    pub const fn pwr_svmcr(&self) -> &PWR_SVMCR {
        &self.pwr_svmcr
    }
    ///0x14 - PWR wakeup control register 1
    #[inline(always)]
    pub const fn pwr_wucr1(&self) -> &PWR_WUCR1 {
        &self.pwr_wucr1
    }
    ///0x18 - PWR wakeup control register 2
    #[inline(always)]
    pub const fn pwr_wucr2(&self) -> &PWR_WUCR2 {
        &self.pwr_wucr2
    }
    ///0x1c - PWR wakeup control register 3
    #[inline(always)]
    pub const fn pwr_wucr3(&self) -> &PWR_WUCR3 {
        &self.pwr_wucr3
    }
    ///0x20 - PWR Backup domain control register 1
    #[inline(always)]
    pub const fn pwr_bdcr1(&self) -> &PWR_BDCR1 {
        &self.pwr_bdcr1
    }
    ///0x24 - PWR Backup domain control register 2
    #[inline(always)]
    pub const fn pwr_bdcr2(&self) -> &PWR_BDCR2 {
        &self.pwr_bdcr2
    }
    ///0x28 - PWR disable Backup domain register
    #[inline(always)]
    pub const fn pwr_dbpr(&self) -> &PWR_DBPR {
        &self.pwr_dbpr
    }
    ///0x2c - PWR USB Type-C™ and Power Delivery register
    #[inline(always)]
    pub const fn pwr_ucpdr(&self) -> &PWR_UCPDR {
        &self.pwr_ucpdr
    }
    ///0x30 - PWR security configuration register
    #[inline(always)]
    pub const fn pwr_seccfgr(&self) -> &PWR_SECCFGR {
        &self.pwr_seccfgr
    }
    ///0x34 - PWR privilege control register
    #[inline(always)]
    pub const fn pwr_privcfgr(&self) -> &PWR_PRIVCFGR {
        &self.pwr_privcfgr
    }
    ///0x38 - PWR status register
    #[inline(always)]
    pub const fn pwr_sr(&self) -> &PWR_SR {
        &self.pwr_sr
    }
    ///0x3c - PWR supply voltage monitoring status register
    #[inline(always)]
    pub const fn pwr_svmsr(&self) -> &PWR_SVMSR {
        &self.pwr_svmsr
    }
    ///0x40 - PWR Backup domain status register
    #[inline(always)]
    pub const fn pwr_bdsr(&self) -> &PWR_BDSR {
        &self.pwr_bdsr
    }
    ///0x44 - PWR wakeup status register
    #[inline(always)]
    pub const fn pwr_wusr(&self) -> &PWR_WUSR {
        &self.pwr_wusr
    }
    ///0x48 - PWR wakeup status clear register
    #[inline(always)]
    pub const fn pwr_wuscr(&self) -> &PWR_WUSCR {
        &self.pwr_wuscr
    }
    ///0x4c - PWR apply pull configuration register
    #[inline(always)]
    pub const fn pwr_apcr(&self) -> &PWR_APCR {
        &self.pwr_apcr
    }
    ///0x50 - PWR port A pull-up control register
    #[inline(always)]
    pub const fn pwr_pucra(&self) -> &PWR_PUCRA {
        &self.pwr_pucra
    }
    ///0x54 - PWR port A pull-down control register
    #[inline(always)]
    pub const fn pwr_pdcra(&self) -> &PWR_PDCRA {
        &self.pwr_pdcra
    }
    ///0x58 - PWR port B pull-up control register
    #[inline(always)]
    pub const fn pwr_pucrb(&self) -> &PWR_PUCRB {
        &self.pwr_pucrb
    }
    ///0x5c - PWR port B pull-down control register
    #[inline(always)]
    pub const fn pwr_pdcrb(&self) -> &PWR_PDCRB {
        &self.pwr_pdcrb
    }
    ///0x60 - Power port C pull up control register
    #[inline(always)]
    pub const fn pwr_pucrc(&self) -> &PWR_PUCRC {
        &self.pwr_pucrc
    }
    ///0x64 - PWR port C pull-down control register
    #[inline(always)]
    pub const fn pwr_pdcrc(&self) -> &PWR_PDCRC {
        &self.pwr_pdcrc
    }
    ///0x68 - PWR port D pull-up control register
    #[inline(always)]
    pub const fn pwr_pucrd(&self) -> &PWR_PUCRD {
        &self.pwr_pucrd
    }
    ///0x6c - PWR port D pull-down control register
    #[inline(always)]
    pub const fn pwr_pdcrd(&self) -> &PWR_PDCRD {
        &self.pwr_pdcrd
    }
    ///0x70 - PWR port E pull-up control register
    #[inline(always)]
    pub const fn pwr_pucre(&self) -> &PWR_PUCRE {
        &self.pwr_pucre
    }
    ///0x74 - PWR port E pull-down control register
    #[inline(always)]
    pub const fn pwr_pdcre(&self) -> &PWR_PDCRE {
        &self.pwr_pdcre
    }
    ///0x78 - PWR port F pull-up control register
    #[inline(always)]
    pub const fn pwr_pucrf(&self) -> &PWR_PUCRF {
        &self.pwr_pucrf
    }
    ///0x7c - PWR port F pull-down control register
    #[inline(always)]
    pub const fn pwr_pdcrf(&self) -> &PWR_PDCRF {
        &self.pwr_pdcrf
    }
    ///0x80 - PWR port G pull-up control register
    #[inline(always)]
    pub const fn pwr_pucrg(&self) -> &PWR_PUCRG {
        &self.pwr_pucrg
    }
    ///0x84 - PWR port G pull-down control register
    #[inline(always)]
    pub const fn pwr_pdcrg(&self) -> &PWR_PDCRG {
        &self.pwr_pdcrg
    }
    ///0x88 - PWR port H pull-up control register
    #[inline(always)]
    pub const fn pwr_pucrh(&self) -> &PWR_PUCRH {
        &self.pwr_pucrh
    }
    ///0x8c - PWR port H pull-down control register
    #[inline(always)]
    pub const fn pwr_pdcrh(&self) -> &PWR_PDCRH {
        &self.pwr_pdcrh
    }
    ///0x90 - PWR port I pull-up control register
    #[inline(always)]
    pub const fn pwr_pucri(&self) -> &PWR_PUCRI {
        &self.pwr_pucri
    }
    ///0x94 - PWR port I pull-down control register
    #[inline(always)]
    pub const fn pwr_pdcri(&self) -> &PWR_PDCRI {
        &self.pwr_pdcri
    }
    ///0x98 - PWR port J pull-up control register
    #[inline(always)]
    pub const fn pwr_pucrj(&self) -> &PWR_PUCRJ {
        &self.pwr_pucrj
    }
    ///0x9c - PWR port J pull-down control register
    #[inline(always)]
    pub const fn pwr_pdcrj(&self) -> &PWR_PDCRJ {
        &self.pwr_pdcrj
    }
    ///0xa8 - PWR control register 4
    #[inline(always)]
    pub const fn pwr_cr4(&self) -> &PWR_CR4 {
        &self.pwr_cr4
    }
}
/**PWR_CR1 (rw) register accessor: PWR control register 1

You can [`read`](crate::Reg::read) this register and get [`pwr_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_CR1)

For information about available fields see [`mod@pwr_cr1`]
module*/
pub type PWR_CR1 = crate::Reg<pwr_cr1::PWR_CR1rs>;
///PWR control register 1
pub mod pwr_cr1;
/**PWR_CR2 (rw) register accessor: PWR control register 2

You can [`read`](crate::Reg::read) this register and get [`pwr_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_CR2)

For information about available fields see [`mod@pwr_cr2`]
module*/
pub type PWR_CR2 = crate::Reg<pwr_cr2::PWR_CR2rs>;
///PWR control register 2
pub mod pwr_cr2;
/**PWR_CR3 (rw) register accessor: PWR control register 3

You can [`read`](crate::Reg::read) this register and get [`pwr_cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_CR3)

For information about available fields see [`mod@pwr_cr3`]
module*/
pub type PWR_CR3 = crate::Reg<pwr_cr3::PWR_CR3rs>;
///PWR control register 3
pub mod pwr_cr3;
/**PWR_VOSR (rw) register accessor: PWR voltage scaling register

You can [`read`](crate::Reg::read) this register and get [`pwr_vosr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_vosr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_VOSR)

For information about available fields see [`mod@pwr_vosr`]
module*/
pub type PWR_VOSR = crate::Reg<pwr_vosr::PWR_VOSRrs>;
///PWR voltage scaling register
pub mod pwr_vosr;
/**PWR_SVMCR (rw) register accessor: PWR supply voltage monitoring control register

You can [`read`](crate::Reg::read) this register and get [`pwr_svmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_svmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_SVMCR)

For information about available fields see [`mod@pwr_svmcr`]
module*/
pub type PWR_SVMCR = crate::Reg<pwr_svmcr::PWR_SVMCRrs>;
///PWR supply voltage monitoring control register
pub mod pwr_svmcr;
/**PWR_WUCR1 (rw) register accessor: PWR wakeup control register 1

You can [`read`](crate::Reg::read) this register and get [`pwr_wucr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_wucr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_WUCR1)

For information about available fields see [`mod@pwr_wucr1`]
module*/
pub type PWR_WUCR1 = crate::Reg<pwr_wucr1::PWR_WUCR1rs>;
///PWR wakeup control register 1
pub mod pwr_wucr1;
/**PWR_WUCR2 (rw) register accessor: PWR wakeup control register 2

You can [`read`](crate::Reg::read) this register and get [`pwr_wucr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_wucr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_WUCR2)

For information about available fields see [`mod@pwr_wucr2`]
module*/
pub type PWR_WUCR2 = crate::Reg<pwr_wucr2::PWR_WUCR2rs>;
///PWR wakeup control register 2
pub mod pwr_wucr2;
/**PWR_WUCR3 (rw) register accessor: PWR wakeup control register 3

You can [`read`](crate::Reg::read) this register and get [`pwr_wucr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_wucr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_WUCR3)

For information about available fields see [`mod@pwr_wucr3`]
module*/
pub type PWR_WUCR3 = crate::Reg<pwr_wucr3::PWR_WUCR3rs>;
///PWR wakeup control register 3
pub mod pwr_wucr3;
/**PWR_BDCR1 (rw) register accessor: PWR Backup domain control register 1

You can [`read`](crate::Reg::read) this register and get [`pwr_bdcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_bdcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_BDCR1)

For information about available fields see [`mod@pwr_bdcr1`]
module*/
pub type PWR_BDCR1 = crate::Reg<pwr_bdcr1::PWR_BDCR1rs>;
///PWR Backup domain control register 1
pub mod pwr_bdcr1;
/**PWR_BDCR2 (rw) register accessor: PWR Backup domain control register 2

You can [`read`](crate::Reg::read) this register and get [`pwr_bdcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_bdcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_BDCR2)

For information about available fields see [`mod@pwr_bdcr2`]
module*/
pub type PWR_BDCR2 = crate::Reg<pwr_bdcr2::PWR_BDCR2rs>;
///PWR Backup domain control register 2
pub mod pwr_bdcr2;
/**PWR_DBPR (rw) register accessor: PWR disable Backup domain register

You can [`read`](crate::Reg::read) this register and get [`pwr_dbpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_dbpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_DBPR)

For information about available fields see [`mod@pwr_dbpr`]
module*/
pub type PWR_DBPR = crate::Reg<pwr_dbpr::PWR_DBPRrs>;
///PWR disable Backup domain register
pub mod pwr_dbpr;
/**PWR_UCPDR (rw) register accessor: PWR USB Type-C™ and Power Delivery register

You can [`read`](crate::Reg::read) this register and get [`pwr_ucpdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_ucpdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_UCPDR)

For information about available fields see [`mod@pwr_ucpdr`]
module*/
pub type PWR_UCPDR = crate::Reg<pwr_ucpdr::PWR_UCPDRrs>;
///PWR USB Type-C™ and Power Delivery register
pub mod pwr_ucpdr;
/**PWR_SECCFGR (rw) register accessor: PWR security configuration register

You can [`read`](crate::Reg::read) this register and get [`pwr_seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_SECCFGR)

For information about available fields see [`mod@pwr_seccfgr`]
module*/
pub type PWR_SECCFGR = crate::Reg<pwr_seccfgr::PWR_SECCFGRrs>;
///PWR security configuration register
pub mod pwr_seccfgr;
/**PWR_PRIVCFGR (rw) register accessor: PWR privilege control register

You can [`read`](crate::Reg::read) this register and get [`pwr_privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_PRIVCFGR)

For information about available fields see [`mod@pwr_privcfgr`]
module*/
pub type PWR_PRIVCFGR = crate::Reg<pwr_privcfgr::PWR_PRIVCFGRrs>;
///PWR privilege control register
pub mod pwr_privcfgr;
/**PWR_SR (rw) register accessor: PWR status register

You can [`read`](crate::Reg::read) this register and get [`pwr_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_SR)

For information about available fields see [`mod@pwr_sr`]
module*/
pub type PWR_SR = crate::Reg<pwr_sr::PWR_SRrs>;
///PWR status register
pub mod pwr_sr;
/**PWR_SVMSR (r) register accessor: PWR supply voltage monitoring status register

You can [`read`](crate::Reg::read) this register and get [`pwr_svmsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_SVMSR)

For information about available fields see [`mod@pwr_svmsr`]
module*/
pub type PWR_SVMSR = crate::Reg<pwr_svmsr::PWR_SVMSRrs>;
///PWR supply voltage monitoring status register
pub mod pwr_svmsr;
/**PWR_BDSR (r) register accessor: PWR Backup domain status register

You can [`read`](crate::Reg::read) this register and get [`pwr_bdsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_BDSR)

For information about available fields see [`mod@pwr_bdsr`]
module*/
pub type PWR_BDSR = crate::Reg<pwr_bdsr::PWR_BDSRrs>;
///PWR Backup domain status register
pub mod pwr_bdsr;
/**PWR_WUSR (r) register accessor: PWR wakeup status register

You can [`read`](crate::Reg::read) this register and get [`pwr_wusr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_WUSR)

For information about available fields see [`mod@pwr_wusr`]
module*/
pub type PWR_WUSR = crate::Reg<pwr_wusr::PWR_WUSRrs>;
///PWR wakeup status register
pub mod pwr_wusr;
/**PWR_WUSCR (rw) register accessor: PWR wakeup status clear register

You can [`read`](crate::Reg::read) this register and get [`pwr_wuscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_wuscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_WUSCR)

For information about available fields see [`mod@pwr_wuscr`]
module*/
pub type PWR_WUSCR = crate::Reg<pwr_wuscr::PWR_WUSCRrs>;
///PWR wakeup status clear register
pub mod pwr_wuscr;
/**PWR_APCR (rw) register accessor: PWR apply pull configuration register

You can [`read`](crate::Reg::read) this register and get [`pwr_apcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_apcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_APCR)

For information about available fields see [`mod@pwr_apcr`]
module*/
pub type PWR_APCR = crate::Reg<pwr_apcr::PWR_APCRrs>;
///PWR apply pull configuration register
pub mod pwr_apcr;
/**PWR_PUCRA (rw) register accessor: PWR port A pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pucra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_PUCRA)

For information about available fields see [`mod@pwr_pucra`]
module*/
pub type PWR_PUCRA = crate::Reg<pwr_pucra::PWR_PUCRArs>;
///PWR port A pull-up control register
pub mod pwr_pucra;
/**PWR_PDCRA (rw) register accessor: PWR port A pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pdcra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_PDCRA)

For information about available fields see [`mod@pwr_pdcra`]
module*/
pub type PWR_PDCRA = crate::Reg<pwr_pdcra::PWR_PDCRArs>;
///PWR port A pull-down control register
pub mod pwr_pdcra;
/**PWR_PUCRB (rw) register accessor: PWR port B pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pucrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_PUCRB)

For information about available fields see [`mod@pwr_pucrb`]
module*/
pub type PWR_PUCRB = crate::Reg<pwr_pucrb::PWR_PUCRBrs>;
///PWR port B pull-up control register
pub mod pwr_pucrb;
/**PWR_PDCRB (rw) register accessor: PWR port B pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pdcrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_PDCRB)

For information about available fields see [`mod@pwr_pdcrb`]
module*/
pub type PWR_PDCRB = crate::Reg<pwr_pdcrb::PWR_PDCRBrs>;
///PWR port B pull-down control register
pub mod pwr_pdcrb;
/**PWR_PUCRC (rw) register accessor: Power port C pull up control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pucrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_PUCRC)

For information about available fields see [`mod@pwr_pucrc`]
module*/
pub type PWR_PUCRC = crate::Reg<pwr_pucrc::PWR_PUCRCrs>;
///Power port C pull up control register
pub mod pwr_pucrc;
/**PWR_PDCRC (rw) register accessor: PWR port C pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pdcrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_PDCRC)

For information about available fields see [`mod@pwr_pdcrc`]
module*/
pub type PWR_PDCRC = crate::Reg<pwr_pdcrc::PWR_PDCRCrs>;
///PWR port C pull-down control register
pub mod pwr_pdcrc;
/**PWR_PUCRD (rw) register accessor: PWR port D pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pucrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_PUCRD)

For information about available fields see [`mod@pwr_pucrd`]
module*/
pub type PWR_PUCRD = crate::Reg<pwr_pucrd::PWR_PUCRDrs>;
///PWR port D pull-up control register
pub mod pwr_pucrd;
/**PWR_PDCRD (rw) register accessor: PWR port D pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pdcrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_PDCRD)

For information about available fields see [`mod@pwr_pdcrd`]
module*/
pub type PWR_PDCRD = crate::Reg<pwr_pdcrd::PWR_PDCRDrs>;
///PWR port D pull-down control register
pub mod pwr_pdcrd;
/**PWR_PUCRE (rw) register accessor: PWR port E pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pucre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_PUCRE)

For information about available fields see [`mod@pwr_pucre`]
module*/
pub type PWR_PUCRE = crate::Reg<pwr_pucre::PWR_PUCRErs>;
///PWR port E pull-up control register
pub mod pwr_pucre;
/**PWR_PDCRE (rw) register accessor: PWR port E pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pdcre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_PDCRE)

For information about available fields see [`mod@pwr_pdcre`]
module*/
pub type PWR_PDCRE = crate::Reg<pwr_pdcre::PWR_PDCRErs>;
///PWR port E pull-down control register
pub mod pwr_pdcre;
/**PWR_PUCRF (rw) register accessor: PWR port F pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pucrf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucrf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_PUCRF)

For information about available fields see [`mod@pwr_pucrf`]
module*/
pub type PWR_PUCRF = crate::Reg<pwr_pucrf::PWR_PUCRFrs>;
///PWR port F pull-up control register
pub mod pwr_pucrf;
/**PWR_PDCRF (rw) register accessor: PWR port F pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pdcrf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcrf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_PDCRF)

For information about available fields see [`mod@pwr_pdcrf`]
module*/
pub type PWR_PDCRF = crate::Reg<pwr_pdcrf::PWR_PDCRFrs>;
///PWR port F pull-down control register
pub mod pwr_pdcrf;
/**PWR_PUCRG (rw) register accessor: PWR port G pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pucrg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucrg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_PUCRG)

For information about available fields see [`mod@pwr_pucrg`]
module*/
pub type PWR_PUCRG = crate::Reg<pwr_pucrg::PWR_PUCRGrs>;
///PWR port G pull-up control register
pub mod pwr_pucrg;
/**PWR_PDCRG (rw) register accessor: PWR port G pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pdcrg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcrg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_PDCRG)

For information about available fields see [`mod@pwr_pdcrg`]
module*/
pub type PWR_PDCRG = crate::Reg<pwr_pdcrg::PWR_PDCRGrs>;
///PWR port G pull-down control register
pub mod pwr_pdcrg;
/**PWR_PUCRH (rw) register accessor: PWR port H pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pucrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_PUCRH)

For information about available fields see [`mod@pwr_pucrh`]
module*/
pub type PWR_PUCRH = crate::Reg<pwr_pucrh::PWR_PUCRHrs>;
///PWR port H pull-up control register
pub mod pwr_pucrh;
/**PWR_PDCRH (rw) register accessor: PWR port H pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pdcrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_PDCRH)

For information about available fields see [`mod@pwr_pdcrh`]
module*/
pub type PWR_PDCRH = crate::Reg<pwr_pdcrh::PWR_PDCRHrs>;
///PWR port H pull-down control register
pub mod pwr_pdcrh;
/**PWR_PUCRI (rw) register accessor: PWR port I pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pucri::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucri::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_PUCRI)

For information about available fields see [`mod@pwr_pucri`]
module*/
pub type PWR_PUCRI = crate::Reg<pwr_pucri::PWR_PUCRIrs>;
///PWR port I pull-up control register
pub mod pwr_pucri;
/**PWR_PDCRI (rw) register accessor: PWR port I pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pdcri::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcri::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_PDCRI)

For information about available fields see [`mod@pwr_pdcri`]
module*/
pub type PWR_PDCRI = crate::Reg<pwr_pdcri::PWR_PDCRIrs>;
///PWR port I pull-down control register
pub mod pwr_pdcri;
/**PWR_PUCRJ (rw) register accessor: PWR port J pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pucrj::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucrj::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_PUCRJ)

For information about available fields see [`mod@pwr_pucrj`]
module*/
pub type PWR_PUCRJ = crate::Reg<pwr_pucrj::PWR_PUCRJrs>;
///PWR port J pull-up control register
pub mod pwr_pucrj;
/**PWR_PDCRJ (rw) register accessor: PWR port J pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pdcrj::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcrj::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_PDCRJ)

For information about available fields see [`mod@pwr_pdcrj`]
module*/
pub type PWR_PDCRJ = crate::Reg<pwr_pdcrj::PWR_PDCRJrs>;
///PWR port J pull-down control register
pub mod pwr_pdcrj;
/**PWR_CR4 (rw) register accessor: PWR control register 4

You can [`read`](crate::Reg::read) this register and get [`pwr_cr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_CR4)

For information about available fields see [`mod@pwr_cr4`]
module*/
pub type PWR_CR4 = crate::Reg<pwr_cr4::PWR_CR4rs>;
///PWR control register 4
pub mod pwr_cr4;
