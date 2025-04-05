#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    lisr: LISR,
    hisr: HISR,
    lifcr: LIFCR,
    hifcr: HIFCR,
    st: [ST; 8],
}
impl RegisterBlock {
    ///0x00 - low interrupt status register
    #[inline(always)]
    pub const fn lisr(&self) -> &LISR {
        &self.lisr
    }
    ///0x04 - high interrupt status register
    #[inline(always)]
    pub const fn hisr(&self) -> &HISR {
        &self.hisr
    }
    ///0x08 - low interrupt flag clear register
    #[inline(always)]
    pub const fn lifcr(&self) -> &LIFCR {
        &self.lifcr
    }
    ///0x0c - high interrupt flag clear register
    #[inline(always)]
    pub const fn hifcr(&self) -> &HIFCR {
        &self.hifcr
    }
    ///0x10..0xd0 - Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers
    #[inline(always)]
    pub const fn st(&self, n: usize) -> &ST {
        &self.st[n]
    }
    ///Iterator for array of:
    ///0x10..0xd0 - Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers
    #[inline(always)]
    pub fn st_iter(&self) -> impl Iterator<Item = &ST> {
        self.st.iter()
    }
}
/**LISR (r) register accessor: low interrupt status register

You can [`read`](crate::Reg::read) this register and get [`lisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMA1:LISR)

For information about available fields see [`mod@lisr`] module*/
pub type LISR = crate::Reg<lisr::LISRrs>;
///low interrupt status register
pub mod lisr;
/**HISR (r) register accessor: high interrupt status register

You can [`read`](crate::Reg::read) this register and get [`hisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMA1:HISR)

For information about available fields see [`mod@hisr`] module*/
pub type HISR = crate::Reg<hisr::HISRrs>;
///high interrupt status register
pub mod hisr;
/**LIFCR (w) register accessor: low interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMA1:LIFCR)

For information about available fields see [`mod@lifcr`] module*/
pub type LIFCR = crate::Reg<lifcr::LIFCRrs>;
///low interrupt flag clear register
pub mod lifcr;
/**HIFCR (w) register accessor: high interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMA1:HIFCR)

For information about available fields see [`mod@hifcr`] module*/
pub type HIFCR = crate::Reg<hifcr::HIFCRrs>;
///high interrupt flag clear register
pub mod hifcr;
///Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers
pub use self::st::ST;
///Cluster
///Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers
pub mod st;
