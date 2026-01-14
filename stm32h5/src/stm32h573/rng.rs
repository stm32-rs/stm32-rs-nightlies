#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    dr: DR,
    nscr: NSCR,
    htcr: HTCR,
}
impl RegisterBlock {
    ///0x00 - RNG control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - RNG status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x08 - RNG data register
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    ///0x0c - RNG noise source control register
    #[inline(always)]
    pub const fn nscr(&self) -> &NSCR {
        &self.nscr
    }
    ///0x10 - RNG health test control register
    #[inline(always)]
    pub const fn htcr(&self) -> &HTCR {
        &self.htcr
    }
}
/**CR (rw) register accessor: RNG control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RNG:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///RNG control register
pub mod cr;
/**SR (rw) register accessor: RNG status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RNG:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///RNG status register
pub mod sr;
/**DR (r) register accessor: RNG data register

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RNG:DR)

For information about available fields see [`mod@dr`] module*/
pub type DR = crate::Reg<dr::DRrs>;
///RNG data register
pub mod dr;
/**NSCR (rw) register accessor: RNG noise source control register

You can [`read`](crate::Reg::read) this register and get [`nscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RNG:NSCR)

For information about available fields see [`mod@nscr`] module*/
pub type NSCR = crate::Reg<nscr::NSCRrs>;
///RNG noise source control register
pub mod nscr;
/**HTCR (rw) register accessor: RNG health test control register

You can [`read`](crate::Reg::read) this register and get [`htcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`htcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RNG:HTCR)

For information about available fields see [`mod@htcr`] module*/
pub type HTCR = crate::Reg<htcr::HTCRrs>;
///RNG health test control register
pub mod htcr;
