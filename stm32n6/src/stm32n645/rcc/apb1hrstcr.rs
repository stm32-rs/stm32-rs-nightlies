///Register `APB1HRSTCR` writer
pub type W = crate::W<APB1HRSTCRrs>;
///Field `MDIOSRSTC` writer - MDIOS reset
pub type MDIOSRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDCANRSTC` writer - FDCAN reset
pub type FDCANRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCPD1RSTC` writer - UCPD1 reset
pub type UCPD1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB1HRSTCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 5 - MDIOS reset
    #[inline(always)]
    pub fn mdiosrstc(&mut self) -> MDIOSRSTC_W<'_, APB1HRSTCRrs> {
        MDIOSRSTC_W::new(self, 5)
    }
    ///Bit 8 - FDCAN reset
    #[inline(always)]
    pub fn fdcanrstc(&mut self) -> FDCANRSTC_W<'_, APB1HRSTCRrs> {
        FDCANRSTC_W::new(self, 8)
    }
    ///Bit 18 - UCPD1 reset
    #[inline(always)]
    pub fn ucpd1rstc(&mut self) -> UCPD1RSTC_W<'_, APB1HRSTCRrs> {
        UCPD1RSTC_W::new(self, 18)
    }
}
/**RCC APB1H reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hrstcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:APB1HRSTCR)*/
pub struct APB1HRSTCRrs;
impl crate::RegisterSpec for APB1HRSTCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb1hrstcr::W`](W) writer structure
impl crate::Writable for APB1HRSTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1HRSTCR to value 0
impl crate::Resettable for APB1HRSTCRrs {}
