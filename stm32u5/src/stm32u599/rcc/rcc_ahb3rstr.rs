#[doc = "Register `RCC_AHB3RSTR` reader"]
pub type R = crate::R<RCC_AHB3RSTRrs>;
#[doc = "Register `RCC_AHB3RSTR` writer"]
pub type W = crate::W<RCC_AHB3RSTRrs>;
#[doc = "Field `LPGPIO1RST` reader - LPGPIO1 reset This bit is set and cleared by software."]
pub type LPGPIO1RST_R = crate::BitReader;
#[doc = "Field `LPGPIO1RST` writer - LPGPIO1 reset This bit is set and cleared by software."]
pub type LPGPIO1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC4RST` reader - ADC4 reset This bit is set and cleared by software."]
pub type ADC4RST_R = crate::BitReader;
#[doc = "Field `ADC4RST` writer - ADC4 reset This bit is set and cleared by software."]
pub type ADC4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC1RST` reader - DAC1 reset This bit is set and cleared by software."]
pub type DAC1RST_R = crate::BitReader;
#[doc = "Field `DAC1RST` writer - DAC1 reset This bit is set and cleared by software."]
pub type DAC1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPDMA1RST` reader - LPDMA1 reset This bit is set and cleared by software."]
pub type LPDMA1RST_R = crate::BitReader;
#[doc = "Field `LPDMA1RST` writer - LPDMA1 reset This bit is set and cleared by software."]
pub type LPDMA1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADF1RST` reader - ADF1 reset This bit is set and cleared by software."]
pub type ADF1RST_R = crate::BitReader;
#[doc = "Field `ADF1RST` writer - ADF1 reset This bit is set and cleared by software."]
pub type ADF1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LPGPIO1 reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn lpgpio1rst(&self) -> LPGPIO1RST_R {
        LPGPIO1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - ADC4 reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn adc4rst(&self) -> ADC4RST_R {
        ADC4RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DAC1 reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dac1rst(&self) -> DAC1RST_R {
        DAC1RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - LPDMA1 reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn lpdma1rst(&self) -> LPDMA1RST_R {
        LPDMA1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADF1 reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn adf1rst(&self) -> ADF1RST_R {
        ADF1RST_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPGPIO1 reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpgpio1rst(&mut self) -> LPGPIO1RST_W<RCC_AHB3RSTRrs> {
        LPGPIO1RST_W::new(self, 0)
    }
    #[doc = "Bit 5 - ADC4 reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn adc4rst(&mut self) -> ADC4RST_W<RCC_AHB3RSTRrs> {
        ADC4RST_W::new(self, 5)
    }
    #[doc = "Bit 6 - DAC1 reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dac1rst(&mut self) -> DAC1RST_W<RCC_AHB3RSTRrs> {
        DAC1RST_W::new(self, 6)
    }
    #[doc = "Bit 9 - LPDMA1 reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpdma1rst(&mut self) -> LPDMA1RST_W<RCC_AHB3RSTRrs> {
        LPDMA1RST_W::new(self, 9)
    }
    #[doc = "Bit 10 - ADF1 reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn adf1rst(&mut self) -> ADF1RST_W<RCC_AHB3RSTRrs> {
        ADF1RST_W::new(self, 10)
    }
}
#[doc = "RCC AHB3 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb3rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb3rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_AHB3RSTRrs;
impl crate::RegisterSpec for RCC_AHB3RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahb3rstr::R`](R) reader structure"]
impl crate::Readable for RCC_AHB3RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahb3rstr::W`](W) writer structure"]
impl crate::Writable for RCC_AHB3RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_AHB3RSTR to value 0"]
impl crate::Resettable for RCC_AHB3RSTRrs {
    const RESET_VALUE: u32 = 0;
}
