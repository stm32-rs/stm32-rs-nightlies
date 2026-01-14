///Register `TIM1_EGR` writer
pub type W = crate::W<TIM1_EGRrs>;
/**Update generation This bit can be set by software, it is automatically cleared by hardware.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UG {
    ///0: No action
    B0x0 = 0,
    ///1: Reinitialize the counter and generates an update of the registers. The prescaler internal counter is also cleared (the prescaler ratio is not affected). The counter is cleared if the center-aligned mode is selected or if DIR=0 (upcounting), else it takes the auto-reload value (TIMx_ARR) if DIR=1 (downcounting).
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
    ///Reinitialize the counter and generates an update of the registers. The prescaler internal counter is also cleared (the prescaler ratio is not affected). The counter is cleared if the center-aligned mode is selected or if DIR=0 (upcounting), else it takes the auto-reload value (TIMx_ARR) if DIR=1 (downcounting).
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UG::B0x1)
    }
}
/**Capture/Compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high.

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
///Field `CC1G` writer - Capture/Compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high.
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
///Field `CC2G` writer - Capture/Compare 2 generation Refer to CC1G description
pub type CC2G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3G` writer - Capture/Compare 3 generation Refer to CC1G description
pub type CC3G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4G` writer - Capture/Compare 4 generation Refer to CC1G description
pub type CC4G_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware Note: This bit acts only on channels having a complementary output.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMG {
    ///0: No action
    B0x0 = 0,
    ///1: When CCPC bit is set, it allows CCxE, CCxNE and OCxM bits to be updated.
    B0x1 = 1,
}
impl From<COMG> for bool {
    #[inline(always)]
    fn from(variant: COMG) -> Self {
        variant as u8 != 0
    }
}
///Field `COMG` writer - Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware Note: This bit acts only on channels having a complementary output.
pub type COMG_W<'a, REG> = crate::BitWriter<'a, REG, COMG>;
impl<'a, REG> COMG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(COMG::B0x0)
    }
    ///When CCPC bit is set, it allows CCxE, CCxNE and OCxM bits to be updated.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(COMG::B0x1)
    }
}
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
/**Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BG {
    ///0: No action
    B0x0 = 0,
    ///1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled.
    B0x1 = 1,
}
impl From<BG> for bool {
    #[inline(always)]
    fn from(variant: BG) -> Self {
        variant as u8 != 0
    }
}
///Field `BG` writer - Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.
pub type BG_W<'a, REG> = crate::BitWriter<'a, REG, BG>;
impl<'a, REG> BG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BG::B0x0)
    }
    ///A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BG::B0x1)
    }
}
/**Break 2 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B2G {
    ///0: No action
    B0x0 = 0,
    ///1: A break 2 event is generated. MOE bit is cleared and B2IF flag is set. Related interrupt can occur if enabled.
    B0x1 = 1,
}
impl From<B2G> for bool {
    #[inline(always)]
    fn from(variant: B2G) -> Self {
        variant as u8 != 0
    }
}
///Field `B2G` writer - Break 2 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.
pub type B2G_W<'a, REG> = crate::BitWriter<'a, REG, B2G>;
impl<'a, REG> B2G_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(B2G::B0x0)
    }
    ///A break 2 event is generated. MOE bit is cleared and B2IF flag is set. Related interrupt can occur if enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(B2G::B0x1)
    }
}
impl core::fmt::Debug for crate::generic::Reg<TIM1_EGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Update generation This bit can be set by software, it is automatically cleared by hardware.
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W<'_, TIM1_EGRrs> {
        UG_W::new(self, 0)
    }
    ///Bit 1 - Capture/Compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high.
    #[inline(always)]
    pub fn cc1g(&mut self) -> CC1G_W<'_, TIM1_EGRrs> {
        CC1G_W::new(self, 1)
    }
    ///Bit 2 - Capture/Compare 2 generation Refer to CC1G description
    #[inline(always)]
    pub fn cc2g(&mut self) -> CC2G_W<'_, TIM1_EGRrs> {
        CC2G_W::new(self, 2)
    }
    ///Bit 3 - Capture/Compare 3 generation Refer to CC1G description
    #[inline(always)]
    pub fn cc3g(&mut self) -> CC3G_W<'_, TIM1_EGRrs> {
        CC3G_W::new(self, 3)
    }
    ///Bit 4 - Capture/Compare 4 generation Refer to CC1G description
    #[inline(always)]
    pub fn cc4g(&mut self) -> CC4G_W<'_, TIM1_EGRrs> {
        CC4G_W::new(self, 4)
    }
    ///Bit 5 - Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware Note: This bit acts only on channels having a complementary output.
    #[inline(always)]
    pub fn comg(&mut self) -> COMG_W<'_, TIM1_EGRrs> {
        COMG_W::new(self, 5)
    }
    ///Bit 6 - Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.
    #[inline(always)]
    pub fn tg(&mut self) -> TG_W<'_, TIM1_EGRrs> {
        TG_W::new(self, 6)
    }
    ///Bit 7 - Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.
    #[inline(always)]
    pub fn bg(&mut self) -> BG_W<'_, TIM1_EGRrs> {
        BG_W::new(self, 7)
    }
    ///Bit 8 - Break 2 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.
    #[inline(always)]
    pub fn b2g(&mut self) -> B2G_W<'_, TIM1_EGRrs> {
        B2G_W::new(self, 8)
    }
}
/**TIM1 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#TIM1:TIM1_EGR)*/
pub struct TIM1_EGRrs;
impl crate::RegisterSpec for TIM1_EGRrs {
    type Ux = u16;
}
///`write(|w| ..)` method takes [`tim1_egr::W`](W) writer structure
impl crate::Writable for TIM1_EGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM1_EGR to value 0
impl crate::Resettable for TIM1_EGRrs {}
