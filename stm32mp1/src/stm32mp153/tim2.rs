#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tim2_cr1: TIM2_CR1,
    _reserved1: [u8; 0x02],
    tim2_cr2: TIM2_CR2,
    tim2_smcr: TIM2_SMCR,
    tim2_dier: TIM2_DIER,
    _reserved4: [u8; 0x02],
    tim2_sr: TIM2_SR,
    tim2_egr: TIM2_EGR,
    _reserved6: [u8; 0x02],
    tim2_ccmr1alternate2: TIM2_CCMR1ALTERNATE2,
    tim2_ccmr2alternate18: TIM2_CCMR2ALTERNATE18,
    tim2_ccer: TIM2_CCER,
    tim2_cnt: TIM2_CNT,
    tim2_psc: TIM2_PSC,
    _reserved11: [u8; 0x02],
    tim2_arr: TIM2_ARR,
    _reserved12: [u8; 0x02],
    tim2_rcr: TIM2_RCR,
    _reserved13: [u8; 0x02],
    tim2_ccr1: TIM2_CCR1,
    _reserved14: [u8; 0x02],
    tim2_ccr2: TIM2_CCR2,
    _reserved15: [u8; 0x02],
    tim2_ccr3: TIM2_CCR3,
    _reserved16: [u8; 0x02],
    tim2_ccr4: TIM2_CCR4,
    _reserved17: [u8; 0x02],
    tim2_bdtr: TIM2_BDTR,
    tim2_dcr: TIM2_DCR,
    _reserved19: [u8; 0x02],
    tim2_dmar: TIM2_DMAR,
    _reserved20: [u8; 0x04],
    tim2_ccmr3: TIM2_CCMR3,
    tim2_ccr5: TIM2_CCR5,
    tim2_ccr6: TIM2_CCR6,
}
impl RegisterBlock {
    ///0x00 - TIM2 control register 1
    #[inline(always)]
    pub const fn tim2_cr1(&self) -> &TIM2_CR1 {
        &self.tim2_cr1
    }
    ///0x04 - TIM2 control register 2
    #[inline(always)]
    pub const fn tim2_cr2(&self) -> &TIM2_CR2 {
        &self.tim2_cr2
    }
    ///0x08 - TIM2 slave mode control register
    #[inline(always)]
    pub const fn tim2_smcr(&self) -> &TIM2_SMCR {
        &self.tim2_smcr
    }
    ///0x0c - TIM2 DMA/interrupt enable register
    #[inline(always)]
    pub const fn tim2_dier(&self) -> &TIM2_DIER {
        &self.tim2_dier
    }
    ///0x10 - TIM2 status register
    #[inline(always)]
    pub const fn tim2_sr(&self) -> &TIM2_SR {
        &self.tim2_sr
    }
    ///0x14 - TIM2 event generation register
    #[inline(always)]
    pub const fn tim2_egr(&self) -> &TIM2_EGR {
        &self.tim2_egr
    }
    ///0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    #[inline(always)]
    pub const fn tim2_ccmr1alternate2(&self) -> &TIM2_CCMR1ALTERNATE2 {
        &self.tim2_ccmr1alternate2
    }
    ///0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    #[inline(always)]
    pub const fn tim2_ccmr2alternate18(&self) -> &TIM2_CCMR2ALTERNATE18 {
        &self.tim2_ccmr2alternate18
    }
    ///0x20 - TIM2 capture/compare enable register
    #[inline(always)]
    pub const fn tim2_ccer(&self) -> &TIM2_CCER {
        &self.tim2_ccer
    }
    ///0x24 - TIM2 counter
    #[inline(always)]
    pub const fn tim2_cnt(&self) -> &TIM2_CNT {
        &self.tim2_cnt
    }
    ///0x28 - TIM2 prescaler
    #[inline(always)]
    pub const fn tim2_psc(&self) -> &TIM2_PSC {
        &self.tim2_psc
    }
    ///0x2c - TIM2 auto-reload register
    #[inline(always)]
    pub const fn tim2_arr(&self) -> &TIM2_ARR {
        &self.tim2_arr
    }
    ///0x30 - TIM2 repetition counter register
    #[inline(always)]
    pub const fn tim2_rcr(&self) -> &TIM2_RCR {
        &self.tim2_rcr
    }
    ///0x34 - TIM2 capture/compare register 1
    #[inline(always)]
    pub const fn tim2_ccr1(&self) -> &TIM2_CCR1 {
        &self.tim2_ccr1
    }
    ///0x38 - TIM2 capture/compare register 2
    #[inline(always)]
    pub const fn tim2_ccr2(&self) -> &TIM2_CCR2 {
        &self.tim2_ccr2
    }
    ///0x3c - TIM2 capture/compare register 3
    #[inline(always)]
    pub const fn tim2_ccr3(&self) -> &TIM2_CCR3 {
        &self.tim2_ccr3
    }
    ///0x40 - TIM2 capture/compare register 4
    #[inline(always)]
    pub const fn tim2_ccr4(&self) -> &TIM2_CCR4 {
        &self.tim2_ccr4
    }
    /**0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
    can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.*/
    #[inline(always)]
    pub const fn tim2_bdtr(&self) -> &TIM2_BDTR {
        &self.tim2_bdtr
    }
    ///0x48 - TIM2 DMA control register
    #[inline(always)]
    pub const fn tim2_dcr(&self) -> &TIM2_DCR {
        &self.tim2_dcr
    }
    ///0x4c - TIM2 DMA address for full transfer
    #[inline(always)]
    pub const fn tim2_dmar(&self) -> &TIM2_DMAR {
        &self.tim2_dmar
    }
    ///0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:
    #[inline(always)]
    pub const fn tim2_ccmr3(&self) -> &TIM2_CCMR3 {
        &self.tim2_ccmr3
    }
    ///0x58 - TIM2 capture/compare register 5
    #[inline(always)]
    pub const fn tim2_ccr5(&self) -> &TIM2_CCR5 {
        &self.tim2_ccr5
    }
    ///0x5c - TIM2 capture/compare register 6
    #[inline(always)]
    pub const fn tim2_ccr6(&self) -> &TIM2_CCR6 {
        &self.tim2_ccr6
    }
}
/**TIM2_CR1 (rw) register accessor: TIM2 control register 1

You can [`read`](crate::Reg::read) this register and get [`tim2_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM2:TIM2_CR1)

For information about available fields see [`mod@tim2_cr1`]
module*/
pub type TIM2_CR1 = crate::Reg<tim2_cr1::TIM2_CR1rs>;
///TIM2 control register 1
pub mod tim2_cr1;
/**TIM2_CR2 (rw) register accessor: TIM2 control register 2

You can [`read`](crate::Reg::read) this register and get [`tim2_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM2:TIM2_CR2)

For information about available fields see [`mod@tim2_cr2`]
module*/
pub type TIM2_CR2 = crate::Reg<tim2_cr2::TIM2_CR2rs>;
///TIM2 control register 2
pub mod tim2_cr2;
/**TIM2_SMCR (rw) register accessor: TIM2 slave mode control register

You can [`read`](crate::Reg::read) this register and get [`tim2_smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM2:TIM2_SMCR)

For information about available fields see [`mod@tim2_smcr`]
module*/
pub type TIM2_SMCR = crate::Reg<tim2_smcr::TIM2_SMCRrs>;
///TIM2 slave mode control register
pub mod tim2_smcr;
/**TIM2_DIER (rw) register accessor: TIM2 DMA/interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tim2_dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM2:TIM2_DIER)

For information about available fields see [`mod@tim2_dier`]
module*/
pub type TIM2_DIER = crate::Reg<tim2_dier::TIM2_DIERrs>;
///TIM2 DMA/interrupt enable register
pub mod tim2_dier;
/**TIM2_SR (rw) register accessor: TIM2 status register

You can [`read`](crate::Reg::read) this register and get [`tim2_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM2:TIM2_SR)

For information about available fields see [`mod@tim2_sr`]
module*/
pub type TIM2_SR = crate::Reg<tim2_sr::TIM2_SRrs>;
///TIM2 status register
pub mod tim2_sr;
/**TIM2_EGR (w) register accessor: TIM2 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM2:TIM2_EGR)

For information about available fields see [`mod@tim2_egr`]
module*/
pub type TIM2_EGR = crate::Reg<tim2_egr::TIM2_EGRrs>;
///TIM2 event generation register
pub mod tim2_egr;
/**TIM2_CCMR1ALTERNATE2 (rw) register accessor: The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:

You can [`read`](crate::Reg::read) this register and get [`tim2_ccmr1alternate2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccmr1alternate2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM2:TIM2_CCMR1ALTERNATE2)

For information about available fields see [`mod@tim2_ccmr1alternate2`]
module*/
pub type TIM2_CCMR1ALTERNATE2 = crate::Reg<tim2_ccmr1alternate2::TIM2_CCMR1ALTERNATE2rs>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod tim2_ccmr1alternate2;
/**TIM2_CCMR2ALTERNATE18 (rw) register accessor: The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:

You can [`read`](crate::Reg::read) this register and get [`tim2_ccmr2alternate18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccmr2alternate18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM2:TIM2_CCMR2ALTERNATE18)

For information about available fields see [`mod@tim2_ccmr2alternate18`]
module*/
pub type TIM2_CCMR2ALTERNATE18 = crate::Reg<tim2_ccmr2alternate18::TIM2_CCMR2ALTERNATE18rs>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod tim2_ccmr2alternate18;
/**TIM2_CCER (rw) register accessor: TIM2 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`tim2_ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM2:TIM2_CCER)

For information about available fields see [`mod@tim2_ccer`]
module*/
pub type TIM2_CCER = crate::Reg<tim2_ccer::TIM2_CCERrs>;
///TIM2 capture/compare enable register
pub mod tim2_ccer;
/**TIM2_CNT (rw) register accessor: TIM2 counter

You can [`read`](crate::Reg::read) this register and get [`tim2_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM2:TIM2_CNT)

For information about available fields see [`mod@tim2_cnt`]
module*/
pub type TIM2_CNT = crate::Reg<tim2_cnt::TIM2_CNTrs>;
///TIM2 counter
pub mod tim2_cnt;
/**TIM2_PSC (rw) register accessor: TIM2 prescaler

You can [`read`](crate::Reg::read) this register and get [`tim2_psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM2:TIM2_PSC)

For information about available fields see [`mod@tim2_psc`]
module*/
pub type TIM2_PSC = crate::Reg<tim2_psc::TIM2_PSCrs>;
///TIM2 prescaler
pub mod tim2_psc;
/**TIM2_ARR (rw) register accessor: TIM2 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`tim2_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM2:TIM2_ARR)

For information about available fields see [`mod@tim2_arr`]
module*/
pub type TIM2_ARR = crate::Reg<tim2_arr::TIM2_ARRrs>;
///TIM2 auto-reload register
pub mod tim2_arr;
/**TIM2_RCR (rw) register accessor: TIM2 repetition counter register

You can [`read`](crate::Reg::read) this register and get [`tim2_rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM2:TIM2_RCR)

For information about available fields see [`mod@tim2_rcr`]
module*/
pub type TIM2_RCR = crate::Reg<tim2_rcr::TIM2_RCRrs>;
///TIM2 repetition counter register
pub mod tim2_rcr;
/**TIM2_CCR1 (rw) register accessor: TIM2 capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`tim2_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM2:TIM2_CCR1)

For information about available fields see [`mod@tim2_ccr1`]
module*/
pub type TIM2_CCR1 = crate::Reg<tim2_ccr1::TIM2_CCR1rs>;
///TIM2 capture/compare register 1
pub mod tim2_ccr1;
/**TIM2_CCR2 (rw) register accessor: TIM2 capture/compare register 2

You can [`read`](crate::Reg::read) this register and get [`tim2_ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM2:TIM2_CCR2)

For information about available fields see [`mod@tim2_ccr2`]
module*/
pub type TIM2_CCR2 = crate::Reg<tim2_ccr2::TIM2_CCR2rs>;
///TIM2 capture/compare register 2
pub mod tim2_ccr2;
/**TIM2_CCR3 (rw) register accessor: TIM2 capture/compare register 3

You can [`read`](crate::Reg::read) this register and get [`tim2_ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM2:TIM2_CCR3)

For information about available fields see [`mod@tim2_ccr3`]
module*/
pub type TIM2_CCR3 = crate::Reg<tim2_ccr3::TIM2_CCR3rs>;
///TIM2 capture/compare register 3
pub mod tim2_ccr3;
/**TIM2_CCR4 (rw) register accessor: TIM2 capture/compare register 4

You can [`read`](crate::Reg::read) this register and get [`tim2_ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM2:TIM2_CCR4)

For information about available fields see [`mod@tim2_ccr4`]
module*/
pub type TIM2_CCR4 = crate::Reg<tim2_ccr4::TIM2_CCR4rs>;
///TIM2 capture/compare register 4
pub mod tim2_ccr4;
/**TIM2_BDTR (rw) register accessor: As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.

You can [`read`](crate::Reg::read) this register and get [`tim2_bdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_bdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM2:TIM2_BDTR)

For information about available fields see [`mod@tim2_bdtr`]
module*/
pub type TIM2_BDTR = crate::Reg<tim2_bdtr::TIM2_BDTRrs>;
/**As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.*/
pub mod tim2_bdtr;
/**TIM2_DCR (rw) register accessor: TIM2 DMA control register

You can [`read`](crate::Reg::read) this register and get [`tim2_dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM2:TIM2_DCR)

For information about available fields see [`mod@tim2_dcr`]
module*/
pub type TIM2_DCR = crate::Reg<tim2_dcr::TIM2_DCRrs>;
///TIM2 DMA control register
pub mod tim2_dcr;
/**TIM2_DMAR (rw) register accessor: TIM2 DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`tim2_dmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_dmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM2:TIM2_DMAR)

For information about available fields see [`mod@tim2_dmar`]
module*/
pub type TIM2_DMAR = crate::Reg<tim2_dmar::TIM2_DMARrs>;
///TIM2 DMA address for full transfer
pub mod tim2_dmar;
/**TIM2_CCMR3 (rw) register accessor: The channels 5 and 6 can only be configured in output. Output compare mode:

You can [`read`](crate::Reg::read) this register and get [`tim2_ccmr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccmr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM2:TIM2_CCMR3)

For information about available fields see [`mod@tim2_ccmr3`]
module*/
pub type TIM2_CCMR3 = crate::Reg<tim2_ccmr3::TIM2_CCMR3rs>;
///The channels 5 and 6 can only be configured in output. Output compare mode:
pub mod tim2_ccmr3;
/**TIM2_CCR5 (rw) register accessor: TIM2 capture/compare register 5

You can [`read`](crate::Reg::read) this register and get [`tim2_ccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM2:TIM2_CCR5)

For information about available fields see [`mod@tim2_ccr5`]
module*/
pub type TIM2_CCR5 = crate::Reg<tim2_ccr5::TIM2_CCR5rs>;
///TIM2 capture/compare register 5
pub mod tim2_ccr5;
/**TIM2_CCR6 (rw) register accessor: TIM2 capture/compare register 6

You can [`read`](crate::Reg::read) this register and get [`tim2_ccr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM2:TIM2_CCR6)

For information about available fields see [`mod@tim2_ccr6`]
module*/
pub type TIM2_CCR6 = crate::Reg<tim2_ccr6::TIM2_CCR6rs>;
///TIM2 capture/compare register 6
pub mod tim2_ccr6;
