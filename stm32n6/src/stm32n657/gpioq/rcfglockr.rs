///Register `RCFGLOCKR` reader
pub type R = crate::R<RCFGLOCKRrs>;
///Register `RCFGLOCKR` writer
pub type W = crate::W<RCFGLOCKRrs>;
///Field `RLOCK(0-15)` reader - I/O pin y of port x resource lock
pub type RLOCK_R = crate::BitReader;
///Field `RLOCK(0-15)` writer - I/O pin y of port x resource lock
pub type RLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///I/O pin y of port x resource lock
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `RLOCK0` field.</div>
    #[inline(always)]
    pub fn rlock(&self, n: u8) -> RLOCK_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        RLOCK_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock_iter(&self) -> impl Iterator<Item = RLOCK_R> + '_ {
        (0..16).map(move |n| RLOCK_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock0(&self) -> RLOCK_R {
        RLOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock1(&self) -> RLOCK_R {
        RLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock2(&self) -> RLOCK_R {
        RLOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock3(&self) -> RLOCK_R {
        RLOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock4(&self) -> RLOCK_R {
        RLOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock5(&self) -> RLOCK_R {
        RLOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock6(&self) -> RLOCK_R {
        RLOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock7(&self) -> RLOCK_R {
        RLOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock8(&self) -> RLOCK_R {
        RLOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock9(&self) -> RLOCK_R {
        RLOCK_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock10(&self) -> RLOCK_R {
        RLOCK_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock11(&self) -> RLOCK_R {
        RLOCK_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock12(&self) -> RLOCK_R {
        RLOCK_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock13(&self) -> RLOCK_R {
        RLOCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock14(&self) -> RLOCK_R {
        RLOCK_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock15(&self) -> RLOCK_R {
        RLOCK_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCFGLOCKR")
            .field("rlock0", &self.rlock0())
            .field("rlock1", &self.rlock1())
            .field("rlock2", &self.rlock2())
            .field("rlock3", &self.rlock3())
            .field("rlock4", &self.rlock4())
            .field("rlock5", &self.rlock5())
            .field("rlock6", &self.rlock6())
            .field("rlock7", &self.rlock7())
            .field("rlock8", &self.rlock8())
            .field("rlock9", &self.rlock9())
            .field("rlock10", &self.rlock10())
            .field("rlock11", &self.rlock11())
            .field("rlock12", &self.rlock12())
            .field("rlock13", &self.rlock13())
            .field("rlock14", &self.rlock14())
            .field("rlock15", &self.rlock15())
            .finish()
    }
}
impl W {
    ///I/O pin y of port x resource lock
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `RLOCK0` field.</div>
    #[inline(always)]
    pub fn rlock(&mut self, n: u8) -> RLOCK_W<'_, RCFGLOCKRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        RLOCK_W::new(self, n)
    }
    ///Bit 0 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock0(&mut self) -> RLOCK_W<'_, RCFGLOCKRrs> {
        RLOCK_W::new(self, 0)
    }
    ///Bit 1 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock1(&mut self) -> RLOCK_W<'_, RCFGLOCKRrs> {
        RLOCK_W::new(self, 1)
    }
    ///Bit 2 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock2(&mut self) -> RLOCK_W<'_, RCFGLOCKRrs> {
        RLOCK_W::new(self, 2)
    }
    ///Bit 3 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock3(&mut self) -> RLOCK_W<'_, RCFGLOCKRrs> {
        RLOCK_W::new(self, 3)
    }
    ///Bit 4 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock4(&mut self) -> RLOCK_W<'_, RCFGLOCKRrs> {
        RLOCK_W::new(self, 4)
    }
    ///Bit 5 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock5(&mut self) -> RLOCK_W<'_, RCFGLOCKRrs> {
        RLOCK_W::new(self, 5)
    }
    ///Bit 6 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock6(&mut self) -> RLOCK_W<'_, RCFGLOCKRrs> {
        RLOCK_W::new(self, 6)
    }
    ///Bit 7 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock7(&mut self) -> RLOCK_W<'_, RCFGLOCKRrs> {
        RLOCK_W::new(self, 7)
    }
    ///Bit 8 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock8(&mut self) -> RLOCK_W<'_, RCFGLOCKRrs> {
        RLOCK_W::new(self, 8)
    }
    ///Bit 9 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock9(&mut self) -> RLOCK_W<'_, RCFGLOCKRrs> {
        RLOCK_W::new(self, 9)
    }
    ///Bit 10 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock10(&mut self) -> RLOCK_W<'_, RCFGLOCKRrs> {
        RLOCK_W::new(self, 10)
    }
    ///Bit 11 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock11(&mut self) -> RLOCK_W<'_, RCFGLOCKRrs> {
        RLOCK_W::new(self, 11)
    }
    ///Bit 12 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock12(&mut self) -> RLOCK_W<'_, RCFGLOCKRrs> {
        RLOCK_W::new(self, 12)
    }
    ///Bit 13 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock13(&mut self) -> RLOCK_W<'_, RCFGLOCKRrs> {
        RLOCK_W::new(self, 13)
    }
    ///Bit 14 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock14(&mut self) -> RLOCK_W<'_, RCFGLOCKRrs> {
        RLOCK_W::new(self, 14)
    }
    ///Bit 15 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock15(&mut self) -> RLOCK_W<'_, RCFGLOCKRrs> {
        RLOCK_W::new(self, 15)
    }
}
/**GPIO port Q resource configuration lock register

You can [`read`](crate::Reg::read) this register and get [`rcfglockr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcfglockr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOQ:RCFGLOCKR)*/
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
