#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    rtsr1: RTSR1,
    ftsr1: FTSR1,
    swier1: SWIER1,
    d3pmr1: D3PMR1,
    d3pcr1l: D3PCR1L,
    d3pcr1h: D3PCR1H,
    _reserved6: [u8; 0x08],
    rtsr2: RTSR2,
    ftsr2: FTSR2,
    swier2: SWIER2,
    d3pmr2: D3PMR2,
    d3pcr2l: D3PCR2L,
    d3pcr2h: D3PCR2H,
    _reserved12: [u8; 0x08],
    rtsr3: RTSR3,
    ftsr3: FTSR3,
    swier3: SWIER3,
    d3pmr3: D3PMR3,
    _reserved16: [u8; 0x04],
    d3pcr3h: D3PCR3H,
    _reserved17: [u8; 0x28],
    c1imr1: C1IMR1,
    c1emr1: C1EMR1,
    c1pr1: C1PR1,
    _reserved20: [u8; 0x04],
    c1imr2: C1IMR2,
    c1emr2: C1EMR2,
    c1pr2: C1PR2,
    _reserved23: [u8; 0x04],
    c1imr3: C1IMR3,
    c1emr3: C1EMR3,
    c1pr3: C1PR3,
    _reserved26: [u8; 0x14],
    c2imr1: C2IMR1,
    c2emr1: C2EMR1,
    c2pr1: C2PR1,
    _reserved29: [u8; 0x04],
    c2imr2: C2IMR2,
    c2emr2: C2EMR2,
    c2pr2: C2PR2,
    _reserved32: [u8; 0x04],
    c2imr3: C2IMR3,
    c2emr3: C2EMR3,
    c2pr3: C2PR3,
}
impl RegisterBlock {
    ///0x00 - EXTI rising trigger selection register
    #[inline(always)]
    pub const fn rtsr1(&self) -> &RTSR1 {
        &self.rtsr1
    }
    ///0x04 - EXTI falling trigger selection register
    #[inline(always)]
    pub const fn ftsr1(&self) -> &FTSR1 {
        &self.ftsr1
    }
    ///0x08 - EXTI software interrupt event register
    #[inline(always)]
    pub const fn swier1(&self) -> &SWIER1 {
        &self.swier1
    }
    ///0x0c - EXTI D3 pending mask register
    #[inline(always)]
    pub const fn d3pmr1(&self) -> &D3PMR1 {
        &self.d3pmr1
    }
    ///0x10 - EXTI D3 pending clear selection register low
    #[inline(always)]
    pub const fn d3pcr1l(&self) -> &D3PCR1L {
        &self.d3pcr1l
    }
    ///0x14 - EXTI D3 pending clear selection register high
    #[inline(always)]
    pub const fn d3pcr1h(&self) -> &D3PCR1H {
        &self.d3pcr1h
    }
    ///0x20 - EXTI rising trigger selection register
    #[inline(always)]
    pub const fn rtsr2(&self) -> &RTSR2 {
        &self.rtsr2
    }
    ///0x24 - EXTI falling trigger selection register
    #[inline(always)]
    pub const fn ftsr2(&self) -> &FTSR2 {
        &self.ftsr2
    }
    ///0x28 - EXTI software interrupt event register
    #[inline(always)]
    pub const fn swier2(&self) -> &SWIER2 {
        &self.swier2
    }
    ///0x2c - EXTI D3 pending mask register
    #[inline(always)]
    pub const fn d3pmr2(&self) -> &D3PMR2 {
        &self.d3pmr2
    }
    ///0x30 - EXTI D3 pending clear selection register low
    #[inline(always)]
    pub const fn d3pcr2l(&self) -> &D3PCR2L {
        &self.d3pcr2l
    }
    ///0x34 - EXTI D3 pending clear selection register high
    #[inline(always)]
    pub const fn d3pcr2h(&self) -> &D3PCR2H {
        &self.d3pcr2h
    }
    ///0x40 - EXTI rising trigger selection register
    #[inline(always)]
    pub const fn rtsr3(&self) -> &RTSR3 {
        &self.rtsr3
    }
    ///0x44 - EXTI falling trigger selection register
    #[inline(always)]
    pub const fn ftsr3(&self) -> &FTSR3 {
        &self.ftsr3
    }
    ///0x48 - EXTI software interrupt event register
    #[inline(always)]
    pub const fn swier3(&self) -> &SWIER3 {
        &self.swier3
    }
    ///0x4c - EXTI D3 pending mask register
    #[inline(always)]
    pub const fn d3pmr3(&self) -> &D3PMR3 {
        &self.d3pmr3
    }
    ///0x54 - EXTI D3 pending clear selection register high
    #[inline(always)]
    pub const fn d3pcr3h(&self) -> &D3PCR3H {
        &self.d3pcr3h
    }
    ///0x80 - EXTI interrupt mask register
    #[inline(always)]
    pub const fn c1imr1(&self) -> &C1IMR1 {
        &self.c1imr1
    }
    ///0x84 - EXTI event mask register
    #[inline(always)]
    pub const fn c1emr1(&self) -> &C1EMR1 {
        &self.c1emr1
    }
    ///0x88 - EXTI pending register
    #[inline(always)]
    pub const fn c1pr1(&self) -> &C1PR1 {
        &self.c1pr1
    }
    ///0x90 - EXTI interrupt mask register
    #[inline(always)]
    pub const fn c1imr2(&self) -> &C1IMR2 {
        &self.c1imr2
    }
    ///0x94 - EXTI event mask register
    #[inline(always)]
    pub const fn c1emr2(&self) -> &C1EMR2 {
        &self.c1emr2
    }
    ///0x98 - EXTI pending register
    #[inline(always)]
    pub const fn c1pr2(&self) -> &C1PR2 {
        &self.c1pr2
    }
    ///0xa0 - EXTI interrupt mask register
    #[inline(always)]
    pub const fn c1imr3(&self) -> &C1IMR3 {
        &self.c1imr3
    }
    ///0xa4 - EXTI event mask register
    #[inline(always)]
    pub const fn c1emr3(&self) -> &C1EMR3 {
        &self.c1emr3
    }
    ///0xa8 - EXTI pending register
    #[inline(always)]
    pub const fn c1pr3(&self) -> &C1PR3 {
        &self.c1pr3
    }
    ///0xc0 - CPU2 EXTI interrupt mask register
    #[inline(always)]
    pub const fn c2imr1(&self) -> &C2IMR1 {
        &self.c2imr1
    }
    ///0xc4 - CPU2 EXTI event mask register
    #[inline(always)]
    pub const fn c2emr1(&self) -> &C2EMR1 {
        &self.c2emr1
    }
    ///0xc8 - CPU2 EXTI pending register
    #[inline(always)]
    pub const fn c2pr1(&self) -> &C2PR1 {
        &self.c2pr1
    }
    ///0xd0 - CPU2 EXTI interrupt mask register
    #[inline(always)]
    pub const fn c2imr2(&self) -> &C2IMR2 {
        &self.c2imr2
    }
    ///0xd4 - CPU2 EXTI event mask register
    #[inline(always)]
    pub const fn c2emr2(&self) -> &C2EMR2 {
        &self.c2emr2
    }
    ///0xd8 - CPU2 EXTI pending register
    #[inline(always)]
    pub const fn c2pr2(&self) -> &C2PR2 {
        &self.c2pr2
    }
    ///0xe0 - CPU2 EXTI interrupt mask register
    #[inline(always)]
    pub const fn c2imr3(&self) -> &C2IMR3 {
        &self.c2imr3
    }
    ///0xe4 - CPU2 EXTI event mask register
    #[inline(always)]
    pub const fn c2emr3(&self) -> &C2EMR3 {
        &self.c2emr3
    }
    ///0xe8 - CPU2 EXTI pending register
    #[inline(always)]
    pub const fn c2pr3(&self) -> &C2PR3 {
        &self.c2pr3
    }
}
/**RTSR1 (rw) register accessor: EXTI rising trigger selection register

You can [`read`](crate::Reg::read) this register and get [`rtsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:RTSR1)

For information about available fields see [`mod@rtsr1`] module*/
pub type RTSR1 = crate::Reg<rtsr1::RTSR1rs>;
///EXTI rising trigger selection register
pub mod rtsr1;
/**FTSR1 (rw) register accessor: EXTI falling trigger selection register

You can [`read`](crate::Reg::read) this register and get [`ftsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:FTSR1)

For information about available fields see [`mod@ftsr1`] module*/
pub type FTSR1 = crate::Reg<ftsr1::FTSR1rs>;
///EXTI falling trigger selection register
pub mod ftsr1;
/**SWIER1 (rw) register accessor: EXTI software interrupt event register

You can [`read`](crate::Reg::read) this register and get [`swier1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:SWIER1)

For information about available fields see [`mod@swier1`] module*/
pub type SWIER1 = crate::Reg<swier1::SWIER1rs>;
///EXTI software interrupt event register
pub mod swier1;
/**D3PMR1 (rw) register accessor: EXTI D3 pending mask register

You can [`read`](crate::Reg::read) this register and get [`d3pmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:D3PMR1)

For information about available fields see [`mod@d3pmr1`] module*/
pub type D3PMR1 = crate::Reg<d3pmr1::D3PMR1rs>;
///EXTI D3 pending mask register
pub mod d3pmr1;
/**D3PCR1L (rw) register accessor: EXTI D3 pending clear selection register low

You can [`read`](crate::Reg::read) this register and get [`d3pcr1l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pcr1l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:D3PCR1L)

For information about available fields see [`mod@d3pcr1l`] module*/
pub type D3PCR1L = crate::Reg<d3pcr1l::D3PCR1Lrs>;
///EXTI D3 pending clear selection register low
pub mod d3pcr1l;
/**D3PCR1H (rw) register accessor: EXTI D3 pending clear selection register high

You can [`read`](crate::Reg::read) this register and get [`d3pcr1h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pcr1h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:D3PCR1H)

For information about available fields see [`mod@d3pcr1h`] module*/
pub type D3PCR1H = crate::Reg<d3pcr1h::D3PCR1Hrs>;
///EXTI D3 pending clear selection register high
pub mod d3pcr1h;
/**RTSR2 (rw) register accessor: EXTI rising trigger selection register

You can [`read`](crate::Reg::read) this register and get [`rtsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:RTSR2)

For information about available fields see [`mod@rtsr2`] module*/
pub type RTSR2 = crate::Reg<rtsr2::RTSR2rs>;
///EXTI rising trigger selection register
pub mod rtsr2;
/**FTSR2 (rw) register accessor: EXTI falling trigger selection register

You can [`read`](crate::Reg::read) this register and get [`ftsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:FTSR2)

For information about available fields see [`mod@ftsr2`] module*/
pub type FTSR2 = crate::Reg<ftsr2::FTSR2rs>;
///EXTI falling trigger selection register
pub mod ftsr2;
/**SWIER2 (rw) register accessor: EXTI software interrupt event register

You can [`read`](crate::Reg::read) this register and get [`swier2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:SWIER2)

For information about available fields see [`mod@swier2`] module*/
pub type SWIER2 = crate::Reg<swier2::SWIER2rs>;
///EXTI software interrupt event register
pub mod swier2;
/**D3PMR2 (rw) register accessor: EXTI D3 pending mask register

You can [`read`](crate::Reg::read) this register and get [`d3pmr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pmr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:D3PMR2)

For information about available fields see [`mod@d3pmr2`] module*/
pub type D3PMR2 = crate::Reg<d3pmr2::D3PMR2rs>;
///EXTI D3 pending mask register
pub mod d3pmr2;
/**D3PCR2L (rw) register accessor: EXTI D3 pending clear selection register low

You can [`read`](crate::Reg::read) this register and get [`d3pcr2l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pcr2l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:D3PCR2L)

For information about available fields see [`mod@d3pcr2l`] module*/
pub type D3PCR2L = crate::Reg<d3pcr2l::D3PCR2Lrs>;
///EXTI D3 pending clear selection register low
pub mod d3pcr2l;
/**D3PCR2H (rw) register accessor: EXTI D3 pending clear selection register high

You can [`read`](crate::Reg::read) this register and get [`d3pcr2h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pcr2h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:D3PCR2H)

For information about available fields see [`mod@d3pcr2h`] module*/
pub type D3PCR2H = crate::Reg<d3pcr2h::D3PCR2Hrs>;
///EXTI D3 pending clear selection register high
pub mod d3pcr2h;
/**RTSR3 (rw) register accessor: EXTI rising trigger selection register

You can [`read`](crate::Reg::read) this register and get [`rtsr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:RTSR3)

For information about available fields see [`mod@rtsr3`] module*/
pub type RTSR3 = crate::Reg<rtsr3::RTSR3rs>;
///EXTI rising trigger selection register
pub mod rtsr3;
/**FTSR3 (rw) register accessor: EXTI falling trigger selection register

You can [`read`](crate::Reg::read) this register and get [`ftsr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:FTSR3)

For information about available fields see [`mod@ftsr3`] module*/
pub type FTSR3 = crate::Reg<ftsr3::FTSR3rs>;
///EXTI falling trigger selection register
pub mod ftsr3;
/**SWIER3 (rw) register accessor: EXTI software interrupt event register

You can [`read`](crate::Reg::read) this register and get [`swier3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:SWIER3)

For information about available fields see [`mod@swier3`] module*/
pub type SWIER3 = crate::Reg<swier3::SWIER3rs>;
///EXTI software interrupt event register
pub mod swier3;
/**D3PMR3 (rw) register accessor: EXTI D3 pending mask register

You can [`read`](crate::Reg::read) this register and get [`d3pmr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pmr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:D3PMR3)

For information about available fields see [`mod@d3pmr3`] module*/
pub type D3PMR3 = crate::Reg<d3pmr3::D3PMR3rs>;
///EXTI D3 pending mask register
pub mod d3pmr3;
/**D3PCR3H (rw) register accessor: EXTI D3 pending clear selection register high

You can [`read`](crate::Reg::read) this register and get [`d3pcr3h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pcr3h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:D3PCR3H)

For information about available fields see [`mod@d3pcr3h`] module*/
pub type D3PCR3H = crate::Reg<d3pcr3h::D3PCR3Hrs>;
///EXTI D3 pending clear selection register high
pub mod d3pcr3h;
/**C1IMR1 (rw) register accessor: EXTI interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`c1imr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1imr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:C1IMR1)

For information about available fields see [`mod@c1imr1`] module*/
pub type C1IMR1 = crate::Reg<c1imr1::C1IMR1rs>;
///EXTI interrupt mask register
pub mod c1imr1;
/**C1EMR1 (rw) register accessor: EXTI event mask register

You can [`read`](crate::Reg::read) this register and get [`c1emr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1emr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:C1EMR1)

For information about available fields see [`mod@c1emr1`] module*/
pub type C1EMR1 = crate::Reg<c1emr1::C1EMR1rs>;
///EXTI event mask register
pub mod c1emr1;
/**C1PR1 (rw) register accessor: EXTI pending register

You can [`read`](crate::Reg::read) this register and get [`c1pr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1pr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:C1PR1)

For information about available fields see [`mod@c1pr1`] module*/
pub type C1PR1 = crate::Reg<c1pr1::C1PR1rs>;
///EXTI pending register
pub mod c1pr1;
/**C1IMR2 (rw) register accessor: EXTI interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`c1imr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1imr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:C1IMR2)

For information about available fields see [`mod@c1imr2`] module*/
pub type C1IMR2 = crate::Reg<c1imr2::C1IMR2rs>;
///EXTI interrupt mask register
pub mod c1imr2;
/**C1EMR2 (rw) register accessor: EXTI event mask register

You can [`read`](crate::Reg::read) this register and get [`c1emr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1emr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:C1EMR2)

For information about available fields see [`mod@c1emr2`] module*/
pub type C1EMR2 = crate::Reg<c1emr2::C1EMR2rs>;
///EXTI event mask register
pub mod c1emr2;
/**C1PR2 (rw) register accessor: EXTI pending register

You can [`read`](crate::Reg::read) this register and get [`c1pr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1pr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:C1PR2)

For information about available fields see [`mod@c1pr2`] module*/
pub type C1PR2 = crate::Reg<c1pr2::C1PR2rs>;
///EXTI pending register
pub mod c1pr2;
/**C1IMR3 (rw) register accessor: EXTI interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`c1imr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1imr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:C1IMR3)

For information about available fields see [`mod@c1imr3`] module*/
pub type C1IMR3 = crate::Reg<c1imr3::C1IMR3rs>;
///EXTI interrupt mask register
pub mod c1imr3;
/**C1EMR3 (rw) register accessor: EXTI event mask register

You can [`read`](crate::Reg::read) this register and get [`c1emr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1emr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:C1EMR3)

For information about available fields see [`mod@c1emr3`] module*/
pub type C1EMR3 = crate::Reg<c1emr3::C1EMR3rs>;
///EXTI event mask register
pub mod c1emr3;
/**C1PR3 (rw) register accessor: EXTI pending register

You can [`read`](crate::Reg::read) this register and get [`c1pr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1pr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:C1PR3)

For information about available fields see [`mod@c1pr3`] module*/
pub type C1PR3 = crate::Reg<c1pr3::C1PR3rs>;
///EXTI pending register
pub mod c1pr3;
/**C2IMR1 (rw) register accessor: CPU2 EXTI interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`c2imr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2imr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:C2IMR1)

For information about available fields see [`mod@c2imr1`] module*/
pub type C2IMR1 = crate::Reg<c2imr1::C2IMR1rs>;
///CPU2 EXTI interrupt mask register
pub mod c2imr1;
/**C2EMR1 (rw) register accessor: CPU2 EXTI event mask register

You can [`read`](crate::Reg::read) this register and get [`c2emr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2emr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:C2EMR1)

For information about available fields see [`mod@c2emr1`] module*/
pub type C2EMR1 = crate::Reg<c2emr1::C2EMR1rs>;
///CPU2 EXTI event mask register
pub mod c2emr1;
/**C2PR1 (rw) register accessor: CPU2 EXTI pending register

You can [`read`](crate::Reg::read) this register and get [`c2pr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2pr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:C2PR1)

For information about available fields see [`mod@c2pr1`] module*/
pub type C2PR1 = crate::Reg<c2pr1::C2PR1rs>;
///CPU2 EXTI pending register
pub mod c2pr1;
/**C2IMR2 (rw) register accessor: CPU2 EXTI interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`c2imr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2imr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:C2IMR2)

For information about available fields see [`mod@c2imr2`] module*/
pub type C2IMR2 = crate::Reg<c2imr2::C2IMR2rs>;
///CPU2 EXTI interrupt mask register
pub mod c2imr2;
/**C2EMR2 (rw) register accessor: CPU2 EXTI event mask register

You can [`read`](crate::Reg::read) this register and get [`c2emr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2emr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:C2EMR2)

For information about available fields see [`mod@c2emr2`] module*/
pub type C2EMR2 = crate::Reg<c2emr2::C2EMR2rs>;
///CPU2 EXTI event mask register
pub mod c2emr2;
/**C2PR2 (rw) register accessor: CPU2 EXTI pending register

You can [`read`](crate::Reg::read) this register and get [`c2pr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2pr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:C2PR2)

For information about available fields see [`mod@c2pr2`] module*/
pub type C2PR2 = crate::Reg<c2pr2::C2PR2rs>;
///CPU2 EXTI pending register
pub mod c2pr2;
/**C2IMR3 (rw) register accessor: CPU2 EXTI interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`c2imr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2imr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:C2IMR3)

For information about available fields see [`mod@c2imr3`] module*/
pub type C2IMR3 = crate::Reg<c2imr3::C2IMR3rs>;
///CPU2 EXTI interrupt mask register
pub mod c2imr3;
/**C2EMR3 (rw) register accessor: CPU2 EXTI event mask register

You can [`read`](crate::Reg::read) this register and get [`c2emr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2emr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:C2EMR3)

For information about available fields see [`mod@c2emr3`] module*/
pub type C2EMR3 = crate::Reg<c2emr3::C2EMR3rs>;
///CPU2 EXTI event mask register
pub mod c2emr3;
/**C2PR3 (rw) register accessor: CPU2 EXTI pending register

You can [`read`](crate::Reg::read) this register and get [`c2pr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2pr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#EXTI:C2PR3)

For information about available fields see [`mod@c2pr3`] module*/
pub type C2PR3 = crate::Reg<c2pr3::C2PR3rs>;
///CPU2 EXTI pending register
pub mod c2pr3;
