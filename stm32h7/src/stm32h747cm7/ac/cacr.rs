#[doc = "Register `CACR` reader"]
pub struct R(crate::R<CACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACR` writer"]
pub struct W(crate::W<CACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACR_SPEC>;
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
impl From<crate::W<CACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIWT` reader - SIWT"]
pub struct SIWT_R(crate::FieldReader<bool, bool>);
impl SIWT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIWT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIWT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIWT` writer - SIWT"]
pub struct SIWT_W<'a> {
    w: &'a mut W,
}
impl<'a> SIWT_W<'a> {
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
#[doc = "Field `ECCEN` reader - ECCEN"]
pub struct ECCEN_R(crate::FieldReader<bool, bool>);
impl ECCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCEN` writer - ECCEN"]
pub struct ECCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCEN_W<'a> {
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
#[doc = "Field `FORCEWT` reader - FORCEWT"]
pub struct FORCEWT_R(crate::FieldReader<bool, bool>);
impl FORCEWT_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCEWT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCEWT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCEWT` writer - FORCEWT"]
pub struct FORCEWT_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEWT_W<'a> {
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
    #[doc = "Bit 0 - SIWT"]
    #[inline(always)]
    pub fn siwt(&self) -> SIWT_R {
        SIWT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ECCEN"]
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FORCEWT"]
    #[inline(always)]
    pub fn forcewt(&self) -> FORCEWT_R {
        FORCEWT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SIWT"]
    #[inline(always)]
    pub fn siwt(&mut self) -> SIWT_W {
        SIWT_W { w: self }
    }
    #[doc = "Bit 1 - ECCEN"]
    #[inline(always)]
    pub fn eccen(&mut self) -> ECCEN_W {
        ECCEN_W { w: self }
    }
    #[doc = "Bit 2 - FORCEWT"]
    #[inline(always)]
    pub fn forcewt(&mut self) -> FORCEWT_W {
        FORCEWT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auxiliary Cache Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cacr](index.html) module"]
pub struct CACR_SPEC;
impl crate::RegisterSpec for CACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cacr::R](R) reader structure"]
impl crate::Readable for CACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cacr::W](W) writer structure"]
impl crate::Writable for CACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACR to value 0"]
impl crate::Resettable for CACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
