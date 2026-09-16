#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    c1cr: C1CR,
    c1mr: C1MR,
    c1scr: C1SCR,
    c1toc2sr: C1TOC2SR,
    c2cr: C2CR,
    c2mr: C2MR,
    c2scr: C2SCR,
    c2toc1sr: C2TOC1SR,
    _reserved8: [u8; 0x03d0],
    hwcfgr: HWCFGR,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00 - IPCC Processor 1 control register
    #[inline(always)]
    pub const fn c1cr(&self) -> &C1CR {
        &self.c1cr
    }
    ///0x04 - IPCC Processor 1 mask register
    #[inline(always)]
    pub const fn c1mr(&self) -> &C1MR {
        &self.c1mr
    }
    ///0x08 - Reading this register will always return 0x0000 0000.
    #[inline(always)]
    pub const fn c1scr(&self) -> &C1SCR {
        &self.c1scr
    }
    ///0x0c - IPCC processor 1 to processor 2 status register
    #[inline(always)]
    pub const fn c1toc2sr(&self) -> &C1TOC2SR {
        &self.c1toc2sr
    }
    ///0x10 - IPCC Processor 2 control register
    #[inline(always)]
    pub const fn c2cr(&self) -> &C2CR {
        &self.c2cr
    }
    ///0x14 - IPCC Processor 2 mask register
    #[inline(always)]
    pub const fn c2mr(&self) -> &C2MR {
        &self.c2mr
    }
    ///0x18 - Reading this register will always return 0x0000 0000.
    #[inline(always)]
    pub const fn c2scr(&self) -> &C2SCR {
        &self.c2scr
    }
    ///0x1c - IPCC processor 2 to processor 1 status register
    #[inline(always)]
    pub const fn c2toc1sr(&self) -> &C2TOC1SR {
        &self.c2toc1sr
    }
    ///0x3f0 - IPCC Hardware configuration register
    #[inline(always)]
    pub const fn hwcfgr(&self) -> &HWCFGR {
        &self.hwcfgr
    }
    ///0x3f4 - IPCC IP Version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - IPCC IP Identification register
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0x3fc - IPCC Size ID register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**C1CR (rw) register accessor: IPCC Processor 1 control register

You can [`read`](crate::Reg::read) this register and get [`c1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#IPCC:C1CR)

For information about available fields see [`mod@c1cr`] module*/
pub type C1CR = crate::Reg<c1cr::C1CRrs>;
///IPCC Processor 1 control register
pub mod c1cr;
/**C1MR (rw) register accessor: IPCC Processor 1 mask register

You can [`read`](crate::Reg::read) this register and get [`c1mr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1mr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#IPCC:C1MR)

For information about available fields see [`mod@c1mr`] module*/
pub type C1MR = crate::Reg<c1mr::C1MRrs>;
///IPCC Processor 1 mask register
pub mod c1mr;
/**C1SCR (rw) register accessor: Reading this register will always return 0x0000 0000.

You can [`read`](crate::Reg::read) this register and get [`c1scr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1scr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#IPCC:C1SCR)

For information about available fields see [`mod@c1scr`] module*/
pub type C1SCR = crate::Reg<c1scr::C1SCRrs>;
///Reading this register will always return 0x0000 0000.
pub mod c1scr;
/**C1TOC2SR (r) register accessor: IPCC processor 1 to processor 2 status register

You can [`read`](crate::Reg::read) this register and get [`c1toc2sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#IPCC:C1TOC2SR)

For information about available fields see [`mod@c1toc2sr`] module*/
pub type C1TOC2SR = crate::Reg<c1toc2sr::C1TOC2SRrs>;
///IPCC processor 1 to processor 2 status register
pub mod c1toc2sr;
/**C2CR (rw) register accessor: IPCC Processor 2 control register

You can [`read`](crate::Reg::read) this register and get [`c2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#IPCC:C2CR)

For information about available fields see [`mod@c2cr`] module*/
pub type C2CR = crate::Reg<c2cr::C2CRrs>;
///IPCC Processor 2 control register
pub mod c2cr;
/**C2MR (rw) register accessor: IPCC Processor 2 mask register

You can [`read`](crate::Reg::read) this register and get [`c2mr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2mr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#IPCC:C2MR)

For information about available fields see [`mod@c2mr`] module*/
pub type C2MR = crate::Reg<c2mr::C2MRrs>;
///IPCC Processor 2 mask register
pub mod c2mr;
/**C2SCR (rw) register accessor: Reading this register will always return 0x0000 0000.

You can [`read`](crate::Reg::read) this register and get [`c2scr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2scr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#IPCC:C2SCR)

For information about available fields see [`mod@c2scr`] module*/
pub type C2SCR = crate::Reg<c2scr::C2SCRrs>;
///Reading this register will always return 0x0000 0000.
pub mod c2scr;
/**C2TOC1SR (r) register accessor: IPCC processor 2 to processor 1 status register

You can [`read`](crate::Reg::read) this register and get [`c2toc1sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#IPCC:C2TOC1SR)

For information about available fields see [`mod@c2toc1sr`] module*/
pub type C2TOC1SR = crate::Reg<c2toc1sr::C2TOC1SRrs>;
///IPCC processor 2 to processor 1 status register
pub mod c2toc1sr;
/**HWCFGR (r) register accessor: IPCC Hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#IPCC:HWCFGR)

For information about available fields see [`mod@hwcfgr`] module*/
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGRrs>;
///IPCC Hardware configuration register
pub mod hwcfgr;
/**VERR (r) register accessor: IPCC IP Version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#IPCC:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///IPCC IP Version register
pub mod verr;
/**IPIDR (r) register accessor: IPCC IP Identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#IPCC:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///IPCC IP Identification register
pub mod ipidr;
/**SIDR (r) register accessor: IPCC Size ID register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#IPCC:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///IPCC Size ID register
pub mod sidr;
