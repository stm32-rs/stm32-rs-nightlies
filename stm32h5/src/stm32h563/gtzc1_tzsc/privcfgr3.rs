#[doc = "Register `PRIVCFGR3` reader"]
pub type R = crate::R<PRIVCFGR3rs>;
#[doc = "Register `PRIVCFGR3` writer"]
pub type W = crate::W<PRIVCFGR3rs>;
#[doc = "Field `LPTIM6PRIV` reader - privileged access mode for LPTIM6"]
pub type LPTIM6PRIV_R = crate::BitReader;
#[doc = "Field `LPTIM6PRIV` writer - privileged access mode for LPTIM6"]
pub type LPTIM6PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFBUFPRIV` reader - privileged access mode for VREFBUF"]
pub type VREFBUFPRIV_R = crate::BitReader;
#[doc = "Field `VREFBUFPRIV` writer - privileged access mode for VREFBUF"]
pub type VREFBUFPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCPRIV` reader - privileged access mode for CRC"]
pub type CRCPRIV_R = crate::BitReader;
#[doc = "Field `CRCPRIV` writer - privileged access mode for CRC"]
pub type CRCPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORDICPRIV` reader - privileged access mode for CORDIC"]
pub type CORDICPRIV_R = crate::BitReader;
#[doc = "Field `CORDICPRIV` writer - privileged access mode for CORDIC"]
pub type CORDICPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMACPRIV` reader - privileged access mode for FMAC"]
pub type FMACPRIV_R = crate::BitReader;
#[doc = "Field `FMACPRIV` writer - privileged access mode for FMAC"]
pub type FMACPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETHPRIV` reader - privileged access mode for register of ETH"]
pub type ETHPRIV_R = crate::BitReader;
#[doc = "Field `ETHPRIV` writer - privileged access mode for register of ETH"]
pub type ETHPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHEPRIV` reader - privileged access mode for ICACHE"]
pub type ICACHEPRIV_R = crate::BitReader;
#[doc = "Field `ICACHEPRIV` writer - privileged access mode for ICACHE"]
pub type ICACHEPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHEPRIV` reader - privileged access mode for DCACHE"]
pub type DCACHEPRIV_R = crate::BitReader;
#[doc = "Field `DCACHEPRIV` writer - privileged access mode for DCACHE"]
pub type DCACHEPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12PRIV` reader - privileged access mode for ADC1 and ADC2"]
pub type ADC12PRIV_R = crate::BitReader;
#[doc = "Field `ADC12PRIV` writer - privileged access mode for ADC1 and ADC2"]
pub type ADC12PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCMIPRIV` reader - privileged access mode for DCMI"]
pub type DCMIPRIV_R = crate::BitReader;
#[doc = "Field `DCMIPRIV` writer - privileged access mode for DCMI"]
pub type DCMIPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASHPRIV` reader - privileged access mode for HASH"]
pub type HASHPRIV_R = crate::BitReader;
#[doc = "Field `HASHPRIV` writer - privileged access mode for HASH"]
pub type HASHPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGPRIV` reader - privileged access mode for RNG"]
pub type RNGPRIV_R = crate::BitReader;
#[doc = "Field `RNGPRIV` writer - privileged access mode for RNG"]
pub type RNGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC2PRIV` reader - privileged access mode for SDMMC2"]
pub type SDMMC2PRIV_R = crate::BitReader;
#[doc = "Field `SDMMC2PRIV` writer - privileged access mode for SDMMC2"]
pub type SDMMC2PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC1PRIV` reader - privileged access mode for SDMMC1"]
pub type SDMMC1PRIV_R = crate::BitReader;
#[doc = "Field `SDMMC1PRIV` writer - privileged access mode for SDMMC1"]
pub type SDMMC1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMCPRIV` reader - privileged access mode for FMC"]
pub type FMCPRIV_R = crate::BitReader;
#[doc = "Field `FMCPRIV` writer - privileged access mode for FMC"]
pub type FMCPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTOSPI1PRIV` reader - privileged access mode for OCTOSPI1"]
pub type OCTOSPI1PRIV_R = crate::BitReader;
#[doc = "Field `OCTOSPI1PRIV` writer - privileged access mode for OCTOSPI1"]
pub type OCTOSPI1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMCFGPRIV` reader - privileged access mode for RAMSCFG"]
pub type RAMCFGPRIV_R = crate::BitReader;
#[doc = "Field `RAMCFGPRIV` writer - privileged access mode for RAMSCFG"]
pub type RAMCFGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - privileged access mode for LPTIM6"]
    #[inline(always)]
    pub fn lptim6priv(&self) -> LPTIM6PRIV_R {
        LPTIM6PRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - privileged access mode for VREFBUF"]
    #[inline(always)]
    pub fn vrefbufpriv(&self) -> VREFBUFPRIV_R {
        VREFBUFPRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - privileged access mode for CRC"]
    #[inline(always)]
    pub fn crcpriv(&self) -> CRCPRIV_R {
        CRCPRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - privileged access mode for CORDIC"]
    #[inline(always)]
    pub fn cordicpriv(&self) -> CORDICPRIV_R {
        CORDICPRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - privileged access mode for FMAC"]
    #[inline(always)]
    pub fn fmacpriv(&self) -> FMACPRIV_R {
        FMACPRIV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - privileged access mode for register of ETH"]
    #[inline(always)]
    pub fn ethpriv(&self) -> ETHPRIV_R {
        ETHPRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - privileged access mode for ICACHE"]
    #[inline(always)]
    pub fn icachepriv(&self) -> ICACHEPRIV_R {
        ICACHEPRIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - privileged access mode for DCACHE"]
    #[inline(always)]
    pub fn dcachepriv(&self) -> DCACHEPRIV_R {
        DCACHEPRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - privileged access mode for ADC1 and ADC2"]
    #[inline(always)]
    pub fn adc12priv(&self) -> ADC12PRIV_R {
        ADC12PRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - privileged access mode for DCMI"]
    #[inline(always)]
    pub fn dcmipriv(&self) -> DCMIPRIV_R {
        DCMIPRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - privileged access mode for HASH"]
    #[inline(always)]
    pub fn hashpriv(&self) -> HASHPRIV_R {
        HASHPRIV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - privileged access mode for RNG"]
    #[inline(always)]
    pub fn rngpriv(&self) -> RNGPRIV_R {
        RNGPRIV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - privileged access mode for SDMMC2"]
    #[inline(always)]
    pub fn sdmmc2priv(&self) -> SDMMC2PRIV_R {
        SDMMC2PRIV_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - privileged access mode for SDMMC1"]
    #[inline(always)]
    pub fn sdmmc1priv(&self) -> SDMMC1PRIV_R {
        SDMMC1PRIV_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - privileged access mode for FMC"]
    #[inline(always)]
    pub fn fmcpriv(&self) -> FMCPRIV_R {
        FMCPRIV_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - privileged access mode for OCTOSPI1"]
    #[inline(always)]
    pub fn octospi1priv(&self) -> OCTOSPI1PRIV_R {
        OCTOSPI1PRIV_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - privileged access mode for RAMSCFG"]
    #[inline(always)]
    pub fn ramcfgpriv(&self) -> RAMCFGPRIV_R {
        RAMCFGPRIV_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - privileged access mode for LPTIM6"]
    #[inline(always)]
    #[must_use]
    pub fn lptim6priv(&mut self) -> LPTIM6PRIV_W<PRIVCFGR3rs> {
        LPTIM6PRIV_W::new(self, 0)
    }
    #[doc = "Bit 1 - privileged access mode for VREFBUF"]
    #[inline(always)]
    #[must_use]
    pub fn vrefbufpriv(&mut self) -> VREFBUFPRIV_W<PRIVCFGR3rs> {
        VREFBUFPRIV_W::new(self, 1)
    }
    #[doc = "Bit 8 - privileged access mode for CRC"]
    #[inline(always)]
    #[must_use]
    pub fn crcpriv(&mut self) -> CRCPRIV_W<PRIVCFGR3rs> {
        CRCPRIV_W::new(self, 8)
    }
    #[doc = "Bit 9 - privileged access mode for CORDIC"]
    #[inline(always)]
    #[must_use]
    pub fn cordicpriv(&mut self) -> CORDICPRIV_W<PRIVCFGR3rs> {
        CORDICPRIV_W::new(self, 9)
    }
    #[doc = "Bit 10 - privileged access mode for FMAC"]
    #[inline(always)]
    #[must_use]
    pub fn fmacpriv(&mut self) -> FMACPRIV_W<PRIVCFGR3rs> {
        FMACPRIV_W::new(self, 10)
    }
    #[doc = "Bit 11 - privileged access mode for register of ETH"]
    #[inline(always)]
    #[must_use]
    pub fn ethpriv(&mut self) -> ETHPRIV_W<PRIVCFGR3rs> {
        ETHPRIV_W::new(self, 11)
    }
    #[doc = "Bit 12 - privileged access mode for ICACHE"]
    #[inline(always)]
    #[must_use]
    pub fn icachepriv(&mut self) -> ICACHEPRIV_W<PRIVCFGR3rs> {
        ICACHEPRIV_W::new(self, 12)
    }
    #[doc = "Bit 13 - privileged access mode for DCACHE"]
    #[inline(always)]
    #[must_use]
    pub fn dcachepriv(&mut self) -> DCACHEPRIV_W<PRIVCFGR3rs> {
        DCACHEPRIV_W::new(self, 13)
    }
    #[doc = "Bit 14 - privileged access mode for ADC1 and ADC2"]
    #[inline(always)]
    #[must_use]
    pub fn adc12priv(&mut self) -> ADC12PRIV_W<PRIVCFGR3rs> {
        ADC12PRIV_W::new(self, 14)
    }
    #[doc = "Bit 15 - privileged access mode for DCMI"]
    #[inline(always)]
    #[must_use]
    pub fn dcmipriv(&mut self) -> DCMIPRIV_W<PRIVCFGR3rs> {
        DCMIPRIV_W::new(self, 15)
    }
    #[doc = "Bit 17 - privileged access mode for HASH"]
    #[inline(always)]
    #[must_use]
    pub fn hashpriv(&mut self) -> HASHPRIV_W<PRIVCFGR3rs> {
        HASHPRIV_W::new(self, 17)
    }
    #[doc = "Bit 18 - privileged access mode for RNG"]
    #[inline(always)]
    #[must_use]
    pub fn rngpriv(&mut self) -> RNGPRIV_W<PRIVCFGR3rs> {
        RNGPRIV_W::new(self, 18)
    }
    #[doc = "Bit 21 - privileged access mode for SDMMC2"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2priv(&mut self) -> SDMMC2PRIV_W<PRIVCFGR3rs> {
        SDMMC2PRIV_W::new(self, 21)
    }
    #[doc = "Bit 22 - privileged access mode for SDMMC1"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1priv(&mut self) -> SDMMC1PRIV_W<PRIVCFGR3rs> {
        SDMMC1PRIV_W::new(self, 22)
    }
    #[doc = "Bit 23 - privileged access mode for FMC"]
    #[inline(always)]
    #[must_use]
    pub fn fmcpriv(&mut self) -> FMCPRIV_W<PRIVCFGR3rs> {
        FMCPRIV_W::new(self, 23)
    }
    #[doc = "Bit 24 - privileged access mode for OCTOSPI1"]
    #[inline(always)]
    #[must_use]
    pub fn octospi1priv(&mut self) -> OCTOSPI1PRIV_W<PRIVCFGR3rs> {
        OCTOSPI1PRIV_W::new(self, 24)
    }
    #[doc = "Bit 26 - privileged access mode for RAMSCFG"]
    #[inline(always)]
    #[must_use]
    pub fn ramcfgpriv(&mut self) -> RAMCFGPRIV_W<PRIVCFGR3rs> {
        RAMCFGPRIV_W::new(self, 26)
    }
}
#[doc = "GTZC1 TZSC privilege configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcfgr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcfgr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIVCFGR3rs;
impl crate::RegisterSpec for PRIVCFGR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`privcfgr3::R`](R) reader structure"]
impl crate::Readable for PRIVCFGR3rs {}
#[doc = "`write(|w| ..)` method takes [`privcfgr3::W`](W) writer structure"]
impl crate::Writable for PRIVCFGR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIVCFGR3 to value 0"]
impl crate::Resettable for PRIVCFGR3rs {
    const RESET_VALUE: u32 = 0;
}
