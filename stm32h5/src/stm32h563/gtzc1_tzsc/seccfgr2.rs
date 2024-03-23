#[doc = "Register `SECCFGR2` reader"]
pub type R = crate::R<SECCFGR2rs>;
#[doc = "Register `SECCFGR2` writer"]
pub type W = crate::W<SECCFGR2rs>;
#[doc = "Field `FDCAN1SEC` reader - secure access mode for FDCAN1"]
pub type FDCAN1SEC_R = crate::BitReader;
#[doc = "Field `FDCAN1SEC` writer - secure access mode for FDCAN1"]
pub type FDCAN1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDCAN2SEC` reader - secure access mode for FDCAN2"]
pub type FDCAN2SEC_R = crate::BitReader;
#[doc = "Field `FDCAN2SEC` writer - secure access mode for FDCAN2"]
pub type FDCAN2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPDSEC` reader - secure access mode for UCPD"]
pub type UCPDSEC_R = crate::BitReader;
#[doc = "Field `UCPDSEC` writer - secure access mode for UCPD"]
pub type UCPDSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1SEC` reader - secure access mode for TIM1"]
pub type TIM1SEC_R = crate::BitReader;
#[doc = "Field `TIM1SEC` writer - secure access mode for TIM1"]
pub type TIM1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1SEC` reader - secure access mode for SPI1"]
pub type SPI1SEC_R = crate::BitReader;
#[doc = "Field `SPI1SEC` writer - secure access mode for SPI1"]
pub type SPI1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8SEC` reader - secure access mode for TIM8"]
pub type TIM8SEC_R = crate::BitReader;
#[doc = "Field `TIM8SEC` writer - secure access mode for TIM8"]
pub type TIM8SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1SEC` reader - secure access mode for USART1"]
pub type USART1SEC_R = crate::BitReader;
#[doc = "Field `USART1SEC` writer - secure access mode for USART1"]
pub type USART1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15SEC` reader - secure access mode for TIM15"]
pub type TIM15SEC_R = crate::BitReader;
#[doc = "Field `TIM15SEC` writer - secure access mode for TIM15"]
pub type TIM15SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16SEC` reader - secure access mode for TIM16"]
pub type TIM16SEC_R = crate::BitReader;
#[doc = "Field `TIM16SEC` writer - secure access mode for TIM16"]
pub type TIM16SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17SEC` reader - secure access mode for TIM17"]
pub type TIM17SEC_R = crate::BitReader;
#[doc = "Field `TIM17SEC` writer - secure access mode for TIM17"]
pub type TIM17SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI4SEC` reader - secure access mode for SPI4"]
pub type SPI4SEC_R = crate::BitReader;
#[doc = "Field `SPI4SEC` writer - secure access mode for SPI4"]
pub type SPI4SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI6SEC` reader - secure access mode for SPI6"]
pub type SPI6SEC_R = crate::BitReader;
#[doc = "Field `SPI6SEC` writer - secure access mode for SPI6"]
pub type SPI6SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI1SEC` reader - secure access mode for SAI1"]
pub type SAI1SEC_R = crate::BitReader;
#[doc = "Field `SAI1SEC` writer - secure access mode for SAI1"]
pub type SAI1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI2SEC` reader - secure access mode for SAI2"]
pub type SAI2SEC_R = crate::BitReader;
#[doc = "Field `SAI2SEC` writer - secure access mode for SAI2"]
pub type SAI2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSEC` reader - secure access mode for USB"]
pub type USBSEC_R = crate::BitReader;
#[doc = "Field `USBSEC` writer - secure access mode for USB"]
pub type USBSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI5SEC` reader - secure access mode for SPI5"]
pub type SPI5SEC_R = crate::BitReader;
#[doc = "Field `SPI5SEC` writer - secure access mode for SPI5"]
pub type SPI5SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUART1SEC` reader - secure access mode for LPUART"]
pub type LPUART1SEC_R = crate::BitReader;
#[doc = "Field `LPUART1SEC` writer - secure access mode for LPUART"]
pub type LPUART1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3SEC` reader - secure access mode for I2C3"]
pub type I2C3SEC_R = crate::BitReader;
#[doc = "Field `I2C3SEC` writer - secure access mode for I2C3"]
pub type I2C3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4SEC` reader - secure access mode for I2C4"]
pub type I2C4SEC_R = crate::BitReader;
#[doc = "Field `I2C4SEC` writer - secure access mode for I2C4"]
pub type I2C4SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1SEC` reader - secure access mode for LPTIM1"]
pub type LPTIM1SEC_R = crate::BitReader;
#[doc = "Field `LPTIM1SEC` writer - secure access mode for LPTIM1"]
pub type LPTIM1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM3SEC` reader - secure access mode for LPTIM3"]
pub type LPTIM3SEC_R = crate::BitReader;
#[doc = "Field `LPTIM3SEC` writer - secure access mode for LPTIM3"]
pub type LPTIM3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM4SEC` reader - secure access mode for LPTIM4"]
pub type LPTIM4SEC_R = crate::BitReader;
#[doc = "Field `LPTIM4SEC` writer - secure access mode for LPTIM4"]
pub type LPTIM4SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM5SEC` reader - secure access mode for LPTIM5"]
pub type LPTIM5SEC_R = crate::BitReader;
#[doc = "Field `LPTIM5SEC` writer - secure access mode for LPTIM5"]
pub type LPTIM5SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - secure access mode for FDCAN1"]
    #[inline(always)]
    pub fn fdcan1sec(&self) -> FDCAN1SEC_R {
        FDCAN1SEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - secure access mode for FDCAN2"]
    #[inline(always)]
    pub fn fdcan2sec(&self) -> FDCAN2SEC_R {
        FDCAN2SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - secure access mode for UCPD"]
    #[inline(always)]
    pub fn ucpdsec(&self) -> UCPDSEC_R {
        UCPDSEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - secure access mode for TIM1"]
    #[inline(always)]
    pub fn tim1sec(&self) -> TIM1SEC_R {
        TIM1SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - secure access mode for SPI1"]
    #[inline(always)]
    pub fn spi1sec(&self) -> SPI1SEC_R {
        SPI1SEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - secure access mode for TIM8"]
    #[inline(always)]
    pub fn tim8sec(&self) -> TIM8SEC_R {
        TIM8SEC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - secure access mode for USART1"]
    #[inline(always)]
    pub fn usart1sec(&self) -> USART1SEC_R {
        USART1SEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - secure access mode for TIM15"]
    #[inline(always)]
    pub fn tim15sec(&self) -> TIM15SEC_R {
        TIM15SEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - secure access mode for TIM16"]
    #[inline(always)]
    pub fn tim16sec(&self) -> TIM16SEC_R {
        TIM16SEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - secure access mode for TIM17"]
    #[inline(always)]
    pub fn tim17sec(&self) -> TIM17SEC_R {
        TIM17SEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - secure access mode for SPI4"]
    #[inline(always)]
    pub fn spi4sec(&self) -> SPI4SEC_R {
        SPI4SEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - secure access mode for SPI6"]
    #[inline(always)]
    pub fn spi6sec(&self) -> SPI6SEC_R {
        SPI6SEC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - secure access mode for SAI1"]
    #[inline(always)]
    pub fn sai1sec(&self) -> SAI1SEC_R {
        SAI1SEC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - secure access mode for SAI2"]
    #[inline(always)]
    pub fn sai2sec(&self) -> SAI2SEC_R {
        SAI2SEC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - secure access mode for USB"]
    #[inline(always)]
    pub fn usbsec(&self) -> USBSEC_R {
        USBSEC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - secure access mode for SPI5"]
    #[inline(always)]
    pub fn spi5sec(&self) -> SPI5SEC_R {
        SPI5SEC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - secure access mode for LPUART"]
    #[inline(always)]
    pub fn lpuart1sec(&self) -> LPUART1SEC_R {
        LPUART1SEC_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - secure access mode for I2C3"]
    #[inline(always)]
    pub fn i2c3sec(&self) -> I2C3SEC_R {
        I2C3SEC_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - secure access mode for I2C4"]
    #[inline(always)]
    pub fn i2c4sec(&self) -> I2C4SEC_R {
        I2C4SEC_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - secure access mode for LPTIM1"]
    #[inline(always)]
    pub fn lptim1sec(&self) -> LPTIM1SEC_R {
        LPTIM1SEC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - secure access mode for LPTIM3"]
    #[inline(always)]
    pub fn lptim3sec(&self) -> LPTIM3SEC_R {
        LPTIM3SEC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - secure access mode for LPTIM4"]
    #[inline(always)]
    pub fn lptim4sec(&self) -> LPTIM4SEC_R {
        LPTIM4SEC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - secure access mode for LPTIM5"]
    #[inline(always)]
    pub fn lptim5sec(&self) -> LPTIM5SEC_R {
        LPTIM5SEC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - secure access mode for FDCAN1"]
    #[inline(always)]
    #[must_use]
    pub fn fdcan1sec(&mut self) -> FDCAN1SEC_W<SECCFGR2rs> {
        FDCAN1SEC_W::new(self, 0)
    }
    #[doc = "Bit 1 - secure access mode for FDCAN2"]
    #[inline(always)]
    #[must_use]
    pub fn fdcan2sec(&mut self) -> FDCAN2SEC_W<SECCFGR2rs> {
        FDCAN2SEC_W::new(self, 1)
    }
    #[doc = "Bit 2 - secure access mode for UCPD"]
    #[inline(always)]
    #[must_use]
    pub fn ucpdsec(&mut self) -> UCPDSEC_W<SECCFGR2rs> {
        UCPDSEC_W::new(self, 2)
    }
    #[doc = "Bit 8 - secure access mode for TIM1"]
    #[inline(always)]
    #[must_use]
    pub fn tim1sec(&mut self) -> TIM1SEC_W<SECCFGR2rs> {
        TIM1SEC_W::new(self, 8)
    }
    #[doc = "Bit 9 - secure access mode for SPI1"]
    #[inline(always)]
    #[must_use]
    pub fn spi1sec(&mut self) -> SPI1SEC_W<SECCFGR2rs> {
        SPI1SEC_W::new(self, 9)
    }
    #[doc = "Bit 10 - secure access mode for TIM8"]
    #[inline(always)]
    #[must_use]
    pub fn tim8sec(&mut self) -> TIM8SEC_W<SECCFGR2rs> {
        TIM8SEC_W::new(self, 10)
    }
    #[doc = "Bit 11 - secure access mode for USART1"]
    #[inline(always)]
    #[must_use]
    pub fn usart1sec(&mut self) -> USART1SEC_W<SECCFGR2rs> {
        USART1SEC_W::new(self, 11)
    }
    #[doc = "Bit 12 - secure access mode for TIM15"]
    #[inline(always)]
    #[must_use]
    pub fn tim15sec(&mut self) -> TIM15SEC_W<SECCFGR2rs> {
        TIM15SEC_W::new(self, 12)
    }
    #[doc = "Bit 13 - secure access mode for TIM16"]
    #[inline(always)]
    #[must_use]
    pub fn tim16sec(&mut self) -> TIM16SEC_W<SECCFGR2rs> {
        TIM16SEC_W::new(self, 13)
    }
    #[doc = "Bit 14 - secure access mode for TIM17"]
    #[inline(always)]
    #[must_use]
    pub fn tim17sec(&mut self) -> TIM17SEC_W<SECCFGR2rs> {
        TIM17SEC_W::new(self, 14)
    }
    #[doc = "Bit 15 - secure access mode for SPI4"]
    #[inline(always)]
    #[must_use]
    pub fn spi4sec(&mut self) -> SPI4SEC_W<SECCFGR2rs> {
        SPI4SEC_W::new(self, 15)
    }
    #[doc = "Bit 16 - secure access mode for SPI6"]
    #[inline(always)]
    #[must_use]
    pub fn spi6sec(&mut self) -> SPI6SEC_W<SECCFGR2rs> {
        SPI6SEC_W::new(self, 16)
    }
    #[doc = "Bit 17 - secure access mode for SAI1"]
    #[inline(always)]
    #[must_use]
    pub fn sai1sec(&mut self) -> SAI1SEC_W<SECCFGR2rs> {
        SAI1SEC_W::new(self, 17)
    }
    #[doc = "Bit 18 - secure access mode for SAI2"]
    #[inline(always)]
    #[must_use]
    pub fn sai2sec(&mut self) -> SAI2SEC_W<SECCFGR2rs> {
        SAI2SEC_W::new(self, 18)
    }
    #[doc = "Bit 19 - secure access mode for USB"]
    #[inline(always)]
    #[must_use]
    pub fn usbsec(&mut self) -> USBSEC_W<SECCFGR2rs> {
        USBSEC_W::new(self, 19)
    }
    #[doc = "Bit 24 - secure access mode for SPI5"]
    #[inline(always)]
    #[must_use]
    pub fn spi5sec(&mut self) -> SPI5SEC_W<SECCFGR2rs> {
        SPI5SEC_W::new(self, 24)
    }
    #[doc = "Bit 25 - secure access mode for LPUART"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sec(&mut self) -> LPUART1SEC_W<SECCFGR2rs> {
        LPUART1SEC_W::new(self, 25)
    }
    #[doc = "Bit 26 - secure access mode for I2C3"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3sec(&mut self) -> I2C3SEC_W<SECCFGR2rs> {
        I2C3SEC_W::new(self, 26)
    }
    #[doc = "Bit 27 - secure access mode for I2C4"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4sec(&mut self) -> I2C4SEC_W<SECCFGR2rs> {
        I2C4SEC_W::new(self, 27)
    }
    #[doc = "Bit 28 - secure access mode for LPTIM1"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1sec(&mut self) -> LPTIM1SEC_W<SECCFGR2rs> {
        LPTIM1SEC_W::new(self, 28)
    }
    #[doc = "Bit 29 - secure access mode for LPTIM3"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3sec(&mut self) -> LPTIM3SEC_W<SECCFGR2rs> {
        LPTIM3SEC_W::new(self, 29)
    }
    #[doc = "Bit 30 - secure access mode for LPTIM4"]
    #[inline(always)]
    #[must_use]
    pub fn lptim4sec(&mut self) -> LPTIM4SEC_W<SECCFGR2rs> {
        LPTIM4SEC_W::new(self, 30)
    }
    #[doc = "Bit 31 - secure access mode for LPTIM5"]
    #[inline(always)]
    #[must_use]
    pub fn lptim5sec(&mut self) -> LPTIM5SEC_W<SECCFGR2rs> {
        LPTIM5SEC_W::new(self, 31)
    }
}
#[doc = "GTZC1 TZSC secure configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECCFGR2rs;
impl crate::RegisterSpec for SECCFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seccfgr2::R`](R) reader structure"]
impl crate::Readable for SECCFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`seccfgr2::W`](W) writer structure"]
impl crate::Writable for SECCFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECCFGR2 to value 0"]
impl crate::Resettable for SECCFGR2rs {
    const RESET_VALUE: u32 = 0;
}
