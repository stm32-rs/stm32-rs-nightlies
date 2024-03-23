#[doc = "Register `GTZC1_TZIC_IER2` reader"]
pub type R = crate::R<GTZC1_TZIC_IER2rs>;
#[doc = "Register `GTZC1_TZIC_IER2` writer"]
pub type W = crate::W<GTZC1_TZIC_IER2rs>;
#[doc = "Field `FDCAN1IE` reader - illegal access interrupt enable for FDCAN1"]
pub type FDCAN1IE_R = crate::BitReader;
#[doc = "Field `FDCAN1IE` writer - illegal access interrupt enable for FDCAN1"]
pub type FDCAN1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDCAN2IE` reader - illegal access interrupt enable for FDCAN2"]
pub type FDCAN2IE_R = crate::BitReader;
#[doc = "Field `FDCAN2IE` writer - illegal access interrupt enable for FDCAN2"]
pub type FDCAN2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPDIE` reader - illegal access interrupt enable for UCPD"]
pub type UCPDIE_R = crate::BitReader;
#[doc = "Field `UCPDIE` writer - illegal access interrupt enable for UCPD"]
pub type UCPDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1IE` reader - illegal access interrupt enable for TIM1"]
pub type TIM1IE_R = crate::BitReader;
#[doc = "Field `TIM1IE` writer - illegal access interrupt enable for TIM1"]
pub type TIM1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1IE` reader - illegal access interrupt enable for SPI1"]
pub type SPI1IE_R = crate::BitReader;
#[doc = "Field `SPI1IE` writer - illegal access interrupt enable for SPI1"]
pub type SPI1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8IE` reader - illegal access interrupt enable for TIM8"]
pub type TIM8IE_R = crate::BitReader;
#[doc = "Field `TIM8IE` writer - illegal access interrupt enable for TIM8"]
pub type TIM8IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1IE` reader - illegal access interrupt enable for USART1"]
pub type USART1IE_R = crate::BitReader;
#[doc = "Field `USART1IE` writer - illegal access interrupt enable for USART1"]
pub type USART1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15IE` reader - illegal access interrupt enable for TIM15"]
pub type TIM15IE_R = crate::BitReader;
#[doc = "Field `TIM15IE` writer - illegal access interrupt enable for TIM15"]
pub type TIM15IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16IE` reader - illegal access interrupt enable for TIM16"]
pub type TIM16IE_R = crate::BitReader;
#[doc = "Field `TIM16IE` writer - illegal access interrupt enable for TIM16"]
pub type TIM16IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17IE` reader - illegal access interrupt enable for TIM17"]
pub type TIM17IE_R = crate::BitReader;
#[doc = "Field `TIM17IE` writer - illegal access interrupt enable for TIM17"]
pub type TIM17IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI4IE` reader - illegal access interrupt enable for SPI4"]
pub type SPI4IE_R = crate::BitReader;
#[doc = "Field `SPI4IE` writer - illegal access interrupt enable for SPI4"]
pub type SPI4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI6IE` reader - illegal access interrupt enable for SPI6"]
pub type SPI6IE_R = crate::BitReader;
#[doc = "Field `SPI6IE` writer - illegal access interrupt enable for SPI6"]
pub type SPI6IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI1IE` reader - illegal access interrupt enable for SAI1"]
pub type SAI1IE_R = crate::BitReader;
#[doc = "Field `SAI1IE` writer - illegal access interrupt enable for SAI1"]
pub type SAI1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI2IE` reader - illegal access interrupt enable for SAI2"]
pub type SAI2IE_R = crate::BitReader;
#[doc = "Field `SAI2IE` writer - illegal access interrupt enable for SAI2"]
pub type SAI2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBIE` reader - illegal access interrupt enable for USB"]
pub type USBIE_R = crate::BitReader;
#[doc = "Field `USBIE` writer - illegal access interrupt enable for USB"]
pub type USBIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI5IE` reader - illegal access interrupt enable for SPI5"]
pub type SPI5IE_R = crate::BitReader;
#[doc = "Field `SPI5IE` writer - illegal access interrupt enable for SPI5"]
pub type SPI5IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUART1IE` reader - illegal access interrupt enable for LPUART"]
pub type LPUART1IE_R = crate::BitReader;
#[doc = "Field `LPUART1IE` writer - illegal access interrupt enable for LPUART"]
pub type LPUART1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3IE` reader - illegal access interrupt enable for I2C3"]
pub type I2C3IE_R = crate::BitReader;
#[doc = "Field `I2C3IE` writer - illegal access interrupt enable for I2C3"]
pub type I2C3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4IE` reader - illegal access interrupt enable for I2C4"]
pub type I2C4IE_R = crate::BitReader;
#[doc = "Field `I2C4IE` writer - illegal access interrupt enable for I2C4"]
pub type I2C4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1IE` reader - illegal access interrupt enable for LPTIM1"]
pub type LPTIM1IE_R = crate::BitReader;
#[doc = "Field `LPTIM1IE` writer - illegal access interrupt enable for LPTIM1"]
pub type LPTIM1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM3IE` reader - illegal access interrupt enable for LPTIM3"]
pub type LPTIM3IE_R = crate::BitReader;
#[doc = "Field `LPTIM3IE` writer - illegal access interrupt enable for LPTIM3"]
pub type LPTIM3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM4IE` reader - illegal access interrupt enable for LPTIM4"]
pub type LPTIM4IE_R = crate::BitReader;
#[doc = "Field `LPTIM4IE` writer - illegal access interrupt enable for LPTIM4"]
pub type LPTIM4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM5IE` reader - illegal access interrupt enable for LPTIM5"]
pub type LPTIM5IE_R = crate::BitReader;
#[doc = "Field `LPTIM5IE` writer - illegal access interrupt enable for LPTIM5"]
pub type LPTIM5IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - illegal access interrupt enable for FDCAN1"]
    #[inline(always)]
    pub fn fdcan1ie(&self) -> FDCAN1IE_R {
        FDCAN1IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - illegal access interrupt enable for FDCAN2"]
    #[inline(always)]
    pub fn fdcan2ie(&self) -> FDCAN2IE_R {
        FDCAN2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - illegal access interrupt enable for UCPD"]
    #[inline(always)]
    pub fn ucpdie(&self) -> UCPDIE_R {
        UCPDIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - illegal access interrupt enable for TIM1"]
    #[inline(always)]
    pub fn tim1ie(&self) -> TIM1IE_R {
        TIM1IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - illegal access interrupt enable for SPI1"]
    #[inline(always)]
    pub fn spi1ie(&self) -> SPI1IE_R {
        SPI1IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - illegal access interrupt enable for TIM8"]
    #[inline(always)]
    pub fn tim8ie(&self) -> TIM8IE_R {
        TIM8IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - illegal access interrupt enable for USART1"]
    #[inline(always)]
    pub fn usart1ie(&self) -> USART1IE_R {
        USART1IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - illegal access interrupt enable for TIM15"]
    #[inline(always)]
    pub fn tim15ie(&self) -> TIM15IE_R {
        TIM15IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - illegal access interrupt enable for TIM16"]
    #[inline(always)]
    pub fn tim16ie(&self) -> TIM16IE_R {
        TIM16IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - illegal access interrupt enable for TIM17"]
    #[inline(always)]
    pub fn tim17ie(&self) -> TIM17IE_R {
        TIM17IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - illegal access interrupt enable for SPI4"]
    #[inline(always)]
    pub fn spi4ie(&self) -> SPI4IE_R {
        SPI4IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - illegal access interrupt enable for SPI6"]
    #[inline(always)]
    pub fn spi6ie(&self) -> SPI6IE_R {
        SPI6IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - illegal access interrupt enable for SAI1"]
    #[inline(always)]
    pub fn sai1ie(&self) -> SAI1IE_R {
        SAI1IE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - illegal access interrupt enable for SAI2"]
    #[inline(always)]
    pub fn sai2ie(&self) -> SAI2IE_R {
        SAI2IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - illegal access interrupt enable for USB"]
    #[inline(always)]
    pub fn usbie(&self) -> USBIE_R {
        USBIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - illegal access interrupt enable for SPI5"]
    #[inline(always)]
    pub fn spi5ie(&self) -> SPI5IE_R {
        SPI5IE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - illegal access interrupt enable for LPUART"]
    #[inline(always)]
    pub fn lpuart1ie(&self) -> LPUART1IE_R {
        LPUART1IE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - illegal access interrupt enable for I2C3"]
    #[inline(always)]
    pub fn i2c3ie(&self) -> I2C3IE_R {
        I2C3IE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - illegal access interrupt enable for I2C4"]
    #[inline(always)]
    pub fn i2c4ie(&self) -> I2C4IE_R {
        I2C4IE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - illegal access interrupt enable for LPTIM1"]
    #[inline(always)]
    pub fn lptim1ie(&self) -> LPTIM1IE_R {
        LPTIM1IE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - illegal access interrupt enable for LPTIM3"]
    #[inline(always)]
    pub fn lptim3ie(&self) -> LPTIM3IE_R {
        LPTIM3IE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - illegal access interrupt enable for LPTIM4"]
    #[inline(always)]
    pub fn lptim4ie(&self) -> LPTIM4IE_R {
        LPTIM4IE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - illegal access interrupt enable for LPTIM5"]
    #[inline(always)]
    pub fn lptim5ie(&self) -> LPTIM5IE_R {
        LPTIM5IE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - illegal access interrupt enable for FDCAN1"]
    #[inline(always)]
    #[must_use]
    pub fn fdcan1ie(&mut self) -> FDCAN1IE_W<GTZC1_TZIC_IER2rs> {
        FDCAN1IE_W::new(self, 0)
    }
    #[doc = "Bit 1 - illegal access interrupt enable for FDCAN2"]
    #[inline(always)]
    #[must_use]
    pub fn fdcan2ie(&mut self) -> FDCAN2IE_W<GTZC1_TZIC_IER2rs> {
        FDCAN2IE_W::new(self, 1)
    }
    #[doc = "Bit 2 - illegal access interrupt enable for UCPD"]
    #[inline(always)]
    #[must_use]
    pub fn ucpdie(&mut self) -> UCPDIE_W<GTZC1_TZIC_IER2rs> {
        UCPDIE_W::new(self, 2)
    }
    #[doc = "Bit 8 - illegal access interrupt enable for TIM1"]
    #[inline(always)]
    #[must_use]
    pub fn tim1ie(&mut self) -> TIM1IE_W<GTZC1_TZIC_IER2rs> {
        TIM1IE_W::new(self, 8)
    }
    #[doc = "Bit 9 - illegal access interrupt enable for SPI1"]
    #[inline(always)]
    #[must_use]
    pub fn spi1ie(&mut self) -> SPI1IE_W<GTZC1_TZIC_IER2rs> {
        SPI1IE_W::new(self, 9)
    }
    #[doc = "Bit 10 - illegal access interrupt enable for TIM8"]
    #[inline(always)]
    #[must_use]
    pub fn tim8ie(&mut self) -> TIM8IE_W<GTZC1_TZIC_IER2rs> {
        TIM8IE_W::new(self, 10)
    }
    #[doc = "Bit 11 - illegal access interrupt enable for USART1"]
    #[inline(always)]
    #[must_use]
    pub fn usart1ie(&mut self) -> USART1IE_W<GTZC1_TZIC_IER2rs> {
        USART1IE_W::new(self, 11)
    }
    #[doc = "Bit 12 - illegal access interrupt enable for TIM15"]
    #[inline(always)]
    #[must_use]
    pub fn tim15ie(&mut self) -> TIM15IE_W<GTZC1_TZIC_IER2rs> {
        TIM15IE_W::new(self, 12)
    }
    #[doc = "Bit 13 - illegal access interrupt enable for TIM16"]
    #[inline(always)]
    #[must_use]
    pub fn tim16ie(&mut self) -> TIM16IE_W<GTZC1_TZIC_IER2rs> {
        TIM16IE_W::new(self, 13)
    }
    #[doc = "Bit 14 - illegal access interrupt enable for TIM17"]
    #[inline(always)]
    #[must_use]
    pub fn tim17ie(&mut self) -> TIM17IE_W<GTZC1_TZIC_IER2rs> {
        TIM17IE_W::new(self, 14)
    }
    #[doc = "Bit 15 - illegal access interrupt enable for SPI4"]
    #[inline(always)]
    #[must_use]
    pub fn spi4ie(&mut self) -> SPI4IE_W<GTZC1_TZIC_IER2rs> {
        SPI4IE_W::new(self, 15)
    }
    #[doc = "Bit 16 - illegal access interrupt enable for SPI6"]
    #[inline(always)]
    #[must_use]
    pub fn spi6ie(&mut self) -> SPI6IE_W<GTZC1_TZIC_IER2rs> {
        SPI6IE_W::new(self, 16)
    }
    #[doc = "Bit 17 - illegal access interrupt enable for SAI1"]
    #[inline(always)]
    #[must_use]
    pub fn sai1ie(&mut self) -> SAI1IE_W<GTZC1_TZIC_IER2rs> {
        SAI1IE_W::new(self, 17)
    }
    #[doc = "Bit 18 - illegal access interrupt enable for SAI2"]
    #[inline(always)]
    #[must_use]
    pub fn sai2ie(&mut self) -> SAI2IE_W<GTZC1_TZIC_IER2rs> {
        SAI2IE_W::new(self, 18)
    }
    #[doc = "Bit 19 - illegal access interrupt enable for USB"]
    #[inline(always)]
    #[must_use]
    pub fn usbie(&mut self) -> USBIE_W<GTZC1_TZIC_IER2rs> {
        USBIE_W::new(self, 19)
    }
    #[doc = "Bit 24 - illegal access interrupt enable for SPI5"]
    #[inline(always)]
    #[must_use]
    pub fn spi5ie(&mut self) -> SPI5IE_W<GTZC1_TZIC_IER2rs> {
        SPI5IE_W::new(self, 24)
    }
    #[doc = "Bit 25 - illegal access interrupt enable for LPUART"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1ie(&mut self) -> LPUART1IE_W<GTZC1_TZIC_IER2rs> {
        LPUART1IE_W::new(self, 25)
    }
    #[doc = "Bit 26 - illegal access interrupt enable for I2C3"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3ie(&mut self) -> I2C3IE_W<GTZC1_TZIC_IER2rs> {
        I2C3IE_W::new(self, 26)
    }
    #[doc = "Bit 27 - illegal access interrupt enable for I2C4"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4ie(&mut self) -> I2C4IE_W<GTZC1_TZIC_IER2rs> {
        I2C4IE_W::new(self, 27)
    }
    #[doc = "Bit 28 - illegal access interrupt enable for LPTIM1"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1ie(&mut self) -> LPTIM1IE_W<GTZC1_TZIC_IER2rs> {
        LPTIM1IE_W::new(self, 28)
    }
    #[doc = "Bit 29 - illegal access interrupt enable for LPTIM3"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3ie(&mut self) -> LPTIM3IE_W<GTZC1_TZIC_IER2rs> {
        LPTIM3IE_W::new(self, 29)
    }
    #[doc = "Bit 30 - illegal access interrupt enable for LPTIM4"]
    #[inline(always)]
    #[must_use]
    pub fn lptim4ie(&mut self) -> LPTIM4IE_W<GTZC1_TZIC_IER2rs> {
        LPTIM4IE_W::new(self, 30)
    }
    #[doc = "Bit 31 - illegal access interrupt enable for LPTIM5"]
    #[inline(always)]
    #[must_use]
    pub fn lptim5ie(&mut self) -> LPTIM5IE_W<GTZC1_TZIC_IER2rs> {
        LPTIM5IE_W::new(self, 31)
    }
}
#[doc = "GTZC1 TZIC interrupt enable register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_tzic_ier2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzic_ier2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTZC1_TZIC_IER2rs;
impl crate::RegisterSpec for GTZC1_TZIC_IER2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtzc1_tzic_ier2::R`](R) reader structure"]
impl crate::Readable for GTZC1_TZIC_IER2rs {}
#[doc = "`write(|w| ..)` method takes [`gtzc1_tzic_ier2::W`](W) writer structure"]
impl crate::Writable for GTZC1_TZIC_IER2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTZC1_TZIC_IER2 to value 0"]
impl crate::Resettable for GTZC1_TZIC_IER2rs {
    const RESET_VALUE: u32 = 0;
}
