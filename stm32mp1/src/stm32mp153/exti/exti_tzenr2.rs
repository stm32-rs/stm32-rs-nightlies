#[doc = "Register `EXTI_TZENR2` reader"]
pub type R = crate::R<EXTI_TZENR2rs>;
#[doc = "Register `EXTI_TZENR2` writer"]
pub type W = crate::W<EXTI_TZENR2rs>;
#[doc = "Field `TZEN41` reader - TZEN41"]
pub type TZEN41_R = crate::BitReader;
#[doc = "Field `TZEN41` writer - TZEN41"]
pub type TZEN41_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZEN54` reader - TZEN54"]
pub type TZEN54_R = crate::BitReader;
#[doc = "Field `TZEN54` writer - TZEN54"]
pub type TZEN54_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZEN55` reader - TZEN55"]
pub type TZEN55_R = crate::BitReader;
#[doc = "Field `TZEN55` writer - TZEN55"]
pub type TZEN55_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZEN56` reader - TZEN56"]
pub type TZEN56_R = crate::BitReader;
#[doc = "Field `TZEN56` writer - TZEN56"]
pub type TZEN56_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZEN57` reader - TZEN57"]
pub type TZEN57_R = crate::BitReader;
#[doc = "Field `TZEN57` writer - TZEN57"]
pub type TZEN57_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZEN58` reader - TZEN58"]
pub type TZEN58_R = crate::BitReader;
#[doc = "Field `TZEN58` writer - TZEN58"]
pub type TZEN58_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZEN59` reader - TZEN59"]
pub type TZEN59_R = crate::BitReader;
#[doc = "Field `TZEN59` writer - TZEN59"]
pub type TZEN59_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZEN60` reader - TZEN60"]
pub type TZEN60_R = crate::BitReader;
#[doc = "Field `TZEN60` writer - TZEN60"]
pub type TZEN60_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 9 - TZEN41"]
    #[inline(always)]
    pub fn tzen41(&self) -> TZEN41_R {
        TZEN41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 22 - TZEN54"]
    #[inline(always)]
    pub fn tzen54(&self) -> TZEN54_R {
        TZEN54_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TZEN55"]
    #[inline(always)]
    pub fn tzen55(&self) -> TZEN55_R {
        TZEN55_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TZEN56"]
    #[inline(always)]
    pub fn tzen56(&self) -> TZEN56_R {
        TZEN56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TZEN57"]
    #[inline(always)]
    pub fn tzen57(&self) -> TZEN57_R {
        TZEN57_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TZEN58"]
    #[inline(always)]
    pub fn tzen58(&self) -> TZEN58_R {
        TZEN58_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TZEN59"]
    #[inline(always)]
    pub fn tzen59(&self) -> TZEN59_R {
        TZEN59_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - TZEN60"]
    #[inline(always)]
    pub fn tzen60(&self) -> TZEN60_R {
        TZEN60_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - TZEN41"]
    #[inline(always)]
    #[must_use]
    pub fn tzen41(&mut self) -> TZEN41_W<EXTI_TZENR2rs> {
        TZEN41_W::new(self, 9)
    }
    #[doc = "Bit 22 - TZEN54"]
    #[inline(always)]
    #[must_use]
    pub fn tzen54(&mut self) -> TZEN54_W<EXTI_TZENR2rs> {
        TZEN54_W::new(self, 22)
    }
    #[doc = "Bit 23 - TZEN55"]
    #[inline(always)]
    #[must_use]
    pub fn tzen55(&mut self) -> TZEN55_W<EXTI_TZENR2rs> {
        TZEN55_W::new(self, 23)
    }
    #[doc = "Bit 24 - TZEN56"]
    #[inline(always)]
    #[must_use]
    pub fn tzen56(&mut self) -> TZEN56_W<EXTI_TZENR2rs> {
        TZEN56_W::new(self, 24)
    }
    #[doc = "Bit 25 - TZEN57"]
    #[inline(always)]
    #[must_use]
    pub fn tzen57(&mut self) -> TZEN57_W<EXTI_TZENR2rs> {
        TZEN57_W::new(self, 25)
    }
    #[doc = "Bit 26 - TZEN58"]
    #[inline(always)]
    #[must_use]
    pub fn tzen58(&mut self) -> TZEN58_W<EXTI_TZENR2rs> {
        TZEN58_W::new(self, 26)
    }
    #[doc = "Bit 27 - TZEN59"]
    #[inline(always)]
    #[must_use]
    pub fn tzen59(&mut self) -> TZEN59_W<EXTI_TZENR2rs> {
        TZEN59_W::new(self, 27)
    }
    #[doc = "Bit 28 - TZEN60"]
    #[inline(always)]
    #[must_use]
    pub fn tzen60(&mut self) -> TZEN60_W<EXTI_TZENR2rs> {
        TZEN60_W::new(self, 28)
    }
}
#[doc = "This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_tzenr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_tzenr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTI_TZENR2rs;
impl crate::RegisterSpec for EXTI_TZENR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_tzenr2::R`](R) reader structure"]
impl crate::Readable for EXTI_TZENR2rs {}
#[doc = "`write(|w| ..)` method takes [`exti_tzenr2::W`](W) writer structure"]
impl crate::Writable for EXTI_TZENR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_TZENR2 to value 0"]
impl crate::Resettable for EXTI_TZENR2rs {
    const RESET_VALUE: u32 = 0;
}
