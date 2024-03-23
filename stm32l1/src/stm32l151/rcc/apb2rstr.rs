#[doc = "Register `APB2RSTR` reader"]
pub type R = crate::R<APB2RSTRrs>;
#[doc = "Register `APB2RSTR` writer"]
pub type W = crate::W<APB2RSTRrs>;
#[doc = "Field `SYSCFGRST` reader - SYSCFGRST"]
pub type SYSCFGRST_R = crate::BitReader;
#[doc = "Field `SYSCFGRST` writer - SYSCFGRST"]
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM9RST` reader - TIM9RST"]
pub type TIM9RST_R = crate::BitReader;
#[doc = "Field `TIM9RST` writer - TIM9RST"]
pub type TIM9RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM10RST` reader - TM10RST"]
pub type TM10RST_R = crate::BitReader;
#[doc = "Field `TM10RST` writer - TM10RST"]
pub type TM10RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM11RST` reader - TM11RST"]
pub type TM11RST_R = crate::BitReader;
#[doc = "Field `TM11RST` writer - TM11RST"]
pub type TM11RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1RST` reader - ADC1RST"]
pub type ADC1RST_R = crate::BitReader;
#[doc = "Field `ADC1RST` writer - ADC1RST"]
pub type ADC1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIORST` reader - SDIORST"]
pub type SDIORST_R = crate::BitReader;
#[doc = "Field `SDIORST` writer - SDIORST"]
pub type SDIORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1RST` reader - SPI1RST"]
pub type SPI1RST_R = crate::BitReader;
#[doc = "Field `SPI1RST` writer - SPI1RST"]
pub type SPI1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1RST` reader - USART1RST"]
pub type USART1RST_R = crate::BitReader;
#[doc = "Field `USART1RST` writer - USART1RST"]
pub type USART1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYSCFGRST"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - TIM9RST"]
    #[inline(always)]
    pub fn tim9rst(&self) -> TIM9RST_R {
        TIM9RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TM10RST"]
    #[inline(always)]
    pub fn tm10rst(&self) -> TM10RST_R {
        TM10RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TM11RST"]
    #[inline(always)]
    pub fn tm11rst(&self) -> TM11RST_R {
        TM11RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC1RST"]
    #[inline(always)]
    pub fn adc1rst(&self) -> ADC1RST_R {
        ADC1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - SDIORST"]
    #[inline(always)]
    pub fn sdiorst(&self) -> SDIORST_R {
        SDIORST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1RST"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1RST"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFGRST"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<APB2RSTRrs> {
        SYSCFGRST_W::new(self, 0)
    }
    #[doc = "Bit 2 - TIM9RST"]
    #[inline(always)]
    #[must_use]
    pub fn tim9rst(&mut self) -> TIM9RST_W<APB2RSTRrs> {
        TIM9RST_W::new(self, 2)
    }
    #[doc = "Bit 3 - TM10RST"]
    #[inline(always)]
    #[must_use]
    pub fn tm10rst(&mut self) -> TM10RST_W<APB2RSTRrs> {
        TM10RST_W::new(self, 3)
    }
    #[doc = "Bit 4 - TM11RST"]
    #[inline(always)]
    #[must_use]
    pub fn tm11rst(&mut self) -> TM11RST_W<APB2RSTRrs> {
        TM11RST_W::new(self, 4)
    }
    #[doc = "Bit 9 - ADC1RST"]
    #[inline(always)]
    #[must_use]
    pub fn adc1rst(&mut self) -> ADC1RST_W<APB2RSTRrs> {
        ADC1RST_W::new(self, 9)
    }
    #[doc = "Bit 11 - SDIORST"]
    #[inline(always)]
    #[must_use]
    pub fn sdiorst(&mut self) -> SDIORST_W<APB2RSTRrs> {
        SDIORST_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1RST"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<APB2RSTRrs> {
        SPI1RST_W::new(self, 12)
    }
    #[doc = "Bit 14 - USART1RST"]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<APB2RSTRrs> {
        USART1RST_W::new(self, 14)
    }
}
#[doc = "APB2 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2RSTRrs;
impl crate::RegisterSpec for APB2RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rstr::R`](R) reader structure"]
impl crate::Readable for APB2RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`apb2rstr::W`](W) writer structure"]
impl crate::Writable for APB2RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2RSTR to value 0"]
impl crate::Resettable for APB2RSTRrs {
    const RESET_VALUE: u32 = 0;
}
