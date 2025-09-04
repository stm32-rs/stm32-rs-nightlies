#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    cr3: CR3,
    vosr: VOSR,
    svmcr: SVMCR,
    wucr1: WUCR1,
    wucr2: WUCR2,
    wucr3: WUCR3,
    _reserved8: [u8; 0x08],
    dbpr: DBPR,
    _reserved9: [u8; 0x04],
    seccfgr: SECCFGR,
    privcfgr: PRIVCFGR,
    sr: SR,
    svmsr: SVMSR,
    _reserved13: [u8; 0x04],
    wusr: WUSR,
    wuscr: WUSCR,
    _reserved15: [u8; 0x04],
    ioretenra: IORETENRA,
    ioretra: IORETRA,
    ioretenrb: IORETENRB,
    ioretrb: IORETRB,
    ioretenrc: IORETENRC,
    ioretrc: IORETRC,
    _reserved21: [u8; 0x20],
    ioretenrh: IORETENRH,
    ioretrh: IORETRH,
    _reserved23: [u8; 0x70],
    radioscr: RADIOSCR,
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
    ///0x0c - PWR voltage scaling register
    #[inline(always)]
    pub const fn vosr(&self) -> &VOSR {
        &self.vosr
    }
    ///0x10 - PWR supply voltage monitoring control register
    #[inline(always)]
    pub const fn svmcr(&self) -> &SVMCR {
        &self.svmcr
    }
    ///0x14 - PWR wakeup control register 1
    #[inline(always)]
    pub const fn wucr1(&self) -> &WUCR1 {
        &self.wucr1
    }
    ///0x18 - PWR wakeup control register 2
    #[inline(always)]
    pub const fn wucr2(&self) -> &WUCR2 {
        &self.wucr2
    }
    ///0x1c - PWR wakeup control register 3
    #[inline(always)]
    pub const fn wucr3(&self) -> &WUCR3 {
        &self.wucr3
    }
    ///0x28 - PWR disable Backup domain register
    #[inline(always)]
    pub const fn dbpr(&self) -> &DBPR {
        &self.dbpr
    }
    ///0x30 - PWR security configuration register
    #[inline(always)]
    pub const fn seccfgr(&self) -> &SECCFGR {
        &self.seccfgr
    }
    ///0x34 - PWR privilege control register
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
    ///0x38 - PWR status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x3c - PWR supply voltage monitoring status register
    #[inline(always)]
    pub const fn svmsr(&self) -> &SVMSR {
        &self.svmsr
    }
    ///0x44 - PWR wakeup status register
    #[inline(always)]
    pub const fn wusr(&self) -> &WUSR {
        &self.wusr
    }
    ///0x48 - PWR wakeup status clear register
    #[inline(always)]
    pub const fn wuscr(&self) -> &WUSCR {
        &self.wuscr
    }
    ///0x50 - PWR port A Standby IO retention enable register
    #[inline(always)]
    pub const fn ioretenra(&self) -> &IORETENRA {
        &self.ioretenra
    }
    ///0x54 - PWR port A Standby IO retention status register
    #[inline(always)]
    pub const fn ioretra(&self) -> &IORETRA {
        &self.ioretra
    }
    ///0x58 - PWR port B Standby IO retention enable register
    #[inline(always)]
    pub const fn ioretenrb(&self) -> &IORETENRB {
        &self.ioretenrb
    }
    ///0x5c - PWR port B Standby IO retention status register
    #[inline(always)]
    pub const fn ioretrb(&self) -> &IORETRB {
        &self.ioretrb
    }
    ///0x60 - PWR port C Standby IO retention enable register
    #[inline(always)]
    pub const fn ioretenrc(&self) -> &IORETENRC {
        &self.ioretenrc
    }
    ///0x64 - PWR port C Standby IO retention status register
    #[inline(always)]
    pub const fn ioretrc(&self) -> &IORETRC {
        &self.ioretrc
    }
    ///0x88 - PWR port H Standby IO retention enable register
    #[inline(always)]
    pub const fn ioretenrh(&self) -> &IORETENRH {
        &self.ioretenrh
    }
    ///0x8c - PWR port H Standby IO retention status register
    #[inline(always)]
    pub const fn ioretrh(&self) -> &IORETRH {
        &self.ioretrh
    }
    ///0x100 - PWR 2.4 GHz RADIO status and control register
    #[inline(always)]
    pub const fn radioscr(&self) -> &RADIOSCR {
        &self.radioscr
    }
}
/**CR1 (rw) register accessor: PWR control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///PWR control register 1
pub mod cr1;
/**CR2 (rw) register accessor: PWR control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///PWR control register 2
pub mod cr2;
/**CR3 (rw) register accessor: PWR control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:CR3)

For information about available fields see [`mod@cr3`] module*/
pub type CR3 = crate::Reg<cr3::CR3rs>;
///PWR control register 3
pub mod cr3;
/**VOSR (rw) register accessor: PWR voltage scaling register

You can [`read`](crate::Reg::read) this register and get [`vosr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vosr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:VOSR)

For information about available fields see [`mod@vosr`] module*/
pub type VOSR = crate::Reg<vosr::VOSRrs>;
///PWR voltage scaling register
pub mod vosr;
/**SVMCR (rw) register accessor: PWR supply voltage monitoring control register

You can [`read`](crate::Reg::read) this register and get [`svmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`svmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:SVMCR)

For information about available fields see [`mod@svmcr`] module*/
pub type SVMCR = crate::Reg<svmcr::SVMCRrs>;
///PWR supply voltage monitoring control register
pub mod svmcr;
/**WUCR1 (rw) register accessor: PWR wakeup control register 1

You can [`read`](crate::Reg::read) this register and get [`wucr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wucr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:WUCR1)

For information about available fields see [`mod@wucr1`] module*/
pub type WUCR1 = crate::Reg<wucr1::WUCR1rs>;
///PWR wakeup control register 1
pub mod wucr1;
/**WUCR2 (rw) register accessor: PWR wakeup control register 2

You can [`read`](crate::Reg::read) this register and get [`wucr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wucr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:WUCR2)

For information about available fields see [`mod@wucr2`] module*/
pub type WUCR2 = crate::Reg<wucr2::WUCR2rs>;
///PWR wakeup control register 2
pub mod wucr2;
/**WUCR3 (rw) register accessor: PWR wakeup control register 3

You can [`read`](crate::Reg::read) this register and get [`wucr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wucr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:WUCR3)

For information about available fields see [`mod@wucr3`] module*/
pub type WUCR3 = crate::Reg<wucr3::WUCR3rs>;
///PWR wakeup control register 3
pub mod wucr3;
/**DBPR (rw) register accessor: PWR disable Backup domain register

You can [`read`](crate::Reg::read) this register and get [`dbpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:DBPR)

For information about available fields see [`mod@dbpr`] module*/
pub type DBPR = crate::Reg<dbpr::DBPRrs>;
///PWR disable Backup domain register
pub mod dbpr;
/**SECCFGR (rw) register accessor: PWR security configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:SECCFGR)

For information about available fields see [`mod@seccfgr`] module*/
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
///PWR security configuration register
pub mod seccfgr;
/**PRIVCFGR (rw) register accessor: PWR privilege control register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:PRIVCFGR)

For information about available fields see [`mod@privcfgr`] module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///PWR privilege control register
pub mod privcfgr;
/**SR (rw) register accessor: PWR status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///PWR status register
pub mod sr;
/**SVMSR (r) register accessor: PWR supply voltage monitoring status register

You can [`read`](crate::Reg::read) this register and get [`svmsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:SVMSR)

For information about available fields see [`mod@svmsr`] module*/
pub type SVMSR = crate::Reg<svmsr::SVMSRrs>;
///PWR supply voltage monitoring status register
pub mod svmsr;
/**WUSR (r) register accessor: PWR wakeup status register

You can [`read`](crate::Reg::read) this register and get [`wusr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:WUSR)

For information about available fields see [`mod@wusr`] module*/
pub type WUSR = crate::Reg<wusr::WUSRrs>;
///PWR wakeup status register
pub mod wusr;
/**WUSCR (w) register accessor: PWR wakeup status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wuscr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:WUSCR)

For information about available fields see [`mod@wuscr`] module*/
pub type WUSCR = crate::Reg<wuscr::WUSCRrs>;
///PWR wakeup status clear register
pub mod wuscr;
/**IORETENRA (rw) register accessor: PWR port A Standby IO retention enable register

You can [`read`](crate::Reg::read) this register and get [`ioretenra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioretenra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:IORETENRA)

For information about available fields see [`mod@ioretenra`] module*/
pub type IORETENRA = crate::Reg<ioretenra::IORETENRArs>;
///PWR port A Standby IO retention enable register
pub mod ioretenra;
/**IORETRA (rw) register accessor: PWR port A Standby IO retention status register

You can [`read`](crate::Reg::read) this register and get [`ioretra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioretra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:IORETRA)

For information about available fields see [`mod@ioretra`] module*/
pub type IORETRA = crate::Reg<ioretra::IORETRArs>;
///PWR port A Standby IO retention status register
pub mod ioretra;
/**IORETENRB (rw) register accessor: PWR port B Standby IO retention enable register

You can [`read`](crate::Reg::read) this register and get [`ioretenrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioretenrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:IORETENRB)

For information about available fields see [`mod@ioretenrb`] module*/
pub type IORETENRB = crate::Reg<ioretenrb::IORETENRBrs>;
///PWR port B Standby IO retention enable register
pub mod ioretenrb;
/**IORETRB (rw) register accessor: PWR port B Standby IO retention status register

You can [`read`](crate::Reg::read) this register and get [`ioretrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioretrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:IORETRB)

For information about available fields see [`mod@ioretrb`] module*/
pub type IORETRB = crate::Reg<ioretrb::IORETRBrs>;
///PWR port B Standby IO retention status register
pub mod ioretrb;
/**IORETENRC (rw) register accessor: PWR port C Standby IO retention enable register

You can [`read`](crate::Reg::read) this register and get [`ioretenrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioretenrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:IORETENRC)

For information about available fields see [`mod@ioretenrc`] module*/
pub type IORETENRC = crate::Reg<ioretenrc::IORETENRCrs>;
///PWR port C Standby IO retention enable register
pub mod ioretenrc;
/**IORETRC (rw) register accessor: PWR port C Standby IO retention status register

You can [`read`](crate::Reg::read) this register and get [`ioretrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioretrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:IORETRC)

For information about available fields see [`mod@ioretrc`] module*/
pub type IORETRC = crate::Reg<ioretrc::IORETRCrs>;
///PWR port C Standby IO retention status register
pub mod ioretrc;
/**IORETENRH (rw) register accessor: PWR port H Standby IO retention enable register

You can [`read`](crate::Reg::read) this register and get [`ioretenrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioretenrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:IORETENRH)

For information about available fields see [`mod@ioretenrh`] module*/
pub type IORETENRH = crate::Reg<ioretenrh::IORETENRHrs>;
///PWR port H Standby IO retention enable register
pub mod ioretenrh;
/**IORETRH (rw) register accessor: PWR port H Standby IO retention status register

You can [`read`](crate::Reg::read) this register and get [`ioretrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioretrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:IORETRH)

For information about available fields see [`mod@ioretrh`] module*/
pub type IORETRH = crate::Reg<ioretrh::IORETRHrs>;
///PWR port H Standby IO retention status register
pub mod ioretrh;
/**RADIOSCR (r) register accessor: PWR 2.4 GHz RADIO status and control register

You can [`read`](crate::Reg::read) this register and get [`radioscr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:RADIOSCR)

For information about available fields see [`mod@radioscr`] module*/
pub type RADIOSCR = crate::Reg<radioscr::RADIOSCRrs>;
///PWR 2.4 GHz RADIO status and control register
pub mod radioscr;
