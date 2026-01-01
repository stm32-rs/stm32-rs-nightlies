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
    _reserved15: [u8; 0x08],
    pwr_pucrf: PWR_PUCRF,
    pwr_pdcrf: PWR_PDCRF,
    _reserved17: [u8; 0x20],
    pwr_bkp0r: PWR_BKP0R,
    pwr_bkp1r: PWR_BKP1R,
    pwr_bkp2r: PWR_BKP2R,
    pwr_bkp3r: PWR_BKP3R,
}
impl RegisterBlock {
    ///0x00 - PWR control register 1
    #[inline(always)]
    pub const fn pwr_cr1(&self) -> &PWR_CR1 {
        &self.pwr_cr1
    }
    ///0x04 - PWR control register 1
    #[inline(always)]
    pub const fn pwr_cr2(&self) -> &PWR_CR2 {
        &self.pwr_cr2
    }
    ///0x08 - PWR control register 3
    #[inline(always)]
    pub const fn pwr_cr3(&self) -> &PWR_CR3 {
        &self.pwr_cr3
    }
    ///0x0c - PWR control register 4
    #[inline(always)]
    pub const fn pwr_cr4(&self) -> &PWR_CR4 {
        &self.pwr_cr4
    }
    ///0x10 - PWR status register 1
    #[inline(always)]
    pub const fn pwr_sr1(&self) -> &PWR_SR1 {
        &self.pwr_sr1
    }
    ///0x14 - PWR status register 2
    #[inline(always)]
    pub const fn pwr_sr2(&self) -> &PWR_SR2 {
        &self.pwr_sr2
    }
    ///0x18 - PWR status clear register
    #[inline(always)]
    pub const fn pwr_scr(&self) -> &PWR_SCR {
        &self.pwr_scr
    }
    ///0x20 - PWR Port A pull-up control register
    #[inline(always)]
    pub const fn pwr_pucra(&self) -> &PWR_PUCRA {
        &self.pwr_pucra
    }
    ///0x24 - PWR Port A pull-down control register
    #[inline(always)]
    pub const fn pwr_pdcra(&self) -> &PWR_PDCRA {
        &self.pwr_pdcra
    }
    ///0x28 - PWR Port B pull-up control register
    #[inline(always)]
    pub const fn pwr_pucrb(&self) -> &PWR_PUCRB {
        &self.pwr_pucrb
    }
    ///0x2c - PWR Port B pull-down control register
    #[inline(always)]
    pub const fn pwr_pdcrb(&self) -> &PWR_PDCRB {
        &self.pwr_pdcrb
    }
    ///0x30 - PWR Port C pull-up control register
    #[inline(always)]
    pub const fn pwr_pucrc(&self) -> &PWR_PUCRC {
        &self.pwr_pucrc
    }
    ///0x34 - PWR Port C pull-down control register
    #[inline(always)]
    pub const fn pwr_pdcrc(&self) -> &PWR_PDCRC {
        &self.pwr_pdcrc
    }
    ///0x38 - PWR Port D pull-up control register
    #[inline(always)]
    pub const fn pwr_pucrd(&self) -> &PWR_PUCRD {
        &self.pwr_pucrd
    }
    ///0x3c - PWR Port D pull-down control register
    #[inline(always)]
    pub const fn pwr_pdcrd(&self) -> &PWR_PDCRD {
        &self.pwr_pdcrd
    }
    ///0x48 - PWR Port F pull-up control register
    #[inline(always)]
    pub const fn pwr_pucrf(&self) -> &PWR_PUCRF {
        &self.pwr_pucrf
    }
    ///0x4c - PWR Port F pull-down control register
    #[inline(always)]
    pub const fn pwr_pdcrf(&self) -> &PWR_PDCRF {
        &self.pwr_pdcrf
    }
    ///0x70 - PWR backup 0 register
    #[inline(always)]
    pub const fn pwr_bkp0r(&self) -> &PWR_BKP0R {
        &self.pwr_bkp0r
    }
    ///0x74 - PWR backup 1 register
    #[inline(always)]
    pub const fn pwr_bkp1r(&self) -> &PWR_BKP1R {
        &self.pwr_bkp1r
    }
    ///0x78 - PWR backup 2 register
    #[inline(always)]
    pub const fn pwr_bkp2r(&self) -> &PWR_BKP2R {
        &self.pwr_bkp2r
    }
    ///0x7c - PWR backup 3 register
    #[inline(always)]
    pub const fn pwr_bkp3r(&self) -> &PWR_BKP3R {
        &self.pwr_bkp3r
    }
}
/**PWR_CR1 (rw) register accessor: PWR control register 1

You can [`read`](crate::Reg::read) this register and get [`pwr_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_CR1)

For information about available fields see [`mod@pwr_cr1`] module*/
pub type PWR_CR1 = crate::Reg<pwr_cr1::PWR_CR1rs>;
///PWR control register 1
pub mod pwr_cr1;
/**PWR_CR2 (rw) register accessor: PWR control register 1

You can [`read`](crate::Reg::read) this register and get [`pwr_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_CR2)

For information about available fields see [`mod@pwr_cr2`] module*/
pub type PWR_CR2 = crate::Reg<pwr_cr2::PWR_CR2rs>;
///PWR control register 1
pub mod pwr_cr2;
/**PWR_CR3 (rw) register accessor: PWR control register 3

You can [`read`](crate::Reg::read) this register and get [`pwr_cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_CR3)

For information about available fields see [`mod@pwr_cr3`] module*/
pub type PWR_CR3 = crate::Reg<pwr_cr3::PWR_CR3rs>;
///PWR control register 3
pub mod pwr_cr3;
/**PWR_CR4 (rw) register accessor: PWR control register 4

You can [`read`](crate::Reg::read) this register and get [`pwr_cr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_CR4)

For information about available fields see [`mod@pwr_cr4`] module*/
pub type PWR_CR4 = crate::Reg<pwr_cr4::PWR_CR4rs>;
///PWR control register 4
pub mod pwr_cr4;
/**PWR_SR1 (r) register accessor: PWR status register 1

You can [`read`](crate::Reg::read) this register and get [`pwr_sr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_SR1)

For information about available fields see [`mod@pwr_sr1`] module*/
pub type PWR_SR1 = crate::Reg<pwr_sr1::PWR_SR1rs>;
///PWR status register 1
pub mod pwr_sr1;
/**PWR_SR2 (r) register accessor: PWR status register 2

You can [`read`](crate::Reg::read) this register and get [`pwr_sr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_SR2)

For information about available fields see [`mod@pwr_sr2`] module*/
pub type PWR_SR2 = crate::Reg<pwr_sr2::PWR_SR2rs>;
///PWR status register 2
pub mod pwr_sr2;
/**PWR_SCR (w) register accessor: PWR status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_SCR)

For information about available fields see [`mod@pwr_scr`] module*/
pub type PWR_SCR = crate::Reg<pwr_scr::PWR_SCRrs>;
///PWR status clear register
pub mod pwr_scr;
/**PWR_PUCRA (rw) register accessor: PWR Port A pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pucra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_PUCRA)

For information about available fields see [`mod@pwr_pucra`] module*/
pub type PWR_PUCRA = crate::Reg<pwr_pucra::PWR_PUCRArs>;
///PWR Port A pull-up control register
pub mod pwr_pucra;
/**PWR_PDCRA (rw) register accessor: PWR Port A pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pdcra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_PDCRA)

For information about available fields see [`mod@pwr_pdcra`] module*/
pub type PWR_PDCRA = crate::Reg<pwr_pdcra::PWR_PDCRArs>;
///PWR Port A pull-down control register
pub mod pwr_pdcra;
/**PWR_PUCRB (rw) register accessor: PWR Port B pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pucrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_PUCRB)

For information about available fields see [`mod@pwr_pucrb`] module*/
pub type PWR_PUCRB = crate::Reg<pwr_pucrb::PWR_PUCRBrs>;
///PWR Port B pull-up control register
pub mod pwr_pucrb;
/**PWR_PDCRB (rw) register accessor: PWR Port B pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pdcrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_PDCRB)

For information about available fields see [`mod@pwr_pdcrb`] module*/
pub type PWR_PDCRB = crate::Reg<pwr_pdcrb::PWR_PDCRBrs>;
///PWR Port B pull-down control register
pub mod pwr_pdcrb;
/**PWR_PUCRC (rw) register accessor: PWR Port C pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pucrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_PUCRC)

For information about available fields see [`mod@pwr_pucrc`] module*/
pub type PWR_PUCRC = crate::Reg<pwr_pucrc::PWR_PUCRCrs>;
///PWR Port C pull-up control register
pub mod pwr_pucrc;
/**PWR_PDCRC (rw) register accessor: PWR Port C pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pdcrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_PDCRC)

For information about available fields see [`mod@pwr_pdcrc`] module*/
pub type PWR_PDCRC = crate::Reg<pwr_pdcrc::PWR_PDCRCrs>;
///PWR Port C pull-down control register
pub mod pwr_pdcrc;
/**PWR_PUCRD (rw) register accessor: PWR Port D pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pucrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_PUCRD)

For information about available fields see [`mod@pwr_pucrd`] module*/
pub type PWR_PUCRD = crate::Reg<pwr_pucrd::PWR_PUCRDrs>;
///PWR Port D pull-up control register
pub mod pwr_pucrd;
/**PWR_PDCRD (rw) register accessor: PWR Port D pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pdcrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_PDCRD)

For information about available fields see [`mod@pwr_pdcrd`] module*/
pub type PWR_PDCRD = crate::Reg<pwr_pdcrd::PWR_PDCRDrs>;
///PWR Port D pull-down control register
pub mod pwr_pdcrd;
/**PWR_PUCRF (rw) register accessor: PWR Port F pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pucrf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucrf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_PUCRF)

For information about available fields see [`mod@pwr_pucrf`] module*/
pub type PWR_PUCRF = crate::Reg<pwr_pucrf::PWR_PUCRFrs>;
///PWR Port F pull-up control register
pub mod pwr_pucrf;
/**PWR_PDCRF (rw) register accessor: PWR Port F pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pdcrf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcrf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_PDCRF)

For information about available fields see [`mod@pwr_pdcrf`] module*/
pub type PWR_PDCRF = crate::Reg<pwr_pdcrf::PWR_PDCRFrs>;
///PWR Port F pull-down control register
pub mod pwr_pdcrf;
/**PWR_BKP0R (rw) register accessor: PWR backup 0 register

You can [`read`](crate::Reg::read) this register and get [`pwr_bkp0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_bkp0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_BKP0R)

For information about available fields see [`mod@pwr_bkp0r`] module*/
pub type PWR_BKP0R = crate::Reg<pwr_bkp0r::PWR_BKP0Rrs>;
///PWR backup 0 register
pub mod pwr_bkp0r;
/**PWR_BKP1R (rw) register accessor: PWR backup 1 register

You can [`read`](crate::Reg::read) this register and get [`pwr_bkp1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_bkp1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_BKP1R)

For information about available fields see [`mod@pwr_bkp1r`] module*/
pub type PWR_BKP1R = crate::Reg<pwr_bkp1r::PWR_BKP1Rrs>;
///PWR backup 1 register
pub mod pwr_bkp1r;
/**PWR_BKP2R (rw) register accessor: PWR backup 2 register

You can [`read`](crate::Reg::read) this register and get [`pwr_bkp2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_bkp2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_BKP2R)

For information about available fields see [`mod@pwr_bkp2r`] module*/
pub type PWR_BKP2R = crate::Reg<pwr_bkp2r::PWR_BKP2Rrs>;
///PWR backup 2 register
pub mod pwr_bkp2r;
/**PWR_BKP3R (rw) register accessor: PWR backup 3 register

You can [`read`](crate::Reg::read) this register and get [`pwr_bkp3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_bkp3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_BKP3R)

For information about available fields see [`mod@pwr_bkp3r`] module*/
pub type PWR_BKP3R = crate::Reg<pwr_bkp3r::PWR_BKP3Rrs>;
///PWR backup 3 register
pub mod pwr_bkp3r;
