#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tim5_cr1: TIM5_CR1,
    _reserved1: [u8; 0x02],
    tim5_cr2: TIM5_CR2,
    tim5_smcr: TIM5_SMCR,
    tim5_dier: TIM5_DIER,
    _reserved4: [u8; 0x02],
    tim5_sr: TIM5_SR,
    tim5_egr: TIM5_EGR,
    _reserved6: [u8; 0x02],
    tim5_ccmr1alternate5: TIM5_CCMR1ALTERNATE5,
    tim5_ccmr2alternate21: TIM5_CCMR2ALTERNATE21,
    tim5_ccer: TIM5_CCER,
    tim5_cnt: TIM5_CNT,
    tim5_psc: TIM5_PSC,
    _reserved11: [u8; 0x02],
    tim5_arr: TIM5_ARR,
    _reserved12: [u8; 0x02],
    tim5_rcr: TIM5_RCR,
    _reserved13: [u8; 0x02],
    tim5_ccr1: TIM5_CCR1,
    _reserved14: [u8; 0x02],
    tim5_ccr2: TIM5_CCR2,
    _reserved15: [u8; 0x02],
    tim5_ccr3: TIM5_CCR3,
    _reserved16: [u8; 0x02],
    tim5_ccr4: TIM5_CCR4,
    _reserved17: [u8; 0x02],
    tim5_bdtr: TIM5_BDTR,
    tim5_dcr: TIM5_DCR,
    _reserved19: [u8; 0x02],
    tim5_dmar: TIM5_DMAR,
    _reserved20: [u8; 0x04],
    tim5_ccmr3: TIM5_CCMR3,
    tim5_ccr5: TIM5_CCR5,
    tim5_ccr6: TIM5_CCR6,
}
impl RegisterBlock {
    ///0x00 - TIM5 control register 1
    #[inline(always)]
    pub const fn tim5_cr1(&self) -> &TIM5_CR1 {
        &self.tim5_cr1
    }
    ///0x04 - TIM5 control register 2
    #[inline(always)]
    pub const fn tim5_cr2(&self) -> &TIM5_CR2 {
        &self.tim5_cr2
    }
    ///0x08 - TIM5 slave mode control register
    #[inline(always)]
    pub const fn tim5_smcr(&self) -> &TIM5_SMCR {
        &self.tim5_smcr
    }
    ///0x0c - TIM5 DMA/interrupt enable register
    #[inline(always)]
    pub const fn tim5_dier(&self) -> &TIM5_DIER {
        &self.tim5_dier
    }
    ///0x10 - TIM5 status register
    #[inline(always)]
    pub const fn tim5_sr(&self) -> &TIM5_SR {
        &self.tim5_sr
    }
    ///0x14 - TIM5 event generation register
    #[inline(always)]
    pub const fn tim5_egr(&self) -> &TIM5_EGR {
        &self.tim5_egr
    }
    ///0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    #[inline(always)]
    pub const fn tim5_ccmr1alternate5(&self) -> &TIM5_CCMR1ALTERNATE5 {
        &self.tim5_ccmr1alternate5
    }
    ///0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    #[inline(always)]
    pub const fn tim5_ccmr2alternate21(&self) -> &TIM5_CCMR2ALTERNATE21 {
        &self.tim5_ccmr2alternate21
    }
    ///0x20 - TIM5 capture/compare enable register
    #[inline(always)]
    pub const fn tim5_ccer(&self) -> &TIM5_CCER {
        &self.tim5_ccer
    }
    ///0x24 - TIM5 counter
    #[inline(always)]
    pub const fn tim5_cnt(&self) -> &TIM5_CNT {
        &self.tim5_cnt
    }
    ///0x28 - TIM5 prescaler
    #[inline(always)]
    pub const fn tim5_psc(&self) -> &TIM5_PSC {
        &self.tim5_psc
    }
    ///0x2c - TIM5 auto-reload register
    #[inline(always)]
    pub const fn tim5_arr(&self) -> &TIM5_ARR {
        &self.tim5_arr
    }
    ///0x30 - TIM5 repetition counter register
    #[inline(always)]
    pub const fn tim5_rcr(&self) -> &TIM5_RCR {
        &self.tim5_rcr
    }
    ///0x34 - TIM5 capture/compare register 1
    #[inline(always)]
    pub const fn tim5_ccr1(&self) -> &TIM5_CCR1 {
        &self.tim5_ccr1
    }
    ///0x38 - TIM5 capture/compare register 2
    #[inline(always)]
    pub const fn tim5_ccr2(&self) -> &TIM5_CCR2 {
        &self.tim5_ccr2
    }
    ///0x3c - TIM5 capture/compare register 3
    #[inline(always)]
    pub const fn tim5_ccr3(&self) -> &TIM5_CCR3 {
        &self.tim5_ccr3
    }
    ///0x40 - TIM5 capture/compare register 4
    #[inline(always)]
    pub const fn tim5_ccr4(&self) -> &TIM5_CCR4 {
        &self.tim5_ccr4
    }
    /**0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
    can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.*/
    #[inline(always)]
    pub const fn tim5_bdtr(&self) -> &TIM5_BDTR {
        &self.tim5_bdtr
    }
    ///0x48 - TIM5 DMA control register
    #[inline(always)]
    pub const fn tim5_dcr(&self) -> &TIM5_DCR {
        &self.tim5_dcr
    }
    ///0x4c - TIM5 DMA address for full transfer
    #[inline(always)]
    pub const fn tim5_dmar(&self) -> &TIM5_DMAR {
        &self.tim5_dmar
    }
    ///0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:
    #[inline(always)]
    pub const fn tim5_ccmr3(&self) -> &TIM5_CCMR3 {
        &self.tim5_ccmr3
    }
    ///0x58 - TIM5 capture/compare register 5
    #[inline(always)]
    pub const fn tim5_ccr5(&self) -> &TIM5_CCR5 {
        &self.tim5_ccr5
    }
    ///0x5c - TIM5 capture/compare register 6
    #[inline(always)]
    pub const fn tim5_ccr6(&self) -> &TIM5_CCR6 {
        &self.tim5_ccr6
    }
}
/**TIM5_CR1 (rw) register accessor: TIM5 control register 1

You can [`read`](crate::Reg::read) this register and get [`tim5_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim5_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:TIM5_CR1)

For information about available fields see [`mod@tim5_cr1`]
module*/
pub type TIM5_CR1 = crate::Reg<tim5_cr1::TIM5_CR1rs>;
///TIM5 control register 1
pub mod tim5_cr1;
/**TIM5_CR2 (rw) register accessor: TIM5 control register 2

You can [`read`](crate::Reg::read) this register and get [`tim5_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim5_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:TIM5_CR2)

For information about available fields see [`mod@tim5_cr2`]
module*/
pub type TIM5_CR2 = crate::Reg<tim5_cr2::TIM5_CR2rs>;
///TIM5 control register 2
pub mod tim5_cr2;
/**TIM5_SMCR (rw) register accessor: TIM5 slave mode control register

You can [`read`](crate::Reg::read) this register and get [`tim5_smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim5_smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:TIM5_SMCR)

For information about available fields see [`mod@tim5_smcr`]
module*/
pub type TIM5_SMCR = crate::Reg<tim5_smcr::TIM5_SMCRrs>;
///TIM5 slave mode control register
pub mod tim5_smcr;
/**TIM5_DIER (rw) register accessor: TIM5 DMA/interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tim5_dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim5_dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:TIM5_DIER)

For information about available fields see [`mod@tim5_dier`]
module*/
pub type TIM5_DIER = crate::Reg<tim5_dier::TIM5_DIERrs>;
///TIM5 DMA/interrupt enable register
pub mod tim5_dier;
/**TIM5_SR (rw) register accessor: TIM5 status register

You can [`read`](crate::Reg::read) this register and get [`tim5_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim5_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:TIM5_SR)

For information about available fields see [`mod@tim5_sr`]
module*/
pub type TIM5_SR = crate::Reg<tim5_sr::TIM5_SRrs>;
///TIM5 status register
pub mod tim5_sr;
/**TIM5_EGR (w) register accessor: TIM5 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim5_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:TIM5_EGR)

For information about available fields see [`mod@tim5_egr`]
module*/
pub type TIM5_EGR = crate::Reg<tim5_egr::TIM5_EGRrs>;
///TIM5 event generation register
pub mod tim5_egr;
/**TIM5_CCMR1ALTERNATE5 (rw) register accessor: The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:

You can [`read`](crate::Reg::read) this register and get [`tim5_ccmr1alternate5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim5_ccmr1alternate5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:TIM5_CCMR1ALTERNATE5)

For information about available fields see [`mod@tim5_ccmr1alternate5`]
module*/
pub type TIM5_CCMR1ALTERNATE5 = crate::Reg<tim5_ccmr1alternate5::TIM5_CCMR1ALTERNATE5rs>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod tim5_ccmr1alternate5;
/**TIM5_CCMR2ALTERNATE21 (rw) register accessor: The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:

You can [`read`](crate::Reg::read) this register and get [`tim5_ccmr2alternate21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim5_ccmr2alternate21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:TIM5_CCMR2ALTERNATE21)

For information about available fields see [`mod@tim5_ccmr2alternate21`]
module*/
pub type TIM5_CCMR2ALTERNATE21 = crate::Reg<tim5_ccmr2alternate21::TIM5_CCMR2ALTERNATE21rs>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod tim5_ccmr2alternate21;
/**TIM5_CCER (rw) register accessor: TIM5 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`tim5_ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim5_ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:TIM5_CCER)

For information about available fields see [`mod@tim5_ccer`]
module*/
pub type TIM5_CCER = crate::Reg<tim5_ccer::TIM5_CCERrs>;
///TIM5 capture/compare enable register
pub mod tim5_ccer;
/**TIM5_CNT (rw) register accessor: TIM5 counter

You can [`read`](crate::Reg::read) this register and get [`tim5_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim5_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:TIM5_CNT)

For information about available fields see [`mod@tim5_cnt`]
module*/
pub type TIM5_CNT = crate::Reg<tim5_cnt::TIM5_CNTrs>;
///TIM5 counter
pub mod tim5_cnt;
/**TIM5_PSC (rw) register accessor: TIM5 prescaler

You can [`read`](crate::Reg::read) this register and get [`tim5_psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim5_psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:TIM5_PSC)

For information about available fields see [`mod@tim5_psc`]
module*/
pub type TIM5_PSC = crate::Reg<tim5_psc::TIM5_PSCrs>;
///TIM5 prescaler
pub mod tim5_psc;
/**TIM5_ARR (rw) register accessor: TIM5 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`tim5_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim5_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:TIM5_ARR)

For information about available fields see [`mod@tim5_arr`]
module*/
pub type TIM5_ARR = crate::Reg<tim5_arr::TIM5_ARRrs>;
///TIM5 auto-reload register
pub mod tim5_arr;
/**TIM5_RCR (rw) register accessor: TIM5 repetition counter register

You can [`read`](crate::Reg::read) this register and get [`tim5_rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim5_rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:TIM5_RCR)

For information about available fields see [`mod@tim5_rcr`]
module*/
pub type TIM5_RCR = crate::Reg<tim5_rcr::TIM5_RCRrs>;
///TIM5 repetition counter register
pub mod tim5_rcr;
/**TIM5_CCR1 (rw) register accessor: TIM5 capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`tim5_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim5_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:TIM5_CCR1)

For information about available fields see [`mod@tim5_ccr1`]
module*/
pub type TIM5_CCR1 = crate::Reg<tim5_ccr1::TIM5_CCR1rs>;
///TIM5 capture/compare register 1
pub mod tim5_ccr1;
/**TIM5_CCR2 (rw) register accessor: TIM5 capture/compare register 2

You can [`read`](crate::Reg::read) this register and get [`tim5_ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim5_ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:TIM5_CCR2)

For information about available fields see [`mod@tim5_ccr2`]
module*/
pub type TIM5_CCR2 = crate::Reg<tim5_ccr2::TIM5_CCR2rs>;
///TIM5 capture/compare register 2
pub mod tim5_ccr2;
/**TIM5_CCR3 (rw) register accessor: TIM5 capture/compare register 3

You can [`read`](crate::Reg::read) this register and get [`tim5_ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim5_ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:TIM5_CCR3)

For information about available fields see [`mod@tim5_ccr3`]
module*/
pub type TIM5_CCR3 = crate::Reg<tim5_ccr3::TIM5_CCR3rs>;
///TIM5 capture/compare register 3
pub mod tim5_ccr3;
/**TIM5_CCR4 (rw) register accessor: TIM5 capture/compare register 4

You can [`read`](crate::Reg::read) this register and get [`tim5_ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim5_ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:TIM5_CCR4)

For information about available fields see [`mod@tim5_ccr4`]
module*/
pub type TIM5_CCR4 = crate::Reg<tim5_ccr4::TIM5_CCR4rs>;
///TIM5 capture/compare register 4
pub mod tim5_ccr4;
/**TIM5_BDTR (rw) register accessor: As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.

You can [`read`](crate::Reg::read) this register and get [`tim5_bdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim5_bdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:TIM5_BDTR)

For information about available fields see [`mod@tim5_bdtr`]
module*/
pub type TIM5_BDTR = crate::Reg<tim5_bdtr::TIM5_BDTRrs>;
/**As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.*/
pub mod tim5_bdtr;
/**TIM5_DCR (rw) register accessor: TIM5 DMA control register

You can [`read`](crate::Reg::read) this register and get [`tim5_dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim5_dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:TIM5_DCR)

For information about available fields see [`mod@tim5_dcr`]
module*/
pub type TIM5_DCR = crate::Reg<tim5_dcr::TIM5_DCRrs>;
///TIM5 DMA control register
pub mod tim5_dcr;
/**TIM5_DMAR (rw) register accessor: TIM5 DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`tim5_dmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim5_dmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:TIM5_DMAR)

For information about available fields see [`mod@tim5_dmar`]
module*/
pub type TIM5_DMAR = crate::Reg<tim5_dmar::TIM5_DMARrs>;
///TIM5 DMA address for full transfer
pub mod tim5_dmar;
/**TIM5_CCMR3 (rw) register accessor: The channels 5 and 6 can only be configured in output. Output compare mode:

You can [`read`](crate::Reg::read) this register and get [`tim5_ccmr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim5_ccmr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:TIM5_CCMR3)

For information about available fields see [`mod@tim5_ccmr3`]
module*/
pub type TIM5_CCMR3 = crate::Reg<tim5_ccmr3::TIM5_CCMR3rs>;
///The channels 5 and 6 can only be configured in output. Output compare mode:
pub mod tim5_ccmr3;
/**TIM5_CCR5 (rw) register accessor: TIM5 capture/compare register 5

You can [`read`](crate::Reg::read) this register and get [`tim5_ccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim5_ccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:TIM5_CCR5)

For information about available fields see [`mod@tim5_ccr5`]
module*/
pub type TIM5_CCR5 = crate::Reg<tim5_ccr5::TIM5_CCR5rs>;
///TIM5 capture/compare register 5
pub mod tim5_ccr5;
/**TIM5_CCR6 (rw) register accessor: TIM5 capture/compare register 6

You can [`read`](crate::Reg::read) this register and get [`tim5_ccr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim5_ccr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM5:TIM5_CCR6)

For information about available fields see [`mod@tim5_ccr6`]
module*/
pub type TIM5_CCR6 = crate::Reg<tim5_ccr6::TIM5_CCR6rs>;
///TIM5 capture/compare register 6
pub mod tim5_ccr6;
