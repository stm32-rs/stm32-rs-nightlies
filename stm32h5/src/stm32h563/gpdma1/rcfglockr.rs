///Register `RCFGLOCKR` reader
pub type R = crate::R<RCFGLOCKRrs>;
///Register `RCFGLOCKR` writer
pub type W = crate::W<RCFGLOCKRrs>;
///Field `LOCK(0-7)` reader - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset.
pub type LOCK_R = crate::BitReader;
///Field `LOCK(0-7)` writer - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset.
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset.
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `LOCK0` field.</div>
    #[inline(always)]
    pub fn lock(&self, n: u8) -> LOCK_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        LOCK_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset.
    #[inline(always)]
    pub fn lock_iter(&self) -> impl Iterator<Item = LOCK_R> + '_ {
        (0..8).map(move |n| LOCK_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset.
    #[inline(always)]
    pub fn lock0(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset.
    #[inline(always)]
    pub fn lock1(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset.
    #[inline(always)]
    pub fn lock2(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset.
    #[inline(always)]
    pub fn lock3(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset.
    #[inline(always)]
    pub fn lock4(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset.
    #[inline(always)]
    pub fn lock5(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset.
    #[inline(always)]
    pub fn lock6(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset.
    #[inline(always)]
    pub fn lock7(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCFGLOCKR")
            .field("lock0", &self.lock0())
            .field("lock1", &self.lock1())
            .field("lock2", &self.lock2())
            .field("lock3", &self.lock3())
            .field("lock4", &self.lock4())
            .field("lock5", &self.lock5())
            .field("lock6", &self.lock6())
            .field("lock7", &self.lock7())
            .finish()
    }
}
impl W {
    ///lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset.
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `LOCK0` field.</div>
    #[inline(always)]
    pub fn lock(&mut self, n: u8) -> LOCK_W<'_, RCFGLOCKRrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        LOCK_W::new(self, n)
    }
    ///Bit 0 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset.
    #[inline(always)]
    pub fn lock0(&mut self) -> LOCK_W<'_, RCFGLOCKRrs> {
        LOCK_W::new(self, 0)
    }
    ///Bit 1 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset.
    #[inline(always)]
    pub fn lock1(&mut self) -> LOCK_W<'_, RCFGLOCKRrs> {
        LOCK_W::new(self, 1)
    }
    ///Bit 2 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset.
    #[inline(always)]
    pub fn lock2(&mut self) -> LOCK_W<'_, RCFGLOCKRrs> {
        LOCK_W::new(self, 2)
    }
    ///Bit 3 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset.
    #[inline(always)]
    pub fn lock3(&mut self) -> LOCK_W<'_, RCFGLOCKRrs> {
        LOCK_W::new(self, 3)
    }
    ///Bit 4 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset.
    #[inline(always)]
    pub fn lock4(&mut self) -> LOCK_W<'_, RCFGLOCKRrs> {
        LOCK_W::new(self, 4)
    }
    ///Bit 5 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset.
    #[inline(always)]
    pub fn lock5(&mut self) -> LOCK_W<'_, RCFGLOCKRrs> {
        LOCK_W::new(self, 5)
    }
    ///Bit 6 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset.
    #[inline(always)]
    pub fn lock6(&mut self) -> LOCK_W<'_, RCFGLOCKRrs> {
        LOCK_W::new(self, 6)
    }
    ///Bit 7 - lock the configuration of GPDMA_SECCFGR.SECx and GPDMA_PRIVCFGR.PRIVx, until a global GPDMA reset (x = 7 to 0) This bit is cleared after reset and, once set, it cannot be reset until a global GPDMA reset.
    #[inline(always)]
    pub fn lock7(&mut self) -> LOCK_W<'_, RCFGLOCKRrs> {
        LOCK_W::new(self, 7)
    }
}
/**GPDMA configuration lock register

You can [`read`](crate::Reg::read) this register and get [`rcfglockr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcfglockr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:RCFGLOCKR)*/
pub struct RCFGLOCKRrs;
impl crate::RegisterSpec for RCFGLOCKRrs {
    type Ux = u32;
}
///`read()` method returns [`rcfglockr::R`](R) reader structure
impl crate::Readable for RCFGLOCKRrs {}
///`write(|w| ..)` method takes [`rcfglockr::W`](W) writer structure
impl crate::Writable for RCFGLOCKRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCFGLOCKR to value 0
impl crate::Resettable for RCFGLOCKRrs {}
