///Register `TIM16_EGR` writer
pub type W = crate::W<TIM16_EGRrs>;
/**Update generation This bit can be set by software, it is automatically cleared by hardware.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UG {
    ///0: No action.
    B0x0 = 0,
    ///1: Reinitialize the counter and generates an update of the registers. Note that the prescaler counter is cleared too (anyway the prescaler ratio is not affected).
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
    ///No action.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UG::B0x0)
    }
    ///Reinitialize the counter and generates an update of the registers. Note that the prescaler counter is cleared too (anyway the prescaler ratio is not affected).
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
    ///0: No action.
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
    ///No action.
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
/**Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware. Note: This bit acts only on channels that have a complementary output.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMG {
    ///0: No action
    B0x0 = 0,
    ///1: When the CCPC bit is set, it is possible to update the CCxE, CCxNE and OCxM bits
    B0x1 = 1,
}
impl From<COMG> for bool {
    #[inline(always)]
    fn from(variant: COMG) -> Self {
        variant as u8 != 0
    }
}
///Field `COMG` writer - Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware. Note: This bit acts only on channels that have a complementary output.
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
    ///When the CCPC bit is set, it is possible to update the CCxE, CCxNE and OCxM bits
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(COMG::B0x1)
    }
}
/**Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BG {
    ///0: No action.
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
    ///No action.
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
impl core::fmt::Debug for crate::generic::Reg<TIM16_EGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Update generation This bit can be set by software, it is automatically cleared by hardware.
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W<'_, TIM16_EGRrs> {
        UG_W::new(self, 0)
    }
    ///Bit 1 - Capture/Compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high.
    #[inline(always)]
    pub fn cc1g(&mut self) -> CC1G_W<'_, TIM16_EGRrs> {
        CC1G_W::new(self, 1)
    }
    ///Bit 5 - Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware. Note: This bit acts only on channels that have a complementary output.
    #[inline(always)]
    pub fn comg(&mut self) -> COMG_W<'_, TIM16_EGRrs> {
        COMG_W::new(self, 5)
    }
    ///Bit 7 - Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware.
    #[inline(always)]
    pub fn bg(&mut self) -> BG_W<'_, TIM16_EGRrs> {
        BG_W::new(self, 7)
    }
}
/**TIM16 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM16:TIM16_EGR)*/
pub struct TIM16_EGRrs;
impl crate::RegisterSpec for TIM16_EGRrs {
    type Ux = u16;
}
///`write(|w| ..)` method takes [`tim16_egr::W`](W) writer structure
impl crate::Writable for TIM16_EGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM16_EGR to value 0
impl crate::Resettable for TIM16_EGRrs {}
