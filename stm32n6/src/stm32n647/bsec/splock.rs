///Register `SPLOCK%s` reader
pub type R = crate::R<SPLOCKrs>;
///Register `SPLOCK%s` writer
pub type W = crate::W<SPLOCKrs>;
///Field `SPLOCK(0-31)` reader - Sticky programming lock for word %s
pub type SPLOCK_R = crate::BitReader;
///Field `SPLOCK(0-31)` writer - Sticky programming lock for word %s
pub type SPLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Sticky programming lock for word (0-31)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SPLOCK0` field.</div>
    #[inline(always)]
    pub fn splock(&self, n: u8) -> SPLOCK_R {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        SPLOCK_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Sticky programming lock for word (0-31)
    #[inline(always)]
    pub fn splock_iter(&self) -> impl Iterator<Item = SPLOCK_R> + '_ {
        (0..32).map(move |n| SPLOCK_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Sticky programming lock for word 0
    #[inline(always)]
    pub fn splock0(&self) -> SPLOCK_R {
        SPLOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Sticky programming lock for word 1
    #[inline(always)]
    pub fn splock1(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Sticky programming lock for word 2
    #[inline(always)]
    pub fn splock2(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Sticky programming lock for word 3
    #[inline(always)]
    pub fn splock3(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Sticky programming lock for word 4
    #[inline(always)]
    pub fn splock4(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Sticky programming lock for word 5
    #[inline(always)]
    pub fn splock5(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Sticky programming lock for word 6
    #[inline(always)]
    pub fn splock6(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Sticky programming lock for word 7
    #[inline(always)]
    pub fn splock7(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Sticky programming lock for word 8
    #[inline(always)]
    pub fn splock8(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Sticky programming lock for word 9
    #[inline(always)]
    pub fn splock9(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Sticky programming lock for word 10
    #[inline(always)]
    pub fn splock10(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Sticky programming lock for word 11
    #[inline(always)]
    pub fn splock11(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Sticky programming lock for word 12
    #[inline(always)]
    pub fn splock12(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Sticky programming lock for word 13
    #[inline(always)]
    pub fn splock13(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Sticky programming lock for word 14
    #[inline(always)]
    pub fn splock14(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Sticky programming lock for word 15
    #[inline(always)]
    pub fn splock15(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Sticky programming lock for word 16
    #[inline(always)]
    pub fn splock16(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Sticky programming lock for word 17
    #[inline(always)]
    pub fn splock17(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Sticky programming lock for word 18
    #[inline(always)]
    pub fn splock18(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Sticky programming lock for word 19
    #[inline(always)]
    pub fn splock19(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Sticky programming lock for word 20
    #[inline(always)]
    pub fn splock20(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Sticky programming lock for word 21
    #[inline(always)]
    pub fn splock21(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Sticky programming lock for word 22
    #[inline(always)]
    pub fn splock22(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Sticky programming lock for word 23
    #[inline(always)]
    pub fn splock23(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Sticky programming lock for word 24
    #[inline(always)]
    pub fn splock24(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Sticky programming lock for word 25
    #[inline(always)]
    pub fn splock25(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Sticky programming lock for word 26
    #[inline(always)]
    pub fn splock26(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Sticky programming lock for word 27
    #[inline(always)]
    pub fn splock27(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Sticky programming lock for word 28
    #[inline(always)]
    pub fn splock28(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Sticky programming lock for word 29
    #[inline(always)]
    pub fn splock29(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Sticky programming lock for word 30
    #[inline(always)]
    pub fn splock30(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Sticky programming lock for word 31
    #[inline(always)]
    pub fn splock31(&self) -> SPLOCK_R {
        SPLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPLOCK")
            .field("splock0", &self.splock0())
            .field("splock1", &self.splock1())
            .field("splock2", &self.splock2())
            .field("splock3", &self.splock3())
            .field("splock4", &self.splock4())
            .field("splock5", &self.splock5())
            .field("splock6", &self.splock6())
            .field("splock7", &self.splock7())
            .field("splock8", &self.splock8())
            .field("splock9", &self.splock9())
            .field("splock10", &self.splock10())
            .field("splock11", &self.splock11())
            .field("splock12", &self.splock12())
            .field("splock13", &self.splock13())
            .field("splock14", &self.splock14())
            .field("splock15", &self.splock15())
            .field("splock16", &self.splock16())
            .field("splock17", &self.splock17())
            .field("splock18", &self.splock18())
            .field("splock19", &self.splock19())
            .field("splock20", &self.splock20())
            .field("splock21", &self.splock21())
            .field("splock22", &self.splock22())
            .field("splock23", &self.splock23())
            .field("splock24", &self.splock24())
            .field("splock25", &self.splock25())
            .field("splock26", &self.splock26())
            .field("splock27", &self.splock27())
            .field("splock28", &self.splock28())
            .field("splock29", &self.splock29())
            .field("splock30", &self.splock30())
            .field("splock31", &self.splock31())
            .finish()
    }
}
impl W {
    ///Sticky programming lock for word (0-31)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SPLOCK0` field.</div>
    #[inline(always)]
    pub fn splock(&mut self, n: u8) -> SPLOCK_W<'_, SPLOCKrs> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        SPLOCK_W::new(self, n)
    }
    ///Bit 0 - Sticky programming lock for word 0
    #[inline(always)]
    pub fn splock0(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 0)
    }
    ///Bit 1 - Sticky programming lock for word 1
    #[inline(always)]
    pub fn splock1(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 1)
    }
    ///Bit 2 - Sticky programming lock for word 2
    #[inline(always)]
    pub fn splock2(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 2)
    }
    ///Bit 3 - Sticky programming lock for word 3
    #[inline(always)]
    pub fn splock3(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 3)
    }
    ///Bit 4 - Sticky programming lock for word 4
    #[inline(always)]
    pub fn splock4(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 4)
    }
    ///Bit 5 - Sticky programming lock for word 5
    #[inline(always)]
    pub fn splock5(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 5)
    }
    ///Bit 6 - Sticky programming lock for word 6
    #[inline(always)]
    pub fn splock6(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 6)
    }
    ///Bit 7 - Sticky programming lock for word 7
    #[inline(always)]
    pub fn splock7(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 7)
    }
    ///Bit 8 - Sticky programming lock for word 8
    #[inline(always)]
    pub fn splock8(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 8)
    }
    ///Bit 9 - Sticky programming lock for word 9
    #[inline(always)]
    pub fn splock9(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 9)
    }
    ///Bit 10 - Sticky programming lock for word 10
    #[inline(always)]
    pub fn splock10(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 10)
    }
    ///Bit 11 - Sticky programming lock for word 11
    #[inline(always)]
    pub fn splock11(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 11)
    }
    ///Bit 12 - Sticky programming lock for word 12
    #[inline(always)]
    pub fn splock12(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 12)
    }
    ///Bit 13 - Sticky programming lock for word 13
    #[inline(always)]
    pub fn splock13(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 13)
    }
    ///Bit 14 - Sticky programming lock for word 14
    #[inline(always)]
    pub fn splock14(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 14)
    }
    ///Bit 15 - Sticky programming lock for word 15
    #[inline(always)]
    pub fn splock15(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 15)
    }
    ///Bit 16 - Sticky programming lock for word 16
    #[inline(always)]
    pub fn splock16(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 16)
    }
    ///Bit 17 - Sticky programming lock for word 17
    #[inline(always)]
    pub fn splock17(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 17)
    }
    ///Bit 18 - Sticky programming lock for word 18
    #[inline(always)]
    pub fn splock18(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 18)
    }
    ///Bit 19 - Sticky programming lock for word 19
    #[inline(always)]
    pub fn splock19(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 19)
    }
    ///Bit 20 - Sticky programming lock for word 20
    #[inline(always)]
    pub fn splock20(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 20)
    }
    ///Bit 21 - Sticky programming lock for word 21
    #[inline(always)]
    pub fn splock21(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 21)
    }
    ///Bit 22 - Sticky programming lock for word 22
    #[inline(always)]
    pub fn splock22(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 22)
    }
    ///Bit 23 - Sticky programming lock for word 23
    #[inline(always)]
    pub fn splock23(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 23)
    }
    ///Bit 24 - Sticky programming lock for word 24
    #[inline(always)]
    pub fn splock24(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 24)
    }
    ///Bit 25 - Sticky programming lock for word 25
    #[inline(always)]
    pub fn splock25(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 25)
    }
    ///Bit 26 - Sticky programming lock for word 26
    #[inline(always)]
    pub fn splock26(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 26)
    }
    ///Bit 27 - Sticky programming lock for word 27
    #[inline(always)]
    pub fn splock27(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 27)
    }
    ///Bit 28 - Sticky programming lock for word 28
    #[inline(always)]
    pub fn splock28(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 28)
    }
    ///Bit 29 - Sticky programming lock for word 29
    #[inline(always)]
    pub fn splock29(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 29)
    }
    ///Bit 30 - Sticky programming lock for word 30
    #[inline(always)]
    pub fn splock30(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 30)
    }
    ///Bit 31 - Sticky programming lock for word 31
    #[inline(always)]
    pub fn splock31(&mut self) -> SPLOCK_W<'_, SPLOCKrs> {
        SPLOCK_W::new(self, 31)
    }
}
/**BSEC sticky programming lock register %s

You can [`read`](crate::Reg::read) this register and get [`splock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`splock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#BSEC:SPLOCK[0])*/
pub struct SPLOCKrs;
impl crate::RegisterSpec for SPLOCKrs {
    type Ux = u32;
}
///`read()` method returns [`splock::R`](R) reader structure
impl crate::Readable for SPLOCKrs {}
///`write(|w| ..)` method takes [`splock::W`](W) writer structure
impl crate::Writable for SPLOCKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPLOCK%s to value 0
impl crate::Resettable for SPLOCKrs {}
