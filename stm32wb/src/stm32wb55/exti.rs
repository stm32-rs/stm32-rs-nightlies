#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    rtsr1: RTSR1,
    ftsr1: FTSR1,
    swier1: SWIER1,
    pr1: PR1,
    _reserved4: [u8; 0x10],
    rtsr2: RTSR2,
    ftsr2: FTSR2,
    swier2: SWIER2,
    pr2: PR2,
    _reserved8: [u8; 0x50],
    imr1: IMR1,
    emr1: EMR1,
    _reserved10: [u8; 0x08],
    imr2: IMR2,
    emr2: EMR2,
    _reserved12: [u8; 0x28],
    c2imr1: C2IMR1,
    c2emr1: C2EMR1,
    _reserved14: [u8; 0x08],
    c2imr2: C2IMR2,
    c2emr2: C2EMR2,
    _reserved16: [u8; 0x0300],
    hwcfgr7: HWCFGR7,
    hwcfgr6: HWCFGR6,
    hwcfgr5: HWCFGR5,
    hwcfgr4: HWCFGR4,
    hwcfgr3: HWCFGR3,
    hwcfgr2: HWCFGR2,
    hwcfgr1: HWCFGR1,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00 - rising trigger selection register
    #[inline(always)]
    pub const fn rtsr1(&self) -> &RTSR1 {
        &self.rtsr1
    }
    ///0x04 - falling trigger selection register
    #[inline(always)]
    pub const fn ftsr1(&self) -> &FTSR1 {
        &self.ftsr1
    }
    ///0x08 - software interrupt event register
    #[inline(always)]
    pub const fn swier1(&self) -> &SWIER1 {
        &self.swier1
    }
    ///0x0c - EXTI pending register
    #[inline(always)]
    pub const fn pr1(&self) -> &PR1 {
        &self.pr1
    }
    ///0x20 - rising trigger selection register
    #[inline(always)]
    pub const fn rtsr2(&self) -> &RTSR2 {
        &self.rtsr2
    }
    ///0x24 - falling trigger selection register
    #[inline(always)]
    pub const fn ftsr2(&self) -> &FTSR2 {
        &self.ftsr2
    }
    ///0x28 - software interrupt event register
    #[inline(always)]
    pub const fn swier2(&self) -> &SWIER2 {
        &self.swier2
    }
    ///0x2c - pending register
    #[inline(always)]
    pub const fn pr2(&self) -> &PR2 {
        &self.pr2
    }
    ///0x80 - CPUm wakeup with interrupt mask register
    #[inline(always)]
    pub const fn imr1(&self) -> &IMR1 {
        &self.imr1
    }
    ///0x84 - CPUm wakeup with event mask register
    #[inline(always)]
    pub const fn emr1(&self) -> &EMR1 {
        &self.emr1
    }
    ///0x90 - CPUm wakeup with interrupt mask register
    #[inline(always)]
    pub const fn imr2(&self) -> &IMR2 {
        &self.imr2
    }
    ///0x94 - CPUm wakeup with event mask register
    #[inline(always)]
    pub const fn emr2(&self) -> &EMR2 {
        &self.emr2
    }
    ///0xc0 - CPUm wakeup with interrupt mask register
    #[inline(always)]
    pub const fn c2imr1(&self) -> &C2IMR1 {
        &self.c2imr1
    }
    ///0xc4 - CPUm wakeup with event mask register
    #[inline(always)]
    pub const fn c2emr1(&self) -> &C2EMR1 {
        &self.c2emr1
    }
    ///0xd0 - CPUm wakeup with interrupt mask register
    #[inline(always)]
    pub const fn c2imr2(&self) -> &C2IMR2 {
        &self.c2imr2
    }
    ///0xd4 - CPUm wakeup with event mask register
    #[inline(always)]
    pub const fn c2emr2(&self) -> &C2EMR2 {
        &self.c2emr2
    }
    ///0x3d8 - EXTI Hardware configuration registers
    #[inline(always)]
    pub const fn hwcfgr7(&self) -> &HWCFGR7 {
        &self.hwcfgr7
    }
    ///0x3dc - Hardware configuration registers
    #[inline(always)]
    pub const fn hwcfgr6(&self) -> &HWCFGR6 {
        &self.hwcfgr6
    }
    ///0x3e0 - Hardware configuration registers
    #[inline(always)]
    pub const fn hwcfgr5(&self) -> &HWCFGR5 {
        &self.hwcfgr5
    }
    ///0x3e4 - Hardware configuration registers
    #[inline(always)]
    pub const fn hwcfgr4(&self) -> &HWCFGR4 {
        &self.hwcfgr4
    }
    ///0x3e8 - Hardware configuration registers
    #[inline(always)]
    pub const fn hwcfgr3(&self) -> &HWCFGR3 {
        &self.hwcfgr3
    }
    ///0x3ec - Hardware configuration registers
    #[inline(always)]
    pub const fn hwcfgr2(&self) -> &HWCFGR2 {
        &self.hwcfgr2
    }
    ///0x3f0 - Hardware configuration register 1
    #[inline(always)]
    pub const fn hwcfgr1(&self) -> &HWCFGR1 {
        &self.hwcfgr1
    }
    ///0x3f4 - EXTI IP Version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - Identification register
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0x3fc - Size ID register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**RTSR1 (rw) register accessor: rising trigger selection register

