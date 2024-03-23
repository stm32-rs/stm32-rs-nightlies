#[doc = "Register `SECCFGR3` reader"]
pub type R = crate::R<SECCFGR3rs>;
#[doc = "Register `SECCFGR3` writer"]
pub type W = crate::W<SECCFGR3rs>;
#[doc = "Field `LPTIM6SEC` reader - secure access mode for LPTIM6"]
pub type LPTIM6SEC_R = crate::BitReader;
#[doc = "Field `LPTIM6SEC` writer - secure access mode for LPTIM6"]
pub type LPTIM6SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFBUFSEC` reader - secure access mode for VREFBUF"]
pub type VREFBUFSEC_R = crate::BitReader;
#[doc = "Field `VREFBUFSEC` writer - secure access mode for VREFBUF"]
pub type VREFBUFSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCSEC` reader - secure access mode for CRC"]
pub type CRCSEC_R = crate::BitReader;
#[doc = "Field `CRCSEC` writer - secure access mode for CRC"]
pub type CRCSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORDICSEC` reader - secure access mode for CORDIC"]
pub type CORDICSEC_R = crate::BitReader;
#[doc = "Field `CORDICSEC` writer - secure access mode for CORDIC"]
pub type CORDICSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMACSEC` reader - secure access mode for FMAC"]
pub type FMACSEC_R = crate::BitReader;
#[doc = "Field `FMACSEC` writer - secure access mode for FMAC"]
pub type FMACSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETHSEC` reader - secure access mode for register of ETH"]
pub type ETHSEC_R = crate::BitReader;
#[doc = "Field `ETHSEC` writer - secure access mode for register of ETH"]
pub type ETHSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHESEC` reader - secure access mode for ICACHE"]
pub type ICACHESEC_R = crate::BitReader;
#[doc = "Field `ICACHESEC` writer - secure access mode for ICACHE"]
pub type ICACHESEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHESEC` reader - secure access mode for DCACHE"]
pub type DCACHESEC_R = crate::BitReader;
#[doc = "Field `DCACHESEC` writer - secure access mode for DCACHE"]
pub type DCACHESEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12SEC` reader - secure access mode for ADC1 and ADC2"]
pub type ADC12SEC_R = crate::BitReader;
#[doc = "Field `ADC12SEC` writer - secure access mode for ADC1 and ADC2"]
pub type ADC12SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCMISEC` reader - secure access mode for DCMI"]
pub type DCMISEC_R = crate::BitReader;
#[doc = "Field `DCMISEC` writer - secure access mode for DCMI"]
pub type DCMISEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASHSEC` reader - secure access mode for HASH"]
pub type HASHSEC_R = crate::BitReader;
#[doc = "Field `HASHSEC` writer - secure access mode for HASH"]
pub type HASHSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGSEC` reader - secure access mode for RNG"]
pub type RNGSEC_R = crate::BitReader;
#[doc = "Field `RNGSEC` writer - secure access mode for RNG"]
pub type RNGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC2SEC` reader - secure access mode for SDMMC2"]
pub type SDMMC2SEC_R = crate::BitReader;
#[doc = "Field `SDMMC2SEC` writer - secure access mode for SDMMC2"]
pub type SDMMC2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC1SEC` reader - secure access mode for SDMMC1"]
pub type SDMMC1SEC_R = crate::BitReader;
#[doc = "Field `SDMMC1SEC` writer - secure access mode for SDMMC1"]
pub type SDMMC1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMCSEC` reader - secure access mode for FMC"]
pub type FMCSEC_R = crate::BitReader;
#[doc = "Field `FMCSEC` writer - secure access mode for FMC"]
pub type FMCSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTOSPI1SEC` reader - secure access mode for OCTOSPI1"]
pub type OCTOSPI1SEC_R = crate::BitReader;
#[doc = "Field `OCTOSPI1SEC` writer - secure access mode for OCTOSPI1"]
pub type OCTOSPI1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMCFGSEC` reader - secure access mode for RAMSCFG"]
pub type RAMCFGSEC_R = crate::BitReader;
#[doc = "Field `RAMCFGSEC` writer - secure access mode for RAMSCFG"]
pub type RAMCFGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - secure access mode for LPTIM6"]
    #[inline(always)]
    pub fn lptim6sec(&self) -> LPTIM6SEC_R {
        LPTIM6SEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - secure access mode for VREFBUF"]
    #[inline(always)]
    pub fn vrefbufsec(&self) -> VREFBUFSEC_R {
        VREFBUFSEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - secure access mode for CRC"]
    #[inline(always)]
    pub fn crcsec(&self) -> CRCSEC_R {
        CRCSEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - secure access mode for CORDIC"]
    #[inline(always)]
    pub fn cordicsec(&self) -> CORDICSEC_R {
        CORDICSEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - secure access mode for FMAC"]
    #[inline(always)]
    pub fn fmacsec(&self) -> FMACSEC_R {
        FMACSEC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - secure access mode for register of ETH"]
    #[inline(always)]
    pub fn ethsec(&self) -> ETHSEC_R {
        ETHSEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - secure access mode for ICACHE"]
    #[inline(always)]
    pub fn icachesec(&self) -> ICACHESEC_R {
        ICACHESEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - secure access mode for DCACHE"]
    #[inline(always)]
    pub fn dcachesec(&self) -> DCACHESEC_R {
        DCACHESEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - secure access mode for ADC1 and ADC2"]
    #[inline(always)]
    pub fn adc12sec(&self) -> ADC12SEC_R {
        ADC12SEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - secure access mode for DCMI"]
    #[inline(always)]
    pub fn dcmisec(&self) -> DCMISEC_R {
        DCMISEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - secure access mode for HASH"]
    #[inline(always)]
    pub fn hashsec(&self) -> HASHSEC_R {
        HASHSEC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - secure access mode for RNG"]
    #[inline(always)]
    pub fn rngsec(&self) -> RNGSEC_R {
        RNGSEC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - secure access mode for SDMMC2"]
    #[inline(always)]
    pub fn sdmmc2sec(&self) -> SDMMC2SEC_R {
        SDMMC2SEC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - secure access mode for SDMMC1"]
    #[inline(always)]
    pub fn sdmmc1sec(&self) -> SDMMC1SEC_R {
        SDMMC1SEC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - secure access mode for FMC"]
    #[inline(always)]
    pub fn fmcsec(&self) -> FMCSEC_R {
        FMCSEC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - secure access mode for OCTOSPI1"]
    #[inline(always)]
    pub fn octospi1sec(&self) -> OCTOSPI1SEC_R {
        OCTOSPI1SEC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - secure access mode for RAMSCFG"]
    #[inline(always)]
    pub fn ramcfgsec(&self) -> RAMCFGSEC_R {
        RAMCFGSEC_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - secure access mode for LPTIM6"]
    #[inline(always)]
    #[must_use]
    pub fn lptim6sec(&mut self) -> LPTIM6SEC_W<SECCFGR3rs> {
        LPTIM6SEC_W::new(self, 0)
    }
    #[doc = "Bit 1 - secure access mode for VREFBUF"]
    #[inline(always)]
    #[must_use]
    pub fn vrefbufsec(&mut self) -> VREFBUFSEC_W<SECCFGR3rs> {
        VREFBUFSEC_W::new(self, 1)
    }
    #[doc = "Bit 8 - secure access mode for CRC"]
    #[inline(always)]
    #[must_use]
    pub fn crcsec(&mut self) -> CRCSEC_W<SECCFGR3rs> {
        CRCSEC_W::new(self, 8)
    }
    #[doc = "Bit 9 - secure access mode for CORDIC"]
    #[inline(always)]
    #[must_use]
    pub fn cordicsec(&mut self) -> CORDICSEC_W<SECCFGR3rs> {
        CORDICSEC_W::new(self, 9)
    }
    #[doc = "Bit 10 - secure access mode for FMAC"]
    #[inline(always)]
    #[must_use]
    pub fn fmacsec(&mut self) -> FMACSEC_W<SECCFGR3rs> {
        FMACSEC_W::new(self, 10)
    }
    #[doc = "Bit 11 - secure access mode for register of ETH"]
    #[inline(always)]
    #[must_use]
    pub fn ethsec(&mut self) -> ETHSEC_W<SECCFGR3rs> {
        ETHSEC_W::new(self, 11)
    }
    #[doc = "Bit 12 - secure access mode for ICACHE"]
    #[inline(always)]
    #[must_use]
    pub fn icachesec(&mut self) -> ICACHESEC_W<SECCFGR3rs> {
        ICACHESEC_W::new(self, 12)
    }
    #[doc = "Bit 13 - secure access mode for DCACHE"]
    #[inline(always)]
    #[must_use]
    pub fn dcachesec(&mut self) -> DCACHESEC_W<SECCFGR3rs> {
        DCACHESEC_W::new(self, 13)
    }
    #[doc = "Bit 14 - secure access mode for ADC1 and ADC2"]
    #[inline(always)]
    #[must_use]
    pub fn adc12sec(&mut self) -> ADC12SEC_W<SECCFGR3rs> {
        ADC12SEC_W::new(self, 14)
    }
    #[doc = "Bit 15 - secure access mode for DCMI"]
    #[inline(always)]
    #[must_use]
    pub fn dcmisec(&mut self) -> DCMISEC_W<SECCFGR3rs> {
        DCMISEC_W::new(self, 15)
    }
    #[doc = "Bit 17 - secure access mode for HASH"]
    #[inline(always)]
    #[must_use]
    pub fn hashsec(&mut self) -> HASHSEC_W<SECCFGR3rs> {
        HASHSEC_W::new(self, 17)
    }
    #[doc = "Bit 18 - secure access mode for RNG"]
    #[inline(always)]
    #[must_use]
    pub fn rngsec(&mut self) -> RNGSEC_W<SECCFGR3rs> {
        RNGSEC_W::new(self, 18)
    }
    #[doc = "Bit 21 - secure access mode for SDMMC2"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2sec(&mut self) -> SDMMC2SEC_W<SECCFGR3rs> {
        SDMMC2SEC_W::new(self, 21)
    }
    #[doc = "Bit 22 - secure access mode for SDMMC1"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1sec(&mut self) -> SDMMC1SEC_W<SECCFGR3rs> {
        SDMMC1SEC_W::new(self, 22)
    }
    #[doc = "Bit 23 - secure access mode for FMC"]
    #[inline(always)]
    #[must_use]
    pub fn fmcsec(&mut self) -> FMCSEC_W<SECCFGR3rs> {
        FMCSEC_W::new(self, 23)
    }
    #[doc = "Bit 24 - secure access mode for OCTOSPI1"]
    #[inline(always)]
    #[must_use]
    pub fn octospi1sec(&mut self) -> OCTOSPI1SEC_W<SECCFGR3rs> {
        OCTOSPI1SEC_W::new(self, 24)
    }
    #[doc = "Bit 26 - secure access mode for RAMSCFG"]
    #[inline(always)]
    #[must_use]
    pub fn ramcfgsec(&mut self) -> RAMCFGSEC_W<SECCFGR3rs> {
        RAMCFGSEC_W::new(self, 26)
    }
}
#[doc = "GTZC1 TZSC secure configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccfgr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccfgr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECCFGR3rs;
impl crate::RegisterSpec for SECCFGR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seccfgr3::R`](R) reader structure"]
impl crate::Readable for SECCFGR3rs {}
#[doc = "`write(|w| ..)` method takes [`seccfgr3::W`](W) writer structure"]
impl crate::Writable for SECCFGR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECCFGR3 to value 0"]
impl crate::Resettable for SECCFGR3rs {
    const RESET_VALUE: u32 = 0;
}
