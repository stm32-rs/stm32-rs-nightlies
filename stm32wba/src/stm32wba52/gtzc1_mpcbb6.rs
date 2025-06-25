#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    _reserved1: [u8; 0x0c],
    cfglock: CFGLOCK,
    _reserved2: [u8; 0xec],
    seccfgr0: SECCFGR0,
    _reserved3: [u8; 0xfc],
    privcfgr0: PRIVCFGR0,
}
impl RegisterBlock {
    ///0x00 - MPCBB control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x10 - GTZC1 SRAMz MPCBB configuration lock register
    #[inline(always)]
    pub const fn cfglock(&self) -> &CFGLOCK {
        &self.cfglock
    }
    ///0x100 - GTZC1 MPCBB security configuration for super-block 0 register
    #[inline(always)]
    pub const fn seccfgr0(&self) -> &SECCFGR0 {
        &self.seccfgr0
    }
    ///0x200 - GTZC1 MPCBB privileged configuration for super-block 0 register
    #[inline(always)]
    pub const fn privcfgr0(&self) -> &PRIVCFGR0 {
        &self.privcfgr0
    }
}
/**CR (rw) register accessor: MPCBB control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GTZC1_MPCBB6:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///MPCBB control register
pub mod cr;
/**CFGLOCK (rw) register accessor: GTZC1 SRAMz MPCBB configuration lock register

You can [`read`](crate::Reg::read) this register and get [`cfglock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfglock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GTZC1_MPCBB6:CFGLOCK)

For information about available fields see [`mod@cfglock`] module*/
pub type CFGLOCK = crate::Reg<cfglock::CFGLOCKrs>;
///GTZC1 SRAMz MPCBB configuration lock register
pub mod cfglock;
/**SECCFGR0 (rw) register accessor: GTZC1 MPCBB security configuration for super-block 0 register

You can [`read`](crate::Reg::read) this register and get [`seccfgr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GTZC1_MPCBB6:SECCFGR0)

For information about available fields see [`mod@seccfgr0`] module*/
pub type SECCFGR0 = crate::Reg<seccfgr0::SECCFGR0rs>;
///GTZC1 MPCBB security configuration for super-block 0 register
pub mod seccfgr0;
/**PRIVCFGR0 (rw) register accessor: GTZC1 MPCBB privileged configuration for super-block 0 register

You can [`read`](crate::Reg::read) this register and get [`privcfgr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GTZC1_MPCBB6:PRIVCFGR0)

For information about available fields see [`mod@privcfgr0`] module*/
pub type PRIVCFGR0 = crate::Reg<privcfgr0::PRIVCFGR0rs>;
///GTZC1 MPCBB privileged configuration for super-block 0 register
pub mod privcfgr0;
