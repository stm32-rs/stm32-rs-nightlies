///Register `RCFGLOCKR` reader
pub type R = crate::R<RCFGLOCKRrs>;
///Register `RCFGLOCKR` writer
pub type W = crate::W<RCFGLOCKRrs>;
///Field `RLOCK0` reader - I/O pin y of port x resource lock
pub type RLOCK0_R = crate::BitReader;
///Field `RLOCK0` writer - I/O pin y of port x resource lock
pub type RLOCK0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RLOCK1` reader - I/O pin y of port x resource lock
pub type RLOCK1_R = crate::BitReader;
///Field `RLOCK1` writer - I/O pin y of port x resource lock
pub type RLOCK1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RLOCK2` reader - I/O pin y of port x resource lock
pub type RLOCK2_R = crate::BitReader;
///Field `RLOCK2` writer - I/O pin y of port x resource lock
pub type RLOCK2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RLOCK3` reader - I/O pin y of port x resource lock
pub type RLOCK3_R = crate::BitReader;
///Field `RLOCK3` writer - I/O pin y of port x resource lock
pub type RLOCK3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RLOCK4` reader - I/O pin y of port x resource lock
pub type RLOCK4_R = crate::BitReader;
///Field `RLOCK4` writer - I/O pin y of port x resource lock
pub type RLOCK4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RLOCK5` reader - I/O pin y of port x resource lock
pub type RLOCK5_R = crate::BitReader;
///Field `RLOCK5` writer - I/O pin y of port x resource lock
pub type RLOCK5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RLOCK6` reader - I/O pin y of port x resource lock
pub type RLOCK6_R = crate::BitReader;
///Field `RLOCK6` writer - I/O pin y of port x resource lock
pub type RLOCK6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RLOCK7` reader - I/O pin y of port x resource lock
pub type RLOCK7_R = crate::BitReader;
///Field `RLOCK7` writer - I/O pin y of port x resource lock
pub type RLOCK7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RLOCK8` reader - I/O pin y of port x resource lock
pub type RLOCK8_R = crate::BitReader;
///Field `RLOCK8` writer - I/O pin y of port x resource lock
pub type RLOCK8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RLOCK9` reader - I/O pin y of port x resource lock
pub type RLOCK9_R = crate::BitReader;
///Field `RLOCK9` writer - I/O pin y of port x resource lock
pub type RLOCK9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RLOCK10` reader - I/O pin y of port x resource lock
pub type RLOCK10_R = crate::BitReader;
///Field `RLOCK10` writer - I/O pin y of port x resource lock
pub type RLOCK10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RLOCK11` reader - I/O pin y of port x resource lock
pub type RLOCK11_R = crate::BitReader;
///Field `RLOCK11` writer - I/O pin y of port x resource lock
pub type RLOCK11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RLOCK12` reader - I/O pin y of port x resource lock
pub type RLOCK12_R = crate::BitReader;
///Field `RLOCK12` writer - I/O pin y of port x resource lock
pub type RLOCK12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RLOCK13` reader - I/O pin y of port x resource lock
pub type RLOCK13_R = crate::BitReader;
///Field `RLOCK13` writer - I/O pin y of port x resource lock
pub type RLOCK13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RLOCK14` reader - I/O pin y of port x resource lock
pub type RLOCK14_R = crate::BitReader;
///Field `RLOCK14` writer - I/O pin y of port x resource lock
pub type RLOCK14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RLOCK15` reader - I/O pin y of port x resource lock
pub type RLOCK15_R = crate::BitReader;
///Field `RLOCK15` writer - I/O pin y of port x resource lock
pub type RLOCK15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock0(&self) -> RLOCK0_R {
        RLOCK0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock1(&self) -> RLOCK1_R {
        RLOCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock2(&self) -> RLOCK2_R {
        RLOCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock3(&self) -> RLOCK3_R {
        RLOCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock4(&self) -> RLOCK4_R {
        RLOCK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock5(&self) -> RLOCK5_R {
        RLOCK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock6(&self) -> RLOCK6_R {
        RLOCK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock7(&self) -> RLOCK7_R {
        RLOCK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock8(&self) -> RLOCK8_R {
        RLOCK8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock9(&self) -> RLOCK9_R {
        RLOCK9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock10(&self) -> RLOCK10_R {
        RLOCK10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock11(&self) -> RLOCK11_R {
        RLOCK11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock12(&self) -> RLOCK12_R {
        RLOCK12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock13(&self) -> RLOCK13_R {
        RLOCK13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock14(&self) -> RLOCK14_R {
        RLOCK14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock15(&self) -> RLOCK15_R {
        RLOCK15_R::new(((self.bits >> 15) & 1) != 0)
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
    ///Bit 0 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock0(&mut self) -> RLOCK0_W<RCFGLOCKRrs> {
        RLOCK0_W::new(self, 0)
    }
    ///Bit 1 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock1(&mut self) -> RLOCK1_W<RCFGLOCKRrs> {
        RLOCK1_W::new(self, 1)
    }
    ///Bit 2 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock2(&mut self) -> RLOCK2_W<RCFGLOCKRrs> {
        RLOCK2_W::new(self, 2)
    }
    ///Bit 3 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock3(&mut self) -> RLOCK3_W<RCFGLOCKRrs> {
        RLOCK3_W::new(self, 3)
    }
    ///Bit 4 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock4(&mut self) -> RLOCK4_W<RCFGLOCKRrs> {
        RLOCK4_W::new(self, 4)
    }
    ///Bit 5 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock5(&mut self) -> RLOCK5_W<RCFGLOCKRrs> {
        RLOCK5_W::new(self, 5)
    }
    ///Bit 6 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock6(&mut self) -> RLOCK6_W<RCFGLOCKRrs> {
        RLOCK6_W::new(self, 6)
    }
    ///Bit 7 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock7(&mut self) -> RLOCK7_W<RCFGLOCKRrs> {
        RLOCK7_W::new(self, 7)
    }
    ///Bit 8 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock8(&mut self) -> RLOCK8_W<RCFGLOCKRrs> {
        RLOCK8_W::new(self, 8)
    }
    ///Bit 9 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock9(&mut self) -> RLOCK9_W<RCFGLOCKRrs> {
        RLOCK9_W::new(self, 9)
    }
    ///Bit 10 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock10(&mut self) -> RLOCK10_W<RCFGLOCKRrs> {
        RLOCK10_W::new(self, 10)
    }
    ///Bit 11 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock11(&mut self) -> RLOCK11_W<RCFGLOCKRrs> {
        RLOCK11_W::new(self, 11)
    }
    ///Bit 12 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock12(&mut self) -> RLOCK12_W<RCFGLOCKRrs> {
        RLOCK12_W::new(self, 12)
    }
    ///Bit 13 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock13(&mut self) -> RLOCK13_W<RCFGLOCKRrs> {
        RLOCK13_W::new(self, 13)
    }
    ///Bit 14 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock14(&mut self) -> RLOCK14_W<RCFGLOCKRrs> {
        RLOCK14_W::new(self, 14)
    }
    ///Bit 15 - I/O pin y of port x resource lock
    #[inline(always)]
    pub fn rlock15(&mut self) -> RLOCK15_W<RCFGLOCKRrs> {
        RLOCK15_W::new(self, 15)
    }
}
/**GPIO port C resource configuration lock register

You can [`read`](crate::Reg::read) this register and get [`rcfglockr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcfglockr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOC:RCFGLOCKR)*/
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
