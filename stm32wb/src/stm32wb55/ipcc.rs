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
    ///0x00 - Control register CPU1
    #[inline(always)]
    pub const fn c1cr(&self) -> &C1CR {
        &self.c1cr
    }
    ///0x04 - Mask register CPU1
    #[inline(always)]
    pub const fn c1mr(&self) -> &C1MR {
        &self.c1mr
    }
    ///0x08 - Status Set or Clear register CPU1
    #[inline(always)]
    pub const fn c1scr(&self) -> &C1SCR {
        &self.c1scr
    }
    ///0x0c - CPU1 to CPU2 status register
    #[inline(always)]
    pub const fn c1toc2sr(&self) -> &C1TOC2SR {
        &self.c1toc2sr
    }
    ///0x10 - Control register CPU2
    #[inline(always)]
    pub const fn c2cr(&self) -> &C2CR {
        &self.c2cr
    }
    ///0x14 - Mask register CPU2
    #[inline(always)]
    pub const fn c2mr(&self) -> &C2MR {
        &self.c2mr
    }
    ///0x18 - Status Set or Clear register CPU2
    #[inline(always)]
    pub const fn c2scr(&self) -> &C2SCR {
        &self.c2scr
    }
    ///0x1c - CPU2 to CPU1 status register
    #[inline(always)]
    pub const fn c2toc1sr(&self) -> &C2TOC1SR {
        &self.c2toc1sr
    }
    ///0x3f0 - IPCC Hardware configuration register
    #[inline(always)]
    pub const fn hwcfgr(&self) -> &HWCFGR {
        &self.hwcfgr
    }
    ///0x3f4 - IPCC version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - IPCC indentification register
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0x3fc - IPCC size indentification register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**C1CR (rw) register accessor: Control register CPU1

You can [`read`](crate::Reg::read) this register and get [`c1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#IPCC:C1CR)

For information about available fields see [`mod@c1cr`] module*/
pub type C1CR = crate::Reg<c1cr::C1CRrs>;
///Control register CPU1
pub mod c1cr;
/**C1MR (rw) register accessor: Mask register CPU1

You can [`read`](crate::Reg::read) this register and get [`c1mr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1mr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#IPCC:C1MR)

For information about available fields see [`mod@c1mr`] module*/
pub type C1MR = crate::Reg<c1mr::C1MRrs>;
///Mask register CPU1
pub mod c1mr;
/**C1SCR (w) register accessor: Status Set or Clear register CPU1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#IPCC:C1SCR)

For information about available fields see [`mod@c1scr`] module*/
pub type C1SCR = crate::Reg<c1scr::C1SCRrs>;
///Status Set or Clear register CPU1
pub mod c1scr;
/**C1TOC2SR (r) register accessor: CPU1 to CPU2 status register

You can [`read`](crate::Reg::read) this register and get [`c1toc2sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#IPCC:C1TOC2SR)

For information about available fields see [`mod@c1toc2sr`] module*/
pub type C1TOC2SR = crate::Reg<c1toc2sr::C1TOC2SRrs>;
///CPU1 to CPU2 status register
pub mod c1toc2sr;
/**C2CR (rw) register accessor: Control register CPU2

You can [`read`](crate::Reg::read) this register and get [`c2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#IPCC:C2CR)

For information about available fields see [`mod@c2cr`] module*/
pub type C2CR = crate::Reg<c2cr::C2CRrs>;
///Control register CPU2
pub mod c2cr;
/**C2MR (rw) register accessor: Mask register CPU2

You can [`read`](crate::Reg::read) this register and get [`c2mr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2mr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#IPCC:C2MR)

For information about available fields see [`mod@c2mr`] module*/
pub type C2MR = crate::Reg<c2mr::C2MRrs>;
///Mask register CPU2
pub mod c2mr;
/**C2SCR (w) register accessor: Status Set or Clear register CPU2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#IPCC:C2SCR)

For information about available fields see [`mod@c2scr`] module*/
pub type C2SCR = crate::Reg<c2scr::C2SCRrs>;
///Status Set or Clear register CPU2
pub mod c2scr;
/**C2TOC1SR (r) register accessor: CPU2 to CPU1 status register

You can [`read`](crate::Reg::read) this register and get [`c2toc1sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#IPCC:C2TOC1SR)

For information about available fields see [`mod@c2toc1sr`] module*/
pub type C2TOC1SR = crate::Reg<c2toc1sr::C2TOC1SRrs>;
///CPU2 to CPU1 status register
pub mod c2toc1sr;
/**HWCFGR (r) register accessor: IPCC Hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#IPCC:HWCFGR)

For information about available fields see [`mod@hwcfgr`] module*/
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGRrs>;
///IPCC Hardware configuration register
pub mod hwcfgr;
/**VERR (r) register accessor: IPCC version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#IPCC:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///IPCC version register
pub mod verr;
/**IPIDR (r) register accessor: IPCC indentification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#IPCC:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///IPCC indentification register
pub mod ipidr;
/**SIDR (r) register accessor: IPCC size indentification register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#IPCC:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///IPCC size indentification register
pub mod sidr;
