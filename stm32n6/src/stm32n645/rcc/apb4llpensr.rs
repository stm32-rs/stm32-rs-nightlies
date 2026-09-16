///Register `APB4LLPENSR` writer
pub type W = crate::W<APB4LLPENSRrs>;
///Field `HDPLPENS` writer - HDP sleep enable
pub type HDPLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1LPENS` writer - LPUART1 sleep enable
pub type LPUART1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI6LPENS` writer - SPI6 sleep enable
pub type SPI6LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4LPENS` writer - I2C4 sleep enable
pub type I2C4LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM2LPENS` writer - LPTIM2 sleep enable
pub type LPTIM2LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3LPENS` writer - LPTIM3 sleep enable
pub type LPTIM3LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM4LPENS` writer - LPTIM4 sleep enable
pub type LPTIM4LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM5LPENS` writer - LPTIM5 sleep enable
pub type LPTIM5LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFBUFLPENS` writer - VREFBUF sleep enable
pub type VREFBUFLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCLPENS` writer - RTC sleep enable
pub type RTCLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCAPBLPENS` writer - RTCAPB sleep enable
pub type RTCAPBLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `R2GRETLPENS` writer - R2GRET sleep enable
pub type R2GRETLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `R2GNPULPENS` writer - R2GNPU sleep enable
pub type R2GNPULPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SERFLPENS` writer - SERF sleep enable
pub type SERFLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB4LLPENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 2 - HDP sleep enable
    #[inline(always)]
    pub fn hdplpens(&mut self) -> HDPLPENS_W<'_, APB4LLPENSRrs> {
        HDPLPENS_W::new(self, 2)
    }
    ///Bit 3 - LPUART1 sleep enable
    #[inline(always)]
    pub fn lpuart1lpens(&mut self) -> LPUART1LPENS_W<'_, APB4LLPENSRrs> {
        LPUART1LPENS_W::new(self, 3)
    }
    ///Bit 5 - SPI6 sleep enable
    #[inline(always)]
    pub fn spi6lpens(&mut self) -> SPI6LPENS_W<'_, APB4LLPENSRrs> {
        SPI6LPENS_W::new(self, 5)
    }
    ///Bit 7 - I2C4 sleep enable
    #[inline(always)]
    pub fn i2c4lpens(&mut self) -> I2C4LPENS_W<'_, APB4LLPENSRrs> {
        I2C4LPENS_W::new(self, 7)
    }
    ///Bit 9 - LPTIM2 sleep enable
    #[inline(always)]
    pub fn lptim2lpens(&mut self) -> LPTIM2LPENS_W<'_, APB4LLPENSRrs> {
        LPTIM2LPENS_W::new(self, 9)
    }
    ///Bit 10 - LPTIM3 sleep enable
    #[inline(always)]
    pub fn lptim3lpens(&mut self) -> LPTIM3LPENS_W<'_, APB4LLPENSRrs> {
        LPTIM3LPENS_W::new(self, 10)
    }
    ///Bit 11 - LPTIM4 sleep enable
    #[inline(always)]
    pub fn lptim4lpens(&mut self) -> LPTIM4LPENS_W<'_, APB4LLPENSRrs> {
        LPTIM4LPENS_W::new(self, 11)
    }
    ///Bit 12 - LPTIM5 sleep enable
    #[inline(always)]
    pub fn lptim5lpens(&mut self) -> LPTIM5LPENS_W<'_, APB4LLPENSRrs> {
        LPTIM5LPENS_W::new(self, 12)
    }
    ///Bit 15 - VREFBUF sleep enable
    #[inline(always)]
    pub fn vrefbuflpens(&mut self) -> VREFBUFLPENS_W<'_, APB4LLPENSRrs> {
        VREFBUFLPENS_W::new(self, 15)
    }
    ///Bit 16 - RTC sleep enable
    #[inline(always)]
    pub fn rtclpens(&mut self) -> RTCLPENS_W<'_, APB4LLPENSRrs> {
        RTCLPENS_W::new(self, 16)
    }
    ///Bit 17 - RTCAPB sleep enable
    #[inline(always)]
    pub fn rtcapblpens(&mut self) -> RTCAPBLPENS_W<'_, APB4LLPENSRrs> {
        RTCAPBLPENS_W::new(self, 17)
    }
    ///Bit 22 - R2GRET sleep enable
    #[inline(always)]
    pub fn r2gretlpens(&mut self) -> R2GRETLPENS_W<'_, APB4LLPENSRrs> {
        R2GRETLPENS_W::new(self, 22)
    }
    ///Bit 23 - R2GNPU sleep enable
    #[inline(always)]
    pub fn r2gnpulpens(&mut self) -> R2GNPULPENS_W<'_, APB4LLPENSRrs> {
        R2GNPULPENS_W::new(self, 23)
    }
    ///Bit 31 - SERF sleep enable
    #[inline(always)]
    pub fn serflpens(&mut self) -> SERFLPENS_W<'_, APB4LLPENSRrs> {
        SERFLPENS_W::new(self, 31)
    }
}
/**RCC APB4L Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4llpensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:APB4LLPENSR)*/
pub struct APB4LLPENSRrs;
impl crate::RegisterSpec for APB4LLPENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb4llpensr::W`](W) writer structure
impl crate::Writable for APB4LLPENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4LLPENSR to value 0
impl crate::Resettable for APB4LLPENSRrs {}
