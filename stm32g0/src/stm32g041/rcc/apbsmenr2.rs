#[doc = "Register `APBSMENR2` reader"]
pub type R = crate::R<APBSMENR2rs>;
#[doc = "Register `APBSMENR2` writer"]
pub type W = crate::W<APBSMENR2rs>;
#[doc = "Field `SYSCFGSMEN` reader - SYSCFG, COMP and VREFBUF clock enable during Sleep mode"]
pub type SYSCFGSMEN_R = crate::BitReader;
#[doc = "Field `SYSCFGSMEN` writer - SYSCFG, COMP and VREFBUF clock enable during Sleep mode"]
pub type SYSCFGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1SMEN` reader - TIM1 timer clock enable during Sleep mode"]
pub type TIM1SMEN_R = crate::BitReader;
#[doc = "Field `TIM1SMEN` writer - TIM1 timer clock enable during Sleep mode"]
pub type TIM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1SMEN` reader - SPI1 clock enable during Sleep mode"]
pub type SPI1SMEN_R = crate::BitReader;
#[doc = "Field `SPI1SMEN` writer - SPI1 clock enable during Sleep mode"]
pub type SPI1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1SMEN` reader - USART1 clock enable during Sleep mode"]
pub type USART1SMEN_R = crate::BitReader;
#[doc = "Field `USART1SMEN` writer - USART1 clock enable during Sleep mode"]
pub type USART1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM14SMEN` reader - TIM14 timer clock enable during Sleep mode"]
pub type TIM14SMEN_R = crate::BitReader;
#[doc = "Field `TIM14SMEN` writer - TIM14 timer clock enable during Sleep mode"]
pub type TIM14SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16SMEN` reader - TIM16 timer clock enable during Sleep mode"]
pub type TIM16SMEN_R = crate::BitReader;
#[doc = "Field `TIM16SMEN` writer - TIM16 timer clock enable during Sleep mode"]
pub type TIM16SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17SMEN` reader - TIM16 timer clock enable during Sleep mode"]
pub type TIM17SMEN_R = crate::BitReader;
#[doc = "Field `TIM17SMEN` writer - TIM16 timer clock enable during Sleep mode"]
pub type TIM17SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCSMEN` reader - ADC clock enable during Sleep mode"]
pub type ADCSMEN_R = crate::BitReader;
#[doc = "Field `ADCSMEN` writer - ADC clock enable during Sleep mode"]
pub type ADCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF clock enable during Sleep mode"]
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SYSCFGSMEN_R {
        SYSCFGSMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim1smen(&self) -> TIM1SMEN_R {
        TIM1SMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TIM14 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim14smen(&self) -> TIM14SMEN_R {
        TIM14SMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim16smen(&self) -> TIM16SMEN_R {
        TIM16SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM16 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim17smen(&self) -> TIM17SMEN_R {
        TIM17SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn adcsmen(&self) -> ADCSMEN_R {
        ADCSMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgsmen(&mut self) -> SYSCFGSMEN_W<APBSMENR2rs> {
        SYSCFGSMEN_W::new(self, 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim1smen(&mut self) -> TIM1SMEN_W<APBSMENR2rs> {
        TIM1SMEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W<APBSMENR2rs> {
        SPI1SMEN_W::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart1smen(&mut self) -> USART1SMEN_W<APBSMENR2rs> {
        USART1SMEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - TIM14 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim14smen(&mut self) -> TIM14SMEN_W<APBSMENR2rs> {
        TIM14SMEN_W::new(self, 15)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim16smen(&mut self) -> TIM16SMEN_W<APBSMENR2rs> {
        TIM16SMEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM16 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim17smen(&mut self) -> TIM17SMEN_W<APBSMENR2rs> {
        TIM17SMEN_W::new(self, 18)
    }
    #[doc = "Bit 20 - ADC clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn adcsmen(&mut self) -> ADCSMEN_W<APBSMENR2rs> {
        ADCSMEN_W::new(self, 20)
    }
}
#[doc = "APB peripheral clock enable in Sleep mode register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbsmenr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbsmenr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBSMENR2rs;
impl crate::RegisterSpec for APBSMENR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbsmenr2::R`](R) reader structure"]
impl crate::Readable for APBSMENR2rs {}
#[doc = "`write(|w| ..)` method takes [`apbsmenr2::W`](W) writer structure"]
impl crate::Writable for APBSMENR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBSMENR2 to value 0"]
impl crate::Resettable for APBSMENR2rs {
    const RESET_VALUE: u32 = 0;
}
