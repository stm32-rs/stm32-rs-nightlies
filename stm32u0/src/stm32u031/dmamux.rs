#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    c0cr: C0CR,
    c1cr: C1CR,
    c2cr: C2CR,
    c3cr: C3CR,
    c4cr: C4CR,
    c5cr: C5CR,
    c6cr: C6CR,
    c7cr: C7CR,
    c8cr: C8CR,
    c9cr: C9CR,
    c10cr: C10CR,
    c11cr: C11CR,
    _reserved12: [u8; 0x50],
    csr: CSR,
    cfr: CFR,
    _reserved14: [u8; 0x78],
    rg0cr: RG0CR,
    rg1cr: RG1CR,
    rg2cr: RG2CR,
    rg3cr: RG3CR,
    _reserved18: [u8; 0x30],
    rgsr: RGSR,
    rgcfr: RGCFR,
}
impl RegisterBlock {
    ///0x00 - DMAMUX request line multiplexer channel 0 configuration register
    #[inline(always)]
    pub const fn c0cr(&self) -> &C0CR {
        &self.c0cr
    }
    ///0x04 - DMAMUX request line multiplexer channel 1 configuration register
    #[inline(always)]
    pub const fn c1cr(&self) -> &C1CR {
        &self.c1cr
    }
    ///0x08 - DMAMUX request line multiplexer channel 2 configuration register
    #[inline(always)]
    pub const fn c2cr(&self) -> &C2CR {
        &self.c2cr
    }
    ///0x0c - DMAMUX request line multiplexer channel 3 configuration register
    #[inline(always)]
    pub const fn c3cr(&self) -> &C3CR {
        &self.c3cr
    }
    ///0x10 - DMAMUX request line multiplexer channel 4 configuration register
    #[inline(always)]
    pub const fn c4cr(&self) -> &C4CR {
        &self.c4cr
    }
    ///0x14 - DMAMUX request line multiplexer channel 5 configuration register
    #[inline(always)]
    pub const fn c5cr(&self) -> &C5CR {
        &self.c5cr
    }
    ///0x18 - DMAMUX request line multiplexer channel 6 configuration register
    #[inline(always)]
    pub const fn c6cr(&self) -> &C6CR {
        &self.c6cr
    }
    ///0x1c - DMAMUX request line multiplexer channel 7 configuration register
    #[inline(always)]
    pub const fn c7cr(&self) -> &C7CR {
        &self.c7cr
    }
    ///0x20 - DMAMUX request line multiplexer channel 8 configuration register
    #[inline(always)]
    pub const fn c8cr(&self) -> &C8CR {
        &self.c8cr
    }
    ///0x24 - DMAMUX request line multiplexer channel 9 configuration register
    #[inline(always)]
    pub const fn c9cr(&self) -> &C9CR {
        &self.c9cr
    }
    ///0x28 - DMAMUX request line multiplexer channel 10 configuration register
    #[inline(always)]
    pub const fn c10cr(&self) -> &C10CR {
        &self.c10cr
    }
    ///0x2c - DMAMUX request line multiplexer channel 11 configuration register
    #[inline(always)]
    pub const fn c11cr(&self) -> &C11CR {
        &self.c11cr
    }
    ///0x80 - DMAMUX request line multiplexer interrupt channel status register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    ///0x84 - DMAMUX request line multiplexer interrupt clear flag register
    #[inline(always)]
    pub const fn cfr(&self) -> &CFR {
        &self.cfr
    }
    ///0x100 - DMAMUX request generator channel 0 configuration register
    #[inline(always)]
    pub const fn rg0cr(&self) -> &RG0CR {
        &self.rg0cr
    }
    ///0x104 - DMAMUX request generator channel 1 configuration register
    #[inline(always)]
    pub const fn rg1cr(&self) -> &RG1CR {
        &self.rg1cr
    }
    ///0x108 - DMAMUX request generator channel 2 configuration register
    #[inline(always)]
    pub const fn rg2cr(&self) -> &RG2CR {
        &self.rg2cr
    }
    ///0x10c - DMAMUX request generator channel 3 configuration register
    #[inline(always)]
    pub const fn rg3cr(&self) -> &RG3CR {
        &self.rg3cr
    }
    ///0x140 - DMAMUX request generator interrupt status register
    #[inline(always)]
    pub const fn rgsr(&self) -> &RGSR {
        &self.rgsr
    }
    ///0x144 - DMAMUX request generator interrupt clear flag register
    #[inline(always)]
    pub const fn rgcfr(&self) -> &RGCFR {
        &self.rgcfr
    }
}
/**C0CR (rw) register accessor: DMAMUX request line multiplexer channel 0 configuration register

You can [`read`](crate::Reg::read) this register and get [`c0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DMAMUX:C0CR)

For information about available fields see [`mod@c0cr`]
module*/
pub type C0CR = crate::Reg<c0cr::C0CRrs>;
///DMAMUX request line multiplexer channel 0 configuration register
pub mod c0cr;
/**C1CR (rw) register accessor: DMAMUX request line multiplexer channel 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`c1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DMAMUX:C1CR)

For information about available fields see [`mod@c1cr`]
module*/
pub type C1CR = crate::Reg<c1cr::C1CRrs>;
///DMAMUX request line multiplexer channel 1 configuration register
pub mod c1cr;
/**C2CR (rw) register accessor: DMAMUX request line multiplexer channel 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`c2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DMAMUX:C2CR)

For information about available fields see [`mod@c2cr`]
module*/
pub type C2CR = crate::Reg<c2cr::C2CRrs>;
///DMAMUX request line multiplexer channel 2 configuration register
pub mod c2cr;
/**C3CR (rw) register accessor: DMAMUX request line multiplexer channel 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`c3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DMAMUX:C3CR)

For information about available fields see [`mod@c3cr`]
module*/
pub type C3CR = crate::Reg<c3cr::C3CRrs>;
///DMAMUX request line multiplexer channel 3 configuration register
pub mod c3cr;
/**C4CR (rw) register accessor: DMAMUX request line multiplexer channel 4 configuration register

You can [`read`](crate::Reg::read) this register and get [`c4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DMAMUX:C4CR)

For information about available fields see [`mod@c4cr`]
module*/
pub type C4CR = crate::Reg<c4cr::C4CRrs>;
///DMAMUX request line multiplexer channel 4 configuration register
pub mod c4cr;
/**C5CR (rw) register accessor: DMAMUX request line multiplexer channel 5 configuration register

You can [`read`](crate::Reg::read) this register and get [`c5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DMAMUX:C5CR)

For information about available fields see [`mod@c5cr`]
module*/
pub type C5CR = crate::Reg<c5cr::C5CRrs>;
///DMAMUX request line multiplexer channel 5 configuration register
pub mod c5cr;
/**C6CR (rw) register accessor: DMAMUX request line multiplexer channel 6 configuration register

You can [`read`](crate::Reg::read) this register and get [`c6cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DMAMUX:C6CR)

For information about available fields see [`mod@c6cr`]
module*/
pub type C6CR = crate::Reg<c6cr::C6CRrs>;
///DMAMUX request line multiplexer channel 6 configuration register
pub mod c6cr;
/**C7CR (rw) register accessor: DMAMUX request line multiplexer channel 7 configuration register

You can [`read`](crate::Reg::read) this register and get [`c7cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DMAMUX:C7CR)

For information about available fields see [`mod@c7cr`]
module*/
pub type C7CR = crate::Reg<c7cr::C7CRrs>;
///DMAMUX request line multiplexer channel 7 configuration register
pub mod c7cr;
/**C8CR (rw) register accessor: DMAMUX request line multiplexer channel 8 configuration register

You can [`read`](crate::Reg::read) this register and get [`c8cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c8cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DMAMUX:C8CR)

For information about available fields see [`mod@c8cr`]
module*/
pub type C8CR = crate::Reg<c8cr::C8CRrs>;
///DMAMUX request line multiplexer channel 8 configuration register
pub mod c8cr;
/**C9CR (rw) register accessor: DMAMUX request line multiplexer channel 9 configuration register

You can [`read`](crate::Reg::read) this register and get [`c9cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DMAMUX:C9CR)

For information about available fields see [`mod@c9cr`]
module*/
pub type C9CR = crate::Reg<c9cr::C9CRrs>;
///DMAMUX request line multiplexer channel 9 configuration register
pub mod c9cr;
/**C10CR (rw) register accessor: DMAMUX request line multiplexer channel 10 configuration register

You can [`read`](crate::Reg::read) this register and get [`c10cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DMAMUX:C10CR)

For information about available fields see [`mod@c10cr`]
module*/
pub type C10CR = crate::Reg<c10cr::C10CRrs>;
///DMAMUX request line multiplexer channel 10 configuration register
pub mod c10cr;
/**C11CR (rw) register accessor: DMAMUX request line multiplexer channel 11 configuration register

You can [`read`](crate::Reg::read) this register and get [`c11cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c11cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DMAMUX:C11CR)

For information about available fields see [`mod@c11cr`]
module*/
pub type C11CR = crate::Reg<c11cr::C11CRrs>;
///DMAMUX request line multiplexer channel 11 configuration register
pub mod c11cr;
/**CSR (r) register accessor: DMAMUX request line multiplexer interrupt channel status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DMAMUX:CSR)

For information about available fields see [`mod@csr`]
module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///DMAMUX request line multiplexer interrupt channel status register
pub mod csr;
/**CFR (w) register accessor: DMAMUX request line multiplexer interrupt clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DMAMUX:CFR)

For information about available fields see [`mod@cfr`]
module*/
pub type CFR = crate::Reg<cfr::CFRrs>;
///DMAMUX request line multiplexer interrupt clear flag register
pub mod cfr;
/**RG0CR (rw) register accessor: DMAMUX request generator channel 0 configuration register

You can [`read`](crate::Reg::read) this register and get [`rg0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DMAMUX:RG0CR)

For information about available fields see [`mod@rg0cr`]
module*/
pub type RG0CR = crate::Reg<rg0cr::RG0CRrs>;
///DMAMUX request generator channel 0 configuration register
pub mod rg0cr;
/**RG1CR (rw) register accessor: DMAMUX request generator channel 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`rg1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DMAMUX:RG1CR)

For information about available fields see [`mod@rg1cr`]
module*/
pub type RG1CR = crate::Reg<rg1cr::RG1CRrs>;
///DMAMUX request generator channel 1 configuration register
pub mod rg1cr;
/**RG2CR (rw) register accessor: DMAMUX request generator channel 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`rg2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DMAMUX:RG2CR)

For information about available fields see [`mod@rg2cr`]
module*/
pub type RG2CR = crate::Reg<rg2cr::RG2CRrs>;
///DMAMUX request generator channel 2 configuration register
pub mod rg2cr;
/**RG3CR (rw) register accessor: DMAMUX request generator channel 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`rg3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DMAMUX:RG3CR)

For information about available fields see [`mod@rg3cr`]
module*/
pub type RG3CR = crate::Reg<rg3cr::RG3CRrs>;
///DMAMUX request generator channel 3 configuration register
pub mod rg3cr;
/**RGSR (r) register accessor: DMAMUX request generator interrupt status register

You can [`read`](crate::Reg::read) this register and get [`rgsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DMAMUX:RGSR)

For information about available fields see [`mod@rgsr`]
module*/
pub type RGSR = crate::Reg<rgsr::RGSRrs>;
///DMAMUX request generator interrupt status register
pub mod rgsr;
/**RGCFR (w) register accessor: DMAMUX request generator interrupt clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgcfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DMAMUX:RGCFR)

For information about available fields see [`mod@rgcfr`]
module*/
pub type RGCFR = crate::Reg<rgcfr::RGCFRrs>;
///DMAMUX request generator interrupt clear flag register
pub mod rgcfr;
