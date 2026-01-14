///Register `TIM2_EGR` writer
pub type W = crate::W<TIM2_EGRrs>;
/**Update generation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UG {
    ///0: No action
    B0x0 = 0,
    ///1: Re-initialize the counter and generates an update of the registers.
    B0x1 = 1,
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
    ///No action
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UG::B0x0)
    }
    ///Re-initialize the counter and generates an update of the registers.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UG::B0x1)
    }
}
/**Capture/compare 1 generation

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
///Field `CC1G` writer - Capture/compare 1 generation
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
///Field `CC2G` writer - Capture/compare 2 generation
pub type CC2G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3G` writer - Capture/compare 3 generation
pub type CC3G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4G` writer - Capture/compare 4 generation
pub type CC4G_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Trigger generation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TG {
    ///0: No action
    B0x0 = 0,
    ///1: The TIF flag is set in TIMx_SR register.
    B0x1 = 1,
}
impl From<TG> for bool {
    #[inline(always)]
    fn from(variant: TG) -> Self {
        variant as u8 != 0
    }
}
///Field `TG` writer - Trigger generation
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
    ///The TIF flag is set in TIMx_SR register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TG::B0x1)
    }
}
impl core::fmt::Debug for crate::generic::Reg<TIM2_EGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Update generation
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W<'_, TIM2_EGRrs> {
        UG_W::new(self, 0)
    }
    ///Bit 1 - Capture/compare 1 generation
    #[inline(always)]
    pub fn cc1g(&mut self) -> CC1G_W<'_, TIM2_EGRrs> {
        CC1G_W::new(self, 1)
    }
    ///Bit 2 - Capture/compare 2 generation
    #[inline(always)]
    pub fn cc2g(&mut self) -> CC2G_W<'_, TIM2_EGRrs> {
        CC2G_W::new(self, 2)
    }
    ///Bit 3 - Capture/compare 3 generation
    #[inline(always)]
    pub fn cc3g(&mut self) -> CC3G_W<'_, TIM2_EGRrs> {
        CC3G_W::new(self, 3)
    }
    ///Bit 4 - Capture/compare 4 generation
    #[inline(always)]
    pub fn cc4g(&mut self) -> CC4G_W<'_, TIM2_EGRrs> {
        CC4G_W::new(self, 4)
    }
    ///Bit 6 - Trigger generation
    #[inline(always)]
    pub fn tg(&mut self) -> TG_W<'_, TIM2_EGRrs> {
        TG_W::new(self, 6)
    }
}
/**TIM2 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_EGR)*/
pub struct TIM2_EGRrs;
impl crate::RegisterSpec for TIM2_EGRrs {
    type Ux = u16;
}
///`write(|w| ..)` method takes [`tim2_egr::W`](W) writer structure
impl crate::Writable for TIM2_EGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM2_EGR to value 0
impl crate::Resettable for TIM2_EGRrs {}
