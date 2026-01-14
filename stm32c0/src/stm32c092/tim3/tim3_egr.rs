///Register `TIM3_EGR` writer
pub type W = crate::W<TIM3_EGRrs>;
/**Update generation This bit can be set by software, it is automatically cleared by hardware.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UG {
    ///0: No action
    B0x0 = 0,
    ///1: Re-initialize the counter and generates an update of the registers. Note that the prescaler counter is cleared too (anyway the prescaler ratio is not affected). The counter is cleared if the center-aligned mode is selected or if DIR=0 (upcounting), else it takes the auto-reload value (TIMx_ARR) if DIR=1 (downcounting).
    B0x1 = 1,
}
impl From<UG> for bool {
    #[inline(always)]
    fn from(variant: UG) -> Self {
        variant as u8 != 0
    }
}
///Field `UG` writer - Update generation This bit can be set by software, it is automatically cleared by hardware.
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG, UG>;
impl<'a, REG> UG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UG::B0x0)
    }
    ///Re-initialize the counter and generates an update of the registers. Note that the prescaler counter is cleared too (anyway the prescaler ratio is not affected). The counter is cleared if the center-aligned mode is selected or if DIR=0 (upcounting), else it takes the auto-reload value (TIMx_ARR) if DIR=1 (downcounting).
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UG::B0x1)
    }
}
/**Capture/compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1G {
    ///0: No action
    B0x0 = 0,
    ///1: A capture/compare event is generated on channel 1:
    B0x1 = 1,
}
impl From<CC1G> for bool {
    #[inline(always)]
    fn from(variant: CC1G) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1G` writer - Capture/compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high.
pub type CC1G_W<'a, REG> = crate::BitWriter<'a, REG, CC1G>;
impl<'a, REG> CC1G_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1G::B0x0)
    }
    ///A capture/compare event is generated on channel 1:
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1G::B0x1)
    }
}
///Field `CC2G` writer - Capture/compare 2 generation Refer to CC1G description
pub type CC2G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3G` writer - Capture/compare 3 generation Refer to CC1G description
pub type CC3G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4G` writer - Capture/compare 4 generation Refer to CC1G description
pub type CC4G_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TG {
    ///0: No action
    B0x0 = 0,
    ///1: The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled.
    B0x1 = 1,
}
impl From<TG> for bool {
    #[inline(always)]
    fn from(variant: TG) -> Self {
        variant as u8 != 0
    }
}
///Field `TG` writer - Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.
pub type TG_W<'a, REG> = crate::BitWriter<'a, REG, TG>;
impl<'a, REG> TG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TG::B0x0)
    }
    ///The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TG::B0x1)
    }
}
impl core::fmt::Debug for crate::generic::Reg<TIM3_EGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Update generation This bit can be set by software, it is automatically cleared by hardware.
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W<'_, TIM3_EGRrs> {
        UG_W::new(self, 0)
    }
    ///Bit 1 - Capture/compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high.
    #[inline(always)]
    pub fn cc1g(&mut self) -> CC1G_W<'_, TIM3_EGRrs> {
        CC1G_W::new(self, 1)
    }
    ///Bit 2 - Capture/compare 2 generation Refer to CC1G description
    #[inline(always)]
    pub fn cc2g(&mut self) -> CC2G_W<'_, TIM3_EGRrs> {
        CC2G_W::new(self, 2)
    }
    ///Bit 3 - Capture/compare 3 generation Refer to CC1G description
    #[inline(always)]
    pub fn cc3g(&mut self) -> CC3G_W<'_, TIM3_EGRrs> {
        CC3G_W::new(self, 3)
    }
    ///Bit 4 - Capture/compare 4 generation Refer to CC1G description
    #[inline(always)]
    pub fn cc4g(&mut self) -> CC4G_W<'_, TIM3_EGRrs> {
        CC4G_W::new(self, 4)
    }
    ///Bit 6 - Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.
    #[inline(always)]
    pub fn tg(&mut self) -> TG_W<'_, TIM3_EGRrs> {
        TG_W::new(self, 6)
    }
}
/**TIM3 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM3:TIM3_EGR)*/
pub struct TIM3_EGRrs;
impl crate::RegisterSpec for TIM3_EGRrs {
    type Ux = u16;
}
///`write(|w| ..)` method takes [`tim3_egr::W`](W) writer structure
impl crate::Writable for TIM3_EGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM3_EGR to value 0
impl crate::Resettable for TIM3_EGRrs {}
