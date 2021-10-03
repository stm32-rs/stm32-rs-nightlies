#[doc = "Register `CSLOCKR` reader"]
pub struct R(crate::R<CSLOCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSLOCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSLOCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSLOCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSLOCKR` writer"]
pub struct W(crate::W<CSLOCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSLOCKR_SPEC>;
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
impl From<crate::W<CSLOCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSLOCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKSVTAIRCR` reader - LOCKSVTAIRCR"]
pub struct LOCKSVTAIRCR_R(crate::FieldReader<bool, bool>);
impl LOCKSVTAIRCR_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCKSVTAIRCR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCKSVTAIRCR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKSVTAIRCR` writer - LOCKSVTAIRCR"]
pub struct LOCKSVTAIRCR_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKSVTAIRCR_W<'a> {
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
#[doc = "Field `LOCKSMPU` reader - LOCKSMPU"]
pub struct LOCKSMPU_R(crate::FieldReader<bool, bool>);
impl LOCKSMPU_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCKSMPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCKSMPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKSMPU` writer - LOCKSMPU"]
pub struct LOCKSMPU_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKSMPU_W<'a> {
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
#[doc = "Field `LOCKSAU` reader - LOCKSAU"]
pub struct LOCKSAU_R(crate::FieldReader<bool, bool>);
impl LOCKSAU_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCKSAU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCKSAU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKSAU` writer - LOCKSAU"]
pub struct LOCKSAU_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKSAU_W<'a> {
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
impl R {
    #[doc = "Bit 0 - LOCKSVTAIRCR"]
    #[inline(always)]
    pub fn locksvtaircr(&self) -> LOCKSVTAIRCR_R {
        LOCKSVTAIRCR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LOCKSMPU"]
    #[inline(always)]
    pub fn locksmpu(&self) -> LOCKSMPU_R {
        LOCKSMPU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LOCKSAU"]
    #[inline(always)]
    pub fn locksau(&self) -> LOCKSAU_R {
        LOCKSAU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LOCKSVTAIRCR"]
    #[inline(always)]
    pub fn locksvtaircr(&mut self) -> LOCKSVTAIRCR_W {
        LOCKSVTAIRCR_W { w: self }
    }
    #[doc = "Bit 1 - LOCKSMPU"]
    #[inline(always)]
    pub fn locksmpu(&mut self) -> LOCKSMPU_W {
        LOCKSMPU_W { w: self }
    }
    #[doc = "Bit 2 - LOCKSAU"]
    #[inline(always)]
    pub fn locksau(&mut self) -> LOCKSAU_W {
        LOCKSAU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG CPU secure lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cslockr](index.html) module"]
pub struct CSLOCKR_SPEC;
impl crate::RegisterSpec for CSLOCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cslockr::R](R) reader structure"]
impl crate::Readable for CSLOCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cslockr::W](W) writer structure"]
impl crate::Writable for CSLOCKR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSLOCKR to value 0"]
impl crate::Resettable for CSLOCKR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
