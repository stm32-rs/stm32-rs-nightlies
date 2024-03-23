#[doc = "Register `RCFGLOCKR` reader"]
pub type R = crate::R<RCFGLOCKRrs>;
#[doc = "Register `RCFGLOCKR` writer"]
pub type W = crate::W<RCFGLOCKRrs>;
#[doc = "Field `LOCK0` reader - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK0_R = crate::BitReader;
#[doc = "Field `LOCK0` writer - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK1` reader - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK1_R = crate::BitReader;
#[doc = "Field `LOCK1` writer - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK2` reader - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK2_R = crate::BitReader;
#[doc = "Field `LOCK2` writer - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK3` reader - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK3_R = crate::BitReader;
#[doc = "Field `LOCK3` writer - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK4` reader - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK4_R = crate::BitReader;
#[doc = "Field `LOCK4` writer - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK5` reader - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK5_R = crate::BitReader;
#[doc = "Field `LOCK5` writer - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK6` reader - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK6_R = crate::BitReader;
#[doc = "Field `LOCK6` writer - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK7` reader - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK7_R = crate::BitReader;
#[doc = "Field `LOCK7` writer - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
pub type LOCK7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    pub fn lock0(&self) -> LOCK0_R {
        LOCK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    pub fn lock1(&self) -> LOCK1_R {
        LOCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    pub fn lock2(&self) -> LOCK2_R {
        LOCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    pub fn lock3(&self) -> LOCK3_R {
        LOCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    pub fn lock4(&self) -> LOCK4_R {
        LOCK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    pub fn lock5(&self) -> LOCK5_R {
        LOCK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    pub fn lock6(&self) -> LOCK6_R {
        LOCK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    pub fn lock7(&self) -> LOCK7_R {
        LOCK7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    #[must_use]
    pub fn lock0(&mut self) -> LOCK0_W<RCFGLOCKRrs> {
        LOCK0_W::new(self, 0)
    }
    #[doc = "Bit 1 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    #[must_use]
    pub fn lock1(&mut self) -> LOCK1_W<RCFGLOCKRrs> {
        LOCK1_W::new(self, 1)
    }
    #[doc = "Bit 2 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    #[must_use]
    pub fn lock2(&mut self) -> LOCK2_W<RCFGLOCKRrs> {
        LOCK2_W::new(self, 2)
    }
    #[doc = "Bit 3 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    #[must_use]
    pub fn lock3(&mut self) -> LOCK3_W<RCFGLOCKRrs> {
        LOCK3_W::new(self, 3)
    }
    #[doc = "Bit 4 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    #[must_use]
    pub fn lock4(&mut self) -> LOCK4_W<RCFGLOCKRrs> {
        LOCK4_W::new(self, 4)
    }
    #[doc = "Bit 5 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    #[must_use]
    pub fn lock5(&mut self) -> LOCK5_W<RCFGLOCKRrs> {
        LOCK5_W::new(self, 5)
    }
    #[doc = "Bit 6 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    #[must_use]
    pub fn lock6(&mut self) -> LOCK6_W<RCFGLOCKRrs> {
        LOCK6_W::new(self, 6)
    }
    #[doc = "Bit 7 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset."]
    #[inline(always)]
    #[must_use]
    pub fn lock7(&mut self) -> LOCK7_W<RCFGLOCKRrs> {
        LOCK7_W::new(self, 7)
    }
}
#[doc = "GPDMA configuration lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcfglockr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcfglockr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCFGLOCKRrs;
impl crate::RegisterSpec for RCFGLOCKRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcfglockr::R`](R) reader structure"]
impl crate::Readable for RCFGLOCKRrs {}
#[doc = "`write(|w| ..)` method takes [`rcfglockr::W`](W) writer structure"]
impl crate::Writable for RCFGLOCKRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCFGLOCKR to value 0"]
impl crate::Resettable for RCFGLOCKRrs {
    const RESET_VALUE: u32 = 0;
}
