///Register `APB1HRSTSR` writer
pub type W = crate::W<APB1HRSTSRrs>;
///Field `MDIOSRSTS` writer - MDIOS reset
pub type MDIOSRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDCANRSTS` writer - FDCAN reset
pub type FDCANRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCPD1RSTS` writer - UCPD1 reset
pub type UCPD1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB1HRSTSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 5 - MDIOS reset
    #[inline(always)]
    pub fn mdiosrsts(&mut self) -> MDIOSRSTS_W<'_, APB1HRSTSRrs> {
        MDIOSRSTS_W::new(self, 5)
    }
    ///Bit 8 - FDCAN reset
    #[inline(always)]
    pub fn fdcanrsts(&mut self) -> FDCANRSTS_W<'_, APB1HRSTSRrs> {
        FDCANRSTS_W::new(self, 8)
    }
    ///Bit 18 - UCPD1 reset
    #[inline(always)]
    pub fn ucpd1rsts(&mut self) -> UCPD1RSTS_W<'_, APB1HRSTSRrs> {
        UCPD1RSTS_W::new(self, 18)
    }
}
/**RCC APB1H reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hrstsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:APB1HRSTSR)*/
pub struct APB1HRSTSRrs;
impl crate::RegisterSpec for APB1HRSTSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb1hrstsr::W`](W) writer structure
impl crate::Writable for APB1HRSTSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1HRSTSR to value 0
impl crate::Resettable for APB1HRSTSRrs {}
