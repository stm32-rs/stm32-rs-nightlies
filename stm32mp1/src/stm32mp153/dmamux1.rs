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
    c12cr: C12CR,
    c13cr: C13CR,
    c14cr: C14CR,
    c15cr: C15CR,
    _reserved16: [u8; 0x40],
    csr: CSR,
    cfr: CFR,
    _reserved18: [u8; 0x78],
    rg0cr: RG0CR,
    rg1cr: RG1CR,
    rg2cr: RG2CR,
    rg3cr: RG3CR,
    rg4cr: RG4CR,
    rg5cr: RG5CR,
    rg6cr: RG6CR,
    rg7cr: RG7CR,
    _reserved26: [u8; 0x20],
    rgsr: RGSR,
    rgcfr: RGCFR,
    _reserved28: [u8; 0x02a4],
    hwcfgr2: HWCFGR2,
    hwcfgr1: HWCFGR1,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
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
    ///0x30 - DMAMUX request line multiplexer channel 12 configuration register
    #[inline(always)]
    pub const fn c12cr(&self) -> &C12CR {
        &self.c12cr
    }
    ///0x34 - DMAMUX request line multiplexer channel 13 configuration register
    #[inline(always)]
    pub const fn c13cr(&self) -> &C13CR {
        &self.c13cr
    }
    ///0x38 - DMAMUX request line multiplexer channel 14 configuration register
    #[inline(always)]
    pub const fn c14cr(&self) -> &C14CR {
        &self.c14cr
    }
    ///0x3c - DMAMUX request line multiplexer channel 15 configuration register
    #[inline(always)]
    pub const fn c15cr(&self) -> &C15CR {
        &self.c15cr
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
    ///0x110 - DMAMUX request generator channel 4 configuration register
    #[inline(always)]
    pub const fn rg4cr(&self) -> &RG4CR {
        &self.rg4cr
    }
    ///0x114 - DMAMUX request generator channel 5 configuration register
    #[inline(always)]
    pub const fn rg5cr(&self) -> &RG5CR {
        &self.rg5cr
    }
    ///0x118 - DMAMUX request generator channel 6 configuration register
    #[inline(always)]
    pub const fn rg6cr(&self) -> &RG6CR {
        &self.rg6cr
    }
    ///0x11c - DMAMUX request generator channel 7 configuration register
    #[inline(always)]
    pub const fn rg7cr(&self) -> &RG7CR {
        &self.rg7cr
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
    ///0x3ec - DMAMUX hardware configuration 2 register
    #[inline(always)]
    pub const fn hwcfgr2(&self) -> &HWCFGR2 {
        &self.hwcfgr2
    }
    ///0x3f0 - DMAMUX hardware configuration 1 register
    #[inline(always)]
    pub const fn hwcfgr1(&self) -> &HWCFGR1 {
        &self.hwcfgr1
    }
    ///0x3f4 - This register identifies the IP version.
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - This register identifies the IP.
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0x3fc - DMAMUX size identification register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**C0CR (rw) register accessor: DMAMUX request line multiplexer channel 0 configuration register

You can [`read`](crate::Reg::read) this register and get [`c0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:C0CR)

For information about available fields see [`mod@c0cr`] module*/
pub type C0CR = crate::Reg<c0cr::C0CRrs>;
///DMAMUX request line multiplexer channel 0 configuration register
pub mod c0cr;
/**C1CR (rw) register accessor: DMAMUX request line multiplexer channel 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`c1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:C1CR)

For information about available fields see [`mod@c1cr`] module*/
pub type C1CR = crate::Reg<c1cr::C1CRrs>;
///DMAMUX request line multiplexer channel 1 configuration register
pub mod c1cr;
/**C2CR (rw) register accessor: DMAMUX request line multiplexer channel 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`c2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:C2CR)

For information about available fields see [`mod@c2cr`] module*/
pub type C2CR = crate::Reg<c2cr::C2CRrs>;
///DMAMUX request line multiplexer channel 2 configuration register
pub mod c2cr;
/**C3CR (rw) register accessor: DMAMUX request line multiplexer channel 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`c3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:C3CR)

For information about available fields see [`mod@c3cr`] module*/
pub type C3CR = crate::Reg<c3cr::C3CRrs>;
///DMAMUX request line multiplexer channel 3 configuration register
pub mod c3cr;
/**C4CR (rw) register accessor: DMAMUX request line multiplexer channel 4 configuration register

You can [`read`](crate::Reg::read) this register and get [`c4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:C4CR)

For information about available fields see [`mod@c4cr`] module*/
pub type C4CR = crate::Reg<c4cr::C4CRrs>;
///DMAMUX request line multiplexer channel 4 configuration register
pub mod c4cr;
/**C5CR (rw) register accessor: DMAMUX request line multiplexer channel 5 configuration register

You can [`read`](crate::Reg::read) this register and get [`c5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:C5CR)

For information about available fields see [`mod@c5cr`] module*/
pub type C5CR = crate::Reg<c5cr::C5CRrs>;
///DMAMUX request line multiplexer channel 5 configuration register
pub mod c5cr;
/**C6CR (rw) register accessor: DMAMUX request line multiplexer channel 6 configuration register

You can [`read`](crate::Reg::read) this register and get [`c6cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:C6CR)

For information about available fields see [`mod@c6cr`] module*/
pub type C6CR = crate::Reg<c6cr::C6CRrs>;
///DMAMUX request line multiplexer channel 6 configuration register
pub mod c6cr;
/**C7CR (rw) register accessor: DMAMUX request line multiplexer channel 7 configuration register

You can [`read`](crate::Reg::read) this register and get [`c7cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:C7CR)

For information about available fields see [`mod@c7cr`] module*/
pub type C7CR = crate::Reg<c7cr::C7CRrs>;
///DMAMUX request line multiplexer channel 7 configuration register
pub mod c7cr;
/**C8CR (rw) register accessor: DMAMUX request line multiplexer channel 8 configuration register

You can [`read`](crate::Reg::read) this register and get [`c8cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c8cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:C8CR)

For information about available fields see [`mod@c8cr`] module*/
pub type C8CR = crate::Reg<c8cr::C8CRrs>;
///DMAMUX request line multiplexer channel 8 configuration register
pub mod c8cr;
/**C9CR (rw) register accessor: DMAMUX request line multiplexer channel 9 configuration register

You can [`read`](crate::Reg::read) this register and get [`c9cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:C9CR)

For information about available fields see [`mod@c9cr`] module*/
pub type C9CR = crate::Reg<c9cr::C9CRrs>;
///DMAMUX request line multiplexer channel 9 configuration register
pub mod c9cr;
/**C10CR (rw) register accessor: DMAMUX request line multiplexer channel 10 configuration register

You can [`read`](crate::Reg::read) this register and get [`c10cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:C10CR)

For information about available fields see [`mod@c10cr`] module*/
pub type C10CR = crate::Reg<c10cr::C10CRrs>;
///DMAMUX request line multiplexer channel 10 configuration register
pub mod c10cr;
/**C11CR (rw) register accessor: DMAMUX request line multiplexer channel 11 configuration register

You can [`read`](crate::Reg::read) this register and get [`c11cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c11cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:C11CR)

For information about available fields see [`mod@c11cr`] module*/
pub type C11CR = crate::Reg<c11cr::C11CRrs>;
///DMAMUX request line multiplexer channel 11 configuration register
pub mod c11cr;
/**C12CR (rw) register accessor: DMAMUX request line multiplexer channel 12 configuration register

You can [`read`](crate::Reg::read) this register and get [`c12cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:C12CR)

For information about available fields see [`mod@c12cr`] module*/
pub type C12CR = crate::Reg<c12cr::C12CRrs>;
///DMAMUX request line multiplexer channel 12 configuration register
pub mod c12cr;
/**C13CR (rw) register accessor: DMAMUX request line multiplexer channel 13 configuration register

You can [`read`](crate::Reg::read) this register and get [`c13cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:C13CR)

For information about available fields see [`mod@c13cr`] module*/
pub type C13CR = crate::Reg<c13cr::C13CRrs>;
///DMAMUX request line multiplexer channel 13 configuration register
pub mod c13cr;
/**C14CR (rw) register accessor: DMAMUX request line multiplexer channel 14 configuration register

You can [`read`](crate::Reg::read) this register and get [`c14cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:C14CR)

For information about available fields see [`mod@c14cr`] module*/
pub type C14CR = crate::Reg<c14cr::C14CRrs>;
///DMAMUX request line multiplexer channel 14 configuration register
pub mod c14cr;
/**C15CR (rw) register accessor: DMAMUX request line multiplexer channel 15 configuration register

You can [`read`](crate::Reg::read) this register and get [`c15cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:C15CR)

For information about available fields see [`mod@c15cr`] module*/
pub type C15CR = crate::Reg<c15cr::C15CRrs>;
///DMAMUX request line multiplexer channel 15 configuration register
pub mod c15cr;
/**CSR (r) register accessor: DMAMUX request line multiplexer interrupt channel status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///DMAMUX request line multiplexer interrupt channel status register
pub mod csr;
/**CFR (w) register accessor: DMAMUX request line multiplexer interrupt clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:CFR)

For information about available fields see [`mod@cfr`] module*/
pub type CFR = crate::Reg<cfr::CFRrs>;
///DMAMUX request line multiplexer interrupt clear flag register
pub mod cfr;
/**RG0CR (rw) register accessor: DMAMUX request generator channel 0 configuration register

You can [`read`](crate::Reg::read) this register and get [`rg0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:RG0CR)

For information about available fields see [`mod@rg0cr`] module*/
pub type RG0CR = crate::Reg<rg0cr::RG0CRrs>;
///DMAMUX request generator channel 0 configuration register
pub mod rg0cr;
/**RG1CR (rw) register accessor: DMAMUX request generator channel 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`rg1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:RG1CR)

For information about available fields see [`mod@rg1cr`] module*/
pub type RG1CR = crate::Reg<rg1cr::RG1CRrs>;
///DMAMUX request generator channel 1 configuration register
pub mod rg1cr;
/**RG2CR (rw) register accessor: DMAMUX request generator channel 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`rg2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:RG2CR)

For information about available fields see [`mod@rg2cr`] module*/
pub type RG2CR = crate::Reg<rg2cr::RG2CRrs>;
///DMAMUX request generator channel 2 configuration register
pub mod rg2cr;
/**RG3CR (rw) register accessor: DMAMUX request generator channel 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`rg3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:RG3CR)

For information about available fields see [`mod@rg3cr`] module*/
pub type RG3CR = crate::Reg<rg3cr::RG3CRrs>;
///DMAMUX request generator channel 3 configuration register
pub mod rg3cr;
/**RG4CR (rw) register accessor: DMAMUX request generator channel 4 configuration register

You can [`read`](crate::Reg::read) this register and get [`rg4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:RG4CR)

For information about available fields see [`mod@rg4cr`] module*/
pub type RG4CR = crate::Reg<rg4cr::RG4CRrs>;
///DMAMUX request generator channel 4 configuration register
pub mod rg4cr;
/**RG5CR (rw) register accessor: DMAMUX request generator channel 5 configuration register

You can [`read`](crate::Reg::read) this register and get [`rg5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:RG5CR)

For information about available fields see [`mod@rg5cr`] module*/
pub type RG5CR = crate::Reg<rg5cr::RG5CRrs>;
///DMAMUX request generator channel 5 configuration register
pub mod rg5cr;
/**RG6CR (rw) register accessor: DMAMUX request generator channel 6 configuration register

You can [`read`](crate::Reg::read) this register and get [`rg6cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg6cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:RG6CR)

For information about available fields see [`mod@rg6cr`] module*/
pub type RG6CR = crate::Reg<rg6cr::RG6CRrs>;
///DMAMUX request generator channel 6 configuration register
pub mod rg6cr;
/**RG7CR (rw) register accessor: DMAMUX request generator channel 7 configuration register

You can [`read`](crate::Reg::read) this register and get [`rg7cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg7cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:RG7CR)

For information about available fields see [`mod@rg7cr`] module*/
pub type RG7CR = crate::Reg<rg7cr::RG7CRrs>;
///DMAMUX request generator channel 7 configuration register
pub mod rg7cr;
/**RGSR (r) register accessor: DMAMUX request generator interrupt status register

You can [`read`](crate::Reg::read) this register and get [`rgsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:RGSR)

For information about available fields see [`mod@rgsr`] module*/
pub type RGSR = crate::Reg<rgsr::RGSRrs>;
///DMAMUX request generator interrupt status register
pub mod rgsr;
/**RGCFR (w) register accessor: DMAMUX request generator interrupt clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgcfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:RGCFR)

For information about available fields see [`mod@rgcfr`] module*/
pub type RGCFR = crate::Reg<rgcfr::RGCFRrs>;
///DMAMUX request generator interrupt clear flag register
pub mod rgcfr;
/**HWCFGR2 (r) register accessor: DMAMUX hardware configuration 2 register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:HWCFGR2)

For information about available fields see [`mod@hwcfgr2`] module*/
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2rs>;
///DMAMUX hardware configuration 2 register
pub mod hwcfgr2;
/**HWCFGR1 (r) register accessor: DMAMUX hardware configuration 1 register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:HWCFGR1)

For information about available fields see [`mod@hwcfgr1`] module*/
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1rs>;
///DMAMUX hardware configuration 1 register
pub mod hwcfgr1;
/**VERR (r) register accessor: This register identifies the IP version.

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///This register identifies the IP version.
pub mod verr;
/**IPIDR (r) register accessor: This register identifies the IP.

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///This register identifies the IP.
pub mod ipidr;
/**SIDR (r) register accessor: DMAMUX size identification register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///DMAMUX size identification register
pub mod sidr;
