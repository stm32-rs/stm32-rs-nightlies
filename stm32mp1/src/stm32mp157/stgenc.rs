#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cntcr: CNTCR,
    cntsr: CNTSR,
    cntcvl: CNTCVL,
    cntcvu: CNTCVU,
    _reserved4: [u8; 0x10],
    cntfid0: CNTFID0,
    _reserved5: [u8; 0x0fac],
    pidr4: PIDR4,
    pidr5: PIDR5,
    pidr6: PIDR6,
    pidr7: PIDR7,
    pidr0: PIDR0,
    pidr1: PIDR1,
    pidr2: PIDR2,
    pidr3: PIDR3,
    cidr0: CIDR0,
    cidr1: CIDR1,
    cidr2: CIDR2,
    cidr3: CIDR3,
}
impl RegisterBlock {
    ///0x00 - STGENC control register
    #[inline(always)]
    pub const fn cntcr(&self) -> &CNTCR {
        &self.cntcr
    }
    ///0x04 - STGENC status register
    #[inline(always)]
    pub const fn cntsr(&self) -> &CNTSR {
        &self.cntsr
    }
    ///0x08 - the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.
    #[inline(always)]
    pub const fn cntcvl(&self) -> &CNTCVL {
        &self.cntcvl
    }
    ///0x0c - the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.
    #[inline(always)]
    pub const fn cntcvu(&self) -> &CNTCVU {
        &self.cntcvu
    }
    ///0x20 - the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.
    #[inline(always)]
    pub const fn cntfid0(&self) -> &CNTFID0 {
        &self.cntfid0
    }
    ///0xfd0 - STGENC peripheral ID4 register
    #[inline(always)]
    pub const fn pidr4(&self) -> &PIDR4 {
        &self.pidr4
    }
    ///0xfd4 - STGENC peripheral ID5 register
    #[inline(always)]
    pub const fn pidr5(&self) -> &PIDR5 {
        &self.pidr5
    }
    ///0xfd8 - STGENC peripheral ID6 register
    #[inline(always)]
    pub const fn pidr6(&self) -> &PIDR6 {
        &self.pidr6
    }
    ///0xfdc - STGENC peripheral ID7 register
    #[inline(always)]
    pub const fn pidr7(&self) -> &PIDR7 {
        &self.pidr7
    }
    ///0xfe0 - STGENC peripheral ID0 register
    #[inline(always)]
    pub const fn pidr0(&self) -> &PIDR0 {
        &self.pidr0
    }
    ///0xfe4 - STGENC peripheral ID1 register
    #[inline(always)]
    pub const fn pidr1(&self) -> &PIDR1 {
        &self.pidr1
    }
    ///0xfe8 - STGENC peripheral ID2 register
    #[inline(always)]
    pub const fn pidr2(&self) -> &PIDR2 {
        &self.pidr2
    }
    ///0xfec - STGENC peripheral ID3 register
    #[inline(always)]
    pub const fn pidr3(&self) -> &PIDR3 {
        &self.pidr3
    }
    ///0xff0 - STGENC component ID0 register
    #[inline(always)]
    pub const fn cidr0(&self) -> &CIDR0 {
        &self.cidr0
    }
    ///0xff4 - STGENC component ID1 register
    #[inline(always)]
    pub const fn cidr1(&self) -> &CIDR1 {
        &self.cidr1
    }
    ///0xff8 - STGENC component ID2 register
    #[inline(always)]
    pub const fn cidr2(&self) -> &CIDR2 {
        &self.cidr2
    }
    ///0xffc - STGENC component ID3 register
    #[inline(always)]
    pub const fn cidr3(&self) -> &CIDR3 {
        &self.cidr3
    }
}
/**CNTCR (rw) register accessor: STGENC control register

You can [`read`](crate::Reg::read) this register and get [`cntcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:CNTCR)

For information about available fields see [`mod@cntcr`] module*/
pub type CNTCR = crate::Reg<cntcr::CNTCRrs>;
///STGENC control register
pub mod cntcr;
/**CNTSR (r) register accessor: STGENC status register

You can [`read`](crate::Reg::read) this register and get [`cntsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:CNTSR)

For information about available fields see [`mod@cntsr`] module*/
pub type CNTSR = crate::Reg<cntsr::CNTSRrs>;
///STGENC status register
pub mod cntsr;
/**CNTCVL (rw) register accessor: the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.

You can [`read`](crate::Reg::read) this register and get [`cntcvl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntcvl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:CNTCVL)

For information about available fields see [`mod@cntcvl`] module*/
pub type CNTCVL = crate::Reg<cntcvl::CNTCVLrs>;
///the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.
pub mod cntcvl;
/**CNTCVU (rw) register accessor: the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.

You can [`read`](crate::Reg::read) this register and get [`cntcvu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntcvu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:CNTCVU)

For information about available fields see [`mod@cntcvu`] module*/
pub type CNTCVU = crate::Reg<cntcvu::CNTCVUrs>;
///the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.
pub mod cntcvu;
/**CNTFID0 (rw) register accessor: the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.

You can [`read`](crate::Reg::read) this register and get [`cntfid0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntfid0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:CNTFID0)

For information about available fields see [`mod@cntfid0`] module*/
pub type CNTFID0 = crate::Reg<cntfid0::CNTFID0rs>;
///the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.
pub mod cntfid0;
/**PIDR4 (r) register accessor: STGENC peripheral ID4 register

You can [`read`](crate::Reg::read) this register and get [`pidr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:PIDR4)

For information about available fields see [`mod@pidr4`] module*/
pub type PIDR4 = crate::Reg<pidr4::PIDR4rs>;
///STGENC peripheral ID4 register
pub mod pidr4;
/**PIDR5 (r) register accessor: STGENC peripheral ID5 register

You can [`read`](crate::Reg::read) this register and get [`pidr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:PIDR5)

For information about available fields see [`mod@pidr5`] module*/
pub type PIDR5 = crate::Reg<pidr5::PIDR5rs>;
///STGENC peripheral ID5 register
pub mod pidr5;
/**PIDR6 (r) register accessor: STGENC peripheral ID6 register

You can [`read`](crate::Reg::read) this register and get [`pidr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:PIDR6)

For information about available fields see [`mod@pidr6`] module*/
pub type PIDR6 = crate::Reg<pidr6::PIDR6rs>;
///STGENC peripheral ID6 register
pub mod pidr6;
/**PIDR7 (r) register accessor: STGENC peripheral ID7 register

You can [`read`](crate::Reg::read) this register and get [`pidr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:PIDR7)

For information about available fields see [`mod@pidr7`] module*/
pub type PIDR7 = crate::Reg<pidr7::PIDR7rs>;
///STGENC peripheral ID7 register
pub mod pidr7;
/**PIDR0 (r) register accessor: STGENC peripheral ID0 register

You can [`read`](crate::Reg::read) this register and get [`pidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:PIDR0)

For information about available fields see [`mod@pidr0`] module*/
pub type PIDR0 = crate::Reg<pidr0::PIDR0rs>;
///STGENC peripheral ID0 register
pub mod pidr0;
/**PIDR1 (r) register accessor: STGENC peripheral ID1 register

You can [`read`](crate::Reg::read) this register and get [`pidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:PIDR1)

For information about available fields see [`mod@pidr1`] module*/
pub type PIDR1 = crate::Reg<pidr1::PIDR1rs>;
///STGENC peripheral ID1 register
pub mod pidr1;
/**PIDR2 (r) register accessor: STGENC peripheral ID2 register

You can [`read`](crate::Reg::read) this register and get [`pidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:PIDR2)

For information about available fields see [`mod@pidr2`] module*/
pub type PIDR2 = crate::Reg<pidr2::PIDR2rs>;
///STGENC peripheral ID2 register
pub mod pidr2;
/**PIDR3 (r) register accessor: STGENC peripheral ID3 register

You can [`read`](crate::Reg::read) this register and get [`pidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:PIDR3)

For information about available fields see [`mod@pidr3`] module*/
pub type PIDR3 = crate::Reg<pidr3::PIDR3rs>;
///STGENC peripheral ID3 register
pub mod pidr3;
/**CIDR0 (r) register accessor: STGENC component ID0 register

You can [`read`](crate::Reg::read) this register and get [`cidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:CIDR0)

For information about available fields see [`mod@cidr0`] module*/
pub type CIDR0 = crate::Reg<cidr0::CIDR0rs>;
///STGENC component ID0 register
pub mod cidr0;
/**CIDR1 (r) register accessor: STGENC component ID1 register

You can [`read`](crate::Reg::read) this register and get [`cidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:CIDR1)

For information about available fields see [`mod@cidr1`] module*/
pub type CIDR1 = crate::Reg<cidr1::CIDR1rs>;
///STGENC component ID1 register
pub mod cidr1;
/**CIDR2 (r) register accessor: STGENC component ID2 register

You can [`read`](crate::Reg::read) this register and get [`cidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:CIDR2)

For information about available fields see [`mod@cidr2`] module*/
pub type CIDR2 = crate::Reg<cidr2::CIDR2rs>;
///STGENC component ID2 register
pub mod cidr2;
/**CIDR3 (r) register accessor: STGENC component ID3 register

You can [`read`](crate::Reg::read) this register and get [`cidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:CIDR3)

For information about available fields see [`mod@cidr3`] module*/
pub type CIDR3 = crate::Reg<cidr3::CIDR3rs>;
///STGENC component ID3 register
pub mod cidr3;
