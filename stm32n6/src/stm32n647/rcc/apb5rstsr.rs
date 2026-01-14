///Register `APB5RSTSR` writer
pub type W = crate::W<APB5RSTSRrs>;
///Field `LTDCRSTS` writer - LTDC reset
pub type LTDCRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCMIPPRSTS` writer - DCMIPP reset
pub type DCMIPPRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXTIMRSTS` writer - GFXTIM reset
pub type GFXTIMRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VENCRSTS` writer - VENC reset
pub type VENCRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSIRSTS` writer - CSI reset
pub type CSIRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB5RSTSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 1 - LTDC reset
    #[inline(always)]
    pub fn ltdcrsts(&mut self) -> LTDCRSTS_W<'_, APB5RSTSRrs> {
        LTDCRSTS_W::new(self, 1)
    }
    ///Bit 2 - DCMIPP reset
    #[inline(always)]
    pub fn dcmipprsts(&mut self) -> DCMIPPRSTS_W<'_, APB5RSTSRrs> {
        DCMIPPRSTS_W::new(self, 2)
    }
    ///Bit 4 - GFXTIM reset
    #[inline(always)]
    pub fn gfxtimrsts(&mut self) -> GFXTIMRSTS_W<'_, APB5RSTSRrs> {
        GFXTIMRSTS_W::new(self, 4)
    }
    ///Bit 5 - VENC reset
    #[inline(always)]
    pub fn vencrsts(&mut self) -> VENCRSTS_W<'_, APB5RSTSRrs> {
        VENCRSTS_W::new(self, 5)
    }
    ///Bit 6 - CSI reset
    #[inline(always)]
    pub fn csirsts(&mut self) -> CSIRSTS_W<'_, APB5RSTSRrs> {
        CSIRSTS_W::new(self, 6)
    }
}
/**RCC APB5 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5rstsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB5RSTSR)*/
pub struct APB5RSTSRrs;
impl crate::RegisterSpec for APB5RSTSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb5rstsr::W`](W) writer structure
impl crate::Writable for APB5RSTSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB5RSTSR to value 0
impl crate::Resettable for APB5RSTSRrs {}
