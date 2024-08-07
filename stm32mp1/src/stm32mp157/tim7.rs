#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tim7_cr1: TIM7_CR1,
    _reserved1: [u8; 0x02],
    tim7_cr2: TIM7_CR2,
    tim7_smcr: TIM7_SMCR,
    tim7_dier: TIM7_DIER,
    _reserved4: [u8; 0x02],
    tim7_sr: TIM7_SR,
    tim7_egr: TIM7_EGR,
    _reserved6: [u8; 0x02],
    tim7_ccmr1alternate7: TIM7_CCMR1ALTERNATE7,
    tim7_ccmr2alternate23: TIM7_CCMR2ALTERNATE23,
    tim7_ccer: TIM7_CCER,
    tim7_cnt: TIM7_CNT,
    tim7_psc: TIM7_PSC,
    _reserved11: [u8; 0x02],
    tim7_arr: TIM7_ARR,
    _reserved12: [u8; 0x02],
    tim7_rcr: TIM7_RCR,
    _reserved13: [u8; 0x02],
    tim7_ccr1: TIM7_CCR1,
    _reserved14: [u8; 0x02],
    tim7_ccr2: TIM7_CCR2,
    _reserved15: [u8; 0x02],
    tim7_ccr3: TIM7_CCR3,
    _reserved16: [u8; 0x02],
    tim7_ccr4: TIM7_CCR4,
    _reserved17: [u8; 0x02],
    tim7_bdtr: TIM7_BDTR,
    tim7_dcr: TIM7_DCR,
    _reserved19: [u8; 0x02],
    tim7_dmar: TIM7_DMAR,
    _reserved20: [u8; 0x04],
    tim7_ccmr3: TIM7_CCMR3,
    tim7_ccr5: TIM7_CCR5,
    tim7_ccr6: TIM7_CCR6,
}
impl RegisterBlock {
    ///0x00 - TIM7 control register 1
    #[inline(always)]
    pub const fn tim7_cr1(&self) -> &TIM7_CR1 {
        &self.tim7_cr1
    }
    ///0x04 - TIM7 control register 2
    #[inline(always)]
    pub const fn tim7_cr2(&self) -> &TIM7_CR2 {
        &self.tim7_cr2
    }
    ///0x08 - TIM7 slave mode control register
    #[inline(always)]
    pub const fn tim7_smcr(&self) -> &TIM7_SMCR {
        &self.tim7_smcr
    }
    ///0x0c - TIM7 DMA/interrupt enable register
    #[inline(always)]
    pub const fn tim7_dier(&self) -> &TIM7_DIER {
        &self.tim7_dier
    }
    ///0x10 - TIM7 status register
    #[inline(always)]
    pub const fn tim7_sr(&self) -> &TIM7_SR {
        &self.tim7_sr
    }
    ///0x14 - TIM7 event generation register
    #[inline(always)]
    pub const fn tim7_egr(&self) -> &TIM7_EGR {
        &self.tim7_egr
    }
    ///0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    #[inline(always)]
    pub const fn tim7_ccmr1alternate7(&self) -> &TIM7_CCMR1ALTERNATE7 {
        &self.tim7_ccmr1alternate7
    }
    ///0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    #[inline(always)]
    pub const fn tim7_ccmr2alternate23(&self) -> &TIM7_CCMR2ALTERNATE23 {
        &self.tim7_ccmr2alternate23
    }
    ///0x20 - TIM7 capture/compare enable register
    #[inline(always)]
    pub const fn tim7_ccer(&self) -> &TIM7_CCER {
        &self.tim7_ccer
    }
    ///0x24 - TIM7 counter
    #[inline(always)]
    pub const fn tim7_cnt(&self) -> &TIM7_CNT {
        &self.tim7_cnt
    }
    ///0x28 - TIM7 prescaler
    #[inline(always)]
    pub const fn tim7_psc(&self) -> &TIM7_PSC {
        &self.tim7_psc
    }
    ///0x2c - TIM7 auto-reload register
    #[inline(always)]
    pub const fn tim7_arr(&self) -> &TIM7_ARR {
        &self.tim7_arr
    }
    ///0x30 - TIM7 repetition counter register
    #[inline(always)]
    pub const fn tim7_rcr(&self) -> &TIM7_RCR {
        &self.tim7_rcr
    }
    ///0x34 - TIM7 capture/compare register 1
    #[inline(always)]
    pub const fn tim7_ccr1(&self) -> &TIM7_CCR1 {
        &self.tim7_ccr1
    }
    ///0x38 - TIM7 capture/compare register 2
    #[inline(always)]
    pub const fn tim7_ccr2(&self) -> &TIM7_CCR2 {
        &self.tim7_ccr2
    }
    ///0x3c - TIM7 capture/compare register 3
    #[inline(always)]
    pub const fn tim7_ccr3(&self) -> &TIM7_CCR3 {
        &self.tim7_ccr3
    }
    ///0x40 - TIM7 capture/compare register 4
    #[inline(always)]
    pub const fn tim7_ccr4(&self) -> &TIM7_CCR4 {
        &self.tim7_ccr4
    }
    /**0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
    can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.*/
    #[inline(always)]
    pub const fn tim7_bdtr(&self) -> &TIM7_BDTR {
        &self.tim7_bdtr
    }
    ///0x48 - TIM7 DMA control register
    #[inline(always)]
    pub const fn tim7_dcr(&self) -> &TIM7_DCR {
        &self.tim7_dcr
    }
    ///0x4c - TIM7 DMA address for full transfer
    #[inline(always)]
    pub const fn tim7_dmar(&self) -> &TIM7_DMAR {
        &self.tim7_dmar
    }
    ///0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:
    #[inline(always)]
    pub const fn tim7_ccmr3(&self) -> &TIM7_CCMR3 {
        &self.tim7_ccmr3
    }
    ///0x58 - TIM7 capture/compare register 5
    #[inline(always)]
    pub const fn tim7_ccr5(&self) -> &TIM7_CCR5 {
        &self.tim7_ccr5
    }
    ///0x5c - TIM7 capture/compare register 6
    #[inline(always)]
    pub const fn tim7_ccr6(&self) -> &TIM7_CCR6 {
        &self.tim7_ccr6
    }
}
/**TIM7_CR1 (rw) register accessor: TIM7 control register 1

You can [`read`](crate::Reg::read) this register and get [`tim7_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM7:TIM7_CR1)

For information about available fields see [`mod@tim7_cr1`]
module*/
pub type TIM7_CR1 = crate::Reg<tim7_cr1::TIM7_CR1rs>;
///TIM7 control register 1
pub mod tim7_cr1;
/**TIM7_CR2 (rw) register accessor: TIM7 control register 2

You can [`read`](crate::Reg::read) this register and get [`tim7_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM7:TIM7_CR2)

For information about available fields see [`mod@tim7_cr2`]
module*/
pub type TIM7_CR2 = crate::Reg<tim7_cr2::TIM7_CR2rs>;
///TIM7 control register 2
pub mod tim7_cr2;
/**TIM7_SMCR (rw) register accessor: TIM7 slave mode control register

You can [`read`](crate::Reg::read) this register and get [`tim7_smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM7:TIM7_SMCR)

For information about available fields see [`mod@tim7_smcr`]
module*/
pub type TIM7_SMCR = crate::Reg<tim7_smcr::TIM7_SMCRrs>;
///TIM7 slave mode control register
pub mod tim7_smcr;
/**TIM7_DIER (rw) register accessor: TIM7 DMA/interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tim7_dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM7:TIM7_DIER)

For information about available fields see [`mod@tim7_dier`]
module*/
pub type TIM7_DIER = crate::Reg<tim7_dier::TIM7_DIERrs>;
///TIM7 DMA/interrupt enable register
pub mod tim7_dier;
/**TIM7_SR (rw) register accessor: TIM7 status register

You can [`read`](crate::Reg::read) this register and get [`tim7_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM7:TIM7_SR)

For information about available fields see [`mod@tim7_sr`]
module*/
pub type TIM7_SR = crate::Reg<tim7_sr::TIM7_SRrs>;
///TIM7 status register
pub mod tim7_sr;
/**TIM7_EGR (w) register accessor: TIM7 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM7:TIM7_EGR)

For information about available fields see [`mod@tim7_egr`]
module*/
pub type TIM7_EGR = crate::Reg<tim7_egr::TIM7_EGRrs>;
///TIM7 event generation register
pub mod tim7_egr;
/**TIM7_CCMR1ALTERNATE7 (rw) register accessor: The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:

You can [`read`](crate::Reg::read) this register and get [`tim7_ccmr1alternate7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_ccmr1alternate7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM7:TIM7_CCMR1ALTERNATE7)

For information about available fields see [`mod@tim7_ccmr1alternate7`]
module*/
pub type TIM7_CCMR1ALTERNATE7 = crate::Reg<tim7_ccmr1alternate7::TIM7_CCMR1ALTERNATE7rs>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod tim7_ccmr1alternate7;
/**TIM7_CCMR2ALTERNATE23 (rw) register accessor: The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:

You can [`read`](crate::Reg::read) this register and get [`tim7_ccmr2alternate23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_ccmr2alternate23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM7:TIM7_CCMR2ALTERNATE23)

For information about available fields see [`mod@tim7_ccmr2alternate23`]
module*/
pub type TIM7_CCMR2ALTERNATE23 = crate::Reg<tim7_ccmr2alternate23::TIM7_CCMR2ALTERNATE23rs>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod tim7_ccmr2alternate23;
/**TIM7_CCER (rw) register accessor: TIM7 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`tim7_ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM7:TIM7_CCER)

For information about available fields see [`mod@tim7_ccer`]
module*/
pub type TIM7_CCER = crate::Reg<tim7_ccer::TIM7_CCERrs>;
///TIM7 capture/compare enable register
pub mod tim7_ccer;
/**TIM7_CNT (rw) register accessor: TIM7 counter

You can [`read`](crate::Reg::read) this register and get [`tim7_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM7:TIM7_CNT)

For information about available fields see [`mod@tim7_cnt`]
module*/
pub type TIM7_CNT = crate::Reg<tim7_cnt::TIM7_CNTrs>;
///TIM7 counter
pub mod tim7_cnt;
/**TIM7_PSC (rw) register accessor: TIM7 prescaler

You can [`read`](crate::Reg::read) this register and get [`tim7_psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM7:TIM7_PSC)

For information about available fields see [`mod@tim7_psc`]
module*/
pub type TIM7_PSC = crate::Reg<tim7_psc::TIM7_PSCrs>;
///TIM7 prescaler
pub mod tim7_psc;
/**TIM7_ARR (rw) register accessor: TIM7 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`tim7_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM7:TIM7_ARR)

For information about available fields see [`mod@tim7_arr`]
module*/
pub type TIM7_ARR = crate::Reg<tim7_arr::TIM7_ARRrs>;
///TIM7 auto-reload register
pub mod tim7_arr;
/**TIM7_RCR (rw) register accessor: TIM7 repetition counter register

You can [`read`](crate::Reg::read) this register and get [`tim7_rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM7:TIM7_RCR)

For information about available fields see [`mod@tim7_rcr`]
module*/
pub type TIM7_RCR = crate::Reg<tim7_rcr::TIM7_RCRrs>;
///TIM7 repetition counter register
pub mod tim7_rcr;
/**TIM7_CCR1 (rw) register accessor: TIM7 capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`tim7_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM7:TIM7_CCR1)

For information about available fields see [`mod@tim7_ccr1`]
module*/
pub type TIM7_CCR1 = crate::Reg<tim7_ccr1::TIM7_CCR1rs>;
///TIM7 capture/compare register 1
pub mod tim7_ccr1;
/**TIM7_CCR2 (rw) register accessor: TIM7 capture/compare register 2

You can [`read`](crate::Reg::read) this register and get [`tim7_ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM7:TIM7_CCR2)

For information about available fields see [`mod@tim7_ccr2`]
module*/
pub type TIM7_CCR2 = crate::Reg<tim7_ccr2::TIM7_CCR2rs>;
///TIM7 capture/compare register 2
pub mod tim7_ccr2;
/**TIM7_CCR3 (rw) register accessor: TIM7 capture/compare register 3

You can [`read`](crate::Reg::read) this register and get [`tim7_ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM7:TIM7_CCR3)

For information about available fields see [`mod@tim7_ccr3`]
module*/
pub type TIM7_CCR3 = crate::Reg<tim7_ccr3::TIM7_CCR3rs>;
///TIM7 capture/compare register 3
pub mod tim7_ccr3;
/**TIM7_CCR4 (rw) register accessor: TIM7 capture/compare register 4

You can [`read`](crate::Reg::read) this register and get [`tim7_ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM7:TIM7_CCR4)

For information about available fields see [`mod@tim7_ccr4`]
module*/
pub type TIM7_CCR4 = crate::Reg<tim7_ccr4::TIM7_CCR4rs>;
///TIM7 capture/compare register 4
pub mod tim7_ccr4;
/**TIM7_BDTR (rw) register accessor: As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.

You can [`read`](crate::Reg::read) this register and get [`tim7_bdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_bdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM7:TIM7_BDTR)

For information about available fields see [`mod@tim7_bdtr`]
module*/
pub type TIM7_BDTR = crate::Reg<tim7_bdtr::TIM7_BDTRrs>;
/**As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.*/
pub mod tim7_bdtr;
/**TIM7_DCR (rw) register accessor: TIM7 DMA control register

You can [`read`](crate::Reg::read) this register and get [`tim7_dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM7:TIM7_DCR)

For information about available fields see [`mod@tim7_dcr`]
module*/
pub type TIM7_DCR = crate::Reg<tim7_dcr::TIM7_DCRrs>;
///TIM7 DMA control register
pub mod tim7_dcr;
/**TIM7_DMAR (rw) register accessor: TIM7 DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`tim7_dmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_dmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM7:TIM7_DMAR)

For information about available fields see [`mod@tim7_dmar`]
module*/
pub type TIM7_DMAR = crate::Reg<tim7_dmar::TIM7_DMARrs>;
///TIM7 DMA address for full transfer
pub mod tim7_dmar;
/**TIM7_CCMR3 (rw) register accessor: The channels 5 and 6 can only be configured in output. Output compare mode:

You can [`read`](crate::Reg::read) this register and get [`tim7_ccmr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_ccmr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM7:TIM7_CCMR3)

For information about available fields see [`mod@tim7_ccmr3`]
module*/
pub type TIM7_CCMR3 = crate::Reg<tim7_ccmr3::TIM7_CCMR3rs>;
///The channels 5 and 6 can only be configured in output. Output compare mode:
pub mod tim7_ccmr3;
/**TIM7_CCR5 (rw) register accessor: TIM7 capture/compare register 5

You can [`read`](crate::Reg::read) this register and get [`tim7_ccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_ccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM7:TIM7_CCR5)

For information about available fields see [`mod@tim7_ccr5`]
module*/
pub type TIM7_CCR5 = crate::Reg<tim7_ccr5::TIM7_CCR5rs>;
///TIM7 capture/compare register 5
pub mod tim7_ccr5;
/**TIM7_CCR6 (rw) register accessor: TIM7 capture/compare register 6

You can [`read`](crate::Reg::read) this register and get [`tim7_ccr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_ccr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM7:TIM7_CCR6)

For information about available fields see [`mod@tim7_ccr6`]
module*/
pub type TIM7_CCR6 = crate::Reg<tim7_ccr6::TIM7_CCR6rs>;
///TIM7 capture/compare register 6
pub mod tim7_ccr6;
