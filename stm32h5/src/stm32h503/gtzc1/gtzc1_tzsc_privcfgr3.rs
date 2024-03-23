#[doc = "Register `GTZC1_TZSC_PRIVCFGR3` reader"]
pub type R = crate::R<GTZC1_TZSC_PRIVCFGR3rs>;
#[doc = "Register `GTZC1_TZSC_PRIVCFGR3` writer"]
pub type W = crate::W<GTZC1_TZSC_PRIVCFGR3rs>;
#[doc = "Field `I3C2PRIV` reader - privileged access mode for I3C2"]
pub type I3C2PRIV_R = crate::BitReader;
#[doc = "Field `I3C2PRIV` writer - privileged access mode for I3C2"]
pub type I3C2PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCPRIV` reader - privileged access mode for CRC"]
pub type CRCPRIV_R = crate::BitReader;
#[doc = "Field `CRCPRIV` writer - privileged access mode for CRC"]
pub type CRCPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHEPRIV` reader - privileged access mode for ICACHE"]
pub type ICACHEPRIV_R = crate::BitReader;
#[doc = "Field `ICACHEPRIV` writer - privileged access mode for ICACHE"]
pub type ICACHEPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1PRIV` reader - privileged access mode for ADC1"]
pub type ADC1PRIV_R = crate::BitReader;
#[doc = "Field `ADC1PRIV` writer - privileged access mode for ADC1"]
pub type ADC1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASHPRIV` reader - privileged access mode for HASH"]
pub type HASHPRIV_R = crate::BitReader;
#[doc = "Field `HASHPRIV` writer - privileged access mode for HASH"]
pub type HASHPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGPRIV` reader - privileged access mode for RNG"]
pub type RNGPRIV_R = crate::BitReader;
#[doc = "Field `RNGPRIV` writer - privileged access mode for RNG"]
pub type RNGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMCFGPRIV` reader - privileged access mode for RAMSCFG"]
pub type RAMCFGPRIV_R = crate::BitReader;
#[doc = "Field `RAMCFGPRIV` writer - privileged access mode for RAMSCFG"]
pub type RAMCFGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - privileged access mode for I3C2"]
    #[inline(always)]
    pub fn i3c2priv(&self) -> I3C2PRIV_R {
        I3C2PRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - privileged access mode for CRC"]
    #[inline(always)]
    pub fn crcpriv(&self) -> CRCPRIV_R {
        CRCPRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - privileged access mode for ICACHE"]
    #[inline(always)]
    pub fn icachepriv(&self) -> ICACHEPRIV_R {
        ICACHEPRIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - privileged access mode for ADC1"]
    #[inline(always)]
    pub fn adc1priv(&self) -> ADC1PRIV_R {
        ADC1PRIV_R::new(((self.bits >> 14) & 1) != 0)
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
    #[doc = "Bit 26 - privileged access mode for RAMSCFG"]
    #[inline(always)]
    pub fn ramcfgpriv(&self) -> RAMCFGPRIV_R {
        RAMCFGPRIV_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - privileged access mode for I3C2"]
    #[inline(always)]
    #[must_use]
    pub fn i3c2priv(&mut self) -> I3C2PRIV_W<GTZC1_TZSC_PRIVCFGR3rs> {
        I3C2PRIV_W::new(self, 2)
    }
    #[doc = "Bit 8 - privileged access mode for CRC"]
    #[inline(always)]
    #[must_use]
    pub fn crcpriv(&mut self) -> CRCPRIV_W<GTZC1_TZSC_PRIVCFGR3rs> {
        CRCPRIV_W::new(self, 8)
    }
    #[doc = "Bit 12 - privileged access mode for ICACHE"]
    #[inline(always)]
    #[must_use]
    pub fn icachepriv(&mut self) -> ICACHEPRIV_W<GTZC1_TZSC_PRIVCFGR3rs> {
        ICACHEPRIV_W::new(self, 12)
    }
    #[doc = "Bit 14 - privileged access mode for ADC1"]
    #[inline(always)]
    #[must_use]
    pub fn adc1priv(&mut self) -> ADC1PRIV_W<GTZC1_TZSC_PRIVCFGR3rs> {
        ADC1PRIV_W::new(self, 14)
    }
    #[doc = "Bit 17 - privileged access mode for HASH"]
    #[inline(always)]
    #[must_use]
    pub fn hashpriv(&mut self) -> HASHPRIV_W<GTZC1_TZSC_PRIVCFGR3rs> {
        HASHPRIV_W::new(self, 17)
    }
    #[doc = "Bit 18 - privileged access mode for RNG"]
    #[inline(always)]
    #[must_use]
    pub fn rngpriv(&mut self) -> RNGPRIV_W<GTZC1_TZSC_PRIVCFGR3rs> {
        RNGPRIV_W::new(self, 18)
    }
    #[doc = "Bit 26 - privileged access mode for RAMSCFG"]
    #[inline(always)]
    #[must_use]
    pub fn ramcfgpriv(&mut self) -> RAMCFGPRIV_W<GTZC1_TZSC_PRIVCFGR3rs> {
        RAMCFGPRIV_W::new(self, 26)
    }
}
#[doc = "GTZC1 TZSC privilege configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_tzsc_privcfgr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzsc_privcfgr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTZC1_TZSC_PRIVCFGR3rs;
impl crate::RegisterSpec for GTZC1_TZSC_PRIVCFGR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtzc1_tzsc_privcfgr3::R`](R) reader structure"]
impl crate::Readable for GTZC1_TZSC_PRIVCFGR3rs {}
#[doc = "`write(|w| ..)` method takes [`gtzc1_tzsc_privcfgr3::W`](W) writer structure"]
impl crate::Writable for GTZC1_TZSC_PRIVCFGR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTZC1_TZSC_PRIVCFGR3 to value 0"]
impl crate::Resettable for GTZC1_TZSC_PRIVCFGR3rs {
    const RESET_VALUE: u32 = 0;
}
