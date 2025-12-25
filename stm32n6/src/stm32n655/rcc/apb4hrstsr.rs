///Register `APB4HRSTSR` writer
pub type W = crate::W<APB4HRSTSRrs>;
///Field `SYSCFGRSTS` writer - SYSCFG reset
pub type SYSCFGRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTSRSTS` writer - DTS reset
pub type DTSRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSPERFMRSTS` writer - BUSPERFM reset
pub type BUSPERFMRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB4HRSTSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - SYSCFG reset
    #[inline(always)]
    pub fn syscfgrsts(&mut self) -> SYSCFGRSTS_W<'_, APB4HRSTSRrs> {
        SYSCFGRSTS_W::new(self, 0)
    }
    ///Bit 2 - DTS reset
    #[inline(always)]
    pub fn dtsrsts(&mut self) -> DTSRSTS_W<'_, APB4HRSTSRrs> {
        DTSRSTS_W::new(self, 2)
    }
    ///Bit 4 - BUSPERFM reset
    #[inline(always)]
    pub fn busperfmrsts(&mut self) -> BUSPERFMRSTS_W<'_, APB4HRSTSRrs> {
        BUSPERFMRSTS_W::new(self, 4)
    }
}
/**RCC APB4H reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4hrstsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:APB4HRSTSR)*/
pub struct APB4HRSTSRrs;
impl crate::RegisterSpec for APB4HRSTSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb4hrstsr::W`](W) writer structure
impl crate::Writable for APB4HRSTSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4HRSTSR to value 0
impl crate::Resettable for APB4HRSTSRrs {}
