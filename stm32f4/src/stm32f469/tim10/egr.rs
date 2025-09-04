///Register `EGR` writer
pub type W = crate::W<EGRrs>;
/**Update generation

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
///Field `UG` writer - Update generation
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
impl core::fmt::Debug for crate::generic::Reg<EGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Update generation
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W<EGRrs> {
        UG_W::new(self, 0)
    }
    ///Capture/compare (1-1) generation
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1G` field.</div>
    #[inline(always)]
    pub fn ccg(&mut self, n: u8) -> CCG_W<EGRrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCG_W::new(self, n * 0 + 1)
    }
    ///Bit 1 - Capture/compare 1 generation
    #[inline(always)]
    pub fn cc1g(&mut self) -> CCG_W<EGRrs> {
        CCG_W::new(self, 1)
    }
}
/**event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#TIM10:EGR)*/
pub struct EGRrs;
impl crate::RegisterSpec for EGRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`egr::W`](W) writer structure
impl crate::Writable for EGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EGR to value 0
impl crate::Resettable for EGRrs {}
