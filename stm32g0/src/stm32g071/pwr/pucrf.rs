#[doc = "Register `PUCRF` reader"]
pub type R = crate::R<PUCRFrs>;
#[doc = "Register `PUCRF` writer"]
pub type W = crate::W<PUCRFrs>;
#[doc = "Field `PU0` reader - Port F pull-up bit y (y=0..15)"]
pub type PU0_R = crate::BitReader;
#[doc = "Field `PU0` writer - Port F pull-up bit y (y=0..15)"]
pub type PU0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU1` reader - Port F pull-up bit y (y=0..15)"]
pub type PU1_R = crate::BitReader;
#[doc = "Field `PU1` writer - Port F pull-up bit y (y=0..15)"]
pub type PU1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU2` reader - Port F pull-up bit y (y=0..15)"]
pub type PU2_R = crate::BitReader;
#[doc = "Field `PU2` writer - Port F pull-up bit y (y=0..15)"]
pub type PU2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port F pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port F pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port F pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port F pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu0(&mut self) -> PU0_W<PUCRFrs> {
        PU0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port F pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu1(&mut self) -> PU1_W<PUCRFrs> {
        PU1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port F pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu2(&mut self) -> PU2_W<PUCRFrs> {
        PU2_W::new(self, 2)
    }
}
#[doc = "Power Port F pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pucrf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pucrf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUCRFrs;
impl crate::RegisterSpec for PUCRFrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pucrf::R`](R) reader structure"]
impl crate::Readable for PUCRFrs {}
#[doc = "`write(|w| ..)` method takes [`pucrf::W`](W) writer structure"]
impl crate::Writable for PUCRFrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUCRF to value 0"]
impl crate::Resettable for PUCRFrs {
    const RESET_VALUE: u32 = 0;
}
