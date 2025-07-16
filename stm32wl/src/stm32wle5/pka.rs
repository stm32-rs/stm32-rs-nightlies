#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    clrfr: CLRFR,
}
impl RegisterBlock {
    ///0x00 - control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x08 - clear flag register
    #[inline(always)]
    pub const fn clrfr(&self) -> &CLRFR {
        &self.clrfr
    }
}
/**CR (rw) register accessor: control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#PKA:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///control register
pub mod cr;
/**SR (r) register accessor: status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#PKA:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///status register
pub mod sr;
/**CLRFR (w) register accessor: clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clrfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#PKA:CLRFR)

For information about available fields see [`mod@clrfr`] module*/
pub type CLRFR = crate::Reg<clrfr::CLRFRrs>;
///clear flag register
pub mod clrfr;
