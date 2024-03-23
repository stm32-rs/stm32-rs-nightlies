#[doc = "Register `PUCRH` reader"]
pub type R = crate::R<PUCRHrs>;
#[doc = "Register `PUCRH` writer"]
pub type W = crate::W<PUCRHrs>;
#[doc = "Field `PU0` reader - Port H pull-up bit y (y=0..1)"]
pub type PU0_R = crate::BitReader;
#[doc = "Field `PU0` writer - Port H pull-up bit y (y=0..1)"]
pub type PU0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU1` reader - Port H pull-up bit y (y=0..1)"]
pub type PU1_R = crate::BitReader;
#[doc = "Field `PU1` writer - Port H pull-up bit y (y=0..1)"]
pub type PU1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port H pull-up bit y (y=0..1)"]
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port H pull-up bit y (y=0..1)"]
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port H pull-up bit y (y=0..1)"]
    #[inline(always)]
    #[must_use]
    pub fn pu0(&mut self) -> PU0_W<PUCRHrs> {
        PU0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port H pull-up bit y (y=0..1)"]
    #[inline(always)]
    #[must_use]
    pub fn pu1(&mut self) -> PU1_W<PUCRHrs> {
        PU1_W::new(self, 1)
    }
}
#[doc = "Power Port H pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pucrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pucrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUCRHrs;
impl crate::RegisterSpec for PUCRHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pucrh::R`](R) reader structure"]
impl crate::Readable for PUCRHrs {}
#[doc = "`write(|w| ..)` method takes [`pucrh::W`](W) writer structure"]
impl crate::Writable for PUCRHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUCRH to value 0"]
impl crate::Resettable for PUCRHrs {
    const RESET_VALUE: u32 = 0;
}
