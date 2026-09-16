///Register `FM1R` reader
pub type R = crate::R<FM1Rrs>;
///Register `FM1R` writer
pub type W = crate::W<FM1Rrs>;
///Field `FBM(0-27)` reader - Filter mode
pub type FBM_R = crate::BitReader;
///Field `FBM(0-27)` writer - Filter mode
pub type FBM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Filter mode
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FBM0` field.</div>
    #[inline(always)]
    pub fn fbm(&self, n: u8) -> FBM_R {
        #[allow(clippy::no_effect)]
        [(); 28][n as usize];
        FBM_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Filter mode
    #[inline(always)]
    pub fn fbm_iter(&self) -> impl Iterator<Item = FBM_R> + '_ {
        (0..28).map(move |n| FBM_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Filter mode
    #[inline(always)]
    pub fn fbm0(&self) -> FBM_R {
        FBM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Filter mode
    #[inline(always)]
    pub fn fbm1(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Filter mode
    #[inline(always)]
    pub fn fbm2(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Filter mode
    #[inline(always)]
    pub fn fbm3(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Filter mode
    #[inline(always)]
    pub fn fbm4(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Filter mode
    #[inline(always)]
    pub fn fbm5(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Filter mode
    #[inline(always)]
    pub fn fbm6(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Filter mode
    #[inline(always)]
    pub fn fbm7(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Filter mode
    #[inline(always)]
    pub fn fbm8(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Filter mode
    #[inline(always)]
    pub fn fbm9(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Filter mode
    #[inline(always)]
    pub fn fbm10(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Filter mode
    #[inline(always)]
    pub fn fbm11(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Filter mode
    #[inline(always)]
    pub fn fbm12(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Filter mode
    #[inline(always)]
    pub fn fbm13(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Filter mode
    #[inline(always)]
    pub fn fbm14(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Filter mode
    #[inline(always)]
    pub fn fbm15(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Filter mode
    #[inline(always)]
    pub fn fbm16(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Filter mode
    #[inline(always)]
    pub fn fbm17(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Filter mode
    #[inline(always)]
    pub fn fbm18(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Filter mode
    #[inline(always)]
    pub fn fbm19(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Filter mode
    #[inline(always)]
    pub fn fbm20(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Filter mode
    #[inline(always)]
    pub fn fbm21(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Filter mode
    #[inline(always)]
    pub fn fbm22(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Filter mode
    #[inline(always)]
    pub fn fbm23(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Filter mode
    #[inline(always)]
    pub fn fbm24(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Filter mode
    #[inline(always)]
    pub fn fbm25(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Filter mode
    #[inline(always)]
    pub fn fbm26(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Filter mode
    #[inline(always)]
    pub fn fbm27(&self) -> FBM_R {
        FBM_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FM1R")
            .field("fbm0", &self.fbm0())
            .field("fbm1", &self.fbm1())
            .field("fbm2", &self.fbm2())
            .field("fbm3", &self.fbm3())
            .field("fbm4", &self.fbm4())
            .field("fbm5", &self.fbm5())
            .field("fbm6", &self.fbm6())
            .field("fbm7", &self.fbm7())
            .field("fbm8", &self.fbm8())
            .field("fbm9", &self.fbm9())
            .field("fbm10", &self.fbm10())
            .field("fbm11", &self.fbm11())
            .field("fbm12", &self.fbm12())
            .field("fbm13", &self.fbm13())
            .field("fbm14", &self.fbm14())
            .field("fbm15", &self.fbm15())
            .field("fbm16", &self.fbm16())
            .field("fbm17", &self.fbm17())
            .field("fbm18", &self.fbm18())
            .field("fbm19", &self.fbm19())
            .field("fbm20", &self.fbm20())
            .field("fbm21", &self.fbm21())
            .field("fbm22", &self.fbm22())
            .field("fbm23", &self.fbm23())
            .field("fbm24", &self.fbm24())
            .field("fbm25", &self.fbm25())
            .field("fbm26", &self.fbm26())
            .field("fbm27", &self.fbm27())
            .finish()
    }
}
impl W {
    ///Filter mode
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FBM0` field.</div>
    #[inline(always)]
    pub fn fbm(&mut self, n: u8) -> FBM_W<'_, FM1Rrs> {
        #[allow(clippy::no_effect)]
        [(); 28][n as usize];
        FBM_W::new(self, n)
    }
    ///Bit 0 - Filter mode
    #[inline(always)]
    pub fn fbm0(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 0)
    }
    ///Bit 1 - Filter mode
    #[inline(always)]
    pub fn fbm1(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 1)
    }
    ///Bit 2 - Filter mode
    #[inline(always)]
    pub fn fbm2(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 2)
    }
    ///Bit 3 - Filter mode
    #[inline(always)]
    pub fn fbm3(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 3)
    }
    ///Bit 4 - Filter mode
    #[inline(always)]
    pub fn fbm4(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 4)
    }
    ///Bit 5 - Filter mode
    #[inline(always)]
    pub fn fbm5(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 5)
    }
    ///Bit 6 - Filter mode
    #[inline(always)]
    pub fn fbm6(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 6)
    }
    ///Bit 7 - Filter mode
    #[inline(always)]
    pub fn fbm7(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 7)
    }
    ///Bit 8 - Filter mode
    #[inline(always)]
    pub fn fbm8(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 8)
    }
    ///Bit 9 - Filter mode
    #[inline(always)]
    pub fn fbm9(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 9)
    }
    ///Bit 10 - Filter mode
    #[inline(always)]
    pub fn fbm10(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 10)
    }
    ///Bit 11 - Filter mode
    #[inline(always)]
    pub fn fbm11(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 11)
    }
    ///Bit 12 - Filter mode
    #[inline(always)]
    pub fn fbm12(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 12)
    }
    ///Bit 13 - Filter mode
    #[inline(always)]
    pub fn fbm13(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 13)
    }
    ///Bit 14 - Filter mode
    #[inline(always)]
    pub fn fbm14(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 14)
    }
    ///Bit 15 - Filter mode
    #[inline(always)]
    pub fn fbm15(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 15)
    }
    ///Bit 16 - Filter mode
    #[inline(always)]
    pub fn fbm16(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 16)
    }
    ///Bit 17 - Filter mode
    #[inline(always)]
    pub fn fbm17(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 17)
    }
    ///Bit 18 - Filter mode
    #[inline(always)]
    pub fn fbm18(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 18)
    }
    ///Bit 19 - Filter mode
    #[inline(always)]
    pub fn fbm19(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 19)
    }
    ///Bit 20 - Filter mode
    #[inline(always)]
    pub fn fbm20(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 20)
    }
    ///Bit 21 - Filter mode
    #[inline(always)]
    pub fn fbm21(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 21)
    }
    ///Bit 22 - Filter mode
    #[inline(always)]
    pub fn fbm22(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 22)
    }
    ///Bit 23 - Filter mode
    #[inline(always)]
    pub fn fbm23(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 23)
    }
    ///Bit 24 - Filter mode
    #[inline(always)]
    pub fn fbm24(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 24)
    }
    ///Bit 25 - Filter mode
    #[inline(always)]
    pub fn fbm25(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 25)
    }
    ///Bit 26 - Filter mode
    #[inline(always)]
    pub fn fbm26(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 26)
    }
    ///Bit 27 - Filter mode
    #[inline(always)]
    pub fn fbm27(&mut self) -> FBM_W<'_, FM1Rrs> {
        FBM_W::new(self, 27)
    }
}
/**filter mode register

You can [`read`](crate::Reg::read) this register and get [`fm1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fm1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#CAN1:FM1R)*/
pub struct FM1Rrs;
impl crate::RegisterSpec for FM1Rrs {
    type Ux = u32;
}
///`read()` method returns [`fm1r::R`](R) reader structure
impl crate::Readable for FM1Rrs {}
///`write(|w| ..)` method takes [`fm1r::W`](W) writer structure
impl crate::Writable for FM1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FM1R to value 0
impl crate::Resettable for FM1Rrs {}
