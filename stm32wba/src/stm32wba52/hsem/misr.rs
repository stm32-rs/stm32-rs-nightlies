///Register `MISR` reader
pub type R = crate::R<MISRrs>;
///Register `MISR` writer
pub type W = crate::W<MISRrs>;
///Field `MISF(0-15)` reader - Masked interrupt semaphore %s status bit after enable (mask)
pub type MISF_R = crate::BitReader;
///Field `MISF(0-15)` writer - Masked interrupt semaphore %s status bit after enable (mask)
pub type MISF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Masked interrupt semaphore (0-15) status bit after enable (mask)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MISF0` field.</div>
    #[inline(always)]
    pub fn misf(&self, n: u8) -> MISF_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        MISF_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Masked interrupt semaphore (0-15) status bit after enable (mask)
    #[inline(always)]
    pub fn misf_iter(&self) -> impl Iterator<Item = MISF_R> + '_ {
        (0..16).map(move |n| MISF_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Masked interrupt semaphore 0 status bit after enable (mask)
    #[inline(always)]
    pub fn misf0(&self) -> MISF_R {
        MISF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Masked interrupt semaphore 1 status bit after enable (mask)
    #[inline(always)]
    pub fn misf1(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Masked interrupt semaphore 2 status bit after enable (mask)
    #[inline(always)]
    pub fn misf2(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Masked interrupt semaphore 3 status bit after enable (mask)
    #[inline(always)]
    pub fn misf3(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Masked interrupt semaphore 4 status bit after enable (mask)
    #[inline(always)]
    pub fn misf4(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Masked interrupt semaphore 5 status bit after enable (mask)
    #[inline(always)]
    pub fn misf5(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Masked interrupt semaphore 6 status bit after enable (mask)
    #[inline(always)]
    pub fn misf6(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Masked interrupt semaphore 7 status bit after enable (mask)
    #[inline(always)]
    pub fn misf7(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Masked interrupt semaphore 8 status bit after enable (mask)
    #[inline(always)]
    pub fn misf8(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Masked interrupt semaphore 9 status bit after enable (mask)
    #[inline(always)]
    pub fn misf9(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Masked interrupt semaphore 10 status bit after enable (mask)
    #[inline(always)]
    pub fn misf10(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Masked interrupt semaphore 11 status bit after enable (mask)
    #[inline(always)]
    pub fn misf11(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Masked interrupt semaphore 12 status bit after enable (mask)
    #[inline(always)]
    pub fn misf12(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Masked interrupt semaphore 13 status bit after enable (mask)
    #[inline(always)]
    pub fn misf13(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Masked interrupt semaphore 14 status bit after enable (mask)
    #[inline(always)]
    pub fn misf14(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Masked interrupt semaphore 15 status bit after enable (mask)
    #[inline(always)]
    pub fn misf15(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISR")
            .field("misf0", &self.misf0())
            .field("misf1", &self.misf1())
            .field("misf2", &self.misf2())
            .field("misf3", &self.misf3())
            .field("misf4", &self.misf4())
            .field("misf5", &self.misf5())
            .field("misf6", &self.misf6())
            .field("misf7", &self.misf7())
            .field("misf8", &self.misf8())
            .field("misf9", &self.misf9())
            .field("misf10", &self.misf10())
            .field("misf11", &self.misf11())
            .field("misf12", &self.misf12())
            .field("misf13", &self.misf13())
            .field("misf14", &self.misf14())
            .field("misf15", &self.misf15())
            .finish()
    }
}
impl W {
    ///Masked interrupt semaphore (0-15) status bit after enable (mask)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MISF0` field.</div>
    #[inline(always)]
    pub fn misf(&mut self, n: u8) -> MISF_W<'_, MISRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        MISF_W::new(self, n)
    }
    ///Bit 0 - Masked interrupt semaphore 0 status bit after enable (mask)
    #[inline(always)]
    pub fn misf0(&mut self) -> MISF_W<'_, MISRrs> {
        MISF_W::new(self, 0)
    }
    ///Bit 1 - Masked interrupt semaphore 1 status bit after enable (mask)
    #[inline(always)]
    pub fn misf1(&mut self) -> MISF_W<'_, MISRrs> {
        MISF_W::new(self, 1)
    }
    ///Bit 2 - Masked interrupt semaphore 2 status bit after enable (mask)
    #[inline(always)]
    pub fn misf2(&mut self) -> MISF_W<'_, MISRrs> {
        MISF_W::new(self, 2)
    }
    ///Bit 3 - Masked interrupt semaphore 3 status bit after enable (mask)
    #[inline(always)]
    pub fn misf3(&mut self) -> MISF_W<'_, MISRrs> {
        MISF_W::new(self, 3)
    }
    ///Bit 4 - Masked interrupt semaphore 4 status bit after enable (mask)
    #[inline(always)]
    pub fn misf4(&mut self) -> MISF_W<'_, MISRrs> {
        MISF_W::new(self, 4)
    }
    ///Bit 5 - Masked interrupt semaphore 5 status bit after enable (mask)
    #[inline(always)]
    pub fn misf5(&mut self) -> MISF_W<'_, MISRrs> {
        MISF_W::new(self, 5)
    }
    ///Bit 6 - Masked interrupt semaphore 6 status bit after enable (mask)
    #[inline(always)]
    pub fn misf6(&mut self) -> MISF_W<'_, MISRrs> {
        MISF_W::new(self, 6)
    }
    ///Bit 7 - Masked interrupt semaphore 7 status bit after enable (mask)
    #[inline(always)]
    pub fn misf7(&mut self) -> MISF_W<'_, MISRrs> {
        MISF_W::new(self, 7)
    }
    ///Bit 8 - Masked interrupt semaphore 8 status bit after enable (mask)
    #[inline(always)]
    pub fn misf8(&mut self) -> MISF_W<'_, MISRrs> {
        MISF_W::new(self, 8)
    }
    ///Bit 9 - Masked interrupt semaphore 9 status bit after enable (mask)
    #[inline(always)]
    pub fn misf9(&mut self) -> MISF_W<'_, MISRrs> {
        MISF_W::new(self, 9)
    }
    ///Bit 10 - Masked interrupt semaphore 10 status bit after enable (mask)
    #[inline(always)]
    pub fn misf10(&mut self) -> MISF_W<'_, MISRrs> {
        MISF_W::new(self, 10)
    }
    ///Bit 11 - Masked interrupt semaphore 11 status bit after enable (mask)
    #[inline(always)]
    pub fn misf11(&mut self) -> MISF_W<'_, MISRrs> {
        MISF_W::new(self, 11)
    }
    ///Bit 12 - Masked interrupt semaphore 12 status bit after enable (mask)
    #[inline(always)]
    pub fn misf12(&mut self) -> MISF_W<'_, MISRrs> {
        MISF_W::new(self, 12)
    }
    ///Bit 13 - Masked interrupt semaphore 13 status bit after enable (mask)
    #[inline(always)]
    pub fn misf13(&mut self) -> MISF_W<'_, MISRrs> {
        MISF_W::new(self, 13)
    }
    ///Bit 14 - Masked interrupt semaphore 14 status bit after enable (mask)
    #[inline(always)]
    pub fn misf14(&mut self) -> MISF_W<'_, MISRrs> {
        MISF_W::new(self, 14)
    }
    ///Bit 15 - Masked interrupt semaphore 15 status bit after enable (mask)
    #[inline(always)]
    pub fn misf15(&mut self) -> MISF_W<'_, MISRrs> {
        MISF_W::new(self, 15)
    }
}
/**HSEM non-secure interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#HSEM:MISR)*/
pub struct MISRrs;
impl crate::RegisterSpec for MISRrs {
    type Ux = u32;
}
///`read()` method returns [`misr::R`](R) reader structure
impl crate::Readable for MISRrs {}
///`write(|w| ..)` method takes [`misr::W`](W) writer structure
impl crate::Writable for MISRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MISR to value 0
impl crate::Resettable for MISRrs {}
