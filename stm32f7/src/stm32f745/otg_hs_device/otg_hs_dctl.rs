#[doc = "Register `OTG_HS_DCTL` reader"]
pub struct R(crate::R<OTG_HS_DCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HS_DCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HS_DCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HS_DCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_HS_DCTL` writer"]
pub struct W(crate::W<OTG_HS_DCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_HS_DCTL_SPEC>;
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
impl From<crate::W<OTG_HS_DCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_HS_DCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWUSIG` reader - Remote wakeup signaling"]
pub struct RWUSIG_R(crate::FieldReader<bool, bool>);
impl RWUSIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWUSIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWUSIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWUSIG` writer - Remote wakeup signaling"]
pub struct RWUSIG_W<'a> {
    w: &'a mut W,
}
impl<'a> RWUSIG_W<'a> {
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
#[doc = "Field `SDIS` reader - Soft disconnect"]
pub struct SDIS_R(crate::FieldReader<bool, bool>);
impl SDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIS` writer - Soft disconnect"]
pub struct SDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIS_W<'a> {
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
#[doc = "Field `GINSTS` reader - Global IN NAK status"]
pub struct GINSTS_R(crate::FieldReader<bool, bool>);
impl GINSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        GINSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GINSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GONSTS` reader - Global OUT NAK status"]
pub struct GONSTS_R(crate::FieldReader<bool, bool>);
impl GONSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        GONSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GONSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCTL` reader - Test control"]
pub struct TCTL_R(crate::FieldReader<u8, u8>);
impl TCTL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TCTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCTL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCTL` writer - Test control"]
pub struct TCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> TCTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `SGINAK` writer - Set global IN NAK"]
pub struct SGINAK_W<'a> {
    w: &'a mut W,
}
impl<'a> SGINAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CGINAK` writer - Clear global IN NAK"]
pub struct CGINAK_W<'a> {
    w: &'a mut W,
}
impl<'a> CGINAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `SGONAK` writer - Set global OUT NAK"]
pub struct SGONAK_W<'a> {
    w: &'a mut W,
}
impl<'a> SGONAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CGONAK` writer - Clear global OUT NAK"]
pub struct CGONAK_W<'a> {
    w: &'a mut W,
}
impl<'a> CGONAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `POPRGDNE` reader - Power-on programming done"]
pub struct POPRGDNE_R(crate::FieldReader<bool, bool>);
impl POPRGDNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        POPRGDNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POPRGDNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POPRGDNE` writer - Power-on programming done"]
pub struct POPRGDNE_W<'a> {
    w: &'a mut W,
}
impl<'a> POPRGDNE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Remote wakeup signaling"]
    #[inline(always)]
    pub fn rwusig(&self) -> RWUSIG_R {
        RWUSIG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    pub fn sdis(&self) -> SDIS_R {
        SDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Global IN NAK status"]
    #[inline(always)]
    pub fn ginsts(&self) -> GINSTS_R {
        GINSTS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Global OUT NAK status"]
    #[inline(always)]
    pub fn gonsts(&self) -> GONSTS_R {
        GONSTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Test control"]
    #[inline(always)]
    pub fn tctl(&self) -> TCTL_R {
        TCTL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Power-on programming done"]
    #[inline(always)]
    pub fn poprgdne(&self) -> POPRGDNE_R {
        POPRGDNE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remote wakeup signaling"]
    #[inline(always)]
    pub fn rwusig(&mut self) -> RWUSIG_W {
        RWUSIG_W { w: self }
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    pub fn sdis(&mut self) -> SDIS_W {
        SDIS_W { w: self }
    }
    #[doc = "Bits 4:6 - Test control"]
    #[inline(always)]
    pub fn tctl(&mut self) -> TCTL_W {
        TCTL_W { w: self }
    }
    #[doc = "Bit 7 - Set global IN NAK"]
    #[inline(always)]
    pub fn sginak(&mut self) -> SGINAK_W {
        SGINAK_W { w: self }
    }
    #[doc = "Bit 8 - Clear global IN NAK"]
    #[inline(always)]
    pub fn cginak(&mut self) -> CGINAK_W {
        CGINAK_W { w: self }
    }
    #[doc = "Bit 9 - Set global OUT NAK"]
    #[inline(always)]
    pub fn sgonak(&mut self) -> SGONAK_W {
        SGONAK_W { w: self }
    }
    #[doc = "Bit 10 - Clear global OUT NAK"]
    #[inline(always)]
    pub fn cgonak(&mut self) -> CGONAK_W {
        CGONAK_W { w: self }
    }
    #[doc = "Bit 11 - Power-on programming done"]
    #[inline(always)]
    pub fn poprgdne(&mut self) -> POPRGDNE_W {
        POPRGDNE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS device control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_dctl](index.html) module"]
pub struct OTG_HS_DCTL_SPEC;
impl crate::RegisterSpec for OTG_HS_DCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hs_dctl::R](R) reader structure"]
impl crate::Readable for OTG_HS_DCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_hs_dctl::W](W) writer structure"]
impl crate::Writable for OTG_HS_DCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_HS_DCTL to value 0"]
impl crate::Resettable for OTG_HS_DCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
