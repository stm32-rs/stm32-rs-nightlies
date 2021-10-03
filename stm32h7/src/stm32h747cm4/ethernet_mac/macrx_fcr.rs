#[doc = "Register `MACRxFCR` reader"]
pub struct R(crate::R<MACRXFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACRXFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACRXFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACRXFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACRxFCR` writer"]
pub struct W(crate::W<MACRXFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACRXFCR_SPEC>;
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
impl From<crate::W<MACRXFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACRXFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFE` reader - Receive Flow Control Enable"]
pub struct RFE_R(crate::FieldReader<bool, bool>);
impl RFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFE` writer - Receive Flow Control Enable"]
pub struct RFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFE_W<'a> {
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
#[doc = "Field `UP` reader - Unicast Pause Packet Detect"]
pub struct UP_R(crate::FieldReader<bool, bool>);
impl UP_R {
    pub(crate) fn new(bits: bool) -> Self {
        UP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UP` writer - Unicast Pause Packet Detect"]
pub struct UP_W<'a> {
    w: &'a mut W,
}
impl<'a> UP_W<'a> {
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
    #[doc = "Bit 0 - Receive Flow Control Enable"]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Unicast Pause Packet Detect"]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Flow Control Enable"]
    #[inline(always)]
    pub fn rfe(&mut self) -> RFE_W {
        RFE_W { w: self }
    }
    #[doc = "Bit 1 - Unicast Pause Packet Detect"]
    #[inline(always)]
    pub fn up(&mut self) -> UP_W {
        UP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx flow control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macrx_fcr](index.html) module"]
pub struct MACRXFCR_SPEC;
impl crate::RegisterSpec for MACRXFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macrx_fcr::R](R) reader structure"]
impl crate::Readable for MACRXFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macrx_fcr::W](W) writer structure"]
impl crate::Writable for MACRXFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACRxFCR to value 0"]
impl crate::Resettable for MACRXFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
