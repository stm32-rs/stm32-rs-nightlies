#[doc = "Register `CNSLCKR` reader"]
pub struct R(crate::R<CNSLCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNSLCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNSLCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNSLCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNSLCKR` writer"]
pub struct W(crate::W<CNSLCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNSLCKR_SPEC>;
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
impl From<crate::W<CNSLCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNSLCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKNSVTOR` reader - VTOR_NS register lock"]
pub struct LOCKNSVTOR_R(crate::FieldReader<bool, bool>);
impl LOCKNSVTOR_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCKNSVTOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCKNSVTOR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKNSVTOR` writer - VTOR_NS register lock"]
pub struct LOCKNSVTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKNSVTOR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `LOCKNSMPU` reader - Non-secure MPU registers lock"]
pub struct LOCKNSMPU_R(crate::FieldReader<bool, bool>);
impl LOCKNSMPU_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCKNSMPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCKNSMPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKNSMPU` writer - Non-secure MPU registers lock"]
pub struct LOCKNSMPU_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKNSMPU_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - VTOR_NS register lock"]
    #[inline(always)]
    pub fn locknsvtor(&self) -> LOCKNSVTOR_R {
        LOCKNSVTOR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Non-secure MPU registers lock"]
    #[inline(always)]
    pub fn locknsmpu(&self) -> LOCKNSMPU_R {
        LOCKNSMPU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VTOR_NS register lock"]
    #[inline(always)]
    pub fn locknsvtor(&mut self) -> LOCKNSVTOR_W {
        LOCKNSVTOR_W { w: self }
    }
    #[doc = "Bit 1 - Non-secure MPU registers lock"]
    #[inline(always)]
    pub fn locknsmpu(&mut self) -> LOCKNSMPU_W {
        LOCKNSMPU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG CPU non-secure lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnslckr](index.html) module"]
pub struct CNSLCKR_SPEC;
impl crate::RegisterSpec for CNSLCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnslckr::R](R) reader structure"]
impl crate::Readable for CNSLCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnslckr::W](W) writer structure"]
impl crate::Writable for CNSLCKR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNSLCKR to value 0"]
impl crate::Resettable for CNSLCKR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
