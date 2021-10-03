#[doc = "Register `MACVTR` reader"]
pub struct R(crate::R<MACVTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACVTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACVTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACVTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACVTR` writer"]
pub struct W(crate::W<MACVTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACVTR_SPEC>;
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
impl From<crate::W<MACVTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACVTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VL` reader - VLAN Tag Identifier for Receive Packets"]
pub struct VL_R(crate::FieldReader<u16, u16>);
impl VL_R {
    pub(crate) fn new(bits: u16) -> Self {
        VL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VL` writer - VLAN Tag Identifier for Receive Packets"]
pub struct VL_W<'a> {
    w: &'a mut W,
}
impl<'a> VL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `ETV` reader - Enable 12-Bit VLAN Tag Comparison"]
pub struct ETV_R(crate::FieldReader<bool, bool>);
impl ETV_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETV` writer - Enable 12-Bit VLAN Tag Comparison"]
pub struct ETV_W<'a> {
    w: &'a mut W,
}
impl<'a> ETV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `VTIM` reader - VLAN Tag Inverse Match Enable"]
pub struct VTIM_R(crate::FieldReader<bool, bool>);
impl VTIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTIM` writer - VLAN Tag Inverse Match Enable"]
pub struct VTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> VTIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `ESVL` reader - Enable S-VLAN"]
pub struct ESVL_R(crate::FieldReader<bool, bool>);
impl ESVL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ESVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESVL` writer - Enable S-VLAN"]
pub struct ESVL_W<'a> {
    w: &'a mut W,
}
impl<'a> ESVL_W<'a> {
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
#[doc = "Field `ERSVLM` reader - Enable Receive S-VLAN Match"]
pub struct ERSVLM_R(crate::FieldReader<bool, bool>);
impl ERSVLM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERSVLM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERSVLM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERSVLM` writer - Enable Receive S-VLAN Match"]
pub struct ERSVLM_W<'a> {
    w: &'a mut W,
}
impl<'a> ERSVLM_W<'a> {
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
#[doc = "Field `DOVLTC` reader - Disable VLAN Type Check"]
pub struct DOVLTC_R(crate::FieldReader<bool, bool>);
impl DOVLTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOVLTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOVLTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOVLTC` writer - Disable VLAN Type Check"]
pub struct DOVLTC_W<'a> {
    w: &'a mut W,
}
impl<'a> DOVLTC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `EVLS` reader - Enable VLAN Tag Stripping on Receive"]
pub struct EVLS_R(crate::FieldReader<u8, u8>);
impl EVLS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EVLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVLS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVLS` writer - Enable VLAN Tag Stripping on Receive"]
pub struct EVLS_W<'a> {
    w: &'a mut W,
}
impl<'a> EVLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Field `EVLRXS` reader - Enable VLAN Tag in Rx status"]
pub struct EVLRXS_R(crate::FieldReader<bool, bool>);
impl EVLRXS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EVLRXS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVLRXS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVLRXS` writer - Enable VLAN Tag in Rx status"]
pub struct EVLRXS_W<'a> {
    w: &'a mut W,
}
impl<'a> EVLRXS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `VTHM` reader - VLAN Tag Hash Table Match Enable"]
pub struct VTHM_R(crate::FieldReader<bool, bool>);
impl VTHM_R {
    pub(crate) fn new(bits: bool) -> Self {
        VTHM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTHM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTHM` writer - VLAN Tag Hash Table Match Enable"]
pub struct VTHM_W<'a> {
    w: &'a mut W,
}
impl<'a> VTHM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `EDVLP` reader - Enable Double VLAN Processing"]
pub struct EDVLP_R(crate::FieldReader<bool, bool>);
impl EDVLP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDVLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDVLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDVLP` writer - Enable Double VLAN Processing"]
pub struct EDVLP_W<'a> {
    w: &'a mut W,
}
impl<'a> EDVLP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `ERIVLT` reader - Enable Inner VLAN Tag"]
pub struct ERIVLT_R(crate::FieldReader<bool, bool>);
impl ERIVLT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERIVLT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERIVLT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERIVLT` writer - Enable Inner VLAN Tag"]
pub struct ERIVLT_W<'a> {
    w: &'a mut W,
}
impl<'a> ERIVLT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `EIVLS` reader - Enable Inner VLAN Tag Stripping on Receive"]
pub struct EIVLS_R(crate::FieldReader<u8, u8>);
impl EIVLS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EIVLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EIVLS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EIVLS` writer - Enable Inner VLAN Tag Stripping on Receive"]
pub struct EIVLS_W<'a> {
    w: &'a mut W,
}
impl<'a> EIVLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `EIVLRXS` reader - Enable Inner VLAN Tag in Rx Status"]
pub struct EIVLRXS_R(crate::FieldReader<bool, bool>);
impl EIVLRXS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EIVLRXS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EIVLRXS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EIVLRXS` writer - Enable Inner VLAN Tag in Rx Status"]
pub struct EIVLRXS_W<'a> {
    w: &'a mut W,
}
impl<'a> EIVLRXS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Packets"]
    #[inline(always)]
    pub fn vl(&self) -> VL_R {
        VL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable"]
    #[inline(always)]
    pub fn vtim(&self) -> VTIM_R {
        VTIM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable S-VLAN"]
    #[inline(always)]
    pub fn esvl(&self) -> ESVL_R {
        ESVL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable Receive S-VLAN Match"]
    #[inline(always)]
    pub fn ersvlm(&self) -> ERSVLM_R {
        ERSVLM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Disable VLAN Type Check"]
    #[inline(always)]
    pub fn dovltc(&self) -> DOVLTC_R {
        DOVLTC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - Enable VLAN Tag Stripping on Receive"]
    #[inline(always)]
    pub fn evls(&self) -> EVLS_R {
        EVLS_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 24 - Enable VLAN Tag in Rx status"]
    #[inline(always)]
    pub fn evlrxs(&self) -> EVLRXS_R {
        EVLRXS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - VLAN Tag Hash Table Match Enable"]
    #[inline(always)]
    pub fn vthm(&self) -> VTHM_R {
        VTHM_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable Double VLAN Processing"]
    #[inline(always)]
    pub fn edvlp(&self) -> EDVLP_R {
        EDVLP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable Inner VLAN Tag"]
    #[inline(always)]
    pub fn erivlt(&self) -> ERIVLT_R {
        ERIVLT_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Enable Inner VLAN Tag Stripping on Receive"]
    #[inline(always)]
    pub fn eivls(&self) -> EIVLS_R {
        EIVLS_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 31 - Enable Inner VLAN Tag in Rx Status"]
    #[inline(always)]
    pub fn eivlrxs(&self) -> EIVLRXS_R {
        EIVLRXS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Packets"]
    #[inline(always)]
    pub fn vl(&mut self) -> VL_W {
        VL_W { w: self }
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    pub fn etv(&mut self) -> ETV_W {
        ETV_W { w: self }
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable"]
    #[inline(always)]
    pub fn vtim(&mut self) -> VTIM_W {
        VTIM_W { w: self }
    }
    #[doc = "Bit 18 - Enable S-VLAN"]
    #[inline(always)]
    pub fn esvl(&mut self) -> ESVL_W {
        ESVL_W { w: self }
    }
    #[doc = "Bit 19 - Enable Receive S-VLAN Match"]
    #[inline(always)]
    pub fn ersvlm(&mut self) -> ERSVLM_W {
        ERSVLM_W { w: self }
    }
    #[doc = "Bit 20 - Disable VLAN Type Check"]
    #[inline(always)]
    pub fn dovltc(&mut self) -> DOVLTC_W {
        DOVLTC_W { w: self }
    }
    #[doc = "Bits 21:22 - Enable VLAN Tag Stripping on Receive"]
    #[inline(always)]
    pub fn evls(&mut self) -> EVLS_W {
        EVLS_W { w: self }
    }
    #[doc = "Bit 24 - Enable VLAN Tag in Rx status"]
    #[inline(always)]
    pub fn evlrxs(&mut self) -> EVLRXS_W {
        EVLRXS_W { w: self }
    }
    #[doc = "Bit 25 - VLAN Tag Hash Table Match Enable"]
    #[inline(always)]
    pub fn vthm(&mut self) -> VTHM_W {
        VTHM_W { w: self }
    }
    #[doc = "Bit 26 - Enable Double VLAN Processing"]
    #[inline(always)]
    pub fn edvlp(&mut self) -> EDVLP_W {
        EDVLP_W { w: self }
    }
    #[doc = "Bit 27 - Enable Inner VLAN Tag"]
    #[inline(always)]
    pub fn erivlt(&mut self) -> ERIVLT_W {
        ERIVLT_W { w: self }
    }
    #[doc = "Bits 28:29 - Enable Inner VLAN Tag Stripping on Receive"]
    #[inline(always)]
    pub fn eivls(&mut self) -> EIVLS_W {
        EIVLS_W { w: self }
    }
    #[doc = "Bit 31 - Enable Inner VLAN Tag in Rx Status"]
    #[inline(always)]
    pub fn eivlrxs(&mut self) -> EIVLRXS_W {
        EIVLRXS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VLAN tag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macvtr](index.html) module"]
pub struct MACVTR_SPEC;
impl crate::RegisterSpec for MACVTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macvtr::R](R) reader structure"]
impl crate::Readable for MACVTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macvtr::W](W) writer structure"]
impl crate::Writable for MACVTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACVTR to value 0"]
impl crate::Resettable for MACVTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
