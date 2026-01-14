///Register `SECCFGR3` reader
pub type R = crate::R<SECCFGR3rs>;
///Register `SECCFGR3` writer
pub type W = crate::W<SECCFGR3rs>;
///Field `CRCSEC` reader - secure access mode for CRC
pub type CRCSEC_R = crate::BitReader;
///Field `CRCSEC` writer - secure access mode for CRC
pub type CRCSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSCSEC` reader - secure access mode for TSC
pub type TSCSEC_R = crate::BitReader;
///Field `TSCSEC` writer - secure access mode for TSC
pub type TSCSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICACHE_REGSEC` reader - secure access mode for ICACHE registers
pub type ICACHE_REGSEC_R = crate::BitReader;
///Field `ICACHE_REGSEC` writer - secure access mode for ICACHE registers
pub type ICACHE_REGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AESSEC` reader - secure access mode for AES
pub type AESSEC_R = crate::BitReader;
///Field `AESSEC` writer - secure access mode for AES
pub type AESSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASHSEC` reader - secure access mode for HASH
pub type HASHSEC_R = crate::BitReader;
///Field `HASHSEC` writer - secure access mode for HASH
pub type HASHSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGSEC` reader - secure access mode for RNG
pub type RNGSEC_R = crate::BitReader;
///Field `RNGSEC` writer - secure access mode for RNG
pub type RNGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAESSEC` reader - secure access mode for SAES
pub type SAESSEC_R = crate::BitReader;
///Field `SAESSEC` writer - secure access mode for SAES
pub type SAESSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKASEC` reader - secure access mode for PKA
pub type PKASEC_R = crate::BitReader;
///Field `PKASEC` writer - secure access mode for PKA
pub type PKASEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMCFGSEC` reader - secure access mode for RAMCFG
pub type RAMCFGSEC_R = crate::BitReader;
///Field `RAMCFGSEC` writer - secure access mode for RAMCFG
pub type RAMCFGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RADIOSEC` reader - secure access mode for 2.4 GHz RADIO
pub type RADIOSEC_R = crate::BitReader;
///Field `RADIOSEC` writer - secure access mode for 2.4 GHz RADIO
pub type RADIOSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - secure access mode for CRC
    #[inline(always)]
    pub fn crcsec(&self) -> CRCSEC_R {
        CRCSEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - secure access mode for TSC
    #[inline(always)]
    pub fn tscsec(&self) -> TSCSEC_R {
        TSCSEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - secure access mode for ICACHE registers
    #[inline(always)]
    pub fn icache_regsec(&self) -> ICACHE_REGSEC_R {
        ICACHE_REGSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 11 - secure access mode for AES
    #[inline(always)]
    pub fn aessec(&self) -> AESSEC_R {
        AESSEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - secure access mode for HASH
    #[inline(always)]
    pub fn hashsec(&self) -> HASHSEC_R {
        HASHSEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - secure access mode for RNG
    #[inline(always)]
    pub fn rngsec(&self) -> RNGSEC_R {
        RNGSEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - secure access mode for SAES
    #[inline(always)]
    pub fn saessec(&self) -> SAESSEC_R {
        SAESSEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - secure access mode for PKA
    #[inline(always)]
    pub fn pkasec(&self) -> PKASEC_R {
        PKASEC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 22 - secure access mode for RAMCFG
    #[inline(always)]
    pub fn ramcfgsec(&self) -> RAMCFGSEC_R {
        RAMCFGSEC_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - secure access mode for 2.4 GHz RADIO
    #[inline(always)]
    pub fn radiosec(&self) -> RADIOSEC_R {
        RADIOSEC_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR3")
            .field("crcsec", &self.crcsec())
            .field("tscsec", &self.tscsec())
            .field("icache_regsec", &self.icache_regsec())
            .field("aessec", &self.aessec())
            .field("hashsec", &self.hashsec())
            .field("rngsec", &self.rngsec())
            .field("saessec", &self.saessec())
            .field("pkasec", &self.pkasec())
            .field("ramcfgsec", &self.ramcfgsec())
            .field("radiosec", &self.radiosec())
            .finish()
    }
}
impl W {
    ///Bit 3 - secure access mode for CRC
    #[inline(always)]
    pub fn crcsec(&mut self) -> CRCSEC_W<'_, SECCFGR3rs> {
        CRCSEC_W::new(self, 3)
    }
    ///Bit 4 - secure access mode for TSC
    #[inline(always)]
    pub fn tscsec(&mut self) -> TSCSEC_W<'_, SECCFGR3rs> {
        TSCSEC_W::new(self, 4)
    }
    ///Bit 6 - secure access mode for ICACHE registers
    #[inline(always)]
    pub fn icache_regsec(&mut self) -> ICACHE_REGSEC_W<'_, SECCFGR3rs> {
        ICACHE_REGSEC_W::new(self, 6)
    }
    ///Bit 11 - secure access mode for AES
    #[inline(always)]
    pub fn aessec(&mut self) -> AESSEC_W<'_, SECCFGR3rs> {
        AESSEC_W::new(self, 11)
    }
    ///Bit 12 - secure access mode for HASH
    #[inline(always)]
    pub fn hashsec(&mut self) -> HASHSEC_W<'_, SECCFGR3rs> {
        HASHSEC_W::new(self, 12)
    }
    ///Bit 13 - secure access mode for RNG
    #[inline(always)]
    pub fn rngsec(&mut self) -> RNGSEC_W<'_, SECCFGR3rs> {
        RNGSEC_W::new(self, 13)
    }
    ///Bit 14 - secure access mode for SAES
    #[inline(always)]
    pub fn saessec(&mut self) -> SAESSEC_W<'_, SECCFGR3rs> {
        SAESSEC_W::new(self, 14)
    }
    ///Bit 16 - secure access mode for PKA
    #[inline(always)]
    pub fn pkasec(&mut self) -> PKASEC_W<'_, SECCFGR3rs> {
        PKASEC_W::new(self, 16)
    }
    ///Bit 22 - secure access mode for RAMCFG
    #[inline(always)]
    pub fn ramcfgsec(&mut self) -> RAMCFGSEC_W<'_, SECCFGR3rs> {
        RAMCFGSEC_W::new(self, 22)
    }
    ///Bit 23 - secure access mode for 2.4 GHz RADIO
    #[inline(always)]
    pub fn radiosec(&mut self) -> RADIOSEC_W<'_, SECCFGR3rs> {
        RADIOSEC_W::new(self, 23)
    }
}
/**GTZC1 TZSC secure configuration register 3

You can [`read`](crate::Reg::read) this register and get [`seccfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#GTZC1_TZSC:SECCFGR3)*/
pub struct SECCFGR3rs;
impl crate::RegisterSpec for SECCFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr3::R`](R) reader structure
impl crate::Readable for SECCFGR3rs {}
///`write(|w| ..)` method takes [`seccfgr3::W`](W) writer structure
impl crate::Writable for SECCFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR3 to value 0
impl crate::Resettable for SECCFGR3rs {}
