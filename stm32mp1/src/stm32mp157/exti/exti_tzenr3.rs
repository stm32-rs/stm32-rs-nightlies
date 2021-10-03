#[doc = "Register `EXTI_TZENR3` reader"]
pub struct R(crate::R<EXTI_TZENR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_TZENR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_TZENR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_TZENR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_TZENR3` writer"]
pub struct W(crate::W<EXTI_TZENR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_TZENR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EXTI_TZENR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_TZENR3_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_tzenr3](index.html) module"]
pub struct EXTI_TZENR3_SPEC;
impl crate::RegisterSpec for EXTI_TZENR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_tzenr3::R](R) reader structure"]
impl crate::Readable for EXTI_TZENR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_tzenr3::W](W) writer structure"]
impl crate::Writable for EXTI_TZENR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTI_TZENR3 to value 0"]
impl crate::Resettable for EXTI_TZENR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
