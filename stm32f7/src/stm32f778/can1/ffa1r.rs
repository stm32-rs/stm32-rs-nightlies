///Register `FFA1R` reader
pub type R = crate::R<FFA1Rrs>;
///Register `FFA1R` writer
pub type W = crate::W<FFA1Rrs>;
///Field `FFA(0-27)` reader - Filter FIFO assignment for filter %s
pub type FFA_R = crate::BitReader;
///Field `FFA(0-27)` writer - Filter FIFO assignment for filter %s
pub type FFA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Filter FIFO assignment for filter (0-27)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FFA0` field.</div>
    #[inline(always)]
    pub fn ffa(&self, n: u8) -> FFA_R {
        #[allow(clippy::no_effect)]
        [(); 28][n as usize];
        FFA_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Filter FIFO assignment for filter (0-27)
    #[inline(always)]
    pub fn ffa_iter(&self) -> impl Iterator<Item = FFA_R> + '_ {
        (0..28).map(move |n| FFA_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Filter FIFO assignment for filter 0
    #[inline(always)]
    pub fn ffa0(&self) -> FFA_R {
        FFA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Filter FIFO assignment for filter 1
    #[inline(always)]
    pub fn ffa1(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Filter FIFO assignment for filter 2
    #[inline(always)]
    pub fn ffa2(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Filter FIFO assignment for filter 3
    #[inline(always)]
    pub fn ffa3(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Filter FIFO assignment for filter 4
    #[inline(always)]
    pub fn ffa4(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Filter FIFO assignment for filter 5
    #[inline(always)]
    pub fn ffa5(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Filter FIFO assignment for filter 6
    #[inline(always)]
    pub fn ffa6(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Filter FIFO assignment for filter 7
    #[inline(always)]
    pub fn ffa7(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Filter FIFO assignment for filter 8
    #[inline(always)]
    pub fn ffa8(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Filter FIFO assignment for filter 9
    #[inline(always)]
    pub fn ffa9(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Filter FIFO assignment for filter 10
    #[inline(always)]
    pub fn ffa10(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Filter FIFO assignment for filter 11
    #[inline(always)]
    pub fn ffa11(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Filter FIFO assignment for filter 12
    #[inline(always)]
    pub fn ffa12(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Filter FIFO assignment for filter 13
    #[inline(always)]
    pub fn ffa13(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Filter FIFO assignment for filter 14
    #[inline(always)]
    pub fn ffa14(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Filter FIFO assignment for filter 15
    #[inline(always)]
    pub fn ffa15(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Filter FIFO assignment for filter 16
    #[inline(always)]
    pub fn ffa16(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Filter FIFO assignment for filter 17
    #[inline(always)]
    pub fn ffa17(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Filter FIFO assignment for filter 18
    #[inline(always)]
    pub fn ffa18(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Filter FIFO assignment for filter 19
    #[inline(always)]
    pub fn ffa19(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Filter FIFO assignment for filter 20
    #[inline(always)]
    pub fn ffa20(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Filter FIFO assignment for filter 21
    #[inline(always)]
    pub fn ffa21(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Filter FIFO assignment for filter 22
    #[inline(always)]
    pub fn ffa22(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Filter FIFO assignment for filter 23
    #[inline(always)]
    pub fn ffa23(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Filter FIFO assignment for filter 24
    #[inline(always)]
    pub fn ffa24(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Filter FIFO assignment for filter 25
    #[inline(always)]
    pub fn ffa25(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Filter FIFO assignment for filter 26
    #[inline(always)]
    pub fn ffa26(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Filter FIFO assignment for filter 27
    #[inline(always)]
    pub fn ffa27(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FFA1R")
            .field("ffa0", &self.ffa0())
            .field("ffa1", &self.ffa1())
            .field("ffa2", &self.ffa2())
            .field("ffa3", &self.ffa3())
            .field("ffa4", &self.ffa4())
            .field("ffa5", &self.ffa5())
            .field("ffa6", &self.ffa6())
            .field("ffa7", &self.ffa7())
            .field("ffa8", &self.ffa8())
            .field("ffa9", &self.ffa9())
            .field("ffa10", &self.ffa10())
            .field("ffa11", &self.ffa11())
            .field("ffa12", &self.ffa12())
            .field("ffa13", &self.ffa13())
            .field("ffa14", &self.ffa14())
            .field("ffa15", &self.ffa15())
            .field("ffa16", &self.ffa16())
            .field("ffa17", &self.ffa17())
            .field("ffa18", &self.ffa18())
            .field("ffa19", &self.ffa19())
            .field("ffa20", &self.ffa20())
            .field("ffa21", &self.ffa21())
            .field("ffa22", &self.ffa22())
            .field("ffa23", &self.ffa23())
            .field("ffa24", &self.ffa24())
            .field("ffa25", &self.ffa25())
            .field("ffa26", &self.ffa26())
            .field("ffa27", &self.ffa27())
            .finish()
    }
}
impl W {
    ///Filter FIFO assignment for filter (0-27)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FFA0` field.</div>
    #[inline(always)]
    pub fn ffa(&mut self, n: u8) -> FFA_W<'_, FFA1Rrs> {
        #[allow(clippy::no_effect)]
        [(); 28][n as usize];
        FFA_W::new(self, n)
    }
    ///Bit 0 - Filter FIFO assignment for filter 0
    #[inline(always)]
    pub fn ffa0(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 0)
    }
    ///Bit 1 - Filter FIFO assignment for filter 1
    #[inline(always)]
    pub fn ffa1(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 1)
    }
    ///Bit 2 - Filter FIFO assignment for filter 2
    #[inline(always)]
    pub fn ffa2(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 2)
    }
    ///Bit 3 - Filter FIFO assignment for filter 3
    #[inline(always)]
    pub fn ffa3(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 3)
    }
    ///Bit 4 - Filter FIFO assignment for filter 4
    #[inline(always)]
    pub fn ffa4(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 4)
    }
    ///Bit 5 - Filter FIFO assignment for filter 5
    #[inline(always)]
    pub fn ffa5(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 5)
    }
    ///Bit 6 - Filter FIFO assignment for filter 6
    #[inline(always)]
    pub fn ffa6(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 6)
    }
    ///Bit 7 - Filter FIFO assignment for filter 7
    #[inline(always)]
    pub fn ffa7(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 7)
    }
    ///Bit 8 - Filter FIFO assignment for filter 8
    #[inline(always)]
    pub fn ffa8(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 8)
    }
    ///Bit 9 - Filter FIFO assignment for filter 9
    #[inline(always)]
    pub fn ffa9(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 9)
    }
    ///Bit 10 - Filter FIFO assignment for filter 10
    #[inline(always)]
    pub fn ffa10(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 10)
    }
    ///Bit 11 - Filter FIFO assignment for filter 11
    #[inline(always)]
    pub fn ffa11(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 11)
    }
    ///Bit 12 - Filter FIFO assignment for filter 12
    #[inline(always)]
    pub fn ffa12(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 12)
    }
    ///Bit 13 - Filter FIFO assignment for filter 13
    #[inline(always)]
    pub fn ffa13(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 13)
    }
    ///Bit 14 - Filter FIFO assignment for filter 14
    #[inline(always)]
    pub fn ffa14(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 14)
    }
    ///Bit 15 - Filter FIFO assignment for filter 15
    #[inline(always)]
    pub fn ffa15(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 15)
    }
    ///Bit 16 - Filter FIFO assignment for filter 16
    #[inline(always)]
    pub fn ffa16(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 16)
    }
    ///Bit 17 - Filter FIFO assignment for filter 17
    #[inline(always)]
    pub fn ffa17(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 17)
    }
    ///Bit 18 - Filter FIFO assignment for filter 18
    #[inline(always)]
    pub fn ffa18(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 18)
    }
    ///Bit 19 - Filter FIFO assignment for filter 19
    #[inline(always)]
    pub fn ffa19(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 19)
    }
    ///Bit 20 - Filter FIFO assignment for filter 20
    #[inline(always)]
    pub fn ffa20(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 20)
    }
    ///Bit 21 - Filter FIFO assignment for filter 21
    #[inline(always)]
    pub fn ffa21(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 21)
    }
    ///Bit 22 - Filter FIFO assignment for filter 22
    #[inline(always)]
    pub fn ffa22(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 22)
    }
    ///Bit 23 - Filter FIFO assignment for filter 23
    #[inline(always)]
    pub fn ffa23(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 23)
    }
    ///Bit 24 - Filter FIFO assignment for filter 24
    #[inline(always)]
    pub fn ffa24(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 24)
    }
    ///Bit 25 - Filter FIFO assignment for filter 25
    #[inline(always)]
    pub fn ffa25(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 25)
    }
    ///Bit 26 - Filter FIFO assignment for filter 26
    #[inline(always)]
    pub fn ffa26(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 26)
    }
    ///Bit 27 - Filter FIFO assignment for filter 27
    #[inline(always)]
    pub fn ffa27(&mut self) -> FFA_W<'_, FFA1Rrs> {
        FFA_W::new(self, 27)
    }
}
/**filter FIFO assignment register

You can [`read`](crate::Reg::read) this register and get [`ffa1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffa1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#CAN1:FFA1R)*/
pub struct FFA1Rrs;
impl crate::RegisterSpec for FFA1Rrs {
    type Ux = u32;
}
///`read()` method returns [`ffa1r::R`](R) reader structure
impl crate::Readable for FFA1Rrs {}
///`write(|w| ..)` method takes [`ffa1r::W`](W) writer structure
impl crate::Writable for FFA1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FFA1R to value 0
impl crate::Resettable for FFA1Rrs {}
