#[doc = "Register `TZSC_SECCFGR3` reader"]
pub type R = crate::R<TZSC_SECCFGR3rs>;
#[doc = "Register `TZSC_SECCFGR3` writer"]
pub type W = crate::W<TZSC_SECCFGR3rs>;
#[doc = "Field `MDF1SEC` reader - secure access mode for MDF1"]
pub type MDF1SEC_R = crate::BitReader;
#[doc = "Field `MDF1SEC` writer - secure access mode for MDF1"]
pub type MDF1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORDICSEC` reader - secure access mode for CORDIC"]
pub type CORDICSEC_R = crate::BitReader;
#[doc = "Field `CORDICSEC` writer - secure access mode for CORDIC"]
pub type CORDICSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMACSEC` reader - secure access mode for FMAC"]
pub type FMACSEC_R = crate::BitReader;
#[doc = "Field `FMACSEC` writer - secure access mode for FMAC"]
pub type FMACSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCSEC` reader - secure access mode for CRC"]
pub type CRCSEC_R = crate::BitReader;
#[doc = "Field `CRCSEC` writer - secure access mode for CRC"]
pub type CRCSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCSEC` reader - secure access mode for TSC"]
pub type TSCSEC_R = crate::BitReader;
#[doc = "Field `TSCSEC` writer - secure access mode for TSC"]
pub type TSCSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE_REGSEC` reader - secure access mode for ICACHE registers"]
pub type ICACHE_REGSEC_R = crate::BitReader;
#[doc = "Field `ICACHE_REGSEC` writer - secure access mode for ICACHE registers"]
pub type ICACHE_REGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE1_REGSEC` reader - secure access mode for DCACHE1 registers"]
pub type DCACHE1_REGSEC_R = crate::BitReader;
#[doc = "Field `DCACHE1_REGSEC` writer - secure access mode for DCACHE1 registers"]
pub type DCACHE1_REGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1SEC` reader - secure access mode for ADC1"]
pub type ADC1SEC_R = crate::BitReader;
#[doc = "Field `ADC1SEC` writer - secure access mode for ADC1"]
pub type ADC1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `SDMMC1SEC` reader - secure access mode"]
pub type SDMMC1SEC_R = crate::BitReader;
#[doc = "Field `SDMMC1SEC` writer - secure access mode"]
pub type SDMMC1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTOSPI1_REGSEC` reader - secure access mode for OCTOSPI1 registers"]
pub type OCTOSPI1_REGSEC_R = crate::BitReader;
#[doc = "Field `OCTOSPI1_REGSEC` writer - secure access mode for OCTOSPI1 registers"]
pub type OCTOSPI1_REGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMCFGSEC` reader - secure access mode for RAMCFG"]
pub type RAMCFGSEC_R = crate::BitReader;
#[doc = "Field `RAMCFGSEC` writer - secure access mode for RAMCFG"]
pub type RAMCFGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPU2DSEC` reader - GPU2DSEC"]
pub type GPU2DSEC_R = crate::BitReader;
#[doc = "Field `GPU2DSEC` writer - GPU2DSEC"]
pub type GPU2DSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSPI1_REGSEC` reader - HSPI1_REGSEC"]
pub type HSPI1_REGSEC_R = crate::BitReader;
#[doc = "Field `HSPI1_REGSEC` writer - HSPI1_REGSEC"]
pub type HSPI1_REGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - secure access mode for MDF1"]
    #[inline(always)]
    pub fn mdf1sec(&self) -> MDF1SEC_R {
        MDF1SEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - secure access mode for CORDIC"]
    #[inline(always)]
    pub fn cordicsec(&self) -> CORDICSEC_R {
        CORDICSEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - secure access mode for FMAC"]
    #[inline(always)]
    pub fn fmacsec(&self) -> FMACSEC_R {
        FMACSEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - secure access mode for CRC"]
    #[inline(always)]
    pub fn crcsec(&self) -> CRCSEC_R {
        CRCSEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - secure access mode for TSC"]
    #[inline(always)]
    pub fn tscsec(&self) -> TSCSEC_R {
        TSCSEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - secure access mode for ICACHE registers"]
    #[inline(always)]
    pub fn icache_regsec(&self) -> ICACHE_REGSEC_R {
        ICACHE_REGSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - secure access mode for DCACHE1 registers"]
    #[inline(always)]
    pub fn dcache1_regsec(&self) -> DCACHE1_REGSEC_R {
        DCACHE1_REGSEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - secure access mode for ADC1"]
    #[inline(always)]
    pub fn adc1sec(&self) -> ADC1SEC_R {
        ADC1SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - secure access mode for DCMI"]
    #[inline(always)]
    pub fn dcmisec(&self) -> DCMISEC_R {
        DCMISEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - secure access mode for HASH"]
    #[inline(always)]
    pub fn hashsec(&self) -> HASHSEC_R {
        HASHSEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - secure access mode for RNG"]
    #[inline(always)]
    pub fn rngsec(&self) -> RNGSEC_R {
        RNGSEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 17 - secure access mode"]
    #[inline(always)]
    pub fn sdmmc1sec(&self) -> SDMMC1SEC_R {
        SDMMC1SEC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - secure access mode for OCTOSPI1 registers"]
    #[inline(always)]
    pub fn octospi1_regsec(&self) -> OCTOSPI1_REGSEC_R {
        OCTOSPI1_REGSEC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - secure access mode for RAMCFG"]
    #[inline(always)]
    pub fn ramcfgsec(&self) -> RAMCFGSEC_R {
        RAMCFGSEC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPU2DSEC"]
    #[inline(always)]
    pub fn gpu2dsec(&self) -> GPU2DSEC_R {
        GPU2DSEC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - HSPI1_REGSEC"]
    #[inline(always)]
    pub fn hspi1_regsec(&self) -> HSPI1_REGSEC_R {
        HSPI1_REGSEC_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - secure access mode for MDF1"]
    #[inline(always)]
    #[must_use]
    pub fn mdf1sec(&mut self) -> MDF1SEC_W<TZSC_SECCFGR3rs> {
        MDF1SEC_W::new(self, 0)
    }
    #[doc = "Bit 1 - secure access mode for CORDIC"]
    #[inline(always)]
    #[must_use]
    pub fn cordicsec(&mut self) -> CORDICSEC_W<TZSC_SECCFGR3rs> {
        CORDICSEC_W::new(self, 1)
    }
    #[doc = "Bit 2 - secure access mode for FMAC"]
    #[inline(always)]
    #[must_use]
    pub fn fmacsec(&mut self) -> FMACSEC_W<TZSC_SECCFGR3rs> {
        FMACSEC_W::new(self, 2)
    }
    #[doc = "Bit 3 - secure access mode for CRC"]
    #[inline(always)]
    #[must_use]
    pub fn crcsec(&mut self) -> CRCSEC_W<TZSC_SECCFGR3rs> {
        CRCSEC_W::new(self, 3)
    }
    #[doc = "Bit 4 - secure access mode for TSC"]
    #[inline(always)]
    #[must_use]
    pub fn tscsec(&mut self) -> TSCSEC_W<TZSC_SECCFGR3rs> {
        TSCSEC_W::new(self, 4)
    }
    #[doc = "Bit 6 - secure access mode for ICACHE registers"]
    #[inline(always)]
    #[must_use]
    pub fn icache_regsec(&mut self) -> ICACHE_REGSEC_W<TZSC_SECCFGR3rs> {
        ICACHE_REGSEC_W::new(self, 6)
    }
    #[doc = "Bit 7 - secure access mode for DCACHE1 registers"]
    #[inline(always)]
    #[must_use]
    pub fn dcache1_regsec(&mut self) -> DCACHE1_REGSEC_W<TZSC_SECCFGR3rs> {
        DCACHE1_REGSEC_W::new(self, 7)
    }
    #[doc = "Bit 8 - secure access mode for ADC1"]
    #[inline(always)]
    #[must_use]
    pub fn adc1sec(&mut self) -> ADC1SEC_W<TZSC_SECCFGR3rs> {
        ADC1SEC_W::new(self, 8)
    }
    #[doc = "Bit 9 - secure access mode for DCMI"]
    #[inline(always)]
    #[must_use]
    pub fn dcmisec(&mut self) -> DCMISEC_W<TZSC_SECCFGR3rs> {
        DCMISEC_W::new(self, 9)
    }
    #[doc = "Bit 12 - secure access mode for HASH"]
    #[inline(always)]
    #[must_use]
    pub fn hashsec(&mut self) -> HASHSEC_W<TZSC_SECCFGR3rs> {
        HASHSEC_W::new(self, 12)
    }
    #[doc = "Bit 13 - secure access mode for RNG"]
    #[inline(always)]
    #[must_use]
    pub fn rngsec(&mut self) -> RNGSEC_W<TZSC_SECCFGR3rs> {
        RNGSEC_W::new(self, 13)
    }
    #[doc = "Bit 17 - secure access mode"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1sec(&mut self) -> SDMMC1SEC_W<TZSC_SECCFGR3rs> {
        SDMMC1SEC_W::new(self, 17)
    }
    #[doc = "Bit 20 - secure access mode for OCTOSPI1 registers"]
    #[inline(always)]
    #[must_use]
    pub fn octospi1_regsec(&mut self) -> OCTOSPI1_REGSEC_W<TZSC_SECCFGR3rs> {
        OCTOSPI1_REGSEC_W::new(self, 20)
    }
    #[doc = "Bit 22 - secure access mode for RAMCFG"]
    #[inline(always)]
    #[must_use]
    pub fn ramcfgsec(&mut self) -> RAMCFGSEC_W<TZSC_SECCFGR3rs> {
        RAMCFGSEC_W::new(self, 22)
    }
    #[doc = "Bit 23 - GPU2DSEC"]
    #[inline(always)]
    #[must_use]
    pub fn gpu2dsec(&mut self) -> GPU2DSEC_W<TZSC_SECCFGR3rs> {
        GPU2DSEC_W::new(self, 23)
    }
    #[doc = "Bit 26 - HSPI1_REGSEC"]
    #[inline(always)]
    #[must_use]
    pub fn hspi1_regsec(&mut self) -> HSPI1_REGSEC_W<TZSC_SECCFGR3rs> {
        HSPI1_REGSEC_W::new(self, 26)
    }
}
#[doc = "TZSC secure configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_seccfgr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_seccfgr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZSC_SECCFGR3rs;
impl crate::RegisterSpec for TZSC_SECCFGR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzsc_seccfgr3::R`](R) reader structure"]
impl crate::Readable for TZSC_SECCFGR3rs {}
#[doc = "`write(|w| ..)` method takes [`tzsc_seccfgr3::W`](W) writer structure"]
impl crate::Writable for TZSC_SECCFGR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZSC_SECCFGR3 to value 0"]
impl crate::Resettable for TZSC_SECCFGR3rs {
    const RESET_VALUE: u32 = 0;
}
