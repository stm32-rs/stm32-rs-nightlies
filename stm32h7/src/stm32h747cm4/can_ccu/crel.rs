#[doc = "Register `CREL` reader"]
pub type R = crate::R<CRELrs>;
#[doc = "Register `CREL` writer"]
pub type W = crate::W<CRELrs>;
#[doc = "Field `DAY` reader - Time Stamp Day"]
pub type DAY_R = crate::FieldReader;
#[doc = "Field `DAY` writer - Time Stamp Day"]
pub type DAY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MON` reader - Time Stamp Month"]
pub type MON_R = crate::FieldReader;
#[doc = "Field `MON` writer - Time Stamp Month"]
pub type MON_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `YEAR` reader - Time Stamp Year"]
pub type YEAR_R = crate::FieldReader;
#[doc = "Field `YEAR` writer - Time Stamp Year"]
pub type YEAR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SUBSTEP` reader - Sub-step of Core Release"]
pub type SUBSTEP_R = crate::FieldReader;
#[doc = "Field `SUBSTEP` writer - Sub-step of Core Release"]
pub type SUBSTEP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `STEP` reader - Step of Core Release"]
pub type STEP_R = crate::FieldReader;
#[doc = "Field `STEP` writer - Step of Core Release"]
pub type STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REL` reader - Core Release"]
pub type REL_R = crate::FieldReader;
#[doc = "Field `REL` writer - Core Release"]
pub type REL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - Time Stamp Day"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Time Stamp Month"]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Time Stamp Year"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Sub-step of Core Release"]
    #[inline(always)]
    pub fn substep(&self) -> SUBSTEP_R {
        SUBSTEP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Step of Core Release"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Core Release"]
    #[inline(always)]
    pub fn rel(&self) -> REL_R {
        REL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Time Stamp Day"]
    #[inline(always)]
    #[must_use]
    pub fn day(&mut self) -> DAY_W<CRELrs> {
        DAY_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Time Stamp Month"]
    #[inline(always)]
    #[must_use]
    pub fn mon(&mut self) -> MON_W<CRELrs> {
        MON_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Time Stamp Year"]
    #[inline(always)]
    #[must_use]
    pub fn year(&mut self) -> YEAR_W<CRELrs> {
        YEAR_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Sub-step of Core Release"]
    #[inline(always)]
    #[must_use]
    pub fn substep(&mut self) -> SUBSTEP_W<CRELrs> {
        SUBSTEP_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Step of Core Release"]
    #[inline(always)]
    #[must_use]
    pub fn step(&mut self) -> STEP_W<CRELrs> {
        STEP_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Core Release"]
    #[inline(always)]
    #[must_use]
    pub fn rel(&mut self) -> REL_W<CRELrs> {
        REL_W::new(self, 28)
    }
}
#[doc = "Clock Calibration Unit Core Release Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRELrs;
impl crate::RegisterSpec for CRELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crel::R`](R) reader structure"]
impl crate::Readable for CRELrs {}
#[doc = "`write(|w| ..)` method takes [`crel::W`](W) writer structure"]
impl crate::Writable for CRELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CREL to value 0"]
impl crate::Resettable for CRELrs {
    const RESET_VALUE: u32 = 0;
}
