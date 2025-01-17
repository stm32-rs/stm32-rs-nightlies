///Register `TIM2_EGR` writer
pub type W = crate::W<TIM2_EGRrs>;
///Field `UG` writer - Update generation
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1G` writer - Capture/compare 1 generation
pub type CC1G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2G` writer - Capture/compare 2 generation
pub type CC2G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3G` writer - Capture/compare 3 generation
pub type CC3G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4G` writer - Capture/compare 4 generation
pub type CC4G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TG` writer - Trigger generation
pub type TG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<TIM2_EGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Update generation
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W<TIM2_EGRrs> {
        UG_W::new(self, 0)
    }
    ///Bit 1 - Capture/compare 1 generation
    #[inline(always)]
    pub fn cc1g(&mut self) -> CC1G_W<TIM2_EGRrs> {
        CC1G_W::new(self, 1)
    }
    ///Bit 2 - Capture/compare 2 generation
    #[inline(always)]
    pub fn cc2g(&mut self) -> CC2G_W<TIM2_EGRrs> {
        CC2G_W::new(self, 2)
    }
    ///Bit 3 - Capture/compare 3 generation
    #[inline(always)]
    pub fn cc3g(&mut self) -> CC3G_W<TIM2_EGRrs> {
        CC3G_W::new(self, 3)
    }
    ///Bit 4 - Capture/compare 4 generation
    #[inline(always)]
    pub fn cc4g(&mut self) -> CC4G_W<TIM2_EGRrs> {
        CC4G_W::new(self, 4)
    }
    ///Bit 6 - Trigger generation
    #[inline(always)]
    pub fn tg(&mut self) -> TG_W<TIM2_EGRrs> {
        TG_W::new(self, 6)
    }
}
/**TIM2 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#TIM2:TIM2_EGR)*/
pub struct TIM2_EGRrs;
impl crate::RegisterSpec for TIM2_EGRrs {
    type Ux = u16;
}
///`write(|w| ..)` method takes [`tim2_egr::W`](W) writer structure
impl crate::Writable for TIM2_EGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIM2_EGR to value 0
impl crate::Resettable for TIM2_EGRrs {
    const RESET_VALUE: u16 = 0;
}
