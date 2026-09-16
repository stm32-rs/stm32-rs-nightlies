///Register `APB4LENCR` writer
pub type W = crate::W<APB4LENCRrs>;
///Field `HDPENC` writer - HDP enable
pub type HDPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1ENC` writer - LPUART1 enable
pub type LPUART1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI6ENC` writer - SPI6 enable
pub type SPI6ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4ENC` writer - I2C4 enable
pub type I2C4ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM2ENC` writer - LPTIM2 enable
pub type LPTIM2ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3ENC` writer - LPTIM3 enable
pub type LPTIM3ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM4ENC` writer - LPTIM4 enable
pub type LPTIM4ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM5ENC` writer - LPTIM5 enable
pub type LPTIM5ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFBUFENC` writer - VREFBUF enable
pub type VREFBUFENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCENC` writer - RTC enable
pub type RTCENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCAPBENC` writer - RTCAPB enable
pub type RTCAPBENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `R2GRETENC` writer - R2GRET enable
pub type R2GRETENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `R2GNPUENC` writer - R2GNPU enable
pub type R2GNPUENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SERFENC` writer - SERF enable
pub type SERFENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB4LENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 2 - HDP enable
    #[inline(always)]
    pub fn hdpenc(&mut self) -> HDPENC_W<'_, APB4LENCRrs> {
        HDPENC_W::new(self, 2)
    }
    ///Bit 3 - LPUART1 enable
    #[inline(always)]
    pub fn lpuart1enc(&mut self) -> LPUART1ENC_W<'_, APB4LENCRrs> {
        LPUART1ENC_W::new(self, 3)
    }
    ///Bit 5 - SPI6 enable
    #[inline(always)]
    pub fn spi6enc(&mut self) -> SPI6ENC_W<'_, APB4LENCRrs> {
        SPI6ENC_W::new(self, 5)
    }
    ///Bit 7 - I2C4 enable
    #[inline(always)]
    pub fn i2c4enc(&mut self) -> I2C4ENC_W<'_, APB4LENCRrs> {
        I2C4ENC_W::new(self, 7)
    }
    ///Bit 9 - LPTIM2 enable
    #[inline(always)]
    pub fn lptim2enc(&mut self) -> LPTIM2ENC_W<'_, APB4LENCRrs> {
        LPTIM2ENC_W::new(self, 9)
    }
    ///Bit 10 - LPTIM3 enable
    #[inline(always)]
    pub fn lptim3enc(&mut self) -> LPTIM3ENC_W<'_, APB4LENCRrs> {
        LPTIM3ENC_W::new(self, 10)
    }
    ///Bit 11 - LPTIM4 enable
    #[inline(always)]
    pub fn lptim4enc(&mut self) -> LPTIM4ENC_W<'_, APB4LENCRrs> {
        LPTIM4ENC_W::new(self, 11)
    }
    ///Bit 12 - LPTIM5 enable
    #[inline(always)]
    pub fn lptim5enc(&mut self) -> LPTIM5ENC_W<'_, APB4LENCRrs> {
        LPTIM5ENC_W::new(self, 12)
    }
    ///Bit 15 - VREFBUF enable
    #[inline(always)]
    pub fn vrefbufenc(&mut self) -> VREFBUFENC_W<'_, APB4LENCRrs> {
        VREFBUFENC_W::new(self, 15)
    }
    ///Bit 16 - RTC enable
    #[inline(always)]
    pub fn rtcenc(&mut self) -> RTCENC_W<'_, APB4LENCRrs> {
        RTCENC_W::new(self, 16)
    }
    ///Bit 17 - RTCAPB enable
    #[inline(always)]
    pub fn rtcapbenc(&mut self) -> RTCAPBENC_W<'_, APB4LENCRrs> {
        RTCAPBENC_W::new(self, 17)
    }
    ///Bit 22 - R2GRET enable
    #[inline(always)]
    pub fn r2gretenc(&mut self) -> R2GRETENC_W<'_, APB4LENCRrs> {
        R2GRETENC_W::new(self, 22)
    }
    ///Bit 23 - R2GNPU enable
    #[inline(always)]
    pub fn r2gnpuenc(&mut self) -> R2GNPUENC_W<'_, APB4LENCRrs> {
        R2GNPUENC_W::new(self, 23)
    }
    ///Bit 31 - SERF enable
    #[inline(always)]
    pub fn serfenc(&mut self) -> SERFENC_W<'_, APB4LENCRrs> {
        SERFENC_W::new(self, 31)
    }
}
/**RCC APB4L enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4lencr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:APB4LENCR)*/
pub struct APB4LENCRrs;
impl crate::RegisterSpec for APB4LENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb4lencr::W`](W) writer structure
impl crate::Writable for APB4LENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4LENCR to value 0
impl crate::Resettable for APB4LENCRrs {}
