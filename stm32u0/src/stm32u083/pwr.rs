#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    pwr_cr1: PWR_CR1,
    pwr_cr2: PWR_CR2,
    pwr_cr3: PWR_CR3,
    pwr_cr4: PWR_CR4,
    pwr_sr1: PWR_SR1,
    pwr_sr2: PWR_SR2,
    pwr_scr: PWR_SCR,
    _reserved7: [u8; 0x04],
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
}
impl RegisterBlock {
    ///0x00 - Power control register 1
    #[inline(always)]
    pub const fn pwr_cr1(&self) -> &PWR_CR1 {
        &self.pwr_cr1
    }
    ///0x04 - Power control register 2
    #[inline(always)]
    pub const fn pwr_cr2(&self) -> &PWR_CR2 {
        &self.pwr_cr2
    }
    ///0x08 - Power control register 3
    #[inline(always)]
    pub const fn pwr_cr3(&self) -> &PWR_CR3 {
        &self.pwr_cr3
    }
    ///0x0c - Power control register 4
    #[inline(always)]
    pub const fn pwr_cr4(&self) -> &PWR_CR4 {
        &self.pwr_cr4
    }
    ///0x10 - Power status register 1
    #[inline(always)]
    pub const fn pwr_sr1(&self) -> &PWR_SR1 {
        &self.pwr_sr1
    }
    ///0x14 - Power status register 2
    #[inline(always)]
    pub const fn pwr_sr2(&self) -> &PWR_SR2 {
        &self.pwr_sr2
    }
    ///0x18 - Power status clear register
    #[inline(always)]
    pub const fn pwr_scr(&self) -> &PWR_SCR {
        &self.pwr_scr
    }
    ///0x20 - Power Port A pull-up control register
    #[inline(always)]
    pub const fn pwr_pucra(&self) -> &PWR_PUCRA {
        &self.pwr_pucra
    }
    ///0x24 - Power Port A pull-down control register
    #[inline(always)]
    pub const fn pwr_pdcra(&self) -> &PWR_PDCRA {
        &self.pwr_pdcra
    }
    ///0x28 - Power Port B pull-up control register
    #[inline(always)]
    pub const fn pwr_pucrb(&self) -> &PWR_PUCRB {
        &self.pwr_pucrb
    }
    ///0x2c - Power Port B pull-down control register
    #[inline(always)]
    pub const fn pwr_pdcrb(&self) -> &PWR_PDCRB {
        &self.pwr_pdcrb
    }
    ///0x30 - Power Port C pull-up control register
    #[inline(always)]
    pub const fn pwr_pucrc(&self) -> &PWR_PUCRC {
        &self.pwr_pucrc
    }
    ///0x34 - Power Port C pull-down control register
    #[inline(always)]
    pub const fn pwr_pdcrc(&self) -> &PWR_PDCRC {
        &self.pwr_pdcrc
    }
    ///0x38 - Power Port D pull-up control register
    #[inline(always)]
    pub const fn pwr_pucrd(&self) -> &PWR_PUCRD {
        &self.pwr_pucrd
    }
    ///0x3c - Power Port D pull-down control register
    #[inline(always)]
    pub const fn pwr_pdcrd(&self) -> &PWR_PDCRD {
        &self.pwr_pdcrd
    }
    ///0x40 - Power Port E pull-up control register
    #[inline(always)]
    pub const fn pwr_pucre(&self) -> &PWR_PUCRE {
        &self.pwr_pucre
    }
    ///0x44 - Power Port E pull-down control register
    #[inline(always)]
    pub const fn pwr_pdcre(&self) -> &PWR_PDCRE {
        &self.pwr_pdcre
    }
    ///0x48 - Power Port F pull-up control register
    #[inline(always)]
    pub const fn pwr_pucrf(&self) -> &PWR_PUCRF {
        &self.pwr_pucrf
    }
    ///0x4c - Power Port F pull-down control register
    #[inline(always)]
    pub const fn pwr_pdcrf(&self) -> &PWR_PDCRF {
        &self.pwr_pdcrf
    }
}
/**PWR_CR1 (rw) register accessor: Power control register 1

You can [`read`](crate::Reg::read) this register and get [`pwr_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#PWR:PWR_CR1)

For information about available fields see [`mod@pwr_cr1`]
module*/
pub type PWR_CR1 = crate::Reg<pwr_cr1::PWR_CR1rs>;
///Power control register 1
pub mod pwr_cr1;
/**PWR_CR2 (rw) register accessor: Power control register 2

You can [`read`](crate::Reg::read) this register and get [`pwr_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#PWR:PWR_CR2)

For information about available fields see [`mod@pwr_cr2`]
module*/
pub type PWR_CR2 = crate::Reg<pwr_cr2::PWR_CR2rs>;
///Power control register 2
pub mod pwr_cr2;
/**PWR_CR3 (rw) register accessor: Power control register 3

You can [`read`](crate::Reg::read) this register and get [`pwr_cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#PWR:PWR_CR3)

For information about available fields see [`mod@pwr_cr3`]
module*/
pub type PWR_CR3 = crate::Reg<pwr_cr3::PWR_CR3rs>;
///Power control register 3
pub mod pwr_cr3;
/**PWR_CR4 (rw) register accessor: Power control register 4

You can [`read`](crate::Reg::read) this register and get [`pwr_cr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#PWR:PWR_CR4)

For information about available fields see [`mod@pwr_cr4`]
module*/
pub type PWR_CR4 = crate::Reg<pwr_cr4::PWR_CR4rs>;
///Power control register 4
pub mod pwr_cr4;
/**PWR_SR1 (r) register accessor: Power status register 1

You can [`read`](crate::Reg::read) this register and get [`pwr_sr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#PWR:PWR_SR1)

For information about available fields see [`mod@pwr_sr1`]
module*/
pub type PWR_SR1 = crate::Reg<pwr_sr1::PWR_SR1rs>;
///Power status register 1
pub mod pwr_sr1;
/**PWR_SR2 (r) register accessor: Power status register 2

You can [`read`](crate::Reg::read) this register and get [`pwr_sr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#PWR:PWR_SR2)

For information about available fields see [`mod@pwr_sr2`]
module*/
pub type PWR_SR2 = crate::Reg<pwr_sr2::PWR_SR2rs>;
///Power status register 2
pub mod pwr_sr2;
/**PWR_SCR (w) register accessor: Power status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#PWR:PWR_SCR)

For information about available fields see [`mod@pwr_scr`]
module*/
pub type PWR_SCR = crate::Reg<pwr_scr::PWR_SCRrs>;
///Power status clear register
pub mod pwr_scr;
/**PWR_PUCRA (rw) register accessor: Power Port A pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pucra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#PWR:PWR_PUCRA)

For information about available fields see [`mod@pwr_pucra`]
module*/
pub type PWR_PUCRA = crate::Reg<pwr_pucra::PWR_PUCRArs>;
///Power Port A pull-up control register
pub mod pwr_pucra;
/**PWR_PDCRA (rw) register accessor: Power Port A pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pdcra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#PWR:PWR_PDCRA)

For information about available fields see [`mod@pwr_pdcra`]
module*/
pub type PWR_PDCRA = crate::Reg<pwr_pdcra::PWR_PDCRArs>;
///Power Port A pull-down control register
pub mod pwr_pdcra;
/**PWR_PUCRB (rw) register accessor: Power Port B pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pucrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#PWR:PWR_PUCRB)

For information about available fields see [`mod@pwr_pucrb`]
module*/
pub type PWR_PUCRB = crate::Reg<pwr_pucrb::PWR_PUCRBrs>;
///Power Port B pull-up control register
pub mod pwr_pucrb;
/**PWR_PDCRB (rw) register accessor: Power Port B pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pdcrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#PWR:PWR_PDCRB)

For information about available fields see [`mod@pwr_pdcrb`]
module*/
pub type PWR_PDCRB = crate::Reg<pwr_pdcrb::PWR_PDCRBrs>;
///Power Port B pull-down control register
pub mod pwr_pdcrb;
/**PWR_PUCRC (rw) register accessor: Power Port C pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pucrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#PWR:PWR_PUCRC)

For information about available fields see [`mod@pwr_pucrc`]
module*/
pub type PWR_PUCRC = crate::Reg<pwr_pucrc::PWR_PUCRCrs>;
///Power Port C pull-up control register
pub mod pwr_pucrc;
/**PWR_PDCRC (rw) register accessor: Power Port C pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pdcrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#PWR:PWR_PDCRC)

For information about available fields see [`mod@pwr_pdcrc`]
module*/
pub type PWR_PDCRC = crate::Reg<pwr_pdcrc::PWR_PDCRCrs>;
///Power Port C pull-down control register
pub mod pwr_pdcrc;
/**PWR_PUCRD (rw) register accessor: Power Port D pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pucrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#PWR:PWR_PUCRD)

For information about available fields see [`mod@pwr_pucrd`]
module*/
pub type PWR_PUCRD = crate::Reg<pwr_pucrd::PWR_PUCRDrs>;
///Power Port D pull-up control register
pub mod pwr_pucrd;
/**PWR_PDCRD (rw) register accessor: Power Port D pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pdcrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#PWR:PWR_PDCRD)

For information about available fields see [`mod@pwr_pdcrd`]
module*/
pub type PWR_PDCRD = crate::Reg<pwr_pdcrd::PWR_PDCRDrs>;
///Power Port D pull-down control register
pub mod pwr_pdcrd;
/**PWR_PUCRE (rw) register accessor: Power Port E pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pucre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#PWR:PWR_PUCRE)

For information about available fields see [`mod@pwr_pucre`]
module*/
pub type PWR_PUCRE = crate::Reg<pwr_pucre::PWR_PUCRErs>;
///Power Port E pull-up control register
pub mod pwr_pucre;
/**PWR_PDCRE (rw) register accessor: Power Port E pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pdcre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#PWR:PWR_PDCRE)

For information about available fields see [`mod@pwr_pdcre`]
module*/
pub type PWR_PDCRE = crate::Reg<pwr_pdcre::PWR_PDCRErs>;
///Power Port E pull-down control register
pub mod pwr_pdcre;
/**PWR_PUCRF (rw) register accessor: Power Port F pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pucrf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucrf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#PWR:PWR_PUCRF)

For information about available fields see [`mod@pwr_pucrf`]
module*/
pub type PWR_PUCRF = crate::Reg<pwr_pucrf::PWR_PUCRFrs>;
///Power Port F pull-up control register
pub mod pwr_pucrf;
/**PWR_PDCRF (rw) register accessor: Power Port F pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pdcrf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcrf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#PWR:PWR_PDCRF)

For information about available fields see [`mod@pwr_pdcrf`]
module*/
pub type PWR_PDCRF = crate::Reg<pwr_pdcrf::PWR_PDCRFrs>;
///Power Port F pull-down control register
pub mod pwr_pdcrf;
