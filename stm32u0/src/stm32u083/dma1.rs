#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dma_isr: DMA_ISR,
    dma_ifcr: DMA_IFCR,
    dma_ccr1: DMA_CCR1,
    dma_cndtr1: DMA_CNDTR1,
    dma_cpar1: DMA_CPAR1,
    dma_cmar1: DMA_CMAR1,
    _reserved6: [u8; 0x04],
    dma_ccr2: DMA_CCR2,
    dma_cndtr2: DMA_CNDTR2,
    dma_cpar2: DMA_CPAR2,
    dma_cmar2: DMA_CMAR2,
    _reserved10: [u8; 0x04],
    dma_ccr3: DMA_CCR3,
    dma_cndtr3: DMA_CNDTR3,
    dma_cpar3: DMA_CPAR3,
    dma_cmar3: DMA_CMAR3,
    _reserved14: [u8; 0x04],
    dma_ccr4: DMA_CCR4,
    dma_cndtr4: DMA_CNDTR4,
    dma_cpar4: DMA_CPAR4,
    dma_cmar4: DMA_CMAR4,
    _reserved18: [u8; 0x04],
    dma_ccr5: DMA_CCR5,
    dma_cndtr5: DMA_CNDTR5,
    dma_cpar5: DMA_CPAR5,
    dma_cmar5: DMA_CMAR5,
    _reserved22: [u8; 0x04],
    dma_ccr6: DMA_CCR6,
    dma_cndtr6: DMA_CNDTR6,
    dma_cpar6: DMA_CPAR6,
    dma_cmar6: DMA_CMAR6,
    _reserved26: [u8; 0x04],
    dma_ccr7: DMA_CCR7,
    dma_cndtr7: DMA_CNDTR7,
    dma_cpar7: DMA_CPAR7,
    dma_cmar7: DMA_CMAR7,
}
impl RegisterBlock {
    ///0x00 - DMA interrupt status register
    #[inline(always)]
    pub const fn dma_isr(&self) -> &DMA_ISR {
        &self.dma_isr
    }
    ///0x04 - DMA interrupt flag clear register
    #[inline(always)]
    pub const fn dma_ifcr(&self) -> &DMA_IFCR {
        &self.dma_ifcr
    }
    ///0x08 - DMA channel 1 configuration register
    #[inline(always)]
    pub const fn dma_ccr1(&self) -> &DMA_CCR1 {
        &self.dma_ccr1
    }
    ///0x0c - DMA channel 1 number of data to transfer register
    #[inline(always)]
    pub const fn dma_cndtr1(&self) -> &DMA_CNDTR1 {
        &self.dma_cndtr1
    }
    ///0x10 - DMA channel 1 peripheral address register
    #[inline(always)]
    pub const fn dma_cpar1(&self) -> &DMA_CPAR1 {
        &self.dma_cpar1
    }
    ///0x14 - DMA channel 1 memory address register
    #[inline(always)]
    pub const fn dma_cmar1(&self) -> &DMA_CMAR1 {
        &self.dma_cmar1
    }
    ///0x1c - DMA channel 2 configuration register
    #[inline(always)]
    pub const fn dma_ccr2(&self) -> &DMA_CCR2 {
        &self.dma_ccr2
    }
    ///0x20 - DMA channel 2 number of data to transfer register
    #[inline(always)]
    pub const fn dma_cndtr2(&self) -> &DMA_CNDTR2 {
        &self.dma_cndtr2
    }
    ///0x24 - DMA channel 2 peripheral address register
    #[inline(always)]
    pub const fn dma_cpar2(&self) -> &DMA_CPAR2 {
        &self.dma_cpar2
    }
    ///0x28 - DMA channel 2 memory address register
    #[inline(always)]
    pub const fn dma_cmar2(&self) -> &DMA_CMAR2 {
        &self.dma_cmar2
    }
    ///0x30 - DMA channel 3 configuration register
    #[inline(always)]
    pub const fn dma_ccr3(&self) -> &DMA_CCR3 {
        &self.dma_ccr3
    }
    ///0x34 - DMA channel 3 number of data to transfer register
    #[inline(always)]
    pub const fn dma_cndtr3(&self) -> &DMA_CNDTR3 {
        &self.dma_cndtr3
    }
    ///0x38 - DMA channel 3 peripheral address register
    #[inline(always)]
    pub const fn dma_cpar3(&self) -> &DMA_CPAR3 {
        &self.dma_cpar3
    }
    ///0x3c - DMA channel 3 memory address register
    #[inline(always)]
    pub const fn dma_cmar3(&self) -> &DMA_CMAR3 {
        &self.dma_cmar3
    }
    ///0x44 - DMA channel 4 configuration register
    #[inline(always)]
    pub const fn dma_ccr4(&self) -> &DMA_CCR4 {
        &self.dma_ccr4
    }
    ///0x48 - DMA channel 4 number of data to transfer register
    #[inline(always)]
    pub const fn dma_cndtr4(&self) -> &DMA_CNDTR4 {
        &self.dma_cndtr4
    }
    ///0x4c - DMA channel 4 peripheral address register
    #[inline(always)]
    pub const fn dma_cpar4(&self) -> &DMA_CPAR4 {
        &self.dma_cpar4
    }
    ///0x50 - DMA channel 4 memory address register
    #[inline(always)]
    pub const fn dma_cmar4(&self) -> &DMA_CMAR4 {
        &self.dma_cmar4
    }
    ///0x58 - DMA channel 5 configuration register
    #[inline(always)]
    pub const fn dma_ccr5(&self) -> &DMA_CCR5 {
        &self.dma_ccr5
    }
    ///0x5c - DMA channel 5 number of data to transfer register
    #[inline(always)]
    pub const fn dma_cndtr5(&self) -> &DMA_CNDTR5 {
        &self.dma_cndtr5
    }
    ///0x60 - DMA channel 5 peripheral address register
    #[inline(always)]
    pub const fn dma_cpar5(&self) -> &DMA_CPAR5 {
        &self.dma_cpar5
    }
    ///0x64 - DMA channel 5 memory address register
    #[inline(always)]
    pub const fn dma_cmar5(&self) -> &DMA_CMAR5 {
        &self.dma_cmar5
    }
    ///0x6c - DMA channel 6 configuration register
    #[inline(always)]
    pub const fn dma_ccr6(&self) -> &DMA_CCR6 {
        &self.dma_ccr6
    }
    ///0x70 - DMA channel 6 number of data to transfer register
    #[inline(always)]
    pub const fn dma_cndtr6(&self) -> &DMA_CNDTR6 {
        &self.dma_cndtr6
    }
    ///0x74 - DMA channel 6 peripheral address register
    #[inline(always)]
    pub const fn dma_cpar6(&self) -> &DMA_CPAR6 {
        &self.dma_cpar6
    }
    ///0x78 - DMA channel 6 memory address register
    #[inline(always)]
    pub const fn dma_cmar6(&self) -> &DMA_CMAR6 {
        &self.dma_cmar6
    }
    ///0x80 - DMA channel 7 configuration register
    #[inline(always)]
    pub const fn dma_ccr7(&self) -> &DMA_CCR7 {
        &self.dma_ccr7
    }
    ///0x84 - DMA channel 7 number of data to transfer register
    #[inline(always)]
    pub const fn dma_cndtr7(&self) -> &DMA_CNDTR7 {
        &self.dma_cndtr7
    }
    ///0x88 - DMA channel 7 peripheral address register
    #[inline(always)]
    pub const fn dma_cpar7(&self) -> &DMA_CPAR7 {
        &self.dma_cpar7
    }
    ///0x8c - DMA channel 7 memory address register
    #[inline(always)]
    pub const fn dma_cmar7(&self) -> &DMA_CMAR7 {
        &self.dma_cmar7
    }
}
/**DMA_ISR (r) register accessor: DMA interrupt status register

You can [`read`](crate::Reg::read) this register and get [`dma_isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_ISR)

For information about available fields see [`mod@dma_isr`]
module*/
pub type DMA_ISR = crate::Reg<dma_isr::DMA_ISRrs>;
///DMA interrupt status register
pub mod dma_isr;
/**DMA_IFCR (w) register accessor: DMA interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_IFCR)

For information about available fields see [`mod@dma_ifcr`]
module*/
pub type DMA_IFCR = crate::Reg<dma_ifcr::DMA_IFCRrs>;
///DMA interrupt flag clear register
pub mod dma_ifcr;
/**DMA_CCR1 (rw) register accessor: DMA channel 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`dma_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CCR1)

For information about available fields see [`mod@dma_ccr1`]
module*/
pub type DMA_CCR1 = crate::Reg<dma_ccr1::DMA_CCR1rs>;
///DMA channel 1 configuration register
pub mod dma_ccr1;
/**DMA_CNDTR1 (rw) register accessor: DMA channel 1 number of data to transfer register

You can [`read`](crate::Reg::read) this register and get [`dma_cndtr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cndtr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CNDTR1)

For information about available fields see [`mod@dma_cndtr1`]
module*/
pub type DMA_CNDTR1 = crate::Reg<dma_cndtr1::DMA_CNDTR1rs>;
///DMA channel 1 number of data to transfer register
pub mod dma_cndtr1;
/**DMA_CPAR1 (rw) register accessor: DMA channel 1 peripheral address register

You can [`read`](crate::Reg::read) this register and get [`dma_cpar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cpar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CPAR1)

For information about available fields see [`mod@dma_cpar1`]
module*/
pub type DMA_CPAR1 = crate::Reg<dma_cpar1::DMA_CPAR1rs>;
///DMA channel 1 peripheral address register
pub mod dma_cpar1;
/**DMA_CMAR1 (rw) register accessor: DMA channel 1 memory address register

You can [`read`](crate::Reg::read) this register and get [`dma_cmar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cmar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CMAR1)

For information about available fields see [`mod@dma_cmar1`]
module*/
pub type DMA_CMAR1 = crate::Reg<dma_cmar1::DMA_CMAR1rs>;
///DMA channel 1 memory address register
pub mod dma_cmar1;
/**DMA_CCR2 (rw) register accessor: DMA channel 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`dma_ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CCR2)

For information about available fields see [`mod@dma_ccr2`]
module*/
pub type DMA_CCR2 = crate::Reg<dma_ccr2::DMA_CCR2rs>;
///DMA channel 2 configuration register
pub mod dma_ccr2;
/**DMA_CNDTR2 (rw) register accessor: DMA channel 2 number of data to transfer register

You can [`read`](crate::Reg::read) this register and get [`dma_cndtr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cndtr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CNDTR2)

For information about available fields see [`mod@dma_cndtr2`]
module*/
pub type DMA_CNDTR2 = crate::Reg<dma_cndtr2::DMA_CNDTR2rs>;
///DMA channel 2 number of data to transfer register
pub mod dma_cndtr2;
/**DMA_CPAR2 (rw) register accessor: DMA channel 2 peripheral address register

You can [`read`](crate::Reg::read) this register and get [`dma_cpar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cpar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CPAR2)

For information about available fields see [`mod@dma_cpar2`]
module*/
pub type DMA_CPAR2 = crate::Reg<dma_cpar2::DMA_CPAR2rs>;
///DMA channel 2 peripheral address register
pub mod dma_cpar2;
/**DMA_CMAR2 (rw) register accessor: DMA channel 2 memory address register

You can [`read`](crate::Reg::read) this register and get [`dma_cmar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cmar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CMAR2)

For information about available fields see [`mod@dma_cmar2`]
module*/
pub type DMA_CMAR2 = crate::Reg<dma_cmar2::DMA_CMAR2rs>;
///DMA channel 2 memory address register
pub mod dma_cmar2;
/**DMA_CCR3 (rw) register accessor: DMA channel 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`dma_ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CCR3)

For information about available fields see [`mod@dma_ccr3`]
module*/
pub type DMA_CCR3 = crate::Reg<dma_ccr3::DMA_CCR3rs>;
///DMA channel 3 configuration register
pub mod dma_ccr3;
/**DMA_CNDTR3 (rw) register accessor: DMA channel 3 number of data to transfer register

You can [`read`](crate::Reg::read) this register and get [`dma_cndtr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cndtr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CNDTR3)

For information about available fields see [`mod@dma_cndtr3`]
module*/
pub type DMA_CNDTR3 = crate::Reg<dma_cndtr3::DMA_CNDTR3rs>;
///DMA channel 3 number of data to transfer register
pub mod dma_cndtr3;
/**DMA_CPAR3 (rw) register accessor: DMA channel 3 peripheral address register

You can [`read`](crate::Reg::read) this register and get [`dma_cpar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cpar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CPAR3)

For information about available fields see [`mod@dma_cpar3`]
module*/
pub type DMA_CPAR3 = crate::Reg<dma_cpar3::DMA_CPAR3rs>;
///DMA channel 3 peripheral address register
pub mod dma_cpar3;
/**DMA_CMAR3 (rw) register accessor: DMA channel 3 memory address register

You can [`read`](crate::Reg::read) this register and get [`dma_cmar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cmar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CMAR3)

For information about available fields see [`mod@dma_cmar3`]
module*/
pub type DMA_CMAR3 = crate::Reg<dma_cmar3::DMA_CMAR3rs>;
///DMA channel 3 memory address register
pub mod dma_cmar3;
/**DMA_CCR4 (rw) register accessor: DMA channel 4 configuration register

You can [`read`](crate::Reg::read) this register and get [`dma_ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CCR4)

For information about available fields see [`mod@dma_ccr4`]
module*/
pub type DMA_CCR4 = crate::Reg<dma_ccr4::DMA_CCR4rs>;
///DMA channel 4 configuration register
pub mod dma_ccr4;
/**DMA_CNDTR4 (rw) register accessor: DMA channel 4 number of data to transfer register

You can [`read`](crate::Reg::read) this register and get [`dma_cndtr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cndtr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CNDTR4)

For information about available fields see [`mod@dma_cndtr4`]
module*/
pub type DMA_CNDTR4 = crate::Reg<dma_cndtr4::DMA_CNDTR4rs>;
///DMA channel 4 number of data to transfer register
pub mod dma_cndtr4;
/**DMA_CPAR4 (rw) register accessor: DMA channel 4 peripheral address register

You can [`read`](crate::Reg::read) this register and get [`dma_cpar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cpar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CPAR4)

For information about available fields see [`mod@dma_cpar4`]
module*/
pub type DMA_CPAR4 = crate::Reg<dma_cpar4::DMA_CPAR4rs>;
///DMA channel 4 peripheral address register
pub mod dma_cpar4;
/**DMA_CMAR4 (rw) register accessor: DMA channel 4 memory address register

You can [`read`](crate::Reg::read) this register and get [`dma_cmar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cmar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CMAR4)

For information about available fields see [`mod@dma_cmar4`]
module*/
pub type DMA_CMAR4 = crate::Reg<dma_cmar4::DMA_CMAR4rs>;
///DMA channel 4 memory address register
pub mod dma_cmar4;
/**DMA_CCR5 (rw) register accessor: DMA channel 5 configuration register

You can [`read`](crate::Reg::read) this register and get [`dma_ccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CCR5)

For information about available fields see [`mod@dma_ccr5`]
module*/
pub type DMA_CCR5 = crate::Reg<dma_ccr5::DMA_CCR5rs>;
///DMA channel 5 configuration register
pub mod dma_ccr5;
/**DMA_CNDTR5 (rw) register accessor: DMA channel 5 number of data to transfer register

You can [`read`](crate::Reg::read) this register and get [`dma_cndtr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cndtr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CNDTR5)

For information about available fields see [`mod@dma_cndtr5`]
module*/
pub type DMA_CNDTR5 = crate::Reg<dma_cndtr5::DMA_CNDTR5rs>;
///DMA channel 5 number of data to transfer register
pub mod dma_cndtr5;
/**DMA_CPAR5 (rw) register accessor: DMA channel 5 peripheral address register

You can [`read`](crate::Reg::read) this register and get [`dma_cpar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cpar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CPAR5)

For information about available fields see [`mod@dma_cpar5`]
module*/
pub type DMA_CPAR5 = crate::Reg<dma_cpar5::DMA_CPAR5rs>;
///DMA channel 5 peripheral address register
pub mod dma_cpar5;
/**DMA_CMAR5 (rw) register accessor: DMA channel 5 memory address register

You can [`read`](crate::Reg::read) this register and get [`dma_cmar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cmar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CMAR5)

For information about available fields see [`mod@dma_cmar5`]
module*/
pub type DMA_CMAR5 = crate::Reg<dma_cmar5::DMA_CMAR5rs>;
///DMA channel 5 memory address register
pub mod dma_cmar5;
/**DMA_CCR6 (rw) register accessor: DMA channel 6 configuration register

You can [`read`](crate::Reg::read) this register and get [`dma_ccr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ccr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CCR6)

For information about available fields see [`mod@dma_ccr6`]
module*/
pub type DMA_CCR6 = crate::Reg<dma_ccr6::DMA_CCR6rs>;
///DMA channel 6 configuration register
pub mod dma_ccr6;
/**DMA_CNDTR6 (rw) register accessor: DMA channel 6 number of data to transfer register

You can [`read`](crate::Reg::read) this register and get [`dma_cndtr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cndtr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CNDTR6)

For information about available fields see [`mod@dma_cndtr6`]
module*/
pub type DMA_CNDTR6 = crate::Reg<dma_cndtr6::DMA_CNDTR6rs>;
///DMA channel 6 number of data to transfer register
pub mod dma_cndtr6;
/**DMA_CPAR6 (rw) register accessor: DMA channel 6 peripheral address register

You can [`read`](crate::Reg::read) this register and get [`dma_cpar6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cpar6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CPAR6)

For information about available fields see [`mod@dma_cpar6`]
module*/
pub type DMA_CPAR6 = crate::Reg<dma_cpar6::DMA_CPAR6rs>;
///DMA channel 6 peripheral address register
pub mod dma_cpar6;
/**DMA_CMAR6 (rw) register accessor: DMA channel 6 memory address register

You can [`read`](crate::Reg::read) this register and get [`dma_cmar6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cmar6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CMAR6)

For information about available fields see [`mod@dma_cmar6`]
module*/
pub type DMA_CMAR6 = crate::Reg<dma_cmar6::DMA_CMAR6rs>;
///DMA channel 6 memory address register
pub mod dma_cmar6;
/**DMA_CCR7 (rw) register accessor: DMA channel 7 configuration register

You can [`read`](crate::Reg::read) this register and get [`dma_ccr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ccr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CCR7)

For information about available fields see [`mod@dma_ccr7`]
module*/
pub type DMA_CCR7 = crate::Reg<dma_ccr7::DMA_CCR7rs>;
///DMA channel 7 configuration register
pub mod dma_ccr7;
/**DMA_CNDTR7 (rw) register accessor: DMA channel 7 number of data to transfer register

You can [`read`](crate::Reg::read) this register and get [`dma_cndtr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cndtr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CNDTR7)

For information about available fields see [`mod@dma_cndtr7`]
module*/
pub type DMA_CNDTR7 = crate::Reg<dma_cndtr7::DMA_CNDTR7rs>;
///DMA channel 7 number of data to transfer register
pub mod dma_cndtr7;
/**DMA_CPAR7 (rw) register accessor: DMA channel 7 peripheral address register

You can [`read`](crate::Reg::read) this register and get [`dma_cpar7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cpar7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CPAR7)

For information about available fields see [`mod@dma_cpar7`]
module*/
pub type DMA_CPAR7 = crate::Reg<dma_cpar7::DMA_CPAR7rs>;
///DMA channel 7 peripheral address register
pub mod dma_cpar7;
/**DMA_CMAR7 (rw) register accessor: DMA channel 7 memory address register

You can [`read`](crate::Reg::read) this register and get [`dma_cmar7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cmar7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DMA1:DMA_CMAR7)

For information about available fields see [`mod@dma_cmar7`]
module*/
pub type DMA_CMAR7 = crate::Reg<dma_cmar7::DMA_CMAR7rs>;
///DMA channel 7 memory address register
pub mod dma_cmar7;
