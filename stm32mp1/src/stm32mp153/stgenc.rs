#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    stgenc_cntcr: STGENC_CNTCR,
    stgenc_cntsr: STGENC_CNTSR,
    stgenc_cntcvl: STGENC_CNTCVL,
    stgenc_cntcvu: STGENC_CNTCVU,
    _reserved4: [u8; 0x10],
    stgenc_cntfid0: STGENC_CNTFID0,
    _reserved5: [u8; 0x0fac],
    stgenc_pidr4: STGENC_PIDR4,
    stgenc_pidr5: STGENC_PIDR5,
    stgenc_pidr6: STGENC_PIDR6,
    stgenc_pidr7: STGENC_PIDR7,
    stgenc_pidr0: STGENC_PIDR0,
    stgenc_pidr1: STGENC_PIDR1,
    stgenc_pidr2: STGENC_PIDR2,
    stgenc_pidr3: STGENC_PIDR3,
    stgenc_cidr0: STGENC_CIDR0,
    stgenc_cidr1: STGENC_CIDR1,
    stgenc_cidr2: STGENC_CIDR2,
    stgenc_cidr3: STGENC_CIDR3,
}
impl RegisterBlock {
    ///0x00 - STGENC control register
    #[inline(always)]
    pub const fn stgenc_cntcr(&self) -> &STGENC_CNTCR {
        &self.stgenc_cntcr
    }
    ///0x04 - STGENC status register
    #[inline(always)]
    pub const fn stgenc_cntsr(&self) -> &STGENC_CNTSR {
        &self.stgenc_cntsr
    }
    ///0x08 - the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.
    #[inline(always)]
    pub const fn stgenc_cntcvl(&self) -> &STGENC_CNTCVL {
        &self.stgenc_cntcvl
    }
    ///0x0c - the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.
    #[inline(always)]
    pub const fn stgenc_cntcvu(&self) -> &STGENC_CNTCVU {
        &self.stgenc_cntcvu
    }
    ///0x20 - the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.
    #[inline(always)]
    pub const fn stgenc_cntfid0(&self) -> &STGENC_CNTFID0 {
        &self.stgenc_cntfid0
    }
    ///0xfd0 - STGENC peripheral ID4 register
    #[inline(always)]
    pub const fn stgenc_pidr4(&self) -> &STGENC_PIDR4 {
        &self.stgenc_pidr4
    }
    ///0xfd4 - STGENC peripheral ID5 register
    #[inline(always)]
    pub const fn stgenc_pidr5(&self) -> &STGENC_PIDR5 {
        &self.stgenc_pidr5
    }
    ///0xfd8 - STGENC peripheral ID6 register
    #[inline(always)]
    pub const fn stgenc_pidr6(&self) -> &STGENC_PIDR6 {
        &self.stgenc_pidr6
    }
    ///0xfdc - STGENC peripheral ID7 register
    #[inline(always)]
    pub const fn stgenc_pidr7(&self) -> &STGENC_PIDR7 {
        &self.stgenc_pidr7
    }
    ///0xfe0 - STGENC peripheral ID0 register
    #[inline(always)]
    pub const fn stgenc_pidr0(&self) -> &STGENC_PIDR0 {
        &self.stgenc_pidr0
    }
    ///0xfe4 - STGENC peripheral ID1 register
    #[inline(always)]
    pub const fn stgenc_pidr1(&self) -> &STGENC_PIDR1 {
        &self.stgenc_pidr1
    }
    ///0xfe8 - STGENC peripheral ID2 register
    #[inline(always)]
    pub const fn stgenc_pidr2(&self) -> &STGENC_PIDR2 {
        &self.stgenc_pidr2
    }
    ///0xfec - STGENC peripheral ID3 register
    #[inline(always)]
    pub const fn stgenc_pidr3(&self) -> &STGENC_PIDR3 {
        &self.stgenc_pidr3
    }
    ///0xff0 - STGENC component ID0 register
    #[inline(always)]
    pub const fn stgenc_cidr0(&self) -> &STGENC_CIDR0 {
        &self.stgenc_cidr0
    }
    ///0xff4 - STGENC component ID1 register
    #[inline(always)]
    pub const fn stgenc_cidr1(&self) -> &STGENC_CIDR1 {
        &self.stgenc_cidr1
    }
    ///0xff8 - STGENC component ID2 register
    #[inline(always)]
    pub const fn stgenc_cidr2(&self) -> &STGENC_CIDR2 {
        &self.stgenc_cidr2
    }
    ///0xffc - STGENC component ID3 register
    #[inline(always)]
    pub const fn stgenc_cidr3(&self) -> &STGENC_CIDR3 {
        &self.stgenc_cidr3
    }
}
/**STGENC_CNTCR (rw) register accessor: STGENC control register

You can [`read`](crate::Reg::read) this register and get [`stgenc_cntcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stgenc_cntcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:STGENC_CNTCR)

For information about available fields see [`mod@stgenc_cntcr`]
module*/
pub type STGENC_CNTCR = crate::Reg<stgenc_cntcr::STGENC_CNTCRrs>;
///STGENC control register
pub mod stgenc_cntcr;
/**STGENC_CNTSR (r) register accessor: STGENC status register

You can [`read`](crate::Reg::read) this register and get [`stgenc_cntsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:STGENC_CNTSR)

For information about available fields see [`mod@stgenc_cntsr`]
module*/
pub type STGENC_CNTSR = crate::Reg<stgenc_cntsr::STGENC_CNTSRrs>;
///STGENC status register
pub mod stgenc_cntsr;
/**STGENC_CNTCVL (rw) register accessor: the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.

You can [`read`](crate::Reg::read) this register and get [`stgenc_cntcvl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stgenc_cntcvl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:STGENC_CNTCVL)

For information about available fields see [`mod@stgenc_cntcvl`]
module*/
pub type STGENC_CNTCVL = crate::Reg<stgenc_cntcvl::STGENC_CNTCVLrs>;
///the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.
pub mod stgenc_cntcvl;
/**STGENC_CNTCVU (rw) register accessor: the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.

You can [`read`](crate::Reg::read) this register and get [`stgenc_cntcvu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stgenc_cntcvu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:STGENC_CNTCVU)

For information about available fields see [`mod@stgenc_cntcvu`]
module*/
pub type STGENC_CNTCVU = crate::Reg<stgenc_cntcvu::STGENC_CNTCVUrs>;
///the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.
pub mod stgenc_cntcvu;
/**STGENC_CNTFID0 (rw) register accessor: the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.

You can [`read`](crate::Reg::read) this register and get [`stgenc_cntfid0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stgenc_cntfid0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:STGENC_CNTFID0)

For information about available fields see [`mod@stgenc_cntfid0`]
module*/
pub type STGENC_CNTFID0 = crate::Reg<stgenc_cntfid0::STGENC_CNTFID0rs>;
///the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.
pub mod stgenc_cntfid0;
/**STGENC_PIDR4 (r) register accessor: STGENC peripheral ID4 register

You can [`read`](crate::Reg::read) this register and get [`stgenc_pidr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:STGENC_PIDR4)

For information about available fields see [`mod@stgenc_pidr4`]
module*/
pub type STGENC_PIDR4 = crate::Reg<stgenc_pidr4::STGENC_PIDR4rs>;
///STGENC peripheral ID4 register
pub mod stgenc_pidr4;
/**STGENC_PIDR5 (r) register accessor: STGENC peripheral ID5 register

You can [`read`](crate::Reg::read) this register and get [`stgenc_pidr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:STGENC_PIDR5)

For information about available fields see [`mod@stgenc_pidr5`]
module*/
pub type STGENC_PIDR5 = crate::Reg<stgenc_pidr5::STGENC_PIDR5rs>;
///STGENC peripheral ID5 register
pub mod stgenc_pidr5;
/**STGENC_PIDR6 (r) register accessor: STGENC peripheral ID6 register

You can [`read`](crate::Reg::read) this register and get [`stgenc_pidr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:STGENC_PIDR6)

For information about available fields see [`mod@stgenc_pidr6`]
module*/
pub type STGENC_PIDR6 = crate::Reg<stgenc_pidr6::STGENC_PIDR6rs>;
///STGENC peripheral ID6 register
pub mod stgenc_pidr6;
/**STGENC_PIDR7 (r) register accessor: STGENC peripheral ID7 register

You can [`read`](crate::Reg::read) this register and get [`stgenc_pidr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:STGENC_PIDR7)

For information about available fields see [`mod@stgenc_pidr7`]
module*/
pub type STGENC_PIDR7 = crate::Reg<stgenc_pidr7::STGENC_PIDR7rs>;
///STGENC peripheral ID7 register
pub mod stgenc_pidr7;
/**STGENC_PIDR0 (r) register accessor: STGENC peripheral ID0 register

You can [`read`](crate::Reg::read) this register and get [`stgenc_pidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:STGENC_PIDR0)

For information about available fields see [`mod@stgenc_pidr0`]
module*/
pub type STGENC_PIDR0 = crate::Reg<stgenc_pidr0::STGENC_PIDR0rs>;
///STGENC peripheral ID0 register
pub mod stgenc_pidr0;
/**STGENC_PIDR1 (r) register accessor: STGENC peripheral ID1 register

You can [`read`](crate::Reg::read) this register and get [`stgenc_pidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:STGENC_PIDR1)

For information about available fields see [`mod@stgenc_pidr1`]
module*/
pub type STGENC_PIDR1 = crate::Reg<stgenc_pidr1::STGENC_PIDR1rs>;
///STGENC peripheral ID1 register
pub mod stgenc_pidr1;
/**STGENC_PIDR2 (r) register accessor: STGENC peripheral ID2 register

You can [`read`](crate::Reg::read) this register and get [`stgenc_pidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:STGENC_PIDR2)

For information about available fields see [`mod@stgenc_pidr2`]
module*/
pub type STGENC_PIDR2 = crate::Reg<stgenc_pidr2::STGENC_PIDR2rs>;
///STGENC peripheral ID2 register
pub mod stgenc_pidr2;
/**STGENC_PIDR3 (r) register accessor: STGENC peripheral ID3 register

You can [`read`](crate::Reg::read) this register and get [`stgenc_pidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:STGENC_PIDR3)

For information about available fields see [`mod@stgenc_pidr3`]
module*/
pub type STGENC_PIDR3 = crate::Reg<stgenc_pidr3::STGENC_PIDR3rs>;
///STGENC peripheral ID3 register
pub mod stgenc_pidr3;
/**STGENC_CIDR0 (r) register accessor: STGENC component ID0 register

You can [`read`](crate::Reg::read) this register and get [`stgenc_cidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:STGENC_CIDR0)

For information about available fields see [`mod@stgenc_cidr0`]
module*/
pub type STGENC_CIDR0 = crate::Reg<stgenc_cidr0::STGENC_CIDR0rs>;
///STGENC component ID0 register
pub mod stgenc_cidr0;
/**STGENC_CIDR1 (r) register accessor: STGENC component ID1 register

You can [`read`](crate::Reg::read) this register and get [`stgenc_cidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:STGENC_CIDR1)

For information about available fields see [`mod@stgenc_cidr1`]
module*/
pub type STGENC_CIDR1 = crate::Reg<stgenc_cidr1::STGENC_CIDR1rs>;
///STGENC component ID1 register
pub mod stgenc_cidr1;
/**STGENC_CIDR2 (r) register accessor: STGENC component ID2 register

You can [`read`](crate::Reg::read) this register and get [`stgenc_cidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:STGENC_CIDR2)

For information about available fields see [`mod@stgenc_cidr2`]
module*/
pub type STGENC_CIDR2 = crate::Reg<stgenc_cidr2::STGENC_CIDR2rs>;
///STGENC component ID2 register
pub mod stgenc_cidr2;
/**STGENC_CIDR3 (r) register accessor: STGENC component ID3 register

You can [`read`](crate::Reg::read) this register and get [`stgenc_cidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:STGENC_CIDR3)

For information about available fields see [`mod@stgenc_cidr3`]
module*/
pub type STGENC_CIDR3 = crate::Reg<stgenc_cidr3::STGENC_CIDR3rs>;
///STGENC component ID3 register
pub mod stgenc_cidr3;
