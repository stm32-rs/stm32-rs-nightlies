///Register `APB4LRSTSR` writer
pub type W = crate::W<APB4LRSTSRrs>;
///Field `HDPRSTS` writer - HDP reset
pub type HDPRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1RSTS` writer - LPUART1 reset
pub type LPUART1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI6RSTS` writer - SPI6 reset
pub type SPI6RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4RSTS` writer - I2C4 reset
pub type I2C4RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM2RSTS` writer - LPTIM2 reset
pub type LPTIM2RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3RSTS` writer - LPTIM3 reset
pub type LPTIM3RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM4RSTS` writer - LPTIM4 reset
pub type LPTIM4RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM5RSTS` writer - LPTIM5 reset
pub type LPTIM5RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFBUFRSTS` writer - VREFBUF reset
pub type VREFBUFRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCRSTS` writer - RTC reset
pub type RTCRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `R2GRETRSTS` writer - R2GRET reset
pub type R2GRETRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `R2GNPURSTS` writer - R2GNPU reset
pub type R2GNPURSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SERFRSTS` writer - SERF reset
pub type SERFRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB4LRSTSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 2 - HDP reset
    #[inline(always)]
    pub fn hdprsts(&mut self) -> HDPRSTS_W<'_, APB4LRSTSRrs> {
        HDPRSTS_W::new(self, 2)
    }
    ///Bit 3 - LPUART1 reset
    #[inline(always)]
    pub fn lpuart1rsts(&mut self) -> LPUART1RSTS_W<'_, APB4LRSTSRrs> {
        LPUART1RSTS_W::new(self, 3)
    }
    ///Bit 5 - SPI6 reset
    #[inline(always)]
    pub fn spi6rsts(&mut self) -> SPI6RSTS_W<'_, APB4LRSTSRrs> {
        SPI6RSTS_W::new(self, 5)
    }
    ///Bit 7 - I2C4 reset
    #[inline(always)]
    pub fn i2c4rsts(&mut self) -> I2C4RSTS_W<'_, APB4LRSTSRrs> {
        I2C4RSTS_W::new(self, 7)
    }
    ///Bit 9 - LPTIM2 reset
    #[inline(always)]
    pub fn lptim2rsts(&mut self) -> LPTIM2RSTS_W<'_, APB4LRSTSRrs> {
        LPTIM2RSTS_W::new(self, 9)
    }
    ///Bit 10 - LPTIM3 reset
    #[inline(always)]
    pub fn lptim3rsts(&mut self) -> LPTIM3RSTS_W<'_, APB4LRSTSRrs> {
        LPTIM3RSTS_W::new(self, 10)
    }
    ///Bit 11 - LPTIM4 reset
    #[inline(always)]
    pub fn lptim4rsts(&mut self) -> LPTIM4RSTS_W<'_, APB4LRSTSRrs> {
        LPTIM4RSTS_W::new(self, 11)
    }
    ///Bit 12 - LPTIM5 reset
    #[inline(always)]
    pub fn lptim5rsts(&mut self) -> LPTIM5RSTS_W<'_, APB4LRSTSRrs> {
        LPTIM5RSTS_W::new(self, 12)
    }
    ///Bit 15 - VREFBUF reset
    #[inline(always)]
    pub fn vrefbufrsts(&mut self) -> VREFBUFRSTS_W<'_, APB4LRSTSRrs> {
        VREFBUFRSTS_W::new(self, 15)
    }
    ///Bit 16 - RTC reset
    #[inline(always)]
    pub fn rtcrsts(&mut self) -> RTCRSTS_W<'_, APB4LRSTSRrs> {
        RTCRSTS_W::new(self, 16)
    }
    ///Bit 22 - R2GRET reset
    #[inline(always)]
    pub fn r2gretrsts(&mut self) -> R2GRETRSTS_W<'_, APB4LRSTSRrs> {
        R2GRETRSTS_W::new(self, 22)
    }
    ///Bit 23 - R2GNPU reset
    #[inline(always)]
    pub fn r2gnpursts(&mut self) -> R2GNPURSTS_W<'_, APB4LRSTSRrs> {
        R2GNPURSTS_W::new(self, 23)
    }
    ///Bit 31 - SERF reset
    #[inline(always)]
    pub fn serfrsts(&mut self) -> SERFRSTS_W<'_, APB4LRSTSRrs> {
        SERFRSTS_W::new(self, 31)
    }
}
/**RCC APB4L reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4lrstsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:APB4LRSTSR)*/
pub struct APB4LRSTSRrs;
impl crate::RegisterSpec for APB4LRSTSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb4lrstsr::W`](W) writer structure
impl crate::Writable for APB4LRSTSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4LRSTSR to value 0
impl crate::Resettable for APB4LRSTSRrs {}
