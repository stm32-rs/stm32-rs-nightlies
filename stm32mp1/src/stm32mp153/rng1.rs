#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    rng_cr: RNG_CR,
    rng_sr: RNG_SR,
    rng_dr: RNG_DR,
    _reserved3: [u8; 0x03e4],
    rng_hwcfgr: RNG_HWCFGR,
    rng_verr: RNG_VERR,
    rng_ipidr: RNG_IPIDR,
    rng_sidr: RNG_SIDR,
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
    ///0x08 - The RNG_DR register is a read-only register.
    #[inline(always)]
    pub const fn rng_dr(&self) -> &RNG_DR {
        &self.rng_dr
    }
    ///0x3f0 - RNG hardware configuration register
    #[inline(always)]
    pub const fn rng_hwcfgr(&self) -> &RNG_HWCFGR {
        &self.rng_hwcfgr
    }
    ///0x3f4 - RNG version register
    #[inline(always)]
    pub const fn rng_verr(&self) -> &RNG_VERR {
        &self.rng_verr
    }
    ///0x3f8 - RNG identification register
    #[inline(always)]
    pub const fn rng_ipidr(&self) -> &RNG_IPIDR {
        &self.rng_ipidr
    }
    ///0x3fc - RNG size ID register
    #[inline(always)]
    pub const fn rng_sidr(&self) -> &RNG_SIDR {
        &self.rng_sidr
    }
}
/**RNG_CR (rw) register accessor: RNG control register

You can [`read`](crate::Reg::read) this register and get [`rng_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RNG1:RNG_CR)

For information about available fields see [`mod@rng_cr`]
module*/
pub type RNG_CR = crate::Reg<rng_cr::RNG_CRrs>;
///RNG control register
pub mod rng_cr;
/**RNG_SR (rw) register accessor: RNG status register

You can [`read`](crate::Reg::read) this register and get [`rng_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RNG1:RNG_SR)

For information about available fields see [`mod@rng_sr`]
module*/
pub type RNG_SR = crate::Reg<rng_sr::RNG_SRrs>;
///RNG status register
pub mod rng_sr;
/**RNG_DR (r) register accessor: The RNG_DR register is a read-only register.

You can [`read`](crate::Reg::read) this register and get [`rng_dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RNG1:RNG_DR)

For information about available fields see [`mod@rng_dr`]
module*/
pub type RNG_DR = crate::Reg<rng_dr::RNG_DRrs>;
///The RNG_DR register is a read-only register.
pub mod rng_dr;
/**RNG_HWCFGR (r) register accessor: RNG hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`rng_hwcfgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RNG1:RNG_HWCFGR)

For information about available fields see [`mod@rng_hwcfgr`]
module*/
pub type RNG_HWCFGR = crate::Reg<rng_hwcfgr::RNG_HWCFGRrs>;
///RNG hardware configuration register
pub mod rng_hwcfgr;
/**RNG_VERR (r) register accessor: RNG version register

You can [`read`](crate::Reg::read) this register and get [`rng_verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RNG1:RNG_VERR)

For information about available fields see [`mod@rng_verr`]
module*/
pub type RNG_VERR = crate::Reg<rng_verr::RNG_VERRrs>;
///RNG version register
pub mod rng_verr;
/**RNG_IPIDR (r) register accessor: RNG identification register

You can [`read`](crate::Reg::read) this register and get [`rng_ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RNG1:RNG_IPIDR)

For information about available fields see [`mod@rng_ipidr`]
module*/
pub type RNG_IPIDR = crate::Reg<rng_ipidr::RNG_IPIDRrs>;
///RNG identification register
pub mod rng_ipidr;
/**RNG_SIDR (r) register accessor: RNG size ID register

You can [`read`](crate::Reg::read) this register and get [`rng_sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RNG1:RNG_SIDR)

For information about available fields see [`mod@rng_sidr`]
module*/
pub type RNG_SIDR = crate::Reg<rng_sidr::RNG_SIDRrs>;
///RNG size ID register
pub mod rng_sidr;
