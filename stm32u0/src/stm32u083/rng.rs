#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    rng_cr: RNG_CR,
    rng_sr: RNG_SR,
    rng_dr: RNG_DR,
    _reserved3: [u8; 0x04],
    rng_htcr: RNG_HTCR,
}
impl RegisterBlock {
    ///0x00 - RNG control register
    #[inline(always)]
    pub const fn rng_cr(&self) -> &RNG_CR {
        &self.rng_cr
    }
    ///0x04 - RNG status register
    #[inline(always)]
    pub const fn rng_sr(&self) -> &RNG_SR {
        &self.rng_sr
    }
    ///0x08 - RNG data register
    #[inline(always)]
    pub const fn rng_dr(&self) -> &RNG_DR {
        &self.rng_dr
    }
    ///0x10 - RNG health test control register
    #[inline(always)]
    pub const fn rng_htcr(&self) -> &RNG_HTCR {
        &self.rng_htcr
    }
}
/**RNG_CR (rw) register accessor: RNG control register

You can [`read`](crate::Reg::read) this register and get [`rng_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RNG:RNG_CR)

For information about available fields see [`mod@rng_cr`]
module*/
pub type RNG_CR = crate::Reg<rng_cr::RNG_CRrs>;
///RNG control register
pub mod rng_cr;
/**RNG_SR (rw) register accessor: RNG status register

You can [`read`](crate::Reg::read) this register and get [`rng_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RNG:RNG_SR)

For information about available fields see [`mod@rng_sr`]
module*/
pub type RNG_SR = crate::Reg<rng_sr::RNG_SRrs>;
///RNG status register
pub mod rng_sr;
/**RNG_DR (r) register accessor: RNG data register

You can [`read`](crate::Reg::read) this register and get [`rng_dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RNG:RNG_DR)

For information about available fields see [`mod@rng_dr`]
module*/
pub type RNG_DR = crate::Reg<rng_dr::RNG_DRrs>;
///RNG data register
pub mod rng_dr;
/**RNG_HTCR (rw) register accessor: RNG health test control register

You can [`read`](crate::Reg::read) this register and get [`rng_htcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_htcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RNG:RNG_HTCR)

For information about available fields see [`mod@rng_htcr`]
module*/
pub type RNG_HTCR = crate::Reg<rng_htcr::RNG_HTCRrs>;
///RNG health test control register
pub mod rng_htcr;
