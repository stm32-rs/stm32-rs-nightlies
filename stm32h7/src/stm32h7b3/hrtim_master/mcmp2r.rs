#[doc = "Register `MCMP2R` reader"]
pub struct R(crate::R<MCMP2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCMP2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCMP2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCMP2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCMP2R` writer"]
pub struct W(crate::W<MCMP2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCMP2R_SPEC>;
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
impl From<crate::W<MCMP2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCMP2R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCMP2` reader - Master Timer Compare 2 value"]
pub struct MCMP2_R(crate::FieldReader<u16, u16>);
impl MCMP2_R {
    pub(crate) fn new(bits: u16) -> Self {
        MCMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCMP2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCMP2` writer - Master Timer Compare 2 value"]
pub struct MCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Master Timer Compare 2 value"]
    #[inline(always)]
    pub fn mcmp2(&self) -> MCMP2_R {
        MCMP2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Master Timer Compare 2 value"]
    #[inline(always)]
    pub fn mcmp2(&mut self) -> MCMP2_W {
        MCMP2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Timer Compare 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcmp2r](index.html) module"]
pub struct MCMP2R_SPEC;
impl crate::RegisterSpec for MCMP2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcmp2r::R](R) reader structure"]
impl crate::Readable for MCMP2R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcmp2r::W](W) writer structure"]
impl crate::Writable for MCMP2R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCMP2R to value 0"]
impl crate::Resettable for MCMP2R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
