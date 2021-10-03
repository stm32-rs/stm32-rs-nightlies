#[doc = "Register `MACTSEACR` reader"]
pub struct R(crate::R<MACTSEACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACTSEACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACTSEACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACTSEACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACTSEACR` writer"]
pub struct W(crate::W<MACTSEACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACTSEACR_SPEC>;
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
impl From<crate::W<MACTSEACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACTSEACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSTEAC` reader - One-Step Timestamp Egress Asymmetry Correction"]
pub struct OSTEAC_R(crate::FieldReader<u32, u32>);
impl OSTEAC_R {
    pub(crate) fn new(bits: u32) -> Self {
        OSTEAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSTEAC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSTEAC` writer - One-Step Timestamp Egress Asymmetry Correction"]
pub struct OSTEAC_W<'a> {
    w: &'a mut W,
}
impl<'a> OSTEAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - One-Step Timestamp Egress Asymmetry Correction"]
    #[inline(always)]
    pub fn osteac(&self) -> OSTEAC_R {
        OSTEAC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - One-Step Timestamp Egress Asymmetry Correction"]
    #[inline(always)]
    pub fn osteac(&mut self) -> OSTEAC_W {
        OSTEAC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timestamp Egress asymmetric correction register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mactseacr](index.html) module"]
pub struct MACTSEACR_SPEC;
impl crate::RegisterSpec for MACTSEACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mactseacr::R](R) reader structure"]
impl crate::Readable for MACTSEACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mactseacr::W](W) writer structure"]
impl crate::Writable for MACTSEACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACTSEACR to value 0"]
impl crate::Resettable for MACTSEACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
