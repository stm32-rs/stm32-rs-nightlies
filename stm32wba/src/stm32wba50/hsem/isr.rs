///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Register `ISR` writer
pub type W = crate::W<ISRrs>;
///Field `ISF(0-15)` reader - Interrupt semaphore %s status bit before enable (mask)
pub type ISF_R = crate::BitReader;
///Field `ISF(0-15)` writer - Interrupt semaphore %s status bit before enable (mask)
pub type ISF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Interrupt semaphore (0-15) status bit before enable (mask)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ISF0` field.</div>
    #[inline(always)]
    pub fn isf(&self, n: u8) -> ISF_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        ISF_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Interrupt semaphore (0-15) status bit before enable (mask)
    #[inline(always)]
    pub fn isf_iter(&self) -> impl Iterator<Item = ISF_R> + '_ {
        (0..16).map(move |n| ISF_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Interrupt semaphore 0 status bit before enable (mask)
    #[inline(always)]
    pub fn isf0(&self) -> ISF_R {
        ISF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt semaphore 1 status bit before enable (mask)
    #[inline(always)]
    pub fn isf1(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt semaphore 2 status bit before enable (mask)
    #[inline(always)]
    pub fn isf2(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt semaphore 3 status bit before enable (mask)
    #[inline(always)]
    pub fn isf3(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Interrupt semaphore 4 status bit before enable (mask)
    #[inline(always)]
    pub fn isf4(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Interrupt semaphore 5 status bit before enable (mask)
    #[inline(always)]
    pub fn isf5(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Interrupt semaphore 6 status bit before enable (mask)
    #[inline(always)]
    pub fn isf6(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Interrupt semaphore 7 status bit before enable (mask)
    #[inline(always)]
    pub fn isf7(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Interrupt semaphore 8 status bit before enable (mask)
    #[inline(always)]
    pub fn isf8(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Interrupt semaphore 9 status bit before enable (mask)
    #[inline(always)]
    pub fn isf9(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Interrupt semaphore 10 status bit before enable (mask)
    #[inline(always)]
    pub fn isf10(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Interrupt semaphore 11 status bit before enable (mask)
    #[inline(always)]
    pub fn isf11(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Interrupt semaphore 12 status bit before enable (mask)
    #[inline(always)]
    pub fn isf12(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Interrupt semaphore 13 status bit before enable (mask)
    #[inline(always)]
    pub fn isf13(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Interrupt semaphore 14 status bit before enable (mask)
    #[inline(always)]
    pub fn isf14(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Interrupt semaphore 15 status bit before enable (mask)
    #[inline(always)]
    pub fn isf15(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("isf0", &self.isf0())
            .field("isf1", &self.isf1())
            .field("isf2", &self.isf2())
            .field("isf3", &self.isf3())
            .field("isf4", &self.isf4())
            .field("isf5", &self.isf5())
            .field("isf6", &self.isf6())
            .field("isf7", &self.isf7())
            .field("isf8", &self.isf8())
            .field("isf9", &self.isf9())
            .field("isf10", &self.isf10())
            .field("isf11", &self.isf11())
            .field("isf12", &self.isf12())
            .field("isf13", &self.isf13())
            .field("isf14", &self.isf14())
            .field("isf15", &self.isf15())
            .finish()
    }
}
impl W {
    ///Interrupt semaphore (0-15) status bit before enable (mask)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ISF0` field.</div>
    #[inline(always)]
    pub fn isf(&mut self, n: u8) -> ISF_W<'_, ISRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        ISF_W::new(self, n)
    }
    ///Bit 0 - Interrupt semaphore 0 status bit before enable (mask)
    #[inline(always)]
    pub fn isf0(&mut self) -> ISF_W<'_, ISRrs> {
        ISF_W::new(self, 0)
    }
    ///Bit 1 - Interrupt semaphore 1 status bit before enable (mask)
    #[inline(always)]
    pub fn isf1(&mut self) -> ISF_W<'_, ISRrs> {
        ISF_W::new(self, 1)
    }
    ///Bit 2 - Interrupt semaphore 2 status bit before enable (mask)
    #[inline(always)]
    pub fn isf2(&mut self) -> ISF_W<'_, ISRrs> {
        ISF_W::new(self, 2)
    }
    ///Bit 3 - Interrupt semaphore 3 status bit before enable (mask)
    #[inline(always)]
    pub fn isf3(&mut self) -> ISF_W<'_, ISRrs> {
        ISF_W::new(self, 3)
    }
    ///Bit 4 - Interrupt semaphore 4 status bit before enable (mask)
    #[inline(always)]
    pub fn isf4(&mut self) -> ISF_W<'_, ISRrs> {
        ISF_W::new(self, 4)
    }
    ///Bit 5 - Interrupt semaphore 5 status bit before enable (mask)
    #[inline(always)]
    pub fn isf5(&mut self) -> ISF_W<'_, ISRrs> {
        ISF_W::new(self, 5)
    }
    ///Bit 6 - Interrupt semaphore 6 status bit before enable (mask)
    #[inline(always)]
    pub fn isf6(&mut self) -> ISF_W<'_, ISRrs> {
        ISF_W::new(self, 6)
    }
    ///Bit 7 - Interrupt semaphore 7 status bit before enable (mask)
    #[inline(always)]
    pub fn isf7(&mut self) -> ISF_W<'_, ISRrs> {
        ISF_W::new(self, 7)
    }
    ///Bit 8 - Interrupt semaphore 8 status bit before enable (mask)
    #[inline(always)]
    pub fn isf8(&mut self) -> ISF_W<'_, ISRrs> {
        ISF_W::new(self, 8)
    }
    ///Bit 9 - Interrupt semaphore 9 status bit before enable (mask)
    #[inline(always)]
    pub fn isf9(&mut self) -> ISF_W<'_, ISRrs> {
        ISF_W::new(self, 9)
    }
    ///Bit 10 - Interrupt semaphore 10 status bit before enable (mask)
    #[inline(always)]
    pub fn isf10(&mut self) -> ISF_W<'_, ISRrs> {
        ISF_W::new(self, 10)
    }
    ///Bit 11 - Interrupt semaphore 11 status bit before enable (mask)
    #[inline(always)]
    pub fn isf11(&mut self) -> ISF_W<'_, ISRrs> {
        ISF_W::new(self, 11)
    }
    ///Bit 12 - Interrupt semaphore 12 status bit before enable (mask)
    #[inline(always)]
    pub fn isf12(&mut self) -> ISF_W<'_, ISRrs> {
        ISF_W::new(self, 12)
    }
    ///Bit 13 - Interrupt semaphore 13 status bit before enable (mask)
    #[inline(always)]
    pub fn isf13(&mut self) -> ISF_W<'_, ISRrs> {
        ISF_W::new(self, 13)
    }
    ///Bit 14 - Interrupt semaphore 14 status bit before enable (mask)
    #[inline(always)]
    pub fn isf14(&mut self) -> ISF_W<'_, ISRrs> {
        ISF_W::new(self, 14)
    }
    ///Bit 15 - Interrupt semaphore 15 status bit before enable (mask)
    #[inline(always)]
    pub fn isf15(&mut self) -> ISF_W<'_, ISRrs> {
        ISF_W::new(self, 15)
    }
}
/**HSEM non-secure interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#HSEM:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`write(|w| ..)` method takes [`isr::W`](W) writer structure
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
