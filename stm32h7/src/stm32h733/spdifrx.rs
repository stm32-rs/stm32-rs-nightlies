#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    imr: IMR,
    sr: SR,
    ifcr: IFCR,
    fmt0_dr: FMT0_DR,
    csr: CSR,
    dir: DIR,
}
impl RegisterBlock {
    ///0x00 - Control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - Interrupt mask register
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
    }
    ///0x08 - Status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x0c - Interrupt Flag Clear register
    #[inline(always)]
    pub const fn ifcr(&self) -> &IFCR {
        &self.ifcr
    }
    ///0x10 - Data input register
    #[inline(always)]
    pub const fn fmt0_dr(&self) -> &FMT0_DR {
        &self.fmt0_dr
    }
    ///0x14 - Channel Status register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    ///0x18 - Debug Information register
    #[inline(always)]
    pub const fn dir(&self) -> &DIR {
        &self.dir
    }
}
/**CR (rw) register accessor: Control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#SPDIFRX:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Control register
pub mod cr;
/**IMR (rw) register accessor: Interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#SPDIFRX:IMR)

For information about available fields see [`mod@imr`] module*/
pub type IMR = crate::Reg<imr::IMRrs>;
///Interrupt mask register
pub mod imr;
/**SR (r) register accessor: Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#SPDIFRX:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///Status register
pub mod sr;
/**IFCR (w) register accessor: Interrupt Flag Clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#SPDIFRX:IFCR)

For information about available fields see [`mod@ifcr`] module*/
pub type IFCR = crate::Reg<ifcr::IFCRrs>;
///Interrupt Flag Clear register
pub mod ifcr;
/**FMT0_DR (r) register accessor: Data input register

You can [`read`](crate::Reg::read) this register and get [`fmt0_dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#SPDIFRX:FMT0_DR)

For information about available fields see [`mod@fmt0_dr`] module*/
pub type FMT0_DR = crate::Reg<fmt0_dr::FMT0_DRrs>;
///Data input register
pub mod fmt0_dr;
/**CSR (r) register accessor: Channel Status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#SPDIFRX:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///Channel Status register
pub mod csr;
/**DIR (r) register accessor: Debug Information register

You can [`read`](crate::Reg::read) this register and get [`dir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#SPDIFRX:DIR)

For information about available fields see [`mod@dir`] module*/
pub type DIR = crate::Reg<dir::DIRrs>;
///Debug Information register
pub mod dir;
