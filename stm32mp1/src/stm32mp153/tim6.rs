#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    _reserved1: [u8; 0x02],
    cr2: CR2,
    smcr: SMCR,
    dier: DIER,
    _reserved4: [u8; 0x02],
    sr: SR,
    egr: EGR,
    _reserved6: [u8; 0x02],
    ccmr1alternate6: CCMR1ALTERNATE6,
    ccmr2alternate22: CCMR2ALTERNATE22,
    ccer: CCER,
    cnt: CNT,
    psc: PSC,
    _reserved11: [u8; 0x02],
    arr: ARR,
    _reserved12: [u8; 0x02],
    rcr: RCR,
    _reserved13: [u8; 0x02],
    ccr1: CCR1,
    _reserved14: [u8; 0x02],
    ccr2: CCR2,
    _reserved15: [u8; 0x02],
    ccr3: CCR3,
    _reserved16: [u8; 0x02],
    ccr4: CCR4,
    _reserved17: [u8; 0x02],
    bdtr: BDTR,
    dcr: DCR,
    _reserved19: [u8; 0x02],
    dmar: DMAR,
    _reserved20: [u8; 0x04],
    ccmr3: CCMR3,
    ccr5: CCR5,
    ccr6: CCR6,
}
impl RegisterBlock {
    ///0x00 - TIM6 control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - TIM6 control register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - TIM6 slave mode control register
    #[inline(always)]
    pub const fn smcr(&self) -> &SMCR {
        &self.smcr
    }
    ///0x0c - TIM6 DMA/interrupt enable register
    #[inline(always)]
    pub const fn dier(&self) -> &DIER {
        &self.dier
    }
    ///0x10 - TIM6 status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x14 - TIM6 event generation register
    #[inline(always)]
    pub const fn egr(&self) -> &EGR {
        &self.egr
    }
    ///0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    #[inline(always)]
    pub const fn ccmr1alternate6(&self) -> &CCMR1ALTERNATE6 {
        &self.ccmr1alternate6
    }
    ///0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    #[inline(always)]
    pub const fn ccmr2alternate22(&self) -> &CCMR2ALTERNATE22 {
        &self.ccmr2alternate22
    }
    ///0x20 - TIM6 capture/compare enable register
    #[inline(always)]
    pub const fn ccer(&self) -> &CCER {
        &self.ccer
    }
    ///0x24 - TIM6 counter
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        &self.cnt
    }
    ///0x28 - TIM6 prescaler
    #[inline(always)]
    pub const fn psc(&self) -> &PSC {
        &self.psc
    }
    ///0x2c - TIM6 auto-reload register
    #[inline(always)]
    pub const fn arr(&self) -> &ARR {
        &self.arr
    }
    ///0x30 - TIM6 repetition counter register
    #[inline(always)]
    pub const fn rcr(&self) -> &RCR {
        &self.rcr
    }
    ///0x34 - TIM6 capture/compare register 1
    #[inline(always)]
    pub const fn ccr1(&self) -> &CCR1 {
        &self.ccr1
    }
    ///0x38 - TIM6 capture/compare register 2
    #[inline(always)]
    pub const fn ccr2(&self) -> &CCR2 {
        &self.ccr2
    }
    ///0x3c - TIM6 capture/compare register 3
    #[inline(always)]
    pub const fn ccr3(&self) -> &CCR3 {
        &self.ccr3
    }
    ///0x40 - TIM6 capture/compare register 4
    #[inline(always)]
    pub const fn ccr4(&self) -> &CCR4 {
        &self.ccr4
    }
    ///0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\] can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.
    #[inline(always)]
    pub const fn bdtr(&self) -> &BDTR {
        &self.bdtr
    }
    ///0x48 - TIM6 DMA control register
    #[inline(always)]
    pub const fn dcr(&self) -> &DCR {
        &self.dcr
    }
    ///0x4c - TIM6 DMA address for full transfer
    #[inline(always)]
    pub const fn dmar(&self) -> &DMAR {
        &self.dmar
    }
    ///0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:
    #[inline(always)]
    pub const fn ccmr3(&self) -> &CCMR3 {
        &self.ccmr3
    }
    ///0x58 - TIM6 capture/compare register 5
    #[inline(always)]
    pub const fn ccr5(&self) -> &CCR5 {
        &self.ccr5
    }
    ///0x5c - TIM6 capture/compare register 6
    #[inline(always)]
    pub const fn ccr6(&self) -> &CCR6 {
        &self.ccr6
    }
}
/**CR1 (rw) register accessor: TIM6 control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM6:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///TIM6 control register 1
pub mod cr1;
/**CR2 (rw) register accessor: TIM6 control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM6:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///TIM6 control register 2
pub mod cr2;
/**SMCR (rw) register accessor: TIM6 slave mode control register

You can [`read`](crate::Reg::read) this register and get [`smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM6:SMCR)

For information about available fields see [`mod@smcr`] module*/
pub type SMCR = crate::Reg<smcr::SMCRrs>;
///TIM6 slave mode control register
pub mod smcr;
/**DIER (rw) register accessor: TIM6 DMA/interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM6:DIER)

For information about available fields see [`mod@dier`] module*/
pub type DIER = crate::Reg<dier::DIERrs>;
///TIM6 DMA/interrupt enable register
pub mod dier;
/**SR (rw) register accessor: TIM6 status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM6:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///TIM6 status register
pub mod sr;
/**EGR (w) register accessor: TIM6 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM6:EGR)

For information about available fields see [`mod@egr`] module*/
pub type EGR = crate::Reg<egr::EGRrs>;
///TIM6 event generation register
pub mod egr;
/**CCMR1ALTERNATE6 (rw) register accessor: The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:

You can [`read`](crate::Reg::read) this register and get [`ccmr1alternate6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1alternate6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM6:CCMR1ALTERNATE6)

For information about available fields see [`mod@ccmr1alternate6`] module*/
pub type CCMR1ALTERNATE6 = crate::Reg<ccmr1alternate6::CCMR1ALTERNATE6rs>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod ccmr1alternate6;
/**CCMR2ALTERNATE22 (rw) register accessor: The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:

You can [`read`](crate::Reg::read) this register and get [`ccmr2alternate22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2alternate22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM6:CCMR2ALTERNATE22)

For information about available fields see [`mod@ccmr2alternate22`] module*/
pub type CCMR2ALTERNATE22 = crate::Reg<ccmr2alternate22::CCMR2ALTERNATE22rs>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod ccmr2alternate22;
/**CCER (rw) register accessor: TIM6 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM6:CCER)

For information about available fields see [`mod@ccer`] module*/
pub type CCER = crate::Reg<ccer::CCERrs>;
///TIM6 capture/compare enable register
pub mod ccer;
/**CNT (rw) register accessor: TIM6 counter

You can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM6:CNT)

For information about available fields see [`mod@cnt`] module*/
pub type CNT = crate::Reg<cnt::CNTrs>;
///TIM6 counter
pub mod cnt;
/**PSC (rw) register accessor: TIM6 prescaler

You can [`read`](crate::Reg::read) this register and get [`psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM6:PSC)

For information about available fields see [`mod@psc`] module*/
pub type PSC = crate::Reg<psc::PSCrs>;
///TIM6 prescaler
pub mod psc;
/**ARR (rw) register accessor: TIM6 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM6:ARR)

For information about available fields see [`mod@arr`] module*/
pub type ARR = crate::Reg<arr::ARRrs>;
///TIM6 auto-reload register
pub mod arr;
/**RCR (rw) register accessor: TIM6 repetition counter register

You can [`read`](crate::Reg::read) this register and get [`rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM6:RCR)

For information about available fields see [`mod@rcr`] module*/
pub type RCR = crate::Reg<rcr::RCRrs>;
///TIM6 repetition counter register
pub mod rcr;
/**CCR1 (rw) register accessor: TIM6 capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM6:CCR1)

For information about available fields see [`mod@ccr1`] module*/
pub type CCR1 = crate::Reg<ccr1::CCR1rs>;
///TIM6 capture/compare register 1
pub mod ccr1;
/**CCR2 (rw) register accessor: TIM6 capture/compare register 2

You can [`read`](crate::Reg::read) this register and get [`ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM6:CCR2)

For information about available fields see [`mod@ccr2`] module*/
pub type CCR2 = crate::Reg<ccr2::CCR2rs>;
///TIM6 capture/compare register 2
pub mod ccr2;
/**CCR3 (rw) register accessor: TIM6 capture/compare register 3

You can [`read`](crate::Reg::read) this register and get [`ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM6:CCR3)

For information about available fields see [`mod@ccr3`] module*/
pub type CCR3 = crate::Reg<ccr3::CCR3rs>;
///TIM6 capture/compare register 3
pub mod ccr3;
/**CCR4 (rw) register accessor: TIM6 capture/compare register 4

You can [`read`](crate::Reg::read) this register and get [`ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM6:CCR4)

For information about available fields see [`mod@ccr4`] module*/
pub type CCR4 = crate::Reg<ccr4::CCR4rs>;
///TIM6 capture/compare register 4
pub mod ccr4;
/**BDTR (rw) register accessor: As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\] can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.

You can [`read`](crate::Reg::read) this register and get [`bdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM6:BDTR)

For information about available fields see [`mod@bdtr`] module*/
pub type BDTR = crate::Reg<bdtr::BDTRrs>;
///As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\] can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.
pub mod bdtr;
/**DCR (rw) register accessor: TIM6 DMA control register

You can [`read`](crate::Reg::read) this register and get [`dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM6:DCR)

For information about available fields see [`mod@dcr`] module*/
pub type DCR = crate::Reg<dcr::DCRrs>;
///TIM6 DMA control register
pub mod dcr;
/**DMAR (rw) register accessor: TIM6 DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`dmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM6:DMAR)

For information about available fields see [`mod@dmar`] module*/
pub type DMAR = crate::Reg<dmar::DMARrs>;
///TIM6 DMA address for full transfer
pub mod dmar;
/**CCMR3 (rw) register accessor: The channels 5 and 6 can only be configured in output. Output compare mode:

You can [`read`](crate::Reg::read) this register and get [`ccmr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM6:CCMR3)

For information about available fields see [`mod@ccmr3`] module*/
pub type CCMR3 = crate::Reg<ccmr3::CCMR3rs>;
///The channels 5 and 6 can only be configured in output. Output compare mode:
pub mod ccmr3;
/**CCR5 (rw) register accessor: TIM6 capture/compare register 5

You can [`read`](crate::Reg::read) this register and get [`ccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM6:CCR5)

For information about available fields see [`mod@ccr5`] module*/
pub type CCR5 = crate::Reg<ccr5::CCR5rs>;
///TIM6 capture/compare register 5
pub mod ccr5;
/**CCR6 (rw) register accessor: TIM6 capture/compare register 6

You can [`read`](crate::Reg::read) this register and get [`ccr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM6:CCR6)

For information about available fields see [`mod@ccr6`] module*/
pub type CCR6 = crate::Reg<ccr6::CCR6rs>;
///TIM6 capture/compare register 6
pub mod ccr6;
