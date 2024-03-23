#[doc = "Register `FFA1R` reader"]
pub type R = crate::R<FFA1Rrs>;
#[doc = "Register `FFA1R` writer"]
pub type W = crate::W<FFA1Rrs>;
#[doc = "Field `FFA(0-13)` reader - Filter FIFO assignment for filter %s"]
pub type FFA_R = crate::BitReader;
#[doc = "Field `FFA(0-13)` writer - Filter FIFO assignment for filter %s"]
pub type FFA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Filter FIFO assignment for filter (0-13)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `FFA0` field"]
    #[inline(always)]
    pub fn ffa(&self, n: u8) -> FFA_R {
        #[allow(clippy::no_effect)]
        [(); 14][n as usize];
        FFA_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Filter FIFO assignment for filter (0-13)"]
    #[inline(always)]
    pub fn ffa_iter(&self) -> impl Iterator<Item = FFA_R> + '_ {
        (0..14).map(move |n| FFA_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Filter FIFO assignment for filter 0"]
    #[inline(always)]
    pub fn ffa0(&self) -> FFA_R {
        FFA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter FIFO assignment for filter 1"]
    #[inline(always)]
    pub fn ffa1(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter FIFO assignment for filter 2"]
    #[inline(always)]
    pub fn ffa2(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter FIFO assignment for filter 3"]
    #[inline(always)]
    pub fn ffa3(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter FIFO assignment for filter 4"]
    #[inline(always)]
    pub fn ffa4(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter FIFO assignment for filter 5"]
    #[inline(always)]
    pub fn ffa5(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter FIFO assignment for filter 6"]
    #[inline(always)]
    pub fn ffa6(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter FIFO assignment for filter 7"]
    #[inline(always)]
    pub fn ffa7(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter FIFO assignment for filter 8"]
    #[inline(always)]
    pub fn ffa8(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter FIFO assignment for filter 9"]
    #[inline(always)]
    pub fn ffa9(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter FIFO assignment for filter 10"]
    #[inline(always)]
    pub fn ffa10(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter FIFO assignment for filter 11"]
    #[inline(always)]
    pub fn ffa11(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter FIFO assignment for filter 12"]
    #[inline(always)]
    pub fn ffa12(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter FIFO assignment for filter 13"]
    #[inline(always)]
    pub fn ffa13(&self) -> FFA_R {
        FFA_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Filter FIFO assignment for filter (0-13)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `FFA0` field"]
    #[inline(always)]
    #[must_use]
    pub fn ffa(&mut self, n: u8) -> FFA_W<FFA1Rrs> {
        #[allow(clippy::no_effect)]
        [(); 14][n as usize];
        FFA_W::new(self, n)
    }
    #[doc = "Bit 0 - Filter FIFO assignment for filter 0"]
    #[inline(always)]
    #[must_use]
    pub fn ffa0(&mut self) -> FFA_W<FFA1Rrs> {
        FFA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Filter FIFO assignment for filter 1"]
    #[inline(always)]
    #[must_use]
    pub fn ffa1(&mut self) -> FFA_W<FFA1Rrs> {
        FFA_W::new(self, 1)
    }
    #[doc = "Bit 2 - Filter FIFO assignment for filter 2"]
    #[inline(always)]
    #[must_use]
    pub fn ffa2(&mut self) -> FFA_W<FFA1Rrs> {
        FFA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Filter FIFO assignment for filter 3"]
    #[inline(always)]
    #[must_use]
    pub fn ffa3(&mut self) -> FFA_W<FFA1Rrs> {
        FFA_W::new(self, 3)
    }
    #[doc = "Bit 4 - Filter FIFO assignment for filter 4"]
    #[inline(always)]
    #[must_use]
    pub fn ffa4(&mut self) -> FFA_W<FFA1Rrs> {
        FFA_W::new(self, 4)
    }
    #[doc = "Bit 5 - Filter FIFO assignment for filter 5"]
    #[inline(always)]
    #[must_use]
    pub fn ffa5(&mut self) -> FFA_W<FFA1Rrs> {
        FFA_W::new(self, 5)
    }
    #[doc = "Bit 6 - Filter FIFO assignment for filter 6"]
    #[inline(always)]
    #[must_use]
    pub fn ffa6(&mut self) -> FFA_W<FFA1Rrs> {
        FFA_W::new(self, 6)
    }
    #[doc = "Bit 7 - Filter FIFO assignment for filter 7"]
    #[inline(always)]
    #[must_use]
    pub fn ffa7(&mut self) -> FFA_W<FFA1Rrs> {
        FFA_W::new(self, 7)
    }
    #[doc = "Bit 8 - Filter FIFO assignment for filter 8"]
    #[inline(always)]
    #[must_use]
    pub fn ffa8(&mut self) -> FFA_W<FFA1Rrs> {
        FFA_W::new(self, 8)
    }
    #[doc = "Bit 9 - Filter FIFO assignment for filter 9"]
    #[inline(always)]
    #[must_use]
    pub fn ffa9(&mut self) -> FFA_W<FFA1Rrs> {
        FFA_W::new(self, 9)
    }
    #[doc = "Bit 10 - Filter FIFO assignment for filter 10"]
    #[inline(always)]
    #[must_use]
    pub fn ffa10(&mut self) -> FFA_W<FFA1Rrs> {
        FFA_W::new(self, 10)
    }
    #[doc = "Bit 11 - Filter FIFO assignment for filter 11"]
    #[inline(always)]
    #[must_use]
    pub fn ffa11(&mut self) -> FFA_W<FFA1Rrs> {
        FFA_W::new(self, 11)
    }
    #[doc = "Bit 12 - Filter FIFO assignment for filter 12"]
    #[inline(always)]
    #[must_use]
    pub fn ffa12(&mut self) -> FFA_W<FFA1Rrs> {
        FFA_W::new(self, 12)
    }
    #[doc = "Bit 13 - Filter FIFO assignment for filter 13"]
    #[inline(always)]
    #[must_use]
    pub fn ffa13(&mut self) -> FFA_W<FFA1Rrs> {
        FFA_W::new(self, 13)
    }
}
#[doc = "CAN_FFA1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ffa1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ffa1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FFA1Rrs;
impl crate::RegisterSpec for FFA1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffa1r::R`](R) reader structure"]
impl crate::Readable for FFA1Rrs {}
#[doc = "`write(|w| ..)` method takes [`ffa1r::W`](W) writer structure"]
impl crate::Writable for FFA1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FFA1R to value 0"]
impl crate::Resettable for FFA1Rrs {
    const RESET_VALUE: u32 = 0;
}
