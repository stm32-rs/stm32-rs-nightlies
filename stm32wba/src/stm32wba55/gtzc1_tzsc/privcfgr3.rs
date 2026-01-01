///Register `PRIVCFGR3` reader
pub type R = crate::R<PRIVCFGR3rs>;
///Register `PRIVCFGR3` writer
pub type W = crate::W<PRIVCFGR3rs>;
///Field `CRCPRIV` reader - privileged access mode for CRC
pub type CRCPRIV_R = crate::BitReader;
///Field `CRCPRIV` writer - privileged access mode for CRC
pub type CRCPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSCPRIV` reader - privileged access mode for TSC
pub type TSCPRIV_R = crate::BitReader;
///Field `TSCPRIV` writer - privileged access mode for TSC
pub type TSCPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICACHE_REGPRIV` reader - privileged access mode for ICACHE registers
pub type ICACHE_REGPRIV_R = crate::BitReader;
///Field `ICACHE_REGPRIV` writer - privileged access mode for ICACHE registers
pub type ICACHE_REGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AESPRIV` reader - privileged access mode for AES
pub type AESPRIV_R = crate::BitReader;
///Field `AESPRIV` writer - privileged access mode for AES
pub type AESPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASHPRIV` reader - privileged access mode for HASH
pub type HASHPRIV_R = crate::BitReader;
///Field `HASHPRIV` writer - privileged access mode for HASH
pub type HASHPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGPRIV` reader - privileged access mode for RNG
pub type RNGPRIV_R = crate::BitReader;
///Field `RNGPRIV` writer - privileged access mode for RNG
pub type RNGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAESPRIV` reader - privileged access mode for SAES
pub type SAESPRIV_R = crate::BitReader;
///Field `SAESPRIV` writer - privileged access mode for SAES
pub type SAESPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKAPRIV` reader - privileged access mode for PKA
pub type PKAPRIV_R = crate::BitReader;
///Field `PKAPRIV` writer - privileged access mode for PKA
pub type PKAPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMCFGPRIV` reader - privileged access mode for RAMCFG
pub type RAMCFGPRIV_R = crate::BitReader;
///Field `RAMCFGPRIV` writer - privileged access mode for RAMCFG
pub type RAMCFGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RADIOPRIV` reader - privileged access mode for 2.4 GHz RADIO
pub type RADIOPRIV_R = crate::BitReader;
///Field `RADIOPRIV` writer - privileged access mode for 2.4 GHz RADIO
pub type RADIOPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - privileged access mode for CRC
    #[inline(always)]
    pub fn crcpriv(&self) -> CRCPRIV_R {
        CRCPRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - privileged access mode for TSC
    #[inline(always)]
    pub fn tscpriv(&self) -> TSCPRIV_R {
        TSCPRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - privileged access mode for ICACHE registers
    #[inline(always)]
    pub fn icache_regpriv(&self) -> ICACHE_REGPRIV_R {
        ICACHE_REGPRIV_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 11 - privileged access mode for AES
    #[inline(always)]
    pub fn aespriv(&self) -> AESPRIV_R {
        AESPRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - privileged access mode for HASH
    #[inline(always)]
    pub fn hashpriv(&self) -> HASHPRIV_R {
        HASHPRIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - privileged access mode for RNG
    #[inline(always)]
    pub fn rngpriv(&self) -> RNGPRIV_R {
        RNGPRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - privileged access mode for SAES
    #[inline(always)]
    pub fn saespriv(&self) -> SAESPRIV_R {
        SAESPRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - privileged access mode for PKA
    #[inline(always)]
    pub fn pkapriv(&self) -> PKAPRIV_R {
        PKAPRIV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 22 - privileged access mode for RAMCFG
    #[inline(always)]
    pub fn ramcfgpriv(&self) -> RAMCFGPRIV_R {
        RAMCFGPRIV_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - privileged access mode for 2.4 GHz RADIO
    #[inline(always)]
    pub fn radiopriv(&self) -> RADIOPRIV_R {
        RADIOPRIV_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR3")
            .field("crcpriv", &self.crcpriv())
            .field("tscpriv", &self.tscpriv())
            .field("icache_regpriv", &self.icache_regpriv())
            .field("aespriv", &self.aespriv())
            .field("hashpriv", &self.hashpriv())
            .field("rngpriv", &self.rngpriv())
            .field("saespriv", &self.saespriv())
            .field("pkapriv", &self.pkapriv())
            .field("ramcfgpriv", &self.ramcfgpriv())
            .field("radiopriv", &self.radiopriv())
            .finish()
    }
}
impl W {
    ///Bit 3 - privileged access mode for CRC
    #[inline(always)]
    pub fn crcpriv(&mut self) -> CRCPRIV_W<'_, PRIVCFGR3rs> {
        CRCPRIV_W::new(self, 3)
    }
    ///Bit 4 - privileged access mode for TSC
    #[inline(always)]
    pub fn tscpriv(&mut self) -> TSCPRIV_W<'_, PRIVCFGR3rs> {
        TSCPRIV_W::new(self, 4)
    }
    ///Bit 6 - privileged access mode for ICACHE registers
    #[inline(always)]
    pub fn icache_regpriv(&mut self) -> ICACHE_REGPRIV_W<'_, PRIVCFGR3rs> {
        ICACHE_REGPRIV_W::new(self, 6)
    }
    ///Bit 11 - privileged access mode for AES
    #[inline(always)]
    pub fn aespriv(&mut self) -> AESPRIV_W<'_, PRIVCFGR3rs> {
        AESPRIV_W::new(self, 11)
    }
    ///Bit 12 - privileged access mode for HASH
    #[inline(always)]
    pub fn hashpriv(&mut self) -> HASHPRIV_W<'_, PRIVCFGR3rs> {
        HASHPRIV_W::new(self, 12)
    }
    ///Bit 13 - privileged access mode for RNG
    #[inline(always)]
    pub fn rngpriv(&mut self) -> RNGPRIV_W<'_, PRIVCFGR3rs> {
        RNGPRIV_W::new(self, 13)
    }
    ///Bit 14 - privileged access mode for SAES
    #[inline(always)]
    pub fn saespriv(&mut self) -> SAESPRIV_W<'_, PRIVCFGR3rs> {
        SAESPRIV_W::new(self, 14)
    }
    ///Bit 16 - privileged access mode for PKA
    #[inline(always)]
    pub fn pkapriv(&mut self) -> PKAPRIV_W<'_, PRIVCFGR3rs> {
        PKAPRIV_W::new(self, 16)
    }
    ///Bit 22 - privileged access mode for RAMCFG
    #[inline(always)]
    pub fn ramcfgpriv(&mut self) -> RAMCFGPRIV_W<'_, PRIVCFGR3rs> {
        RAMCFGPRIV_W::new(self, 22)
    }
    ///Bit 23 - privileged access mode for 2.4 GHz RADIO
    #[inline(always)]
    pub fn radiopriv(&mut self) -> RADIOPRIV_W<'_, PRIVCFGR3rs> {
        RADIOPRIV_W::new(self, 23)
    }
}
/**GTZC1 TZSC privilege configuration register 3

You can [`read`](crate::Reg::read) this register and get [`privcfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#GTZC1_TZSC:PRIVCFGR3)*/
pub struct PRIVCFGR3rs;
impl crate::RegisterSpec for PRIVCFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr3::R`](R) reader structure
impl crate::Readable for PRIVCFGR3rs {}
///`write(|w| ..)` method takes [`privcfgr3::W`](W) writer structure
impl crate::Writable for PRIVCFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR3 to value 0
impl crate::Resettable for PRIVCFGR3rs {}
