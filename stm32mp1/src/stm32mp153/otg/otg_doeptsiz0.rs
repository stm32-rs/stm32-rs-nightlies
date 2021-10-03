#[doc = "Register `OTG_DOEPTSIZ0` reader"]
pub struct R(crate::R<OTG_DOEPTSIZ0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_DOEPTSIZ0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_DOEPTSIZ0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_DOEPTSIZ0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_DOEPTSIZ0` writer"]
pub struct W(crate::W<OTG_DOEPTSIZ0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_DOEPTSIZ0_SPEC>;
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
impl From<crate::W<OTG_DOEPTSIZ0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_DOEPTSIZ0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRSIZ` reader - XFRSIZ"]
pub struct XFRSIZ_R(crate::FieldReader<u8, u8>);
impl XFRSIZ_R {
    pub(crate) fn new(bits: u8) -> Self {
        XFRSIZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XFRSIZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XFRSIZ` writer - XFRSIZ"]
pub struct XFRSIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> XFRSIZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `PKTCNT` reader - PKTCNT"]
pub struct PKTCNT_R(crate::FieldReader<bool, bool>);
impl PKTCNT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKTCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKTCNT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKTCNT` writer - PKTCNT"]
pub struct PKTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PKTCNT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `STUPCNT` reader - STUPCNT"]
pub struct STUPCNT_R(crate::FieldReader<u8, u8>);
impl STUPCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        STUPCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STUPCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STUPCNT` writer - STUPCNT"]
pub struct STUPCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> STUPCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | ((value as u32 & 0x03) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - XFRSIZ"]
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 19 - PKTCNT"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - STUPCNT"]
    #[inline(always)]
    pub fn stupcnt(&self) -> STUPCNT_R {
        STUPCNT_R::new(((self.bits >> 29) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - XFRSIZ"]
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W {
        XFRSIZ_W { w: self }
    }
    #[doc = "Bit 19 - PKTCNT"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W {
        PKTCNT_W { w: self }
    }
    #[doc = "Bits 29:30 - STUPCNT"]
    #[inline(always)]
    pub fn stupcnt(&mut self) -> STUPCNT_W {
        STUPCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The application must modify this register before enabling endpoint 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doeptsiz0](index.html) module"]
pub struct OTG_DOEPTSIZ0_SPEC;
impl crate::RegisterSpec for OTG_DOEPTSIZ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_doeptsiz0::R](R) reader structure"]
impl crate::Readable for OTG_DOEPTSIZ0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_doeptsiz0::W](W) writer structure"]
impl crate::Writable for OTG_DOEPTSIZ0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_DOEPTSIZ0 to value 0"]
impl crate::Resettable for OTG_DOEPTSIZ0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
