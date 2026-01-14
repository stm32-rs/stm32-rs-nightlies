///Register `APB4LLPENCR` writer
pub type W = crate::W<APB4LLPENCRrs>;
///Field `HDPLPENC` writer - HDP sleep enable
pub type HDPLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1LPENC` writer - LPUART1 sleep enable
pub type LPUART1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI6LPENC` writer - SPI6 sleep enable
pub type SPI6LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4LPENC` writer - I2C4 sleep enable
pub type I2C4LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM2LPENC` writer - LPTIM2 sleep enable
pub type LPTIM2LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3LPENC` writer - LPTIM3 sleep enable
pub type LPTIM3LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM4LPENC` writer - LPTIM4 sleep enable
pub type LPTIM4LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM5LPENC` writer - LPTIM5 sleep enable
pub type LPTIM5LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFBUFLPENC` writer - VREFBUF sleep enable
pub type VREFBUFLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCLPENC` writer - RTC sleep enable
pub type RTCLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCAPBLPENC` writer - RTCAPB sleep enable
pub type RTCAPBLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `R2GRETLPENC` writer - R2GRET sleep enable
pub type R2GRETLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `R2GNPULPENC` writer - R2GNPU sleep enable
pub type R2GNPULPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SERFLPENC` writer - SERF sleep enable
pub type SERFLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB4LLPENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 2 - HDP sleep enable
    #[inline(always)]
    pub fn hdplpenc(&mut self) -> HDPLPENC_W<'_, APB4LLPENCRrs> {
        HDPLPENC_W::new(self, 2)
    }
    ///Bit 3 - LPUART1 sleep enable
    #[inline(always)]
    pub fn lpuart1lpenc(&mut self) -> LPUART1LPENC_W<'_, APB4LLPENCRrs> {
        LPUART1LPENC_W::new(self, 3)
    }
    ///Bit 5 - SPI6 sleep enable
    #[inline(always)]
    pub fn spi6lpenc(&mut self) -> SPI6LPENC_W<'_, APB4LLPENCRrs> {
        SPI6LPENC_W::new(self, 5)
    }
    ///Bit 7 - I2C4 sleep enable
    #[inline(always)]
    pub fn i2c4lpenc(&mut self) -> I2C4LPENC_W<'_, APB4LLPENCRrs> {
        I2C4LPENC_W::new(self, 7)
    }
    ///Bit 9 - LPTIM2 sleep enable
    #[inline(always)]
    pub fn lptim2lpenc(&mut self) -> LPTIM2LPENC_W<'_, APB4LLPENCRrs> {
        LPTIM2LPENC_W::new(self, 9)
    }
    ///Bit 10 - LPTIM3 sleep enable
    #[inline(always)]
    pub fn lptim3lpenc(&mut self) -> LPTIM3LPENC_W<'_, APB4LLPENCRrs> {
        LPTIM3LPENC_W::new(self, 10)
    }
    ///Bit 11 - LPTIM4 sleep enable
    #[inline(always)]
    pub fn lptim4lpenc(&mut self) -> LPTIM4LPENC_W<'_, APB4LLPENCRrs> {
        LPTIM4LPENC_W::new(self, 11)
    }
    ///Bit 12 - LPTIM5 sleep enable
    #[inline(always)]
    pub fn lptim5lpenc(&mut self) -> LPTIM5LPENC_W<'_, APB4LLPENCRrs> {
        LPTIM5LPENC_W::new(self, 12)
    }
    ///Bit 15 - VREFBUF sleep enable
    #[inline(always)]
    pub fn vrefbuflpenc(&mut self) -> VREFBUFLPENC_W<'_, APB4LLPENCRrs> {
        VREFBUFLPENC_W::new(self, 15)
    }
    ///Bit 16 - RTC sleep enable
    #[inline(always)]
    pub fn rtclpenc(&mut self) -> RTCLPENC_W<'_, APB4LLPENCRrs> {
        RTCLPENC_W::new(self, 16)
    }
    ///Bit 17 - RTCAPB sleep enable
    #[inline(always)]
    pub fn rtcapblpenc(&mut self) -> RTCAPBLPENC_W<'_, APB4LLPENCRrs> {
        RTCAPBLPENC_W::new(self, 17)
    }
    ///Bit 22 - R2GRET sleep enable
    #[inline(always)]
    pub fn r2gretlpenc(&mut self) -> R2GRETLPENC_W<'_, APB4LLPENCRrs> {
        R2GRETLPENC_W::new(self, 22)
    }
    ///Bit 23 - R2GNPU sleep enable
    #[inline(always)]
    pub fn r2gnpulpenc(&mut self) -> R2GNPULPENC_W<'_, APB4LLPENCRrs> {
        R2GNPULPENC_W::new(self, 23)
    }
    ///Bit 31 - SERF sleep enable
    #[inline(always)]
    pub fn serflpenc(&mut self) -> SERFLPENC_W<'_, APB4LLPENCRrs> {
        SERFLPENC_W::new(self, 31)
    }
}
/**RCC APB4L Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4llpencr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:APB4LLPENCR)*/
pub struct APB4LLPENCRrs;
impl crate::RegisterSpec for APB4LLPENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb4llpencr::W`](W) writer structure
impl crate::Writable for APB4LLPENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4LLPENCR to value 0
impl crate::Resettable for APB4LLPENCRrs {}
