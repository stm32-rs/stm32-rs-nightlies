#[doc = "Register `TZSC_SECCFGR1` reader"]
pub struct R(crate::R<TZSC_SECCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZSC_SECCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZSC_SECCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZSC_SECCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TZSC_SECCFGR1` writer"]
pub struct W(crate::W<TZSC_SECCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZSC_SECCFGR1_SPEC>;
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
impl From<crate::W<TZSC_SECCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZSC_SECCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AESSEC` reader - AESSEC"]
pub struct AESSEC_R(crate::FieldReader<bool, bool>);
impl AESSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        AESSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESSEC` writer - AESSEC"]
pub struct AESSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> AESSEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `RNGSEC` reader - RNGSEC"]
pub struct RNGSEC_R(crate::FieldReader<bool, bool>);
impl RNGSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNGSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNGSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNGSEC` writer - RNGSEC"]
pub struct RNGSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGSEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `PKASEC` reader - PKASEC"]
pub struct PKASEC_R(crate::FieldReader<bool, bool>);
impl PKASEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKASEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKASEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKASEC` writer - PKASEC"]
pub struct PKASEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PKASEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - AESSEC"]
    #[inline(always)]
    pub fn aessec(&self) -> AESSEC_R {
        AESSEC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RNGSEC"]
    #[inline(always)]
    pub fn rngsec(&self) -> RNGSEC_R {
        RNGSEC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PKASEC"]
    #[inline(always)]
    pub fn pkasec(&self) -> PKASEC_R {
        PKASEC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - AESSEC"]
    #[inline(always)]
    pub fn aessec(&mut self) -> AESSEC_W {
        AESSEC_W { w: self }
    }
    #[doc = "Bit 3 - RNGSEC"]
    #[inline(always)]
    pub fn rngsec(&mut self) -> RNGSEC_W {
        RNGSEC_W { w: self }
    }
    #[doc = "Bit 13 - PKASEC"]
    #[inline(always)]
    pub fn pkasec(&mut self) -> PKASEC_W {
        PKASEC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZSC security configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzsc_seccfgr1](index.html) module"]
pub struct TZSC_SECCFGR1_SPEC;
impl crate::RegisterSpec for TZSC_SECCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzsc_seccfgr1::R](R) reader structure"]
impl crate::Readable for TZSC_SECCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzsc_seccfgr1::W](W) writer structure"]
impl crate::Writable for TZSC_SECCFGR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TZSC_SECCFGR1 to value 0"]
impl crate::Resettable for TZSC_SECCFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
