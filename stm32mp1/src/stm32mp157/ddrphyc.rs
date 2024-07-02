#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ddrphyc_ridr: DDRPHYC_RIDR,
    ddrphyc_pir: DDRPHYC_PIR,
    ddrphyc_pgcr: DDRPHYC_PGCR,
    ddrphyc_pgsr: DDRPHYC_PGSR,
    ddrphyc_dllgcr: DDRPHYC_DLLGCR,
    ddrphyc_acdllcr: DDRPHYC_ACDLLCR,
    ddrphyc_ptr0: DDRPHYC_PTR0,
    ddrphyc_ptr1: DDRPHYC_PTR1,
    ddrphyc_ptr2: DDRPHYC_PTR2,
    ddrphyc_aciocr: DDRPHYC_ACIOCR,
    ddrphyc_dxccr: DDRPHYC_DXCCR,
    ddrphyc_dsgcr: DDRPHYC_DSGCR,
    ddrphyc_dcr: DDRPHYC_DCR,
    ddrphyc_dtpr0: DDRPHYC_DTPR0,
    ddrphyc_dtpr1: DDRPHYC_DTPR1,
    ddrphyc_dtpr2: DDRPHYC_DTPR2,
    ddrphyc_ddr3_mr0: DDRPHYC_DDR3_MR0,
    _reserved17: [u8; 0x02],
    ddrphyc_ddr3_mr1: DDRPHYC_DDR3_MR1,
    _reserved18: [u8; 0x02],
    ddrphyc_ddr3_mr2: DDRPHYC_DDR3_MR2,
    _reserved19: [u8; 0x02],
    ddrphyc_ddr3_mr3: DDRPHYC_DDR3_MR3,
    _reserved20: [u8; 0x03],
    ddrphyc_odtcr: DDRPHYC_ODTCR,
    ddrphyc_dtar: DDRPHYC_DTAR,
    ddrphyc_dtdr0: DDRPHYC_DTDR0,
    ddrphyc_dtdr1: DDRPHYC_DTDR1,
    _reserved24: [u8; 0x0118],
    ddrphyc_gpr0: DDRPHYC_GPR0,
    ddrphyc_gpr1: DDRPHYC_GPR1,
    ddrphyc_zq0cr0: DDRPHYC_ZQ0CR0,
    ddrphyc_zq0cr1: DDRPHYC_ZQ0CR1,
    _reserved28: [u8; 0x03],
    ddrphyc_zq0sr0: DDRPHYC_ZQ0SR0,
    ddrphyc_zq0sr1: DDRPHYC_ZQ0SR1,
    _reserved30: [u8; 0x33],
    ddrphyc_dx0gcr: DDRPHYC_DX0GCR,
    ddrphyc_dx0gsr0: DDRPHYC_DX0GSR0,
    _reserved32: [u8; 0x02],
    ddrphyc_dx0gsr1: DDRPHYC_DX0GSR1,
    ddrphyc_dx0dllcr: DDRPHYC_DX0DLLCR,
    ddrphyc_dx0dqtr: DDRPHYC_DX0DQTR,
    ddrphyc_dx0dqstr: DDRPHYC_DX0DQSTR,
    _reserved36: [u8; 0x28],
    ddrphyc_dx1gcr: DDRPHYC_DX1GCR,
    ddrphyc_dx1gsr0: DDRPHYC_DX1GSR0,
    _reserved38: [u8; 0x02],
    ddrphyc_dx1gsr1: DDRPHYC_DX1GSR1,
    ddrphyc_dx1dllcr: DDRPHYC_DX1DLLCR,
    ddrphyc_dx1dqtr: DDRPHYC_DX1DQTR,
    ddrphyc_dx1dqstr: DDRPHYC_DX1DQSTR,
    _reserved42: [u8; 0x28],
    ddrphyc_dx2gcr: DDRPHYC_DX2GCR,
    ddrphyc_dx2gsr0: DDRPHYC_DX2GSR0,
    _reserved44: [u8; 0x02],
    ddrphyc_dx2gsr1: DDRPHYC_DX2GSR1,
    ddrphyc_dx2dllcr: DDRPHYC_DX2DLLCR,
    ddrphyc_dx2dqtr: DDRPHYC_DX2DQTR,
    ddrphyc_dx2dqstr: DDRPHYC_DX2DQSTR,
    _reserved48: [u8; 0x28],
    ddrphyc_dx3gcr: DDRPHYC_DX3GCR,
    ddrphyc_dx3gsr0: DDRPHYC_DX3GSR0,
    _reserved50: [u8; 0x02],
    ddrphyc_dx3gsr1: DDRPHYC_DX3GSR1,
    ddrphyc_dx3dllcr: DDRPHYC_DX3DLLCR,
    ddrphyc_dx3dqtr: DDRPHYC_DX3DQTR,
    ddrphyc_dx3dqstr: DDRPHYC_DX3DQSTR,
}
impl RegisterBlock {
    ///0x00 - DDRPHYC revision ID register
    #[inline(always)]
    pub const fn ddrphyc_ridr(&self) -> &DDRPHYC_RIDR {
        &self.ddrphyc_ridr
    }
    ///0x04 - DDRPHYC PHY initialization register
    #[inline(always)]
    pub const fn ddrphyc_pir(&self) -> &DDRPHYC_PIR {
        &self.ddrphyc_pir
    }
    ///0x08 - DDRPHYC PHY global control register
    #[inline(always)]
    pub const fn ddrphyc_pgcr(&self) -> &DDRPHYC_PGCR {
        &self.ddrphyc_pgcr
    }
    ///0x0c - DDRPHYC PHY global status register
    #[inline(always)]
    pub const fn ddrphyc_pgsr(&self) -> &DDRPHYC_PGSR {
        &self.ddrphyc_pgsr
    }
    ///0x10 - DDRPHYC DDR global control register
    #[inline(always)]
    pub const fn ddrphyc_dllgcr(&self) -> &DDRPHYC_DLLGCR {
        &self.ddrphyc_dllgcr
    }
    ///0x14 - DDRPHYC AC DLL control register
    #[inline(always)]
    pub const fn ddrphyc_acdllcr(&self) -> &DDRPHYC_ACDLLCR {
        &self.ddrphyc_acdllcr
    }
    ///0x18 - DDRPHYC PT register 0
    #[inline(always)]
    pub const fn ddrphyc_ptr0(&self) -> &DDRPHYC_PTR0 {
        &self.ddrphyc_ptr0
    }
    ///0x1c - DDRPHYC PT register 1
    #[inline(always)]
    pub const fn ddrphyc_ptr1(&self) -> &DDRPHYC_PTR1 {
        &self.ddrphyc_ptr1
    }
    ///0x20 - DDRPHYC PT register 2
    #[inline(always)]
    pub const fn ddrphyc_ptr2(&self) -> &DDRPHYC_PTR2 {
        &self.ddrphyc_ptr2
    }
    ///0x24 - DDRPHYC ACIOC register
    #[inline(always)]
    pub const fn ddrphyc_aciocr(&self) -> &DDRPHYC_ACIOCR {
        &self.ddrphyc_aciocr
    }
    ///0x28 - DDRPHYC DXCC register
    #[inline(always)]
    pub const fn ddrphyc_dxccr(&self) -> &DDRPHYC_DXCCR {
        &self.ddrphyc_dxccr
    }
    ///0x2c - DDRPHYC DSGC register
    #[inline(always)]
    pub const fn ddrphyc_dsgcr(&self) -> &DDRPHYC_DSGCR {
        &self.ddrphyc_dsgcr
    }
    ///0x30 - DDRPHYC DC register
    #[inline(always)]
    pub const fn ddrphyc_dcr(&self) -> &DDRPHYC_DCR {
        &self.ddrphyc_dcr
    }
    ///0x34 - DDRPHYC DTP register 0
    #[inline(always)]
    pub const fn ddrphyc_dtpr0(&self) -> &DDRPHYC_DTPR0 {
        &self.ddrphyc_dtpr0
    }
    ///0x38 - DDRPHYC DTP register 1
    #[inline(always)]
    pub const fn ddrphyc_dtpr1(&self) -> &DDRPHYC_DTPR1 {
        &self.ddrphyc_dtpr1
    }
    ///0x3c - DDRPHYC DTP register 2
    #[inline(always)]
    pub const fn ddrphyc_dtpr2(&self) -> &DDRPHYC_DTPR2 {
        &self.ddrphyc_dtpr2
    }
    ///0x40 - DDRPHYC MR0 register for DDR3
    #[inline(always)]
    pub const fn ddrphyc_ddr3_mr0(&self) -> &DDRPHYC_DDR3_MR0 {
        &self.ddrphyc_ddr3_mr0
    }
    ///0x44 - DDRPHYC MR1 register for DDR3
    #[inline(always)]
    pub const fn ddrphyc_ddr3_mr1(&self) -> &DDRPHYC_DDR3_MR1 {
        &self.ddrphyc_ddr3_mr1
    }
    ///0x48 - DDRPHYC MR2 register for DDR3
    #[inline(always)]
    pub const fn ddrphyc_ddr3_mr2(&self) -> &DDRPHYC_DDR3_MR2 {
        &self.ddrphyc_ddr3_mr2
    }
    ///0x4c - DDRPHYC MR3 register for DDR3
    #[inline(always)]
    pub const fn ddrphyc_ddr3_mr3(&self) -> &DDRPHYC_DDR3_MR3 {
        &self.ddrphyc_ddr3_mr3
    }
    ///0x50 - DDRPHYC ODTC register
    #[inline(always)]
    pub const fn ddrphyc_odtcr(&self) -> &DDRPHYC_ODTCR {
        &self.ddrphyc_odtcr
    }
    ///0x54 - DDRPHYC DTA register
    #[inline(always)]
    pub const fn ddrphyc_dtar(&self) -> &DDRPHYC_DTAR {
        &self.ddrphyc_dtar
    }
    ///0x58 - DDRPHYC DTD register 0
    #[inline(always)]
    pub const fn ddrphyc_dtdr0(&self) -> &DDRPHYC_DTDR0 {
        &self.ddrphyc_dtdr0
    }
    ///0x5c - DDRPHYC DTD register 1
    #[inline(always)]
    pub const fn ddrphyc_dtdr1(&self) -> &DDRPHYC_DTDR1 {
        &self.ddrphyc_dtdr1
    }
    ///0x178 - DDRPHYC general purpose register 0
    #[inline(always)]
    pub const fn ddrphyc_gpr0(&self) -> &DDRPHYC_GPR0 {
        &self.ddrphyc_gpr0
    }
    ///0x17c - DDRPHYC general purpose register 1
    #[inline(always)]
    pub const fn ddrphyc_gpr1(&self) -> &DDRPHYC_GPR1 {
        &self.ddrphyc_gpr1
    }
    ///0x180 - DDRPHYC ZQ0C register 0
    #[inline(always)]
    pub const fn ddrphyc_zq0cr0(&self) -> &DDRPHYC_ZQ0CR0 {
        &self.ddrphyc_zq0cr0
    }
    ///0x184 - DDRPHYC ZQ0CR1 register
    #[inline(always)]
    pub const fn ddrphyc_zq0cr1(&self) -> &DDRPHYC_ZQ0CR1 {
        &self.ddrphyc_zq0cr1
    }
    ///0x188 - DDRPHYC ZQ0S register 0
    #[inline(always)]
    pub const fn ddrphyc_zq0sr0(&self) -> &DDRPHYC_ZQ0SR0 {
        &self.ddrphyc_zq0sr0
    }
    ///0x18c - DDRPHYC ZQ0S register 1
    #[inline(always)]
    pub const fn ddrphyc_zq0sr1(&self) -> &DDRPHYC_ZQ0SR1 {
        &self.ddrphyc_zq0sr1
    }
    ///0x1c0 - DDRPHYC byte lane 0 GC register
    #[inline(always)]
    pub const fn ddrphyc_dx0gcr(&self) -> &DDRPHYC_DX0GCR {
        &self.ddrphyc_dx0gcr
    }
    ///0x1c4 - DDRPHYC byte lane 0 GS register 0
    #[inline(always)]
    pub const fn ddrphyc_dx0gsr0(&self) -> &DDRPHYC_DX0GSR0 {
        &self.ddrphyc_dx0gsr0
    }
    ///0x1c8 - DDRPHYC byte lane 0 GS register 1
    #[inline(always)]
    pub const fn ddrphyc_dx0gsr1(&self) -> &DDRPHYC_DX0GSR1 {
        &self.ddrphyc_dx0gsr1
    }
    ///0x1cc - DDRPHYC byte lane 0 DLLC register
    #[inline(always)]
    pub const fn ddrphyc_dx0dllcr(&self) -> &DDRPHYC_DX0DLLCR {
        &self.ddrphyc_dx0dllcr
    }
    ///0x1d0 - DDRPHYC byte lane 0 DQT register
    #[inline(always)]
    pub const fn ddrphyc_dx0dqtr(&self) -> &DDRPHYC_DX0DQTR {
        &self.ddrphyc_dx0dqtr
    }
    ///0x1d4 - DDRPHYC byte lane 0 DQST register
    #[inline(always)]
    pub const fn ddrphyc_dx0dqstr(&self) -> &DDRPHYC_DX0DQSTR {
        &self.ddrphyc_dx0dqstr
    }
    ///0x200 - DDRPHYC byte lane 1 GC register
    #[inline(always)]
    pub const fn ddrphyc_dx1gcr(&self) -> &DDRPHYC_DX1GCR {
        &self.ddrphyc_dx1gcr
    }
    ///0x204 - DDRPHYC byte lane 1 GS register 0
    #[inline(always)]
    pub const fn ddrphyc_dx1gsr0(&self) -> &DDRPHYC_DX1GSR0 {
        &self.ddrphyc_dx1gsr0
    }
    ///0x208 - DDRPHYC byte lane 1 GS register 1
    #[inline(always)]
    pub const fn ddrphyc_dx1gsr1(&self) -> &DDRPHYC_DX1GSR1 {
        &self.ddrphyc_dx1gsr1
    }
    ///0x20c - DDRPHYC byte lane 1 DLLC register
    #[inline(always)]
    pub const fn ddrphyc_dx1dllcr(&self) -> &DDRPHYC_DX1DLLCR {
        &self.ddrphyc_dx1dllcr
    }
    ///0x210 - DDRPHYC byte lane 1 DQT register
    #[inline(always)]
    pub const fn ddrphyc_dx1dqtr(&self) -> &DDRPHYC_DX1DQTR {
        &self.ddrphyc_dx1dqtr
    }
    ///0x214 - DDRPHYC byte lane 1 DQST register
    #[inline(always)]
    pub const fn ddrphyc_dx1dqstr(&self) -> &DDRPHYC_DX1DQSTR {
        &self.ddrphyc_dx1dqstr
    }
    ///0x240 - DDRPHYC byte lane 2 GC register
    #[inline(always)]
    pub const fn ddrphyc_dx2gcr(&self) -> &DDRPHYC_DX2GCR {
        &self.ddrphyc_dx2gcr
    }
    ///0x244 - DDRPHYC byte lane 2 GS register 0
    #[inline(always)]
    pub const fn ddrphyc_dx2gsr0(&self) -> &DDRPHYC_DX2GSR0 {
        &self.ddrphyc_dx2gsr0
    }
    ///0x248 - DDRPHYC byte lane 2 GS register 1
    #[inline(always)]
    pub const fn ddrphyc_dx2gsr1(&self) -> &DDRPHYC_DX2GSR1 {
        &self.ddrphyc_dx2gsr1
    }
    ///0x24c - DDRPHYC byte lane 2 DLLC register
    #[inline(always)]
    pub const fn ddrphyc_dx2dllcr(&self) -> &DDRPHYC_DX2DLLCR {
        &self.ddrphyc_dx2dllcr
    }
    ///0x250 - DDRPHYC byte lane 2 DQT register
    #[inline(always)]
    pub const fn ddrphyc_dx2dqtr(&self) -> &DDRPHYC_DX2DQTR {
        &self.ddrphyc_dx2dqtr
    }
    ///0x254 - DDRPHYC byte lane 2 DQST register
    #[inline(always)]
    pub const fn ddrphyc_dx2dqstr(&self) -> &DDRPHYC_DX2DQSTR {
        &self.ddrphyc_dx2dqstr
    }
    ///0x280 - DDRPHYC byte lane 3 GC register
    #[inline(always)]
    pub const fn ddrphyc_dx3gcr(&self) -> &DDRPHYC_DX3GCR {
        &self.ddrphyc_dx3gcr
    }
    ///0x284 - DDRPHYC byte lane 3 GS register 0
    #[inline(always)]
    pub const fn ddrphyc_dx3gsr0(&self) -> &DDRPHYC_DX3GSR0 {
        &self.ddrphyc_dx3gsr0
    }
    ///0x288 - DDRPHYC byte lane 3 GS register 1
    #[inline(always)]
    pub const fn ddrphyc_dx3gsr1(&self) -> &DDRPHYC_DX3GSR1 {
        &self.ddrphyc_dx3gsr1
    }
    ///0x28c - DDRPHYC byte lane 3 DLLC register
    #[inline(always)]
    pub const fn ddrphyc_dx3dllcr(&self) -> &DDRPHYC_DX3DLLCR {
        &self.ddrphyc_dx3dllcr
    }
    ///0x290 - DDRPHYC byte lane 3 DQT register
    #[inline(always)]
    pub const fn ddrphyc_dx3dqtr(&self) -> &DDRPHYC_DX3DQTR {
        &self.ddrphyc_dx3dqtr
    }
    ///0x294 - DDRPHYC byte lane 3 DQST register
    #[inline(always)]
    pub const fn ddrphyc_dx3dqstr(&self) -> &DDRPHYC_DX3DQSTR {
        &self.ddrphyc_dx3dqstr
    }
}
/**DDRPHYC_RIDR (r) register accessor: DDRPHYC revision ID register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_ridr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_RIDR)

For information about available fields see [`mod@ddrphyc_ridr`]
module*/
pub type DDRPHYC_RIDR = crate::Reg<ddrphyc_ridr::DDRPHYC_RIDRrs>;
///DDRPHYC revision ID register
pub mod ddrphyc_ridr;
/**DDRPHYC_PIR (w) register accessor: DDRPHYC PHY initialization register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_pir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_PIR)

For information about available fields see [`mod@ddrphyc_pir`]
module*/
pub type DDRPHYC_PIR = crate::Reg<ddrphyc_pir::DDRPHYC_PIRrs>;
///DDRPHYC PHY initialization register
pub mod ddrphyc_pir;
/**DDRPHYC_PGCR (rw) register accessor: DDRPHYC PHY global control register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_pgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_pgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_PGCR)

For information about available fields see [`mod@ddrphyc_pgcr`]
module*/
pub type DDRPHYC_PGCR = crate::Reg<ddrphyc_pgcr::DDRPHYC_PGCRrs>;
///DDRPHYC PHY global control register
pub mod ddrphyc_pgcr;
/**DDRPHYC_PGSR (r) register accessor: DDRPHYC PHY global status register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_pgsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_PGSR)

For information about available fields see [`mod@ddrphyc_pgsr`]
module*/
pub type DDRPHYC_PGSR = crate::Reg<ddrphyc_pgsr::DDRPHYC_PGSRrs>;
///DDRPHYC PHY global status register
pub mod ddrphyc_pgsr;
/**DDRPHYC_DLLGCR (rw) register accessor: DDRPHYC DDR global control register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dllgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dllgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DLLGCR)

For information about available fields see [`mod@ddrphyc_dllgcr`]
module*/
pub type DDRPHYC_DLLGCR = crate::Reg<ddrphyc_dllgcr::DDRPHYC_DLLGCRrs>;
///DDRPHYC DDR global control register
pub mod ddrphyc_dllgcr;
/**DDRPHYC_ACDLLCR (rw) register accessor: DDRPHYC AC DLL control register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_acdllcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_acdllcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_ACDLLCR)

For information about available fields see [`mod@ddrphyc_acdllcr`]
module*/
pub type DDRPHYC_ACDLLCR = crate::Reg<ddrphyc_acdllcr::DDRPHYC_ACDLLCRrs>;
///DDRPHYC AC DLL control register
pub mod ddrphyc_acdllcr;
/**DDRPHYC_PTR0 (rw) register accessor: DDRPHYC PT register 0

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_ptr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_ptr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_PTR0)

For information about available fields see [`mod@ddrphyc_ptr0`]
module*/
pub type DDRPHYC_PTR0 = crate::Reg<ddrphyc_ptr0::DDRPHYC_PTR0rs>;
///DDRPHYC PT register 0
pub mod ddrphyc_ptr0;
/**DDRPHYC_PTR1 (rw) register accessor: DDRPHYC PT register 1

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_ptr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_ptr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_PTR1)

For information about available fields see [`mod@ddrphyc_ptr1`]
module*/
pub type DDRPHYC_PTR1 = crate::Reg<ddrphyc_ptr1::DDRPHYC_PTR1rs>;
///DDRPHYC PT register 1
pub mod ddrphyc_ptr1;
/**DDRPHYC_PTR2 (rw) register accessor: DDRPHYC PT register 2

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_ptr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_ptr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_PTR2)

For information about available fields see [`mod@ddrphyc_ptr2`]
module*/
pub type DDRPHYC_PTR2 = crate::Reg<ddrphyc_ptr2::DDRPHYC_PTR2rs>;
///DDRPHYC PT register 2
pub mod ddrphyc_ptr2;
/**DDRPHYC_ACIOCR (rw) register accessor: DDRPHYC ACIOC register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_aciocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_aciocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_ACIOCR)

For information about available fields see [`mod@ddrphyc_aciocr`]
module*/
pub type DDRPHYC_ACIOCR = crate::Reg<ddrphyc_aciocr::DDRPHYC_ACIOCRrs>;
///DDRPHYC ACIOC register
pub mod ddrphyc_aciocr;
/**DDRPHYC_DXCCR (rw) register accessor: DDRPHYC DXCC register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dxccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dxccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DXCCR)

For information about available fields see [`mod@ddrphyc_dxccr`]
module*/
pub type DDRPHYC_DXCCR = crate::Reg<ddrphyc_dxccr::DDRPHYC_DXCCRrs>;
///DDRPHYC DXCC register
pub mod ddrphyc_dxccr;
/**DDRPHYC_DSGCR (rw) register accessor: DDRPHYC DSGC register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dsgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dsgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DSGCR)

For information about available fields see [`mod@ddrphyc_dsgcr`]
module*/
pub type DDRPHYC_DSGCR = crate::Reg<ddrphyc_dsgcr::DDRPHYC_DSGCRrs>;
///DDRPHYC DSGC register
pub mod ddrphyc_dsgcr;
/**DDRPHYC_DCR (rw) register accessor: DDRPHYC DC register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DCR)

For information about available fields see [`mod@ddrphyc_dcr`]
module*/
pub type DDRPHYC_DCR = crate::Reg<ddrphyc_dcr::DDRPHYC_DCRrs>;
///DDRPHYC DC register
pub mod ddrphyc_dcr;
/**DDRPHYC_DTPR0 (rw) register accessor: DDRPHYC DTP register 0

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dtpr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dtpr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DTPR0)

For information about available fields see [`mod@ddrphyc_dtpr0`]
module*/
pub type DDRPHYC_DTPR0 = crate::Reg<ddrphyc_dtpr0::DDRPHYC_DTPR0rs>;
///DDRPHYC DTP register 0
pub mod ddrphyc_dtpr0;
/**DDRPHYC_DTPR1 (rw) register accessor: DDRPHYC DTP register 1

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dtpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dtpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DTPR1)

For information about available fields see [`mod@ddrphyc_dtpr1`]
module*/
pub type DDRPHYC_DTPR1 = crate::Reg<ddrphyc_dtpr1::DDRPHYC_DTPR1rs>;
///DDRPHYC DTP register 1
pub mod ddrphyc_dtpr1;
/**DDRPHYC_DTPR2 (rw) register accessor: DDRPHYC DTP register 2

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dtpr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dtpr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DTPR2)

For information about available fields see [`mod@ddrphyc_dtpr2`]
module*/
pub type DDRPHYC_DTPR2 = crate::Reg<ddrphyc_dtpr2::DDRPHYC_DTPR2rs>;
///DDRPHYC DTP register 2
pub mod ddrphyc_dtpr2;
/**DDRPHYC_DDR3_MR0 (rw) register accessor: DDRPHYC MR0 register for DDR3

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_ddr3_mr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_ddr3_mr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DDR3_MR0)

For information about available fields see [`mod@ddrphyc_ddr3_mr0`]
module*/
pub type DDRPHYC_DDR3_MR0 = crate::Reg<ddrphyc_ddr3_mr0::DDRPHYC_DDR3_MR0rs>;
///DDRPHYC MR0 register for DDR3
pub mod ddrphyc_ddr3_mr0;
/**DDRPHYC_DDR3_MR1 (rw) register accessor: DDRPHYC MR1 register for DDR3

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_ddr3_mr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_ddr3_mr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DDR3_MR1)

For information about available fields see [`mod@ddrphyc_ddr3_mr1`]
module*/
pub type DDRPHYC_DDR3_MR1 = crate::Reg<ddrphyc_ddr3_mr1::DDRPHYC_DDR3_MR1rs>;
///DDRPHYC MR1 register for DDR3
pub mod ddrphyc_ddr3_mr1;
/**DDRPHYC_DDR3_MR2 (rw) register accessor: DDRPHYC MR2 register for DDR3

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_ddr3_mr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_ddr3_mr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DDR3_MR2)

For information about available fields see [`mod@ddrphyc_ddr3_mr2`]
module*/
pub type DDRPHYC_DDR3_MR2 = crate::Reg<ddrphyc_ddr3_mr2::DDRPHYC_DDR3_MR2rs>;
///DDRPHYC MR2 register for DDR3
pub mod ddrphyc_ddr3_mr2;
/**DDRPHYC_DDR3_MR3 (rw) register accessor: DDRPHYC MR3 register for DDR3

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_ddr3_mr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_ddr3_mr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DDR3_MR3)

For information about available fields see [`mod@ddrphyc_ddr3_mr3`]
module*/
pub type DDRPHYC_DDR3_MR3 = crate::Reg<ddrphyc_ddr3_mr3::DDRPHYC_DDR3_MR3rs>;
///DDRPHYC MR3 register for DDR3
pub mod ddrphyc_ddr3_mr3;
/**DDRPHYC_ODTCR (rw) register accessor: DDRPHYC ODTC register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_odtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_odtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_ODTCR)

For information about available fields see [`mod@ddrphyc_odtcr`]
module*/
pub type DDRPHYC_ODTCR = crate::Reg<ddrphyc_odtcr::DDRPHYC_ODTCRrs>;
///DDRPHYC ODTC register
pub mod ddrphyc_odtcr;
/**DDRPHYC_DTAR (rw) register accessor: DDRPHYC DTA register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dtar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dtar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DTAR)

For information about available fields see [`mod@ddrphyc_dtar`]
module*/
pub type DDRPHYC_DTAR = crate::Reg<ddrphyc_dtar::DDRPHYC_DTARrs>;
///DDRPHYC DTA register
pub mod ddrphyc_dtar;
/**DDRPHYC_DTDR0 (rw) register accessor: DDRPHYC DTD register 0

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dtdr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dtdr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DTDR0)

For information about available fields see [`mod@ddrphyc_dtdr0`]
module*/
pub type DDRPHYC_DTDR0 = crate::Reg<ddrphyc_dtdr0::DDRPHYC_DTDR0rs>;
///DDRPHYC DTD register 0
pub mod ddrphyc_dtdr0;
/**DDRPHYC_DTDR1 (rw) register accessor: DDRPHYC DTD register 1

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dtdr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dtdr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DTDR1)

For information about available fields see [`mod@ddrphyc_dtdr1`]
module*/
pub type DDRPHYC_DTDR1 = crate::Reg<ddrphyc_dtdr1::DDRPHYC_DTDR1rs>;
///DDRPHYC DTD register 1
pub mod ddrphyc_dtdr1;
/**DDRPHYC_GPR0 (rw) register accessor: DDRPHYC general purpose register 0

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_gpr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_gpr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_GPR0)

For information about available fields see [`mod@ddrphyc_gpr0`]
module*/
pub type DDRPHYC_GPR0 = crate::Reg<ddrphyc_gpr0::DDRPHYC_GPR0rs>;
///DDRPHYC general purpose register 0
pub mod ddrphyc_gpr0;
/**DDRPHYC_GPR1 (rw) register accessor: DDRPHYC general purpose register 1

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_gpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_gpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_GPR1)

For information about available fields see [`mod@ddrphyc_gpr1`]
module*/
pub type DDRPHYC_GPR1 = crate::Reg<ddrphyc_gpr1::DDRPHYC_GPR1rs>;
///DDRPHYC general purpose register 1
pub mod ddrphyc_gpr1;
/**DDRPHYC_ZQ0CR0 (rw) register accessor: DDRPHYC ZQ0C register 0

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_zq0cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_zq0cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_ZQ0CR0)

For information about available fields see [`mod@ddrphyc_zq0cr0`]
module*/
pub type DDRPHYC_ZQ0CR0 = crate::Reg<ddrphyc_zq0cr0::DDRPHYC_ZQ0CR0rs>;
///DDRPHYC ZQ0C register 0
pub mod ddrphyc_zq0cr0;
/**DDRPHYC_ZQ0CR1 (rw) register accessor: DDRPHYC ZQ0CR1 register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_zq0cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_zq0cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_ZQ0CR1)

For information about available fields see [`mod@ddrphyc_zq0cr1`]
module*/
pub type DDRPHYC_ZQ0CR1 = crate::Reg<ddrphyc_zq0cr1::DDRPHYC_ZQ0CR1rs>;
///DDRPHYC ZQ0CR1 register
pub mod ddrphyc_zq0cr1;
/**DDRPHYC_ZQ0SR0 (r) register accessor: DDRPHYC ZQ0S register 0

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_zq0sr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_ZQ0SR0)

For information about available fields see [`mod@ddrphyc_zq0sr0`]
module*/
pub type DDRPHYC_ZQ0SR0 = crate::Reg<ddrphyc_zq0sr0::DDRPHYC_ZQ0SR0rs>;
///DDRPHYC ZQ0S register 0
pub mod ddrphyc_zq0sr0;
/**DDRPHYC_ZQ0SR1 (r) register accessor: DDRPHYC ZQ0S register 1

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_zq0sr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_ZQ0SR1)

For information about available fields see [`mod@ddrphyc_zq0sr1`]
module*/
pub type DDRPHYC_ZQ0SR1 = crate::Reg<ddrphyc_zq0sr1::DDRPHYC_ZQ0SR1rs>;
///DDRPHYC ZQ0S register 1
pub mod ddrphyc_zq0sr1;
/**DDRPHYC_DX0GCR (rw) register accessor: DDRPHYC byte lane 0 GC register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dx0gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dx0gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DX0GCR)

For information about available fields see [`mod@ddrphyc_dx0gcr`]
module*/
pub type DDRPHYC_DX0GCR = crate::Reg<ddrphyc_dx0gcr::DDRPHYC_DX0GCRrs>;
///DDRPHYC byte lane 0 GC register
pub mod ddrphyc_dx0gcr;
/**DDRPHYC_DX0GSR0 (r) register accessor: DDRPHYC byte lane 0 GS register 0

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dx0gsr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DX0GSR0)

For information about available fields see [`mod@ddrphyc_dx0gsr0`]
module*/
pub type DDRPHYC_DX0GSR0 = crate::Reg<ddrphyc_dx0gsr0::DDRPHYC_DX0GSR0rs>;
///DDRPHYC byte lane 0 GS register 0
pub mod ddrphyc_dx0gsr0;
/**DDRPHYC_DX0GSR1 (r) register accessor: DDRPHYC byte lane 0 GS register 1

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dx0gsr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DX0GSR1)

For information about available fields see [`mod@ddrphyc_dx0gsr1`]
module*/
pub type DDRPHYC_DX0GSR1 = crate::Reg<ddrphyc_dx0gsr1::DDRPHYC_DX0GSR1rs>;
///DDRPHYC byte lane 0 GS register 1
pub mod ddrphyc_dx0gsr1;
/**DDRPHYC_DX0DLLCR (rw) register accessor: DDRPHYC byte lane 0 DLLC register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dx0dllcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dx0dllcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DX0DLLCR)

For information about available fields see [`mod@ddrphyc_dx0dllcr`]
module*/
pub type DDRPHYC_DX0DLLCR = crate::Reg<ddrphyc_dx0dllcr::DDRPHYC_DX0DLLCRrs>;
///DDRPHYC byte lane 0 DLLC register
pub mod ddrphyc_dx0dllcr;
/**DDRPHYC_DX0DQTR (rw) register accessor: DDRPHYC byte lane 0 DQT register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dx0dqtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dx0dqtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DX0DQTR)

For information about available fields see [`mod@ddrphyc_dx0dqtr`]
module*/
pub type DDRPHYC_DX0DQTR = crate::Reg<ddrphyc_dx0dqtr::DDRPHYC_DX0DQTRrs>;
///DDRPHYC byte lane 0 DQT register
pub mod ddrphyc_dx0dqtr;
/**DDRPHYC_DX0DQSTR (rw) register accessor: DDRPHYC byte lane 0 DQST register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dx0dqstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dx0dqstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DX0DQSTR)

For information about available fields see [`mod@ddrphyc_dx0dqstr`]
module*/
pub type DDRPHYC_DX0DQSTR = crate::Reg<ddrphyc_dx0dqstr::DDRPHYC_DX0DQSTRrs>;
///DDRPHYC byte lane 0 DQST register
pub mod ddrphyc_dx0dqstr;
/**DDRPHYC_DX1GCR (rw) register accessor: DDRPHYC byte lane 1 GC register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dx1gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dx1gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DX1GCR)

For information about available fields see [`mod@ddrphyc_dx1gcr`]
module*/
pub type DDRPHYC_DX1GCR = crate::Reg<ddrphyc_dx1gcr::DDRPHYC_DX1GCRrs>;
///DDRPHYC byte lane 1 GC register
pub mod ddrphyc_dx1gcr;
/**DDRPHYC_DX1GSR0 (r) register accessor: DDRPHYC byte lane 1 GS register 0

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dx1gsr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DX1GSR0)

For information about available fields see [`mod@ddrphyc_dx1gsr0`]
module*/
pub type DDRPHYC_DX1GSR0 = crate::Reg<ddrphyc_dx1gsr0::DDRPHYC_DX1GSR0rs>;
///DDRPHYC byte lane 1 GS register 0
pub mod ddrphyc_dx1gsr0;
/**DDRPHYC_DX1GSR1 (r) register accessor: DDRPHYC byte lane 1 GS register 1

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dx1gsr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DX1GSR1)

For information about available fields see [`mod@ddrphyc_dx1gsr1`]
module*/
pub type DDRPHYC_DX1GSR1 = crate::Reg<ddrphyc_dx1gsr1::DDRPHYC_DX1GSR1rs>;
///DDRPHYC byte lane 1 GS register 1
pub mod ddrphyc_dx1gsr1;
/**DDRPHYC_DX1DLLCR (rw) register accessor: DDRPHYC byte lane 1 DLLC register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dx1dllcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dx1dllcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DX1DLLCR)

For information about available fields see [`mod@ddrphyc_dx1dllcr`]
module*/
pub type DDRPHYC_DX1DLLCR = crate::Reg<ddrphyc_dx1dllcr::DDRPHYC_DX1DLLCRrs>;
///DDRPHYC byte lane 1 DLLC register
pub mod ddrphyc_dx1dllcr;
/**DDRPHYC_DX1DQTR (rw) register accessor: DDRPHYC byte lane 1 DQT register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dx1dqtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dx1dqtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DX1DQTR)

For information about available fields see [`mod@ddrphyc_dx1dqtr`]
module*/
pub type DDRPHYC_DX1DQTR = crate::Reg<ddrphyc_dx1dqtr::DDRPHYC_DX1DQTRrs>;
///DDRPHYC byte lane 1 DQT register
pub mod ddrphyc_dx1dqtr;
/**DDRPHYC_DX1DQSTR (rw) register accessor: DDRPHYC byte lane 1 DQST register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dx1dqstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dx1dqstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DX1DQSTR)

For information about available fields see [`mod@ddrphyc_dx1dqstr`]
module*/
pub type DDRPHYC_DX1DQSTR = crate::Reg<ddrphyc_dx1dqstr::DDRPHYC_DX1DQSTRrs>;
///DDRPHYC byte lane 1 DQST register
pub mod ddrphyc_dx1dqstr;
/**DDRPHYC_DX2GCR (rw) register accessor: DDRPHYC byte lane 2 GC register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dx2gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dx2gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DX2GCR)

For information about available fields see [`mod@ddrphyc_dx2gcr`]
module*/
pub type DDRPHYC_DX2GCR = crate::Reg<ddrphyc_dx2gcr::DDRPHYC_DX2GCRrs>;
///DDRPHYC byte lane 2 GC register
pub mod ddrphyc_dx2gcr;
/**DDRPHYC_DX2GSR0 (r) register accessor: DDRPHYC byte lane 2 GS register 0

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dx2gsr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DX2GSR0)

For information about available fields see [`mod@ddrphyc_dx2gsr0`]
module*/
pub type DDRPHYC_DX2GSR0 = crate::Reg<ddrphyc_dx2gsr0::DDRPHYC_DX2GSR0rs>;
///DDRPHYC byte lane 2 GS register 0
pub mod ddrphyc_dx2gsr0;
/**DDRPHYC_DX2GSR1 (r) register accessor: DDRPHYC byte lane 2 GS register 1

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dx2gsr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DX2GSR1)

For information about available fields see [`mod@ddrphyc_dx2gsr1`]
module*/
pub type DDRPHYC_DX2GSR1 = crate::Reg<ddrphyc_dx2gsr1::DDRPHYC_DX2GSR1rs>;
///DDRPHYC byte lane 2 GS register 1
pub mod ddrphyc_dx2gsr1;
/**DDRPHYC_DX2DLLCR (rw) register accessor: DDRPHYC byte lane 2 DLLC register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dx2dllcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dx2dllcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DX2DLLCR)

For information about available fields see [`mod@ddrphyc_dx2dllcr`]
module*/
pub type DDRPHYC_DX2DLLCR = crate::Reg<ddrphyc_dx2dllcr::DDRPHYC_DX2DLLCRrs>;
///DDRPHYC byte lane 2 DLLC register
pub mod ddrphyc_dx2dllcr;
/**DDRPHYC_DX2DQTR (rw) register accessor: DDRPHYC byte lane 2 DQT register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dx2dqtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dx2dqtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DX2DQTR)

For information about available fields see [`mod@ddrphyc_dx2dqtr`]
module*/
pub type DDRPHYC_DX2DQTR = crate::Reg<ddrphyc_dx2dqtr::DDRPHYC_DX2DQTRrs>;
///DDRPHYC byte lane 2 DQT register
pub mod ddrphyc_dx2dqtr;
/**DDRPHYC_DX2DQSTR (rw) register accessor: DDRPHYC byte lane 2 DQST register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dx2dqstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dx2dqstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DX2DQSTR)

For information about available fields see [`mod@ddrphyc_dx2dqstr`]
module*/
pub type DDRPHYC_DX2DQSTR = crate::Reg<ddrphyc_dx2dqstr::DDRPHYC_DX2DQSTRrs>;
///DDRPHYC byte lane 2 DQST register
pub mod ddrphyc_dx2dqstr;
/**DDRPHYC_DX3GCR (rw) register accessor: DDRPHYC byte lane 3 GC register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dx3gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dx3gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DX3GCR)

For information about available fields see [`mod@ddrphyc_dx3gcr`]
module*/
pub type DDRPHYC_DX3GCR = crate::Reg<ddrphyc_dx3gcr::DDRPHYC_DX3GCRrs>;
///DDRPHYC byte lane 3 GC register
pub mod ddrphyc_dx3gcr;
/**DDRPHYC_DX3GSR0 (r) register accessor: DDRPHYC byte lane 3 GS register 0

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dx3gsr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DX3GSR0)

For information about available fields see [`mod@ddrphyc_dx3gsr0`]
module*/
pub type DDRPHYC_DX3GSR0 = crate::Reg<ddrphyc_dx3gsr0::DDRPHYC_DX3GSR0rs>;
///DDRPHYC byte lane 3 GS register 0
pub mod ddrphyc_dx3gsr0;
/**DDRPHYC_DX3GSR1 (r) register accessor: DDRPHYC byte lane 3 GS register 1

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dx3gsr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DX3GSR1)

For information about available fields see [`mod@ddrphyc_dx3gsr1`]
module*/
pub type DDRPHYC_DX3GSR1 = crate::Reg<ddrphyc_dx3gsr1::DDRPHYC_DX3GSR1rs>;
///DDRPHYC byte lane 3 GS register 1
pub mod ddrphyc_dx3gsr1;
/**DDRPHYC_DX3DLLCR (rw) register accessor: DDRPHYC byte lane 3 DLLC register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dx3dllcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dx3dllcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DX3DLLCR)

For information about available fields see [`mod@ddrphyc_dx3dllcr`]
module*/
pub type DDRPHYC_DX3DLLCR = crate::Reg<ddrphyc_dx3dllcr::DDRPHYC_DX3DLLCRrs>;
///DDRPHYC byte lane 3 DLLC register
pub mod ddrphyc_dx3dllcr;
/**DDRPHYC_DX3DQTR (rw) register accessor: DDRPHYC byte lane 3 DQT register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dx3dqtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dx3dqtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DX3DQTR)

For information about available fields see [`mod@ddrphyc_dx3dqtr`]
module*/
pub type DDRPHYC_DX3DQTR = crate::Reg<ddrphyc_dx3dqtr::DDRPHYC_DX3DQTRrs>;
///DDRPHYC byte lane 3 DQT register
pub mod ddrphyc_dx3dqtr;
/**DDRPHYC_DX3DQSTR (rw) register accessor: DDRPHYC byte lane 3 DQST register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_dx3dqstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_dx3dqstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_DX3DQSTR)

For information about available fields see [`mod@ddrphyc_dx3dqstr`]
module*/
pub type DDRPHYC_DX3DQSTR = crate::Reg<ddrphyc_dx3dqstr::DDRPHYC_DX3DQSTRrs>;
///DDRPHYC byte lane 3 DQST register
pub mod ddrphyc_dx3dqstr;
