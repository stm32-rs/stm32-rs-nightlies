#[doc = "Register `APB1LPENR` reader"]
pub type R = crate::R<APB1LPENRrs>;
#[doc = "Register `APB1LPENR` writer"]
pub type W = crate::W<APB1LPENRrs>;
#[doc = "Field `TIM2LPEN` reader - Timer 2 clock enable during Sleep mode"]
pub type TIM2LPEN_R = crate::BitReader;
#[doc = "Field `TIM2LPEN` writer - Timer 2 clock enable during Sleep mode"]
pub type TIM2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM3LPEN` reader - Timer 3 clock enable during Sleep mode"]
pub type TIM3LPEN_R = crate::BitReader;
#[doc = "Field `TIM3LPEN` writer - Timer 3 clock enable during Sleep mode"]
pub type TIM3LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM4LPEN` reader - Timer 4 clock enable during Sleep mode"]
pub type TIM4LPEN_R = crate::BitReader;
#[doc = "Field `TIM4LPEN` writer - Timer 4 clock enable during Sleep mode"]
pub type TIM4LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM6LPEN` reader - Timer 6 clock enable during Sleep mode"]
pub type TIM6LPEN_R = crate::BitReader;
#[doc = "Field `TIM6LPEN` writer - Timer 6 clock enable during Sleep mode"]
pub type TIM6LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM7LPEN` reader - Timer 7 clock enable during Sleep mode"]
pub type TIM7LPEN_R = crate::BitReader;
#[doc = "Field `TIM7LPEN` writer - Timer 7 clock enable during Sleep mode"]
pub type TIM7LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDLPEN` reader - LCD clock enable during Sleep mode"]
pub type LCDLPEN_R = crate::BitReader;
#[doc = "Field `LCDLPEN` writer - LCD clock enable during Sleep mode"]
pub type LCDLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGLPEN` reader - Window watchdog clock enable during Sleep mode"]
pub type WWDGLPEN_R = crate::BitReader;
#[doc = "Field `WWDGLPEN` writer - Window watchdog clock enable during Sleep mode"]
pub type WWDGLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2LPEN` reader - SPI 2 clock enable during Sleep mode"]
pub type SPI2LPEN_R = crate::BitReader;
#[doc = "Field `SPI2LPEN` writer - SPI 2 clock enable during Sleep mode"]
pub type SPI2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2LPEN` reader - USART 2 clock enable during Sleep mode"]
pub type USART2LPEN_R = crate::BitReader;
#[doc = "Field `USART2LPEN` writer - USART 2 clock enable during Sleep mode"]
pub type USART2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3LPEN` reader - USART 3 clock enable during Sleep mode"]
pub type USART3LPEN_R = crate::BitReader;
#[doc = "Field `USART3LPEN` writer - USART 3 clock enable during Sleep mode"]
pub type USART3LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1LPEN` reader - I2C 1 clock enable during Sleep mode"]
pub type I2C1LPEN_R = crate::BitReader;
#[doc = "Field `I2C1LPEN` writer - I2C 1 clock enable during Sleep mode"]
pub type I2C1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2LPEN` reader - I2C 2 clock enable during Sleep mode"]
pub type I2C2LPEN_R = crate::BitReader;
#[doc = "Field `I2C2LPEN` writer - I2C 2 clock enable during Sleep mode"]
pub type I2C2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBLPEN` reader - USB clock enable during Sleep mode"]
pub type USBLPEN_R = crate::BitReader;
#[doc = "Field `USBLPEN` writer - USB clock enable during Sleep mode"]
pub type USBLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRLPEN` reader - Power interface clock enable during Sleep mode"]
pub type PWRLPEN_R = crate::BitReader;
#[doc = "Field `PWRLPEN` writer - Power interface clock enable during Sleep mode"]
pub type PWRLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACLPEN` reader - DAC interface clock enable during Sleep mode"]
pub type DACLPEN_R = crate::BitReader;
#[doc = "Field `DACLPEN` writer - DAC interface clock enable during Sleep mode"]
pub type DACLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPLPEN` reader - COMP interface clock enable during Sleep mode"]
pub type COMPLPEN_R = crate::BitReader;
#[doc = "Field `COMPLPEN` writer - COMP interface clock enable during Sleep mode"]
pub type COMPLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer 2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim2lpen(&self) -> TIM2LPEN_R {
        TIM2LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim3lpen(&self) -> TIM3LPEN_R {
        TIM3LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim4lpen(&self) -> TIM4LPEN_R {
        TIM4LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 6 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim6lpen(&self) -> TIM6LPEN_R {
        TIM6LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 7 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim7lpen(&self) -> TIM7LPEN_R {
        TIM7LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD clock enable during Sleep mode"]
    #[inline(always)]
    pub fn lcdlpen(&self) -> LCDLPEN_R {
        LCDLPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable during Sleep mode"]
    #[inline(always)]
    pub fn wwdglpen(&self) -> WWDGLPEN_R {
        WWDGLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI 2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi2lpen(&self) -> SPI2LPEN_R {
        SPI2LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - USART 2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart2lpen(&self) -> USART2LPEN_R {
        USART2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART 3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart3lpen(&self) -> USART3LPEN_R {
        USART3LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C 1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c1lpen(&self) -> I2C1LPEN_R {
        I2C1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C 2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c2lpen(&self) -> I2C2LPEN_R {
        I2C2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USB clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usblpen(&self) -> USBLPEN_R {
        USBLPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn pwrlpen(&self) -> PWRLPEN_R {
        PWRLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn daclpen(&self) -> DACLPEN_R {
        DACLPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - COMP interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn complpen(&self) -> COMPLPEN_R {
        COMPLPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim2lpen(&mut self) -> TIM2LPEN_W<APB1LPENRrs> {
        TIM2LPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer 3 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim3lpen(&mut self) -> TIM3LPEN_W<APB1LPENRrs> {
        TIM3LPEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer 4 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim4lpen(&mut self) -> TIM4LPEN_W<APB1LPENRrs> {
        TIM4LPEN_W::new(self, 2)
    }
    #[doc = "Bit 4 - Timer 6 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim6lpen(&mut self) -> TIM6LPEN_W<APB1LPENRrs> {
        TIM6LPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Timer 7 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim7lpen(&mut self) -> TIM7LPEN_W<APB1LPENRrs> {
        TIM7LPEN_W::new(self, 5)
    }
    #[doc = "Bit 9 - LCD clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn lcdlpen(&mut self) -> LCDLPEN_W<APB1LPENRrs> {
        LCDLPEN_W::new(self, 9)
    }
    #[doc = "Bit 11 - Window watchdog clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn wwdglpen(&mut self) -> WWDGLPEN_W<APB1LPENRrs> {
        WWDGLPEN_W::new(self, 11)
    }
    #[doc = "Bit 14 - SPI 2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi2lpen(&mut self) -> SPI2LPEN_W<APB1LPENRrs> {
        SPI2LPEN_W::new(self, 14)
    }
    #[doc = "Bit 17 - USART 2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart2lpen(&mut self) -> USART2LPEN_W<APB1LPENRrs> {
        USART2LPEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART 3 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart3lpen(&mut self) -> USART3LPEN_W<APB1LPENRrs> {
        USART3LPEN_W::new(self, 18)
    }
    #[doc = "Bit 21 - I2C 1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1lpen(&mut self) -> I2C1LPEN_W<APB1LPENRrs> {
        I2C1LPEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C 2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2lpen(&mut self) -> I2C2LPEN_W<APB1LPENRrs> {
        I2C2LPEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - USB clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usblpen(&mut self) -> USBLPEN_W<APB1LPENRrs> {
        USBLPEN_W::new(self, 23)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwrlpen(&mut self) -> PWRLPEN_W<APB1LPENRrs> {
        PWRLPEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - DAC interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn daclpen(&mut self) -> DACLPEN_W<APB1LPENRrs> {
        DACLPEN_W::new(self, 29)
    }
    #[doc = "Bit 31 - COMP interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn complpen(&mut self) -> COMPLPEN_W<APB1LPENRrs> {
        COMPLPEN_W::new(self, 31)
    }
}
#[doc = "APB1 peripheral clock enable in low power mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1LPENRrs;
impl crate::RegisterSpec for APB1LPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1lpenr::R`](R) reader structure"]
impl crate::Readable for APB1LPENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb1lpenr::W`](W) writer structure"]
impl crate::Writable for APB1LPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1LPENR to value 0"]
impl crate::Resettable for APB1LPENRrs {
    const RESET_VALUE: u32 = 0;
}
