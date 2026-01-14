#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    val: VAL,
    _reserved3: [u8; 0x74],
    tcr: TCR,
    itip: ITIP,
    _reserved5: [u8; 0x0f58],
    periph_id0: PERIPH_ID0,
    periph_id1: PERIPH_ID1,
    periph_id2: PERIPH_ID2,
    periph_id3: PERIPH_ID3,
    pcell_id0: PCELL_ID0,
    pcell_id1: PCELL_ID1,
    pcell_id2: PCELL_ID2,
    pcell_id3: PCELL_ID3,
}
impl RegisterBlock {
    ///0x00 - RNG_CR register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - RNG_SR register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x08 - RNG_VAL register
    #[inline(always)]
    pub const fn val(&self) -> &VAL {
        &self.val
    }
    ///0x80 - RNG_TCR register
    #[inline(always)]
    pub const fn tcr(&self) -> &TCR {
        &self.tcr
    }
    ///0x84 - RNG_ITIP register
    #[inline(always)]
    pub const fn itip(&self) -> &ITIP {
        &self.itip
    }
    ///0xfe0 - RNGPeriphID0 register
    #[inline(always)]
    pub const fn periph_id0(&self) -> &PERIPH_ID0 {
        &self.periph_id0
    }
    ///0xfe4 - RNGPeriphID1 register
    #[inline(always)]
    pub const fn periph_id1(&self) -> &PERIPH_ID1 {
        &self.periph_id1
    }
    ///0xfe8 - RNGPeriphID2 register
    #[inline(always)]
    pub const fn periph_id2(&self) -> &PERIPH_ID2 {
        &self.periph_id2
    }
    ///0xfec - RNGPeriphID3 register
    #[inline(always)]
    pub const fn periph_id3(&self) -> &PERIPH_ID3 {
        &self.periph_id3
    }
    ///0xff0 - RNGPCellID0 register
    #[inline(always)]
    pub const fn pcell_id0(&self) -> &PCELL_ID0 {
        &self.pcell_id0
    }
    ///0xff4 - RNGPCellID1 register
    #[inline(always)]
    pub const fn pcell_id1(&self) -> &PCELL_ID1 {
        &self.pcell_id1
    }
    ///0xff8 - RNGPCellID2 register
    #[inline(always)]
    pub const fn pcell_id2(&self) -> &PCELL_ID2 {
        &self.pcell_id2
    }
    ///0xffc - RNGPCellID3 register
    #[inline(always)]
    pub const fn pcell_id3(&self) -> &PCELL_ID3 {
        &self.pcell_id3
    }
}
/**CR (rw) register accessor: RNG_CR register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///RNG_CR register
pub mod cr;
/**SR (rw) register accessor: RNG_SR register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///RNG_SR register
pub mod sr;
/**VAL (r) register accessor: RNG_VAL register

You can [`read`](crate::Reg::read) this register and get [`val::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:VAL)

For information about available fields see [`mod@val`] module*/
pub type VAL = crate::Reg<val::VALrs>;
///RNG_VAL register
pub mod val;
/**TCR (rw) register accessor: RNG_TCR register

You can [`read`](crate::Reg::read) this register and get [`tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:TCR)

For information about available fields see [`mod@tcr`] module*/
pub type TCR = crate::Reg<tcr::TCRrs>;
///RNG_TCR register
pub mod tcr;
/**ITIP (rw) register accessor: RNG_ITIP register

You can [`read`](crate::Reg::read) this register and get [`itip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:ITIP)

For information about available fields see [`mod@itip`] module*/
pub type ITIP = crate::Reg<itip::ITIPrs>;
///RNG_ITIP register
pub mod itip;
/**PeriphID0 (r) register accessor: RNGPeriphID0 register

You can [`read`](crate::Reg::read) this register and get [`periph_id0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:PeriphID0)

For information about available fields see [`mod@periph_id0`] module*/
#[doc(alias = "PeriphID0")]
pub type PERIPH_ID0 = crate::Reg<periph_id0::PERIPH_ID0rs>;
///RNGPeriphID0 register
pub mod periph_id0;
/**PeriphID1 (r) register accessor: RNGPeriphID1 register

You can [`read`](crate::Reg::read) this register and get [`periph_id1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:PeriphID1)

For information about available fields see [`mod@periph_id1`] module*/
#[doc(alias = "PeriphID1")]
pub type PERIPH_ID1 = crate::Reg<periph_id1::PERIPH_ID1rs>;
///RNGPeriphID1 register
pub mod periph_id1;
/**PeriphID2 (r) register accessor: RNGPeriphID2 register

You can [`read`](crate::Reg::read) this register and get [`periph_id2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:PeriphID2)

For information about available fields see [`mod@periph_id2`] module*/
#[doc(alias = "PeriphID2")]
pub type PERIPH_ID2 = crate::Reg<periph_id2::PERIPH_ID2rs>;
///RNGPeriphID2 register
pub mod periph_id2;
/**PeriphID3 (r) register accessor: RNGPeriphID3 register

You can [`read`](crate::Reg::read) this register and get [`periph_id3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:PeriphID3)

For information about available fields see [`mod@periph_id3`] module*/
#[doc(alias = "PeriphID3")]
pub type PERIPH_ID3 = crate::Reg<periph_id3::PERIPH_ID3rs>;
///RNGPeriphID3 register
pub mod periph_id3;
/**PCellID0 (r) register accessor: RNGPCellID0 register

You can [`read`](crate::Reg::read) this register and get [`pcell_id0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:PCellID0)

For information about available fields see [`mod@pcell_id0`] module*/
#[doc(alias = "PCellID0")]
pub type PCELL_ID0 = crate::Reg<pcell_id0::PCELL_ID0rs>;
///RNGPCellID0 register
pub mod pcell_id0;
/**PCellID1 (r) register accessor: RNGPCellID1 register

You can [`read`](crate::Reg::read) this register and get [`pcell_id1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:PCellID1)

For information about available fields see [`mod@pcell_id1`] module*/
#[doc(alias = "PCellID1")]
pub type PCELL_ID1 = crate::Reg<pcell_id1::PCELL_ID1rs>;
///RNGPCellID1 register
pub mod pcell_id1;
/**PCellID2 (r) register accessor: RNGPCellID2 register

You can [`read`](crate::Reg::read) this register and get [`pcell_id2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:PCellID2)

For information about available fields see [`mod@pcell_id2`] module*/
#[doc(alias = "PCellID2")]
pub type PCELL_ID2 = crate::Reg<pcell_id2::PCELL_ID2rs>;
///RNGPCellID2 register
pub mod pcell_id2;
/**PCellID3 (r) register accessor: RNGPCellID3 register

You can [`read`](crate::Reg::read) this register and get [`pcell_id3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:PCellID3)

For information about available fields see [`mod@pcell_id3`] module*/
#[doc(alias = "PCellID3")]
pub type PCELL_ID3 = crate::Reg<pcell_id3::PCELL_ID3rs>;
///RNGPCellID3 register
pub mod pcell_id3;
