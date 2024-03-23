#[doc = "Register `FA1R` reader"]
pub type R = crate::R<FA1Rrs>;
#[doc = "Register `FA1R` writer"]
pub type W = crate::W<FA1Rrs>;
#[doc = "Field `FACT(0-27)` reader - Filter active"]
pub type FACT_R = crate::BitReader;
#[doc = "Field `FACT(0-27)` writer - Filter active"]
pub type FACT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Filter active"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `FACT0` field"]
    #[inline(always)]
    pub fn fact(&self, n: u8) -> FACT_R {
        #[allow(clippy::no_effect)]
        [(); 28][n as usize];
        FACT_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Filter active"]
    #[inline(always)]
    pub fn fact_iter(&self) -> impl Iterator<Item = FACT_R> + '_ {
        (0..28).map(move |n| FACT_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Filter active"]
    #[inline(always)]
    pub fn fact0(&self) -> FACT_R {
        FACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter active"]
    #[inline(always)]
    pub fn fact1(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter active"]
    #[inline(always)]
    pub fn fact2(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter active"]
    #[inline(always)]
    pub fn fact3(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter active"]
    #[inline(always)]
    pub fn fact4(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter active"]
    #[inline(always)]
    pub fn fact5(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter active"]
    #[inline(always)]
    pub fn fact6(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter active"]
    #[inline(always)]
    pub fn fact7(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter active"]
    #[inline(always)]
    pub fn fact8(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter active"]
    #[inline(always)]
    pub fn fact9(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter active"]
    #[inline(always)]
    pub fn fact10(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter active"]
    #[inline(always)]
    pub fn fact11(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter active"]
    #[inline(always)]
    pub fn fact12(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter active"]
    #[inline(always)]
    pub fn fact13(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Filter active"]
    #[inline(always)]
    pub fn fact14(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Filter active"]
    #[inline(always)]
    pub fn fact15(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Filter active"]
    #[inline(always)]
    pub fn fact16(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Filter active"]
    #[inline(always)]
    pub fn fact17(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Filter active"]
    #[inline(always)]
    pub fn fact18(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Filter active"]
    #[inline(always)]
    pub fn fact19(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Filter active"]
    #[inline(always)]
    pub fn fact20(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Filter active"]
    #[inline(always)]
    pub fn fact21(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Filter active"]
    #[inline(always)]
    pub fn fact22(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Filter active"]
    #[inline(always)]
    pub fn fact23(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Filter active"]
    #[inline(always)]
    pub fn fact24(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Filter active"]
    #[inline(always)]
    pub fn fact25(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Filter active"]
    #[inline(always)]
    pub fn fact26(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Filter active"]
    #[inline(always)]
    pub fn fact27(&self) -> FACT_R {
        FACT_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Filter active"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `FACT0` field"]
    #[inline(always)]
    #[must_use]
    pub fn fact(&mut self, n: u8) -> FACT_W<FA1Rrs> {
        #[allow(clippy::no_effect)]
        [(); 28][n as usize];
        FACT_W::new(self, n)
    }
    #[doc = "Bit 0 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact0(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact1(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact2(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact3(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact4(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact5(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 5)
    }
    #[doc = "Bit 6 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact6(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact7(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 7)
    }
    #[doc = "Bit 8 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact8(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 8)
    }
    #[doc = "Bit 9 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact9(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 9)
    }
    #[doc = "Bit 10 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact10(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 10)
    }
    #[doc = "Bit 11 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact11(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 11)
    }
    #[doc = "Bit 12 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact12(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 12)
    }
    #[doc = "Bit 13 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact13(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 13)
    }
    #[doc = "Bit 14 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact14(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 14)
    }
    #[doc = "Bit 15 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact15(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 15)
    }
    #[doc = "Bit 16 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact16(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 16)
    }
    #[doc = "Bit 17 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact17(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 17)
    }
    #[doc = "Bit 18 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact18(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 18)
    }
    #[doc = "Bit 19 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact19(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 19)
    }
    #[doc = "Bit 20 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact20(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 20)
    }
    #[doc = "Bit 21 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact21(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 21)
    }
    #[doc = "Bit 22 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact22(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 22)
    }
    #[doc = "Bit 23 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact23(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 23)
    }
    #[doc = "Bit 24 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact24(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 24)
    }
    #[doc = "Bit 25 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact25(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 25)
    }
    #[doc = "Bit 26 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact26(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 26)
    }
    #[doc = "Bit 27 - Filter active"]
    #[inline(always)]
    #[must_use]
    pub fn fact27(&mut self) -> FACT_W<FA1Rrs> {
        FACT_W::new(self, 27)
    }
}
#[doc = "CAN_FA1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fa1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fa1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FA1Rrs;
impl crate::RegisterSpec for FA1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fa1r::R`](R) reader structure"]
impl crate::Readable for FA1Rrs {}
#[doc = "`write(|w| ..)` method takes [`fa1r::W`](W) writer structure"]
impl crate::Writable for FA1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FA1R to value 0"]
impl crate::Resettable for FA1Rrs {
    const RESET_VALUE: u32 = 0;
}
