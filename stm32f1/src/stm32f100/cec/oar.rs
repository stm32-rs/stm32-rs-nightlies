#[doc = "Register `OAR` reader"]
pub struct R(crate::R<OAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OAR` writer"]
pub struct W(crate::W<OAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OAR_SPEC>;
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
impl From<crate::W<OAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OA` reader - Own address"]
pub struct OA_R(crate::FieldReader<u8, u8>);
impl OA_R {
    pub(crate) fn new(bits: u8) -> Self {
        OA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA` writer - Own address"]
pub struct OA_W<'a> {
    w: &'a mut W,
}
impl<'a> OA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Own address"]
    #[inline(always)]
    pub fn oa(&self) -> OA_R {
        OA_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Own address"]
    #[inline(always)]
    pub fn oa(&mut self) -> OA_W {
        OA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CEC own address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oar](index.html) module"]
pub struct OAR_SPEC;
impl crate::RegisterSpec for OAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oar::R](R) reader structure"]
impl crate::Readable for OAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oar::W](W) writer structure"]
impl crate::Writable for OAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OAR to value 0"]
impl crate::Resettable for OAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
