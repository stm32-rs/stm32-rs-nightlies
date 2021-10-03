#[doc = "Register `SECHDPCR` reader"]
pub struct R(crate::R<SECHDPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECHDPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECHDPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECHDPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECHDPCR` writer"]
pub struct W(crate::W<SECHDPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECHDPCR_SPEC>;
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
impl From<crate::W<SECHDPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECHDPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HDP1_ACCDIS` reader - HDP1_ACCDIS"]
pub struct HDP1_ACCDIS_R(crate::FieldReader<bool, bool>);
impl HDP1_ACCDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDP1_ACCDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HDP1_ACCDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDP1_ACCDIS` writer - HDP1_ACCDIS"]
pub struct HDP1_ACCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HDP1_ACCDIS_W<'a> {
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
#[doc = "Field `HDP2_ACCDIS` reader - HDP2_ACCDIS"]
pub struct HDP2_ACCDIS_R(crate::FieldReader<bool, bool>);
impl HDP2_ACCDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDP2_ACCDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HDP2_ACCDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDP2_ACCDIS` writer - HDP2_ACCDIS"]
pub struct HDP2_ACCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HDP2_ACCDIS_W<'a> {
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
    #[doc = "Bit 0 - HDP1_ACCDIS"]
    #[inline(always)]
    pub fn hdp1_accdis(&self) -> HDP1_ACCDIS_R {
        HDP1_ACCDIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HDP2_ACCDIS"]
    #[inline(always)]
    pub fn hdp2_accdis(&self) -> HDP2_ACCDIS_R {
        HDP2_ACCDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HDP1_ACCDIS"]
    #[inline(always)]
    pub fn hdp1_accdis(&mut self) -> HDP1_ACCDIS_W {
        HDP1_ACCDIS_W { w: self }
    }
    #[doc = "Bit 1 - HDP2_ACCDIS"]
    #[inline(always)]
    pub fn hdp2_accdis(&mut self) -> HDP2_ACCDIS_W {
        HDP2_ACCDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH secure HDP control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sechdpcr](index.html) module"]
pub struct SECHDPCR_SPEC;
impl crate::RegisterSpec for SECHDPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sechdpcr::R](R) reader structure"]
impl crate::Readable for SECHDPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sechdpcr::W](W) writer structure"]
impl crate::Writable for SECHDPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECHDPCR to value 0"]
impl crate::Resettable for SECHDPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
