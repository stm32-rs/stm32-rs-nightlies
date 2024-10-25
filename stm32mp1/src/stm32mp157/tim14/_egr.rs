///Register `_EGR` writer
pub type W = crate::W<_EGRrs>;
///Field `UG` writer - UG
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1G` writer - CC1G
pub type CC1G_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<_EGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - UG
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<_EGRrs> {
        UG_W::new(self, 0)
    }
    ///Bit 1 - CC1G
    #[inline(always)]
    #[must_use]
    pub fn cc1g(&mut self) -> CC1G_W<_EGRrs> {
        CC1G_W::new(self, 1)
    }
}
/**TIM14 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM14:_EGR)*/
pub struct _EGRrs;
impl crate::RegisterSpec for _EGRrs {
    type Ux = u16;
}
///`write(|w| ..)` method takes [`_egr::W`](W) writer structure
impl crate::Writable for _EGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets _EGR to value 0
impl crate::Resettable for _EGRrs {
    const RESET_VALUE: u16 = 0;
}
