#[doc = "Register `GTZC1_TZIC_FCR3` writer"]
pub type W = crate::W<GTZC1_TZIC_FCR3rs>;
#[doc = "Field `CLPTIM6F` writer - clear illegal access flag for LPTIM6"]
pub type CLPTIM6F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CVREFBUFF` writer - clear illegal access flag for VREFBUF"]
pub type CVREFBUFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCRCF` writer - clear illegal access flag for CRC"]
pub type CCRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCORDICF` writer - clear illegal access flag for CORDIC"]
pub type CCORDICF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFMACF` writer - clear illegal access flag for FMAC"]
pub type CFMACF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CICACHEF` writer - clear illegal access flag for ICACHE"]
pub type CICACHEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDCACHEF` writer - clear illegal access flag for DCACHE"]
pub type CDCACHEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CADC12F` writer - clear illegal access flag for ADC1 and ADC2"]
pub type CADC12F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDCMIF` writer - clear illegal access flag for DCMI"]
pub type CDCMIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHASHF` writer - clear illegal access flag for HASH"]
pub type CHASHF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRNGF` writer - clear illegal access flag for RNG"]
pub type CRNGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSDMMC1F` writer - clear illegal access flag for SDMMC1"]
pub type CSDMMC1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFMCF` writer - clear illegal access flag for FMC"]
pub type CFMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCTOSPI1F` writer - clear illegal access flag for OCTOSPI1"]
pub type COCTOSPI1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRAMCFGF` writer - clear illegal access flag for RAMSCFG"]
pub type CRAMCFGF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - clear illegal access flag for LPTIM6"]
    #[inline(always)]
    #[must_use]
    pub fn clptim6f(&mut self) -> CLPTIM6F_W<GTZC1_TZIC_FCR3rs> {
        CLPTIM6F_W::new(self, 0)
    }
    #[doc = "Bit 1 - clear illegal access flag for VREFBUF"]
    #[inline(always)]
    #[must_use]
    pub fn cvrefbuff(&mut self) -> CVREFBUFF_W<GTZC1_TZIC_FCR3rs> {
        CVREFBUFF_W::new(self, 1)
    }
    #[doc = "Bit 8 - clear illegal access flag for CRC"]
    #[inline(always)]
    #[must_use]
    pub fn ccrcf(&mut self) -> CCRCF_W<GTZC1_TZIC_FCR3rs> {
        CCRCF_W::new(self, 8)
    }
    #[doc = "Bit 9 - clear illegal access flag for CORDIC"]
    #[inline(always)]
    #[must_use]
    pub fn ccordicf(&mut self) -> CCORDICF_W<GTZC1_TZIC_FCR3rs> {
        CCORDICF_W::new(self, 9)
    }
    #[doc = "Bit 10 - clear illegal access flag for FMAC"]
    #[inline(always)]
    #[must_use]
    pub fn cfmacf(&mut self) -> CFMACF_W<GTZC1_TZIC_FCR3rs> {
        CFMACF_W::new(self, 10)
    }
    #[doc = "Bit 12 - clear illegal access flag for ICACHE"]
    #[inline(always)]
    #[must_use]
    pub fn cicachef(&mut self) -> CICACHEF_W<GTZC1_TZIC_FCR3rs> {
        CICACHEF_W::new(self, 12)
    }
    #[doc = "Bit 13 - clear illegal access flag for DCACHE"]
    #[inline(always)]
    #[must_use]
    pub fn cdcachef(&mut self) -> CDCACHEF_W<GTZC1_TZIC_FCR3rs> {
        CDCACHEF_W::new(self, 13)
    }
    #[doc = "Bit 14 - clear illegal access flag for ADC1 and ADC2"]
    #[inline(always)]
    #[must_use]
    pub fn cadc12f(&mut self) -> CADC12F_W<GTZC1_TZIC_FCR3rs> {
        CADC12F_W::new(self, 14)
    }
    #[doc = "Bit 15 - clear illegal access flag for DCMI"]
    #[inline(always)]
    #[must_use]
    pub fn cdcmif(&mut self) -> CDCMIF_W<GTZC1_TZIC_FCR3rs> {
        CDCMIF_W::new(self, 15)
    }
    #[doc = "Bit 17 - clear illegal access flag for HASH"]
    #[inline(always)]
    #[must_use]
    pub fn chashf(&mut self) -> CHASHF_W<GTZC1_TZIC_FCR3rs> {
        CHASHF_W::new(self, 17)
    }
    #[doc = "Bit 18 - clear illegal access flag for RNG"]
    #[inline(always)]
    #[must_use]
    pub fn crngf(&mut self) -> CRNGF_W<GTZC1_TZIC_FCR3rs> {
        CRNGF_W::new(self, 18)
    }
    #[doc = "Bit 22 - clear illegal access flag for SDMMC1"]
    #[inline(always)]
    #[must_use]
    pub fn csdmmc1f(&mut self) -> CSDMMC1F_W<GTZC1_TZIC_FCR3rs> {
        CSDMMC1F_W::new(self, 22)
    }
    #[doc = "Bit 23 - clear illegal access flag for FMC"]
    #[inline(always)]
    #[must_use]
    pub fn cfmcf(&mut self) -> CFMCF_W<GTZC1_TZIC_FCR3rs> {
        CFMCF_W::new(self, 23)
    }
    #[doc = "Bit 24 - clear illegal access flag for OCTOSPI1"]
    #[inline(always)]
    #[must_use]
    pub fn coctospi1f(&mut self) -> COCTOSPI1F_W<GTZC1_TZIC_FCR3rs> {
        COCTOSPI1F_W::new(self, 24)
    }
    #[doc = "Bit 26 - clear illegal access flag for RAMSCFG"]
    #[inline(always)]
    #[must_use]
    pub fn cramcfgf(&mut self) -> CRAMCFGF_W<GTZC1_TZIC_FCR3rs> {
        CRAMCFGF_W::new(self, 26)
    }
}
#[doc = "TZIC flag clear register 3\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzic_fcr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTZC1_TZIC_FCR3rs;
impl crate::RegisterSpec for GTZC1_TZIC_FCR3rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gtzc1_tzic_fcr3::W`](W) writer structure"]
impl crate::Writable for GTZC1_TZIC_FCR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTZC1_TZIC_FCR3 to value 0"]
impl crate::Resettable for GTZC1_TZIC_FCR3rs {
    const RESET_VALUE: u32 = 0;
}
