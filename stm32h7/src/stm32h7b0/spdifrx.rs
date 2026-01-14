#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    imr: IMR,
    sr: SR,
    ifcr: IFCR,
    _reserved_4_dr_: [u8; 0x04],
    csr: CSR,
    dir: DIR,
    _reserved7: [u8; 0x03d8],
    verr: VERR,
    idr: IDR,
    sidr: SIDR,
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
    pub const fn dr_10(&self) -> &DR_10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    ///0x10 - Data input register
    #[inline(always)]
    pub const fn dr_01(&self) -> &DR_01 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    ///0x10 - Data input register
    #[inline(always)]
    pub const fn dr_00(&self) -> &DR_00 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
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
    ///0x3f4 - SPDIFRX version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - SPDIFRX identification register
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    ///0x3fc - SPDIFRX size identification register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**CR (rw) register accessor: Control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#SPDIFRX:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Control register
pub mod cr;
/**IMR (rw) register accessor: Interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#SPDIFRX:IMR)

For information about available fields see [`mod@imr`] module*/
pub type IMR = crate::Reg<imr::IMRrs>;
///Interrupt mask register
pub mod imr;
/**SR (r) register accessor: Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#SPDIFRX:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///Status register
pub mod sr;
/**IFCR (w) register accessor: Interrupt Flag Clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#SPDIFRX:IFCR)

For information about available fields see [`mod@ifcr`] module*/
pub type IFCR = crate::Reg<ifcr::IFCRrs>;
///Interrupt Flag Clear register
pub mod ifcr;
/**DR_00 (r) register accessor: Data input register

You can [`read`](crate::Reg::read) this register and get [`dr_00::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#SPDIFRX:DR_00)

For information about available fields see [`mod@dr_00`] module*/
pub type DR_00 = crate::Reg<dr_00::DR_00rs>;
///Data input register
pub mod dr_00;
/**CSR (r) register accessor: Channel Status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#SPDIFRX:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///Channel Status register
pub mod csr;
/**DIR (r) register accessor: Debug Information register

You can [`read`](crate::Reg::read) this register and get [`dir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#SPDIFRX:DIR)

For information about available fields see [`mod@dir`] module*/
pub type DIR = crate::Reg<dir::DIRrs>;
///Debug Information register
pub mod dir;
/**VERR (r) register accessor: SPDIFRX version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#SPDIFRX:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///SPDIFRX version register
pub mod verr;
/**IDR (r) register accessor: SPDIFRX identification register

You can [`read`](crate::Reg::read) this register and get [`idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#SPDIFRX:IDR)

For information about available fields see [`mod@idr`] module*/
pub type IDR = crate::Reg<idr::IDRrs>;
///SPDIFRX identification register
pub mod idr;
/**SIDR (r) register accessor: SPDIFRX size identification register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#SPDIFRX:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///SPDIFRX size identification register
pub mod sidr;
/**DR_01 (r) register accessor: Data input register

You can [`read`](crate::Reg::read) this register and get [`dr_01::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#SPDIFRX:DR_01)

For information about available fields see [`mod@dr_01`] module*/
pub type DR_01 = crate::Reg<dr_01::DR_01rs>;
///Data input register
pub mod dr_01;
/**DR_10 (r) register accessor: Data input register

You can [`read`](crate::Reg::read) this register and get [`dr_10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#SPDIFRX:DR_10)

For information about available fields see [`mod@dr_10`] module*/
pub type DR_10 = crate::Reg<dr_10::DR_10rs>;
///Data input register
pub mod dr_10;
