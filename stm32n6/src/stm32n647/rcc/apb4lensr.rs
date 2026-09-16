///Register `APB4LENSR` writer
pub type W = crate::W<APB4LENSRrs>;
///Field `HDPENS` writer - HDP enable
pub type HDPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1ENS` writer - LPUART1 enable
pub type LPUART1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI6ENS` writer - SPI6 enable
pub type SPI6ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4ENS` writer - I2C4 enable
pub type I2C4ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM2ENS` writer - LPTIM2 enable
pub type LPTIM2ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3ENS` writer - LPTIM3 enable
pub type LPTIM3ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM4ENS` writer - LPTIM4 enable
pub type LPTIM4ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM5ENS` writer - LPTIM5 enable
pub type LPTIM5ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFBUFENS` writer - VREFBUF enable
pub type VREFBUFENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCENS` writer - RTC enable
pub type RTCENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCAPBENS` writer - RTCAPB enable
pub type RTCAPBENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `R2GRETENS` writer - R2GRET enable
pub type R2GRETENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `R2GNPUENS` writer - R2GNPU enable
pub type R2GNPUENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SERFENS` writer - SERF enable
pub type SERFENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB4LENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 2 - HDP enable
    #[inline(always)]
    pub fn hdpens(&mut self) -> HDPENS_W<'_, APB4LENSRrs> {
        HDPENS_W::new(self, 2)
    }
    ///Bit 3 - LPUART1 enable
    #[inline(always)]
    pub fn lpuart1ens(&mut self) -> LPUART1ENS_W<'_, APB4LENSRrs> {
        LPUART1ENS_W::new(self, 3)
    }
    ///Bit 5 - SPI6 enable
    #[inline(always)]
    pub fn spi6ens(&mut self) -> SPI6ENS_W<'_, APB4LENSRrs> {
        SPI6ENS_W::new(self, 5)
    }
    ///Bit 7 - I2C4 enable
    #[inline(always)]
    pub fn i2c4ens(&mut self) -> I2C4ENS_W<'_, APB4LENSRrs> {
        I2C4ENS_W::new(self, 7)
    }
    ///Bit 9 - LPTIM2 enable
    #[inline(always)]
    pub fn lptim2ens(&mut self) -> LPTIM2ENS_W<'_, APB4LENSRrs> {
        LPTIM2ENS_W::new(self, 9)
    }
    ///Bit 10 - LPTIM3 enable
    #[inline(always)]
    pub fn lptim3ens(&mut self) -> LPTIM3ENS_W<'_, APB4LENSRrs> {
        LPTIM3ENS_W::new(self, 10)
    }
    ///Bit 11 - LPTIM4 enable
    #[inline(always)]
    pub fn lptim4ens(&mut self) -> LPTIM4ENS_W<'_, APB4LENSRrs> {
        LPTIM4ENS_W::new(self, 11)
    }
    ///Bit 12 - LPTIM5 enable
    #[inline(always)]
    pub fn lptim5ens(&mut self) -> LPTIM5ENS_W<'_, APB4LENSRrs> {
        LPTIM5ENS_W::new(self, 12)
    }
    ///Bit 15 - VREFBUF enable
    #[inline(always)]
    pub fn vrefbufens(&mut self) -> VREFBUFENS_W<'_, APB4LENSRrs> {
        VREFBUFENS_W::new(self, 15)
    }
    ///Bit 16 - RTC enable
    #[inline(always)]
    pub fn rtcens(&mut self) -> RTCENS_W<'_, APB4LENSRrs> {
        RTCENS_W::new(self, 16)
    }
    ///Bit 17 - RTCAPB enable
    #[inline(always)]
    pub fn rtcapbens(&mut self) -> RTCAPBENS_W<'_, APB4LENSRrs> {
        RTCAPBENS_W::new(self, 17)
    }
    ///Bit 22 - R2GRET enable
    #[inline(always)]
    pub fn r2gretens(&mut self) -> R2GRETENS_W<'_, APB4LENSRrs> {
        R2GRETENS_W::new(self, 22)
    }
    ///Bit 23 - R2GNPU enable
    #[inline(always)]
    pub fn r2gnpuens(&mut self) -> R2GNPUENS_W<'_, APB4LENSRrs> {
        R2GNPUENS_W::new(self, 23)
    }
    ///Bit 31 - SERF enable
    #[inline(always)]
    pub fn serfens(&mut self) -> SERFENS_W<'_, APB4LENSRrs> {
        SERFENS_W::new(self, 31)
    }
}
/**RCC APB4L enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4lensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB4LENSR)*/
pub struct APB4LENSRrs;
impl crate::RegisterSpec for APB4LENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb4lensr::W`](W) writer structure
impl crate::Writable for APB4LENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4LENSR to value 0
impl crate::Resettable for APB4LENSRrs {}
