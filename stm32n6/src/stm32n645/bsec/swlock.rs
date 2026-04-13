///Register `SWLOCK%s` reader
pub type R = crate::R<SWLOCKrs>;
///Register `SWLOCK%s` writer
pub type W = crate::W<SWLOCKrs>;
///Field `SWLOCK(0-31)` reader - sticky write lock for shadow register %s
pub type SWLOCK_R = crate::BitReader;
///Field `SWLOCK(0-31)` writer - sticky write lock for shadow register %s
pub type SWLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///sticky write lock for shadow register (0-31)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SWLOCK0` field.</div>
    #[inline(always)]
    pub fn swlock(&self, n: u8) -> SWLOCK_R {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        SWLOCK_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///sticky write lock for shadow register (0-31)
    #[inline(always)]
    pub fn swlock_iter(&self) -> impl Iterator<Item = SWLOCK_R> + '_ {
        (0..32).map(move |n| SWLOCK_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - sticky write lock for shadow register 0
    #[inline(always)]
    pub fn swlock0(&self) -> SWLOCK_R {
        SWLOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - sticky write lock for shadow register 1
    #[inline(always)]
    pub fn swlock1(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - sticky write lock for shadow register 2
    #[inline(always)]
    pub fn swlock2(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - sticky write lock for shadow register 3
    #[inline(always)]
    pub fn swlock3(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - sticky write lock for shadow register 4
    #[inline(always)]
    pub fn swlock4(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - sticky write lock for shadow register 5
    #[inline(always)]
    pub fn swlock5(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - sticky write lock for shadow register 6
    #[inline(always)]
    pub fn swlock6(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - sticky write lock for shadow register 7
    #[inline(always)]
    pub fn swlock7(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - sticky write lock for shadow register 8
    #[inline(always)]
    pub fn swlock8(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - sticky write lock for shadow register 9
    #[inline(always)]
    pub fn swlock9(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - sticky write lock for shadow register 10
    #[inline(always)]
    pub fn swlock10(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - sticky write lock for shadow register 11
    #[inline(always)]
    pub fn swlock11(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - sticky write lock for shadow register 12
    #[inline(always)]
    pub fn swlock12(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - sticky write lock for shadow register 13
    #[inline(always)]
    pub fn swlock13(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - sticky write lock for shadow register 14
    #[inline(always)]
    pub fn swlock14(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - sticky write lock for shadow register 15
    #[inline(always)]
    pub fn swlock15(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - sticky write lock for shadow register 16
    #[inline(always)]
    pub fn swlock16(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - sticky write lock for shadow register 17
    #[inline(always)]
    pub fn swlock17(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - sticky write lock for shadow register 18
    #[inline(always)]
    pub fn swlock18(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - sticky write lock for shadow register 19
    #[inline(always)]
    pub fn swlock19(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - sticky write lock for shadow register 20
    #[inline(always)]
    pub fn swlock20(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - sticky write lock for shadow register 21
    #[inline(always)]
    pub fn swlock21(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - sticky write lock for shadow register 22
    #[inline(always)]
    pub fn swlock22(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - sticky write lock for shadow register 23
    #[inline(always)]
    pub fn swlock23(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - sticky write lock for shadow register 24
    #[inline(always)]
    pub fn swlock24(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - sticky write lock for shadow register 25
    #[inline(always)]
    pub fn swlock25(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - sticky write lock for shadow register 26
    #[inline(always)]
    pub fn swlock26(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - sticky write lock for shadow register 27
    #[inline(always)]
    pub fn swlock27(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - sticky write lock for shadow register 28
    #[inline(always)]
    pub fn swlock28(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - sticky write lock for shadow register 29
    #[inline(always)]
    pub fn swlock29(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - sticky write lock for shadow register 30
    #[inline(always)]
    pub fn swlock30(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - sticky write lock for shadow register 31
    #[inline(always)]
    pub fn swlock31(&self) -> SWLOCK_R {
        SWLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWLOCK")
            .field("swlock0", &self.swlock0())
            .field("swlock1", &self.swlock1())
            .field("swlock2", &self.swlock2())
            .field("swlock3", &self.swlock3())
            .field("swlock4", &self.swlock4())
            .field("swlock5", &self.swlock5())
            .field("swlock6", &self.swlock6())
            .field("swlock7", &self.swlock7())
            .field("swlock8", &self.swlock8())
            .field("swlock9", &self.swlock9())
            .field("swlock10", &self.swlock10())
            .field("swlock11", &self.swlock11())
            .field("swlock12", &self.swlock12())
            .field("swlock13", &self.swlock13())
            .field("swlock14", &self.swlock14())
            .field("swlock15", &self.swlock15())
            .field("swlock16", &self.swlock16())
            .field("swlock17", &self.swlock17())
            .field("swlock18", &self.swlock18())
            .field("swlock19", &self.swlock19())
            .field("swlock20", &self.swlock20())
            .field("swlock21", &self.swlock21())
            .field("swlock22", &self.swlock22())
            .field("swlock23", &self.swlock23())
            .field("swlock24", &self.swlock24())
            .field("swlock25", &self.swlock25())
            .field("swlock26", &self.swlock26())
            .field("swlock27", &self.swlock27())
            .field("swlock28", &self.swlock28())
            .field("swlock29", &self.swlock29())
            .field("swlock30", &self.swlock30())
            .field("swlock31", &self.swlock31())
            .finish()
    }
}
impl W {
    ///sticky write lock for shadow register (0-31)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SWLOCK0` field.</div>
    #[inline(always)]
    pub fn swlock(&mut self, n: u8) -> SWLOCK_W<'_, SWLOCKrs> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        SWLOCK_W::new(self, n)
    }
    ///Bit 0 - sticky write lock for shadow register 0
    #[inline(always)]
    pub fn swlock0(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 0)
    }
    ///Bit 1 - sticky write lock for shadow register 1
    #[inline(always)]
    pub fn swlock1(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 1)
    }
    ///Bit 2 - sticky write lock for shadow register 2
    #[inline(always)]
    pub fn swlock2(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 2)
    }
    ///Bit 3 - sticky write lock for shadow register 3
    #[inline(always)]
    pub fn swlock3(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 3)
    }
    ///Bit 4 - sticky write lock for shadow register 4
    #[inline(always)]
    pub fn swlock4(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 4)
    }
    ///Bit 5 - sticky write lock for shadow register 5
    #[inline(always)]
    pub fn swlock5(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 5)
    }
    ///Bit 6 - sticky write lock for shadow register 6
    #[inline(always)]
    pub fn swlock6(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 6)
    }
    ///Bit 7 - sticky write lock for shadow register 7
    #[inline(always)]
    pub fn swlock7(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 7)
    }
    ///Bit 8 - sticky write lock for shadow register 8
    #[inline(always)]
    pub fn swlock8(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 8)
    }
    ///Bit 9 - sticky write lock for shadow register 9
    #[inline(always)]
    pub fn swlock9(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 9)
    }
    ///Bit 10 - sticky write lock for shadow register 10
    #[inline(always)]
    pub fn swlock10(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 10)
    }
    ///Bit 11 - sticky write lock for shadow register 11
    #[inline(always)]
    pub fn swlock11(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 11)
    }
    ///Bit 12 - sticky write lock for shadow register 12
    #[inline(always)]
    pub fn swlock12(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 12)
    }
    ///Bit 13 - sticky write lock for shadow register 13
    #[inline(always)]
    pub fn swlock13(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 13)
    }
    ///Bit 14 - sticky write lock for shadow register 14
    #[inline(always)]
    pub fn swlock14(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 14)
    }
    ///Bit 15 - sticky write lock for shadow register 15
    #[inline(always)]
    pub fn swlock15(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 15)
    }
    ///Bit 16 - sticky write lock for shadow register 16
    #[inline(always)]
    pub fn swlock16(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 16)
    }
    ///Bit 17 - sticky write lock for shadow register 17
    #[inline(always)]
    pub fn swlock17(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 17)
    }
    ///Bit 18 - sticky write lock for shadow register 18
    #[inline(always)]
    pub fn swlock18(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 18)
    }
    ///Bit 19 - sticky write lock for shadow register 19
    #[inline(always)]
    pub fn swlock19(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 19)
    }
    ///Bit 20 - sticky write lock for shadow register 20
    #[inline(always)]
    pub fn swlock20(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 20)
    }
    ///Bit 21 - sticky write lock for shadow register 21
    #[inline(always)]
    pub fn swlock21(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 21)
    }
    ///Bit 22 - sticky write lock for shadow register 22
    #[inline(always)]
    pub fn swlock22(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 22)
    }
    ///Bit 23 - sticky write lock for shadow register 23
    #[inline(always)]
    pub fn swlock23(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 23)
    }
    ///Bit 24 - sticky write lock for shadow register 24
    #[inline(always)]
    pub fn swlock24(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 24)
    }
    ///Bit 25 - sticky write lock for shadow register 25
    #[inline(always)]
    pub fn swlock25(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 25)
    }
    ///Bit 26 - sticky write lock for shadow register 26
    #[inline(always)]
    pub fn swlock26(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 26)
    }
    ///Bit 27 - sticky write lock for shadow register 27
    #[inline(always)]
    pub fn swlock27(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 27)
    }
    ///Bit 28 - sticky write lock for shadow register 28
    #[inline(always)]
    pub fn swlock28(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 28)
    }
    ///Bit 29 - sticky write lock for shadow register 29
    #[inline(always)]
    pub fn swlock29(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 29)
    }
    ///Bit 30 - sticky write lock for shadow register 30
    #[inline(always)]
    pub fn swlock30(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 30)
    }
    ///Bit 31 - sticky write lock for shadow register 31
    #[inline(always)]
    pub fn swlock31(&mut self) -> SWLOCK_W<'_, SWLOCKrs> {
        SWLOCK_W::new(self, 31)
    }
}
/**BSEC sticky write lock register %s

You can [`read`](crate::Reg::read) this register and get [`swlock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swlock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:SWLOCK[0])*/
pub struct SWLOCKrs;
impl crate::RegisterSpec for SWLOCKrs {
    type Ux = u32;
}
///`read()` method returns [`swlock::R`](R) reader structure
impl crate::Readable for SWLOCKrs {}
///`write(|w| ..)` method takes [`swlock::W`](W) writer structure
impl crate::Writable for SWLOCKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWLOCK%s to value 0
impl crate::Resettable for SWLOCKrs {}
