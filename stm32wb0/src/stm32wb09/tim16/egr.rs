///Register `EGR` reader
pub type R = crate::R<EGRrs>;
///Register `EGR` writer
pub type W = crate::W<EGRrs>;
/**UG: Update generation This bit can be set by software, it is automatically cleared by hardware. 0: No action. 1: Reinitialize the counter and generates an update of the registers. Note that the prescaler counter is cleared too (anyway the prescaler ratio is not affected).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UG {
    ///1: Re-initializes the timer counter and generates an update of the registers.
    Update = 1,
}
impl From<UG> for bool {
    #[inline(always)]
    fn from(variant: UG) -> Self {
        variant as u8 != 0
    }
}
///Field `UG` writer - UG: Update generation This bit can be set by software, it is automatically cleared by hardware. 0: No action. 1: Reinitialize the counter and generates an update of the registers. Note that the prescaler counter is cleared too (anyway the prescaler ratio is not affected).
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG, UG>;
impl<'a, REG> UG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Re-initializes the timer counter and generates an update of the registers.
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(UG::Update)
    }
}
/**Capture/compare %s generation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1GW {
    ///1: If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register.
    Trigger = 1,
}
impl From<CC1GW> for bool {
    #[inline(always)]
    fn from(variant: CC1GW) -> Self {
        variant as u8 != 0
    }
}
///Field `CCG(1-1)` writer - Capture/compare %s generation
pub type CCG_W<'a, REG> = crate::BitWriter<'a, REG, CC1GW>;
impl<'a, REG> CCG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register.
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(CC1GW::Trigger)
    }
}
/**COMG: Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware. 0: No action 1: When the CCPC bit is set, it is possible to update the CCxE, CCxNE and OCxM bits Note: This bit acts only on channels that have a complementary output.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMGW {
    ///1: When CCPC bit is set, it allows CCxE, CCxNE and OCxM bits to be updated
    Trigger = 1,
}
impl From<COMGW> for bool {
    #[inline(always)]
    fn from(variant: COMGW) -> Self {
        variant as u8 != 0
    }
}
///Field `COMG` writer - COMG: Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware. 0: No action 1: When the CCPC bit is set, it is possible to update the CCxE, CCxNE and OCxM bits Note: This bit acts only on channels that have a complementary output.
pub type COMG_W<'a, REG> = crate::BitWriter<'a, REG, COMGW>;
impl<'a, REG> COMG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When CCPC bit is set, it allows CCxE, CCxNE and OCxM bits to be updated
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(COMGW::Trigger)
    }
}
/**BG: Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action. 1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BGW {
    ///1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled
    Trigger = 1,
}
impl From<BGW> for bool {
    #[inline(always)]
    fn from(variant: BGW) -> Self {
        variant as u8 != 0
    }
}
///Field `BG` writer - BG: Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action. 1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled.
pub type BG_W<'a, REG> = crate::BitWriter<'a, REG, BGW>;
impl<'a, REG> BG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(BGW::Trigger)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EGR").finish()
    }
}
impl W {
    ///Bit 0 - UG: Update generation This bit can be set by software, it is automatically cleared by hardware. 0: No action. 1: Reinitialize the counter and generates an update of the registers. Note that the prescaler counter is cleared too (anyway the prescaler ratio is not affected).
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W<'_, EGRrs> {
        UG_W::new(self, 0)
    }
    ///Capture/compare (1-1) generation
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1G` field.</div>
    #[inline(always)]
    pub fn ccg(&mut self, n: u8) -> CCG_W<'_, EGRrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCG_W::new(self, n * 0 + 1)
    }
    ///Bit 1 - Capture/compare 1 generation
    #[inline(always)]
    pub fn cc1g(&mut self) -> CCG_W<'_, EGRrs> {
        CCG_W::new(self, 1)
    }
    ///Bit 5 - COMG: Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware. 0: No action 1: When the CCPC bit is set, it is possible to update the CCxE, CCxNE and OCxM bits Note: This bit acts only on channels that have a complementary output.
    #[inline(always)]
    pub fn comg(&mut self) -> COMG_W<'_, EGRrs> {
        COMG_W::new(self, 5)
    }
    ///Bit 7 - BG: Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action. 1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled.
    #[inline(always)]
    pub fn bg(&mut self) -> BG_W<'_, EGRrs> {
        BG_W::new(self, 7)
    }
}
/**EGR register

You can [`read`](crate::Reg::read) this register and get [`egr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TIM16:EGR)*/
pub struct EGRrs;
impl crate::RegisterSpec for EGRrs {
    type Ux = u16;
}
///`read()` method returns [`egr::R`](R) reader structure
impl crate::Readable for EGRrs {}
///`write(|w| ..)` method takes [`egr::W`](W) writer structure
impl crate::Writable for EGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EGR to value 0
impl crate::Resettable for EGRrs {}
