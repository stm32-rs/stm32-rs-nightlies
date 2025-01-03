#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    pssi_cr: PSSI_CR,
    pssi_sr: PSSI_SR,
    pssi_ris: PSSI_RIS,
    pssi_ier: PSSI_IER,
    pssi_mis: PSSI_MIS,
    pssi_icr: PSSI_ICR,
    _reserved6: [u8; 0x10],
    pssi_dr: PSSI_DR,
}
impl RegisterBlock {
    ///0x00 -
    #[inline(always)]
    pub const fn pssi_cr(&self) -> &PSSI_CR {
        &self.pssi_cr
    }
    ///0x04 -
    #[inline(always)]
    pub const fn pssi_sr(&self) -> &PSSI_SR {
        &self.pssi_sr
    }
    ///0x08 - PSSI raw interrupt status register
    #[inline(always)]
    pub const fn pssi_ris(&self) -> &PSSI_RIS {
        &self.pssi_ris
    }
    ///0x0c -
    #[inline(always)]
    pub const fn pssi_ier(&self) -> &PSSI_IER {
        &self.pssi_ier
    }
    ///0x10 - PSSI masked interrupt status register
    #[inline(always)]
    pub const fn pssi_mis(&self) -> &PSSI_MIS {
        &self.pssi_mis
    }
    ///0x14 - PSSI interrupt clear register
    #[inline(always)]
    pub const fn pssi_icr(&self) -> &PSSI_ICR {
        &self.pssi_icr
    }
    ///0x28 - PSSI data register
    #[inline(always)]
    pub const fn pssi_dr(&self) -> &PSSI_DR {
        &self.pssi_dr
    }
}
/**PSSI_CR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`pssi_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pssi_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#PSSI:PSSI_CR)

For information about available fields see [`mod@pssi_cr`]
module*/
pub type PSSI_CR = crate::Reg<pssi_cr::PSSI_CRrs>;
///
pub mod pssi_cr;
/**PSSI_SR (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`pssi_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#PSSI:PSSI_SR)

For information about available fields see [`mod@pssi_sr`]
module*/
pub type PSSI_SR = crate::Reg<pssi_sr::PSSI_SRrs>;
///
pub mod pssi_sr;
/**PSSI_RIS (r) register accessor: PSSI raw interrupt status register

You can [`read`](crate::Reg::read) this register and get [`pssi_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#PSSI:PSSI_RIS)

For information about available fields see [`mod@pssi_ris`]
module*/
pub type PSSI_RIS = crate::Reg<pssi_ris::PSSI_RISrs>;
///PSSI raw interrupt status register
pub mod pssi_ris;
/**PSSI_IER (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`pssi_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pssi_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#PSSI:PSSI_IER)

For information about available fields see [`mod@pssi_ier`]
module*/
pub type PSSI_IER = crate::Reg<pssi_ier::PSSI_IERrs>;
///
pub mod pssi_ier;
/**PSSI_MIS (r) register accessor: PSSI masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`pssi_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#PSSI:PSSI_MIS)

For information about available fields see [`mod@pssi_mis`]
module*/
pub type PSSI_MIS = crate::Reg<pssi_mis::PSSI_MISrs>;
///PSSI masked interrupt status register
pub mod pssi_mis;
/**PSSI_ICR (w) register accessor: PSSI interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pssi_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#PSSI:PSSI_ICR)

For information about available fields see [`mod@pssi_icr`]
module*/
pub type PSSI_ICR = crate::Reg<pssi_icr::PSSI_ICRrs>;
///PSSI interrupt clear register
pub mod pssi_icr;
/**PSSI_DR (rw) register accessor: PSSI data register

You can [`read`](crate::Reg::read) this register and get [`pssi_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pssi_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#PSSI:PSSI_DR)

For information about available fields see [`mod@pssi_dr`]
module*/
pub type PSSI_DR = crate::Reg<pssi_dr::PSSI_DRrs>;
///PSSI data register
pub mod pssi_dr;