You can [`read`](crate::Reg::read) this register and get [`rtsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:RTSR1)

For information about available fields see [`mod@rtsr1`] module*/
pub type RTSR1 = crate::Reg<rtsr1::RTSR1rs>;
///rising trigger selection register
pub mod rtsr1;
/**FTSR1 (rw) register accessor: falling trigger selection register

You can [`read`](crate::Reg::read) this register and get [`ftsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:FTSR1)

For information about available fields see [`mod@ftsr1`] module*/
pub type FTSR1 = crate::Reg<ftsr1::FTSR1rs>;
///falling trigger selection register
pub mod ftsr1;
/**SWIER1 (rw) register accessor: software interrupt event register

You can [`read`](crate::Reg::read) this register and get [`swier1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:SWIER1)

For information about available fields see [`mod@swier1`] module*/
pub type SWIER1 = crate::Reg<swier1::SWIER1rs>;
///software interrupt event register
pub mod swier1;
/**PR1 (rw) register accessor: EXTI pending register

You can [`read`](crate::Reg::read) this register and get [`pr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:PR1)

For information about available fields see [`mod@pr1`] module*/
pub type PR1 = crate::Reg<pr1::PR1rs>;
///EXTI pending register
pub mod pr1;
/**RTSR2 (rw) register accessor: rising trigger selection register

You can [`read`](crate::Reg::read) this register and get [`rtsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:RTSR2)

For information about available fields see [`mod@rtsr2`] module*/
pub type RTSR2 = crate::Reg<rtsr2::RTSR2rs>;
///rising trigger selection register
pub mod rtsr2;
/**FTSR2 (rw) register accessor: falling trigger selection register

You can [`read`](crate::Reg::read) this register and get [`ftsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:FTSR2)

For information about available fields see [`mod@ftsr2`] module*/
pub type FTSR2 = crate::Reg<ftsr2::FTSR2rs>;
///falling trigger selection register
pub mod ftsr2;
/**SWIER2 (rw) register accessor: software interrupt event register

You can [`read`](crate::Reg::read) this register and get [`swier2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:SWIER2)

For information about available fields see [`mod@swier2`] module*/
pub type SWIER2 = crate::Reg<swier2::SWIER2rs>;
///software interrupt event register
pub mod swier2;
/**PR2 (rw) register accessor: pending register

You can [`read`](crate::Reg::read) this register and get [`pr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:PR2)

For information about available fields see [`mod@pr2`] module*/
pub type PR2 = crate::Reg<pr2::PR2rs>;
///pending register
pub mod pr2;
/**IMR1 (rw) register accessor: CPUm wakeup with interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`imr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:IMR1)

For information about available fields see [`mod@imr1`] module*/
pub type IMR1 = crate::Reg<imr1::IMR1rs>;
///CPUm wakeup with interrupt mask register
pub mod imr1;
/**C2IMR1 (rw) register accessor: CPUm wakeup with interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`c2imr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2imr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:C2IMR1)

For information about available fields see [`mod@c2imr1`] module*/
pub type C2IMR1 = crate::Reg<c2imr1::C2IMR1rs>;
///CPUm wakeup with interrupt mask register
pub mod c2imr1;
/**EMR1 (rw) register accessor: CPUm wakeup with event mask register

You can [`read`](crate::Reg::read) this register and get [`emr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:EMR1)

For information about available fields see [`mod@emr1`] module*/
pub type EMR1 = crate::Reg<emr1::EMR1rs>;
///CPUm wakeup with event mask register
pub mod emr1;
/**C2EMR1 (rw) register accessor: CPUm wakeup with event mask register

You can [`read`](crate::Reg::read) this register and get [`c2emr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2emr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:C2EMR1)

For information about available fields see [`mod@c2emr1`] module*/
pub type C2EMR1 = crate::Reg<c2emr1::C2EMR1rs>;
///CPUm wakeup with event mask register
pub mod c2emr1;
/**IMR2 (rw) register accessor: CPUm wakeup with interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`imr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:IMR2)

For information about available fields see [`mod@imr2`] module*/
pub type IMR2 = crate::Reg<imr2::IMR2rs>;
///CPUm wakeup with interrupt mask register
pub mod imr2;
/**C2IMR2 (rw) register accessor: CPUm wakeup with interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`c2imr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2imr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:C2IMR2)

For information about available fields see [`mod@c2imr2`] module*/
pub type C2IMR2 = crate::Reg<c2imr2::C2IMR2rs>;
///CPUm wakeup with interrupt mask register
pub mod c2imr2;
/**EMR2 (rw) register accessor: CPUm wakeup with event mask register

You can [`read`](crate::Reg::read) this register and get [`emr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:EMR2)

For information about available fields see [`mod@emr2`] module*/
pub type EMR2 = crate::Reg<emr2::EMR2rs>;
///CPUm wakeup with event mask register
pub mod emr2;
/**C2EMR2 (rw) register accessor: CPUm wakeup with event mask register

You can [`read`](crate::Reg::read) this register and get [`c2emr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2emr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:C2EMR2)

For information about available fields see [`mod@c2emr2`] module*/
pub type C2EMR2 = crate::Reg<c2emr2::C2EMR2rs>;
///CPUm wakeup with event mask register
pub mod c2emr2;
/**HWCFGR5 (r) register accessor: Hardware configuration registers

You can [`read`](crate::Reg::read) this register and get [`hwcfgr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:HWCFGR5)

For information about available fields see [`mod@hwcfgr5`] module*/
pub type HWCFGR5 = crate::Reg<hwcfgr5::HWCFGR5rs>;
///Hardware configuration registers
pub mod hwcfgr5;
/**HWCFGR6 (r) register accessor: Hardware configuration registers

You can [`read`](crate::Reg::read) this register and get [`hwcfgr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:HWCFGR6)

For information about available fields see [`mod@hwcfgr6`] module*/
pub type HWCFGR6 = crate::Reg<hwcfgr6::HWCFGR6rs>;
///Hardware configuration registers
pub mod hwcfgr6;
/**HWCFGR7 (r) register accessor: EXTI Hardware configuration registers

You can [`read`](crate::Reg::read) this register and get [`hwcfgr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:HWCFGR7)

For information about available fields see [`mod@hwcfgr7`] module*/
pub type HWCFGR7 = crate::Reg<hwcfgr7::HWCFGR7rs>;
///EXTI Hardware configuration registers
pub mod hwcfgr7;
/**HWCFGR2 (r) register accessor: Hardware configuration registers

You can [`read`](crate::Reg::read) this register and get [`hwcfgr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:HWCFGR2)

For information about available fields see [`mod@hwcfgr2`] module*/
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2rs>;
///Hardware configuration registers
pub mod hwcfgr2;
/**HWCFGR3 (r) register accessor: Hardware configuration registers

You can [`read`](crate::Reg::read) this register and get [`hwcfgr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:HWCFGR3)

For information about available fields see [`mod@hwcfgr3`] module*/
pub type HWCFGR3 = crate::Reg<hwcfgr3::HWCFGR3rs>;
///Hardware configuration registers
pub mod hwcfgr3;
/**HWCFGR4 (r) register accessor: Hardware configuration registers

You can [`read`](crate::Reg::read) this register and get [`hwcfgr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:HWCFGR4)

For information about available fields see [`mod@hwcfgr4`] module*/
pub type HWCFGR4 = crate::Reg<hwcfgr4::HWCFGR4rs>;
///Hardware configuration registers
pub mod hwcfgr4;
/**HWCFGR1 (r) register accessor: Hardware configuration register 1

You can [`read`](crate::Reg::read) this register and get [`hwcfgr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:HWCFGR1)

For information about available fields see [`mod@hwcfgr1`] module*/
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1rs>;
///Hardware configuration register 1
pub mod hwcfgr1;
/**VERR (r) register accessor: EXTI IP Version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///EXTI IP Version register
pub mod verr;
/**IPIDR (r) register accessor: Identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///Identification register
pub mod ipidr;
/**SIDR (r) register accessor: Size ID register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///Size ID register
pub mod sidr;
