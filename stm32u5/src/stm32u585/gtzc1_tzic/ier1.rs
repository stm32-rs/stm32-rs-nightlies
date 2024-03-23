#[doc = "Register `IER1` reader"]
pub type R = crate::R<IER1rs>;
#[doc = "Register `IER1` writer"]
pub type W = crate::W<IER1rs>;
#[doc = "Field `TIM2IE` reader - TIM2IE"]
pub type TIM2IE_R = crate::BitReader;
#[doc = "Field `TIM2IE` writer - TIM2IE"]
pub type TIM2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM3IE` reader - TIM3IE"]
pub type TIM3IE_R = crate::BitReader;
#[doc = "Field `TIM3IE` writer - TIM3IE"]
pub type TIM3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM4IE` reader - TIM4IE"]
pub type TIM4IE_R = crate::BitReader;
#[doc = "Field `TIM4IE` writer - TIM4IE"]
pub type TIM4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM5IE` reader - TIM5IE"]
pub type TIM5IE_R = crate::BitReader;
#[doc = "Field `TIM5IE` writer - TIM5IE"]
pub type TIM5IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM6IE` reader - TIM6IE"]
pub type TIM6IE_R = crate::BitReader;
#[doc = "Field `TIM6IE` writer - TIM6IE"]
pub type TIM6IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM7IE` reader - TIM7IE"]
pub type TIM7IE_R = crate::BitReader;
#[doc = "Field `TIM7IE` writer - TIM7IE"]
pub type TIM7IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGIE` reader - WWDGIE"]
pub type WWDGIE_R = crate::BitReader;
#[doc = "Field `WWDGIE` writer - WWDGIE"]
pub type WWDGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDGIE` reader - IWDGIE"]
pub type IWDGIE_R = crate::BitReader;
#[doc = "Field `IWDGIE` writer - IWDGIE"]
pub type IWDGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2IE` reader - SPI2IE"]
pub type SPI2IE_R = crate::BitReader;
#[doc = "Field `SPI2IE` writer - SPI2IE"]
pub type SPI2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2IE` reader - illegal access interrupt enable for USART2"]
pub type USART2IE_R = crate::BitReader;
#[doc = "Field `USART2IE` writer - illegal access interrupt enable for USART2"]
pub type USART2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3IE` reader - illegal access interrupt enable for USART3"]
pub type USART3IE_R = crate::BitReader;
#[doc = "Field `USART3IE` writer - illegal access interrupt enable for USART3"]
pub type USART3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART4IE` reader - illegal access interrupt enable for UART4"]
pub type USART4IE_R = crate::BitReader;
#[doc = "Field `USART4IE` writer - illegal access interrupt enable for UART4"]
pub type USART4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5IE` reader - illegal access interrupt enable for UART5"]
pub type UART5IE_R = crate::BitReader;
#[doc = "Field `UART5IE` writer - illegal access interrupt enable for UART5"]
pub type UART5IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1IE` reader - illegal access interrupt enable for I2C1"]
pub type I2C1IE_R = crate::BitReader;
#[doc = "Field `I2C1IE` writer - illegal access interrupt enable for I2C1"]
pub type I2C1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2IE` reader - illegal access interrupt enable for I2C2"]
pub type I2C2IE_R = crate::BitReader;
#[doc = "Field `I2C2IE` writer - illegal access interrupt enable for I2C2"]
pub type I2C2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRSIE` reader - illegal access interrupt enable for CRS"]
pub type CRSIE_R = crate::BitReader;
#[doc = "Field `CRSIE` writer - illegal access interrupt enable for CRS"]
pub type CRSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4IE` reader - illegal access interrupt enable for I2C4"]
pub type I2C4IE_R = crate::BitReader;
#[doc = "Field `I2C4IE` writer - illegal access interrupt enable for I2C4"]
pub type I2C4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM2IE` reader - illegal access interrupt enable for LPTIM2"]
pub type LPTIM2IE_R = crate::BitReader;
#[doc = "Field `LPTIM2IE` writer - illegal access interrupt enable for LPTIM2"]
pub type LPTIM2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDCAN1IE` reader - illegal access interrupt enable for FDCAN1"]
pub type FDCAN1IE_R = crate::BitReader;
#[doc = "Field `FDCAN1IE` writer - illegal access interrupt enable for FDCAN1"]
pub type FDCAN1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPD1IE` reader - illegal access interrupt enable for UCPD1"]
pub type UCPD1IE_R = crate::BitReader;
#[doc = "Field `UCPD1IE` writer - illegal access interrupt enable for UCPD1"]
pub type UCPD1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM2IE"]
    #[inline(always)]
    pub fn tim2ie(&self) -> TIM2IE_R {
        TIM2IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3IE"]
    #[inline(always)]
    pub fn tim3ie(&self) -> TIM3IE_R {
        TIM3IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4IE"]
    #[inline(always)]
    pub fn tim4ie(&self) -> TIM4IE_R {
        TIM4IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5IE"]
    #[inline(always)]
    pub fn tim5ie(&self) -> TIM5IE_R {
        TIM5IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6IE"]
    #[inline(always)]
    pub fn tim6ie(&self) -> TIM6IE_R {
        TIM6IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7IE"]
    #[inline(always)]
    pub fn tim7ie(&self) -> TIM7IE_R {
        TIM7IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - WWDGIE"]
    #[inline(always)]
    pub fn wwdgie(&self) -> WWDGIE_R {
        WWDGIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IWDGIE"]
    #[inline(always)]
    pub fn iwdgie(&self) -> IWDGIE_R {
        IWDGIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SPI2IE"]
    #[inline(always)]
    pub fn spi2ie(&self) -> SPI2IE_R {
        SPI2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - illegal access interrupt enable for USART2"]
    #[inline(always)]
    pub fn usart2ie(&self) -> USART2IE_R {
        USART2IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - illegal access interrupt enable for USART3"]
    #[inline(always)]
    pub fn usart3ie(&self) -> USART3IE_R {
        USART3IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - illegal access interrupt enable for UART4"]
    #[inline(always)]
    pub fn usart4ie(&self) -> USART4IE_R {
        USART4IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - illegal access interrupt enable for UART5"]
    #[inline(always)]
    pub fn uart5ie(&self) -> UART5IE_R {
        UART5IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - illegal access interrupt enable for I2C1"]
    #[inline(always)]
    pub fn i2c1ie(&self) -> I2C1IE_R {
        I2C1IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - illegal access interrupt enable for I2C2"]
    #[inline(always)]
    pub fn i2c2ie(&self) -> I2C2IE_R {
        I2C2IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - illegal access interrupt enable for CRS"]
    #[inline(always)]
    pub fn crsie(&self) -> CRSIE_R {
        CRSIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - illegal access interrupt enable for I2C4"]
    #[inline(always)]
    pub fn i2c4ie(&self) -> I2C4IE_R {
        I2C4IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - illegal access interrupt enable for LPTIM2"]
    #[inline(always)]
    pub fn lptim2ie(&self) -> LPTIM2IE_R {
        LPTIM2IE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - illegal access interrupt enable for FDCAN1"]
    #[inline(always)]
    pub fn fdcan1ie(&self) -> FDCAN1IE_R {
        FDCAN1IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - illegal access interrupt enable for UCPD1"]
    #[inline(always)]
    pub fn ucpd1ie(&self) -> UCPD1IE_R {
        UCPD1IE_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2IE"]
    #[inline(always)]
    #[must_use]
    pub fn tim2ie(&mut self) -> TIM2IE_W<IER1rs> {
        TIM2IE_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3IE"]
    #[inline(always)]
    #[must_use]
    pub fn tim3ie(&mut self) -> TIM3IE_W<IER1rs> {
        TIM3IE_W::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4IE"]
    #[inline(always)]
    #[must_use]
    pub fn tim4ie(&mut self) -> TIM4IE_W<IER1rs> {
        TIM4IE_W::new(self, 2)
    }
    #[doc = "Bit 3 - TIM5IE"]
    #[inline(always)]
    #[must_use]
    pub fn tim5ie(&mut self) -> TIM5IE_W<IER1rs> {
        TIM5IE_W::new(self, 3)
    }
    #[doc = "Bit 4 - TIM6IE"]
    #[inline(always)]
    #[must_use]
    pub fn tim6ie(&mut self) -> TIM6IE_W<IER1rs> {
        TIM6IE_W::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7IE"]
    #[inline(always)]
    #[must_use]
    pub fn tim7ie(&mut self) -> TIM7IE_W<IER1rs> {
        TIM7IE_W::new(self, 5)
    }
    #[doc = "Bit 6 - WWDGIE"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgie(&mut self) -> WWDGIE_W<IER1rs> {
        WWDGIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - IWDGIE"]
    #[inline(always)]
    #[must_use]
    pub fn iwdgie(&mut self) -> IWDGIE_W<IER1rs> {
        IWDGIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - SPI2IE"]
    #[inline(always)]
    #[must_use]
    pub fn spi2ie(&mut self) -> SPI2IE_W<IER1rs> {
        SPI2IE_W::new(self, 8)
    }
    #[doc = "Bit 9 - illegal access interrupt enable for USART2"]
    #[inline(always)]
    #[must_use]
    pub fn usart2ie(&mut self) -> USART2IE_W<IER1rs> {
        USART2IE_W::new(self, 9)
    }
    #[doc = "Bit 10 - illegal access interrupt enable for USART3"]
    #[inline(always)]
    #[must_use]
    pub fn usart3ie(&mut self) -> USART3IE_W<IER1rs> {
        USART3IE_W::new(self, 10)
    }
    #[doc = "Bit 11 - illegal access interrupt enable for UART4"]
    #[inline(always)]
    #[must_use]
    pub fn usart4ie(&mut self) -> USART4IE_W<IER1rs> {
        USART4IE_W::new(self, 11)
    }
    #[doc = "Bit 12 - illegal access interrupt enable for UART5"]
    #[inline(always)]
    #[must_use]
    pub fn uart5ie(&mut self) -> UART5IE_W<IER1rs> {
        UART5IE_W::new(self, 12)
    }
    #[doc = "Bit 13 - illegal access interrupt enable for I2C1"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1ie(&mut self) -> I2C1IE_W<IER1rs> {
        I2C1IE_W::new(self, 13)
    }
    #[doc = "Bit 14 - illegal access interrupt enable for I2C2"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2ie(&mut self) -> I2C2IE_W<IER1rs> {
        I2C2IE_W::new(self, 14)
    }
    #[doc = "Bit 15 - illegal access interrupt enable for CRS"]
    #[inline(always)]
    #[must_use]
    pub fn crsie(&mut self) -> CRSIE_W<IER1rs> {
        CRSIE_W::new(self, 15)
    }
    #[doc = "Bit 16 - illegal access interrupt enable for I2C4"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4ie(&mut self) -> I2C4IE_W<IER1rs> {
        I2C4IE_W::new(self, 16)
    }
    #[doc = "Bit 17 - illegal access interrupt enable for LPTIM2"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2ie(&mut self) -> LPTIM2IE_W<IER1rs> {
        LPTIM2IE_W::new(self, 17)
    }
    #[doc = "Bit 18 - illegal access interrupt enable for FDCAN1"]
    #[inline(always)]
    #[must_use]
    pub fn fdcan1ie(&mut self) -> FDCAN1IE_W<IER1rs> {
        FDCAN1IE_W::new(self, 18)
    }
    #[doc = "Bit 19 - illegal access interrupt enable for UCPD1"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1ie(&mut self) -> UCPD1IE_W<IER1rs> {
        UCPD1IE_W::new(self, 19)
    }
}
#[doc = "TZIC interrupt enable register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER1rs;
impl crate::RegisterSpec for IER1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier1::R`](R) reader structure"]
impl crate::Readable for IER1rs {}
#[doc = "`write(|w| ..)` method takes [`ier1::W`](W) writer structure"]
impl crate::Writable for IER1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER1 to value 0"]
impl crate::Resettable for IER1rs {
    const RESET_VALUE: u32 = 0;
}
