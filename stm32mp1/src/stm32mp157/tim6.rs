#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tim6_cr1: TIM6_CR1,
    _reserved1: [u8; 0x02],
    tim6_cr2: TIM6_CR2,
    tim6_smcr: TIM6_SMCR,
    tim6_dier: TIM6_DIER,
    _reserved4: [u8; 0x02],
    tim6_sr: TIM6_SR,
    tim6_egr: TIM6_EGR,
    _reserved6: [u8; 0x02],
    tim6_ccmr1alternate6: TIM6_CCMR1ALTERNATE6,
    tim6_ccmr2alternate22: TIM6_CCMR2ALTERNATE22,
    tim6_ccer: TIM6_CCER,
    tim6_cnt: TIM6_CNT,
    tim6_psc: TIM6_PSC,
    _reserved11: [u8; 0x02],
    tim6_arr: TIM6_ARR,
    _reserved12: [u8; 0x02],
    tim6_rcr: TIM6_RCR,
    _reserved13: [u8; 0x02],
    tim6_ccr1: TIM6_CCR1,
    _reserved14: [u8; 0x02],
    tim6_ccr2: TIM6_CCR2,
    _reserved15: [u8; 0x02],
    tim6_ccr3: TIM6_CCR3,
    _reserved16: [u8; 0x02],
    tim6_ccr4: TIM6_CCR4,
    _reserved17: [u8; 0x02],
    tim6_bdtr: TIM6_BDTR,
    tim6_dcr: TIM6_DCR,
    _reserved19: [u8; 0x02],
    tim6_dmar: TIM6_DMAR,
    _reserved20: [u8; 0x04],
    tim6_ccmr3: TIM6_CCMR3,
    tim6_ccr5: TIM6_CCR5,
    tim6_ccr6: TIM6_CCR6,
}
impl RegisterBlock {
    ///0x00 - TIM6 control register 1
    #[inline(always)]
    pub const fn tim6_cr1(&self) -> &TIM6_CR1 {
        &self.tim6_cr1
    }
    ///0x04 - TIM6 control register 2
    #[inline(always)]
    pub const fn tim6_cr2(&self) -> &TIM6_CR2 {
        &self.tim6_cr2
    }
    ///0x08 - TIM6 slave mode control register
    #[inline(always)]
    pub const fn tim6_smcr(&self) -> &TIM6_SMCR {
        &self.tim6_smcr
    }
    ///0x0c - TIM6 DMA/interrupt enable register
    #[inline(always)]
    pub const fn tim6_dier(&self) -> &TIM6_DIER {
        &self.tim6_dier
    }
    ///0x10 - TIM6 status register
    #[inline(always)]
    pub const fn tim6_sr(&self) -> &TIM6_SR {
        &self.tim6_sr
    }
    ///0x14 - TIM6 event generation register
    #[inline(always)]
    pub const fn tim6_egr(&self) -> &TIM6_EGR {
        &self.tim6_egr
    }
    ///0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    #[inline(always)]
    pub const fn tim6_ccmr1alternate6(&self) -> &TIM6_CCMR1ALTERNATE6 {
        &self.tim6_ccmr1alternate6
    }
    ///0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    #[inline(always)]
    pub const fn tim6_ccmr2alternate22(&self) -> &TIM6_CCMR2ALTERNATE22 {
        &self.tim6_ccmr2alternate22
    }
    ///0x20 - TIM6 capture/compare enable register
    #[inline(always)]
    pub const fn tim6_ccer(&self) -> &TIM6_CCER {
        &self.tim6_ccer
    }
    ///0x24 - TIM6 counter
    #[inline(always)]
    pub const fn tim6_cnt(&self) -> &TIM6_CNT {
        &self.tim6_cnt
    }
    ///0x28 - TIM6 prescaler
    #[inline(always)]
    pub const fn tim6_psc(&self) -> &TIM6_PSC {
        &self.tim6_psc
    }
    ///0x2c - TIM6 auto-reload register
    #[inline(always)]
    pub const fn tim6_arr(&self) -> &TIM6_ARR {
        &self.tim6_arr
    }
    ///0x30 - TIM6 repetition counter register
    #[inline(always)]
    pub const fn tim6_rcr(&self) -> &TIM6_RCR {
        &self.tim6_rcr
    }
    ///0x34 - TIM6 capture/compare register 1
    #[inline(always)]
    pub const fn tim6_ccr1(&self) -> &TIM6_CCR1 {
        &self.tim6_ccr1
    }
    ///0x38 - TIM6 capture/compare register 2
    #[inline(always)]
    pub const fn tim6_ccr2(&self) -> &TIM6_CCR2 {
        &self.tim6_ccr2
    }
    ///0x3c - TIM6 capture/compare register 3
    #[inline(always)]
    pub const fn tim6_ccr3(&self) -> &TIM6_CCR3 {
        &self.tim6_ccr3
    }
    ///0x40 - TIM6 capture/compare register 4
    #[inline(always)]
    pub const fn tim6_ccr4(&self) -> &TIM6_CCR4 {
        &self.tim6_ccr4
    }
    /**0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
    can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.*/
    #[inline(always)]
    pub const fn tim6_bdtr(&self) -> &TIM6_BDTR {
        &self.tim6_bdtr
    }
    ///0x48 - TIM6 DMA control register
    #[inline(always)]
    pub const fn tim6_dcr(&self) -> &TIM6_DCR {
        &self.tim6_dcr
    }
    ///0x4c - TIM6 DMA address for full transfer
    #[inline(always)]
    pub const fn tim6_dmar(&self) -> &TIM6_DMAR {
        &self.tim6_dmar
    }
    ///0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:
    #[inline(always)]
    pub const fn tim6_ccmr3(&self) -> &TIM6_CCMR3 {
        &self.tim6_ccmr3
    }
    ///0x58 - TIM6 capture/compare register 5
    #[inline(always)]
    pub const fn tim6_ccr5(&self) -> &TIM6_CCR5 {
        &self.tim6_ccr5
    }
    ///0x5c - TIM6 capture/compare register 6
    #[inline(always)]
    pub const fn tim6_ccr6(&self) -> &TIM6_CCR6 {
        &self.tim6_ccr6
    }
}
/**TIM6_CR1 (rw) register accessor: TIM6 control register 1

You can [`read`](crate::Reg::read) this register and get [`tim6_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM6:TIM6_CR1)

For information about available fields see [`mod@tim6_cr1`]
module*/
pub type TIM6_CR1 = crate::Reg<tim6_cr1::TIM6_CR1rs>;
///TIM6 control register 1
pub mod tim6_cr1;
/**TIM6_CR2 (rw) register accessor: TIM6 control register 2

You can [`read`](crate::Reg::read) this register and get [`tim6_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM6:TIM6_CR2)

For information about available fields see [`mod@tim6_cr2`]
module*/
pub type TIM6_CR2 = crate::Reg<tim6_cr2::TIM6_CR2rs>;
///TIM6 control register 2
pub mod tim6_cr2;
/**TIM6_SMCR (rw) register accessor: TIM6 slave mode control register

You can [`read`](crate::Reg::read) this register and get [`tim6_smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM6:TIM6_SMCR)

For information about available fields see [`mod@tim6_smcr`]
module*/
pub type TIM6_SMCR = crate::Reg<tim6_smcr::TIM6_SMCRrs>;
///TIM6 slave mode control register
pub mod tim6_smcr;
/**TIM6_DIER (rw) register accessor: TIM6 DMA/interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tim6_dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM6:TIM6_DIER)

For information about available fields see [`mod@tim6_dier`]
module*/
pub type TIM6_DIER = crate::Reg<tim6_dier::TIM6_DIERrs>;
///TIM6 DMA/interrupt enable register
pub mod tim6_dier;
/**TIM6_SR (rw) register accessor: TIM6 status register

You can [`read`](crate::Reg::read) this register and get [`tim6_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM6:TIM6_SR)

For information about available fields see [`mod@tim6_sr`]
module*/
pub type TIM6_SR = crate::Reg<tim6_sr::TIM6_SRrs>;
///TIM6 status register
pub mod tim6_sr;
/**TIM6_EGR (w) register accessor: TIM6 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM6:TIM6_EGR)

For information about available fields see [`mod@tim6_egr`]
module*/
pub type TIM6_EGR = crate::Reg<tim6_egr::TIM6_EGRrs>;
///TIM6 event generation register
pub mod tim6_egr;
/**TIM6_CCMR1ALTERNATE6 (rw) register accessor: The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:

You can [`read`](crate::Reg::read) this register and get [`tim6_ccmr1alternate6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_ccmr1alternate6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM6:TIM6_CCMR1ALTERNATE6)

For information about available fields see [`mod@tim6_ccmr1alternate6`]
module*/
pub type TIM6_CCMR1ALTERNATE6 = crate::Reg<tim6_ccmr1alternate6::TIM6_CCMR1ALTERNATE6rs>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod tim6_ccmr1alternate6;
/**TIM6_CCMR2ALTERNATE22 (rw) register accessor: The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:

You can [`read`](crate::Reg::read) this register and get [`tim6_ccmr2alternate22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_ccmr2alternate22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM6:TIM6_CCMR2ALTERNATE22)

For information about available fields see [`mod@tim6_ccmr2alternate22`]
module*/
pub type TIM6_CCMR2ALTERNATE22 = crate::Reg<tim6_ccmr2alternate22::TIM6_CCMR2ALTERNATE22rs>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod tim6_ccmr2alternate22;
/**TIM6_CCER (rw) register accessor: TIM6 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`tim6_ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM6:TIM6_CCER)

For information about available fields see [`mod@tim6_ccer`]
module*/
pub type TIM6_CCER = crate::Reg<tim6_ccer::TIM6_CCERrs>;
///TIM6 capture/compare enable register
pub mod tim6_ccer;
/**TIM6_CNT (rw) register accessor: TIM6 counter

You can [`read`](crate::Reg::read) this register and get [`tim6_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM6:TIM6_CNT)

For information about available fields see [`mod@tim6_cnt`]
module*/
pub type TIM6_CNT = crate::Reg<tim6_cnt::TIM6_CNTrs>;
///TIM6 counter
pub mod tim6_cnt;
/**TIM6_PSC (rw) register accessor: TIM6 prescaler

You can [`read`](crate::Reg::read) this register and get [`tim6_psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM6:TIM6_PSC)

For information about available fields see [`mod@tim6_psc`]
module*/
pub type TIM6_PSC = crate::Reg<tim6_psc::TIM6_PSCrs>;
///TIM6 prescaler
pub mod tim6_psc;
/**TIM6_ARR (rw) register accessor: TIM6 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`tim6_arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM6:TIM6_ARR)

For information about available fields see [`mod@tim6_arr`]
module*/
pub type TIM6_ARR = crate::Reg<tim6_arr::TIM6_ARRrs>;
///TIM6 auto-reload register
pub mod tim6_arr;
/**TIM6_RCR (rw) register accessor: TIM6 repetition counter register

You can [`read`](crate::Reg::read) this register and get [`tim6_rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM6:TIM6_RCR)

For information about available fields see [`mod@tim6_rcr`]
module*/
pub type TIM6_RCR = crate::Reg<tim6_rcr::TIM6_RCRrs>;
///TIM6 repetition counter register
pub mod tim6_rcr;
/**TIM6_CCR1 (rw) register accessor: TIM6 capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`tim6_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM6:TIM6_CCR1)

For information about available fields see [`mod@tim6_ccr1`]
module*/
pub type TIM6_CCR1 = crate::Reg<tim6_ccr1::TIM6_CCR1rs>;
///TIM6 capture/compare register 1
pub mod tim6_ccr1;
/**TIM6_CCR2 (rw) register accessor: TIM6 capture/compare register 2

You can [`read`](crate::Reg::read) this register and get [`tim6_ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM6:TIM6_CCR2)

For information about available fields see [`mod@tim6_ccr2`]
module*/
pub type TIM6_CCR2 = crate::Reg<tim6_ccr2::TIM6_CCR2rs>;
///TIM6 capture/compare register 2
pub mod tim6_ccr2;
/**TIM6_CCR3 (rw) register accessor: TIM6 capture/compare register 3

You can [`read`](crate::Reg::read) this register and get [`tim6_ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM6:TIM6_CCR3)

For information about available fields see [`mod@tim6_ccr3`]
module*/
pub type TIM6_CCR3 = crate::Reg<tim6_ccr3::TIM6_CCR3rs>;
///TIM6 capture/compare register 3
pub mod tim6_ccr3;
/**TIM6_CCR4 (rw) register accessor: TIM6 capture/compare register 4

You can [`read`](crate::Reg::read) this register and get [`tim6_ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM6:TIM6_CCR4)

For information about available fields see [`mod@tim6_ccr4`]
module*/
pub type TIM6_CCR4 = crate::Reg<tim6_ccr4::TIM6_CCR4rs>;
///TIM6 capture/compare register 4
pub mod tim6_ccr4;
/**TIM6_BDTR (rw) register accessor: As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.

You can [`read`](crate::Reg::read) this register and get [`tim6_bdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_bdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM6:TIM6_BDTR)

For information about available fields see [`mod@tim6_bdtr`]
module*/
pub type TIM6_BDTR = crate::Reg<tim6_bdtr::TIM6_BDTRrs>;
/**As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.*/
pub mod tim6_bdtr;
/**TIM6_DCR (rw) register accessor: TIM6 DMA control register

You can [`read`](crate::Reg::read) this register and get [`tim6_dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM6:TIM6_DCR)

For information about available fields see [`mod@tim6_dcr`]
module*/
pub type TIM6_DCR = crate::Reg<tim6_dcr::TIM6_DCRrs>;
///TIM6 DMA control register
pub mod tim6_dcr;
/**TIM6_DMAR (rw) register accessor: TIM6 DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`tim6_dmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_dmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM6:TIM6_DMAR)

For information about available fields see [`mod@tim6_dmar`]
module*/
pub type TIM6_DMAR = crate::Reg<tim6_dmar::TIM6_DMARrs>;
///TIM6 DMA address for full transfer
pub mod tim6_dmar;
/**TIM6_CCMR3 (rw) register accessor: The channels 5 and 6 can only be configured in output. Output compare mode:

You can [`read`](crate::Reg::read) this register and get [`tim6_ccmr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_ccmr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM6:TIM6_CCMR3)

For information about available fields see [`mod@tim6_ccmr3`]
module*/
pub type TIM6_CCMR3 = crate::Reg<tim6_ccmr3::TIM6_CCMR3rs>;
///The channels 5 and 6 can only be configured in output. Output compare mode:
pub mod tim6_ccmr3;
/**TIM6_CCR5 (rw) register accessor: TIM6 capture/compare register 5

You can [`read`](crate::Reg::read) this register and get [`tim6_ccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_ccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM6:TIM6_CCR5)

For information about available fields see [`mod@tim6_ccr5`]
module*/
pub type TIM6_CCR5 = crate::Reg<tim6_ccr5::TIM6_CCR5rs>;
///TIM6 capture/compare register 5
pub mod tim6_ccr5;
/**TIM6_CCR6 (rw) register accessor: TIM6 capture/compare register 6

You can [`read`](crate::Reg::read) this register and get [`tim6_ccr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_ccr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM6:TIM6_CCR6)

For information about available fields see [`mod@tim6_ccr6`]
module*/
pub type TIM6_CCR6 = crate::Reg<tim6_ccr6::TIM6_CCR6rs>;
///TIM6 capture/compare register 6
pub mod tim6_ccr6;
