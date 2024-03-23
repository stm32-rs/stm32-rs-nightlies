#[doc = "Register `SECCFGR` reader"]
pub type R = crate::R<SECCFGRrs>;
#[doc = "Register `SECCFGR` writer"]
pub type W = crate::W<SECCFGRrs>;
#[doc = "Field `SEC0` reader - secure state of channel x (x = 7 to 0)"]
pub type SEC0_R = crate::BitReader;
#[doc = "Field `SEC0` writer - secure state of channel x (x = 7 to 0)"]
pub type SEC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC1` reader - secure state of channel x (x = 7 to 0)"]
pub type SEC1_R = crate::BitReader;
#[doc = "Field `SEC1` writer - secure state of channel x (x = 7 to 0)"]
pub type SEC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC2` reader - secure state of channel x (x = 7 to 0)"]
pub type SEC2_R = crate::BitReader;
#[doc = "Field `SEC2` writer - secure state of channel x (x = 7 to 0)"]
pub type SEC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC3` reader - secure state of channel x (x = 7 to 0)"]
pub type SEC3_R = crate::BitReader;
#[doc = "Field `SEC3` writer - secure state of channel x (x = 7 to 0)"]
pub type SEC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC4` reader - secure state of channel x (x = 7 to 0)"]
pub type SEC4_R = crate::BitReader;
#[doc = "Field `SEC4` writer - secure state of channel x (x = 7 to 0)"]
pub type SEC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC5` reader - secure state of channel x (x = 7 to 0)"]
pub type SEC5_R = crate::BitReader;
#[doc = "Field `SEC5` writer - secure state of channel x (x = 7 to 0)"]
pub type SEC5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC6` reader - secure state of channel x (x = 7 to 0)"]
pub type SEC6_R = crate::BitReader;
#[doc = "Field `SEC6` writer - secure state of channel x (x = 7 to 0)"]
pub type SEC6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC7` reader - secure state of channel x (x = 7 to 0)"]
pub type SEC7_R = crate::BitReader;
#[doc = "Field `SEC7` writer - secure state of channel x (x = 7 to 0)"]
pub type SEC7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn sec0(&self) -> SEC0_R {
        SEC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn sec1(&self) -> SEC1_R {
        SEC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn sec2(&self) -> SEC2_R {
        SEC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn sec3(&self) -> SEC3_R {
        SEC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn sec4(&self) -> SEC4_R {
        SEC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn sec5(&self) -> SEC5_R {
        SEC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn sec6(&self) -> SEC6_R {
        SEC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    pub fn sec7(&self) -> SEC7_R {
        SEC7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    #[must_use]
    pub fn sec0(&mut self) -> SEC0_W<SECCFGRrs> {
        SEC0_W::new(self, 0)
    }
    #[doc = "Bit 1 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    #[must_use]
    pub fn sec1(&mut self) -> SEC1_W<SECCFGRrs> {
        SEC1_W::new(self, 1)
    }
    #[doc = "Bit 2 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    #[must_use]
    pub fn sec2(&mut self) -> SEC2_W<SECCFGRrs> {
        SEC2_W::new(self, 2)
    }
    #[doc = "Bit 3 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    #[must_use]
    pub fn sec3(&mut self) -> SEC3_W<SECCFGRrs> {
        SEC3_W::new(self, 3)
    }
    #[doc = "Bit 4 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    #[must_use]
    pub fn sec4(&mut self) -> SEC4_W<SECCFGRrs> {
        SEC4_W::new(self, 4)
    }
    #[doc = "Bit 5 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    #[must_use]
    pub fn sec5(&mut self) -> SEC5_W<SECCFGRrs> {
        SEC5_W::new(self, 5)
    }
    #[doc = "Bit 6 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    #[must_use]
    pub fn sec6(&mut self) -> SEC6_W<SECCFGRrs> {
        SEC6_W::new(self, 6)
    }
    #[doc = "Bit 7 - secure state of channel x (x = 7 to 0)"]
    #[inline(always)]
    #[must_use]
    pub fn sec7(&mut self) -> SEC7_W<SECCFGRrs> {
        SEC7_W::new(self, 7)
    }
}
#[doc = "GPDMA secure configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECCFGRrs;
impl crate::RegisterSpec for SECCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seccfgr::R`](R) reader structure"]
impl crate::Readable for SECCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`seccfgr::W`](W) writer structure"]
impl crate::Writable for SECCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECCFGR to value 0"]
impl crate::Resettable for SECCFGRrs {
    const RESET_VALUE: u32 = 0;
}
