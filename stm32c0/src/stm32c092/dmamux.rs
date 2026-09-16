#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dmamux_c0cr: DMAMUX_C0CR,
    dmamux_c1cr: DMAMUX_C1CR,
    dmamux_c2cr: DMAMUX_C2CR,
    dmamux_c3cr: DMAMUX_C3CR,
    dmamux_c4cr: DMAMUX_C4CR,
    _reserved5: [u8; 0x6c],
    dmamux_csr: DMAMUX_CSR,
    dmamux_cfr: DMAMUX_CFR,
    _reserved7: [u8; 0x78],
    dmamux_rg0cr: DMAMUX_RG0CR,
    dmamux_rg1cr: DMAMUX_RG1CR,
    dmamux_rg2cr: DMAMUX_RG2CR,
    dmamux_rg3cr: DMAMUX_RG3CR,
    _reserved11: [u8; 0x30],
    dmamux_rgsr: DMAMUX_RGSR,
    dmamux_rgcfr: DMAMUX_RGCFR,
}
impl RegisterBlock {
    ///0x00 - DMAMUX request line multiplexer channel 0 configuration register
    #[inline(always)]
    pub const fn dmamux_c0cr(&self) -> &DMAMUX_C0CR {
        &self.dmamux_c0cr
    }
    ///0x04 - DMAMUX request line multiplexer channel 1 configuration register
    #[inline(always)]
    pub const fn dmamux_c1cr(&self) -> &DMAMUX_C1CR {
        &self.dmamux_c1cr
    }
    ///0x08 - DMAMUX request line multiplexer channel 2 configuration register
    #[inline(always)]
    pub const fn dmamux_c2cr(&self) -> &DMAMUX_C2CR {
        &self.dmamux_c2cr
    }
    ///0x0c - DMAMUX request line multiplexer channel 3 configuration register
    #[inline(always)]
    pub const fn dmamux_c3cr(&self) -> &DMAMUX_C3CR {
        &self.dmamux_c3cr
    }
    ///0x10 - DMAMUX request line multiplexer channel 4 configuration register
    #[inline(always)]
    pub const fn dmamux_c4cr(&self) -> &DMAMUX_C4CR {
        &self.dmamux_c4cr
    }
    ///0x80 - DMAMUX request line multiplexer interrupt channel status register
    #[inline(always)]
    pub const fn dmamux_csr(&self) -> &DMAMUX_CSR {
        &self.dmamux_csr
    }
    ///0x84 - DMAMUX request line multiplexer interrupt clear flag register
    #[inline(always)]
    pub const fn dmamux_cfr(&self) -> &DMAMUX_CFR {
        &self.dmamux_cfr
    }
    ///0x100 - DMAMUX request generator channel 0 configuration register
    #[inline(always)]
    pub const fn dmamux_rg0cr(&self) -> &DMAMUX_RG0CR {
        &self.dmamux_rg0cr
    }
    ///0x104 - DMAMUX request generator channel 1 configuration register
    #[inline(always)]
    pub const fn dmamux_rg1cr(&self) -> &DMAMUX_RG1CR {
        &self.dmamux_rg1cr
    }
    ///0x108 - DMAMUX request generator channel 2 configuration register
    #[inline(always)]
    pub const fn dmamux_rg2cr(&self) -> &DMAMUX_RG2CR {
        &self.dmamux_rg2cr
    }
    ///0x10c - DMAMUX request generator channel 3 configuration register
    #[inline(always)]
    pub const fn dmamux_rg3cr(&self) -> &DMAMUX_RG3CR {
        &self.dmamux_rg3cr
    }
    ///0x140 - DMAMUX request generator interrupt status register
    #[inline(always)]
    pub const fn dmamux_rgsr(&self) -> &DMAMUX_RGSR {
        &self.dmamux_rgsr
    }
    ///0x144 - DMAMUX request generator interrupt clear flag register
    #[inline(always)]
    pub const fn dmamux_rgcfr(&self) -> &DMAMUX_RGCFR {
        &self.dmamux_rgcfr
    }
}
/**DMAMUX_C0CR (rw) register accessor: DMAMUX request line multiplexer channel 0 configuration register

You can [`read`](crate::Reg::read) this register and get [`dmamux_c0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamux_c0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#DMAMUX:DMAMUX_C0CR)

For information about available fields see [`mod@dmamux_c0cr`] module*/
pub type DMAMUX_C0CR = crate::Reg<dmamux_c0cr::DMAMUX_C0CRrs>;
///DMAMUX request line multiplexer channel 0 configuration register
pub mod dmamux_c0cr;
/**DMAMUX_C1CR (rw) register accessor: DMAMUX request line multiplexer channel 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`dmamux_c1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamux_c1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#DMAMUX:DMAMUX_C1CR)

For information about available fields see [`mod@dmamux_c1cr`] module*/
pub type DMAMUX_C1CR = crate::Reg<dmamux_c1cr::DMAMUX_C1CRrs>;
///DMAMUX request line multiplexer channel 1 configuration register
pub mod dmamux_c1cr;
/**DMAMUX_C2CR (rw) register accessor: DMAMUX request line multiplexer channel 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`dmamux_c2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamux_c2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#DMAMUX:DMAMUX_C2CR)

For information about available fields see [`mod@dmamux_c2cr`] module*/
pub type DMAMUX_C2CR = crate::Reg<dmamux_c2cr::DMAMUX_C2CRrs>;
///DMAMUX request line multiplexer channel 2 configuration register
pub mod dmamux_c2cr;
/**DMAMUX_C3CR (rw) register accessor: DMAMUX request line multiplexer channel 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`dmamux_c3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamux_c3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#DMAMUX:DMAMUX_C3CR)

For information about available fields see [`mod@dmamux_c3cr`] module*/
pub type DMAMUX_C3CR = crate::Reg<dmamux_c3cr::DMAMUX_C3CRrs>;
///DMAMUX request line multiplexer channel 3 configuration register
pub mod dmamux_c3cr;
/**DMAMUX_C4CR (rw) register accessor: DMAMUX request line multiplexer channel 4 configuration register

You can [`read`](crate::Reg::read) this register and get [`dmamux_c4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamux_c4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#DMAMUX:DMAMUX_C4CR)

For information about available fields see [`mod@dmamux_c4cr`] module*/
pub type DMAMUX_C4CR = crate::Reg<dmamux_c4cr::DMAMUX_C4CRrs>;
///DMAMUX request line multiplexer channel 4 configuration register
pub mod dmamux_c4cr;
/**DMAMUX_CSR (r) register accessor: DMAMUX request line multiplexer interrupt channel status register

You can [`read`](crate::Reg::read) this register and get [`dmamux_csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#DMAMUX:DMAMUX_CSR)

For information about available fields see [`mod@dmamux_csr`] module*/
pub type DMAMUX_CSR = crate::Reg<dmamux_csr::DMAMUX_CSRrs>;
///DMAMUX request line multiplexer interrupt channel status register
pub mod dmamux_csr;
/**DMAMUX_CFR (w) register accessor: DMAMUX request line multiplexer interrupt clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamux_cfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#DMAMUX:DMAMUX_CFR)

For information about available fields see [`mod@dmamux_cfr`] module*/
pub type DMAMUX_CFR = crate::Reg<dmamux_cfr::DMAMUX_CFRrs>;
///DMAMUX request line multiplexer interrupt clear flag register
pub mod dmamux_cfr;
/**DMAMUX_RG0CR (rw) register accessor: DMAMUX request generator channel 0 configuration register

You can [`read`](crate::Reg::read) this register and get [`dmamux_rg0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamux_rg0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#DMAMUX:DMAMUX_RG0CR)

For information about available fields see [`mod@dmamux_rg0cr`] module*/
pub type DMAMUX_RG0CR = crate::Reg<dmamux_rg0cr::DMAMUX_RG0CRrs>;
///DMAMUX request generator channel 0 configuration register
pub mod dmamux_rg0cr;
/**DMAMUX_RG1CR (rw) register accessor: DMAMUX request generator channel 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`dmamux_rg1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamux_rg1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#DMAMUX:DMAMUX_RG1CR)

For information about available fields see [`mod@dmamux_rg1cr`] module*/
pub type DMAMUX_RG1CR = crate::Reg<dmamux_rg1cr::DMAMUX_RG1CRrs>;
///DMAMUX request generator channel 1 configuration register
pub mod dmamux_rg1cr;
/**DMAMUX_RG2CR (rw) register accessor: DMAMUX request generator channel 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`dmamux_rg2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamux_rg2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#DMAMUX:DMAMUX_RG2CR)

For information about available fields see [`mod@dmamux_rg2cr`] module*/
pub type DMAMUX_RG2CR = crate::Reg<dmamux_rg2cr::DMAMUX_RG2CRrs>;
///DMAMUX request generator channel 2 configuration register
pub mod dmamux_rg2cr;
/**DMAMUX_RG3CR (rw) register accessor: DMAMUX request generator channel 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`dmamux_rg3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamux_rg3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#DMAMUX:DMAMUX_RG3CR)

For information about available fields see [`mod@dmamux_rg3cr`] module*/
pub type DMAMUX_RG3CR = crate::Reg<dmamux_rg3cr::DMAMUX_RG3CRrs>;
///DMAMUX request generator channel 3 configuration register
pub mod dmamux_rg3cr;
/**DMAMUX_RGSR (r) register accessor: DMAMUX request generator interrupt status register

You can [`read`](crate::Reg::read) this register and get [`dmamux_rgsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#DMAMUX:DMAMUX_RGSR)

For information about available fields see [`mod@dmamux_rgsr`] module*/
pub type DMAMUX_RGSR = crate::Reg<dmamux_rgsr::DMAMUX_RGSRrs>;
///DMAMUX request generator interrupt status register
pub mod dmamux_rgsr;
/**DMAMUX_RGCFR (w) register accessor: DMAMUX request generator interrupt clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamux_rgcfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#DMAMUX:DMAMUX_RGCFR)

For information about available fields see [`mod@dmamux_rgcfr`] module*/
pub type DMAMUX_RGCFR = crate::Reg<dmamux_rgcfr::DMAMUX_RGCFRrs>;
///DMAMUX request generator interrupt clear flag register
pub mod dmamux_rgcfr;
