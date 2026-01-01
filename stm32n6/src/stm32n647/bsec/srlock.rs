///Register `SRLOCK%s` reader
pub type R = crate::R<SRLOCKrs>;
///Register `SRLOCK%s` writer
pub type W = crate::W<SRLOCKrs>;
///Field `SRLOCK(0-31)` reader - sticky reload lock for fuse word %s
pub type SRLOCK_R = crate::BitReader;
///Field `SRLOCK(0-31)` writer - sticky reload lock for fuse word %s
pub type SRLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///sticky reload lock for fuse word (0-31)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SRLOCK0` field.</div>
    #[inline(always)]
    pub fn srlock(&self, n: u8) -> SRLOCK_R {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        SRLOCK_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///sticky reload lock for fuse word (0-31)
    #[inline(always)]
    pub fn srlock_iter(&self) -> impl Iterator<Item = SRLOCK_R> + '_ {
        (0..32).map(move |n| SRLOCK_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - sticky reload lock for fuse word 0
    #[inline(always)]
    pub fn srlock0(&self) -> SRLOCK_R {
        SRLOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - sticky reload lock for fuse word 1
    #[inline(always)]
    pub fn srlock1(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - sticky reload lock for fuse word 2
    #[inline(always)]
    pub fn srlock2(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - sticky reload lock for fuse word 3
    #[inline(always)]
    pub fn srlock3(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - sticky reload lock for fuse word 4
    #[inline(always)]
    pub fn srlock4(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - sticky reload lock for fuse word 5
    #[inline(always)]
    pub fn srlock5(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - sticky reload lock for fuse word 6
    #[inline(always)]
    pub fn srlock6(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - sticky reload lock for fuse word 7
    #[inline(always)]
    pub fn srlock7(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - sticky reload lock for fuse word 8
    #[inline(always)]
    pub fn srlock8(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - sticky reload lock for fuse word 9
    #[inline(always)]
    pub fn srlock9(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - sticky reload lock for fuse word 10
    #[inline(always)]
    pub fn srlock10(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - sticky reload lock for fuse word 11
    #[inline(always)]
    pub fn srlock11(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - sticky reload lock for fuse word 12
    #[inline(always)]
    pub fn srlock12(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - sticky reload lock for fuse word 13
    #[inline(always)]
    pub fn srlock13(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - sticky reload lock for fuse word 14
    #[inline(always)]
    pub fn srlock14(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - sticky reload lock for fuse word 15
    #[inline(always)]
    pub fn srlock15(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - sticky reload lock for fuse word 16
    #[inline(always)]
    pub fn srlock16(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - sticky reload lock for fuse word 17
    #[inline(always)]
    pub fn srlock17(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - sticky reload lock for fuse word 18
    #[inline(always)]
    pub fn srlock18(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - sticky reload lock for fuse word 19
    #[inline(always)]
    pub fn srlock19(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - sticky reload lock for fuse word 20
    #[inline(always)]
    pub fn srlock20(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - sticky reload lock for fuse word 21
    #[inline(always)]
    pub fn srlock21(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - sticky reload lock for fuse word 22
    #[inline(always)]
    pub fn srlock22(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - sticky reload lock for fuse word 23
    #[inline(always)]
    pub fn srlock23(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - sticky reload lock for fuse word 24
    #[inline(always)]
    pub fn srlock24(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - sticky reload lock for fuse word 25
    #[inline(always)]
    pub fn srlock25(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - sticky reload lock for fuse word 26
    #[inline(always)]
    pub fn srlock26(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - sticky reload lock for fuse word 27
    #[inline(always)]
    pub fn srlock27(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - sticky reload lock for fuse word 28
    #[inline(always)]
    pub fn srlock28(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - sticky reload lock for fuse word 29
    #[inline(always)]
    pub fn srlock29(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - sticky reload lock for fuse word 30
    #[inline(always)]
    pub fn srlock30(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - sticky reload lock for fuse word 31
    #[inline(always)]
    pub fn srlock31(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRLOCK")
            .field("srlock0", &self.srlock0())
            .field("srlock1", &self.srlock1())
            .field("srlock2", &self.srlock2())
            .field("srlock3", &self.srlock3())
            .field("srlock4", &self.srlock4())
            .field("srlock5", &self.srlock5())
            .field("srlock6", &self.srlock6())
            .field("srlock7", &self.srlock7())
            .field("srlock8", &self.srlock8())
            .field("srlock9", &self.srlock9())
            .field("srlock10", &self.srlock10())
            .field("srlock11", &self.srlock11())
            .field("srlock12", &self.srlock12())
            .field("srlock13", &self.srlock13())
            .field("srlock14", &self.srlock14())
            .field("srlock15", &self.srlock15())
            .field("srlock16", &self.srlock16())
            .field("srlock17", &self.srlock17())
            .field("srlock18", &self.srlock18())
            .field("srlock19", &self.srlock19())
            .field("srlock20", &self.srlock20())
            .field("srlock21", &self.srlock21())
            .field("srlock22", &self.srlock22())
            .field("srlock23", &self.srlock23())
            .field("srlock24", &self.srlock24())
            .field("srlock25", &self.srlock25())
            .field("srlock26", &self.srlock26())
            .field("srlock27", &self.srlock27())
            .field("srlock28", &self.srlock28())
            .field("srlock29", &self.srlock29())
            .field("srlock30", &self.srlock30())
            .field("srlock31", &self.srlock31())
            .finish()
    }
}
impl W {
    ///sticky reload lock for fuse word (0-31)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SRLOCK0` field.</div>
    #[inline(always)]
    pub fn srlock(&mut self, n: u8) -> SRLOCK_W<'_, SRLOCKrs> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        SRLOCK_W::new(self, n)
    }
    ///Bit 0 - sticky reload lock for fuse word 0
    #[inline(always)]
    pub fn srlock0(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 0)
    }
    ///Bit 1 - sticky reload lock for fuse word 1
    #[inline(always)]
    pub fn srlock1(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 1)
    }
    ///Bit 2 - sticky reload lock for fuse word 2
    #[inline(always)]
    pub fn srlock2(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 2)
    }
    ///Bit 3 - sticky reload lock for fuse word 3
    #[inline(always)]
    pub fn srlock3(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 3)
    }
    ///Bit 4 - sticky reload lock for fuse word 4
    #[inline(always)]
    pub fn srlock4(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 4)
    }
    ///Bit 5 - sticky reload lock for fuse word 5
    #[inline(always)]
    pub fn srlock5(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 5)
    }
    ///Bit 6 - sticky reload lock for fuse word 6
    #[inline(always)]
    pub fn srlock6(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 6)
    }
    ///Bit 7 - sticky reload lock for fuse word 7
    #[inline(always)]
    pub fn srlock7(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 7)
    }
    ///Bit 8 - sticky reload lock for fuse word 8
    #[inline(always)]
    pub fn srlock8(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 8)
    }
    ///Bit 9 - sticky reload lock for fuse word 9
    #[inline(always)]
    pub fn srlock9(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 9)
    }
    ///Bit 10 - sticky reload lock for fuse word 10
    #[inline(always)]
    pub fn srlock10(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 10)
    }
    ///Bit 11 - sticky reload lock for fuse word 11
    #[inline(always)]
    pub fn srlock11(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 11)
    }
    ///Bit 12 - sticky reload lock for fuse word 12
    #[inline(always)]
    pub fn srlock12(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 12)
    }
    ///Bit 13 - sticky reload lock for fuse word 13
    #[inline(always)]
    pub fn srlock13(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 13)
    }
    ///Bit 14 - sticky reload lock for fuse word 14
    #[inline(always)]
    pub fn srlock14(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 14)
    }
    ///Bit 15 - sticky reload lock for fuse word 15
    #[inline(always)]
    pub fn srlock15(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 15)
    }
    ///Bit 16 - sticky reload lock for fuse word 16
    #[inline(always)]
    pub fn srlock16(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 16)
    }
    ///Bit 17 - sticky reload lock for fuse word 17
    #[inline(always)]
    pub fn srlock17(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 17)
    }
    ///Bit 18 - sticky reload lock for fuse word 18
    #[inline(always)]
    pub fn srlock18(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 18)
    }
    ///Bit 19 - sticky reload lock for fuse word 19
    #[inline(always)]
    pub fn srlock19(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 19)
    }
    ///Bit 20 - sticky reload lock for fuse word 20
    #[inline(always)]
    pub fn srlock20(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 20)
    }
    ///Bit 21 - sticky reload lock for fuse word 21
    #[inline(always)]
    pub fn srlock21(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 21)
    }
    ///Bit 22 - sticky reload lock for fuse word 22
    #[inline(always)]
    pub fn srlock22(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 22)
    }
    ///Bit 23 - sticky reload lock for fuse word 23
    #[inline(always)]
    pub fn srlock23(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 23)
    }
    ///Bit 24 - sticky reload lock for fuse word 24
    #[inline(always)]
    pub fn srlock24(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 24)
    }
    ///Bit 25 - sticky reload lock for fuse word 25
    #[inline(always)]
    pub fn srlock25(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 25)
    }
    ///Bit 26 - sticky reload lock for fuse word 26
    #[inline(always)]
    pub fn srlock26(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 26)
    }
    ///Bit 27 - sticky reload lock for fuse word 27
    #[inline(always)]
    pub fn srlock27(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 27)
    }
    ///Bit 28 - sticky reload lock for fuse word 28
    #[inline(always)]
    pub fn srlock28(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 28)
    }
    ///Bit 29 - sticky reload lock for fuse word 29
    #[inline(always)]
    pub fn srlock29(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 29)
    }
    ///Bit 30 - sticky reload lock for fuse word 30
    #[inline(always)]
    pub fn srlock30(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 30)
    }
    ///Bit 31 - sticky reload lock for fuse word 31
    #[inline(always)]
    pub fn srlock31(&mut self) -> SRLOCK_W<'_, SRLOCKrs> {
        SRLOCK_W::new(self, 31)
    }
}
/**BSEC sticky reload lock register %s

You can [`read`](crate::Reg::read) this register and get [`srlock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srlock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#BSEC:SRLOCK[0])*/
pub struct SRLOCKrs;
impl crate::RegisterSpec for SRLOCKrs {
    type Ux = u32;
}
///`read()` method returns [`srlock::R`](R) reader structure
impl crate::Readable for SRLOCKrs {}
///`write(|w| ..)` method takes [`srlock::W`](W) writer structure
impl crate::Writable for SRLOCKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SRLOCK%s to value 0
impl crate::Resettable for SRLOCKrs {}
