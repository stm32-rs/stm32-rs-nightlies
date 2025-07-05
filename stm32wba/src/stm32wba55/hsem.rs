#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    r0: R0,
    r1: R1,
    r2: R2,
    r3: R3,
    r4: R4,
    r5: R5,
    r6: R6,
    r7: R7,
    r8: R8,
    r9: R9,
    r10: R10,
    r11: R11,
    r12: R12,
    r13: R13,
    r14: R14,
    r15: R15,
    _reserved16: [u8; 0x40],
    rlr0: RLR0,
    rlr1: RLR1,
    rlr2: RLR2,
    rlr3: RLR3,
    rlr4: RLR4,
    rlr5: RLR5,
    rlr6: RLR6,
    rlr7: RLR7,
    rlr8: RLR8,
    rlr9: RLR9,
    rlr10: RLR10,
    rlr11: RLR11,
    rlr12: RLR12,
    rlr13: RLR13,
    rlr14: RLR14,
    rlr15: RLR15,
    _reserved32: [u8; 0x40],
    ier: IER,
    icr: ICR,
    isr: ISR,
    misr: MISR,
    _reserved36: [u8; 0x70],
    sier: SIER,
    sicr: SICR,
    sisr: SISR,
    msisr: MSISR,
    _reserved40: [u8; 0x70],
    seccfgr: SECCFGR,
    _reserved41: [u8; 0x0c],
    privcfgr: PRIVCFGR,
    _reserved42: [u8; 0x1c],
    cr: CR,
    keyr: KEYR,
}
impl RegisterBlock {
    ///0x00 - HSEM register semaphore 0
    #[inline(always)]
    pub const fn r0(&self) -> &R0 {
        &self.r0
    }
    ///0x04 - HSEM register semaphore 1
    #[inline(always)]
    pub const fn r1(&self) -> &R1 {
        &self.r1
    }
    ///0x08 - HSEM register semaphore 2
    #[inline(always)]
    pub const fn r2(&self) -> &R2 {
        &self.r2
    }
    ///0x0c - HSEM register semaphore 3
    #[inline(always)]
    pub const fn r3(&self) -> &R3 {
        &self.r3
    }
    ///0x10 - HSEM register semaphore 4
    #[inline(always)]
    pub const fn r4(&self) -> &R4 {
        &self.r4
    }
    ///0x14 - HSEM register semaphore 5
    #[inline(always)]
    pub const fn r5(&self) -> &R5 {
        &self.r5
    }
    ///0x18 - HSEM register semaphore 6
    #[inline(always)]
    pub const fn r6(&self) -> &R6 {
        &self.r6
    }
    ///0x1c - HSEM register semaphore 7
    #[inline(always)]
    pub const fn r7(&self) -> &R7 {
        &self.r7
    }
    ///0x20 - HSEM register semaphore 8
    #[inline(always)]
    pub const fn r8(&self) -> &R8 {
        &self.r8
    }
    ///0x24 - HSEM register semaphore 9
    #[inline(always)]
    pub const fn r9(&self) -> &R9 {
        &self.r9
    }
    ///0x28 - HSEM register semaphore 10
    #[inline(always)]
    pub const fn r10(&self) -> &R10 {
        &self.r10
    }
    ///0x2c - HSEM register semaphore 11
    #[inline(always)]
    pub const fn r11(&self) -> &R11 {
        &self.r11
    }
    ///0x30 - HSEM register semaphore 12
    #[inline(always)]
    pub const fn r12(&self) -> &R12 {
        &self.r12
    }
    ///0x34 - HSEM register semaphore 13
    #[inline(always)]
    pub const fn r13(&self) -> &R13 {
        &self.r13
    }
    ///0x38 - HSEM register semaphore 14
    #[inline(always)]
    pub const fn r14(&self) -> &R14 {
        &self.r14
    }
    ///0x3c - HSEM register semaphore 15
    #[inline(always)]
    pub const fn r15(&self) -> &R15 {
        &self.r15
    }
    ///0x80 - HSEM read lock register semaphore 0
    #[inline(always)]
    pub const fn rlr0(&self) -> &RLR0 {
        &self.rlr0
    }
    ///0x84 - HSEM read lock register semaphore 1
    #[inline(always)]
    pub const fn rlr1(&self) -> &RLR1 {
        &self.rlr1
    }
    ///0x88 - HSEM read lock register semaphore 2
    #[inline(always)]
    pub const fn rlr2(&self) -> &RLR2 {
        &self.rlr2
    }
    ///0x8c - HSEM read lock register semaphore 3
    #[inline(always)]
    pub const fn rlr3(&self) -> &RLR3 {
        &self.rlr3
    }
    ///0x90 - HSEM read lock register semaphore 4
    #[inline(always)]
    pub const fn rlr4(&self) -> &RLR4 {
        &self.rlr4
    }
    ///0x94 - HSEM read lock register semaphore 5
    #[inline(always)]
    pub const fn rlr5(&self) -> &RLR5 {
        &self.rlr5
    }
    ///0x98 - HSEM read lock register semaphore 6
    #[inline(always)]
    pub const fn rlr6(&self) -> &RLR6 {
        &self.rlr6
    }
    ///0x9c - HSEM read lock register semaphore 7
    #[inline(always)]
    pub const fn rlr7(&self) -> &RLR7 {
        &self.rlr7
    }
    ///0xa0 - HSEM read lock register semaphore 8
    #[inline(always)]
    pub const fn rlr8(&self) -> &RLR8 {
        &self.rlr8
    }
    ///0xa4 - HSEM read lock register semaphore 9
    #[inline(always)]
    pub const fn rlr9(&self) -> &RLR9 {
        &self.rlr9
    }
    ///0xa8 - HSEM read lock register semaphore 10
    #[inline(always)]
    pub const fn rlr10(&self) -> &RLR10 {
        &self.rlr10
    }
    ///0xac - HSEM read lock register semaphore 11
    #[inline(always)]
    pub const fn rlr11(&self) -> &RLR11 {
        &self.rlr11
    }
    ///0xb0 - HSEM read lock register semaphore 12
    #[inline(always)]
    pub const fn rlr12(&self) -> &RLR12 {
        &self.rlr12
    }
    ///0xb4 - HSEM read lock register semaphore 13
    #[inline(always)]
    pub const fn rlr13(&self) -> &RLR13 {
        &self.rlr13
    }
    ///0xb8 - HSEM read lock register semaphore 14
    #[inline(always)]
    pub const fn rlr14(&self) -> &RLR14 {
        &self.rlr14
    }
    ///0xbc - HSEM read lock register semaphore 15
    #[inline(always)]
    pub const fn rlr15(&self) -> &RLR15 {
        &self.rlr15
    }
    ///0x100 - HSEM non-secure interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x104 - HSEM non-secure interrupt clear register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x108 - HSEM non-secure interrupt status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x10c - HSEM non-secure interrupt status register
    #[inline(always)]
    pub const fn misr(&self) -> &MISR {
        &self.misr
    }
    ///0x180 - HSEM secure interrupt enable register
    #[inline(always)]
    pub const fn sier(&self) -> &SIER {
        &self.sier
    }
    ///0x184 - HSEM secure interrupt clear register
    #[inline(always)]
    pub const fn sicr(&self) -> &SICR {
        &self.sicr
    }
    ///0x188 - HSEM secure interrupt status register
    #[inline(always)]
    pub const fn sisr(&self) -> &SISR {
        &self.sisr
    }
    ///0x18c - HSEM secure masked interrupt status register
    #[inline(always)]
    pub const fn msisr(&self) -> &MSISR {
        &self.msisr
    }
    ///0x200 - HSEM security configuration register
    #[inline(always)]
    pub const fn seccfgr(&self) -> &SECCFGR {
        &self.seccfgr
    }
    ///0x210 - HSEM privilege configuration register
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
    ///0x230 - HSEM clear register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x234 - HSEM interrupt clear register
    #[inline(always)]
    pub const fn keyr(&self) -> &KEYR {
        &self.keyr
    }
}
/**R0 (rw) register accessor: HSEM register semaphore 0

You can [`read`](crate::Reg::read) this register and get [`r0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:R0)

For information about available fields see [`mod@r0`] module*/
pub type R0 = crate::Reg<r0::R0rs>;
///HSEM register semaphore 0
pub mod r0;
/**R1 (rw) register accessor: HSEM register semaphore 1

You can [`read`](crate::Reg::read) this register and get [`r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:R1)

For information about available fields see [`mod@r1`] module*/
pub type R1 = crate::Reg<r1::R1rs>;
///HSEM register semaphore 1
pub mod r1;
/**R2 (rw) register accessor: HSEM register semaphore 2

You can [`read`](crate::Reg::read) this register and get [`r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:R2)

For information about available fields see [`mod@r2`] module*/
pub type R2 = crate::Reg<r2::R2rs>;
///HSEM register semaphore 2
pub mod r2;
/**R3 (rw) register accessor: HSEM register semaphore 3

You can [`read`](crate::Reg::read) this register and get [`r3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:R3)

For information about available fields see [`mod@r3`] module*/
pub type R3 = crate::Reg<r3::R3rs>;
///HSEM register semaphore 3
pub mod r3;
/**R4 (rw) register accessor: HSEM register semaphore 4

You can [`read`](crate::Reg::read) this register and get [`r4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:R4)

For information about available fields see [`mod@r4`] module*/
pub type R4 = crate::Reg<r4::R4rs>;
///HSEM register semaphore 4
pub mod r4;
/**R5 (rw) register accessor: HSEM register semaphore 5

You can [`read`](crate::Reg::read) this register and get [`r5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:R5)

For information about available fields see [`mod@r5`] module*/
pub type R5 = crate::Reg<r5::R5rs>;
///HSEM register semaphore 5
pub mod r5;
/**R6 (rw) register accessor: HSEM register semaphore 6

You can [`read`](crate::Reg::read) this register and get [`r6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:R6)

For information about available fields see [`mod@r6`] module*/
pub type R6 = crate::Reg<r6::R6rs>;
///HSEM register semaphore 6
pub mod r6;
/**R7 (rw) register accessor: HSEM register semaphore 7

You can [`read`](crate::Reg::read) this register and get [`r7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:R7)

For information about available fields see [`mod@r7`] module*/
pub type R7 = crate::Reg<r7::R7rs>;
///HSEM register semaphore 7
pub mod r7;
/**R8 (rw) register accessor: HSEM register semaphore 8

You can [`read`](crate::Reg::read) this register and get [`r8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:R8)

For information about available fields see [`mod@r8`] module*/
pub type R8 = crate::Reg<r8::R8rs>;
///HSEM register semaphore 8
pub mod r8;
/**R9 (rw) register accessor: HSEM register semaphore 9

You can [`read`](crate::Reg::read) this register and get [`r9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:R9)

For information about available fields see [`mod@r9`] module*/
pub type R9 = crate::Reg<r9::R9rs>;
///HSEM register semaphore 9
pub mod r9;
/**R10 (rw) register accessor: HSEM register semaphore 10

You can [`read`](crate::Reg::read) this register and get [`r10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:R10)

For information about available fields see [`mod@r10`] module*/
pub type R10 = crate::Reg<r10::R10rs>;
///HSEM register semaphore 10
pub mod r10;
/**R11 (rw) register accessor: HSEM register semaphore 11

You can [`read`](crate::Reg::read) this register and get [`r11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:R11)

For information about available fields see [`mod@r11`] module*/
pub type R11 = crate::Reg<r11::R11rs>;
///HSEM register semaphore 11
pub mod r11;
/**R12 (rw) register accessor: HSEM register semaphore 12

You can [`read`](crate::Reg::read) this register and get [`r12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:R12)

For information about available fields see [`mod@r12`] module*/
pub type R12 = crate::Reg<r12::R12rs>;
///HSEM register semaphore 12
pub mod r12;
/**R13 (rw) register accessor: HSEM register semaphore 13

You can [`read`](crate::Reg::read) this register and get [`r13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:R13)

For information about available fields see [`mod@r13`] module*/
pub type R13 = crate::Reg<r13::R13rs>;
///HSEM register semaphore 13
pub mod r13;
/**R14 (rw) register accessor: HSEM register semaphore 14

You can [`read`](crate::Reg::read) this register and get [`r14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:R14)

For information about available fields see [`mod@r14`] module*/
pub type R14 = crate::Reg<r14::R14rs>;
///HSEM register semaphore 14
pub mod r14;
/**R15 (rw) register accessor: HSEM register semaphore 15

You can [`read`](crate::Reg::read) this register and get [`r15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:R15)

For information about available fields see [`mod@r15`] module*/
pub type R15 = crate::Reg<r15::R15rs>;
///HSEM register semaphore 15
pub mod r15;
/**RLR0 (r) register accessor: HSEM read lock register semaphore 0

You can [`read`](crate::Reg::read) this register and get [`rlr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:RLR0)

For information about available fields see [`mod@rlr0`] module*/
pub type RLR0 = crate::Reg<rlr0::RLR0rs>;
///HSEM read lock register semaphore 0
pub mod rlr0;
/**RLR1 (r) register accessor: HSEM read lock register semaphore 1

You can [`read`](crate::Reg::read) this register and get [`rlr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:RLR1)

For information about available fields see [`mod@rlr1`] module*/
pub type RLR1 = crate::Reg<rlr1::RLR1rs>;
///HSEM read lock register semaphore 1
pub mod rlr1;
/**RLR2 (r) register accessor: HSEM read lock register semaphore 2

You can [`read`](crate::Reg::read) this register and get [`rlr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:RLR2)

For information about available fields see [`mod@rlr2`] module*/
pub type RLR2 = crate::Reg<rlr2::RLR2rs>;
///HSEM read lock register semaphore 2
pub mod rlr2;
/**RLR3 (r) register accessor: HSEM read lock register semaphore 3

You can [`read`](crate::Reg::read) this register and get [`rlr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:RLR3)

For information about available fields see [`mod@rlr3`] module*/
pub type RLR3 = crate::Reg<rlr3::RLR3rs>;
///HSEM read lock register semaphore 3
pub mod rlr3;
/**RLR4 (r) register accessor: HSEM read lock register semaphore 4

You can [`read`](crate::Reg::read) this register and get [`rlr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:RLR4)

For information about available fields see [`mod@rlr4`] module*/
pub type RLR4 = crate::Reg<rlr4::RLR4rs>;
///HSEM read lock register semaphore 4
pub mod rlr4;
/**RLR5 (r) register accessor: HSEM read lock register semaphore 5

You can [`read`](crate::Reg::read) this register and get [`rlr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:RLR5)

For information about available fields see [`mod@rlr5`] module*/
pub type RLR5 = crate::Reg<rlr5::RLR5rs>;
///HSEM read lock register semaphore 5
pub mod rlr5;
/**RLR6 (r) register accessor: HSEM read lock register semaphore 6

You can [`read`](crate::Reg::read) this register and get [`rlr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:RLR6)

For information about available fields see [`mod@rlr6`] module*/
pub type RLR6 = crate::Reg<rlr6::RLR6rs>;
///HSEM read lock register semaphore 6
pub mod rlr6;
/**RLR7 (r) register accessor: HSEM read lock register semaphore 7

You can [`read`](crate::Reg::read) this register and get [`rlr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:RLR7)

For information about available fields see [`mod@rlr7`] module*/
pub type RLR7 = crate::Reg<rlr7::RLR7rs>;
///HSEM read lock register semaphore 7
pub mod rlr7;
/**RLR8 (r) register accessor: HSEM read lock register semaphore 8

You can [`read`](crate::Reg::read) this register and get [`rlr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:RLR8)

For information about available fields see [`mod@rlr8`] module*/
pub type RLR8 = crate::Reg<rlr8::RLR8rs>;
///HSEM read lock register semaphore 8
pub mod rlr8;
/**RLR9 (r) register accessor: HSEM read lock register semaphore 9

You can [`read`](crate::Reg::read) this register and get [`rlr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:RLR9)

For information about available fields see [`mod@rlr9`] module*/
pub type RLR9 = crate::Reg<rlr9::RLR9rs>;
///HSEM read lock register semaphore 9
pub mod rlr9;
/**RLR10 (r) register accessor: HSEM read lock register semaphore 10

You can [`read`](crate::Reg::read) this register and get [`rlr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:RLR10)

For information about available fields see [`mod@rlr10`] module*/
pub type RLR10 = crate::Reg<rlr10::RLR10rs>;
///HSEM read lock register semaphore 10
pub mod rlr10;
/**RLR11 (r) register accessor: HSEM read lock register semaphore 11

You can [`read`](crate::Reg::read) this register and get [`rlr11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:RLR11)

For information about available fields see [`mod@rlr11`] module*/
pub type RLR11 = crate::Reg<rlr11::RLR11rs>;
///HSEM read lock register semaphore 11
pub mod rlr11;
/**RLR12 (r) register accessor: HSEM read lock register semaphore 12

You can [`read`](crate::Reg::read) this register and get [`rlr12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:RLR12)

For information about available fields see [`mod@rlr12`] module*/
pub type RLR12 = crate::Reg<rlr12::RLR12rs>;
///HSEM read lock register semaphore 12
pub mod rlr12;
/**RLR13 (r) register accessor: HSEM read lock register semaphore 13

You can [`read`](crate::Reg::read) this register and get [`rlr13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:RLR13)

For information about available fields see [`mod@rlr13`] module*/
pub type RLR13 = crate::Reg<rlr13::RLR13rs>;
///HSEM read lock register semaphore 13
pub mod rlr13;
/**RLR14 (r) register accessor: HSEM read lock register semaphore 14

You can [`read`](crate::Reg::read) this register and get [`rlr14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:RLR14)

For information about available fields see [`mod@rlr14`] module*/
pub type RLR14 = crate::Reg<rlr14::RLR14rs>;
///HSEM read lock register semaphore 14
pub mod rlr14;
/**RLR15 (r) register accessor: HSEM read lock register semaphore 15

You can [`read`](crate::Reg::read) this register and get [`rlr15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:RLR15)

For information about available fields see [`mod@rlr15`] module*/
pub type RLR15 = crate::Reg<rlr15::RLR15rs>;
///HSEM read lock register semaphore 15
pub mod rlr15;
/**IER (rw) register accessor: HSEM non-secure interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///HSEM non-secure interrupt enable register
pub mod ier;
/**ICR (rw) register accessor: HSEM non-secure interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///HSEM non-secure interrupt clear register
pub mod icr;
/**ISR (r) register accessor: HSEM non-secure interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///HSEM non-secure interrupt status register
pub mod isr;
/**MISR (r) register accessor: HSEM non-secure interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:MISR)

For information about available fields see [`mod@misr`] module*/
pub type MISR = crate::Reg<misr::MISRrs>;
///HSEM non-secure interrupt status register
pub mod misr;
/**SIER (rw) register accessor: HSEM secure interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`sier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:SIER)

For information about available fields see [`mod@sier`] module*/
pub type SIER = crate::Reg<sier::SIERrs>;
///HSEM secure interrupt enable register
pub mod sier;
/**SICR (rw) register accessor: HSEM secure interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`sicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:SICR)

For information about available fields see [`mod@sicr`] module*/
pub type SICR = crate::Reg<sicr::SICRrs>;
///HSEM secure interrupt clear register
pub mod sicr;
/**SISR (r) register accessor: HSEM secure interrupt status register

You can [`read`](crate::Reg::read) this register and get [`sisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:SISR)

For information about available fields see [`mod@sisr`] module*/
pub type SISR = crate::Reg<sisr::SISRrs>;
///HSEM secure interrupt status register
pub mod sisr;
/**MSISR (r) register accessor: HSEM secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`msisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:MSISR)

For information about available fields see [`mod@msisr`] module*/
pub type MSISR = crate::Reg<msisr::MSISRrs>;
///HSEM secure masked interrupt status register
pub mod msisr;
/**SECCFGR (rw) register accessor: HSEM security configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:SECCFGR)

For information about available fields see [`mod@seccfgr`] module*/
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
///HSEM security configuration register
pub mod seccfgr;
/**PRIVCFGR (rw) register accessor: HSEM privilege configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:PRIVCFGR)

For information about available fields see [`mod@privcfgr`] module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///HSEM privilege configuration register
pub mod privcfgr;
/**CR (w) register accessor: HSEM clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///HSEM clear register
pub mod cr;
/**KEYR (rw) register accessor: HSEM interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`keyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#HSEM:KEYR)

For information about available fields see [`mod@keyr`] module*/
pub type KEYR = crate::Reg<keyr::KEYRrs>;
///HSEM interrupt clear register
pub mod keyr;
