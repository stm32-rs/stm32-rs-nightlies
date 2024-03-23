#[doc = "Register `TZSC_SECCFGR1` reader"]
pub type R = crate::R<TZSC_SECCFGR1rs>;
#[doc = "Register `TZSC_SECCFGR1` writer"]
pub type W = crate::W<TZSC_SECCFGR1rs>;
#[doc = "Field `SPI3SEC` reader - secure access mode for SPI3"]
pub type SPI3SEC_R = crate::BitReader;
#[doc = "Field `SPI3SEC` writer - secure access mode for SPI3"]
pub type SPI3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUART1SEC` reader - secure access mode for LPUART1"]
pub type LPUART1SEC_R = crate::BitReader;
#[doc = "Field `LPUART1SEC` writer - secure access mode for LPUART1"]
pub type LPUART1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3SEC` reader - secure access mode for I2C3"]
pub type I2C3SEC_R = crate::BitReader;
#[doc = "Field `I2C3SEC` writer - secure access mode for I2C3"]
pub type I2C3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1SEC` reader - secure access mode for LPTIM1"]
pub type LPTIM1SEC_R = crate::BitReader;
#[doc = "Field `LPTIM1SEC` writer - secure access mode for LPTIM1"]
pub type LPTIM1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM3SEC` reader - secure access mode for LPTIM3"]
pub type LPTIM3SEC_R = crate::BitReader;
#[doc = "Field `LPTIM3SEC` writer - secure access mode for LPTIM3"]
pub type LPTIM3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM4SEC` reader - secure access mode for LPTIM4"]
pub type LPTIM4SEC_R = crate::BitReader;
#[doc = "Field `LPTIM4SEC` writer - secure access mode for LPTIM4"]
pub type LPTIM4SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPAMPSEC` reader - secure access mode for OPAMP"]
pub type OPAMPSEC_R = crate::BitReader;
#[doc = "Field `OPAMPSEC` writer - secure access mode for OPAMP"]
pub type OPAMPSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPSEC` reader - secure access mode for COMP"]
pub type COMPSEC_R = crate::BitReader;
#[doc = "Field `COMPSEC` writer - secure access mode for COMP"]
pub type COMPSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFBUFSEC` reader - secure access mode for VREFBUF"]
pub type VREFBUFSEC_R = crate::BitReader;
#[doc = "Field `VREFBUFSEC` writer - secure access mode for VREFBUF"]
pub type VREFBUFSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC1SEC` reader - secure access mode for DAC1"]
pub type DAC1SEC_R = crate::BitReader;
#[doc = "Field `DAC1SEC` writer - secure access mode for DAC1"]
pub type DAC1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADF1SEC` reader - secure access mode for ADF1"]
pub type ADF1SEC_R = crate::BitReader;
#[doc = "Field `ADF1SEC` writer - secure access mode for ADF1"]
pub type ADF1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - secure access mode for SPI3"]
    #[inline(always)]
    pub fn spi3sec(&self) -> SPI3SEC_R {
        SPI3SEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - secure access mode for LPUART1"]
    #[inline(always)]
    pub fn lpuart1sec(&self) -> LPUART1SEC_R {
        LPUART1SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - secure access mode for I2C3"]
    #[inline(always)]
    pub fn i2c3sec(&self) -> I2C3SEC_R {
        I2C3SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - secure access mode for LPTIM1"]
    #[inline(always)]
    pub fn lptim1sec(&self) -> LPTIM1SEC_R {
        LPTIM1SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - secure access mode for LPTIM3"]
    #[inline(always)]
    pub fn lptim3sec(&self) -> LPTIM3SEC_R {
        LPTIM3SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - secure access mode for LPTIM4"]
    #[inline(always)]
    pub fn lptim4sec(&self) -> LPTIM4SEC_R {
        LPTIM4SEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - secure access mode for OPAMP"]
    #[inline(always)]
    pub fn opampsec(&self) -> OPAMPSEC_R {
        OPAMPSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - secure access mode for COMP"]
    #[inline(always)]
    pub fn compsec(&self) -> COMPSEC_R {
        COMPSEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - secure access mode for VREFBUF"]
    #[inline(always)]
    pub fn vrefbufsec(&self) -> VREFBUFSEC_R {
        VREFBUFSEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - secure access mode for DAC1"]
    #[inline(always)]
    pub fn dac1sec(&self) -> DAC1SEC_R {
        DAC1SEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - secure access mode for ADF1"]
    #[inline(always)]
    pub fn adf1sec(&self) -> ADF1SEC_R {
        ADF1SEC_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - secure access mode for SPI3"]
    #[inline(always)]
    #[must_use]
    pub fn spi3sec(&mut self) -> SPI3SEC_W<TZSC_SECCFGR1rs> {
        SPI3SEC_W::new(self, 0)
    }
    #[doc = "Bit 1 - secure access mode for LPUART1"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sec(&mut self) -> LPUART1SEC_W<TZSC_SECCFGR1rs> {
        LPUART1SEC_W::new(self, 1)
    }
    #[doc = "Bit 2 - secure access mode for I2C3"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3sec(&mut self) -> I2C3SEC_W<TZSC_SECCFGR1rs> {
        I2C3SEC_W::new(self, 2)
    }
    #[doc = "Bit 3 - secure access mode for LPTIM1"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1sec(&mut self) -> LPTIM1SEC_W<TZSC_SECCFGR1rs> {
        LPTIM1SEC_W::new(self, 3)
    }
    #[doc = "Bit 4 - secure access mode for LPTIM3"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3sec(&mut self) -> LPTIM3SEC_W<TZSC_SECCFGR1rs> {
        LPTIM3SEC_W::new(self, 4)
    }
    #[doc = "Bit 5 - secure access mode for LPTIM4"]
    #[inline(always)]
    #[must_use]
    pub fn lptim4sec(&mut self) -> LPTIM4SEC_W<TZSC_SECCFGR1rs> {
        LPTIM4SEC_W::new(self, 5)
    }
    #[doc = "Bit 6 - secure access mode for OPAMP"]
    #[inline(always)]
    #[must_use]
    pub fn opampsec(&mut self) -> OPAMPSEC_W<TZSC_SECCFGR1rs> {
        OPAMPSEC_W::new(self, 6)
    }
    #[doc = "Bit 7 - secure access mode for COMP"]
    #[inline(always)]
    #[must_use]
    pub fn compsec(&mut self) -> COMPSEC_W<TZSC_SECCFGR1rs> {
        COMPSEC_W::new(self, 7)
    }
    #[doc = "Bit 9 - secure access mode for VREFBUF"]
    #[inline(always)]
    #[must_use]
    pub fn vrefbufsec(&mut self) -> VREFBUFSEC_W<TZSC_SECCFGR1rs> {
        VREFBUFSEC_W::new(self, 9)
    }
    #[doc = "Bit 11 - secure access mode for DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn dac1sec(&mut self) -> DAC1SEC_W<TZSC_SECCFGR1rs> {
        DAC1SEC_W::new(self, 11)
    }
    #[doc = "Bit 12 - secure access mode for ADF1"]
    #[inline(always)]
    #[must_use]
    pub fn adf1sec(&mut self) -> ADF1SEC_W<TZSC_SECCFGR1rs> {
        ADF1SEC_W::new(self, 12)
    }
}
#[doc = "TZSC secure configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_seccfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_seccfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZSC_SECCFGR1rs;
impl crate::RegisterSpec for TZSC_SECCFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzsc_seccfgr1::R`](R) reader structure"]
impl crate::Readable for TZSC_SECCFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`tzsc_seccfgr1::W`](W) writer structure"]
impl crate::Writable for TZSC_SECCFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZSC_SECCFGR1 to value 0"]
impl crate::Resettable for TZSC_SECCFGR1rs {
    const RESET_VALUE: u32 = 0;
}
