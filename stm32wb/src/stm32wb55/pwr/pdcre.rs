#[doc = "Register `PDCRE` reader"]
pub type R = crate::R<PDCRErs>;
#[doc = "Register `PDCRE` writer"]
pub type W = crate::W<PDCRErs>;
#[doc = "Field `PD0` reader - Port E pull-down bit y (y=0..15)"]
pub type PD0_R = crate::BitReader;
#[doc = "Field `PD0` writer - Port E pull-down bit y (y=0..15)"]
pub type PD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD1` reader - Port E pull-down bit y (y=0..15)"]
pub type PD1_R = crate::BitReader;
#[doc = "Field `PD1` writer - Port E pull-down bit y (y=0..15)"]
pub type PD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD2` reader - Port E pull-down bit y (y=0..15)"]
pub type PD2_R = crate::BitReader;
#[doc = "Field `PD2` writer - Port E pull-down bit y (y=0..15)"]
pub type PD2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD3` reader - Port E pull-down bit y (y=0..15)"]
pub type PD3_R = crate::BitReader;
#[doc = "Field `PD3` writer - Port E pull-down bit y (y=0..15)"]
pub type PD3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD4` reader - Port E pull-down bit y (y=0..15)"]
pub type PD4_R = crate::BitReader;
#[doc = "Field `PD4` writer - Port E pull-down bit y (y=0..15)"]
pub type PD4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port E pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port E pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port E pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port E pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port E pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port E pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> PD0_W<PDCRErs> {
        PD0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port E pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> PD1_W<PDCRErs> {
        PD1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port E pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> PD2_W<PDCRErs> {
        PD2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port E pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> PD3_W<PDCRErs> {
        PD3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port E pull-down bit y (y=0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pd4(&mut self) -> PD4_W<PDCRErs> {
        PD4_W::new(self, 4)
    }
}
#[doc = "Power Port E pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdcre::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdcre::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDCRErs;
impl crate::RegisterSpec for PDCRErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdcre::R`](R) reader structure"]
impl crate::Readable for PDCRErs {}
#[doc = "`write(|w| ..)` method takes [`pdcre::W`](W) writer structure"]
impl crate::Writable for PDCRErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDCRE to value 0"]
impl crate::Resettable for PDCRErs {
    const RESET_VALUE: u32 = 0;
}
