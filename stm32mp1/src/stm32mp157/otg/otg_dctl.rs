#[doc = "Register `OTG_DCTL` reader"]
pub struct R(crate::R<OTG_DCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_DCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_DCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_DCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_DCTL` writer"]
pub struct W(crate::W<OTG_DCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_DCTL_SPEC>;
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
impl From<crate::W<OTG_DCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_DCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWUSIG` reader - RWUSIG"]
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
#[doc = "Field `RWUSIG` writer - RWUSIG"]
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
#[doc = "Field `SDIS` reader - SDIS"]
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
#[doc = "Field `SDIS` writer - SDIS"]
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
#[doc = "Field `GINSTS` reader - GINSTS"]
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
#[doc = "Field `GONSTS` reader - GONSTS"]
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
#[doc = "Field `TCTL` reader - TCTL"]
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
#[doc = "Field `TCTL` writer - TCTL"]
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
#[doc = "Field `SGINAK` writer - SGINAK"]
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
#[doc = "Field `CGINAK` writer - CGINAK"]
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
#[doc = "Field `SGONAK` writer - SGONAK"]
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
#[doc = "Field `CGONAK` writer - CGONAK"]
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
#[doc = "Field `POPRGDNE` reader - POPRGDNE"]
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
#[doc = "Field `POPRGDNE` writer - POPRGDNE"]
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
#[doc = "Field `DSBESLRJCT` reader - DSBESLRJCT"]
pub struct DSBESLRJCT_R(crate::FieldReader<bool, bool>);
impl DSBESLRJCT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSBESLRJCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSBESLRJCT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSBESLRJCT` writer - DSBESLRJCT"]
pub struct DSBESLRJCT_W<'a> {
    w: &'a mut W,
}
impl<'a> DSBESLRJCT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RWUSIG"]
    #[inline(always)]
    pub fn rwusig(&self) -> RWUSIG_R {
        RWUSIG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SDIS"]
    #[inline(always)]
    pub fn sdis(&self) -> SDIS_R {
        SDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GINSTS"]
    #[inline(always)]
    pub fn ginsts(&self) -> GINSTS_R {
        GINSTS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GONSTS"]
    #[inline(always)]
    pub fn gonsts(&self) -> GONSTS_R {
        GONSTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - TCTL"]
    #[inline(always)]
    pub fn tctl(&self) -> TCTL_R {
        TCTL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 11 - POPRGDNE"]
    #[inline(always)]
    pub fn poprgdne(&self) -> POPRGDNE_R {
        POPRGDNE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DSBESLRJCT"]
    #[inline(always)]
    pub fn dsbeslrjct(&self) -> DSBESLRJCT_R {
        DSBESLRJCT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RWUSIG"]
    #[inline(always)]
    pub fn rwusig(&mut self) -> RWUSIG_W {
        RWUSIG_W { w: self }
    }
    #[doc = "Bit 1 - SDIS"]
    #[inline(always)]
    pub fn sdis(&mut self) -> SDIS_W {
        SDIS_W { w: self }
    }
    #[doc = "Bits 4:6 - TCTL"]
    #[inline(always)]
    pub fn tctl(&mut self) -> TCTL_W {
        TCTL_W { w: self }
    }
    #[doc = "Bit 7 - SGINAK"]
    #[inline(always)]
    pub fn sginak(&mut self) -> SGINAK_W {
        SGINAK_W { w: self }
    }
    #[doc = "Bit 8 - CGINAK"]
    #[inline(always)]
    pub fn cginak(&mut self) -> CGINAK_W {
        CGINAK_W { w: self }
    }
    #[doc = "Bit 9 - SGONAK"]
    #[inline(always)]
    pub fn sgonak(&mut self) -> SGONAK_W {
        SGONAK_W { w: self }
    }
    #[doc = "Bit 10 - CGONAK"]
    #[inline(always)]
    pub fn cgonak(&mut self) -> CGONAK_W {
        CGONAK_W { w: self }
    }
    #[doc = "Bit 11 - POPRGDNE"]
    #[inline(always)]
    pub fn poprgdne(&mut self) -> POPRGDNE_W {
        POPRGDNE_W { w: self }
    }
    #[doc = "Bit 18 - DSBESLRJCT"]
    #[inline(always)]
    pub fn dsbeslrjct(&mut self) -> DSBESLRJCT_W {
        DSBESLRJCT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG device control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dctl](index.html) module"]
pub struct OTG_DCTL_SPEC;
impl crate::RegisterSpec for OTG_DCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_dctl::R](R) reader structure"]
impl crate::Readable for OTG_DCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_dctl::W](W) writer structure"]
impl crate::Writable for OTG_DCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_DCTL to value 0x02"]
impl crate::Resettable for OTG_DCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}