#[doc = "Register `LTDC_TWCR` reader"]
pub struct R(crate::R<LTDC_TWCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_TWCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_TWCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_TWCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTDC_TWCR` writer"]
pub struct W(crate::W<LTDC_TWCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_TWCR_SPEC>;
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
impl From<crate::W<LTDC_TWCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_TWCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOTALH` reader - TOTALH"]
pub struct TOTALH_R(crate::FieldReader<u16, u16>);
impl TOTALH_R {
    pub(crate) fn new(bits: u16) -> Self {
        TOTALH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOTALH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOTALH` writer - TOTALH"]
pub struct TOTALH_W<'a> {
    w: &'a mut W,
}
impl<'a> TOTALH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `TOTALW` reader - TOTALW"]
pub struct TOTALW_R(crate::FieldReader<u16, u16>);
impl TOTALW_R {
    pub(crate) fn new(bits: u16) -> Self {
        TOTALW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOTALW_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOTALW` writer - TOTALW"]
pub struct TOTALW_W<'a> {
    w: &'a mut W,
}
impl<'a> TOTALW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - TOTALH"]
    #[inline(always)]
    pub fn totalh(&self) -> TOTALH_R {
        TOTALH_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - TOTALW"]
    #[inline(always)]
    pub fn totalw(&self) -> TOTALW_R {
        TOTALW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - TOTALH"]
    #[inline(always)]
    pub fn totalh(&mut self) -> TOTALH_W {
        TOTALH_W { w: self }
    }
    #[doc = "Bits 16:27 - TOTALW"]
    #[inline(always)]
    pub fn totalw(&mut self) -> TOTALW_W {
        TOTALW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the accumulated number of horizontal synchronization, back porch, active and front porch pixels minus 1 (HSYNCwidth+HBP+activewidth+HFP-1) and the accumulated number of vertical synchronization, back porch lines, active and front lines minus 1 (VSYNCheight+BVBP+activeheight+VFP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_twcr](index.html) module"]
pub struct LTDC_TWCR_SPEC;
impl crate::RegisterSpec for LTDC_TWCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_twcr::R](R) reader structure"]
impl crate::Readable for LTDC_TWCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltdc_twcr::W](W) writer structure"]
impl crate::Writable for LTDC_TWCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTDC_TWCR to value 0"]
impl crate::Resettable for LTDC_TWCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
