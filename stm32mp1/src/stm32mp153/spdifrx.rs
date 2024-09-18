#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    spdifrx_cr: SPDIFRX_CR,
    spdifrx_imr: SPDIFRX_IMR,
    spdifrx_sr: SPDIFRX_SR,
    spdifrx_ifcr: SPDIFRX_IFCR,
    spdifrx_fmt0_dr: SPDIFRX_FMT0_DR,
    spdifrx_csr: SPDIFRX_CSR,
    spdifrx_dir: SPDIFRX_DIR,
    _reserved7: [u8; 0x03d8],
    spdifrx_verr: SPDIFRX_VERR,
    spdifrx_ipidr: SPDIFRX_IPIDR,
    spdifrx_sidr: SPDIFRX_SIDR,
}
impl RegisterBlock {
    ///0x00 - Control register
    #[inline(always)]
    pub const fn spdifrx_cr(&self) -> &SPDIFRX_CR {
        &self.spdifrx_cr
    }
    ///0x04 - Interrupt mask register
    #[inline(always)]
    pub const fn spdifrx_imr(&self) -> &SPDIFRX_IMR {
        &self.spdifrx_imr
    }
    ///0x08 - Status register
    #[inline(always)]
    pub const fn spdifrx_sr(&self) -> &SPDIFRX_SR {
        &self.spdifrx_sr
    }
    ///0x0c - Interrupt flag clear register
    #[inline(always)]
    pub const fn spdifrx_ifcr(&self) -> &SPDIFRX_IFCR {
        &self.spdifrx_ifcr
    }
    ///0x10 - This register can take 3 different formats according to DRFMT. Here is the format when DRFMT = 00:
    #[inline(always)]
    pub const fn spdifrx_fmt0_dr(&self) -> &SPDIFRX_FMT0_DR {
        &self.spdifrx_fmt0_dr
    }
    ///0x14 - Channel status register
    #[inline(always)]
    pub const fn spdifrx_csr(&self) -> &SPDIFRX_CSR {
        &self.spdifrx_csr
    }
    ///0x18 - Debug information register
    #[inline(always)]
    pub const fn spdifrx_dir(&self) -> &SPDIFRX_DIR {
        &self.spdifrx_dir
    }
    ///0x3f4 - SPDIFRX version register
    #[inline(always)]
    pub const fn spdifrx_verr(&self) -> &SPDIFRX_VERR {
        &self.spdifrx_verr
    }
    ///0x3f8 - SPDIFRX identification register
    #[inline(always)]
    pub const fn spdifrx_ipidr(&self) -> &SPDIFRX_IPIDR {
        &self.spdifrx_ipidr
    }
    ///0x3fc - SPDIFRX size identification register
    #[inline(always)]
    pub const fn spdifrx_sidr(&self) -> &SPDIFRX_SIDR {
        &self.spdifrx_sidr
    }
}
/**SPDIFRX_CR (rw) register accessor: Control register

You can [`read`](crate::Reg::read) this register and get [`spdifrx_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdifrx_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPDIFRX:SPDIFRX_CR)

For information about available fields see [`mod@spdifrx_cr`]
module*/
pub type SPDIFRX_CR = crate::Reg<spdifrx_cr::SPDIFRX_CRrs>;
///Control register
pub mod spdifrx_cr;
/**SPDIFRX_IMR (rw) register accessor: Interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`spdifrx_imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdifrx_imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPDIFRX:SPDIFRX_IMR)

For information about available fields see [`mod@spdifrx_imr`]
module*/
pub type SPDIFRX_IMR = crate::Reg<spdifrx_imr::SPDIFRX_IMRrs>;
///Interrupt mask register
pub mod spdifrx_imr;
/**SPDIFRX_SR (r) register accessor: Status register

You can [`read`](crate::Reg::read) this register and get [`spdifrx_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPDIFRX:SPDIFRX_SR)

For information about available fields see [`mod@spdifrx_sr`]
module*/
pub type SPDIFRX_SR = crate::Reg<spdifrx_sr::SPDIFRX_SRrs>;
///Status register
pub mod spdifrx_sr;
/**SPDIFRX_IFCR (rw) register accessor: Interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`spdifrx_ifcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdifrx_ifcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPDIFRX:SPDIFRX_IFCR)

For information about available fields see [`mod@spdifrx_ifcr`]
module*/
pub type SPDIFRX_IFCR = crate::Reg<spdifrx_ifcr::SPDIFRX_IFCRrs>;
///Interrupt flag clear register
pub mod spdifrx_ifcr;
/**SPDIFRX_FMT0_DR (r) register accessor: This register can take 3 different formats according to DRFMT. Here is the format when DRFMT = 00:

You can [`read`](crate::Reg::read) this register and get [`spdifrx_fmt0_dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPDIFRX:SPDIFRX_FMT0_DR)

For information about available fields see [`mod@spdifrx_fmt0_dr`]
module*/
pub type SPDIFRX_FMT0_DR = crate::Reg<spdifrx_fmt0_dr::SPDIFRX_FMT0_DRrs>;
///This register can take 3 different formats according to DRFMT. Here is the format when DRFMT = 00:
pub mod spdifrx_fmt0_dr;
/**SPDIFRX_CSR (r) register accessor: Channel status register

You can [`read`](crate::Reg::read) this register and get [`spdifrx_csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPDIFRX:SPDIFRX_CSR)

For information about available fields see [`mod@spdifrx_csr`]
module*/
pub type SPDIFRX_CSR = crate::Reg<spdifrx_csr::SPDIFRX_CSRrs>;
///Channel status register
pub mod spdifrx_csr;
/**SPDIFRX_DIR (r) register accessor: Debug information register

You can [`read`](crate::Reg::read) this register and get [`spdifrx_dir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPDIFRX:SPDIFRX_DIR)

For information about available fields see [`mod@spdifrx_dir`]
module*/
pub type SPDIFRX_DIR = crate::Reg<spdifrx_dir::SPDIFRX_DIRrs>;
///Debug information register
pub mod spdifrx_dir;
/**SPDIFRX_VERR (r) register accessor: SPDIFRX version register

You can [`read`](crate::Reg::read) this register and get [`spdifrx_verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPDIFRX:SPDIFRX_VERR)

For information about available fields see [`mod@spdifrx_verr`]
module*/
pub type SPDIFRX_VERR = crate::Reg<spdifrx_verr::SPDIFRX_VERRrs>;
///SPDIFRX version register
pub mod spdifrx_verr;
/**SPDIFRX_IPIDR (r) register accessor: SPDIFRX identification register

You can [`read`](crate::Reg::read) this register and get [`spdifrx_ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPDIFRX:SPDIFRX_IPIDR)

For information about available fields see [`mod@spdifrx_ipidr`]
module*/
pub type SPDIFRX_IPIDR = crate::Reg<spdifrx_ipidr::SPDIFRX_IPIDRrs>;
///SPDIFRX identification register
pub mod spdifrx_ipidr;
/**SPDIFRX_SIDR (r) register accessor: SPDIFRX size identification register

You can [`read`](crate::Reg::read) this register and get [`spdifrx_sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPDIFRX:SPDIFRX_SIDR)

For information about available fields see [`mod@spdifrx_sidr`]
module*/
pub type SPDIFRX_SIDR = crate::Reg<spdifrx_sidr::SPDIFRX_SIDRrs>;
///SPDIFRX size identification register
pub mod spdifrx_sidr;
