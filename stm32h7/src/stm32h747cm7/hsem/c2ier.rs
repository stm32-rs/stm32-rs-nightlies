///Register `C2IER` reader
pub type R = crate::R<C2IERrs>;
///Register `C2IER` writer
pub type W = crate::W<C2IERrs>;
///Field `ISEM(0-31)` reader - Interrupt(2) semaphore %s enable bit
pub type ISEM_R = crate::BitReader;
///Field `ISEM(0-31)` writer - Interrupt(2) semaphore %s enable bit
pub type ISEM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Interrupt(2) semaphore (0-31) enable bit
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ISEM0` field.</div>
    #[inline(always)]
    pub fn isem(&self, n: u8) -> ISEM_R {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        ISEM_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Interrupt(2) semaphore (0-31) enable bit
    #[inline(always)]
    pub fn isem_iter(&self) -> impl Iterator<Item = ISEM_R> + '_ {
        (0..32).map(move |n| ISEM_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Interrupt(2) semaphore 0 enable bit
    #[inline(always)]
    pub fn isem0(&self) -> ISEM_R {
        ISEM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt(2) semaphore 1 enable bit
    #[inline(always)]
    pub fn isem1(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt(2) semaphore 2 enable bit
    #[inline(always)]
    pub fn isem2(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt(2) semaphore 3 enable bit
    #[inline(always)]
    pub fn isem3(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Interrupt(2) semaphore 4 enable bit
    #[inline(always)]
    pub fn isem4(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Interrupt(2) semaphore 5 enable bit
    #[inline(always)]
    pub fn isem5(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Interrupt(2) semaphore 6 enable bit
    #[inline(always)]
    pub fn isem6(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Interrupt(2) semaphore 7 enable bit
    #[inline(always)]
    pub fn isem7(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Interrupt(2) semaphore 8 enable bit
    #[inline(always)]
    pub fn isem8(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Interrupt(2) semaphore 9 enable bit
    #[inline(always)]
    pub fn isem9(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Interrupt(2) semaphore 10 enable bit
    #[inline(always)]
    pub fn isem10(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Interrupt(2) semaphore 11 enable bit
    #[inline(always)]
    pub fn isem11(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Interrupt(2) semaphore 12 enable bit
    #[inline(always)]
    pub fn isem12(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Interrupt(2) semaphore 13 enable bit
    #[inline(always)]
    pub fn isem13(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Interrupt(2) semaphore 14 enable bit
    #[inline(always)]
    pub fn isem14(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Interrupt(2) semaphore 15 enable bit
    #[inline(always)]
    pub fn isem15(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Interrupt(2) semaphore 16 enable bit
    #[inline(always)]
    pub fn isem16(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Interrupt(2) semaphore 17 enable bit
    #[inline(always)]
    pub fn isem17(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Interrupt(2) semaphore 18 enable bit
    #[inline(always)]
    pub fn isem18(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Interrupt(2) semaphore 19 enable bit
    #[inline(always)]
    pub fn isem19(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Interrupt(2) semaphore 20 enable bit
    #[inline(always)]
    pub fn isem20(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Interrupt(2) semaphore 21 enable bit
    #[inline(always)]
    pub fn isem21(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Interrupt(2) semaphore 22 enable bit
    #[inline(always)]
    pub fn isem22(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Interrupt(2) semaphore 23 enable bit
    #[inline(always)]
    pub fn isem23(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Interrupt(2) semaphore 24 enable bit
    #[inline(always)]
    pub fn isem24(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Interrupt(2) semaphore 25 enable bit
    #[inline(always)]
    pub fn isem25(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Interrupt(2) semaphore 26 enable bit
    #[inline(always)]
    pub fn isem26(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Interrupt(2) semaphore 27 enable bit
    #[inline(always)]
    pub fn isem27(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Interrupt(2) semaphore 28 enable bit
    #[inline(always)]
    pub fn isem28(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Interrupt(2) semaphore 29 enable bit
    #[inline(always)]
    pub fn isem29(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Interrupt(2) semaphore 30 enable bit
    #[inline(always)]
    pub fn isem30(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Interrupt(2) semaphore 31 enable bit
    #[inline(always)]
    pub fn isem31(&self) -> ISEM_R {
        ISEM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2IER")
            .field("isem0", &self.isem0())
            .field("isem1", &self.isem1())
            .field("isem2", &self.isem2())
            .field("isem3", &self.isem3())
            .field("isem4", &self.isem4())
            .field("isem5", &self.isem5())
            .field("isem6", &self.isem6())
            .field("isem7", &self.isem7())
            .field("isem8", &self.isem8())
            .field("isem9", &self.isem9())
            .field("isem10", &self.isem10())
            .field("isem11", &self.isem11())
            .field("isem12", &self.isem12())
            .field("isem13", &self.isem13())
            .field("isem14", &self.isem14())
            .field("isem15", &self.isem15())
            .field("isem16", &self.isem16())
            .field("isem17", &self.isem17())
            .field("isem18", &self.isem18())
            .field("isem19", &self.isem19())
            .field("isem20", &self.isem20())
            .field("isem21", &self.isem21())
            .field("isem22", &self.isem22())
            .field("isem23", &self.isem23())
            .field("isem24", &self.isem24())
            .field("isem25", &self.isem25())
            .field("isem26", &self.isem26())
            .field("isem27", &self.isem27())
            .field("isem28", &self.isem28())
            .field("isem29", &self.isem29())
            .field("isem30", &self.isem30())
            .field("isem31", &self.isem31())
            .finish()
    }
}
impl W {
    ///Interrupt(2) semaphore (0-31) enable bit
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ISEM0` field.</div>
    #[inline(always)]
    pub fn isem(&mut self, n: u8) -> ISEM_W<C2IERrs> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        ISEM_W::new(self, n)
    }
    ///Bit 0 - Interrupt(2) semaphore 0 enable bit
    #[inline(always)]
    pub fn isem0(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 0)
    }
    ///Bit 1 - Interrupt(2) semaphore 1 enable bit
    #[inline(always)]
    pub fn isem1(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 1)
    }
    ///Bit 2 - Interrupt(2) semaphore 2 enable bit
    #[inline(always)]
    pub fn isem2(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 2)
    }
    ///Bit 3 - Interrupt(2) semaphore 3 enable bit
    #[inline(always)]
    pub fn isem3(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 3)
    }
    ///Bit 4 - Interrupt(2) semaphore 4 enable bit
    #[inline(always)]
    pub fn isem4(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 4)
    }
    ///Bit 5 - Interrupt(2) semaphore 5 enable bit
    #[inline(always)]
    pub fn isem5(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 5)
    }
    ///Bit 6 - Interrupt(2) semaphore 6 enable bit
    #[inline(always)]
    pub fn isem6(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 6)
    }
    ///Bit 7 - Interrupt(2) semaphore 7 enable bit
    #[inline(always)]
    pub fn isem7(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 7)
    }
    ///Bit 8 - Interrupt(2) semaphore 8 enable bit
    #[inline(always)]
    pub fn isem8(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 8)
    }
    ///Bit 9 - Interrupt(2) semaphore 9 enable bit
    #[inline(always)]
    pub fn isem9(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 9)
    }
    ///Bit 10 - Interrupt(2) semaphore 10 enable bit
    #[inline(always)]
    pub fn isem10(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 10)
    }
    ///Bit 11 - Interrupt(2) semaphore 11 enable bit
    #[inline(always)]
    pub fn isem11(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 11)
    }
    ///Bit 12 - Interrupt(2) semaphore 12 enable bit
    #[inline(always)]
    pub fn isem12(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 12)
    }
    ///Bit 13 - Interrupt(2) semaphore 13 enable bit
    #[inline(always)]
    pub fn isem13(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 13)
    }
    ///Bit 14 - Interrupt(2) semaphore 14 enable bit
    #[inline(always)]
    pub fn isem14(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 14)
    }
    ///Bit 15 - Interrupt(2) semaphore 15 enable bit
    #[inline(always)]
    pub fn isem15(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 15)
    }
    ///Bit 16 - Interrupt(2) semaphore 16 enable bit
    #[inline(always)]
    pub fn isem16(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 16)
    }
    ///Bit 17 - Interrupt(2) semaphore 17 enable bit
    #[inline(always)]
    pub fn isem17(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 17)
    }
    ///Bit 18 - Interrupt(2) semaphore 18 enable bit
    #[inline(always)]
    pub fn isem18(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 18)
    }
    ///Bit 19 - Interrupt(2) semaphore 19 enable bit
    #[inline(always)]
    pub fn isem19(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 19)
    }
    ///Bit 20 - Interrupt(2) semaphore 20 enable bit
    #[inline(always)]
    pub fn isem20(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 20)
    }
    ///Bit 21 - Interrupt(2) semaphore 21 enable bit
    #[inline(always)]
    pub fn isem21(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 21)
    }
    ///Bit 22 - Interrupt(2) semaphore 22 enable bit
    #[inline(always)]
    pub fn isem22(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 22)
    }
    ///Bit 23 - Interrupt(2) semaphore 23 enable bit
    #[inline(always)]
    pub fn isem23(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 23)
    }
    ///Bit 24 - Interrupt(2) semaphore 24 enable bit
    #[inline(always)]
    pub fn isem24(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 24)
    }
    ///Bit 25 - Interrupt(2) semaphore 25 enable bit
    #[inline(always)]
    pub fn isem25(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 25)
    }
    ///Bit 26 - Interrupt(2) semaphore 26 enable bit
    #[inline(always)]
    pub fn isem26(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 26)
    }
    ///Bit 27 - Interrupt(2) semaphore 27 enable bit
    #[inline(always)]
    pub fn isem27(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 27)
    }
    ///Bit 28 - Interrupt(2) semaphore 28 enable bit
    #[inline(always)]
    pub fn isem28(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 28)
    }
    ///Bit 29 - Interrupt(2) semaphore 29 enable bit
    #[inline(always)]
    pub fn isem29(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 29)
    }
    ///Bit 30 - Interrupt(2) semaphore 30 enable bit
    #[inline(always)]
    pub fn isem30(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 30)
    }
    ///Bit 31 - Interrupt(2) semaphore 31 enable bit
    #[inline(always)]
    pub fn isem31(&mut self) -> ISEM_W<C2IERrs> {
        ISEM_W::new(self, 31)
    }
}
/**HSEM Interrupt enable register of Line 2

You can [`read`](crate::Reg::read) this register and get [`c2ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#HSEM:C2IER)*/
pub struct C2IERrs;
impl crate::RegisterSpec for C2IERrs {
    type Ux = u32;
}
///`read()` method returns [`c2ier::R`](R) reader structure
impl crate::Readable for C2IERrs {}
///`write(|w| ..)` method takes [`c2ier::W`](W) writer structure
impl crate::Writable for C2IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets C2IER to value 0
impl crate::Resettable for C2IERrs {
    const RESET_VALUE: u32 = 0;
}
