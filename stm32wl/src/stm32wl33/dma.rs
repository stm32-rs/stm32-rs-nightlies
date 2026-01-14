#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    isr: ISR,
    ifcr: IFCR,
    ccr1: CCR1,
    cndtr1: CNDTR1,
    cpar1: CPAR1,
    cmar1: CMAR1,
    _reserved6: [u8; 0x04],
    ccr2: CCR2,
    cndtr2: CNDTR2,
    cpar2: CPAR2,
    cmar2: CMAR2,
    _reserved10: [u8; 0x04],
    ccr3: CCR3,
    cndtr3: CNDTR3,
    cpar3: CPAR3,
    cmar3: CMAR3,
    _reserved14: [u8; 0x04],
    ccr4: CCR4,
    cndtr4: CNDTR4,
    cpar4: CPAR4,
    cmar4: CMAR4,
    _reserved18: [u8; 0x04],
    ccr5: CCR5,
    cndtr5: CNDTR5,
    cpar5: CPAR5,
    cmar5: CMAR5,
    _reserved22: [u8; 0x04],
    ccr6: CCR6,
    cndtr6: CNDTR6,
    cpar6: CPAR6,
    cmar6: CMAR6,
    _reserved26: [u8; 0x04],
    ccr7: CCR7,
    cndtr7: CNDTR7,
    cpar7: CPAR7,
    cmar7: CMAR7,
    _reserved30: [u8; 0x04],
    ccr8: CCR8,
    cndtr8: CNDTR8,
    cpar8: CPAR8,
    cmar8: CMAR8,
}
impl RegisterBlock {
    ///0x00 - DMA_ISR register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x04 - DMA_IFCR register
    #[inline(always)]
    pub const fn ifcr(&self) -> &IFCR {
        &self.ifcr
    }
    ///0x08 - DMA_CCRx register
    #[inline(always)]
    pub const fn ccr1(&self) -> &CCR1 {
        &self.ccr1
    }
    ///0x0c - DMA_CNDTRx register
    #[inline(always)]
    pub const fn cndtr1(&self) -> &CNDTR1 {
        &self.cndtr1
    }
    ///0x10 - DMA_CPARx register
    #[inline(always)]
    pub const fn cpar1(&self) -> &CPAR1 {
        &self.cpar1
    }
    ///0x14 - DMA_CMARx register
    #[inline(always)]
    pub const fn cmar1(&self) -> &CMAR1 {
        &self.cmar1
    }
    ///0x1c - DMA_CCRx register
    #[inline(always)]
    pub const fn ccr2(&self) -> &CCR2 {
        &self.ccr2
    }
    ///0x20 - DMA_CNDTRx register
    #[inline(always)]
    pub const fn cndtr2(&self) -> &CNDTR2 {
        &self.cndtr2
    }
    ///0x24 - DMA_CPARx register
    #[inline(always)]
    pub const fn cpar2(&self) -> &CPAR2 {
        &self.cpar2
    }
    ///0x28 - DMA_CMARx register
    #[inline(always)]
    pub const fn cmar2(&self) -> &CMAR2 {
        &self.cmar2
    }
    ///0x30 - DMA_CCRx register
    #[inline(always)]
    pub const fn ccr3(&self) -> &CCR3 {
        &self.ccr3
    }
    ///0x34 - DMA_CNDTRx register
    #[inline(always)]
    pub const fn cndtr3(&self) -> &CNDTR3 {
        &self.cndtr3
    }
    ///0x38 - DMA_CPARx register
    #[inline(always)]
    pub const fn cpar3(&self) -> &CPAR3 {
        &self.cpar3
    }
    ///0x3c - DMA_CMARx register
    #[inline(always)]
    pub const fn cmar3(&self) -> &CMAR3 {
        &self.cmar3
    }
    ///0x44 - DMA_CCRx register
    #[inline(always)]
    pub const fn ccr4(&self) -> &CCR4 {
        &self.ccr4
    }
    ///0x48 - DMA_CNDTRx register
    #[inline(always)]
    pub const fn cndtr4(&self) -> &CNDTR4 {
        &self.cndtr4
    }
    ///0x4c - DMA_CPARx register
    #[inline(always)]
    pub const fn cpar4(&self) -> &CPAR4 {
        &self.cpar4
    }
    ///0x50 - DMA_CMARx register
    #[inline(always)]
    pub const fn cmar4(&self) -> &CMAR4 {
        &self.cmar4
    }
    ///0x58 - DMA_CCRx register
    #[inline(always)]
    pub const fn ccr5(&self) -> &CCR5 {
        &self.ccr5
    }
    ///0x5c - DMA_CNDTRx register
    #[inline(always)]
    pub const fn cndtr5(&self) -> &CNDTR5 {
        &self.cndtr5
    }
    ///0x60 - DMA_CPARx register
    #[inline(always)]
    pub const fn cpar5(&self) -> &CPAR5 {
        &self.cpar5
    }
    ///0x64 - DMA_CMARx register
    #[inline(always)]
    pub const fn cmar5(&self) -> &CMAR5 {
        &self.cmar5
    }
    ///0x6c - DMA_CCRx register
    #[inline(always)]
    pub const fn ccr6(&self) -> &CCR6 {
        &self.ccr6
    }
    ///0x70 - DMA_CNDTRx register
    #[inline(always)]
    pub const fn cndtr6(&self) -> &CNDTR6 {
        &self.cndtr6
    }
    ///0x74 - DMA_CPARx register
    #[inline(always)]
    pub const fn cpar6(&self) -> &CPAR6 {
        &self.cpar6
    }
    ///0x78 - DMA_CMARx register
    #[inline(always)]
    pub const fn cmar6(&self) -> &CMAR6 {
        &self.cmar6
    }
    ///0x80 - DMA_CCRx register
    #[inline(always)]
    pub const fn ccr7(&self) -> &CCR7 {
        &self.ccr7
    }
    ///0x84 - DMA_CNDTRx register
    #[inline(always)]
    pub const fn cndtr7(&self) -> &CNDTR7 {
        &self.cndtr7
    }
    ///0x88 - DMA_CPARx register
    #[inline(always)]
    pub const fn cpar7(&self) -> &CPAR7 {
        &self.cpar7
    }
    ///0x8c - DMA_CMARx register
    #[inline(always)]
    pub const fn cmar7(&self) -> &CMAR7 {
        &self.cmar7
    }
    ///0x94 - DMA_CCRx register
    #[inline(always)]
    pub const fn ccr8(&self) -> &CCR8 {
        &self.ccr8
    }
    ///0x98 - DMA_CNDTRx register
    #[inline(always)]
    pub const fn cndtr8(&self) -> &CNDTR8 {
        &self.cndtr8
    }
    ///0x9c - DMA_CPARx register
    #[inline(always)]
    pub const fn cpar8(&self) -> &CPAR8 {
        &self.cpar8
    }
    ///0xa0 - DMA_CMARx register
    #[inline(always)]
    pub const fn cmar8(&self) -> &CMAR8 {
        &self.cmar8
    }
}
/**ISR (r) register accessor: DMA_ISR register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///DMA_ISR register
pub mod isr;
/**IFCR (w) register accessor: DMA_IFCR register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:IFCR)

For information about available fields see [`mod@ifcr`] module*/
pub type IFCR = crate::Reg<ifcr::IFCRrs>;
///DMA_IFCR register
pub mod ifcr;
/**CCR1 (rw) register accessor: DMA_CCRx register

You can [`read`](crate::Reg::read) this register and get [`ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CCR1)

For information about available fields see [`mod@ccr1`] module*/
pub type CCR1 = crate::Reg<ccr1::CCR1rs>;
///DMA_CCRx register
pub mod ccr1;
/**CNDTR1 (rw) register accessor: DMA_CNDTRx register

You can [`read`](crate::Reg::read) this register and get [`cndtr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CNDTR1)

For information about available fields see [`mod@cndtr1`] module*/
pub type CNDTR1 = crate::Reg<cndtr1::CNDTR1rs>;
///DMA_CNDTRx register
pub mod cndtr1;
/**CPAR1 (rw) register accessor: DMA_CPARx register

You can [`read`](crate::Reg::read) this register and get [`cpar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CPAR1)

For information about available fields see [`mod@cpar1`] module*/
pub type CPAR1 = crate::Reg<cpar1::CPAR1rs>;
///DMA_CPARx register
pub mod cpar1;
/**CMAR1 (rw) register accessor: DMA_CMARx register

You can [`read`](crate::Reg::read) this register and get [`cmar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CMAR1)

For information about available fields see [`mod@cmar1`] module*/
pub type CMAR1 = crate::Reg<cmar1::CMAR1rs>;
///DMA_CMARx register
pub mod cmar1;
/**CCR2 (rw) register accessor: DMA_CCRx register

You can [`read`](crate::Reg::read) this register and get [`ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CCR2)

For information about available fields see [`mod@ccr2`] module*/
pub type CCR2 = crate::Reg<ccr2::CCR2rs>;
///DMA_CCRx register
pub mod ccr2;
/**CNDTR2 (rw) register accessor: DMA_CNDTRx register

You can [`read`](crate::Reg::read) this register and get [`cndtr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CNDTR2)

For information about available fields see [`mod@cndtr2`] module*/
pub type CNDTR2 = crate::Reg<cndtr2::CNDTR2rs>;
///DMA_CNDTRx register
pub mod cndtr2;
/**CPAR2 (rw) register accessor: DMA_CPARx register

You can [`read`](crate::Reg::read) this register and get [`cpar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CPAR2)

For information about available fields see [`mod@cpar2`] module*/
pub type CPAR2 = crate::Reg<cpar2::CPAR2rs>;
///DMA_CPARx register
pub mod cpar2;
/**CMAR2 (rw) register accessor: DMA_CMARx register

You can [`read`](crate::Reg::read) this register and get [`cmar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CMAR2)

For information about available fields see [`mod@cmar2`] module*/
pub type CMAR2 = crate::Reg<cmar2::CMAR2rs>;
///DMA_CMARx register
pub mod cmar2;
/**CCR3 (rw) register accessor: DMA_CCRx register

You can [`read`](crate::Reg::read) this register and get [`ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CCR3)

For information about available fields see [`mod@ccr3`] module*/
pub type CCR3 = crate::Reg<ccr3::CCR3rs>;
///DMA_CCRx register
pub mod ccr3;
/**CNDTR3 (rw) register accessor: DMA_CNDTRx register

You can [`read`](crate::Reg::read) this register and get [`cndtr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CNDTR3)

For information about available fields see [`mod@cndtr3`] module*/
pub type CNDTR3 = crate::Reg<cndtr3::CNDTR3rs>;
///DMA_CNDTRx register
pub mod cndtr3;
/**CPAR3 (rw) register accessor: DMA_CPARx register

You can [`read`](crate::Reg::read) this register and get [`cpar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CPAR3)

For information about available fields see [`mod@cpar3`] module*/
pub type CPAR3 = crate::Reg<cpar3::CPAR3rs>;
///DMA_CPARx register
pub mod cpar3;
/**CMAR3 (rw) register accessor: DMA_CMARx register

You can [`read`](crate::Reg::read) this register and get [`cmar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CMAR3)

For information about available fields see [`mod@cmar3`] module*/
pub type CMAR3 = crate::Reg<cmar3::CMAR3rs>;
///DMA_CMARx register
pub mod cmar3;
/**CCR4 (rw) register accessor: DMA_CCRx register

You can [`read`](crate::Reg::read) this register and get [`ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CCR4)

For information about available fields see [`mod@ccr4`] module*/
pub type CCR4 = crate::Reg<ccr4::CCR4rs>;
///DMA_CCRx register
pub mod ccr4;
/**CNDTR4 (rw) register accessor: DMA_CNDTRx register

You can [`read`](crate::Reg::read) this register and get [`cndtr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CNDTR4)

For information about available fields see [`mod@cndtr4`] module*/
pub type CNDTR4 = crate::Reg<cndtr4::CNDTR4rs>;
///DMA_CNDTRx register
pub mod cndtr4;
/**CPAR4 (rw) register accessor: DMA_CPARx register

You can [`read`](crate::Reg::read) this register and get [`cpar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CPAR4)

For information about available fields see [`mod@cpar4`] module*/
pub type CPAR4 = crate::Reg<cpar4::CPAR4rs>;
///DMA_CPARx register
pub mod cpar4;
/**CMAR4 (rw) register accessor: DMA_CMARx register

You can [`read`](crate::Reg::read) this register and get [`cmar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CMAR4)

For information about available fields see [`mod@cmar4`] module*/
pub type CMAR4 = crate::Reg<cmar4::CMAR4rs>;
///DMA_CMARx register
pub mod cmar4;
/**CCR5 (rw) register accessor: DMA_CCRx register

You can [`read`](crate::Reg::read) this register and get [`ccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CCR5)

For information about available fields see [`mod@ccr5`] module*/
pub type CCR5 = crate::Reg<ccr5::CCR5rs>;
///DMA_CCRx register
pub mod ccr5;
/**CNDTR5 (rw) register accessor: DMA_CNDTRx register

You can [`read`](crate::Reg::read) this register and get [`cndtr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CNDTR5)

For information about available fields see [`mod@cndtr5`] module*/
pub type CNDTR5 = crate::Reg<cndtr5::CNDTR5rs>;
///DMA_CNDTRx register
pub mod cndtr5;
/**CPAR5 (rw) register accessor: DMA_CPARx register

You can [`read`](crate::Reg::read) this register and get [`cpar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CPAR5)

For information about available fields see [`mod@cpar5`] module*/
pub type CPAR5 = crate::Reg<cpar5::CPAR5rs>;
///DMA_CPARx register
pub mod cpar5;
/**CMAR5 (rw) register accessor: DMA_CMARx register

You can [`read`](crate::Reg::read) this register and get [`cmar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CMAR5)

For information about available fields see [`mod@cmar5`] module*/
pub type CMAR5 = crate::Reg<cmar5::CMAR5rs>;
///DMA_CMARx register
pub mod cmar5;
/**CCR6 (rw) register accessor: DMA_CCRx register

You can [`read`](crate::Reg::read) this register and get [`ccr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CCR6)

For information about available fields see [`mod@ccr6`] module*/
pub type CCR6 = crate::Reg<ccr6::CCR6rs>;
///DMA_CCRx register
pub mod ccr6;
/**CNDTR6 (rw) register accessor: DMA_CNDTRx register

You can [`read`](crate::Reg::read) this register and get [`cndtr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CNDTR6)

For information about available fields see [`mod@cndtr6`] module*/
pub type CNDTR6 = crate::Reg<cndtr6::CNDTR6rs>;
///DMA_CNDTRx register
pub mod cndtr6;
/**CPAR6 (rw) register accessor: DMA_CPARx register

You can [`read`](crate::Reg::read) this register and get [`cpar6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CPAR6)

For information about available fields see [`mod@cpar6`] module*/
pub type CPAR6 = crate::Reg<cpar6::CPAR6rs>;
///DMA_CPARx register
pub mod cpar6;
/**CMAR6 (rw) register accessor: DMA_CMARx register

You can [`read`](crate::Reg::read) this register and get [`cmar6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CMAR6)

For information about available fields see [`mod@cmar6`] module*/
pub type CMAR6 = crate::Reg<cmar6::CMAR6rs>;
///DMA_CMARx register
pub mod cmar6;
/**CCR7 (rw) register accessor: DMA_CCRx register

You can [`read`](crate::Reg::read) this register and get [`ccr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CCR7)

For information about available fields see [`mod@ccr7`] module*/
pub type CCR7 = crate::Reg<ccr7::CCR7rs>;
///DMA_CCRx register
pub mod ccr7;
/**CNDTR7 (rw) register accessor: DMA_CNDTRx register

You can [`read`](crate::Reg::read) this register and get [`cndtr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CNDTR7)

For information about available fields see [`mod@cndtr7`] module*/
pub type CNDTR7 = crate::Reg<cndtr7::CNDTR7rs>;
///DMA_CNDTRx register
pub mod cndtr7;
/**CPAR7 (rw) register accessor: DMA_CPARx register

You can [`read`](crate::Reg::read) this register and get [`cpar7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CPAR7)

For information about available fields see [`mod@cpar7`] module*/
pub type CPAR7 = crate::Reg<cpar7::CPAR7rs>;
///DMA_CPARx register
pub mod cpar7;
/**CMAR7 (rw) register accessor: DMA_CMARx register

You can [`read`](crate::Reg::read) this register and get [`cmar7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CMAR7)

For information about available fields see [`mod@cmar7`] module*/
pub type CMAR7 = crate::Reg<cmar7::CMAR7rs>;
///DMA_CMARx register
pub mod cmar7;
/**CCR8 (rw) register accessor: DMA_CCRx register

You can [`read`](crate::Reg::read) this register and get [`ccr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CCR8)

For information about available fields see [`mod@ccr8`] module*/
pub type CCR8 = crate::Reg<ccr8::CCR8rs>;
///DMA_CCRx register
pub mod ccr8;
/**CNDTR8 (rw) register accessor: DMA_CNDTRx register

You can [`read`](crate::Reg::read) this register and get [`cndtr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CNDTR8)

For information about available fields see [`mod@cndtr8`] module*/
pub type CNDTR8 = crate::Reg<cndtr8::CNDTR8rs>;
///DMA_CNDTRx register
pub mod cndtr8;
/**CPAR8 (rw) register accessor: DMA_CPARx register

You can [`read`](crate::Reg::read) this register and get [`cpar8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CPAR8)

For information about available fields see [`mod@cpar8`] module*/
pub type CPAR8 = crate::Reg<cpar8::CPAR8rs>;
///DMA_CPARx register
pub mod cpar8;
/**CMAR8 (rw) register accessor: DMA_CMARx register

You can [`read`](crate::Reg::read) this register and get [`cmar8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:CMAR8)

For information about available fields see [`mod@cmar8`] module*/
pub type CMAR8 = crate::Reg<cmar8::CMAR8rs>;
///DMA_CMARx register
pub mod cmar8;
