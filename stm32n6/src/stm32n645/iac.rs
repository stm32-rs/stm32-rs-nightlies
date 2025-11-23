#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ier0: IER0,
    ier1: IER1,
    ier2: IER2,
    ier3: IER3,
    ier4: IER4,
    ier5: IER5,
    _reserved6: [u8; 0x68],
    isr0: ISR0,
    isr1: ISR1,
    isr2: ISR2,
    isr3: ISR3,
    isr4: ISR4,
    isr5: ISR5,
    _reserved12: [u8; 0x68],
    icr0: ICR0,
    icr1: ICR1,
    icr2: ICR2,
    icr3: ICR3,
    icr4: ICR4,
    icr5: ICR5,
    _reserved18: [u8; 0x0254],
    iisr0: IISR0,
    iisr1: IISR1,
    iisr2: IISR2,
    iisr3: IISR3,
    iisr4: IISR4,
    _reserved23: [u8; 0x04],
    iisr5: IISR5,
}
impl RegisterBlock {
    ///0x00 - IAC interrupt enable register 0
    #[inline(always)]
    pub const fn ier0(&self) -> &IER0 {
        &self.ier0
    }
    ///0x04 - IAC interrupt enable register 1
    #[inline(always)]
    pub const fn ier1(&self) -> &IER1 {
        &self.ier1
    }
    ///0x08 - IAC interrupt enable register 2
    #[inline(always)]
    pub const fn ier2(&self) -> &IER2 {
        &self.ier2
    }
    ///0x0c - IAC interrupt enable register 3
    #[inline(always)]
    pub const fn ier3(&self) -> &IER3 {
        &self.ier3
    }
    ///0x10 - IAC interrupt enable register 4
    #[inline(always)]
    pub const fn ier4(&self) -> &IER4 {
        &self.ier4
    }
    ///0x14 - IAC interrupt enable register 5
    #[inline(always)]
    pub const fn ier5(&self) -> &IER5 {
        &self.ier5
    }
    ///0x80 - IAC interrupt status register 0
    #[inline(always)]
    pub const fn isr0(&self) -> &ISR0 {
        &self.isr0
    }
    ///0x84 - IAC interrupt status register 1
    #[inline(always)]
    pub const fn isr1(&self) -> &ISR1 {
        &self.isr1
    }
    ///0x88 - IAC interrupt status register 2
    #[inline(always)]
    pub const fn isr2(&self) -> &ISR2 {
        &self.isr2
    }
    ///0x8c - IAC interrupt status register 3
    #[inline(always)]
    pub const fn isr3(&self) -> &ISR3 {
        &self.isr3
    }
    ///0x90 - IAC interrupt status register 4
    #[inline(always)]
    pub const fn isr4(&self) -> &ISR4 {
        &self.isr4
    }
    ///0x94 - IAC interrupt status register 5
    #[inline(always)]
    pub const fn isr5(&self) -> &ISR5 {
        &self.isr5
    }
    ///0x100 - IAC interrupt clear register 0
    #[inline(always)]
    pub const fn icr0(&self) -> &ICR0 {
        &self.icr0
    }
    ///0x104 - IAC interrupt clear register 1
    #[inline(always)]
    pub const fn icr1(&self) -> &ICR1 {
        &self.icr1
    }
    ///0x108 - IAC interrupt clear register 2
    #[inline(always)]
    pub const fn icr2(&self) -> &ICR2 {
        &self.icr2
    }
    ///0x10c - IAC interrupt clear register 3
    #[inline(always)]
    pub const fn icr3(&self) -> &ICR3 {
        &self.icr3
    }
    ///0x110 - IAC interrupt clear register 4
    #[inline(always)]
    pub const fn icr4(&self) -> &ICR4 {
        &self.icr4
    }
    ///0x114 - IAC interrupt clear register 5
    #[inline(always)]
    pub const fn icr5(&self) -> &ICR5 {
        &self.icr5
    }
    ///0x36c - IAC ILAC input status register 0
    #[inline(always)]
    pub const fn iisr0(&self) -> &IISR0 {
        &self.iisr0
    }
    ///0x370 - IAC ILAC input status register 1
    #[inline(always)]
    pub const fn iisr1(&self) -> &IISR1 {
        &self.iisr1
    }
    ///0x374 - IAC ILAC input status register 2
    #[inline(always)]
    pub const fn iisr2(&self) -> &IISR2 {
        &self.iisr2
    }
    ///0x378 - IAC ILAC input status register 3
    #[inline(always)]
    pub const fn iisr3(&self) -> &IISR3 {
        &self.iisr3
    }
    ///0x37c - IAC ILAC input status register 4
    #[inline(always)]
    pub const fn iisr4(&self) -> &IISR4 {
        &self.iisr4
    }
    ///0x384 - IAC ILAC input status register 5
    #[inline(always)]
    pub const fn iisr5(&self) -> &IISR5 {
        &self.iisr5
    }
}
/**IER0 (rw) register accessor: IAC interrupt enable register 0

You can [`read`](crate::Reg::read) this register and get [`ier0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:IER0)

For information about available fields see [`mod@ier0`] module*/
pub type IER0 = crate::Reg<ier0::IER0rs>;
///IAC interrupt enable register 0
pub mod ier0;
/**IER1 (rw) register accessor: IAC interrupt enable register 1

You can [`read`](crate::Reg::read) this register and get [`ier1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:IER1)

For information about available fields see [`mod@ier1`] module*/
pub type IER1 = crate::Reg<ier1::IER1rs>;
///IAC interrupt enable register 1
pub mod ier1;
/**IER2 (rw) register accessor: IAC interrupt enable register 2

You can [`read`](crate::Reg::read) this register and get [`ier2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:IER2)

For information about available fields see [`mod@ier2`] module*/
pub type IER2 = crate::Reg<ier2::IER2rs>;
///IAC interrupt enable register 2
pub mod ier2;
/**IER3 (rw) register accessor: IAC interrupt enable register 3

You can [`read`](crate::Reg::read) this register and get [`ier3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:IER3)

For information about available fields see [`mod@ier3`] module*/
pub type IER3 = crate::Reg<ier3::IER3rs>;
///IAC interrupt enable register 3
pub mod ier3;
/**IER4 (rw) register accessor: IAC interrupt enable register 4

You can [`read`](crate::Reg::read) this register and get [`ier4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:IER4)

For information about available fields see [`mod@ier4`] module*/
pub type IER4 = crate::Reg<ier4::IER4rs>;
///IAC interrupt enable register 4
pub mod ier4;
/**IER5 (rw) register accessor: IAC interrupt enable register 5

You can [`read`](crate::Reg::read) this register and get [`ier5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:IER5)

For information about available fields see [`mod@ier5`] module*/
pub type IER5 = crate::Reg<ier5::IER5rs>;
///IAC interrupt enable register 5
pub mod ier5;
/**ISR0 (r) register accessor: IAC interrupt status register 0

You can [`read`](crate::Reg::read) this register and get [`isr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:ISR0)

For information about available fields see [`mod@isr0`] module*/
pub type ISR0 = crate::Reg<isr0::ISR0rs>;
///IAC interrupt status register 0
pub mod isr0;
/**ISR1 (r) register accessor: IAC interrupt status register 1

You can [`read`](crate::Reg::read) this register and get [`isr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:ISR1)

For information about available fields see [`mod@isr1`] module*/
pub type ISR1 = crate::Reg<isr1::ISR1rs>;
///IAC interrupt status register 1
pub mod isr1;
/**ISR2 (r) register accessor: IAC interrupt status register 2

You can [`read`](crate::Reg::read) this register and get [`isr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:ISR2)

For information about available fields see [`mod@isr2`] module*/
pub type ISR2 = crate::Reg<isr2::ISR2rs>;
///IAC interrupt status register 2
pub mod isr2;
/**ISR3 (r) register accessor: IAC interrupt status register 3

You can [`read`](crate::Reg::read) this register and get [`isr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:ISR3)

For information about available fields see [`mod@isr3`] module*/
pub type ISR3 = crate::Reg<isr3::ISR3rs>;
///IAC interrupt status register 3
pub mod isr3;
/**ISR4 (r) register accessor: IAC interrupt status register 4

You can [`read`](crate::Reg::read) this register and get [`isr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:ISR4)

For information about available fields see [`mod@isr4`] module*/
pub type ISR4 = crate::Reg<isr4::ISR4rs>;
///IAC interrupt status register 4
pub mod isr4;
/**ISR5 (r) register accessor: IAC interrupt status register 5

You can [`read`](crate::Reg::read) this register and get [`isr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:ISR5)

For information about available fields see [`mod@isr5`] module*/
pub type ISR5 = crate::Reg<isr5::ISR5rs>;
///IAC interrupt status register 5
pub mod isr5;
/**ICR0 (w) register accessor: IAC interrupt clear register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:ICR0)

For information about available fields see [`mod@icr0`] module*/
pub type ICR0 = crate::Reg<icr0::ICR0rs>;
///IAC interrupt clear register 0
pub mod icr0;
/**ICR1 (w) register accessor: IAC interrupt clear register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:ICR1)

For information about available fields see [`mod@icr1`] module*/
pub type ICR1 = crate::Reg<icr1::ICR1rs>;
///IAC interrupt clear register 1
pub mod icr1;
/**ICR2 (w) register accessor: IAC interrupt clear register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:ICR2)

For information about available fields see [`mod@icr2`] module*/
pub type ICR2 = crate::Reg<icr2::ICR2rs>;
///IAC interrupt clear register 2
pub mod icr2;
/**ICR3 (w) register accessor: IAC interrupt clear register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:ICR3)

For information about available fields see [`mod@icr3`] module*/
pub type ICR3 = crate::Reg<icr3::ICR3rs>;
///IAC interrupt clear register 3
pub mod icr3;
/**ICR4 (w) register accessor: IAC interrupt clear register 4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:ICR4)

For information about available fields see [`mod@icr4`] module*/
pub type ICR4 = crate::Reg<icr4::ICR4rs>;
///IAC interrupt clear register 4
pub mod icr4;
/**ICR5 (w) register accessor: IAC interrupt clear register 5

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:ICR5)

For information about available fields see [`mod@icr5`] module*/
pub type ICR5 = crate::Reg<icr5::ICR5rs>;
///IAC interrupt clear register 5
pub mod icr5;
/**IISR0 (r) register accessor: IAC ILAC input status register 0

You can [`read`](crate::Reg::read) this register and get [`iisr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:IISR0)

For information about available fields see [`mod@iisr0`] module*/
pub type IISR0 = crate::Reg<iisr0::IISR0rs>;
///IAC ILAC input status register 0
pub mod iisr0;
/**IISR1 (r) register accessor: IAC ILAC input status register 1

You can [`read`](crate::Reg::read) this register and get [`iisr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:IISR1)

For information about available fields see [`mod@iisr1`] module*/
pub type IISR1 = crate::Reg<iisr1::IISR1rs>;
///IAC ILAC input status register 1
pub mod iisr1;
/**IISR2 (r) register accessor: IAC ILAC input status register 2

You can [`read`](crate::Reg::read) this register and get [`iisr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:IISR2)

For information about available fields see [`mod@iisr2`] module*/
pub type IISR2 = crate::Reg<iisr2::IISR2rs>;
///IAC ILAC input status register 2
pub mod iisr2;
/**IISR3 (r) register accessor: IAC ILAC input status register 3

You can [`read`](crate::Reg::read) this register and get [`iisr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:IISR3)

For information about available fields see [`mod@iisr3`] module*/
pub type IISR3 = crate::Reg<iisr3::IISR3rs>;
///IAC ILAC input status register 3
pub mod iisr3;
/**IISR4 (r) register accessor: IAC ILAC input status register 4

You can [`read`](crate::Reg::read) this register and get [`iisr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:IISR4)

For information about available fields see [`mod@iisr4`] module*/
pub type IISR4 = crate::Reg<iisr4::IISR4rs>;
///IAC ILAC input status register 4
pub mod iisr4;
/**IISR5 (r) register accessor: IAC ILAC input status register 5

You can [`read`](crate::Reg::read) this register and get [`iisr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:IISR5)

For information about available fields see [`mod@iisr5`] module*/
pub type IISR5 = crate::Reg<iisr5::IISR5rs>;
///IAC ILAC input status register 5
pub mod iisr5;
