#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2rs>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2rs>;
#[doc = "Field `LOCKUP_LOCK` reader - Cortex-M0+ LOCKUP bit enable bit"]
pub type LOCKUP_LOCK_R = crate::BitReader;
#[doc = "Field `LOCKUP_LOCK` writer - Cortex-M0+ LOCKUP bit enable bit"]
pub type LOCKUP_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM_PARITY_LOCK` reader - SRAM parity lock bit"]
pub type SRAM_PARITY_LOCK_R = crate::BitReader;
#[doc = "Field `SRAM_PARITY_LOCK` writer - SRAM parity lock bit"]
pub type SRAM_PARITY_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD_LOCK` reader - PVD lock enable bit"]
pub type PVD_LOCK_R = crate::BitReader;
#[doc = "Field `PVD_LOCK` writer - PVD lock enable bit"]
pub type PVD_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_LOCK` reader - ECC error lock bit"]
pub type ECC_LOCK_R = crate::BitReader;
#[doc = "Field `ECC_LOCK` writer - ECC error lock bit"]
pub type ECC_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM_PEF` reader - SRAM parity error flag"]
pub type SRAM_PEF_R = crate::BitReader;
#[doc = "Field `SRAM_PEF` writer - SRAM parity error flag"]
pub type SRAM_PEF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Cortex-M0+ LOCKUP bit enable bit"]
    #[inline(always)]
    pub fn lockup_lock(&self) -> LOCKUP_LOCK_R {
        LOCKUP_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM parity lock bit"]
    #[inline(always)]
    pub fn sram_parity_lock(&self) -> SRAM_PARITY_LOCK_R {
        SRAM_PARITY_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvd_lock(&self) -> PVD_LOCK_R {
        PVD_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECC error lock bit"]
    #[inline(always)]
    pub fn ecc_lock(&self) -> ECC_LOCK_R {
        ECC_LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM parity error flag"]
    #[inline(always)]
    pub fn sram_pef(&self) -> SRAM_PEF_R {
        SRAM_PEF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cortex-M0+ LOCKUP bit enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn lockup_lock(&mut self) -> LOCKUP_LOCK_W<CFGR2rs> {
        LOCKUP_LOCK_W::new(self, 0)
    }
    #[doc = "Bit 1 - SRAM parity lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn sram_parity_lock(&mut self) -> SRAM_PARITY_LOCK_W<CFGR2rs> {
        SRAM_PARITY_LOCK_W::new(self, 1)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pvd_lock(&mut self) -> PVD_LOCK_W<CFGR2rs> {
        PVD_LOCK_W::new(self, 2)
    }
    #[doc = "Bit 3 - ECC error lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_lock(&mut self) -> ECC_LOCK_W<CFGR2rs> {
        ECC_LOCK_W::new(self, 3)
    }
    #[doc = "Bit 8 - SRAM parity error flag"]
    #[inline(always)]
    #[must_use]
    pub fn sram_pef(&mut self) -> SRAM_PEF_W<CFGR2rs> {
        SRAM_PEF_W::new(self, 8)
    }
}
#[doc = "SYSCFG configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
