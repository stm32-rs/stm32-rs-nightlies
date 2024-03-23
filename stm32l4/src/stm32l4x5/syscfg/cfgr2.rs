#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2rs>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2rs>;
#[doc = "Field `CLL` writer - Cortex LOCKUP (Hardfault) output enable bit"]
pub type CLL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPL` writer - SRAM2 parity lock bit"]
pub type SPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDL` writer - PVD lock enable bit"]
pub type PVDL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCL` writer - ECC Lock"]
pub type ECCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPF` reader - SRAM2 parity error flag"]
pub type SPF_R = crate::BitReader;
#[doc = "Field `SPF` writer - SRAM2 parity error flag"]
pub type SPF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - SRAM2 parity error flag"]
    #[inline(always)]
    pub fn spf(&self) -> SPF_R {
        SPF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cortex LOCKUP (Hardfault) output enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cll(&mut self) -> CLL_W<CFGR2rs> {
        CLL_W::new(self, 0)
    }
    #[doc = "Bit 1 - SRAM2 parity lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn spl(&mut self) -> SPL_W<CFGR2rs> {
        SPL_W::new(self, 1)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pvdl(&mut self) -> PVDL_W<CFGR2rs> {
        PVDL_W::new(self, 2)
    }
    #[doc = "Bit 3 - ECC Lock"]
    #[inline(always)]
    #[must_use]
    pub fn eccl(&mut self) -> ECCL_W<CFGR2rs> {
        ECCL_W::new(self, 3)
    }
    #[doc = "Bit 8 - SRAM2 parity error flag"]
    #[inline(always)]
    #[must_use]
    pub fn spf(&mut self) -> SPF_W<CFGR2rs> {
        SPF_W::new(self, 8)
    }
}
#[doc = "CFGR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
