#[doc = "Register `FS1R` reader"]
pub type R = crate::R<FS1Rrs>;
#[doc = "Register `FS1R` writer"]
pub type W = crate::W<FS1Rrs>;
#[doc = "Field `FSC(0-13)` reader - Filter scale configuration"]
pub type FSC_R = crate::BitReader;
#[doc = "Field `FSC(0-13)` writer - Filter scale configuration"]
pub type FSC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Filter scale configuration"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `FSC0` field"]
    #[inline(always)]
    pub fn fsc(&self, n: u8) -> FSC_R {
        #[allow(clippy::no_effect)]
        [(); 14][n as usize];
        FSC_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Filter scale configuration"]
    #[inline(always)]
    pub fn fsc_iter(&self) -> impl Iterator<Item = FSC_R> + '_ {
        (0..14).map(move |n| FSC_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc0(&self) -> FSC_R {
        FSC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc1(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc2(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc3(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc4(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc5(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc6(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc7(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc8(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc9(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc10(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc11(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc12(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc13(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Filter scale configuration"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `FSC0` field"]
    #[inline(always)]
    #[must_use]
    pub fn fsc(&mut self, n: u8) -> FSC_W<FS1Rrs> {
        #[allow(clippy::no_effect)]
        [(); 14][n as usize];
        FSC_W::new(self, n)
    }
    #[doc = "Bit 0 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc0(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc1(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc2(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc3(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 3)
    }
    #[doc = "Bit 4 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc4(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc5(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 5)
    }
    #[doc = "Bit 6 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc6(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 6)
    }
    #[doc = "Bit 7 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc7(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 7)
    }
    #[doc = "Bit 8 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc8(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 8)
    }
    #[doc = "Bit 9 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc9(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 9)
    }
    #[doc = "Bit 10 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc10(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 10)
    }
    #[doc = "Bit 11 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc11(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 11)
    }
    #[doc = "Bit 12 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc12(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 12)
    }
    #[doc = "Bit 13 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc13(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 13)
    }
}
#[doc = "CAN_FS1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS1Rrs;
impl crate::RegisterSpec for FS1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs1r::R`](R) reader structure"]
impl crate::Readable for FS1Rrs {}
#[doc = "`write(|w| ..)` method takes [`fs1r::W`](W) writer structure"]
impl crate::Writable for FS1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FS1R to value 0"]
impl crate::Resettable for FS1Rrs {
    const RESET_VALUE: u32 = 0;
}
