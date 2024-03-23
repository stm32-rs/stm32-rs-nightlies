#[doc = "Register `RCC_AHB3SMENR` reader"]
pub type R = crate::R<RCC_AHB3SMENRrs>;
#[doc = "Register `RCC_AHB3SMENR` writer"]
pub type W = crate::W<RCC_AHB3SMENRrs>;
#[doc = "Field `LPGPIO1SMEN` reader - LPGPIO1 enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type LPGPIO1SMEN_R = crate::BitReader;
#[doc = "Field `LPGPIO1SMEN` writer - LPGPIO1 enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type LPGPIO1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSMEN` reader - PWR clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type PWRSMEN_R = crate::BitReader;
#[doc = "Field `PWRSMEN` writer - PWR clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type PWRSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC4SMEN` reader - ADC4 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
pub type ADC4SMEN_R = crate::BitReader;
#[doc = "Field `ADC4SMEN` writer - ADC4 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
pub type ADC4SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC1SMEN` reader - DAC1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
pub type DAC1SMEN_R = crate::BitReader;
#[doc = "Field `DAC1SMEN` writer - DAC1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
pub type DAC1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPDMA1SMEN` reader - LPDMA1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
pub type LPDMA1SMEN_R = crate::BitReader;
#[doc = "Field `LPDMA1SMEN` writer - LPDMA1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
pub type LPDMA1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADF1SMEN` reader - ADF1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
pub type ADF1SMEN_R = crate::BitReader;
#[doc = "Field `ADF1SMEN` writer - ADF1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
pub type ADF1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTZC2SMEN` reader - GTZC2 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type GTZC2SMEN_R = crate::BitReader;
#[doc = "Field `GTZC2SMEN` writer - GTZC2 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type GTZC2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM4SMEN` reader - SRAM4 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type SRAM4SMEN_R = crate::BitReader;
#[doc = "Field `SRAM4SMEN` writer - SRAM4 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type SRAM4SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LPGPIO1 enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    pub fn lpgpio1smen(&self) -> LPGPIO1SMEN_R {
        LPGPIO1SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PWR clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC4 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
    #[inline(always)]
    pub fn adc4smen(&self) -> ADC4SMEN_R {
        ADC4SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DAC1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
    #[inline(always)]
    pub fn dac1smen(&self) -> DAC1SMEN_R {
        DAC1SMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - LPDMA1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
    #[inline(always)]
    pub fn lpdma1smen(&self) -> LPDMA1SMEN_R {
        LPDMA1SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADF1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
    #[inline(always)]
    pub fn adf1smen(&self) -> ADF1SMEN_R {
        ADF1SMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - GTZC2 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gtzc2smen(&self) -> GTZC2SMEN_R {
        GTZC2SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 31 - SRAM4 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    pub fn sram4smen(&self) -> SRAM4SMEN_R {
        SRAM4SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPGPIO1 enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpgpio1smen(&mut self) -> LPGPIO1SMEN_W<RCC_AHB3SMENRrs> {
        LPGPIO1SMEN_W::new(self, 0)
    }
    #[doc = "Bit 2 - PWR clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W<RCC_AHB3SMENRrs> {
        PWRSMEN_W::new(self, 2)
    }
    #[doc = "Bit 5 - ADC4 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
    #[inline(always)]
    #[must_use]
    pub fn adc4smen(&mut self) -> ADC4SMEN_W<RCC_AHB3SMENRrs> {
        ADC4SMEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - DAC1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
    #[inline(always)]
    #[must_use]
    pub fn dac1smen(&mut self) -> DAC1SMEN_W<RCC_AHB3SMENRrs> {
        DAC1SMEN_W::new(self, 6)
    }
    #[doc = "Bit 9 - LPDMA1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
    #[inline(always)]
    #[must_use]
    pub fn lpdma1smen(&mut self) -> LPDMA1SMEN_W<RCC_AHB3SMENRrs> {
        LPDMA1SMEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - ADF1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
    #[inline(always)]
    #[must_use]
    pub fn adf1smen(&mut self) -> ADF1SMEN_W<RCC_AHB3SMENRrs> {
        ADF1SMEN_W::new(self, 10)
    }
    #[doc = "Bit 12 - GTZC2 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gtzc2smen(&mut self) -> GTZC2SMEN_W<RCC_AHB3SMENRrs> {
        GTZC2SMEN_W::new(self, 12)
    }
    #[doc = "Bit 31 - SRAM4 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn sram4smen(&mut self) -> SRAM4SMEN_W<RCC_AHB3SMENRrs> {
        SRAM4SMEN_W::new(self, 31)
    }
}
#[doc = "RCC AHB3 peripheral clock enable in Sleep and Stop modes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb3smenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb3smenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_AHB3SMENRrs;
impl crate::RegisterSpec for RCC_AHB3SMENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahb3smenr::R`](R) reader structure"]
impl crate::Readable for RCC_AHB3SMENRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahb3smenr::W`](W) writer structure"]
impl crate::Writable for RCC_AHB3SMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_AHB3SMENR to value 0xffff_ffff"]
impl crate::Resettable for RCC_AHB3SMENRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
