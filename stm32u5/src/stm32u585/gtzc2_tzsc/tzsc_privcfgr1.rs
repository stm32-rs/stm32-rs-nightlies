#[doc = "Register `TZSC_PRIVCFGR1` reader"]
pub type R = crate::R<TZSC_PRIVCFGR1rs>;
#[doc = "Register `TZSC_PRIVCFGR1` writer"]
pub type W = crate::W<TZSC_PRIVCFGR1rs>;
#[doc = "Field `SPI3PRIV` reader - privileged access mode for SPI3"]
pub type SPI3PRIV_R = crate::BitReader;
#[doc = "Field `SPI3PRIV` writer - privileged access mode for SPI3"]
pub type SPI3PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUART1PRIV` reader - privileged access mode for LPUART1"]
pub type LPUART1PRIV_R = crate::BitReader;
#[doc = "Field `LPUART1PRIV` writer - privileged access mode for LPUART1"]
pub type LPUART1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3PRIV` reader - privileged access mode for I2C3"]
pub type I2C3PRIV_R = crate::BitReader;
#[doc = "Field `I2C3PRIV` writer - privileged access mode for I2C3"]
pub type I2C3PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1PRIV` reader - privileged access mode for LPTIM1"]
pub type LPTIM1PRIV_R = crate::BitReader;
#[doc = "Field `LPTIM1PRIV` writer - privileged access mode for LPTIM1"]
pub type LPTIM1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM3PRIV` reader - privileged access mode for LPTIM3"]
pub type LPTIM3PRIV_R = crate::BitReader;
#[doc = "Field `LPTIM3PRIV` writer - privileged access mode for LPTIM3"]
pub type LPTIM3PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM4PRIV` reader - privileged access mode for LPTIM4"]
pub type LPTIM4PRIV_R = crate::BitReader;
#[doc = "Field `LPTIM4PRIV` writer - privileged access mode for LPTIM4"]
pub type LPTIM4PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPAMPPRIV` reader - privileged access mode for OPAMP"]
pub type OPAMPPRIV_R = crate::BitReader;
#[doc = "Field `OPAMPPRIV` writer - privileged access mode for OPAMP"]
pub type OPAMPPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPPRIV` reader - privileged access mode for COMP"]
pub type COMPPRIV_R = crate::BitReader;
#[doc = "Field `COMPPRIV` writer - privileged access mode for COMP"]
pub type COMPPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC4PRIV` reader - privileged access mode for ADC4"]
pub type ADC4PRIV_R = crate::BitReader;
#[doc = "Field `ADC4PRIV` writer - privileged access mode for ADC4"]
pub type ADC4PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFBUFPRIV` reader - privileged access mode for VREFBUF"]
pub type VREFBUFPRIV_R = crate::BitReader;
#[doc = "Field `VREFBUFPRIV` writer - privileged access mode for VREFBUF"]
pub type VREFBUFPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC1PRIV` reader - privileged access mode for DAC1"]
pub type DAC1PRIV_R = crate::BitReader;
#[doc = "Field `DAC1PRIV` writer - privileged access mode for DAC1"]
pub type DAC1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADF1PRIV` reader - privileged access mode for ADF1"]
pub type ADF1PRIV_R = crate::BitReader;
#[doc = "Field `ADF1PRIV` writer - privileged access mode for ADF1"]
pub type ADF1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - privileged access mode for SPI3"]
    #[inline(always)]
    pub fn spi3priv(&self) -> SPI3PRIV_R {
        SPI3PRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - privileged access mode for LPUART1"]
    #[inline(always)]
    pub fn lpuart1priv(&self) -> LPUART1PRIV_R {
        LPUART1PRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - privileged access mode for I2C3"]
    #[inline(always)]
    pub fn i2c3priv(&self) -> I2C3PRIV_R {
        I2C3PRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - privileged access mode for LPTIM1"]
    #[inline(always)]
    pub fn lptim1priv(&self) -> LPTIM1PRIV_R {
        LPTIM1PRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - privileged access mode for LPTIM3"]
    #[inline(always)]
    pub fn lptim3priv(&self) -> LPTIM3PRIV_R {
        LPTIM3PRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - privileged access mode for LPTIM4"]
    #[inline(always)]
    pub fn lptim4priv(&self) -> LPTIM4PRIV_R {
        LPTIM4PRIV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - privileged access mode for OPAMP"]
    #[inline(always)]
    pub fn opamppriv(&self) -> OPAMPPRIV_R {
        OPAMPPRIV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - privileged access mode for COMP"]
    #[inline(always)]
    pub fn comppriv(&self) -> COMPPRIV_R {
        COMPPRIV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - privileged access mode for ADC4"]
    #[inline(always)]
    pub fn adc4priv(&self) -> ADC4PRIV_R {
        ADC4PRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - privileged access mode for VREFBUF"]
    #[inline(always)]
    pub fn vrefbufpriv(&self) -> VREFBUFPRIV_R {
        VREFBUFPRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - privileged access mode for DAC1"]
    #[inline(always)]
    pub fn dac1priv(&self) -> DAC1PRIV_R {
        DAC1PRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - privileged access mode for ADF1"]
    #[inline(always)]
    pub fn adf1priv(&self) -> ADF1PRIV_R {
        ADF1PRIV_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - privileged access mode for SPI3"]
    #[inline(always)]
    #[must_use]
    pub fn spi3priv(&mut self) -> SPI3PRIV_W<TZSC_PRIVCFGR1rs> {
        SPI3PRIV_W::new(self, 0)
    }
    #[doc = "Bit 1 - privileged access mode for LPUART1"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1priv(&mut self) -> LPUART1PRIV_W<TZSC_PRIVCFGR1rs> {
        LPUART1PRIV_W::new(self, 1)
    }
    #[doc = "Bit 2 - privileged access mode for I2C3"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3priv(&mut self) -> I2C3PRIV_W<TZSC_PRIVCFGR1rs> {
        I2C3PRIV_W::new(self, 2)
    }
    #[doc = "Bit 3 - privileged access mode for LPTIM1"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1priv(&mut self) -> LPTIM1PRIV_W<TZSC_PRIVCFGR1rs> {
        LPTIM1PRIV_W::new(self, 3)
    }
    #[doc = "Bit 4 - privileged access mode for LPTIM3"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3priv(&mut self) -> LPTIM3PRIV_W<TZSC_PRIVCFGR1rs> {
        LPTIM3PRIV_W::new(self, 4)
    }
    #[doc = "Bit 5 - privileged access mode for LPTIM4"]
    #[inline(always)]
    #[must_use]
    pub fn lptim4priv(&mut self) -> LPTIM4PRIV_W<TZSC_PRIVCFGR1rs> {
        LPTIM4PRIV_W::new(self, 5)
    }
    #[doc = "Bit 6 - privileged access mode for OPAMP"]
    #[inline(always)]
    #[must_use]
    pub fn opamppriv(&mut self) -> OPAMPPRIV_W<TZSC_PRIVCFGR1rs> {
        OPAMPPRIV_W::new(self, 6)
    }
    #[doc = "Bit 7 - privileged access mode for COMP"]
    #[inline(always)]
    #[must_use]
    pub fn comppriv(&mut self) -> COMPPRIV_W<TZSC_PRIVCFGR1rs> {
        COMPPRIV_W::new(self, 7)
    }
    #[doc = "Bit 8 - privileged access mode for ADC4"]
    #[inline(always)]
    #[must_use]
    pub fn adc4priv(&mut self) -> ADC4PRIV_W<TZSC_PRIVCFGR1rs> {
        ADC4PRIV_W::new(self, 8)
    }
    #[doc = "Bit 9 - privileged access mode for VREFBUF"]
    #[inline(always)]
    #[must_use]
    pub fn vrefbufpriv(&mut self) -> VREFBUFPRIV_W<TZSC_PRIVCFGR1rs> {
        VREFBUFPRIV_W::new(self, 9)
    }
    #[doc = "Bit 11 - privileged access mode for DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn dac1priv(&mut self) -> DAC1PRIV_W<TZSC_PRIVCFGR1rs> {
        DAC1PRIV_W::new(self, 11)
    }
    #[doc = "Bit 12 - privileged access mode for ADF1"]
    #[inline(always)]
    #[must_use]
    pub fn adf1priv(&mut self) -> ADF1PRIV_W<TZSC_PRIVCFGR1rs> {
        ADF1PRIV_W::new(self, 12)
    }
}
#[doc = "TZSC privilege configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_privcfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_privcfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZSC_PRIVCFGR1rs;
impl crate::RegisterSpec for TZSC_PRIVCFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzsc_privcfgr1::R`](R) reader structure"]
impl crate::Readable for TZSC_PRIVCFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`tzsc_privcfgr1::W`](W) writer structure"]
impl crate::Writable for TZSC_PRIVCFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZSC_PRIVCFGR1 to value 0"]
impl crate::Resettable for TZSC_PRIVCFGR1rs {
    const RESET_VALUE: u32 = 0;
}
