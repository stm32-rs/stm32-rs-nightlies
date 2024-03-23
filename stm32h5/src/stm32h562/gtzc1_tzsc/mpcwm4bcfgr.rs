#[doc = "Register `MPCWM4BCFGR` reader"]
pub type R = crate::R<MPCWM4BCFGRrs>;
#[doc = "Register `MPCWM4BCFGR` writer"]
pub type W = crate::W<MPCWM4BCFGRrs>;
#[doc = "Field `SREN` reader - Sub-region B enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value)."]
pub type SREN_R = crate::BitReader;
#[doc = "Field `SREN` writer - Sub-region B enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value)."]
pub type SREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRLOCK` reader - Sub-region B lock This bit, once set, can be cleared only by a system reset."]
pub type SRLOCK_R = crate::BitReader;
#[doc = "Field `SRLOCK` writer - Sub-region B lock This bit, once set, can be cleared only by a system reset."]
pub type SRLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC` reader - Secure sub-region B of base region x This bit is taken into account only if SREN is set."]
pub type SEC_R = crate::BitReader;
#[doc = "Field `SEC` writer - Secure sub-region B of base region x This bit is taken into account only if SREN is set."]
pub type SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV` reader - Privileged sub-region B of base region x This bit is taken into account only if SREN is set."]
pub type PRIV_R = crate::BitReader;
#[doc = "Field `PRIV` writer - Privileged sub-region B of base region x This bit is taken into account only if SREN is set."]
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sub-region B enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value)."]
    #[inline(always)]
    pub fn sren(&self) -> SREN_R {
        SREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sub-region B lock This bit, once set, can be cleared only by a system reset."]
    #[inline(always)]
    pub fn srlock(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Secure sub-region B of base region x This bit is taken into account only if SREN is set."]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Privileged sub-region B of base region x This bit is taken into account only if SREN is set."]
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sub-region B enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value)."]
    #[inline(always)]
    #[must_use]
    pub fn sren(&mut self) -> SREN_W<MPCWM4BCFGRrs> {
        SREN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Sub-region B lock This bit, once set, can be cleared only by a system reset."]
    #[inline(always)]
    #[must_use]
    pub fn srlock(&mut self) -> SRLOCK_W<MPCWM4BCFGRrs> {
        SRLOCK_W::new(self, 1)
    }
    #[doc = "Bit 8 - Secure sub-region B of base region x This bit is taken into account only if SREN is set."]
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SEC_W<MPCWM4BCFGRrs> {
        SEC_W::new(self, 8)
    }
    #[doc = "Bit 9 - Privileged sub-region B of base region x This bit is taken into account only if SREN is set."]
    #[inline(always)]
    #[must_use]
    pub fn priv_(&mut self) -> PRIV_W<MPCWM4BCFGRrs> {
        PRIV_W::new(self, 9)
    }
}
#[doc = "GTZC1 TZSC memory 4 sub-region B watermark configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm4bcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm4bcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPCWM4BCFGRrs;
impl crate::RegisterSpec for MPCWM4BCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpcwm4bcfgr::R`](R) reader structure"]
impl crate::Readable for MPCWM4BCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`mpcwm4bcfgr::W`](W) writer structure"]
impl crate::Writable for MPCWM4BCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPCWM4BCFGR to value 0"]
impl crate::Resettable for MPCWM4BCFGRrs {
    const RESET_VALUE: u32 = 0;
}
