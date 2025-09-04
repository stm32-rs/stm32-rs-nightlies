///Register `RCFGLOCKR` reader
pub type R = crate::R<RCFGLOCKRrs>;
///Register `RCFGLOCKR` writer
pub type W = crate::W<RCFGLOCKRrs>;
///Field `LOCK(0-15)` reader - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
pub type LOCK_R = crate::BitReader;
///Field `LOCK(0-15)` writer - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `LOCK0` field.</div>
    #[inline(always)]
    pub fn lock(&self, n: u8) -> LOCK_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        LOCK_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock_iter(&self) -> impl Iterator<Item = LOCK_R> + '_ {
        (0..16).map(move |n| LOCK_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock0(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock1(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock2(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock3(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock4(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock5(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock6(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock7(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock8(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock9(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock10(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock11(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock12(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock13(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock14(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock15(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 15) & 1) != 0)
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
            .field("lock8", &self.lock8())
            .field("lock9", &self.lock9())
            .field("lock10", &self.lock10())
            .field("lock11", &self.lock11())
            .field("lock12", &self.lock12())
            .field("lock13", &self.lock13())
            .field("lock14", &self.lock14())
            .field("lock15", &self.lock15())
            .finish()
    }
}
impl W {
    ///lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `LOCK0` field.</div>
    #[inline(always)]
    pub fn lock(&mut self, n: u8) -> LOCK_W<RCFGLOCKRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        LOCK_W::new(self, n)
    }
    ///Bit 0 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock0(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 0)
    }
    ///Bit 1 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock1(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 1)
    }
    ///Bit 2 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock2(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 2)
    }
    ///Bit 3 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock3(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 3)
    }
    ///Bit 4 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock4(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 4)
    }
    ///Bit 5 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock5(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 5)
    }
    ///Bit 6 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock6(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 6)
    }
    ///Bit 7 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock7(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 7)
    }
    ///Bit 8 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock8(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 8)
    }
    ///Bit 9 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock9(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 9)
    }
    ///Bit 10 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock10(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 10)
    }
    ///Bit 11 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock11(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 11)
    }
    ///Bit 12 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock12(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 12)
    }
    ///Bit 13 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock13(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 13)
    }
    ///Bit 14 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock14(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 14)
    }
    ///Bit 15 - lock the configuration of HPDMA_PRIVCFGR.PRIVx until a global HPDMA reset This bit is cleared after reset and, once set, it cannot be reset until a global HPDMA reset.
    #[inline(always)]
    pub fn lock15(&mut self) -> LOCK_W<RCFGLOCKRrs> {
        LOCK_W::new(self, 15)
    }
}
/**HPDMA configuration lock register

You can [`read`](crate::Reg::read) this register and get [`rcfglockr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcfglockr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#HPDMA:RCFGLOCKR)*/
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
