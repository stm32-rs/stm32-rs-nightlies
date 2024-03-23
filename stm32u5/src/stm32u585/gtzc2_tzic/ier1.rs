#[doc = "Register `IER1` reader"]
pub type R = crate::R<IER1rs>;
#[doc = "Register `IER1` writer"]
pub type W = crate::W<IER1rs>;
#[doc = "Field `SPI3IE` reader - illegal access interrupt enable for SPI3"]
pub type SPI3IE_R = crate::BitReader;
#[doc = "Field `SPI3IE` writer - illegal access interrupt enable for SPI3"]
pub type SPI3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUART1IE` reader - illegal access interrupt enable for LPUART1"]
pub type LPUART1IE_R = crate::BitReader;
#[doc = "Field `LPUART1IE` writer - illegal access interrupt enable for LPUART1"]
pub type LPUART1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3IE` reader - illegal access interrupt enable for I2C3"]
pub type I2C3IE_R = crate::BitReader;
#[doc = "Field `I2C3IE` writer - illegal access interrupt enable for I2C3"]
pub type I2C3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1IE` reader - illegal access interrupt enable for LPTIM1"]
pub type LPTIM1IE_R = crate::BitReader;
#[doc = "Field `LPTIM1IE` writer - illegal access interrupt enable for LPTIM1"]
pub type LPTIM1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM3IE` reader - illegal access interrupt enable for LPTIM3"]
pub type LPTIM3IE_R = crate::BitReader;
#[doc = "Field `LPTIM3IE` writer - illegal access interrupt enable for LPTIM3"]
pub type LPTIM3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM4IE` reader - illegal access interrupt enable for LPTIM4"]
pub type LPTIM4IE_R = crate::BitReader;
#[doc = "Field `LPTIM4IE` writer - illegal access interrupt enable for LPTIM4"]
pub type LPTIM4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPAMPIE` reader - illegal access interrupt enable for OPAMP"]
pub type OPAMPIE_R = crate::BitReader;
#[doc = "Field `OPAMPIE` writer - illegal access interrupt enable for OPAMP"]
pub type OPAMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPIE` reader - illegal access interrupt enable for COMP"]
pub type COMPIE_R = crate::BitReader;
#[doc = "Field `COMPIE` writer - illegal access interrupt enable for COMP"]
pub type COMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC4IE` reader - illegal access interrupt enable for ADC4"]
pub type ADC4IE_R = crate::BitReader;
#[doc = "Field `ADC4IE` writer - illegal access interrupt enable for ADC4"]
pub type ADC4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFBUFIE` reader - illegal access interrupt enable for VREFBUF"]
pub type VREFBUFIE_R = crate::BitReader;
#[doc = "Field `VREFBUFIE` writer - illegal access interrupt enable for VREFBUF"]
pub type VREFBUFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC1IE` reader - illegal access interrupt enable for DAC1"]
pub type DAC1IE_R = crate::BitReader;
#[doc = "Field `DAC1IE` writer - illegal access interrupt enable for DAC1"]
pub type DAC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADF1IE` reader - illegal access interrupt enable for ADF1"]
pub type ADF1IE_R = crate::BitReader;
#[doc = "Field `ADF1IE` writer - illegal access interrupt enable for ADF1"]
pub type ADF1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - illegal access interrupt enable for SPI3"]
    #[inline(always)]
    pub fn spi3ie(&self) -> SPI3IE_R {
        SPI3IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - illegal access interrupt enable for LPUART1"]
    #[inline(always)]
    pub fn lpuart1ie(&self) -> LPUART1IE_R {
        LPUART1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - illegal access interrupt enable for I2C3"]
    #[inline(always)]
    pub fn i2c3ie(&self) -> I2C3IE_R {
        I2C3IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - illegal access interrupt enable for LPTIM1"]
    #[inline(always)]
    pub fn lptim1ie(&self) -> LPTIM1IE_R {
        LPTIM1IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - illegal access interrupt enable for LPTIM3"]
    #[inline(always)]
    pub fn lptim3ie(&self) -> LPTIM3IE_R {
        LPTIM3IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - illegal access interrupt enable for LPTIM4"]
    #[inline(always)]
    pub fn lptim4ie(&self) -> LPTIM4IE_R {
        LPTIM4IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - illegal access interrupt enable for OPAMP"]
    #[inline(always)]
    pub fn opampie(&self) -> OPAMPIE_R {
        OPAMPIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - illegal access interrupt enable for COMP"]
    #[inline(always)]
    pub fn compie(&self) -> COMPIE_R {
        COMPIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - illegal access interrupt enable for ADC4"]
    #[inline(always)]
    pub fn adc4ie(&self) -> ADC4IE_R {
        ADC4IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - illegal access interrupt enable for VREFBUF"]
    #[inline(always)]
    pub fn vrefbufie(&self) -> VREFBUFIE_R {
        VREFBUFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - illegal access interrupt enable for DAC1"]
    #[inline(always)]
    pub fn dac1ie(&self) -> DAC1IE_R {
        DAC1IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - illegal access interrupt enable for ADF1"]
    #[inline(always)]
    pub fn adf1ie(&self) -> ADF1IE_R {
        ADF1IE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - illegal access interrupt enable for SPI3"]
    #[inline(always)]
    #[must_use]
    pub fn spi3ie(&mut self) -> SPI3IE_W<IER1rs> {
        SPI3IE_W::new(self, 0)
    }
    #[doc = "Bit 1 - illegal access interrupt enable for LPUART1"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1ie(&mut self) -> LPUART1IE_W<IER1rs> {
        LPUART1IE_W::new(self, 1)
    }
    #[doc = "Bit 2 - illegal access interrupt enable for I2C3"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3ie(&mut self) -> I2C3IE_W<IER1rs> {
        I2C3IE_W::new(self, 2)
    }
    #[doc = "Bit 3 - illegal access interrupt enable for LPTIM1"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1ie(&mut self) -> LPTIM1IE_W<IER1rs> {
        LPTIM1IE_W::new(self, 3)
    }
    #[doc = "Bit 4 - illegal access interrupt enable for LPTIM3"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3ie(&mut self) -> LPTIM3IE_W<IER1rs> {
        LPTIM3IE_W::new(self, 4)
    }
    #[doc = "Bit 5 - illegal access interrupt enable for LPTIM4"]
    #[inline(always)]
    #[must_use]
    pub fn lptim4ie(&mut self) -> LPTIM4IE_W<IER1rs> {
        LPTIM4IE_W::new(self, 5)
    }
    #[doc = "Bit 6 - illegal access interrupt enable for OPAMP"]
    #[inline(always)]
    #[must_use]
    pub fn opampie(&mut self) -> OPAMPIE_W<IER1rs> {
        OPAMPIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - illegal access interrupt enable for COMP"]
    #[inline(always)]
    #[must_use]
    pub fn compie(&mut self) -> COMPIE_W<IER1rs> {
        COMPIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - illegal access interrupt enable for ADC4"]
    #[inline(always)]
    #[must_use]
    pub fn adc4ie(&mut self) -> ADC4IE_W<IER1rs> {
        ADC4IE_W::new(self, 8)
    }
    #[doc = "Bit 9 - illegal access interrupt enable for VREFBUF"]
    #[inline(always)]
    #[must_use]
    pub fn vrefbufie(&mut self) -> VREFBUFIE_W<IER1rs> {
        VREFBUFIE_W::new(self, 9)
    }
    #[doc = "Bit 11 - illegal access interrupt enable for DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn dac1ie(&mut self) -> DAC1IE_W<IER1rs> {
        DAC1IE_W::new(self, 11)
    }
    #[doc = "Bit 12 - illegal access interrupt enable for ADF1"]
    #[inline(always)]
    #[must_use]
    pub fn adf1ie(&mut self) -> ADF1IE_W<IER1rs> {
        ADF1IE_W::new(self, 12)
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
