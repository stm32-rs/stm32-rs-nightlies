///Register `IER2` reader
pub type R = crate::R<IER2rs>;
///Register `IER2` writer
pub type W = crate::W<IER2rs>;
///Field `FDCAN1IE` reader - illegal access interrupt enable for FDCAN1
pub type FDCAN1IE_R = crate::BitReader;
///Field `FDCAN1IE` writer - illegal access interrupt enable for FDCAN1
pub type FDCAN1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDCAN2IE` reader - illegal access interrupt enable for FDCAN2
pub type FDCAN2IE_R = crate::BitReader;
///Field `FDCAN2IE` writer - illegal access interrupt enable for FDCAN2
pub type FDCAN2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCPDIE` reader - illegal access interrupt enable for UCPD
pub type UCPDIE_R = crate::BitReader;
///Field `UCPDIE` writer - illegal access interrupt enable for UCPD
pub type UCPDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM1IE` reader - illegal access interrupt enable for TIM1
pub type TIM1IE_R = crate::BitReader;
///Field `TIM1IE` writer - illegal access interrupt enable for TIM1
pub type TIM1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1IE` reader - illegal access interrupt enable for SPI1
pub type SPI1IE_R = crate::BitReader;
///Field `SPI1IE` writer - illegal access interrupt enable for SPI1
pub type SPI1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM8IE` reader - illegal access interrupt enable for TIM8
pub type TIM8IE_R = crate::BitReader;
///Field `TIM8IE` writer - illegal access interrupt enable for TIM8
pub type TIM8IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1IE` reader - illegal access interrupt enable for USART1
pub type USART1IE_R = crate::BitReader;
///Field `USART1IE` writer - illegal access interrupt enable for USART1
pub type USART1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM15IE` reader - illegal access interrupt enable for TIM15
pub type TIM15IE_R = crate::BitReader;
///Field `TIM15IE` writer - illegal access interrupt enable for TIM15
pub type TIM15IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16IE` reader - illegal access interrupt enable for TIM16
pub type TIM16IE_R = crate::BitReader;
///Field `TIM16IE` writer - illegal access interrupt enable for TIM16
pub type TIM16IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17IE` reader - illegal access interrupt enable for TIM17
pub type TIM17IE_R = crate::BitReader;
///Field `TIM17IE` writer - illegal access interrupt enable for TIM17
pub type TIM17IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI4IE` reader - illegal access interrupt enable for SPI4
pub type SPI4IE_R = crate::BitReader;
///Field `SPI4IE` writer - illegal access interrupt enable for SPI4
pub type SPI4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI6IE` reader - illegal access interrupt enable for SPI6
pub type SPI6IE_R = crate::BitReader;
///Field `SPI6IE` writer - illegal access interrupt enable for SPI6
pub type SPI6IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI1IE` reader - illegal access interrupt enable for SAI1
pub type SAI1IE_R = crate::BitReader;
///Field `SAI1IE` writer - illegal access interrupt enable for SAI1
pub type SAI1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI2IE` reader - illegal access interrupt enable for SAI2
pub type SAI2IE_R = crate::BitReader;
///Field `SAI2IE` writer - illegal access interrupt enable for SAI2
pub type SAI2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBIE` reader - illegal access interrupt enable for USB
pub type USBIE_R = crate::BitReader;
///Field `USBIE` writer - illegal access interrupt enable for USB
pub type USBIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI5IE` reader - illegal access interrupt enable for SPI5
pub type SPI5IE_R = crate::BitReader;
///Field `SPI5IE` writer - illegal access interrupt enable for SPI5
pub type SPI5IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1IE` reader - illegal access interrupt enable for LPUART
pub type LPUART1IE_R = crate::BitReader;
///Field `LPUART1IE` writer - illegal access interrupt enable for LPUART
pub type LPUART1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3IE` reader - illegal access interrupt enable for I2C3
pub type I2C3IE_R = crate::BitReader;
///Field `I2C3IE` writer - illegal access interrupt enable for I2C3
pub type I2C3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4IE` reader - illegal access interrupt enable for I2C4
pub type I2C4IE_R = crate::BitReader;
///Field `I2C4IE` writer - illegal access interrupt enable for I2C4
pub type I2C4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1IE` reader - illegal access interrupt enable for LPTIM1
pub type LPTIM1IE_R = crate::BitReader;
///Field `LPTIM1IE` writer - illegal access interrupt enable for LPTIM1
pub type LPTIM1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3IE` reader - illegal access interrupt enable for LPTIM3
pub type LPTIM3IE_R = crate::BitReader;
///Field `LPTIM3IE` writer - illegal access interrupt enable for LPTIM3
pub type LPTIM3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM4IE` reader - illegal access interrupt enable for LPTIM4
pub type LPTIM4IE_R = crate::BitReader;
///Field `LPTIM4IE` writer - illegal access interrupt enable for LPTIM4
pub type LPTIM4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM5IE` reader - illegal access interrupt enable for LPTIM5
pub type LPTIM5IE_R = crate::BitReader;
///Field `LPTIM5IE` writer - illegal access interrupt enable for LPTIM5
pub type LPTIM5IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - illegal access interrupt enable for FDCAN1
    #[inline(always)]
    pub fn fdcan1ie(&self) -> FDCAN1IE_R {
        FDCAN1IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access interrupt enable for FDCAN2
    #[inline(always)]
    pub fn fdcan2ie(&self) -> FDCAN2IE_R {
        FDCAN2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access interrupt enable for UCPD
    #[inline(always)]
    pub fn ucpdie(&self) -> UCPDIE_R {
        UCPDIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - illegal access interrupt enable for TIM1
    #[inline(always)]
    pub fn tim1ie(&self) -> TIM1IE_R {
        TIM1IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access interrupt enable for SPI1
    #[inline(always)]
    pub fn spi1ie(&self) -> SPI1IE_R {
        SPI1IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access interrupt enable for TIM8
    #[inline(always)]
    pub fn tim8ie(&self) -> TIM8IE_R {
        TIM8IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access interrupt enable for USART1
    #[inline(always)]
    pub fn usart1ie(&self) -> USART1IE_R {
        USART1IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access interrupt enable for TIM15
    #[inline(always)]
    pub fn tim15ie(&self) -> TIM15IE_R {
        TIM15IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access interrupt enable for TIM16
    #[inline(always)]
    pub fn tim16ie(&self) -> TIM16IE_R {
        TIM16IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access interrupt enable for TIM17
    #[inline(always)]
    pub fn tim17ie(&self) -> TIM17IE_R {
        TIM17IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access interrupt enable for SPI4
    #[inline(always)]
    pub fn spi4ie(&self) -> SPI4IE_R {
        SPI4IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access interrupt enable for SPI6
    #[inline(always)]
    pub fn spi6ie(&self) -> SPI6IE_R {
        SPI6IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access interrupt enable for SAI1
    #[inline(always)]
    pub fn sai1ie(&self) -> SAI1IE_R {
        SAI1IE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access interrupt enable for SAI2
    #[inline(always)]
    pub fn sai2ie(&self) -> SAI2IE_R {
        SAI2IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access interrupt enable for USB
    #[inline(always)]
    pub fn usbie(&self) -> USBIE_R {
        USBIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - illegal access interrupt enable for SPI5
    #[inline(always)]
    pub fn spi5ie(&self) -> SPI5IE_R {
        SPI5IE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access interrupt enable for LPUART
    #[inline(always)]
    pub fn lpuart1ie(&self) -> LPUART1IE_R {
        LPUART1IE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - illegal access interrupt enable for I2C3
    #[inline(always)]
    pub fn i2c3ie(&self) -> I2C3IE_R {
        I2C3IE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - illegal access interrupt enable for I2C4
    #[inline(always)]
    pub fn i2c4ie(&self) -> I2C4IE_R {
        I2C4IE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - illegal access interrupt enable for LPTIM1
    #[inline(always)]
    pub fn lptim1ie(&self) -> LPTIM1IE_R {
        LPTIM1IE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - illegal access interrupt enable for LPTIM3
    #[inline(always)]
    pub fn lptim3ie(&self) -> LPTIM3IE_R {
        LPTIM3IE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - illegal access interrupt enable for LPTIM4
    #[inline(always)]
    pub fn lptim4ie(&self) -> LPTIM4IE_R {
        LPTIM4IE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - illegal access interrupt enable for LPTIM5
    #[inline(always)]
    pub fn lptim5ie(&self) -> LPTIM5IE_R {
        LPTIM5IE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER2")
            .field("fdcan1ie", &self.fdcan1ie())
            .field("fdcan2ie", &self.fdcan2ie())
            .field("ucpdie", &self.ucpdie())
            .field("tim1ie", &self.tim1ie())
            .field("spi1ie", &self.spi1ie())
            .field("tim8ie", &self.tim8ie())
            .field("usart1ie", &self.usart1ie())
            .field("tim15ie", &self.tim15ie())
            .field("tim16ie", &self.tim16ie())
            .field("tim17ie", &self.tim17ie())
            .field("spi4ie", &self.spi4ie())
            .field("spi6ie", &self.spi6ie())
            .field("sai1ie", &self.sai1ie())
            .field("sai2ie", &self.sai2ie())
            .field("usbie", &self.usbie())
            .field("spi5ie", &self.spi5ie())
            .field("lpuart1ie", &self.lpuart1ie())
            .field("i2c3ie", &self.i2c3ie())
            .field("i2c4ie", &self.i2c4ie())
            .field("lptim1ie", &self.lptim1ie())
            .field("lptim3ie", &self.lptim3ie())
            .field("lptim4ie", &self.lptim4ie())
            .field("lptim5ie", &self.lptim5ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - illegal access interrupt enable for FDCAN1
    #[inline(always)]
    pub fn fdcan1ie(&mut self) -> FDCAN1IE_W<'_, IER2rs> {
        FDCAN1IE_W::new(self, 0)
    }
    ///Bit 1 - illegal access interrupt enable for FDCAN2
    #[inline(always)]
    pub fn fdcan2ie(&mut self) -> FDCAN2IE_W<'_, IER2rs> {
        FDCAN2IE_W::new(self, 1)
    }
    ///Bit 2 - illegal access interrupt enable for UCPD
    #[inline(always)]
    pub fn ucpdie(&mut self) -> UCPDIE_W<'_, IER2rs> {
        UCPDIE_W::new(self, 2)
    }
    ///Bit 8 - illegal access interrupt enable for TIM1
    #[inline(always)]
    pub fn tim1ie(&mut self) -> TIM1IE_W<'_, IER2rs> {
        TIM1IE_W::new(self, 8)
    }
    ///Bit 9 - illegal access interrupt enable for SPI1
    #[inline(always)]
    pub fn spi1ie(&mut self) -> SPI1IE_W<'_, IER2rs> {
        SPI1IE_W::new(self, 9)
    }
    ///Bit 10 - illegal access interrupt enable for TIM8
    #[inline(always)]
    pub fn tim8ie(&mut self) -> TIM8IE_W<'_, IER2rs> {
        TIM8IE_W::new(self, 10)
    }
    ///Bit 11 - illegal access interrupt enable for USART1
    #[inline(always)]
    pub fn usart1ie(&mut self) -> USART1IE_W<'_, IER2rs> {
        USART1IE_W::new(self, 11)
    }
    ///Bit 12 - illegal access interrupt enable for TIM15
    #[inline(always)]
    pub fn tim15ie(&mut self) -> TIM15IE_W<'_, IER2rs> {
        TIM15IE_W::new(self, 12)
    }
    ///Bit 13 - illegal access interrupt enable for TIM16
    #[inline(always)]
    pub fn tim16ie(&mut self) -> TIM16IE_W<'_, IER2rs> {
        TIM16IE_W::new(self, 13)
    }
    ///Bit 14 - illegal access interrupt enable for TIM17
    #[inline(always)]
    pub fn tim17ie(&mut self) -> TIM17IE_W<'_, IER2rs> {
        TIM17IE_W::new(self, 14)
    }
    ///Bit 15 - illegal access interrupt enable for SPI4
    #[inline(always)]
    pub fn spi4ie(&mut self) -> SPI4IE_W<'_, IER2rs> {
        SPI4IE_W::new(self, 15)
    }
    ///Bit 16 - illegal access interrupt enable for SPI6
    #[inline(always)]
    pub fn spi6ie(&mut self) -> SPI6IE_W<'_, IER2rs> {
        SPI6IE_W::new(self, 16)
    }
    ///Bit 17 - illegal access interrupt enable for SAI1
    #[inline(always)]
    pub fn sai1ie(&mut self) -> SAI1IE_W<'_, IER2rs> {
        SAI1IE_W::new(self, 17)
    }
    ///Bit 18 - illegal access interrupt enable for SAI2
    #[inline(always)]
    pub fn sai2ie(&mut self) -> SAI2IE_W<'_, IER2rs> {
        SAI2IE_W::new(self, 18)
    }
    ///Bit 19 - illegal access interrupt enable for USB
    #[inline(always)]
    pub fn usbie(&mut self) -> USBIE_W<'_, IER2rs> {
        USBIE_W::new(self, 19)
    }
    ///Bit 24 - illegal access interrupt enable for SPI5
    #[inline(always)]
    pub fn spi5ie(&mut self) -> SPI5IE_W<'_, IER2rs> {
        SPI5IE_W::new(self, 24)
    }
    ///Bit 25 - illegal access interrupt enable for LPUART
    #[inline(always)]
    pub fn lpuart1ie(&mut self) -> LPUART1IE_W<'_, IER2rs> {
        LPUART1IE_W::new(self, 25)
    }
    ///Bit 26 - illegal access interrupt enable for I2C3
    #[inline(always)]
    pub fn i2c3ie(&mut self) -> I2C3IE_W<'_, IER2rs> {
        I2C3IE_W::new(self, 26)
    }
    ///Bit 27 - illegal access interrupt enable for I2C4
    #[inline(always)]
    pub fn i2c4ie(&mut self) -> I2C4IE_W<'_, IER2rs> {
        I2C4IE_W::new(self, 27)
    }
    ///Bit 28 - illegal access interrupt enable for LPTIM1
    #[inline(always)]
    pub fn lptim1ie(&mut self) -> LPTIM1IE_W<'_, IER2rs> {
        LPTIM1IE_W::new(self, 28)
    }
    ///Bit 29 - illegal access interrupt enable for LPTIM3
    #[inline(always)]
    pub fn lptim3ie(&mut self) -> LPTIM3IE_W<'_, IER2rs> {
        LPTIM3IE_W::new(self, 29)
    }
    ///Bit 30 - illegal access interrupt enable for LPTIM4
    #[inline(always)]
    pub fn lptim4ie(&mut self) -> LPTIM4IE_W<'_, IER2rs> {
        LPTIM4IE_W::new(self, 30)
    }
    ///Bit 31 - illegal access interrupt enable for LPTIM5
    #[inline(always)]
    pub fn lptim5ie(&mut self) -> LPTIM5IE_W<'_, IER2rs> {
        LPTIM5IE_W::new(self, 31)
    }
}
/**GTZC1 TZIC interrupt enable register 2

You can [`read`](crate::Reg::read) this register and get [`ier2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#GTZC1_TZIC:IER2)*/
pub struct IER2rs;
impl crate::RegisterSpec for IER2rs {
    type Ux = u32;
}
///`read()` method returns [`ier2::R`](R) reader structure
impl crate::Readable for IER2rs {}
///`write(|w| ..)` method takes [`ier2::W`](W) writer structure
impl crate::Writable for IER2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER2 to value 0
impl crate::Resettable for IER2rs {}
