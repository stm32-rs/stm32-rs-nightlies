#[doc = "Register `FCR3` writer"]
pub type W = crate::W<FCR3rs>;
#[doc = "Field `CMDF1F` writer - clear the illegal access flag for MDF1"]
pub type CMDF1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCORDICF` writer - clear the illegal access flag for CORDIC"]
pub type CCORDICF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFMACF` writer - clear the illegal access flag for FMAC"]
pub type CFMACF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCRCF` writer - clear the illegal access flag for CRC"]
pub type CCRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSCF` writer - clear the illegal access flag for TSC"]
pub type CTSCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CICACHE_REGF` writer - clear the illegal access flag for ICACHE registers"]
pub type CICACHE_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDCACHE1_REGF` writer - clear the illegal access flag for DCACHE1 registers"]
pub type CDCACHE1_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CADC12F` writer - clear the illegal access flag for ADC1 and ADC2"]
pub type CADC12F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDCMIF` writer - clear the illegal access flag for DCMI"]
pub type CDCMIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAESF` writer - clear the illegal access flag for AES"]
pub type CAESF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHASHF` writer - clear the illegal access flag for HASH"]
pub type CHASHF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRNGF` writer - clear the illegal access flag for RNG"]
pub type CRNGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPKAF` writer - clear the illegal access flag for PKA"]
pub type CPKAF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSAESF` writer - clear the illegal access flag for SAES"]
pub type CSAESF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSDMMC1F` writer - clear the illegal access flag"]
pub type CSDMMC1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCTOSPI1_REGF` writer - clear the illegal access flag for OCTOSPI1 registers"]
pub type COCTOSPI1_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRAMCFGF` writer - clear the illegal access flag for RAMCFG"]
pub type CRAMCFGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGPU2DF` writer - clear the illegal access flag for GPU2D"]
pub type CGPU2DF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - clear the illegal access flag for MDF1"]
    #[inline(always)]
    #[must_use]
    pub fn cmdf1f(&mut self) -> CMDF1F_W<FCR3rs> {
        CMDF1F_W::new(self, 0)
    }
    #[doc = "Bit 1 - clear the illegal access flag for CORDIC"]
    #[inline(always)]
    #[must_use]
    pub fn ccordicf(&mut self) -> CCORDICF_W<FCR3rs> {
        CCORDICF_W::new(self, 1)
    }
    #[doc = "Bit 2 - clear the illegal access flag for FMAC"]
    #[inline(always)]
    #[must_use]
    pub fn cfmacf(&mut self) -> CFMACF_W<FCR3rs> {
        CFMACF_W::new(self, 2)
    }
    #[doc = "Bit 3 - clear the illegal access flag for CRC"]
    #[inline(always)]
    #[must_use]
    pub fn ccrcf(&mut self) -> CCRCF_W<FCR3rs> {
        CCRCF_W::new(self, 3)
    }
    #[doc = "Bit 4 - clear the illegal access flag for TSC"]
    #[inline(always)]
    #[must_use]
    pub fn ctscf(&mut self) -> CTSCF_W<FCR3rs> {
        CTSCF_W::new(self, 4)
    }
    #[doc = "Bit 6 - clear the illegal access flag for ICACHE registers"]
    #[inline(always)]
    #[must_use]
    pub fn cicache_regf(&mut self) -> CICACHE_REGF_W<FCR3rs> {
        CICACHE_REGF_W::new(self, 6)
    }
    #[doc = "Bit 7 - clear the illegal access flag for DCACHE1 registers"]
    #[inline(always)]
    #[must_use]
    pub fn cdcache1_regf(&mut self) -> CDCACHE1_REGF_W<FCR3rs> {
        CDCACHE1_REGF_W::new(self, 7)
    }
    #[doc = "Bit 8 - clear the illegal access flag for ADC1 and ADC2"]
    #[inline(always)]
    #[must_use]
    pub fn cadc12f(&mut self) -> CADC12F_W<FCR3rs> {
        CADC12F_W::new(self, 8)
    }
    #[doc = "Bit 9 - clear the illegal access flag for DCMI"]
    #[inline(always)]
    #[must_use]
    pub fn cdcmif(&mut self) -> CDCMIF_W<FCR3rs> {
        CDCMIF_W::new(self, 9)
    }
    #[doc = "Bit 11 - clear the illegal access flag for AES"]
    #[inline(always)]
    #[must_use]
    pub fn caesf(&mut self) -> CAESF_W<FCR3rs> {
        CAESF_W::new(self, 11)
    }
    #[doc = "Bit 12 - clear the illegal access flag for HASH"]
    #[inline(always)]
    #[must_use]
    pub fn chashf(&mut self) -> CHASHF_W<FCR3rs> {
        CHASHF_W::new(self, 12)
    }
    #[doc = "Bit 13 - clear the illegal access flag for RNG"]
    #[inline(always)]
    #[must_use]
    pub fn crngf(&mut self) -> CRNGF_W<FCR3rs> {
        CRNGF_W::new(self, 13)
    }
    #[doc = "Bit 14 - clear the illegal access flag for PKA"]
    #[inline(always)]
    #[must_use]
    pub fn cpkaf(&mut self) -> CPKAF_W<FCR3rs> {
        CPKAF_W::new(self, 14)
    }
    #[doc = "Bit 15 - clear the illegal access flag for SAES"]
    #[inline(always)]
    #[must_use]
    pub fn csaesf(&mut self) -> CSAESF_W<FCR3rs> {
        CSAESF_W::new(self, 15)
    }
    #[doc = "Bit 17 - clear the illegal access flag"]
    #[inline(always)]
    #[must_use]
    pub fn csdmmc1f(&mut self) -> CSDMMC1F_W<FCR3rs> {
        CSDMMC1F_W::new(self, 17)
    }
    #[doc = "Bit 20 - clear the illegal access flag for OCTOSPI1 registers"]
    #[inline(always)]
    #[must_use]
    pub fn coctospi1_regf(&mut self) -> COCTOSPI1_REGF_W<FCR3rs> {
        COCTOSPI1_REGF_W::new(self, 20)
    }
    #[doc = "Bit 22 - clear the illegal access flag for RAMCFG"]
    #[inline(always)]
    #[must_use]
    pub fn cramcfgf(&mut self) -> CRAMCFGF_W<FCR3rs> {
        CRAMCFGF_W::new(self, 22)
    }
    #[doc = "Bit 23 - clear the illegal access flag for GPU2D"]
    #[inline(always)]
    #[must_use]
    pub fn cgpu2df(&mut self) -> CGPU2DF_W<FCR3rs> {
        CGPU2DF_W::new(self, 23)
    }
}
#[doc = "TZIC flag clear register 3\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCR3rs;
impl crate::RegisterSpec for FCR3rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fcr3::W`](W) writer structure"]
impl crate::Writable for FCR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCR3 to value 0"]
impl crate::Resettable for FCR3rs {
    const RESET_VALUE: u32 = 0;
}
