#[doc = "Register `OPAMP2_OTR` reader"]
pub struct R(crate::R<OPAMP2_OTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAMP2_OTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAMP2_OTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAMP2_OTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPAMP2_OTR` writer"]
pub struct W(crate::W<OPAMP2_OTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAMP2_OTR_SPEC>;
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
impl From<crate::W<OPAMP2_OTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAMP2_OTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIMOFFSETN` reader - Trim for NMOS differential pairs"]
pub struct TRIMOFFSETN_R(crate::FieldReader<u8, u8>);
impl TRIMOFFSETN_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIMOFFSETN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIMOFFSETN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIMOFFSETN` writer - Trim for NMOS differential pairs"]
pub struct TRIMOFFSETN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMOFFSETN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `TRIMOFFSETP` reader - Trim for PMOS differential pairs"]
pub struct TRIMOFFSETP_R(crate::FieldReader<u8, u8>);
impl TRIMOFFSETP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIMOFFSETP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIMOFFSETP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIMOFFSETP` writer - Trim for PMOS differential pairs"]
pub struct TRIMOFFSETP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMOFFSETP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Trim for NMOS differential pairs"]
    #[inline(always)]
    pub fn trimoffsetn(&self) -> TRIMOFFSETN_R {
        TRIMOFFSETN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Trim for PMOS differential pairs"]
    #[inline(always)]
    pub fn trimoffsetp(&self) -> TRIMOFFSETP_R {
        TRIMOFFSETP_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trim for NMOS differential pairs"]
    #[inline(always)]
    pub fn trimoffsetn(&mut self) -> TRIMOFFSETN_W {
        TRIMOFFSETN_W { w: self }
    }
    #[doc = "Bits 8:12 - Trim for PMOS differential pairs"]
    #[inline(always)]
    pub fn trimoffsetp(&mut self) -> TRIMOFFSETP_W {
        TRIMOFFSETP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPAMP2 offset trimming register in normal mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opamp2_otr](index.html) module"]
pub struct OPAMP2_OTR_SPEC;
impl crate::RegisterSpec for OPAMP2_OTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opamp2_otr::R](R) reader structure"]
impl crate::Readable for OPAMP2_OTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opamp2_otr::W](W) writer structure"]
impl crate::Writable for OPAMP2_OTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPAMP2_OTR to value 0"]
impl crate::Resettable for OPAMP2_OTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
