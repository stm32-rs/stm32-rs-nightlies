///Register `TIM16_EGR` writer
pub type W = crate::W<TIM16_EGRrs>;
///Field `UG` writer - Update generation
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1G` writer - Capture/Compare 1 generation
pub type CC1G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMG` writer - Capture/Compare control update generation
pub type COMG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BG` writer - Break generation
pub type BG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<TIM16_EGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Update generation
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W<TIM16_EGRrs> {
        UG_W::new(self, 0)
    }
    ///Bit 1 - Capture/Compare 1 generation
    #[inline(always)]
    pub fn cc1g(&mut self) -> CC1G_W<TIM16_EGRrs> {
        CC1G_W::new(self, 1)
    }
    ///Bit 5 - Capture/Compare control update generation
    #[inline(always)]
    pub fn comg(&mut self) -> COMG_W<TIM16_EGRrs> {
        COMG_W::new(self, 5)
    }
    ///Bit 7 - Break generation
    #[inline(always)]
    pub fn bg(&mut self) -> BG_W<TIM16_EGRrs> {
        BG_W::new(self, 7)
    }
}
/**TIM16 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#TIM16:TIM16_EGR)*/
pub struct TIM16_EGRrs;
impl crate::RegisterSpec for TIM16_EGRrs {
    type Ux = u16;
}
///`write(|w| ..)` method takes [`tim16_egr::W`](W) writer structure
impl crate::Writable for TIM16_EGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIM16_EGR to value 0
impl crate::Resettable for TIM16_EGRrs {
    const RESET_VALUE: u16 = 0;
}
