#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    imr: IMR,
    sr: SR,
    ifcr: IFCR,
    _reserved_4_fmt0_dr: [u8; 0x04],
    csr: CSR,
    dir: DIR,
}
impl RegisterBlock {
    ///0x00 - SPDIFRX control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - SPDIFRX interrupt mask register
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
    }
    ///0x08 - SPDIFRX status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x0c - SPDIFRX interrupt flag clear register
    #[inline(always)]
    pub const fn ifcr(&self) -> &IFCR {
        &self.ifcr
    }
    ///0x10 - SPDIFRX data input register
    #[inline(always)]
    pub const fn fmt2_dr(&self) -> &FMT2_DR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    ///0x10 - SPDIFRX data input register
    #[inline(always)]
    pub const fn fmt1_dr(&self) -> &FMT1_DR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    ///0x10 - SPDIFRX data input register
    #[inline(always)]
    pub const fn fmt0_dr(&self) -> &FMT0_DR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    ///0x14 - SPDIFRX channel status register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    ///0x18 - SPDIFRX debug information register
    #[inline(always)]
    pub const fn dir(&self) -> &DIR {
        &self.dir
    }
}
/**CR (rw) register accessor: SPDIFRX control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SPDIFRX:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///SPDIFRX control register
pub mod cr;
/**IMR (rw) register accessor: SPDIFRX interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SPDIFRX:IMR)

For information about available fields see [`mod@imr`] module*/
pub type IMR = crate::Reg<imr::IMRrs>;
///SPDIFRX interrupt mask register
pub mod imr;
/**SR (r) register accessor: SPDIFRX status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SPDIFRX:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///SPDIFRX status register
pub mod sr;
/**IFCR (w) register accessor: SPDIFRX interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SPDIFRX:IFCR)

For information about available fields see [`mod@ifcr`] module*/
pub type IFCR = crate::Reg<ifcr::IFCRrs>;
///SPDIFRX interrupt flag clear register
pub mod ifcr;
/**FMT0_DR (r) register accessor: SPDIFRX data input register

You can [`read`](crate::Reg::read) this register and get [`fmt0_dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SPDIFRX:FMT0_DR)

For information about available fields see [`mod@fmt0_dr`] module*/
pub type FMT0_DR = crate::Reg<fmt0_dr::FMT0_DRrs>;
///SPDIFRX data input register
pub mod fmt0_dr;
/**FMT1_DR (r) register accessor: SPDIFRX data input register

You can [`read`](crate::Reg::read) this register and get [`fmt1_dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SPDIFRX:FMT1_DR)

For information about available fields see [`mod@fmt1_dr`] module*/
pub type FMT1_DR = crate::Reg<fmt1_dr::FMT1_DRrs>;
///SPDIFRX data input register
pub mod fmt1_dr;
/**FMT2_DR (r) register accessor: SPDIFRX data input register

You can [`read`](crate::Reg::read) this register and get [`fmt2_dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SPDIFRX:FMT2_DR)

For information about available fields see [`mod@fmt2_dr`] module*/
pub type FMT2_DR = crate::Reg<fmt2_dr::FMT2_DRrs>;
///SPDIFRX data input register
pub mod fmt2_dr;
/**CSR (r) register accessor: SPDIFRX channel status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SPDIFRX:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///SPDIFRX channel status register
pub mod csr;
/**DIR (r) register accessor: SPDIFRX debug information register

You can [`read`](crate::Reg::read) this register and get [`dir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SPDIFRX:DIR)

For information about available fields see [`mod@dir`] module*/
pub type DIR = crate::Reg<dir::DIRrs>;
///SPDIFRX debug information register
pub mod dir;
