#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    risc_cr: RISC_CR,
    _reserved1: [u8; 0x0c],
    risc_seccfgr0: RISC_SECCFGR0,
    risc_seccfgr1: RISC_SECCFGR1,
    risc_seccfgr2: RISC_SECCFGR2,
    risc_seccfgr3: RISC_SECCFGR3,
    risc_seccfgr4: RISC_SECCFGR4,
    risc_seccfgr5: RISC_SECCFGR5,
    _reserved7: [u8; 0x08],
    risc_privcfgr0: RISC_PRIVCFGR0,
    risc_privcfgr1: RISC_PRIVCFGR1,
    risc_privcfgr2: RISC_PRIVCFGR2,
    risc_privcfgr3: RISC_PRIVCFGR3,
    risc_privcfgr4: RISC_PRIVCFGR4,
    risc_privcfgr5: RISC_PRIVCFGR5,
    _reserved13: [u8; 0x08],
    risc_rcfglockr0: RISC_RCFGLOCKR0,
    risc_rcfglockr1: RISC_RCFGLOCKR1,
    risc_rcfglockr2: RISC_RCFGLOCKR2,
    risc_rcfglockr3: RISC_RCFGLOCKR3,
    risc_rcfglockr4: RISC_RCFGLOCKR4,
    risc_rcfglockr5: RISC_RCFGLOCKR5,
    _reserved19: [u8; 0x0b98],
    rimc_cr: RIMC_CR,
    _reserved20: [u8; 0x0c],
    rimc_attr0: RIMC_ATTR0,
    rimc_attr1: RIMC_ATTR1,
    rimc_attr2: RIMC_ATTR2,
    rimc_attr3: RIMC_ATTR3,
    rimc_attr4: RIMC_ATTR4,
    rimc_attr5: RIMC_ATTR5,
    rimc_attr6: RIMC_ATTR6,
    rimc_attr7: RIMC_ATTR7,
    rimc_attr8: RIMC_ATTR8,
    rimc_attr9: RIMC_ATTR9,
    rimc_attr10: RIMC_ATTR10,
    rimc_attr11: RIMC_ATTR11,
    _reserved32: [u8; 0x0370],
    ppsr0: PPSR0,
    ppsr1: PPSR1,
    ppsr2: PPSR2,
    ppsr3: PPSR3,
    ppsr4: PPSR4,
    ppsr5: PPSR5,
}
impl RegisterBlock {
    ///0x00 - RIFSC RISC slave configuration register x
    #[inline(always)]
    pub const fn risc_cr(&self) -> &RISC_CR {
        &self.risc_cr
    }
    ///0x10 - RIFSC RISC slave security configuration register 0
    #[inline(always)]
    pub const fn risc_seccfgr0(&self) -> &RISC_SECCFGR0 {
        &self.risc_seccfgr0
    }
    ///0x14 - RIFSC RISC slave security configuration register 1
    #[inline(always)]
    pub const fn risc_seccfgr1(&self) -> &RISC_SECCFGR1 {
        &self.risc_seccfgr1
    }
    ///0x18 - RIFSC RISC slave security configuration register 2
    #[inline(always)]
    pub const fn risc_seccfgr2(&self) -> &RISC_SECCFGR2 {
        &self.risc_seccfgr2
    }
    ///0x1c - RIFSC RISC slave security configuration register 3
    #[inline(always)]
    pub const fn risc_seccfgr3(&self) -> &RISC_SECCFGR3 {
        &self.risc_seccfgr3
    }
    ///0x20 - RIFSC RISC slave security configuration register 4
    #[inline(always)]
    pub const fn risc_seccfgr4(&self) -> &RISC_SECCFGR4 {
        &self.risc_seccfgr4
    }
    ///0x24 - RIFSC RISC slave security configuration register 5
    #[inline(always)]
    pub const fn risc_seccfgr5(&self) -> &RISC_SECCFGR5 {
        &self.risc_seccfgr5
    }
    ///0x30 - RIFSC RISFC slave privileged register 0
    #[inline(always)]
    pub const fn risc_privcfgr0(&self) -> &RISC_PRIVCFGR0 {
        &self.risc_privcfgr0
    }
    ///0x34 - RIFSC RISFC slave privileged register 1
    #[inline(always)]
    pub const fn risc_privcfgr1(&self) -> &RISC_PRIVCFGR1 {
        &self.risc_privcfgr1
    }
    ///0x38 - RIFSC RISFC slave privileged register 2
    #[inline(always)]
    pub const fn risc_privcfgr2(&self) -> &RISC_PRIVCFGR2 {
        &self.risc_privcfgr2
    }
    ///0x3c - RIFSC RISFC slave privileged register 3
    #[inline(always)]
    pub const fn risc_privcfgr3(&self) -> &RISC_PRIVCFGR3 {
        &self.risc_privcfgr3
    }
    ///0x40 - RIFSC RISFC slave privileged register 4
    #[inline(always)]
    pub const fn risc_privcfgr4(&self) -> &RISC_PRIVCFGR4 {
        &self.risc_privcfgr4
    }
    ///0x44 - RIFSC RISFC slave privileged register 5
    #[inline(always)]
    pub const fn risc_privcfgr5(&self) -> &RISC_PRIVCFGR5 {
        &self.risc_privcfgr5
    }
    ///0x50 - RIFSC RISC slave resource configuration lock register 0
    #[inline(always)]
    pub const fn risc_rcfglockr0(&self) -> &RISC_RCFGLOCKR0 {
        &self.risc_rcfglockr0
    }
    ///0x54 - RIFSC RISC slave resource configuration lock register 1
    #[inline(always)]
    pub const fn risc_rcfglockr1(&self) -> &RISC_RCFGLOCKR1 {
        &self.risc_rcfglockr1
    }
    ///0x58 - RIFSC RISC slave resource configuration lock register 2
    #[inline(always)]
    pub const fn risc_rcfglockr2(&self) -> &RISC_RCFGLOCKR2 {
        &self.risc_rcfglockr2
    }
    ///0x5c - RIFSC RISC slave resource configuration lock register 3
    #[inline(always)]
    pub const fn risc_rcfglockr3(&self) -> &RISC_RCFGLOCKR3 {
        &self.risc_rcfglockr3
    }
    ///0x60 - RIFSC RISC slave resource configuration lock register 4
    #[inline(always)]
    pub const fn risc_rcfglockr4(&self) -> &RISC_RCFGLOCKR4 {
        &self.risc_rcfglockr4
    }
    ///0x64 - RIFSC RISC slave resource configuration lock register 5
    #[inline(always)]
    pub const fn risc_rcfglockr5(&self) -> &RISC_RCFGLOCKR5 {
        &self.risc_rcfglockr5
    }
    ///0xc00 - RIFSC RIMC master configuration register
    #[inline(always)]
    pub const fn rimc_cr(&self) -> &RIMC_CR {
        &self.rimc_cr
    }
    ///0xc10 - RIFSC RIMC master attribute register 0
    #[inline(always)]
    pub const fn rimc_attr0(&self) -> &RIMC_ATTR0 {
        &self.rimc_attr0
    }
    ///0xc14 - RIFSC RIMC master attribute register 1
    #[inline(always)]
    pub const fn rimc_attr1(&self) -> &RIMC_ATTR1 {
        &self.rimc_attr1
    }
    ///0xc18 - RIFSC RIMC master attribute register 2
    #[inline(always)]
    pub const fn rimc_attr2(&self) -> &RIMC_ATTR2 {
        &self.rimc_attr2
    }
    ///0xc1c - RIFSC RIMC master attribute register 3
    #[inline(always)]
    pub const fn rimc_attr3(&self) -> &RIMC_ATTR3 {
        &self.rimc_attr3
    }
    ///0xc20 - RIFSC RIMC master attribute register 4
    #[inline(always)]
    pub const fn rimc_attr4(&self) -> &RIMC_ATTR4 {
        &self.rimc_attr4
    }
    ///0xc24 - RIFSC RIMC master attribute register 5
    #[inline(always)]
    pub const fn rimc_attr5(&self) -> &RIMC_ATTR5 {
        &self.rimc_attr5
    }
    ///0xc28 - RIFSC RIMC master attribute register 6
    #[inline(always)]
    pub const fn rimc_attr6(&self) -> &RIMC_ATTR6 {
        &self.rimc_attr6
    }
    ///0xc2c - RIFSC RIMC master attribute register 7
    #[inline(always)]
    pub const fn rimc_attr7(&self) -> &RIMC_ATTR7 {
        &self.rimc_attr7
    }
    ///0xc30 - RIFSC RIMC master attribute register 8
    #[inline(always)]
    pub const fn rimc_attr8(&self) -> &RIMC_ATTR8 {
        &self.rimc_attr8
    }
    ///0xc34 - RIFSC RIMC master attribute register 9
    #[inline(always)]
    pub const fn rimc_attr9(&self) -> &RIMC_ATTR9 {
        &self.rimc_attr9
    }
    ///0xc38 - RIFSC RIMC master attribute register 10
    #[inline(always)]
    pub const fn rimc_attr10(&self) -> &RIMC_ATTR10 {
        &self.rimc_attr10
    }
    ///0xc3c - RIFSC RIMC master attribute register 11
    #[inline(always)]
    pub const fn rimc_attr11(&self) -> &RIMC_ATTR11 {
        &self.rimc_attr11
    }
    ///0xfb0 - RIFSC peripheral protection status register 0
    #[inline(always)]
    pub const fn ppsr0(&self) -> &PPSR0 {
        &self.ppsr0
    }
    ///0xfb4 - RIFSC peripheral protection status register 1
    #[inline(always)]
    pub const fn ppsr1(&self) -> &PPSR1 {
        &self.ppsr1
    }
    ///0xfb8 - RIFSC peripheral protection status register 2
    #[inline(always)]
    pub const fn ppsr2(&self) -> &PPSR2 {
        &self.ppsr2
    }
    ///0xfbc - RIFSC peripheral protection status register 3
    #[inline(always)]
    pub const fn ppsr3(&self) -> &PPSR3 {
        &self.ppsr3
    }
    ///0xfc0 - RIFSC peripheral protection status register 4
    #[inline(always)]
    pub const fn ppsr4(&self) -> &PPSR4 {
        &self.ppsr4
    }
    ///0xfc4 - RIFSC peripheral protection status register 5
    #[inline(always)]
    pub const fn ppsr5(&self) -> &PPSR5 {
        &self.ppsr5
    }
}
/**RISC_CR (rw) register accessor: RIFSC RISC slave configuration register x

You can [`read`](crate::Reg::read) this register and get [`risc_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`risc_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RISC_CR)

For information about available fields see [`mod@risc_cr`] module*/
pub type RISC_CR = crate::Reg<risc_cr::RISC_CRrs>;
///RIFSC RISC slave configuration register x
pub mod risc_cr;
/**RISC_SECCFGR0 (rw) register accessor: RIFSC RISC slave security configuration register 0

You can [`read`](crate::Reg::read) this register and get [`risc_seccfgr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`risc_seccfgr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RISC_SECCFGR0)

For information about available fields see [`mod@risc_seccfgr0`] module*/
pub type RISC_SECCFGR0 = crate::Reg<risc_seccfgr0::RISC_SECCFGR0rs>;
///RIFSC RISC slave security configuration register 0
pub mod risc_seccfgr0;
/**RISC_SECCFGR1 (rw) register accessor: RIFSC RISC slave security configuration register 1

You can [`read`](crate::Reg::read) this register and get [`risc_seccfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`risc_seccfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RISC_SECCFGR1)

For information about available fields see [`mod@risc_seccfgr1`] module*/
pub type RISC_SECCFGR1 = crate::Reg<risc_seccfgr1::RISC_SECCFGR1rs>;
///RIFSC RISC slave security configuration register 1
pub mod risc_seccfgr1;
/**RISC_SECCFGR2 (rw) register accessor: RIFSC RISC slave security configuration register 2

You can [`read`](crate::Reg::read) this register and get [`risc_seccfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`risc_seccfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RISC_SECCFGR2)

For information about available fields see [`mod@risc_seccfgr2`] module*/
pub type RISC_SECCFGR2 = crate::Reg<risc_seccfgr2::RISC_SECCFGR2rs>;
///RIFSC RISC slave security configuration register 2
pub mod risc_seccfgr2;
/**RISC_SECCFGR3 (rw) register accessor: RIFSC RISC slave security configuration register 3

You can [`read`](crate::Reg::read) this register and get [`risc_seccfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`risc_seccfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RISC_SECCFGR3)

For information about available fields see [`mod@risc_seccfgr3`] module*/
pub type RISC_SECCFGR3 = crate::Reg<risc_seccfgr3::RISC_SECCFGR3rs>;
///RIFSC RISC slave security configuration register 3
pub mod risc_seccfgr3;
/**RISC_SECCFGR4 (rw) register accessor: RIFSC RISC slave security configuration register 4

You can [`read`](crate::Reg::read) this register and get [`risc_seccfgr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`risc_seccfgr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RISC_SECCFGR4)

For information about available fields see [`mod@risc_seccfgr4`] module*/
pub type RISC_SECCFGR4 = crate::Reg<risc_seccfgr4::RISC_SECCFGR4rs>;
///RIFSC RISC slave security configuration register 4
pub mod risc_seccfgr4;
/**RISC_SECCFGR5 (rw) register accessor: RIFSC RISC slave security configuration register 5

You can [`read`](crate::Reg::read) this register and get [`risc_seccfgr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`risc_seccfgr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RISC_SECCFGR5)

For information about available fields see [`mod@risc_seccfgr5`] module*/
pub type RISC_SECCFGR5 = crate::Reg<risc_seccfgr5::RISC_SECCFGR5rs>;
///RIFSC RISC slave security configuration register 5
pub mod risc_seccfgr5;
/**RISC_PRIVCFGR0 (rw) register accessor: RIFSC RISFC slave privileged register 0

You can [`read`](crate::Reg::read) this register and get [`risc_privcfgr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`risc_privcfgr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RISC_PRIVCFGR0)

For information about available fields see [`mod@risc_privcfgr0`] module*/
pub type RISC_PRIVCFGR0 = crate::Reg<risc_privcfgr0::RISC_PRIVCFGR0rs>;
///RIFSC RISFC slave privileged register 0
pub mod risc_privcfgr0;
/**RISC_PRIVCFGR1 (rw) register accessor: RIFSC RISFC slave privileged register 1

You can [`read`](crate::Reg::read) this register and get [`risc_privcfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`risc_privcfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RISC_PRIVCFGR1)

For information about available fields see [`mod@risc_privcfgr1`] module*/
pub type RISC_PRIVCFGR1 = crate::Reg<risc_privcfgr1::RISC_PRIVCFGR1rs>;
///RIFSC RISFC slave privileged register 1
pub mod risc_privcfgr1;
/**RISC_PRIVCFGR2 (rw) register accessor: RIFSC RISFC slave privileged register 2

You can [`read`](crate::Reg::read) this register and get [`risc_privcfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`risc_privcfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RISC_PRIVCFGR2)

For information about available fields see [`mod@risc_privcfgr2`] module*/
pub type RISC_PRIVCFGR2 = crate::Reg<risc_privcfgr2::RISC_PRIVCFGR2rs>;
///RIFSC RISFC slave privileged register 2
pub mod risc_privcfgr2;
/**RISC_PRIVCFGR3 (rw) register accessor: RIFSC RISFC slave privileged register 3

You can [`read`](crate::Reg::read) this register and get [`risc_privcfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`risc_privcfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RISC_PRIVCFGR3)

For information about available fields see [`mod@risc_privcfgr3`] module*/
pub type RISC_PRIVCFGR3 = crate::Reg<risc_privcfgr3::RISC_PRIVCFGR3rs>;
///RIFSC RISFC slave privileged register 3
pub mod risc_privcfgr3;
/**RISC_PRIVCFGR4 (rw) register accessor: RIFSC RISFC slave privileged register 4

You can [`read`](crate::Reg::read) this register and get [`risc_privcfgr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`risc_privcfgr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RISC_PRIVCFGR4)

For information about available fields see [`mod@risc_privcfgr4`] module*/
pub type RISC_PRIVCFGR4 = crate::Reg<risc_privcfgr4::RISC_PRIVCFGR4rs>;
///RIFSC RISFC slave privileged register 4
pub mod risc_privcfgr4;
/**RISC_PRIVCFGR5 (rw) register accessor: RIFSC RISFC slave privileged register 5

You can [`read`](crate::Reg::read) this register and get [`risc_privcfgr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`risc_privcfgr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RISC_PRIVCFGR5)

For information about available fields see [`mod@risc_privcfgr5`] module*/
pub type RISC_PRIVCFGR5 = crate::Reg<risc_privcfgr5::RISC_PRIVCFGR5rs>;
///RIFSC RISFC slave privileged register 5
pub mod risc_privcfgr5;
/**RISC_RCFGLOCKR0 (rw) register accessor: RIFSC RISC slave resource configuration lock register 0

You can [`read`](crate::Reg::read) this register and get [`risc_rcfglockr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`risc_rcfglockr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RISC_RCFGLOCKR0)

For information about available fields see [`mod@risc_rcfglockr0`] module*/
pub type RISC_RCFGLOCKR0 = crate::Reg<risc_rcfglockr0::RISC_RCFGLOCKR0rs>;
///RIFSC RISC slave resource configuration lock register 0
pub mod risc_rcfglockr0;
/**RISC_RCFGLOCKR1 (rw) register accessor: RIFSC RISC slave resource configuration lock register 1

You can [`read`](crate::Reg::read) this register and get [`risc_rcfglockr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`risc_rcfglockr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RISC_RCFGLOCKR1)

For information about available fields see [`mod@risc_rcfglockr1`] module*/
pub type RISC_RCFGLOCKR1 = crate::Reg<risc_rcfglockr1::RISC_RCFGLOCKR1rs>;
///RIFSC RISC slave resource configuration lock register 1
pub mod risc_rcfglockr1;
/**RISC_RCFGLOCKR2 (rw) register accessor: RIFSC RISC slave resource configuration lock register 2

You can [`read`](crate::Reg::read) this register and get [`risc_rcfglockr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`risc_rcfglockr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RISC_RCFGLOCKR2)

For information about available fields see [`mod@risc_rcfglockr2`] module*/
pub type RISC_RCFGLOCKR2 = crate::Reg<risc_rcfglockr2::RISC_RCFGLOCKR2rs>;
///RIFSC RISC slave resource configuration lock register 2
pub mod risc_rcfglockr2;
/**RISC_RCFGLOCKR3 (rw) register accessor: RIFSC RISC slave resource configuration lock register 3

You can [`read`](crate::Reg::read) this register and get [`risc_rcfglockr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`risc_rcfglockr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RISC_RCFGLOCKR3)

For information about available fields see [`mod@risc_rcfglockr3`] module*/
pub type RISC_RCFGLOCKR3 = crate::Reg<risc_rcfglockr3::RISC_RCFGLOCKR3rs>;
///RIFSC RISC slave resource configuration lock register 3
pub mod risc_rcfglockr3;
/**RISC_RCFGLOCKR4 (rw) register accessor: RIFSC RISC slave resource configuration lock register 4

You can [`read`](crate::Reg::read) this register and get [`risc_rcfglockr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`risc_rcfglockr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RISC_RCFGLOCKR4)

For information about available fields see [`mod@risc_rcfglockr4`] module*/
pub type RISC_RCFGLOCKR4 = crate::Reg<risc_rcfglockr4::RISC_RCFGLOCKR4rs>;
///RIFSC RISC slave resource configuration lock register 4
pub mod risc_rcfglockr4;
/**RISC_RCFGLOCKR5 (rw) register accessor: RIFSC RISC slave resource configuration lock register 5

You can [`read`](crate::Reg::read) this register and get [`risc_rcfglockr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`risc_rcfglockr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RISC_RCFGLOCKR5)

For information about available fields see [`mod@risc_rcfglockr5`] module*/
pub type RISC_RCFGLOCKR5 = crate::Reg<risc_rcfglockr5::RISC_RCFGLOCKR5rs>;
///RIFSC RISC slave resource configuration lock register 5
pub mod risc_rcfglockr5;
/**RIMC_CR (rw) register accessor: RIFSC RIMC master configuration register

You can [`read`](crate::Reg::read) this register and get [`rimc_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rimc_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RIMC_CR)

For information about available fields see [`mod@rimc_cr`] module*/
pub type RIMC_CR = crate::Reg<rimc_cr::RIMC_CRrs>;
///RIFSC RIMC master configuration register
pub mod rimc_cr;
/**RIMC_ATTR0 (rw) register accessor: RIFSC RIMC master attribute register 0

You can [`read`](crate::Reg::read) this register and get [`rimc_attr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rimc_attr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RIMC_ATTR0)

For information about available fields see [`mod@rimc_attr0`] module*/
pub type RIMC_ATTR0 = crate::Reg<rimc_attr0::RIMC_ATTR0rs>;
///RIFSC RIMC master attribute register 0
pub mod rimc_attr0;
/**RIMC_ATTR1 (rw) register accessor: RIFSC RIMC master attribute register 1

You can [`read`](crate::Reg::read) this register and get [`rimc_attr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rimc_attr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RIMC_ATTR1)

For information about available fields see [`mod@rimc_attr1`] module*/
pub type RIMC_ATTR1 = crate::Reg<rimc_attr1::RIMC_ATTR1rs>;
///RIFSC RIMC master attribute register 1
pub mod rimc_attr1;
/**RIMC_ATTR2 (rw) register accessor: RIFSC RIMC master attribute register 2

You can [`read`](crate::Reg::read) this register and get [`rimc_attr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rimc_attr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RIMC_ATTR2)

For information about available fields see [`mod@rimc_attr2`] module*/
pub type RIMC_ATTR2 = crate::Reg<rimc_attr2::RIMC_ATTR2rs>;
///RIFSC RIMC master attribute register 2
pub mod rimc_attr2;
/**RIMC_ATTR3 (rw) register accessor: RIFSC RIMC master attribute register 3

You can [`read`](crate::Reg::read) this register and get [`rimc_attr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rimc_attr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RIMC_ATTR3)

For information about available fields see [`mod@rimc_attr3`] module*/
pub type RIMC_ATTR3 = crate::Reg<rimc_attr3::RIMC_ATTR3rs>;
///RIFSC RIMC master attribute register 3
pub mod rimc_attr3;
/**RIMC_ATTR4 (rw) register accessor: RIFSC RIMC master attribute register 4

You can [`read`](crate::Reg::read) this register and get [`rimc_attr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rimc_attr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RIMC_ATTR4)

For information about available fields see [`mod@rimc_attr4`] module*/
pub type RIMC_ATTR4 = crate::Reg<rimc_attr4::RIMC_ATTR4rs>;
///RIFSC RIMC master attribute register 4
pub mod rimc_attr4;
/**RIMC_ATTR5 (rw) register accessor: RIFSC RIMC master attribute register 5

You can [`read`](crate::Reg::read) this register and get [`rimc_attr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rimc_attr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RIMC_ATTR5)

For information about available fields see [`mod@rimc_attr5`] module*/
pub type RIMC_ATTR5 = crate::Reg<rimc_attr5::RIMC_ATTR5rs>;
///RIFSC RIMC master attribute register 5
pub mod rimc_attr5;
/**RIMC_ATTR6 (rw) register accessor: RIFSC RIMC master attribute register 6

You can [`read`](crate::Reg::read) this register and get [`rimc_attr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rimc_attr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RIMC_ATTR6)

For information about available fields see [`mod@rimc_attr6`] module*/
pub type RIMC_ATTR6 = crate::Reg<rimc_attr6::RIMC_ATTR6rs>;
///RIFSC RIMC master attribute register 6
pub mod rimc_attr6;
/**RIMC_ATTR7 (rw) register accessor: RIFSC RIMC master attribute register 7

You can [`read`](crate::Reg::read) this register and get [`rimc_attr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rimc_attr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RIMC_ATTR7)

For information about available fields see [`mod@rimc_attr7`] module*/
pub type RIMC_ATTR7 = crate::Reg<rimc_attr7::RIMC_ATTR7rs>;
///RIFSC RIMC master attribute register 7
pub mod rimc_attr7;
/**RIMC_ATTR8 (rw) register accessor: RIFSC RIMC master attribute register 8

You can [`read`](crate::Reg::read) this register and get [`rimc_attr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rimc_attr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RIMC_ATTR8)

For information about available fields see [`mod@rimc_attr8`] module*/
pub type RIMC_ATTR8 = crate::Reg<rimc_attr8::RIMC_ATTR8rs>;
///RIFSC RIMC master attribute register 8
pub mod rimc_attr8;
/**RIMC_ATTR9 (rw) register accessor: RIFSC RIMC master attribute register 9

You can [`read`](crate::Reg::read) this register and get [`rimc_attr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rimc_attr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RIMC_ATTR9)

For information about available fields see [`mod@rimc_attr9`] module*/
pub type RIMC_ATTR9 = crate::Reg<rimc_attr9::RIMC_ATTR9rs>;
///RIFSC RIMC master attribute register 9
pub mod rimc_attr9;
/**RIMC_ATTR10 (rw) register accessor: RIFSC RIMC master attribute register 10

You can [`read`](crate::Reg::read) this register and get [`rimc_attr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rimc_attr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RIMC_ATTR10)

For information about available fields see [`mod@rimc_attr10`] module*/
pub type RIMC_ATTR10 = crate::Reg<rimc_attr10::RIMC_ATTR10rs>;
///RIFSC RIMC master attribute register 10
pub mod rimc_attr10;
/**RIMC_ATTR11 (rw) register accessor: RIFSC RIMC master attribute register 11

You can [`read`](crate::Reg::read) this register and get [`rimc_attr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rimc_attr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:RIMC_ATTR11)

For information about available fields see [`mod@rimc_attr11`] module*/
pub type RIMC_ATTR11 = crate::Reg<rimc_attr11::RIMC_ATTR11rs>;
///RIFSC RIMC master attribute register 11
pub mod rimc_attr11;
/**PPSR0 (r) register accessor: RIFSC peripheral protection status register 0

You can [`read`](crate::Reg::read) this register and get [`ppsr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:PPSR0)

For information about available fields see [`mod@ppsr0`] module*/
pub type PPSR0 = crate::Reg<ppsr0::PPSR0rs>;
///RIFSC peripheral protection status register 0
pub mod ppsr0;
/**PPSR1 (r) register accessor: RIFSC peripheral protection status register 1

You can [`read`](crate::Reg::read) this register and get [`ppsr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:PPSR1)

For information about available fields see [`mod@ppsr1`] module*/
pub type PPSR1 = crate::Reg<ppsr1::PPSR1rs>;
///RIFSC peripheral protection status register 1
pub mod ppsr1;
/**PPSR2 (r) register accessor: RIFSC peripheral protection status register 2

You can [`read`](crate::Reg::read) this register and get [`ppsr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:PPSR2)

For information about available fields see [`mod@ppsr2`] module*/
pub type PPSR2 = crate::Reg<ppsr2::PPSR2rs>;
///RIFSC peripheral protection status register 2
pub mod ppsr2;
/**PPSR3 (r) register accessor: RIFSC peripheral protection status register 3

You can [`read`](crate::Reg::read) this register and get [`ppsr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:PPSR3)

For information about available fields see [`mod@ppsr3`] module*/
pub type PPSR3 = crate::Reg<ppsr3::PPSR3rs>;
///RIFSC peripheral protection status register 3
pub mod ppsr3;
/**PPSR4 (r) register accessor: RIFSC peripheral protection status register 4

You can [`read`](crate::Reg::read) this register and get [`ppsr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:PPSR4)

For information about available fields see [`mod@ppsr4`] module*/
pub type PPSR4 = crate::Reg<ppsr4::PPSR4rs>;
///RIFSC peripheral protection status register 4
pub mod ppsr4;
/**PPSR5 (r) register accessor: RIFSC peripheral protection status register 5

You can [`read`](crate::Reg::read) this register and get [`ppsr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:PPSR5)

For information about available fields see [`mod@ppsr5`] module*/
pub type PPSR5 = crate::Reg<ppsr5::PPSR5rs>;
///RIFSC peripheral protection status register 5
pub mod ppsr5;
