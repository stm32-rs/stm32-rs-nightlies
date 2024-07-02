///Register `RCC_AHB3RSTR` reader
pub type R = crate::R<RCC_AHB3RSTRrs>;
///Register `RCC_AHB3RSTR` writer
pub type W = crate::W<RCC_AHB3RSTRrs>;
///Field `LPGPIO1RST` reader - LPGPIO1 reset This bit is set and cleared by software.
pub type LPGPIO1RST_R = crate::BitReader;
///Field `LPGPIO1RST` writer - LPGPIO1 reset This bit is set and cleared by software.
pub type LPGPIO1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC4RST` reader - ADC4 reset This bit is set and cleared by software.
pub type ADC4RST_R = crate::BitReader;
///Field `ADC4RST` writer - ADC4 reset This bit is set and cleared by software.
pub type ADC4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC1RST` reader - DAC1 reset This bit is set and cleared by software.
pub type DAC1RST_R = crate::BitReader;
///Field `DAC1RST` writer - DAC1 reset This bit is set and cleared by software.
pub type DAC1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPDMA1RST` reader - LPDMA1 reset This bit is set and cleared by software.
pub type LPDMA1RST_R = crate::BitReader;
///Field `LPDMA1RST` writer - LPDMA1 reset This bit is set and cleared by software.
pub type LPDMA1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADF1RST` reader - ADF1 reset This bit is set and cleared by software.
pub type ADF1RST_R = crate::BitReader;
///Field `ADF1RST` writer - ADF1 reset This bit is set and cleared by software.
pub type ADF1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LPGPIO1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn lpgpio1rst(&self) -> LPGPIO1RST_R {
        LPGPIO1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 5 - ADC4 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn adc4rst(&self) -> ADC4RST_R {
        ADC4RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DAC1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn dac1rst(&self) -> DAC1RST_R {
        DAC1RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - LPDMA1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn lpdma1rst(&self) -> LPDMA1RST_R {
        LPDMA1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ADF1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn adf1rst(&self) -> ADF1RST_R {
        ADF1RST_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_AHB3RSTR")
            .field("lpgpio1rst", &self.lpgpio1rst())
            .field("adc4rst", &self.adc4rst())
            .field("dac1rst", &self.dac1rst())
            .field("lpdma1rst", &self.lpdma1rst())
            .field("adf1rst", &self.adf1rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - LPGPIO1 reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lpgpio1rst(&mut self) -> LPGPIO1RST_W<RCC_AHB3RSTRrs> {
        LPGPIO1RST_W::new(self, 0)
    }
    ///Bit 5 - ADC4 reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn adc4rst(&mut self) -> ADC4RST_W<RCC_AHB3RSTRrs> {
        ADC4RST_W::new(self, 5)
    }
    ///Bit 6 - DAC1 reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn dac1rst(&mut self) -> DAC1RST_W<RCC_AHB3RSTRrs> {
        DAC1RST_W::new(self, 6)
    }
    ///Bit 9 - LPDMA1 reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lpdma1rst(&mut self) -> LPDMA1RST_W<RCC_AHB3RSTRrs> {
        LPDMA1RST_W::new(self, 9)
    }
    ///Bit 10 - ADF1 reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn adf1rst(&mut self) -> ADF1RST_W<RCC_AHB3RSTRrs> {
        ADF1RST_W::new(self, 10)
    }
}
/**RCC AHB3 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`rcc_ahb3rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb3rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#RCC:RCC_AHB3RSTR)*/
pub struct RCC_AHB3RSTRrs;
impl crate::RegisterSpec for RCC_AHB3RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_ahb3rstr::R`](R) reader structure
impl crate::Readable for RCC_AHB3RSTRrs {}
///`write(|w| ..)` method takes [`rcc_ahb3rstr::W`](W) writer structure
impl crate::Writable for RCC_AHB3RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_AHB3RSTR to value 0
impl crate::Resettable for RCC_AHB3RSTRrs {
    const RESET_VALUE: u32 = 0;
}
