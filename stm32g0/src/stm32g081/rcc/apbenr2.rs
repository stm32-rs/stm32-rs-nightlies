#[doc = "Register `APBENR2` reader"]
pub type R = crate::R<APBENR2rs>;
#[doc = "Register `APBENR2` writer"]
pub type W = crate::W<APBENR2rs>;
#[doc = "Field `SYSCFGEN` reader - SYSCFG, COMP and VREFBUF clock enable"]
pub type SYSCFGEN_R = crate::BitReader;
#[doc = "Field `SYSCFGEN` writer - SYSCFG, COMP and VREFBUF clock enable"]
pub type SYSCFGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1EN` reader - TIM1 timer clock enable"]
pub type TIM1EN_R = crate::BitReader;
#[doc = "Field `TIM1EN` writer - TIM1 timer clock enable"]
pub type TIM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1EN` reader - SPI1 clock enable"]
pub type SPI1EN_R = crate::BitReader;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable"]
pub type SPI1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub type USART1EN_R = crate::BitReader;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub type USART1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM14EN` reader - TIM14 timer clock enable"]
pub type TIM14EN_R = crate::BitReader;
#[doc = "Field `TIM14EN` writer - TIM14 timer clock enable"]
pub type TIM14EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15EN` reader - TIM15 timer clock enable"]
pub type TIM15EN_R = crate::BitReader;
#[doc = "Field `TIM15EN` writer - TIM15 timer clock enable"]
pub type TIM15EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16EN` reader - TIM16 timer clock enable"]
pub type TIM16EN_R = crate::BitReader;
#[doc = "Field `TIM16EN` writer - TIM16 timer clock enable"]
pub type TIM16EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17EN` reader - TIM16 timer clock enable"]
pub type TIM17EN_R = crate::BitReader;
#[doc = "Field `TIM17EN` writer - TIM16 timer clock enable"]
pub type TIM17EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCEN` reader - ADC clock enable"]
pub type ADCEN_R = crate::BitReader;
#[doc = "Field `ADCEN` writer - ADC clock enable"]
pub type ADCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF clock enable"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TIM14 timer clock enable"]
    #[inline(always)]
    pub fn tim14en(&self) -> TIM14EN_R {
        TIM14EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer clock enable"]
    #[inline(always)]
    pub fn tim15en(&self) -> TIM15EN_R {
        TIM15EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable"]
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM16 timer clock enable"]
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC clock enable"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<APBENR2rs> {
        SYSCFGEN_W::new(self, 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<APBENR2rs> {
        TIM1EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<APBENR2rs> {
        SPI1EN_W::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<APBENR2rs> {
        USART1EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - TIM14 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim14en(&mut self) -> TIM14EN_W<APBENR2rs> {
        TIM14EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - TIM15 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim15en(&mut self) -> TIM15EN_W<APBENR2rs> {
        TIM15EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim16en(&mut self) -> TIM16EN_W<APBENR2rs> {
        TIM16EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM16 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim17en(&mut self) -> TIM17EN_W<APBENR2rs> {
        TIM17EN_W::new(self, 18)
    }
    #[doc = "Bit 20 - ADC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> ADCEN_W<APBENR2rs> {
        ADCEN_W::new(self, 20)
    }
}
#[doc = "APB peripheral clock enable register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbenr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbenr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBENR2rs;
impl crate::RegisterSpec for APBENR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbenr2::R`](R) reader structure"]
impl crate::Readable for APBENR2rs {}
#[doc = "`write(|w| ..)` method takes [`apbenr2::W`](W) writer structure"]
impl crate::Writable for APBENR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBENR2 to value 0"]
impl crate::Resettable for APBENR2rs {
    const RESET_VALUE: u32 = 0;
}
