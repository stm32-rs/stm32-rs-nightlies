#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    swtrgr: SWTRGR,
    dhr12r1: DHR12R1,
    dhr12l1: DHR12L1,
    dhr8r1: DHR8R1,
    dhr12r2: DHR12R2,
    dhr12l2: DHR12L2,
    dhr8r2: DHR8R2,
    dhr12rd: DHR12RD,
    dhr12ld: DHR12LD,
    dhr8rd: DHR8RD,
    dor1: DOR1,
    dor2: DOR2,
    sr: SR,
    ccr: CCR,
    mcr: MCR,
    shsr1: SHSR1,
    shsr2: SHSR2,
    shhr: SHHR,
    shrr: SHRR,
    _reserved20: [u8; 0x03a0],
    hwcfgr0: HWCFGR0,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00 - DAC control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - DAC software trigger register
    #[inline(always)]
    pub const fn swtrgr(&self) -> &SWTRGR {
        &self.swtrgr
    }
    ///0x08 - DAC channel1 12-bit right-aligned data holding register
    #[inline(always)]
    pub const fn dhr12r1(&self) -> &DHR12R1 {
        &self.dhr12r1
    }
    ///0x0c - DAC channel1 12-bit left aligned data holding register
    #[inline(always)]
    pub const fn dhr12l1(&self) -> &DHR12L1 {
        &self.dhr12l1
    }
    ///0x10 - DAC channel1 8-bit right aligned data holding register
    #[inline(always)]
    pub const fn dhr8r1(&self) -> &DHR8R1 {
        &self.dhr8r1
    }
    ///0x14 - This register is available only on dual-channel DACs. Refer to Section29.3: DAC implementation.
    #[inline(always)]
    pub const fn dhr12r2(&self) -> &DHR12R2 {
        &self.dhr12r2
    }
    ///0x18 - This register is available only on dual-channel DACs. Refer to Section29.3: DAC implementation.
    #[inline(always)]
    pub const fn dhr12l2(&self) -> &DHR12L2 {
        &self.dhr12l2
    }
    ///0x1c - This register is available only on dual-channel DACs. Refer to Section29.3: DAC implementation.
    #[inline(always)]
    pub const fn dhr8r2(&self) -> &DHR8R2 {
        &self.dhr8r2
    }
    ///0x20 - Dual DAC 12-bit right-aligned data holding register
    #[inline(always)]
    pub const fn dhr12rd(&self) -> &DHR12RD {
        &self.dhr12rd
    }
    ///0x24 - Dual DAC 12-bit left aligned data holding register
    #[inline(always)]
    pub const fn dhr12ld(&self) -> &DHR12LD {
        &self.dhr12ld
    }
    ///0x28 - Dual DAC 8-bit right aligned data holding register
    #[inline(always)]
    pub const fn dhr8rd(&self) -> &DHR8RD {
        &self.dhr8rd
    }
    ///0x2c - DAC channel1 data output register
    #[inline(always)]
    pub const fn dor1(&self) -> &DOR1 {
        &self.dor1
    }
    ///0x30 - This register is available only on dual-channel DACs. Refer to Section29.3: DAC implementation.
    #[inline(always)]
    pub const fn dor2(&self) -> &DOR2 {
        &self.dor2
    }
    ///0x34 - DAC status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x38 - DAC calibration control register
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    ///0x3c - DAC mode control register
    #[inline(always)]
    pub const fn mcr(&self) -> &MCR {
        &self.mcr
    }
    ///0x40 - DAC channel 1 sample and hold sample time register
    #[inline(always)]
    pub const fn shsr1(&self) -> &SHSR1 {
        &self.shsr1
    }
    ///0x44 - This register is available only on dual-channel DACs. Refer to Section29.3: DAC implementation.
    #[inline(always)]
    pub const fn shsr2(&self) -> &SHSR2 {
        &self.shsr2
    }
    ///0x48 - DAC sample and hold time register
    #[inline(always)]
    pub const fn shhr(&self) -> &SHHR {
        &self.shhr
    }
    ///0x4c - DAC sample and hold refresh time register
    #[inline(always)]
    pub const fn shrr(&self) -> &SHRR {
        &self.shrr
    }
    ///0x3f0 - DAC IP hardware configuration register
    #[inline(always)]
    pub const fn hwcfgr0(&self) -> &HWCFGR0 {
        &self.hwcfgr0
    }
    ///0x3f4 - No
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - No
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0x3fc - No
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**CR (rw) register accessor: DAC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///DAC control register
pub mod cr;
/**SWTRGR (w) register accessor: DAC software trigger register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrgr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:SWTRGR)

For information about available fields see [`mod@swtrgr`] module*/
pub type SWTRGR = crate::Reg<swtrgr::SWTRGRrs>;
///DAC software trigger register
pub mod swtrgr;
/**DHR12R1 (rw) register accessor: DAC channel1 12-bit right-aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:DHR12R1)

For information about available fields see [`mod@dhr12r1`] module*/
pub type DHR12R1 = crate::Reg<dhr12r1::DHR12R1rs>;
///DAC channel1 12-bit right-aligned data holding register
pub mod dhr12r1;
/**DHR12L1 (rw) register accessor: DAC channel1 12-bit left aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12l1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12l1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:DHR12L1)

For information about available fields see [`mod@dhr12l1`] module*/
pub type DHR12L1 = crate::Reg<dhr12l1::DHR12L1rs>;
///DAC channel1 12-bit left aligned data holding register
pub mod dhr12l1;
/**DHR8R1 (rw) register accessor: DAC channel1 8-bit right aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr8r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:DHR8R1)

For information about available fields see [`mod@dhr8r1`] module*/
pub type DHR8R1 = crate::Reg<dhr8r1::DHR8R1rs>;
///DAC channel1 8-bit right aligned data holding register
pub mod dhr8r1;
/**DHR12R2 (rw) register accessor: This register is available only on dual-channel DACs. Refer to Section29.3: DAC implementation.

You can [`read`](crate::Reg::read) this register and get [`dhr12r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:DHR12R2)

For information about available fields see [`mod@dhr12r2`] module*/
pub type DHR12R2 = crate::Reg<dhr12r2::DHR12R2rs>;
///This register is available only on dual-channel DACs. Refer to Section29.3: DAC implementation.
pub mod dhr12r2;
/**DHR12L2 (rw) register accessor: This register is available only on dual-channel DACs. Refer to Section29.3: DAC implementation.

You can [`read`](crate::Reg::read) this register and get [`dhr12l2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12l2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:DHR12L2)

For information about available fields see [`mod@dhr12l2`] module*/
pub type DHR12L2 = crate::Reg<dhr12l2::DHR12L2rs>;
///This register is available only on dual-channel DACs. Refer to Section29.3: DAC implementation.
pub mod dhr12l2;
/**DHR8R2 (rw) register accessor: This register is available only on dual-channel DACs. Refer to Section29.3: DAC implementation.

You can [`read`](crate::Reg::read) this register and get [`dhr8r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:DHR8R2)

For information about available fields see [`mod@dhr8r2`] module*/
pub type DHR8R2 = crate::Reg<dhr8r2::DHR8R2rs>;
///This register is available only on dual-channel DACs. Refer to Section29.3: DAC implementation.
pub mod dhr8r2;
/**DHR12RD (rw) register accessor: Dual DAC 12-bit right-aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12rd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12rd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:DHR12RD)

For information about available fields see [`mod@dhr12rd`] module*/
pub type DHR12RD = crate::Reg<dhr12rd::DHR12RDrs>;
///Dual DAC 12-bit right-aligned data holding register
pub mod dhr12rd;
/**DHR12LD (rw) register accessor: Dual DAC 12-bit left aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12ld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12ld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:DHR12LD)

For information about available fields see [`mod@dhr12ld`] module*/
pub type DHR12LD = crate::Reg<dhr12ld::DHR12LDrs>;
///Dual DAC 12-bit left aligned data holding register
pub mod dhr12ld;
/**DHR8RD (rw) register accessor: Dual DAC 8-bit right aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr8rd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8rd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:DHR8RD)

For information about available fields see [`mod@dhr8rd`] module*/
pub type DHR8RD = crate::Reg<dhr8rd::DHR8RDrs>;
///Dual DAC 8-bit right aligned data holding register
pub mod dhr8rd;
/**DOR1 (r) register accessor: DAC channel1 data output register

You can [`read`](crate::Reg::read) this register and get [`dor1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:DOR1)

For information about available fields see [`mod@dor1`] module*/
pub type DOR1 = crate::Reg<dor1::DOR1rs>;
///DAC channel1 data output register
pub mod dor1;
/**DOR2 (r) register accessor: This register is available only on dual-channel DACs. Refer to Section29.3: DAC implementation.

You can [`read`](crate::Reg::read) this register and get [`dor2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:DOR2)

For information about available fields see [`mod@dor2`] module*/
pub type DOR2 = crate::Reg<dor2::DOR2rs>;
///This register is available only on dual-channel DACs. Refer to Section29.3: DAC implementation.
pub mod dor2;
/**SR (rw) register accessor: DAC status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///DAC status register
pub mod sr;
/**CCR (rw) register accessor: DAC calibration control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:CCR)

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///DAC calibration control register
pub mod ccr;
/**MCR (rw) register accessor: DAC mode control register

You can [`read`](crate::Reg::read) this register and get [`mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:MCR)

For information about available fields see [`mod@mcr`] module*/
pub type MCR = crate::Reg<mcr::MCRrs>;
///DAC mode control register
pub mod mcr;
/**SHSR1 (rw) register accessor: DAC channel 1 sample and hold sample time register

You can [`read`](crate::Reg::read) this register and get [`shsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:SHSR1)

For information about available fields see [`mod@shsr1`] module*/
pub type SHSR1 = crate::Reg<shsr1::SHSR1rs>;
///DAC channel 1 sample and hold sample time register
pub mod shsr1;
/**SHSR2 (rw) register accessor: This register is available only on dual-channel DACs. Refer to Section29.3: DAC implementation.

You can [`read`](crate::Reg::read) this register and get [`shsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:SHSR2)

For information about available fields see [`mod@shsr2`] module*/
pub type SHSR2 = crate::Reg<shsr2::SHSR2rs>;
///This register is available only on dual-channel DACs. Refer to Section29.3: DAC implementation.
pub mod shsr2;
/**SHHR (rw) register accessor: DAC sample and hold time register

You can [`read`](crate::Reg::read) this register and get [`shhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:SHHR)

For information about available fields see [`mod@shhr`] module*/
pub type SHHR = crate::Reg<shhr::SHHRrs>;
///DAC sample and hold time register
pub mod shhr;
/**SHRR (rw) register accessor: DAC sample and hold refresh time register

You can [`read`](crate::Reg::read) this register and get [`shrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:SHRR)

For information about available fields see [`mod@shrr`] module*/
pub type SHRR = crate::Reg<shrr::SHRRrs>;
///DAC sample and hold refresh time register
pub mod shrr;
/**HWCFGR0 (r) register accessor: DAC IP hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:HWCFGR0)

For information about available fields see [`mod@hwcfgr0`] module*/
pub type HWCFGR0 = crate::Reg<hwcfgr0::HWCFGR0rs>;
///DAC IP hardware configuration register
pub mod hwcfgr0;
/**VERR (r) register accessor: No

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///No
pub mod verr;
/**IPIDR (r) register accessor: No

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///No
pub mod ipidr;
/**SIDR (r) register accessor: No

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///No
pub mod sidr;
