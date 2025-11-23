///Register `FA1R` reader
pub type R = crate::R<FA1Rrs>;
///Register `FA1R` writer
pub type W = crate::W<FA1Rrs>;
///Field `FACT(0-13)` reader - Filter active
pub type FACT_R = crate::BitReader;
///Field `FACT(0-13)` writer - Filter active
pub type FACT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Filter active
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FACT0` field.</div>
    #[inline(always)]
    pub fn fact(&self, n: u8) -> FACT_R {
        #[allow(clippy::no_effect)]
        [(); 14][n as usize];
        FACT_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Filter active
    #[inline(always)]
    pub fn fact_iter(&self) -> impl Iterator<Item = FACT_R> + '_ {
        (0..14).map(move |n| FACT_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Filter active
    #[inline(always)]
    pub fn fact0(&self) -> FACT_R {
        FACT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Filter active
    #[inline(always)]
    pub fn fact1(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Filter active
    #[inline(always)]
    pub fn fact2(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Filter active
    #[inline(always)]
    pub fn fact3(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Filter active
    #[inline(always)]
    pub fn fact4(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Filter active
    #[inline(always)]
    pub fn fact5(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Filter active
    #[inline(always)]
    pub fn fact6(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Filter active
    #[inline(always)]
    pub fn fact7(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Filter active
    #[inline(always)]
    pub fn fact8(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Filter active
    #[inline(always)]
    pub fn fact9(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Filter active
    #[inline(always)]
    pub fn fact10(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Filter active
    #[inline(always)]
    pub fn fact11(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Filter active
    #[inline(always)]
    pub fn fact12(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Filter active
    #[inline(always)]
    pub fn fact13(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FA1R")
            .field("fact0", &self.fact0())
            .field("fact1", &self.fact1())
            .field("fact2", &self.fact2())
            .field("fact3", &self.fact3())
            .field("fact4", &self.fact4())
            .field("fact5", &self.fact5())
            .field("fact6", &self.fact6())
            .field("fact7", &self.fact7())
            .field("fact8", &self.fact8())
            .field("fact9", &self.fact9())
            .field("fact10", &self.fact10())
            .field("fact11", &self.fact11())
            .field("fact12", &self.fact12())
            .field("fact13", &self.fact13())
            .finish()
    }
}
impl W {
    ///Filter active
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FACT0` field.</div>
    #[inline(always)]
    pub fn fact(&mut self, n: u8) -> FACT_W<'_, FA1Rrs> {
        #[allow(clippy::no_effect)]
        [(); 14][n as usize];
        FACT_W::new(self, n)
    }
    ///Bit 0 - Filter active
    #[inline(always)]
    pub fn fact0(&mut self) -> FACT_W<'_, FA1Rrs> {
        FACT_W::new(self, 0)
    }
    ///Bit 1 - Filter active
    #[inline(always)]
    pub fn fact1(&mut self) -> FACT_W<'_, FA1Rrs> {
        FACT_W::new(self, 1)
    }
    ///Bit 2 - Filter active
    #[inline(always)]
    pub fn fact2(&mut self) -> FACT_W<'_, FA1Rrs> {
        FACT_W::new(self, 2)
    }
    ///Bit 3 - Filter active
    #[inline(always)]
    pub fn fact3(&mut self) -> FACT_W<'_, FA1Rrs> {
        FACT_W::new(self, 3)
    }
    ///Bit 4 - Filter active
    #[inline(always)]
    pub fn fact4(&mut self) -> FACT_W<'_, FA1Rrs> {
        FACT_W::new(self, 4)
    }
    ///Bit 5 - Filter active
    #[inline(always)]
    pub fn fact5(&mut self) -> FACT_W<'_, FA1Rrs> {
        FACT_W::new(self, 5)
    }
    ///Bit 6 - Filter active
    #[inline(always)]
    pub fn fact6(&mut self) -> FACT_W<'_, FA1Rrs> {
        FACT_W::new(self, 6)
    }
    ///Bit 7 - Filter active
    #[inline(always)]
    pub fn fact7(&mut self) -> FACT_W<'_, FA1Rrs> {
        FACT_W::new(self, 7)
    }
    ///Bit 8 - Filter active
    #[inline(always)]
    pub fn fact8(&mut self) -> FACT_W<'_, FA1Rrs> {
        FACT_W::new(self, 8)
    }
    ///Bit 9 - Filter active
    #[inline(always)]
    pub fn fact9(&mut self) -> FACT_W<'_, FA1Rrs> {
        FACT_W::new(self, 9)
    }
    ///Bit 10 - Filter active
    #[inline(always)]
    pub fn fact10(&mut self) -> FACT_W<'_, FA1Rrs> {
        FACT_W::new(self, 10)
    }
    ///Bit 11 - Filter active
    #[inline(always)]
    pub fn fact11(&mut self) -> FACT_W<'_, FA1Rrs> {
        FACT_W::new(self, 11)
    }
    ///Bit 12 - Filter active
    #[inline(always)]
    pub fn fact12(&mut self) -> FACT_W<'_, FA1Rrs> {
        FACT_W::new(self, 12)
    }
    ///Bit 13 - Filter active
    #[inline(always)]
    pub fn fact13(&mut self) -> FACT_W<'_, FA1Rrs> {
        FACT_W::new(self, 13)
    }
}
/**filter activation register

You can [`read`](crate::Reg::read) this register and get [`fa1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fa1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F732.html#CAN1:FA1R)*/
pub struct FA1Rrs;
impl crate::RegisterSpec for FA1Rrs {
    type Ux = u32;
}
///`read()` method returns [`fa1r::R`](R) reader structure
impl crate::Readable for FA1Rrs {}
///`write(|w| ..)` method takes [`fa1r::W`](W) writer structure
impl crate::Writable for FA1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FA1R to value 0
impl crate::Resettable for FA1Rrs {}
