#[doc = "Register `OTG_HS_DVBUSPULSE` reader"]
pub struct R(crate::R<OTG_HS_DVBUSPULSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HS_DVBUSPULSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HS_DVBUSPULSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HS_DVBUSPULSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_HS_DVBUSPULSE` writer"]
pub struct W(crate::W<OTG_HS_DVBUSPULSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_HS_DVBUSPULSE_SPEC>;
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
impl From<crate::W<OTG_HS_DVBUSPULSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_HS_DVBUSPULSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DVBUSP` reader - Device VBUS pulsing time"]
pub struct DVBUSP_R(crate::FieldReader<u16, u16>);
impl DVBUSP_R {
    pub(crate) fn new(bits: u16) -> Self {
        DVBUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DVBUSP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DVBUSP` writer - Device VBUS pulsing time"]
pub struct DVBUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> DVBUSP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Device VBUS pulsing time"]
    #[inline(always)]
    pub fn dvbusp(&self) -> DVBUSP_R {
        DVBUSP_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Device VBUS pulsing time"]
    #[inline(always)]
    pub fn dvbusp(&mut self) -> DVBUSP_W {
        DVBUSP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS device VBUS pulsing time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dvbuspulse](index.html) module"]
pub struct OTG_HS_DVBUSPULSE_SPEC;
impl crate::RegisterSpec for OTG_HS_DVBUSPULSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hs_dvbuspulse::R](R) reader structure"]
impl crate::Readable for OTG_HS_DVBUSPULSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_hs_dvbuspulse::W](W) writer structure"]
impl crate::Writable for OTG_HS_DVBUSPULSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_HS_DVBUSPULSE to value 0x05b8"]
impl crate::Resettable for OTG_HS_DVBUSPULSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05b8
    }
}
