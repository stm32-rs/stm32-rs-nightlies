///Register `LCKR` reader
pub type R = crate::R<LCKRrs>;
///Register `LCKR` writer
pub type W = crate::W<LCKRrs>;
///Field `LCK13` reader - Port C lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOC SECy.
pub type LCK13_R = crate::BitReader;
///Field `LCK13` writer - Port C lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOC SECy.
pub type LCK13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCK14` reader - Port C lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOC SECy.
pub type LCK14_R = crate::BitReader;
///Field `LCK14` writer - Port C lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOC SECy.
pub type LCK14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCK15` reader - Port C lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOC SECy.
pub type LCK15_R = crate::BitReader;
///Field `LCK15` writer - Port C lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOC SECy.
pub type LCK15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCKK` reader - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. Access is protected by any GPIOC SECy. - LOCK key write sequence: WR LCKR\[16\] = 1 + LCKR\[15:13\] WR LCKR\[16\] = 0 + LCKR\[15:13\] WR LCKR\[16\] = 1 + LCKR\[15:13\] - LOCK key read RD LCKR\[16\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\[15:13\] must not change. Note: Any error in the lock sequence aborts the LOCK. Note: After the first LOCK sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset.
pub type LCKK_R = crate::BitReader;
///Field `LCKK` writer - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. Access is protected by any GPIOC SECy. - LOCK key write sequence: WR LCKR\[16\] = 1 + LCKR\[15:13\] WR LCKR\[16\] = 0 + LCKR\[15:13\] WR LCKR\[16\] = 1 + LCKR\[15:13\] - LOCK key read RD LCKR\[16\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\[15:13\] must not change. Note: Any error in the lock sequence aborts the LOCK. Note: After the first LOCK sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset.
pub type LCKK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 13 - Port C lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOC SECy.
    #[inline(always)]
    pub fn lck13(&self) -> LCK13_R {
        LCK13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port C lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOC SECy.
    #[inline(always)]
    pub fn lck14(&self) -> LCK14_R {
        LCK14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port C lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOC SECy.
    #[inline(always)]
    pub fn lck15(&self) -> LCK15_R {
        LCK15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. Access is protected by any GPIOC SECy. - LOCK key write sequence: WR LCKR\[16\] = 1 + LCKR\[15:13\] WR LCKR\[16\] = 0 + LCKR\[15:13\] WR LCKR\[16\] = 1 + LCKR\[15:13\] - LOCK key read RD LCKR\[16\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\[15:13\] must not change. Note: Any error in the lock sequence aborts the LOCK. Note: After the first LOCK sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset.
    #[inline(always)]
    pub fn lckk(&self) -> LCKK_R {
        LCKK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCKR")
            .field("lck13", &self.lck13())
            .field("lck14", &self.lck14())
            .field("lck15", &self.lck15())
            .field("lckk", &self.lckk())
            .finish()
    }
}
impl W {
    ///Bit 13 - Port C lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOC SECy.
    #[inline(always)]
    pub fn lck13(&mut self) -> LCK13_W<LCKRrs> {
        LCK13_W::new(self, 13)
    }
    ///Bit 14 - Port C lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOC SECy.
    #[inline(always)]
    pub fn lck14(&mut self) -> LCK14_W<LCKRrs> {
        LCK14_W::new(self, 14)
    }
    ///Bit 15 - Port C lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOC SECy.
    #[inline(always)]
    pub fn lck15(&mut self) -> LCK15_W<LCKRrs> {
        LCK15_W::new(self, 15)
    }
    ///Bit 16 - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. Access is protected by any GPIOC SECy. - LOCK key write sequence: WR LCKR\[16\] = 1 + LCKR\[15:13\] WR LCKR\[16\] = 0 + LCKR\[15:13\] WR LCKR\[16\] = 1 + LCKR\[15:13\] - LOCK key read RD LCKR\[16\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\[15:13\] must not change. Note: Any error in the lock sequence aborts the LOCK. Note: After the first LOCK sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset.
    #[inline(always)]
    pub fn lckk(&mut self) -> LCKK_W<LCKRrs> {
        LCKK_W::new(self, 16)
    }
}
/**GPIO port C configuration lock register

You can [`read`](crate::Reg::read) this register and get [`lckr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lckr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#GPIOC:LCKR)*/
pub struct LCKRrs;
impl crate::RegisterSpec for LCKRrs {
    type Ux = u32;
}
///`read()` method returns [`lckr::R`](R) reader structure
impl crate::Readable for LCKRrs {}
///`write(|w| ..)` method takes [`lckr::W`](W) writer structure
impl crate::Writable for LCKRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LCKR to value 0
impl crate::Resettable for LCKRrs {}
