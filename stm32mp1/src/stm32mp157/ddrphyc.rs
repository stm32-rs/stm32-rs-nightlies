#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ridr: RIDR,
    pir: PIR,
    pgcr: PGCR,
    pgsr: PGSR,
    dllgcr: DLLGCR,
    acdllcr: ACDLLCR,
    ptr0: PTR0,
    ptr1: PTR1,
    ptr2: PTR2,
    aciocr: ACIOCR,
    dxccr: DXCCR,
    dsgcr: DSGCR,
    dcr: DCR,
    dtpr0: DTPR0,
    dtpr1: DTPR1,
    dtpr2: DTPR2,
    ddr3_mr0: DDR3_MR0,
    _reserved17: [u8; 0x02],
    ddr3_mr1: DDR3_MR1,
    _reserved18: [u8; 0x02],
    ddr3_mr2: DDR3_MR2,
    _reserved19: [u8; 0x02],
    ddr3_mr3: DDR3_MR3,
    _reserved20: [u8; 0x03],
    odtcr: ODTCR,
    dtar: DTAR,
    dtdr0: DTDR0,
    dtdr1: DTDR1,
    _reserved24: [u8; 0x0118],
    gpr0: GPR0,
    gpr1: GPR1,
    zq0cr0: ZQ0CR0,
    zq0cr1: ZQ0CR1,
    _reserved28: [u8; 0x03],
    zq0sr0: ZQ0SR0,
    zq0sr1: ZQ0SR1,
    _reserved30: [u8; 0x33],
    dx0gcr: DX0GCR,
    dx0gsr0: DX0GSR0,
    _reserved32: [u8; 0x02],
    dx0gsr1: DX0GSR1,
    dx0dllcr: DX0DLLCR,
    dx0dqtr: DX0DQTR,
    dx0dqstr: DX0DQSTR,
    _reserved36: [u8; 0x28],
    dx1gcr: DX1GCR,
    dx1gsr0: DX1GSR0,
    _reserved38: [u8; 0x02],
    dx1gsr1: DX1GSR1,
    dx1dllcr: DX1DLLCR,
    dx1dqtr: DX1DQTR,
    dx1dqstr: DX1DQSTR,
    _reserved42: [u8; 0x28],
    dx2gcr: DX2GCR,
    dx2gsr0: DX2GSR0,
    _reserved44: [u8; 0x02],
    dx2gsr1: DX2GSR1,
    dx2dllcr: DX2DLLCR,
    dx2dqtr: DX2DQTR,
    dx2dqstr: DX2DQSTR,
    _reserved48: [u8; 0x28],
    dx3gcr: DX3GCR,
    dx3gsr0: DX3GSR0,
    _reserved50: [u8; 0x02],
    dx3gsr1: DX3GSR1,
    dx3dllcr: DX3DLLCR,
    dx3dqtr: DX3DQTR,
    dx3dqstr: DX3DQSTR,
}
impl RegisterBlock {
    ///0x00 - DDRPHYC revision ID register
    #[inline(always)]
    pub const fn ridr(&self) -> &RIDR {
        &self.ridr
    }
    ///0x04 - DDRPHYC PHY initialization register
    #[inline(always)]
    pub const fn pir(&self) -> &PIR {
        &self.pir
    }
    ///0x08 - DDRPHYC PHY global control register
    #[inline(always)]
    pub const fn pgcr(&self) -> &PGCR {
        &self.pgcr
    }
    ///0x0c - DDRPHYC PHY global status register
    #[inline(always)]
    pub const fn pgsr(&self) -> &PGSR {
        &self.pgsr
    }
    ///0x10 - DDRPHYC DDR global control register
    #[inline(always)]
    pub const fn dllgcr(&self) -> &DLLGCR {
        &self.dllgcr
    }
    ///0x14 - DDRPHYC AC DLL control register
    #[inline(always)]
    pub const fn acdllcr(&self) -> &ACDLLCR {
        &self.acdllcr
    }
    ///0x18 - DDRPHYC PT register 0
    #[inline(always)]
    pub const fn ptr0(&self) -> &PTR0 {
        &self.ptr0
    }
    ///0x1c - DDRPHYC PT register 1
    #[inline(always)]
    pub const fn ptr1(&self) -> &PTR1 {
        &self.ptr1
    }
    ///0x20 - DDRPHYC PT register 2
    #[inline(always)]
    pub const fn ptr2(&self) -> &PTR2 {
        &self.ptr2
    }
    ///0x24 - DDRPHYC ACIOC register
    #[inline(always)]
    pub const fn aciocr(&self) -> &ACIOCR {
        &self.aciocr
    }
    ///0x28 - DDRPHYC DXCC register
    #[inline(always)]
    pub const fn dxccr(&self) -> &DXCCR {
        &self.dxccr
    }
    ///0x2c - DDRPHYC DSGC register
    #[inline(always)]
    pub const fn dsgcr(&self) -> &DSGCR {
        &self.dsgcr
    }
    ///0x30 - DDRPHYC DC register
    #[inline(always)]
    pub const fn dcr(&self) -> &DCR {
        &self.dcr
    }
    ///0x34 - DDRPHYC DTP register 0
    #[inline(always)]
    pub const fn dtpr0(&self) -> &DTPR0 {
        &self.dtpr0
    }
    ///0x38 - DDRPHYC DTP register 1
    #[inline(always)]
    pub const fn dtpr1(&self) -> &DTPR1 {
        &self.dtpr1
    }
    ///0x3c - DDRPHYC DTP register 2
    #[inline(always)]
    pub const fn dtpr2(&self) -> &DTPR2 {
        &self.dtpr2
    }
    ///0x40 - DDRPHYC MR0 register for DDR3
    #[inline(always)]
    pub const fn ddr3_mr0(&self) -> &DDR3_MR0 {
        &self.ddr3_mr0
    }
    ///0x44 - DDRPHYC MR1 register for DDR3
    #[inline(always)]
    pub const fn ddr3_mr1(&self) -> &DDR3_MR1 {
        &self.ddr3_mr1
    }
    ///0x48 - DDRPHYC MR2 register for DDR3
    #[inline(always)]
    pub const fn ddr3_mr2(&self) -> &DDR3_MR2 {
        &self.ddr3_mr2
    }
    ///0x4c - DDRPHYC MR3 register for DDR3
    #[inline(always)]
    pub const fn ddr3_mr3(&self) -> &DDR3_MR3 {
        &self.ddr3_mr3
    }
    ///0x50 - DDRPHYC ODTC register
    #[inline(always)]
    pub const fn odtcr(&self) -> &ODTCR {
        &self.odtcr
    }
    ///0x54 - DDRPHYC DTA register
    #[inline(always)]
    pub const fn dtar(&self) -> &DTAR {
        &self.dtar
    }
    ///0x58 - DDRPHYC DTD register 0
    #[inline(always)]
    pub const fn dtdr0(&self) -> &DTDR0 {
        &self.dtdr0
    }
    ///0x5c - DDRPHYC DTD register 1
    #[inline(always)]
    pub const fn dtdr1(&self) -> &DTDR1 {
        &self.dtdr1
    }
    ///0x178 - DDRPHYC general purpose register 0
    #[inline(always)]
    pub const fn gpr0(&self) -> &GPR0 {
        &self.gpr0
    }
    ///0x17c - DDRPHYC general purpose register 1
    #[inline(always)]
    pub const fn gpr1(&self) -> &GPR1 {
        &self.gpr1
    }
    ///0x180 - DDRPHYC ZQ0C register 0
    #[inline(always)]
    pub const fn zq0cr0(&self) -> &ZQ0CR0 {
        &self.zq0cr0
    }
    ///0x184 - DDRPHYC ZQ0CR1 register
    #[inline(always)]
    pub const fn zq0cr1(&self) -> &ZQ0CR1 {
        &self.zq0cr1
    }
    ///0x188 - DDRPHYC ZQ0S register 0
    #[inline(always)]
    pub const fn zq0sr0(&self) -> &ZQ0SR0 {
        &self.zq0sr0
    }
    ///0x18c - DDRPHYC ZQ0S register 1
    #[inline(always)]
    pub const fn zq0sr1(&self) -> &ZQ0SR1 {
        &self.zq0sr1
    }
    ///0x1c0 - DDRPHYC byte lane 0 GC register
    #[inline(always)]
    pub const fn dx0gcr(&self) -> &DX0GCR {
        &self.dx0gcr
    }
    ///0x1c4 - DDRPHYC byte lane 0 GS register 0
    #[inline(always)]
    pub const fn dx0gsr0(&self) -> &DX0GSR0 {
        &self.dx0gsr0
    }
    ///0x1c8 - DDRPHYC byte lane 0 GS register 1
    #[inline(always)]
    pub const fn dx0gsr1(&self) -> &DX0GSR1 {
        &self.dx0gsr1
    }
    ///0x1cc - DDRPHYC byte lane 0 DLLC register
    #[inline(always)]
    pub const fn dx0dllcr(&self) -> &DX0DLLCR {
        &self.dx0dllcr
    }
    ///0x1d0 - DDRPHYC byte lane 0 DQT register
    #[inline(always)]
    pub const fn dx0dqtr(&self) -> &DX0DQTR {
        &self.dx0dqtr
    }
    ///0x1d4 - DDRPHYC byte lane 0 DQST register
    #[inline(always)]
    pub const fn dx0dqstr(&self) -> &DX0DQSTR {
        &self.dx0dqstr
    }
    ///0x200 - DDRPHYC byte lane 1 GC register
    #[inline(always)]
    pub const fn dx1gcr(&self) -> &DX1GCR {
        &self.dx1gcr
    }
    ///0x204 - DDRPHYC byte lane 1 GS register 0
    #[inline(always)]
    pub const fn dx1gsr0(&self) -> &DX1GSR0 {
        &self.dx1gsr0
    }
    ///0x208 - DDRPHYC byte lane 1 GS register 1
    #[inline(always)]
    pub const fn dx1gsr1(&self) -> &DX1GSR1 {
        &self.dx1gsr1
    }
    ///0x20c - DDRPHYC byte lane 1 DLLC register
    #[inline(always)]
    pub const fn dx1dllcr(&self) -> &DX1DLLCR {
        &self.dx1dllcr
    }
    ///0x210 - DDRPHYC byte lane 1 DQT register
    #[inline(always)]
    pub const fn dx1dqtr(&self) -> &DX1DQTR {
        &self.dx1dqtr
    }
    ///0x214 - DDRPHYC byte lane 1 DQST register
    #[inline(always)]
    pub const fn dx1dqstr(&self) -> &DX1DQSTR {
        &self.dx1dqstr
    }
    ///0x240 - DDRPHYC byte lane 2 GC register
    #[inline(always)]
    pub const fn dx2gcr(&self) -> &DX2GCR {
        &self.dx2gcr
    }
    ///0x244 - DDRPHYC byte lane 2 GS register 0
    #[inline(always)]
    pub const fn dx2gsr0(&self) -> &DX2GSR0 {
        &self.dx2gsr0
    }
    ///0x248 - DDRPHYC byte lane 2 GS register 1
    #[inline(always)]
    pub const fn dx2gsr1(&self) -> &DX2GSR1 {
        &self.dx2gsr1
    }
    ///0x24c - DDRPHYC byte lane 2 DLLC register
    #[inline(always)]
    pub const fn dx2dllcr(&self) -> &DX2DLLCR {
        &self.dx2dllcr
    }
    ///0x250 - DDRPHYC byte lane 2 DQT register
    #[inline(always)]
    pub const fn dx2dqtr(&self) -> &DX2DQTR {
        &self.dx2dqtr
    }
    ///0x254 - DDRPHYC byte lane 2 DQST register
    #[inline(always)]
    pub const fn dx2dqstr(&self) -> &DX2DQSTR {
        &self.dx2dqstr
    }
    ///0x280 - DDRPHYC byte lane 3 GC register
    #[inline(always)]
    pub const fn dx3gcr(&self) -> &DX3GCR {
        &self.dx3gcr
    }
    ///0x284 - DDRPHYC byte lane 3 GS register 0
    #[inline(always)]
    pub const fn dx3gsr0(&self) -> &DX3GSR0 {
        &self.dx3gsr0
    }
    ///0x288 - DDRPHYC byte lane 3 GS register 1
    #[inline(always)]
    pub const fn dx3gsr1(&self) -> &DX3GSR1 {
        &self.dx3gsr1
    }
    ///0x28c - DDRPHYC byte lane 3 DLLC register
    #[inline(always)]
    pub const fn dx3dllcr(&self) -> &DX3DLLCR {
        &self.dx3dllcr
    }
    ///0x290 - DDRPHYC byte lane 3 DQT register
    #[inline(always)]
    pub const fn dx3dqtr(&self) -> &DX3DQTR {
        &self.dx3dqtr
    }
    ///0x294 - DDRPHYC byte lane 3 DQST register
    #[inline(always)]
    pub const fn dx3dqstr(&self) -> &DX3DQSTR {
        &self.dx3dqstr
    }
}
/**RIDR (r) register accessor: DDRPHYC revision ID register

You can [`read`](crate::Reg::read) this register and get [`ridr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:RIDR)

For information about available fields see [`mod@ridr`] module*/
pub type RIDR = crate::Reg<ridr::RIDRrs>;
///DDRPHYC revision ID register
pub mod ridr;
/**PIR (w) register accessor: DDRPHYC PHY initialization register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:PIR)

For information about available fields see [`mod@pir`] module*/
pub type PIR = crate::Reg<pir::PIRrs>;
///DDRPHYC PHY initialization register
pub mod pir;
/**PGCR (rw) register accessor: DDRPHYC PHY global control register

You can [`read`](crate::Reg::read) this register and get [`pgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:PGCR)

For information about available fields see [`mod@pgcr`] module*/
pub type PGCR = crate::Reg<pgcr::PGCRrs>;
///DDRPHYC PHY global control register
pub mod pgcr;
/**PGSR (r) register accessor: DDRPHYC PHY global status register

You can [`read`](crate::Reg::read) this register and get [`pgsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:PGSR)

For information about available fields see [`mod@pgsr`] module*/
pub type PGSR = crate::Reg<pgsr::PGSRrs>;
///DDRPHYC PHY global status register
pub mod pgsr;
/**DLLGCR (rw) register accessor: DDRPHYC DDR global control register

You can [`read`](crate::Reg::read) this register and get [`dllgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dllgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DLLGCR)

For information about available fields see [`mod@dllgcr`] module*/
pub type DLLGCR = crate::Reg<dllgcr::DLLGCRrs>;
///DDRPHYC DDR global control register
pub mod dllgcr;
/**ACDLLCR (rw) register accessor: DDRPHYC AC DLL control register

You can [`read`](crate::Reg::read) this register and get [`acdllcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acdllcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:ACDLLCR)

For information about available fields see [`mod@acdllcr`] module*/
pub type ACDLLCR = crate::Reg<acdllcr::ACDLLCRrs>;
///DDRPHYC AC DLL control register
pub mod acdllcr;
/**PTR0 (rw) register accessor: DDRPHYC PT register 0

You can [`read`](crate::Reg::read) this register and get [`ptr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:PTR0)

For information about available fields see [`mod@ptr0`] module*/
pub type PTR0 = crate::Reg<ptr0::PTR0rs>;
///DDRPHYC PT register 0
pub mod ptr0;
/**PTR1 (rw) register accessor: DDRPHYC PT register 1

You can [`read`](crate::Reg::read) this register and get [`ptr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:PTR1)

For information about available fields see [`mod@ptr1`] module*/
pub type PTR1 = crate::Reg<ptr1::PTR1rs>;
///DDRPHYC PT register 1
pub mod ptr1;
/**PTR2 (rw) register accessor: DDRPHYC PT register 2

You can [`read`](crate::Reg::read) this register and get [`ptr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:PTR2)

For information about available fields see [`mod@ptr2`] module*/
pub type PTR2 = crate::Reg<ptr2::PTR2rs>;
///DDRPHYC PT register 2
pub mod ptr2;
/**ACIOCR (rw) register accessor: DDRPHYC ACIOC register

You can [`read`](crate::Reg::read) this register and get [`aciocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aciocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:ACIOCR)

For information about available fields see [`mod@aciocr`] module*/
pub type ACIOCR = crate::Reg<aciocr::ACIOCRrs>;
///DDRPHYC ACIOC register
pub mod aciocr;
/**DXCCR (rw) register accessor: DDRPHYC DXCC register

You can [`read`](crate::Reg::read) this register and get [`dxccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dxccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DXCCR)

For information about available fields see [`mod@dxccr`] module*/
pub type DXCCR = crate::Reg<dxccr::DXCCRrs>;
///DDRPHYC DXCC register
pub mod dxccr;
/**DSGCR (rw) register accessor: DDRPHYC DSGC register

You can [`read`](crate::Reg::read) this register and get [`dsgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DSGCR)

For information about available fields see [`mod@dsgcr`] module*/
pub type DSGCR = crate::Reg<dsgcr::DSGCRrs>;
///DDRPHYC DSGC register
pub mod dsgcr;
/**DCR (rw) register accessor: DDRPHYC DC register

You can [`read`](crate::Reg::read) this register and get [`dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DCR)

For information about available fields see [`mod@dcr`] module*/
pub type DCR = crate::Reg<dcr::DCRrs>;
///DDRPHYC DC register
pub mod dcr;
/**DTPR0 (rw) register accessor: DDRPHYC DTP register 0

You can [`read`](crate::Reg::read) this register and get [`dtpr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtpr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DTPR0)

For information about available fields see [`mod@dtpr0`] module*/
pub type DTPR0 = crate::Reg<dtpr0::DTPR0rs>;
///DDRPHYC DTP register 0
pub mod dtpr0;
/**DTPR1 (rw) register accessor: DDRPHYC DTP register 1

You can [`read`](crate::Reg::read) this register and get [`dtpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DTPR1)

For information about available fields see [`mod@dtpr1`] module*/
pub type DTPR1 = crate::Reg<dtpr1::DTPR1rs>;
///DDRPHYC DTP register 1
pub mod dtpr1;
/**DTPR2 (rw) register accessor: DDRPHYC DTP register 2

You can [`read`](crate::Reg::read) this register and get [`dtpr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtpr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DTPR2)

For information about available fields see [`mod@dtpr2`] module*/
pub type DTPR2 = crate::Reg<dtpr2::DTPR2rs>;
///DDRPHYC DTP register 2
pub mod dtpr2;
/**DDR3_MR0 (rw) register accessor: DDRPHYC MR0 register for DDR3

You can [`read`](crate::Reg::read) this register and get [`ddr3_mr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddr3_mr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDR3_MR0)

For information about available fields see [`mod@ddr3_mr0`] module*/
pub type DDR3_MR0 = crate::Reg<ddr3_mr0::DDR3_MR0rs>;
///DDRPHYC MR0 register for DDR3
pub mod ddr3_mr0;
/**DDR3_MR1 (rw) register accessor: DDRPHYC MR1 register for DDR3

You can [`read`](crate::Reg::read) this register and get [`ddr3_mr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddr3_mr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDR3_MR1)

For information about available fields see [`mod@ddr3_mr1`] module*/
pub type DDR3_MR1 = crate::Reg<ddr3_mr1::DDR3_MR1rs>;
///DDRPHYC MR1 register for DDR3
pub mod ddr3_mr1;
/**DDR3_MR2 (rw) register accessor: DDRPHYC MR2 register for DDR3

You can [`read`](crate::Reg::read) this register and get [`ddr3_mr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddr3_mr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDR3_MR2)

For information about available fields see [`mod@ddr3_mr2`] module*/
pub type DDR3_MR2 = crate::Reg<ddr3_mr2::DDR3_MR2rs>;
///DDRPHYC MR2 register for DDR3
pub mod ddr3_mr2;
/**DDR3_MR3 (rw) register accessor: DDRPHYC MR3 register for DDR3

You can [`read`](crate::Reg::read) this register and get [`ddr3_mr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddr3_mr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDR3_MR3)

For information about available fields see [`mod@ddr3_mr3`] module*/
pub type DDR3_MR3 = crate::Reg<ddr3_mr3::DDR3_MR3rs>;
///DDRPHYC MR3 register for DDR3
pub mod ddr3_mr3;
/**ODTCR (rw) register accessor: DDRPHYC ODTC register

You can [`read`](crate::Reg::read) this register and get [`odtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:ODTCR)

For information about available fields see [`mod@odtcr`] module*/
pub type ODTCR = crate::Reg<odtcr::ODTCRrs>;
///DDRPHYC ODTC register
pub mod odtcr;
/**DTAR (rw) register accessor: DDRPHYC DTA register

You can [`read`](crate::Reg::read) this register and get [`dtar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DTAR)

For information about available fields see [`mod@dtar`] module*/
pub type DTAR = crate::Reg<dtar::DTARrs>;
///DDRPHYC DTA register
pub mod dtar;
/**DTDR0 (rw) register accessor: DDRPHYC DTD register 0

You can [`read`](crate::Reg::read) this register and get [`dtdr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtdr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DTDR0)

For information about available fields see [`mod@dtdr0`] module*/
pub type DTDR0 = crate::Reg<dtdr0::DTDR0rs>;
///DDRPHYC DTD register 0
pub mod dtdr0;
/**DTDR1 (rw) register accessor: DDRPHYC DTD register 1

You can [`read`](crate::Reg::read) this register and get [`dtdr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtdr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DTDR1)

For information about available fields see [`mod@dtdr1`] module*/
pub type DTDR1 = crate::Reg<dtdr1::DTDR1rs>;
///DDRPHYC DTD register 1
pub mod dtdr1;
/**GPR0 (rw) register accessor: DDRPHYC general purpose register 0

You can [`read`](crate::Reg::read) this register and get [`gpr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:GPR0)

For information about available fields see [`mod@gpr0`] module*/
pub type GPR0 = crate::Reg<gpr0::GPR0rs>;
///DDRPHYC general purpose register 0
pub mod gpr0;
/**GPR1 (rw) register accessor: DDRPHYC general purpose register 1

You can [`read`](crate::Reg::read) this register and get [`gpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:GPR1)

For information about available fields see [`mod@gpr1`] module*/
pub type GPR1 = crate::Reg<gpr1::GPR1rs>;
///DDRPHYC general purpose register 1
pub mod gpr1;
/**ZQ0CR0 (rw) register accessor: DDRPHYC ZQ0C register 0

You can [`read`](crate::Reg::read) this register and get [`zq0cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zq0cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:ZQ0CR0)

For information about available fields see [`mod@zq0cr0`] module*/
pub type ZQ0CR0 = crate::Reg<zq0cr0::ZQ0CR0rs>;
///DDRPHYC ZQ0C register 0
pub mod zq0cr0;
/**ZQ0CR1 (rw) register accessor: DDRPHYC ZQ0CR1 register

You can [`read`](crate::Reg::read) this register and get [`zq0cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zq0cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:ZQ0CR1)

For information about available fields see [`mod@zq0cr1`] module*/
pub type ZQ0CR1 = crate::Reg<zq0cr1::ZQ0CR1rs>;
///DDRPHYC ZQ0CR1 register
pub mod zq0cr1;
/**ZQ0SR0 (r) register accessor: DDRPHYC ZQ0S register 0

You can [`read`](crate::Reg::read) this register and get [`zq0sr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:ZQ0SR0)

For information about available fields see [`mod@zq0sr0`] module*/
pub type ZQ0SR0 = crate::Reg<zq0sr0::ZQ0SR0rs>;
///DDRPHYC ZQ0S register 0
pub mod zq0sr0;
/**ZQ0SR1 (r) register accessor: DDRPHYC ZQ0S register 1

You can [`read`](crate::Reg::read) this register and get [`zq0sr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:ZQ0SR1)

For information about available fields see [`mod@zq0sr1`] module*/
pub type ZQ0SR1 = crate::Reg<zq0sr1::ZQ0SR1rs>;
///DDRPHYC ZQ0S register 1
pub mod zq0sr1;
/**DX0GCR (rw) register accessor: DDRPHYC byte lane 0 GC register

You can [`read`](crate::Reg::read) this register and get [`dx0gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dx0gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX0GCR)

For information about available fields see [`mod@dx0gcr`] module*/
pub type DX0GCR = crate::Reg<dx0gcr::DX0GCRrs>;
///DDRPHYC byte lane 0 GC register
pub mod dx0gcr;
/**DX0GSR0 (r) register accessor: DDRPHYC byte lane 0 GS register 0

You can [`read`](crate::Reg::read) this register and get [`dx0gsr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX0GSR0)

For information about available fields see [`mod@dx0gsr0`] module*/
pub type DX0GSR0 = crate::Reg<dx0gsr0::DX0GSR0rs>;
///DDRPHYC byte lane 0 GS register 0
pub mod dx0gsr0;
/**DX0GSR1 (r) register accessor: DDRPHYC byte lane 0 GS register 1

You can [`read`](crate::Reg::read) this register and get [`dx0gsr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX0GSR1)

For information about available fields see [`mod@dx0gsr1`] module*/
pub type DX0GSR1 = crate::Reg<dx0gsr1::DX0GSR1rs>;
///DDRPHYC byte lane 0 GS register 1
pub mod dx0gsr1;
/**DX0DLLCR (rw) register accessor: DDRPHYC byte lane 0 DLLC register

You can [`read`](crate::Reg::read) this register and get [`dx0dllcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dx0dllcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX0DLLCR)

For information about available fields see [`mod@dx0dllcr`] module*/
pub type DX0DLLCR = crate::Reg<dx0dllcr::DX0DLLCRrs>;
///DDRPHYC byte lane 0 DLLC register
pub mod dx0dllcr;
/**DX0DQTR (rw) register accessor: DDRPHYC byte lane 0 DQT register

You can [`read`](crate::Reg::read) this register and get [`dx0dqtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dx0dqtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX0DQTR)

For information about available fields see [`mod@dx0dqtr`] module*/
pub type DX0DQTR = crate::Reg<dx0dqtr::DX0DQTRrs>;
///DDRPHYC byte lane 0 DQT register
pub mod dx0dqtr;
/**DX0DQSTR (rw) register accessor: DDRPHYC byte lane 0 DQST register

You can [`read`](crate::Reg::read) this register and get [`dx0dqstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dx0dqstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX0DQSTR)

For information about available fields see [`mod@dx0dqstr`] module*/
pub type DX0DQSTR = crate::Reg<dx0dqstr::DX0DQSTRrs>;
///DDRPHYC byte lane 0 DQST register
pub mod dx0dqstr;
/**DX1GCR (rw) register accessor: DDRPHYC byte lane 1 GC register

You can [`read`](crate::Reg::read) this register and get [`dx1gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dx1gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX1GCR)

For information about available fields see [`mod@dx1gcr`] module*/
pub type DX1GCR = crate::Reg<dx1gcr::DX1GCRrs>;
///DDRPHYC byte lane 1 GC register
pub mod dx1gcr;
/**DX1GSR0 (r) register accessor: DDRPHYC byte lane 1 GS register 0

You can [`read`](crate::Reg::read) this register and get [`dx1gsr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX1GSR0)

For information about available fields see [`mod@dx1gsr0`] module*/
pub type DX1GSR0 = crate::Reg<dx1gsr0::DX1GSR0rs>;
///DDRPHYC byte lane 1 GS register 0
pub mod dx1gsr0;
/**DX1GSR1 (r) register accessor: DDRPHYC byte lane 1 GS register 1

You can [`read`](crate::Reg::read) this register and get [`dx1gsr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX1GSR1)

For information about available fields see [`mod@dx1gsr1`] module*/
pub type DX1GSR1 = crate::Reg<dx1gsr1::DX1GSR1rs>;
///DDRPHYC byte lane 1 GS register 1
pub mod dx1gsr1;
/**DX1DLLCR (rw) register accessor: DDRPHYC byte lane 1 DLLC register

You can [`read`](crate::Reg::read) this register and get [`dx1dllcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dx1dllcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX1DLLCR)

For information about available fields see [`mod@dx1dllcr`] module*/
pub type DX1DLLCR = crate::Reg<dx1dllcr::DX1DLLCRrs>;
///DDRPHYC byte lane 1 DLLC register
pub mod dx1dllcr;
/**DX1DQTR (rw) register accessor: DDRPHYC byte lane 1 DQT register

You can [`read`](crate::Reg::read) this register and get [`dx1dqtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dx1dqtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX1DQTR)

For information about available fields see [`mod@dx1dqtr`] module*/
pub type DX1DQTR = crate::Reg<dx1dqtr::DX1DQTRrs>;
///DDRPHYC byte lane 1 DQT register
pub mod dx1dqtr;
/**DX1DQSTR (rw) register accessor: DDRPHYC byte lane 1 DQST register

You can [`read`](crate::Reg::read) this register and get [`dx1dqstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dx1dqstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX1DQSTR)

For information about available fields see [`mod@dx1dqstr`] module*/
pub type DX1DQSTR = crate::Reg<dx1dqstr::DX1DQSTRrs>;
///DDRPHYC byte lane 1 DQST register
pub mod dx1dqstr;
/**DX2GCR (rw) register accessor: DDRPHYC byte lane 2 GC register

You can [`read`](crate::Reg::read) this register and get [`dx2gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dx2gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX2GCR)

For information about available fields see [`mod@dx2gcr`] module*/
pub type DX2GCR = crate::Reg<dx2gcr::DX2GCRrs>;
///DDRPHYC byte lane 2 GC register
pub mod dx2gcr;
/**DX2GSR0 (r) register accessor: DDRPHYC byte lane 2 GS register 0

You can [`read`](crate::Reg::read) this register and get [`dx2gsr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX2GSR0)

For information about available fields see [`mod@dx2gsr0`] module*/
pub type DX2GSR0 = crate::Reg<dx2gsr0::DX2GSR0rs>;
///DDRPHYC byte lane 2 GS register 0
pub mod dx2gsr0;
/**DX2GSR1 (r) register accessor: DDRPHYC byte lane 2 GS register 1

You can [`read`](crate::Reg::read) this register and get [`dx2gsr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX2GSR1)

For information about available fields see [`mod@dx2gsr1`] module*/
pub type DX2GSR1 = crate::Reg<dx2gsr1::DX2GSR1rs>;
///DDRPHYC byte lane 2 GS register 1
pub mod dx2gsr1;
/**DX2DLLCR (rw) register accessor: DDRPHYC byte lane 2 DLLC register

You can [`read`](crate::Reg::read) this register and get [`dx2dllcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dx2dllcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX2DLLCR)

For information about available fields see [`mod@dx2dllcr`] module*/
pub type DX2DLLCR = crate::Reg<dx2dllcr::DX2DLLCRrs>;
///DDRPHYC byte lane 2 DLLC register
pub mod dx2dllcr;
/**DX2DQTR (rw) register accessor: DDRPHYC byte lane 2 DQT register

You can [`read`](crate::Reg::read) this register and get [`dx2dqtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dx2dqtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX2DQTR)

For information about available fields see [`mod@dx2dqtr`] module*/
pub type DX2DQTR = crate::Reg<dx2dqtr::DX2DQTRrs>;
///DDRPHYC byte lane 2 DQT register
pub mod dx2dqtr;
/**DX2DQSTR (rw) register accessor: DDRPHYC byte lane 2 DQST register

You can [`read`](crate::Reg::read) this register and get [`dx2dqstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dx2dqstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX2DQSTR)

For information about available fields see [`mod@dx2dqstr`] module*/
pub type DX2DQSTR = crate::Reg<dx2dqstr::DX2DQSTRrs>;
///DDRPHYC byte lane 2 DQST register
pub mod dx2dqstr;
/**DX3GCR (rw) register accessor: DDRPHYC byte lane 3 GC register

You can [`read`](crate::Reg::read) this register and get [`dx3gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dx3gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX3GCR)

For information about available fields see [`mod@dx3gcr`] module*/
pub type DX3GCR = crate::Reg<dx3gcr::DX3GCRrs>;
///DDRPHYC byte lane 3 GC register
pub mod dx3gcr;
/**DX3GSR0 (r) register accessor: DDRPHYC byte lane 3 GS register 0

You can [`read`](crate::Reg::read) this register and get [`dx3gsr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX3GSR0)

For information about available fields see [`mod@dx3gsr0`] module*/
pub type DX3GSR0 = crate::Reg<dx3gsr0::DX3GSR0rs>;
///DDRPHYC byte lane 3 GS register 0
pub mod dx3gsr0;
/**DX3GSR1 (r) register accessor: DDRPHYC byte lane 3 GS register 1

You can [`read`](crate::Reg::read) this register and get [`dx3gsr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX3GSR1)

For information about available fields see [`mod@dx3gsr1`] module*/
pub type DX3GSR1 = crate::Reg<dx3gsr1::DX3GSR1rs>;
///DDRPHYC byte lane 3 GS register 1
pub mod dx3gsr1;
/**DX3DLLCR (rw) register accessor: DDRPHYC byte lane 3 DLLC register

You can [`read`](crate::Reg::read) this register and get [`dx3dllcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dx3dllcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX3DLLCR)

For information about available fields see [`mod@dx3dllcr`] module*/
pub type DX3DLLCR = crate::Reg<dx3dllcr::DX3DLLCRrs>;
///DDRPHYC byte lane 3 DLLC register
pub mod dx3dllcr;
/**DX3DQTR (rw) register accessor: DDRPHYC byte lane 3 DQT register

You can [`read`](crate::Reg::read) this register and get [`dx3dqtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dx3dqtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX3DQTR)

For information about available fields see [`mod@dx3dqtr`] module*/
pub type DX3DQTR = crate::Reg<dx3dqtr::DX3DQTRrs>;
///DDRPHYC byte lane 3 DQT register
pub mod dx3dqtr;
/**DX3DQSTR (rw) register accessor: DDRPHYC byte lane 3 DQST register

You can [`read`](crate::Reg::read) this register and get [`dx3dqstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dx3dqstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX3DQSTR)

For information about available fields see [`mod@dx3dqstr`] module*/
pub type DX3DQSTR = crate::Reg<dx3dqstr::DX3DQSTRrs>;
///DDRPHYC byte lane 3 DQST register
pub mod dx3dqstr;
