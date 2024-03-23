#[doc = "Register `TZSC_PRIVCFGR3` reader"]
pub type R = crate::R<TZSC_PRIVCFGR3rs>;
#[doc = "Register `TZSC_PRIVCFGR3` writer"]
pub type W = crate::W<TZSC_PRIVCFGR3rs>;
#[doc = "Field `MDF1PRIV` reader - privileged access mode for MDF1"]
pub type MDF1PRIV_R = crate::BitReader;
#[doc = "Field `MDF1PRIV` writer - privileged access mode for MDF1"]
pub type MDF1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORDICPRIV` reader - privileged access mode for CORDIC"]
pub type CORDICPRIV_R = crate::BitReader;
#[doc = "Field `CORDICPRIV` writer - privileged access mode for CORDIC"]
pub type CORDICPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMACPRIV` reader - privileged access mode for FMAC"]
pub type FMACPRIV_R = crate::BitReader;
#[doc = "Field `FMACPRIV` writer - privileged access mode for FMAC"]
pub type FMACPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCPRIV` reader - privileged access mode for CRC"]
pub type CRCPRIV_R = crate::BitReader;
#[doc = "Field `CRCPRIV` writer - privileged access mode for CRC"]
pub type CRCPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCPRIV` reader - privileged access mode for TSC"]
pub type TSCPRIV_R = crate::BitReader;
#[doc = "Field `TSCPRIV` writer - privileged access mode for TSC"]
pub type TSCPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2DPRIV` reader - privileged access mode for register of DMA2D"]
pub type DMA2DPRIV_R = crate::BitReader;
#[doc = "Field `DMA2DPRIV` writer - privileged access mode for register of DMA2D"]
pub type DMA2DPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE_REGPRIV` reader - privileged access mode for ICACHE registers"]
pub type ICACHE_REGPRIV_R = crate::BitReader;
#[doc = "Field `ICACHE_REGPRIV` writer - privileged access mode for ICACHE registers"]
pub type ICACHE_REGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_REGPRIV` reader - privileged access mode for DCACHE registers"]
pub type DCACHE_REGPRIV_R = crate::BitReader;
#[doc = "Field `DCACHE_REGPRIV` writer - privileged access mode for DCACHE registers"]
pub type DCACHE_REGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1PRIV` reader - privileged access mode for ADC1"]
pub type ADC1PRIV_R = crate::BitReader;
#[doc = "Field `ADC1PRIV` writer - privileged access mode for ADC1"]
pub type ADC1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCMIPRIV` reader - privileged access mode for DCMI"]
pub type DCMIPRIV_R = crate::BitReader;
#[doc = "Field `DCMIPRIV` writer - privileged access mode for DCMI"]
pub type DCMIPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGFSPRIV` reader - privileged access mode for OTG_FS"]
pub type OTGFSPRIV_R = crate::BitReader;
#[doc = "Field `OTGFSPRIV` writer - privileged access mode for OTG_FS"]
pub type OTGFSPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESPRIV` reader - privileged access mode for AES"]
pub type AESPRIV_R = crate::BitReader;
#[doc = "Field `AESPRIV` writer - privileged access mode for AES"]
pub type AESPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASHPRIV` reader - privileged access mode for HASH"]
pub type HASHPRIV_R = crate::BitReader;
#[doc = "Field `HASHPRIV` writer - privileged access mode for HASH"]
pub type HASHPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGPRIV` reader - privileged access mode for RNG"]
pub type RNGPRIV_R = crate::BitReader;
#[doc = "Field `RNGPRIV` writer - privileged access mode for RNG"]
pub type RNGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKAPRIV` reader - privileged access mode for PKA"]
pub type PKAPRIV_R = crate::BitReader;
#[doc = "Field `PKAPRIV` writer - privileged access mode for PKA"]
pub type PKAPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAESPRIV` reader - privileged access mode for SAES"]
pub type SAESPRIV_R = crate::BitReader;
#[doc = "Field `SAESPRIV` writer - privileged access mode for SAES"]
pub type SAESPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTOSPIMPRIV` reader - privileged access mode for OCTOSPIM"]
pub type OCTOSPIMPRIV_R = crate::BitReader;
#[doc = "Field `OCTOSPIMPRIV` writer - privileged access mode for OCTOSPIM"]
pub type OCTOSPIMPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC1PRIV` reader - privileged access mode for SDMMC2"]
pub type SDMMC1PRIV_R = crate::BitReader;
#[doc = "Field `SDMMC1PRIV` writer - privileged access mode for SDMMC2"]
pub type SDMMC1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC2PRIV` reader - privileged access mode for SDMMC1"]
pub type SDMMC2PRIV_R = crate::BitReader;
#[doc = "Field `SDMMC2PRIV` writer - privileged access mode for SDMMC1"]
pub type SDMMC2PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSMC_REGPRIV` reader - privileged access mode for FSMC registers"]
pub type FSMC_REGPRIV_R = crate::BitReader;
#[doc = "Field `FSMC_REGPRIV` writer - privileged access mode for FSMC registers"]
pub type FSMC_REGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTOSPI1_REGPRIV` reader - privileged access mode for OCTOSPI1"]
pub type OCTOSPI1_REGPRIV_R = crate::BitReader;
#[doc = "Field `OCTOSPI1_REGPRIV` writer - privileged access mode for OCTOSPI1"]
pub type OCTOSPI1_REGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTOSPI2_REGPRIV` reader - privileged access mode for OCTOSPI2"]
pub type OCTOSPI2_REGPRIV_R = crate::BitReader;
#[doc = "Field `OCTOSPI2_REGPRIV` writer - privileged access mode for OCTOSPI2"]
pub type OCTOSPI2_REGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMCFGPRIV` reader - privileged access mode for RAMCFG"]
pub type RAMCFGPRIV_R = crate::BitReader;
#[doc = "Field `RAMCFGPRIV` writer - privileged access mode for RAMCFG"]
pub type RAMCFGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - privileged access mode for MDF1"]
    #[inline(always)]
    pub fn mdf1priv(&self) -> MDF1PRIV_R {
        MDF1PRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - privileged access mode for CORDIC"]
    #[inline(always)]
    pub fn cordicpriv(&self) -> CORDICPRIV_R {
        CORDICPRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - privileged access mode for FMAC"]
    #[inline(always)]
    pub fn fmacpriv(&self) -> FMACPRIV_R {
        FMACPRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - privileged access mode for CRC"]
    #[inline(always)]
    pub fn crcpriv(&self) -> CRCPRIV_R {
        CRCPRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - privileged access mode for TSC"]
    #[inline(always)]
    pub fn tscpriv(&self) -> TSCPRIV_R {
        TSCPRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - privileged access mode for register of DMA2D"]
    #[inline(always)]
    pub fn dma2dpriv(&self) -> DMA2DPRIV_R {
        DMA2DPRIV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - privileged access mode for ICACHE registers"]
    #[inline(always)]
    pub fn icache_regpriv(&self) -> ICACHE_REGPRIV_R {
        ICACHE_REGPRIV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - privileged access mode for DCACHE registers"]
    #[inline(always)]
    pub fn dcache_regpriv(&self) -> DCACHE_REGPRIV_R {
        DCACHE_REGPRIV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - privileged access mode for ADC1"]
    #[inline(always)]
    pub fn adc1priv(&self) -> ADC1PRIV_R {
        ADC1PRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - privileged access mode for DCMI"]
    #[inline(always)]
    pub fn dcmipriv(&self) -> DCMIPRIV_R {
        DCMIPRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - privileged access mode for OTG_FS"]
    #[inline(always)]
    pub fn otgfspriv(&self) -> OTGFSPRIV_R {
        OTGFSPRIV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - privileged access mode for AES"]
    #[inline(always)]
    pub fn aespriv(&self) -> AESPRIV_R {
        AESPRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - privileged access mode for HASH"]
    #[inline(always)]
    pub fn hashpriv(&self) -> HASHPRIV_R {
        HASHPRIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - privileged access mode for RNG"]
    #[inline(always)]
    pub fn rngpriv(&self) -> RNGPRIV_R {
        RNGPRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - privileged access mode for PKA"]
    #[inline(always)]
    pub fn pkapriv(&self) -> PKAPRIV_R {
        PKAPRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - privileged access mode for SAES"]
    #[inline(always)]
    pub fn saespriv(&self) -> SAESPRIV_R {
        SAESPRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - privileged access mode for OCTOSPIM"]
    #[inline(always)]
    pub fn octospimpriv(&self) -> OCTOSPIMPRIV_R {
        OCTOSPIMPRIV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - privileged access mode for SDMMC2"]
    #[inline(always)]
    pub fn sdmmc1priv(&self) -> SDMMC1PRIV_R {
        SDMMC1PRIV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - privileged access mode for SDMMC1"]
    #[inline(always)]
    pub fn sdmmc2priv(&self) -> SDMMC2PRIV_R {
        SDMMC2PRIV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - privileged access mode for FSMC registers"]
    #[inline(always)]
    pub fn fsmc_regpriv(&self) -> FSMC_REGPRIV_R {
        FSMC_REGPRIV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - privileged access mode for OCTOSPI1"]
    #[inline(always)]
    pub fn octospi1_regpriv(&self) -> OCTOSPI1_REGPRIV_R {
        OCTOSPI1_REGPRIV_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - privileged access mode for OCTOSPI2"]
    #[inline(always)]
    pub fn octospi2_regpriv(&self) -> OCTOSPI2_REGPRIV_R {
        OCTOSPI2_REGPRIV_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - privileged access mode for RAMCFG"]
    #[inline(always)]
    pub fn ramcfgpriv(&self) -> RAMCFGPRIV_R {
        RAMCFGPRIV_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - privileged access mode for MDF1"]
    #[inline(always)]
    #[must_use]
    pub fn mdf1priv(&mut self) -> MDF1PRIV_W<TZSC_PRIVCFGR3rs> {
        MDF1PRIV_W::new(self, 0)
    }
    #[doc = "Bit 1 - privileged access mode for CORDIC"]
    #[inline(always)]
    #[must_use]
    pub fn cordicpriv(&mut self) -> CORDICPRIV_W<TZSC_PRIVCFGR3rs> {
        CORDICPRIV_W::new(self, 1)
    }
    #[doc = "Bit 2 - privileged access mode for FMAC"]
    #[inline(always)]
    #[must_use]
    pub fn fmacpriv(&mut self) -> FMACPRIV_W<TZSC_PRIVCFGR3rs> {
        FMACPRIV_W::new(self, 2)
    }
    #[doc = "Bit 3 - privileged access mode for CRC"]
    #[inline(always)]
    #[must_use]
    pub fn crcpriv(&mut self) -> CRCPRIV_W<TZSC_PRIVCFGR3rs> {
        CRCPRIV_W::new(self, 3)
    }
    #[doc = "Bit 4 - privileged access mode for TSC"]
    #[inline(always)]
    #[must_use]
    pub fn tscpriv(&mut self) -> TSCPRIV_W<TZSC_PRIVCFGR3rs> {
        TSCPRIV_W::new(self, 4)
    }
    #[doc = "Bit 5 - privileged access mode for register of DMA2D"]
    #[inline(always)]
    #[must_use]
    pub fn dma2dpriv(&mut self) -> DMA2DPRIV_W<TZSC_PRIVCFGR3rs> {
        DMA2DPRIV_W::new(self, 5)
    }
    #[doc = "Bit 6 - privileged access mode for ICACHE registers"]
    #[inline(always)]
    #[must_use]
    pub fn icache_regpriv(&mut self) -> ICACHE_REGPRIV_W<TZSC_PRIVCFGR3rs> {
        ICACHE_REGPRIV_W::new(self, 6)
    }
    #[doc = "Bit 7 - privileged access mode for DCACHE registers"]
    #[inline(always)]
    #[must_use]
    pub fn dcache_regpriv(&mut self) -> DCACHE_REGPRIV_W<TZSC_PRIVCFGR3rs> {
        DCACHE_REGPRIV_W::new(self, 7)
    }
    #[doc = "Bit 8 - privileged access mode for ADC1"]
    #[inline(always)]
    #[must_use]
    pub fn adc1priv(&mut self) -> ADC1PRIV_W<TZSC_PRIVCFGR3rs> {
        ADC1PRIV_W::new(self, 8)
    }
    #[doc = "Bit 9 - privileged access mode for DCMI"]
    #[inline(always)]
    #[must_use]
    pub fn dcmipriv(&mut self) -> DCMIPRIV_W<TZSC_PRIVCFGR3rs> {
        DCMIPRIV_W::new(self, 9)
    }
    #[doc = "Bit 10 - privileged access mode for OTG_FS"]
    #[inline(always)]
    #[must_use]
    pub fn otgfspriv(&mut self) -> OTGFSPRIV_W<TZSC_PRIVCFGR3rs> {
        OTGFSPRIV_W::new(self, 10)
    }
    #[doc = "Bit 11 - privileged access mode for AES"]
    #[inline(always)]
    #[must_use]
    pub fn aespriv(&mut self) -> AESPRIV_W<TZSC_PRIVCFGR3rs> {
        AESPRIV_W::new(self, 11)
    }
    #[doc = "Bit 12 - privileged access mode for HASH"]
    #[inline(always)]
    #[must_use]
    pub fn hashpriv(&mut self) -> HASHPRIV_W<TZSC_PRIVCFGR3rs> {
        HASHPRIV_W::new(self, 12)
    }
    #[doc = "Bit 13 - privileged access mode for RNG"]
    #[inline(always)]
    #[must_use]
    pub fn rngpriv(&mut self) -> RNGPRIV_W<TZSC_PRIVCFGR3rs> {
        RNGPRIV_W::new(self, 13)
    }
    #[doc = "Bit 14 - privileged access mode for PKA"]
    #[inline(always)]
    #[must_use]
    pub fn pkapriv(&mut self) -> PKAPRIV_W<TZSC_PRIVCFGR3rs> {
        PKAPRIV_W::new(self, 14)
    }
    #[doc = "Bit 15 - privileged access mode for SAES"]
    #[inline(always)]
    #[must_use]
    pub fn saespriv(&mut self) -> SAESPRIV_W<TZSC_PRIVCFGR3rs> {
        SAESPRIV_W::new(self, 15)
    }
    #[doc = "Bit 16 - privileged access mode for OCTOSPIM"]
    #[inline(always)]
    #[must_use]
    pub fn octospimpriv(&mut self) -> OCTOSPIMPRIV_W<TZSC_PRIVCFGR3rs> {
        OCTOSPIMPRIV_W::new(self, 16)
    }
    #[doc = "Bit 17 - privileged access mode for SDMMC2"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1priv(&mut self) -> SDMMC1PRIV_W<TZSC_PRIVCFGR3rs> {
        SDMMC1PRIV_W::new(self, 17)
    }
    #[doc = "Bit 18 - privileged access mode for SDMMC1"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2priv(&mut self) -> SDMMC2PRIV_W<TZSC_PRIVCFGR3rs> {
        SDMMC2PRIV_W::new(self, 18)
    }
    #[doc = "Bit 19 - privileged access mode for FSMC registers"]
    #[inline(always)]
    #[must_use]
    pub fn fsmc_regpriv(&mut self) -> FSMC_REGPRIV_W<TZSC_PRIVCFGR3rs> {
        FSMC_REGPRIV_W::new(self, 19)
    }
    #[doc = "Bit 20 - privileged access mode for OCTOSPI1"]
    #[inline(always)]
    #[must_use]
    pub fn octospi1_regpriv(&mut self) -> OCTOSPI1_REGPRIV_W<TZSC_PRIVCFGR3rs> {
        OCTOSPI1_REGPRIV_W::new(self, 20)
    }
    #[doc = "Bit 21 - privileged access mode for OCTOSPI2"]
    #[inline(always)]
    #[must_use]
    pub fn octospi2_regpriv(&mut self) -> OCTOSPI2_REGPRIV_W<TZSC_PRIVCFGR3rs> {
        OCTOSPI2_REGPRIV_W::new(self, 21)
    }
    #[doc = "Bit 22 - privileged access mode for RAMCFG"]
    #[inline(always)]
    #[must_use]
    pub fn ramcfgpriv(&mut self) -> RAMCFGPRIV_W<TZSC_PRIVCFGR3rs> {
        RAMCFGPRIV_W::new(self, 22)
    }
}
#[doc = "TZSC privilege configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_privcfgr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_privcfgr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZSC_PRIVCFGR3rs;
impl crate::RegisterSpec for TZSC_PRIVCFGR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzsc_privcfgr3::R`](R) reader structure"]
impl crate::Readable for TZSC_PRIVCFGR3rs {}
#[doc = "`write(|w| ..)` method takes [`tzsc_privcfgr3::W`](W) writer structure"]
impl crate::Writable for TZSC_PRIVCFGR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZSC_PRIVCFGR3 to value 0"]
impl crate::Resettable for TZSC_PRIVCFGR3rs {
    const RESET_VALUE: u32 = 0;
}
