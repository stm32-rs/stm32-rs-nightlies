#[doc = "Register `PUCRE` reader"]
pub type R = crate::R<PUCRErs>;
#[doc = "Register `PUCRE` writer"]
pub type W = crate::W<PUCRErs>;
#[doc = "Field `PU0` reader - Port E pull-up bit y (y=0..15)"]
pub type PU0_R = crate::BitReader;
#[doc = "Field `PU0` writer - Port E pull-up bit y (y=0..15)"]
pub type PU0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU1` reader - Port E pull-up bit y (y=0..15)"]
pub type PU1_R = crate::BitReader;
#[doc = "Field `PU1` writer - Port E pull-up bit y (y=0..15)"]
pub type PU1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU2` reader - Port E pull-up bit y (y=0..15)"]
pub type PU2_R = crate::BitReader;
#[doc = "Field `PU2` writer - Port E pull-up bit y (y=0..15)"]
pub type PU2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU3` reader - Port E pull-up bit y (y=0..15)"]
pub type PU3_R = crate::BitReader;
#[doc = "Field `PU3` writer - Port E pull-up bit y (y=0..15)"]
pub type PU3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU4` reader - Port E pull-up bit y (y=0..15)"]
pub type PU4_R = crate::BitReader;
#[doc = "Field `PU4` writer - Port E pull-up bit y (y=0..15)"]
pub type PU4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu4(&self) -> PU4_R {
        PU4_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu0(&mut self) -> PU0_W<PUCRErs> {
        PU0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu1(&mut self) -> PU1_W<PUCRErs> {
        PU1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu2(&mut self) -> PU2_W<PUCRErs> {
        PU2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu3(&mut self) -> PU3_W<PUCRErs> {
        PU3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pu4(&mut self) -> PU4_W<PUCRErs> {
        PU4_W::new(self, 4)
    }
}
#[doc = "Power Port E pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pucre::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pucre::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUCRErs;
impl crate::RegisterSpec for PUCRErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pucre::R`](R) reader structure"]
impl crate::Readable for PUCRErs {}
#[doc = "`write(|w| ..)` method takes [`pucre::W`](W) writer structure"]
impl crate::Writable for PUCRErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUCRE to value 0"]
impl crate::Resettable for PUCRErs {
    const RESET_VALUE: u32 = 0;
}
