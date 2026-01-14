#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    cr3: CR3,
    cr4: CR4,
    _reserved4: [u8; 0x10],
    voscr: VOSCR,
    bdcr1: BDCR1,
    bdcr2: BDCR2,
    dbpcr: DBPCR,
    cpucr: CPUCR,
    svmcr1: SVMCR1,
    svmcr2: SVMCR2,
    svmcr3: SVMCR3,
    _reserved12: [u8; 0x10],
    wkupcr: WKUPCR,
    wkupsr: WKUPSR,
    wkupepr: WKUPEPR,
    _reserved15: [u8; 0x14],
    seccfgr: SECCFGR,
    privcfgr: PRIVCFGR,
}
impl RegisterBlock {
    ///0x00 - PWR control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - PWR control register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - PWR control register 3
    #[inline(always)]
    pub const fn cr3(&self) -> &CR3 {
        &self.cr3
    }
    ///0x0c - PWR control register 4
    #[inline(always)]
    pub const fn cr4(&self) -> &CR4 {
        &self.cr4
    }
    ///0x20 - PWR voltage scaling control register
    #[inline(always)]
    pub const fn voscr(&self) -> &VOSCR {
        &self.voscr
    }
    ///0x24 - PWR backup domain control register 1
    #[inline(always)]
    pub const fn bdcr1(&self) -> &BDCR1 {
        &self.bdcr1
    }
    ///0x28 - PWR backup domain control register 2
    #[inline(always)]
    pub const fn bdcr2(&self) -> &BDCR2 {
        &self.bdcr2
    }
    ///0x2c - PWR disable backup protection control register
    #[inline(always)]
    pub const fn dbpcr(&self) -> &DBPCR {
        &self.dbpcr
    }
    ///0x30 - PWR CPU control register
    #[inline(always)]
    pub const fn cpucr(&self) -> &CPUCR {
        &self.cpucr
    }
    ///0x34 - PWR supply voltage monitoring control register 1
    #[inline(always)]
    pub const fn svmcr1(&self) -> &SVMCR1 {
        &self.svmcr1
    }
    ///0x38 - PWR supply voltage monitoring control register 2
    #[inline(always)]
    pub const fn svmcr2(&self) -> &SVMCR2 {
        &self.svmcr2
    }
    ///0x3c - PWR supply voltage monitoring control register 3
    #[inline(always)]
    pub const fn svmcr3(&self) -> &SVMCR3 {
        &self.svmcr3
    }
    ///0x50 - PWR wake-up clear register
    #[inline(always)]
    pub const fn wkupcr(&self) -> &WKUPCR {
        &self.wkupcr
    }
    ///0x54 - PWR wake-up status register
    #[inline(always)]
    pub const fn wkupsr(&self) -> &WKUPSR {
        &self.wkupsr
    }
    ///0x58 - PWR wake-up enable and polarity register
    #[inline(always)]
    pub const fn wkupepr(&self) -> &WKUPEPR {
        &self.wkupepr
    }
    ///0x70 - PWR security configuration register
    #[inline(always)]
    pub const fn seccfgr(&self) -> &SECCFGR {
        &self.seccfgr
    }
    ///0x74 - PWR privilege configuration register
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
}
/**CR1 (rw) register accessor: PWR control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#PWR:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///PWR control register 1
pub mod cr1;
/**CR2 (rw) register accessor: PWR control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#PWR:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///PWR control register 2
pub mod cr2;
/**CR3 (rw) register accessor: PWR control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#PWR:CR3)

For information about available fields see [`mod@cr3`] module*/
pub type CR3 = crate::Reg<cr3::CR3rs>;
///PWR control register 3
pub mod cr3;
/**CR4 (rw) register accessor: PWR control register 4

You can [`read`](crate::Reg::read) this register and get [`cr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#PWR:CR4)

For information about available fields see [`mod@cr4`] module*/
pub type CR4 = crate::Reg<cr4::CR4rs>;
///PWR control register 4
pub mod cr4;
/**VOSCR (rw) register accessor: PWR voltage scaling control register

You can [`read`](crate::Reg::read) this register and get [`voscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`voscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#PWR:VOSCR)

For information about available fields see [`mod@voscr`] module*/
pub type VOSCR = crate::Reg<voscr::VOSCRrs>;
///PWR voltage scaling control register
pub mod voscr;
/**BDCR1 (rw) register accessor: PWR backup domain control register 1

You can [`read`](crate::Reg::read) this register and get [`bdcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#PWR:BDCR1)

For information about available fields see [`mod@bdcr1`] module*/
pub type BDCR1 = crate::Reg<bdcr1::BDCR1rs>;
///PWR backup domain control register 1
pub mod bdcr1;
/**BDCR2 (rw) register accessor: PWR backup domain control register 2

You can [`read`](crate::Reg::read) this register and get [`bdcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#PWR:BDCR2)

For information about available fields see [`mod@bdcr2`] module*/
pub type BDCR2 = crate::Reg<bdcr2::BDCR2rs>;
///PWR backup domain control register 2
pub mod bdcr2;
/**DBPCR (rw) register accessor: PWR disable backup protection control register

You can [`read`](crate::Reg::read) this register and get [`dbpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#PWR:DBPCR)

For information about available fields see [`mod@dbpcr`] module*/
pub type DBPCR = crate::Reg<dbpcr::DBPCRrs>;
///PWR disable backup protection control register
pub mod dbpcr;
/**CPUCR (rw) register accessor: PWR CPU control register

You can [`read`](crate::Reg::read) this register and get [`cpucr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpucr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#PWR:CPUCR)

For information about available fields see [`mod@cpucr`] module*/
pub type CPUCR = crate::Reg<cpucr::CPUCRrs>;
///PWR CPU control register
pub mod cpucr;
/**SVMCR1 (rw) register accessor: PWR supply voltage monitoring control register 1

You can [`read`](crate::Reg::read) this register and get [`svmcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`svmcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#PWR:SVMCR1)

For information about available fields see [`mod@svmcr1`] module*/
pub type SVMCR1 = crate::Reg<svmcr1::SVMCR1rs>;
///PWR supply voltage monitoring control register 1
pub mod svmcr1;
/**SVMCR2 (rw) register accessor: PWR supply voltage monitoring control register 2

You can [`read`](crate::Reg::read) this register and get [`svmcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`svmcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#PWR:SVMCR2)

For information about available fields see [`mod@svmcr2`] module*/
pub type SVMCR2 = crate::Reg<svmcr2::SVMCR2rs>;
///PWR supply voltage monitoring control register 2
pub mod svmcr2;
/**SVMCR3 (rw) register accessor: PWR supply voltage monitoring control register 3

You can [`read`](crate::Reg::read) this register and get [`svmcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`svmcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#PWR:SVMCR3)

For information about available fields see [`mod@svmcr3`] module*/
pub type SVMCR3 = crate::Reg<svmcr3::SVMCR3rs>;
///PWR supply voltage monitoring control register 3
pub mod svmcr3;
/**WKUPCR (w) register accessor: PWR wake-up clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkupcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#PWR:WKUPCR)

For information about available fields see [`mod@wkupcr`] module*/
pub type WKUPCR = crate::Reg<wkupcr::WKUPCRrs>;
///PWR wake-up clear register
pub mod wkupcr;
/**WKUPSR (r) register accessor: PWR wake-up status register

You can [`read`](crate::Reg::read) this register and get [`wkupsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#PWR:WKUPSR)

For information about available fields see [`mod@wkupsr`] module*/
pub type WKUPSR = crate::Reg<wkupsr::WKUPSRrs>;
///PWR wake-up status register
pub mod wkupsr;
/**WKUPEPR (rw) register accessor: PWR wake-up enable and polarity register

You can [`read`](crate::Reg::read) this register and get [`wkupepr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkupepr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#PWR:WKUPEPR)

For information about available fields see [`mod@wkupepr`] module*/
pub type WKUPEPR = crate::Reg<wkupepr::WKUPEPRrs>;
///PWR wake-up enable and polarity register
pub mod wkupepr;
/**SECCFGR (rw) register accessor: PWR security configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#PWR:SECCFGR)

For information about available fields see [`mod@seccfgr`] module*/
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
///PWR security configuration register
pub mod seccfgr;
/**PRIVCFGR (rw) register accessor: PWR privilege configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#PWR:PRIVCFGR)

For information about available fields see [`mod@privcfgr`] module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///PWR privilege configuration register
pub mod privcfgr;
