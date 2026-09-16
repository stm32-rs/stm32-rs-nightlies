///Register `APB4LRSTCR` writer
pub type W = crate::W<APB4LRSTCRrs>;
///Field `HDPRSTC` writer - HDP reset
pub type HDPRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1RSTC` writer - LPUART1 reset
pub type LPUART1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI6RSTC` writer - SPI6 reset
pub type SPI6RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4RSTC` writer - I2C4 reset
pub type I2C4RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM2RSTC` writer - LPTIM2 reset
pub type LPTIM2RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3RSTC` writer - LPTIM3 reset
pub type LPTIM3RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM4RSTC` writer - LPTIM4 reset
pub type LPTIM4RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM5RSTC` writer - LPTIM5 reset
pub type LPTIM5RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFBUFRSTC` writer - VREFBUF reset
pub type VREFBUFRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCRSTC` writer - RTC reset
pub type RTCRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `R2GRETRSTC` writer - R2GRET reset
pub type R2GRETRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `R2GNPURSTC` writer - R2GNPU reset
pub type R2GNPURSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SERFRSTC` writer - SERF reset
pub type SERFRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB4LRSTCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 2 - HDP reset
    #[inline(always)]
    pub fn hdprstc(&mut self) -> HDPRSTC_W<'_, APB4LRSTCRrs> {
        HDPRSTC_W::new(self, 2)
    }
    ///Bit 3 - LPUART1 reset
    #[inline(always)]
    pub fn lpuart1rstc(&mut self) -> LPUART1RSTC_W<'_, APB4LRSTCRrs> {
        LPUART1RSTC_W::new(self, 3)
    }
    ///Bit 5 - SPI6 reset
    #[inline(always)]
    pub fn spi6rstc(&mut self) -> SPI6RSTC_W<'_, APB4LRSTCRrs> {
        SPI6RSTC_W::new(self, 5)
    }
    ///Bit 7 - I2C4 reset
    #[inline(always)]
    pub fn i2c4rstc(&mut self) -> I2C4RSTC_W<'_, APB4LRSTCRrs> {
        I2C4RSTC_W::new(self, 7)
    }
    ///Bit 9 - LPTIM2 reset
    #[inline(always)]
    pub fn lptim2rstc(&mut self) -> LPTIM2RSTC_W<'_, APB4LRSTCRrs> {
        LPTIM2RSTC_W::new(self, 9)
    }
    ///Bit 10 - LPTIM3 reset
    #[inline(always)]
    pub fn lptim3rstc(&mut self) -> LPTIM3RSTC_W<'_, APB4LRSTCRrs> {
        LPTIM3RSTC_W::new(self, 10)
    }
    ///Bit 11 - LPTIM4 reset
    #[inline(always)]
    pub fn lptim4rstc(&mut self) -> LPTIM4RSTC_W<'_, APB4LRSTCRrs> {
        LPTIM4RSTC_W::new(self, 11)
    }
    ///Bit 12 - LPTIM5 reset
    #[inline(always)]
    pub fn lptim5rstc(&mut self) -> LPTIM5RSTC_W<'_, APB4LRSTCRrs> {
        LPTIM5RSTC_W::new(self, 12)
    }
    ///Bit 15 - VREFBUF reset
    #[inline(always)]
    pub fn vrefbufrstc(&mut self) -> VREFBUFRSTC_W<'_, APB4LRSTCRrs> {
        VREFBUFRSTC_W::new(self, 15)
    }
    ///Bit 16 - RTC reset
    #[inline(always)]
    pub fn rtcrstc(&mut self) -> RTCRSTC_W<'_, APB4LRSTCRrs> {
        RTCRSTC_W::new(self, 16)
    }
    ///Bit 22 - R2GRET reset
    #[inline(always)]
    pub fn r2gretrstc(&mut self) -> R2GRETRSTC_W<'_, APB4LRSTCRrs> {
        R2GRETRSTC_W::new(self, 22)
    }
    ///Bit 23 - R2GNPU reset
    #[inline(always)]
    pub fn r2gnpurstc(&mut self) -> R2GNPURSTC_W<'_, APB4LRSTCRrs> {
        R2GNPURSTC_W::new(self, 23)
    }
    ///Bit 31 - SERF reset
    #[inline(always)]
    pub fn serfrstc(&mut self) -> SERFRSTC_W<'_, APB4LRSTCRrs> {
        SERFRSTC_W::new(self, 31)
    }
}
/**RCC APB4L reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4lrstcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:APB4LRSTCR)*/
pub struct APB4LRSTCRrs;
impl crate::RegisterSpec for APB4LRSTCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb4lrstcr::W`](W) writer structure
impl crate::Writable for APB4LRSTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4LRSTCR to value 0
impl crate::Resettable for APB4LRSTCRrs {}
