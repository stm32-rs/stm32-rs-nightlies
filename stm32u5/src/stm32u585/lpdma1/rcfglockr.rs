///Register `RCFGLOCKR` reader
pub type R = crate::R<RCFGLOCKRrs>;
///Register `RCFGLOCKR` writer
pub type W = crate::W<RCFGLOCKRrs>;
///Field `LOCK0` reader - LOCK0
pub type LOCK0_R = crate::BitReader;
///Field `LOCK0` writer - LOCK0
pub type LOCK0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCK1` reader - LOCK1
pub type LOCK1_R = crate::BitReader;
///Field `LOCK1` writer - LOCK1
pub type LOCK1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCK2` reader - LOCK2
pub type LOCK2_R = crate::BitReader;
///Field `LOCK2` writer - LOCK2
pub type LOCK2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCK3` reader - LOCK3
pub type LOCK3_R = crate::BitReader;
///Field `LOCK3` writer - LOCK3
pub type LOCK3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LOCK0
    #[inline(always)]
    pub fn lock0(&self) -> LOCK0_R {
        LOCK0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LOCK1
    #[inline(always)]
    pub fn lock1(&self) -> LOCK1_R {
        LOCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LOCK2
    #[inline(always)]
    pub fn lock2(&self) -> LOCK2_R {
        LOCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LOCK3
    #[inline(always)]
    pub fn lock3(&self) -> LOCK3_R {
        LOCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCFGLOCKR")
            .field("lock0", &self.lock0())
            .field("lock1", &self.lock1())
            .field("lock2", &self.lock2())
            .field("lock3", &self.lock3())
            .finish()
    }
}
impl W {
    ///Bit 0 - LOCK0
    #[inline(always)]
    #[must_use]
    pub fn lock0(&mut self) -> LOCK0_W<RCFGLOCKRrs> {
        LOCK0_W::new(self, 0)
    }
    ///Bit 1 - LOCK1
    #[inline(always)]
    #[must_use]
    pub fn lock1(&mut self) -> LOCK1_W<RCFGLOCKRrs> {
        LOCK1_W::new(self, 1)
    }
    ///Bit 2 - LOCK2
    #[inline(always)]
    #[must_use]
    pub fn lock2(&mut self) -> LOCK2_W<RCFGLOCKRrs> {
        LOCK2_W::new(self, 2)
    }
    ///Bit 3 - LOCK3
    #[inline(always)]
    #[must_use]
    pub fn lock3(&mut self) -> LOCK3_W<RCFGLOCKRrs> {
        LOCK3_W::new(self, 3)
    }
}
/**LPDMA configuration lock register

You can [`read`](crate::Reg::read) this register and get [`rcfglockr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcfglockr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#LPDMA1:RCFGLOCKR)*/
pub struct RCFGLOCKRrs;
impl crate::RegisterSpec for RCFGLOCKRrs {
    type Ux = u32;
}
///`read()` method returns [`rcfglockr::R`](R) reader structure
impl crate::Readable for RCFGLOCKRrs {}
///`write(|w| ..)` method takes [`rcfglockr::W`](W) writer structure
impl crate::Writable for RCFGLOCKRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCFGLOCKR to value 0
impl crate::Resettable for RCFGLOCKRrs {
    const RESET_VALUE: u32 = 0;
}
