#[doc = "Register `OTG_DOEPMSK` reader"]
pub struct R(crate::R<OTG_DOEPMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_DOEPMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_DOEPMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_DOEPMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_DOEPMSK` writer"]
pub struct W(crate::W<OTG_DOEPMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_DOEPMSK_SPEC>;
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
impl From<crate::W<OTG_DOEPMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_DOEPMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRCM` reader - XFRCM"]
pub struct XFRCM_R(crate::FieldReader<bool, bool>);
impl XFRCM_R {
    pub(crate) fn new(bits: bool) -> Self {
        XFRCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XFRCM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XFRCM` writer - XFRCM"]
pub struct XFRCM_W<'a> {
    w: &'a mut W,
}
impl<'a> XFRCM_W<'a> {
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
#[doc = "Field `EPDM` reader - EPDM"]
pub struct EPDM_R(crate::FieldReader<bool, bool>);
impl EPDM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPDM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPDM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPDM` writer - EPDM"]
pub struct EPDM_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDM_W<'a> {
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
#[doc = "Field `AHBERRM` reader - AHBERRM"]
pub struct AHBERRM_R(crate::FieldReader<bool, bool>);
impl AHBERRM_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHBERRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBERRM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBERRM` writer - AHBERRM"]
pub struct AHBERRM_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBERRM_W<'a> {
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
#[doc = "Field `STUPM` reader - STUPM"]
pub struct STUPM_R(crate::FieldReader<bool, bool>);
impl STUPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        STUPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STUPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STUPM` writer - STUPM"]
pub struct STUPM_W<'a> {
    w: &'a mut W,
}
impl<'a> STUPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `OTEPDM` reader - OTEPDM"]
pub struct OTEPDM_R(crate::FieldReader<bool, bool>);
impl OTEPDM_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTEPDM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTEPDM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTEPDM` writer - OTEPDM"]
pub struct OTEPDM_W<'a> {
    w: &'a mut W,
}
impl<'a> OTEPDM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `STSPHSRXM` reader - STSPHSRXM"]
pub struct STSPHSRXM_R(crate::FieldReader<bool, bool>);
impl STSPHSRXM_R {
    pub(crate) fn new(bits: bool) -> Self {
        STSPHSRXM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STSPHSRXM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STSPHSRXM` writer - STSPHSRXM"]
pub struct STSPHSRXM_W<'a> {
    w: &'a mut W,
}
impl<'a> STSPHSRXM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `B2BSTUPM` reader - B2BSTUPM"]
pub struct B2BSTUPM_R(crate::FieldReader<bool, bool>);
impl B2BSTUPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2BSTUPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2BSTUPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2BSTUPM` writer - B2BSTUPM"]
pub struct B2BSTUPM_W<'a> {
    w: &'a mut W,
}
impl<'a> B2BSTUPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `OUTPKTERRM` reader - OUTPKTERRM"]
pub struct OUTPKTERRM_R(crate::FieldReader<bool, bool>);
impl OUTPKTERRM_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUTPKTERRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTPKTERRM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTPKTERRM` writer - OUTPKTERRM"]
pub struct OUTPKTERRM_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPKTERRM_W<'a> {
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
#[doc = "Field `BNAM` reader - BNAM"]
pub struct BNAM_R(crate::FieldReader<bool, bool>);
impl BNAM_R {
    pub(crate) fn new(bits: bool) -> Self {
        BNAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BNAM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BNAM` writer - BNAM"]
pub struct BNAM_W<'a> {
    w: &'a mut W,
}
impl<'a> BNAM_W<'a> {
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
#[doc = "Field `BERRM` reader - BERRM"]
pub struct BERRM_R(crate::FieldReader<bool, bool>);
impl BERRM_R {
    pub(crate) fn new(bits: bool) -> Self {
        BERRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BERRM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BERRM` writer - BERRM"]
pub struct BERRM_W<'a> {
    w: &'a mut W,
}
impl<'a> BERRM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `NAKMSK` reader - NAKMSK"]
pub struct NAKMSK_R(crate::FieldReader<bool, bool>);
impl NAKMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAKMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAKMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAKMSK` writer - NAKMSK"]
pub struct NAKMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `NYETMSK` reader - NYETMSK"]
pub struct NYETMSK_R(crate::FieldReader<bool, bool>);
impl NYETMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        NYETMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NYETMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NYETMSK` writer - NYETMSK"]
pub struct NYETMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> NYETMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - XFRCM"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EPDM"]
    #[inline(always)]
    pub fn epdm(&self) -> EPDM_R {
        EPDM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AHBERRM"]
    #[inline(always)]
    pub fn ahberrm(&self) -> AHBERRM_R {
        AHBERRM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - STUPM"]
    #[inline(always)]
    pub fn stupm(&self) -> STUPM_R {
        STUPM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OTEPDM"]
    #[inline(always)]
    pub fn otepdm(&self) -> OTEPDM_R {
        OTEPDM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - STSPHSRXM"]
    #[inline(always)]
    pub fn stsphsrxm(&self) -> STSPHSRXM_R {
        STSPHSRXM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B2BSTUPM"]
    #[inline(always)]
    pub fn b2bstupm(&self) -> B2BSTUPM_R {
        B2BSTUPM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - OUTPKTERRM"]
    #[inline(always)]
    pub fn outpkterrm(&self) -> OUTPKTERRM_R {
        OUTPKTERRM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BNAM"]
    #[inline(always)]
    pub fn bnam(&self) -> BNAM_R {
        BNAM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - BERRM"]
    #[inline(always)]
    pub fn berrm(&self) -> BERRM_R {
        BERRM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - NAKMSK"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - NYETMSK"]
    #[inline(always)]
    pub fn nyetmsk(&self) -> NYETMSK_R {
        NYETMSK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XFRCM"]
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XFRCM_W {
        XFRCM_W { w: self }
    }
    #[doc = "Bit 1 - EPDM"]
    #[inline(always)]
    pub fn epdm(&mut self) -> EPDM_W {
        EPDM_W { w: self }
    }
    #[doc = "Bit 2 - AHBERRM"]
    #[inline(always)]
    pub fn ahberrm(&mut self) -> AHBERRM_W {
        AHBERRM_W { w: self }
    }
    #[doc = "Bit 3 - STUPM"]
    #[inline(always)]
    pub fn stupm(&mut self) -> STUPM_W {
        STUPM_W { w: self }
    }
    #[doc = "Bit 4 - OTEPDM"]
    #[inline(always)]
    pub fn otepdm(&mut self) -> OTEPDM_W {
        OTEPDM_W { w: self }
    }
    #[doc = "Bit 5 - STSPHSRXM"]
    #[inline(always)]
    pub fn stsphsrxm(&mut self) -> STSPHSRXM_W {
        STSPHSRXM_W { w: self }
    }
    #[doc = "Bit 6 - B2BSTUPM"]
    #[inline(always)]
    pub fn b2bstupm(&mut self) -> B2BSTUPM_W {
        B2BSTUPM_W { w: self }
    }
    #[doc = "Bit 8 - OUTPKTERRM"]
    #[inline(always)]
    pub fn outpkterrm(&mut self) -> OUTPKTERRM_W {
        OUTPKTERRM_W { w: self }
    }
    #[doc = "Bit 9 - BNAM"]
    #[inline(always)]
    pub fn bnam(&mut self) -> BNAM_W {
        BNAM_W { w: self }
    }
    #[doc = "Bit 12 - BERRM"]
    #[inline(always)]
    pub fn berrm(&mut self) -> BERRM_W {
        BERRM_W { w: self }
    }
    #[doc = "Bit 13 - NAKMSK"]
    #[inline(always)]
    pub fn nakmsk(&mut self) -> NAKMSK_W {
        NAKMSK_W { w: self }
    }
    #[doc = "Bit 14 - NYETMSK"]
    #[inline(always)]
    pub fn nyetmsk(&mut self) -> NYETMSK_W {
        NYETMSK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register works with each of the OTG_DOEPINTx registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the OTG_DOEPINTx register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepmsk](index.html) module"]
pub struct OTG_DOEPMSK_SPEC;
impl crate::RegisterSpec for OTG_DOEPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_doepmsk::R](R) reader structure"]
impl crate::Readable for OTG_DOEPMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_doepmsk::W](W) writer structure"]
impl crate::Writable for OTG_DOEPMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_DOEPMSK to value 0"]
impl crate::Resettable for OTG_DOEPMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
