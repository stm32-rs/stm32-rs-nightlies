#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tim8_cr1: TIM8_CR1,
    _reserved1: [u8; 0x02],
    tim8_cr2: TIM8_CR2,
    tim8_smcr: TIM8_SMCR,
    tim8_dier: TIM8_DIER,
    _reserved4: [u8; 0x02],
    tim8_sr: TIM8_SR,
    tim8_egr: TIM8_EGR,
    _reserved6: [u8; 0x02],
    tim8_ccmr1alternate8: TIM8_CCMR1ALTERNATE8,
    tim8_ccmr2alternate24: TIM8_CCMR2ALTERNATE24,
    tim8_ccer: TIM8_CCER,
    tim8_cnt: TIM8_CNT,
    tim8_psc: TIM8_PSC,
    _reserved11: [u8; 0x02],
    tim8_arr: TIM8_ARR,
    _reserved12: [u8; 0x02],
    tim8_rcr: TIM8_RCR,
    _reserved13: [u8; 0x02],
    tim8_ccr1: TIM8_CCR1,
    _reserved14: [u8; 0x02],
    tim8_ccr2: TIM8_CCR2,
    _reserved15: [u8; 0x02],
    tim8_ccr3: TIM8_CCR3,
    _reserved16: [u8; 0x02],
    tim8_ccr4: TIM8_CCR4,
    _reserved17: [u8; 0x02],
    tim8_bdtr: TIM8_BDTR,
    tim8_dcr: TIM8_DCR,
    _reserved19: [u8; 0x02],
    tim8_dmar: TIM8_DMAR,
    _reserved20: [u8; 0x04],
    tim8_ccmr3: TIM8_CCMR3,
    tim8_ccr5: TIM8_CCR5,
    tim8_ccr6: TIM8_CCR6,
    _reserved23: [u8; 0x02],
    tim8_af1: TIM8_AF1,
    tim8_af2: TIM8_AF2,
    tim8_tisel: TIM8_TISEL,
}
impl RegisterBlock {
    ///0x00 - TIM8 control register 1
    #[inline(always)]
    pub const fn tim8_cr1(&self) -> &TIM8_CR1 {
        &self.tim8_cr1
    }
    ///0x04 - TIM8 control register 2
    #[inline(always)]
    pub const fn tim8_cr2(&self) -> &TIM8_CR2 {
        &self.tim8_cr2
    }
    ///0x08 - TIM8 slave mode control register
    #[inline(always)]
    pub const fn tim8_smcr(&self) -> &TIM8_SMCR {
        &self.tim8_smcr
    }
    ///0x0c - TIM8 DMA/interrupt enable register
    #[inline(always)]
    pub const fn tim8_dier(&self) -> &TIM8_DIER {
        &self.tim8_dier
    }
    ///0x10 - TIM8 status register
    #[inline(always)]
    pub const fn tim8_sr(&self) -> &TIM8_SR {
        &self.tim8_sr
    }
    ///0x14 - TIM8 event generation register
    #[inline(always)]
    pub const fn tim8_egr(&self) -> &TIM8_EGR {
        &self.tim8_egr
    }
    ///0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    #[inline(always)]
    pub const fn tim8_ccmr1alternate8(&self) -> &TIM8_CCMR1ALTERNATE8 {
        &self.tim8_ccmr1alternate8
    }
    ///0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    #[inline(always)]
    pub const fn tim8_ccmr2alternate24(&self) -> &TIM8_CCMR2ALTERNATE24 {
        &self.tim8_ccmr2alternate24
    }
    ///0x20 - TIM8 capture/compare enable register
    #[inline(always)]
    pub const fn tim8_ccer(&self) -> &TIM8_CCER {
        &self.tim8_ccer
    }
    ///0x24 - TIM8 counter
    #[inline(always)]
    pub const fn tim8_cnt(&self) -> &TIM8_CNT {
        &self.tim8_cnt
    }
    ///0x28 - TIM8 prescaler
    #[inline(always)]
    pub const fn tim8_psc(&self) -> &TIM8_PSC {
        &self.tim8_psc
    }
    ///0x2c - TIM8 auto-reload register
    #[inline(always)]
    pub const fn tim8_arr(&self) -> &TIM8_ARR {
        &self.tim8_arr
    }
    ///0x30 - TIM8 repetition counter register
    #[inline(always)]
    pub const fn tim8_rcr(&self) -> &TIM8_RCR {
        &self.tim8_rcr
    }
    ///0x34 - TIM8 capture/compare register 1
    #[inline(always)]
    pub const fn tim8_ccr1(&self) -> &TIM8_CCR1 {
        &self.tim8_ccr1
    }
    ///0x38 - TIM8 capture/compare register 2
    #[inline(always)]
    pub const fn tim8_ccr2(&self) -> &TIM8_CCR2 {
        &self.tim8_ccr2
    }
    ///0x3c - TIM8 capture/compare register 3
    #[inline(always)]
    pub const fn tim8_ccr3(&self) -> &TIM8_CCR3 {
        &self.tim8_ccr3
    }
    ///0x40 - TIM8 capture/compare register 4
    #[inline(always)]
    pub const fn tim8_ccr4(&self) -> &TIM8_CCR4 {
        &self.tim8_ccr4
    }
    /**0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
    can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.*/
    #[inline(always)]
    pub const fn tim8_bdtr(&self) -> &TIM8_BDTR {
        &self.tim8_bdtr
    }
    ///0x48 - TIM8 DMA control register
    #[inline(always)]
    pub const fn tim8_dcr(&self) -> &TIM8_DCR {
        &self.tim8_dcr
    }
    ///0x4c - TIM8 DMA address for full transfer
    #[inline(always)]
    pub const fn tim8_dmar(&self) -> &TIM8_DMAR {
        &self.tim8_dmar
    }
    ///0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:
    #[inline(always)]
    pub const fn tim8_ccmr3(&self) -> &TIM8_CCMR3 {
        &self.tim8_ccmr3
    }
    ///0x58 - TIM8 capture/compare register 5
    #[inline(always)]
    pub const fn tim8_ccr5(&self) -> &TIM8_CCR5 {
        &self.tim8_ccr5
    }
    ///0x5c - TIM8 capture/compare register 6
    #[inline(always)]
    pub const fn tim8_ccr6(&self) -> &TIM8_CCR6 {
        &self.tim8_ccr6
    }
    ///0x60 - TIM8 Alternate function option register 1
    #[inline(always)]
    pub const fn tim8_af1(&self) -> &TIM8_AF1 {
        &self.tim8_af1
    }
    ///0x64 - TIM8 Alternate function option register 2
    #[inline(always)]
    pub const fn tim8_af2(&self) -> &TIM8_AF2 {
        &self.tim8_af2
    }
    ///0x68 - TIM8 timer input selection register
    #[inline(always)]
    pub const fn tim8_tisel(&self) -> &TIM8_TISEL {
        &self.tim8_tisel
    }
}
/**TIM8_CR1 (rw) register accessor: TIM8 control register 1

You can [`read`](crate::Reg::read) this register and get [`tim8_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_CR1)

For information about available fields see [`mod@tim8_cr1`]
module*/
pub type TIM8_CR1 = crate::Reg<tim8_cr1::TIM8_CR1rs>;
///TIM8 control register 1
pub mod tim8_cr1;
/**TIM8_CR2 (rw) register accessor: TIM8 control register 2

You can [`read`](crate::Reg::read) this register and get [`tim8_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_CR2)

For information about available fields see [`mod@tim8_cr2`]
module*/
pub type TIM8_CR2 = crate::Reg<tim8_cr2::TIM8_CR2rs>;
///TIM8 control register 2
pub mod tim8_cr2;
/**TIM8_SMCR (rw) register accessor: TIM8 slave mode control register

You can [`read`](crate::Reg::read) this register and get [`tim8_smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_SMCR)

For information about available fields see [`mod@tim8_smcr`]
module*/
pub type TIM8_SMCR = crate::Reg<tim8_smcr::TIM8_SMCRrs>;
///TIM8 slave mode control register
pub mod tim8_smcr;
/**TIM8_DIER (rw) register accessor: TIM8 DMA/interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tim8_dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_DIER)

For information about available fields see [`mod@tim8_dier`]
module*/
pub type TIM8_DIER = crate::Reg<tim8_dier::TIM8_DIERrs>;
///TIM8 DMA/interrupt enable register
pub mod tim8_dier;
/**TIM8_SR (rw) register accessor: TIM8 status register

You can [`read`](crate::Reg::read) this register and get [`tim8_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_SR)

For information about available fields see [`mod@tim8_sr`]
module*/
pub type TIM8_SR = crate::Reg<tim8_sr::TIM8_SRrs>;
///TIM8 status register
pub mod tim8_sr;
/**TIM8_EGR (w) register accessor: TIM8 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_EGR)

For information about available fields see [`mod@tim8_egr`]
module*/
pub type TIM8_EGR = crate::Reg<tim8_egr::TIM8_EGRrs>;
///TIM8 event generation register
pub mod tim8_egr;
/**TIM8_CCMR1ALTERNATE8 (rw) register accessor: The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:

You can [`read`](crate::Reg::read) this register and get [`tim8_ccmr1alternate8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_ccmr1alternate8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_CCMR1ALTERNATE8)

For information about available fields see [`mod@tim8_ccmr1alternate8`]
module*/
pub type TIM8_CCMR1ALTERNATE8 = crate::Reg<tim8_ccmr1alternate8::TIM8_CCMR1ALTERNATE8rs>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod tim8_ccmr1alternate8;
/**TIM8_CCMR2ALTERNATE24 (rw) register accessor: The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:

You can [`read`](crate::Reg::read) this register and get [`tim8_ccmr2alternate24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_ccmr2alternate24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_CCMR2ALTERNATE24)

For information about available fields see [`mod@tim8_ccmr2alternate24`]
module*/
pub type TIM8_CCMR2ALTERNATE24 = crate::Reg<tim8_ccmr2alternate24::TIM8_CCMR2ALTERNATE24rs>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod tim8_ccmr2alternate24;
/**TIM8_CCER (rw) register accessor: TIM8 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`tim8_ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_CCER)

For information about available fields see [`mod@tim8_ccer`]
module*/
pub type TIM8_CCER = crate::Reg<tim8_ccer::TIM8_CCERrs>;
///TIM8 capture/compare enable register
pub mod tim8_ccer;
/**TIM8_CNT (rw) register accessor: TIM8 counter

You can [`read`](crate::Reg::read) this register and get [`tim8_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_CNT)

For information about available fields see [`mod@tim8_cnt`]
module*/
pub type TIM8_CNT = crate::Reg<tim8_cnt::TIM8_CNTrs>;
///TIM8 counter
pub mod tim8_cnt;
/**TIM8_PSC (rw) register accessor: TIM8 prescaler

You can [`read`](crate::Reg::read) this register and get [`tim8_psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_PSC)

For information about available fields see [`mod@tim8_psc`]
module*/
pub type TIM8_PSC = crate::Reg<tim8_psc::TIM8_PSCrs>;
///TIM8 prescaler
pub mod tim8_psc;
/**TIM8_ARR (rw) register accessor: TIM8 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`tim8_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_ARR)

For information about available fields see [`mod@tim8_arr`]
module*/
pub type TIM8_ARR = crate::Reg<tim8_arr::TIM8_ARRrs>;
///TIM8 auto-reload register
pub mod tim8_arr;
/**TIM8_RCR (rw) register accessor: TIM8 repetition counter register

You can [`read`](crate::Reg::read) this register and get [`tim8_rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_RCR)

For information about available fields see [`mod@tim8_rcr`]
module*/
pub type TIM8_RCR = crate::Reg<tim8_rcr::TIM8_RCRrs>;
///TIM8 repetition counter register
pub mod tim8_rcr;
/**TIM8_CCR1 (rw) register accessor: TIM8 capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`tim8_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_CCR1)

For information about available fields see [`mod@tim8_ccr1`]
module*/
pub type TIM8_CCR1 = crate::Reg<tim8_ccr1::TIM8_CCR1rs>;
///TIM8 capture/compare register 1
pub mod tim8_ccr1;
/**TIM8_CCR2 (rw) register accessor: TIM8 capture/compare register 2

You can [`read`](crate::Reg::read) this register and get [`tim8_ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_CCR2)

For information about available fields see [`mod@tim8_ccr2`]
module*/
pub type TIM8_CCR2 = crate::Reg<tim8_ccr2::TIM8_CCR2rs>;
///TIM8 capture/compare register 2
pub mod tim8_ccr2;
/**TIM8_CCR3 (rw) register accessor: TIM8 capture/compare register 3

You can [`read`](crate::Reg::read) this register and get [`tim8_ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_CCR3)

For information about available fields see [`mod@tim8_ccr3`]
module*/
pub type TIM8_CCR3 = crate::Reg<tim8_ccr3::TIM8_CCR3rs>;
///TIM8 capture/compare register 3
pub mod tim8_ccr3;
/**TIM8_CCR4 (rw) register accessor: TIM8 capture/compare register 4

You can [`read`](crate::Reg::read) this register and get [`tim8_ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_CCR4)

For information about available fields see [`mod@tim8_ccr4`]
module*/
pub type TIM8_CCR4 = crate::Reg<tim8_ccr4::TIM8_CCR4rs>;
///TIM8 capture/compare register 4
pub mod tim8_ccr4;
/**TIM8_BDTR (rw) register accessor: As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.

You can [`read`](crate::Reg::read) this register and get [`tim8_bdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_bdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_BDTR)

For information about available fields see [`mod@tim8_bdtr`]
module*/
pub type TIM8_BDTR = crate::Reg<tim8_bdtr::TIM8_BDTRrs>;
/**As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.*/
pub mod tim8_bdtr;
/**TIM8_DCR (rw) register accessor: TIM8 DMA control register

You can [`read`](crate::Reg::read) this register and get [`tim8_dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_DCR)

For information about available fields see [`mod@tim8_dcr`]
module*/
pub type TIM8_DCR = crate::Reg<tim8_dcr::TIM8_DCRrs>;
///TIM8 DMA control register
pub mod tim8_dcr;
/**TIM8_DMAR (rw) register accessor: TIM8 DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`tim8_dmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_dmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_DMAR)

For information about available fields see [`mod@tim8_dmar`]
module*/
pub type TIM8_DMAR = crate::Reg<tim8_dmar::TIM8_DMARrs>;
///TIM8 DMA address for full transfer
pub mod tim8_dmar;
/**TIM8_CCMR3 (rw) register accessor: The channels 5 and 6 can only be configured in output. Output compare mode:

You can [`read`](crate::Reg::read) this register and get [`tim8_ccmr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_ccmr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_CCMR3)

For information about available fields see [`mod@tim8_ccmr3`]
module*/
pub type TIM8_CCMR3 = crate::Reg<tim8_ccmr3::TIM8_CCMR3rs>;
///The channels 5 and 6 can only be configured in output. Output compare mode:
pub mod tim8_ccmr3;
/**TIM8_CCR5 (rw) register accessor: TIM8 capture/compare register 5

You can [`read`](crate::Reg::read) this register and get [`tim8_ccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_ccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_CCR5)

For information about available fields see [`mod@tim8_ccr5`]
module*/
pub type TIM8_CCR5 = crate::Reg<tim8_ccr5::TIM8_CCR5rs>;
///TIM8 capture/compare register 5
pub mod tim8_ccr5;
/**TIM8_CCR6 (rw) register accessor: TIM8 capture/compare register 6

You can [`read`](crate::Reg::read) this register and get [`tim8_ccr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_ccr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_CCR6)

For information about available fields see [`mod@tim8_ccr6`]
module*/
pub type TIM8_CCR6 = crate::Reg<tim8_ccr6::TIM8_CCR6rs>;
///TIM8 capture/compare register 6
pub mod tim8_ccr6;
/**TIM8_AF1 (rw) register accessor: TIM8 Alternate function option register 1

You can [`read`](crate::Reg::read) this register and get [`tim8_af1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_af1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_AF1)

For information about available fields see [`mod@tim8_af1`]
module*/
pub type TIM8_AF1 = crate::Reg<tim8_af1::TIM8_AF1rs>;
///TIM8 Alternate function option register 1
pub mod tim8_af1;
/**TIM8_AF2 (rw) register accessor: TIM8 Alternate function option register 2

You can [`read`](crate::Reg::read) this register and get [`tim8_af2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_af2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_AF2)

For information about available fields see [`mod@tim8_af2`]
module*/
pub type TIM8_AF2 = crate::Reg<tim8_af2::TIM8_AF2rs>;
///TIM8 Alternate function option register 2
pub mod tim8_af2;
/**TIM8_TISEL (rw) register accessor: TIM8 timer input selection register

You can [`read`](crate::Reg::read) this register and get [`tim8_tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim8_tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:TIM8_TISEL)

For information about available fields see [`mod@tim8_tisel`]
module*/
pub type TIM8_TISEL = crate::Reg<tim8_tisel::TIM8_TISELrs>;
///TIM8 timer input selection register
pub mod tim8_tisel;
