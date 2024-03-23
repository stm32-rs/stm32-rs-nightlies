#[doc = "Register `APB2LPENR` reader"]
pub type R = crate::R<APB2LPENRrs>;
#[doc = "Register `APB2LPENR` writer"]
pub type W = crate::W<APB2LPENRrs>;
#[doc = "Field `SYSCFGLPEN` reader - System configuration controller clock enable during Sleep mode"]
pub type SYSCFGLPEN_R = crate::BitReader;
#[doc = "Field `SYSCFGLPEN` writer - System configuration controller clock enable during Sleep mode"]
pub type SYSCFGLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM9LPEN` reader - TIM9 timer clock enable during Sleep mode"]
pub type TIM9LPEN_R = crate::BitReader;
#[doc = "Field `TIM9LPEN` writer - TIM9 timer clock enable during Sleep mode"]
pub type TIM9LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM10LPEN` reader - TIM10 timer clock enable during Sleep mode"]
pub type TIM10LPEN_R = crate::BitReader;
#[doc = "Field `TIM10LPEN` writer - TIM10 timer clock enable during Sleep mode"]
pub type TIM10LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM11LPEN` reader - TIM11 timer clock enable during Sleep mode"]
pub type TIM11LPEN_R = crate::BitReader;
#[doc = "Field `TIM11LPEN` writer - TIM11 timer clock enable during Sleep mode"]
pub type TIM11LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1LPEN` reader - ADC1 interface clock enable during Sleep mode"]
pub type ADC1LPEN_R = crate::BitReader;
#[doc = "Field `ADC1LPEN` writer - ADC1 interface clock enable during Sleep mode"]
pub type ADC1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOLPEN` reader - SDIO clock enable during Sleep mode"]
pub type SDIOLPEN_R = crate::BitReader;
#[doc = "Field `SDIOLPEN` writer - SDIO clock enable during Sleep mode"]
pub type SDIOLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1LPEN` reader - SPI 1 clock enable during Sleep mode"]
pub type SPI1LPEN_R = crate::BitReader;
#[doc = "Field `SPI1LPEN` writer - SPI 1 clock enable during Sleep mode"]
pub type SPI1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1LPEN` reader - USART1 clock enable during Sleep mode"]
pub type USART1LPEN_R = crate::BitReader;
#[doc = "Field `USART1LPEN` writer - USART1 clock enable during Sleep mode"]
pub type USART1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - System configuration controller clock enable during Sleep mode"]
    #[inline(always)]
    pub fn syscfglpen(&self) -> SYSCFGLPEN_R {
        SYSCFGLPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - TIM9 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim9lpen(&self) -> TIM9LPEN_R {
        TIM9LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM10 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim10lpen(&self) -> TIM10LPEN_R {
        TIM10LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM11 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim11lpen(&self) -> TIM11LPEN_R {
        TIM11LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC1 interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn adc1lpen(&self) -> ADC1LPEN_R {
        ADC1LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - SDIO clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sdiolpen(&self) -> SDIOLPEN_R {
        SDIOLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI 1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi1lpen(&self) -> SPI1LPEN_R {
        SPI1LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart1lpen(&self) -> USART1LPEN_R {
        USART1LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System configuration controller clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn syscfglpen(&mut self) -> SYSCFGLPEN_W<APB2LPENRrs> {
        SYSCFGLPEN_W::new(self, 0)
    }
    #[doc = "Bit 2 - TIM9 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim9lpen(&mut self) -> TIM9LPEN_W<APB2LPENRrs> {
        TIM9LPEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - TIM10 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim10lpen(&mut self) -> TIM10LPEN_W<APB2LPENRrs> {
        TIM10LPEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - TIM11 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim11lpen(&mut self) -> TIM11LPEN_W<APB2LPENRrs> {
        TIM11LPEN_W::new(self, 4)
    }
    #[doc = "Bit 9 - ADC1 interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn adc1lpen(&mut self) -> ADC1LPEN_W<APB2LPENRrs> {
        ADC1LPEN_W::new(self, 9)
    }
    #[doc = "Bit 11 - SDIO clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sdiolpen(&mut self) -> SDIOLPEN_W<APB2LPENRrs> {
        SDIOLPEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI 1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi1lpen(&mut self) -> SPI1LPEN_W<APB2LPENRrs> {
        SPI1LPEN_W::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W<APB2LPENRrs> {
        USART1LPEN_W::new(self, 14)
    }
}
#[doc = "APB2 peripheral clock enable in low power mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2LPENRrs;
impl crate::RegisterSpec for APB2LPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2lpenr::R`](R) reader structure"]
impl crate::Readable for APB2LPENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb2lpenr::W`](W) writer structure"]
impl crate::Writable for APB2LPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2LPENR to value 0"]
impl crate::Resettable for APB2LPENRrs {
    const RESET_VALUE: u32 = 0;
}
