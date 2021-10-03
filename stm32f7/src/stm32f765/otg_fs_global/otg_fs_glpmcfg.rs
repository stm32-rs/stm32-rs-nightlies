#[doc = "Register `OTG_FS_GLPMCFG` reader"]
pub struct R(crate::R<OTG_FS_GLPMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_FS_GLPMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_FS_GLPMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_FS_GLPMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_FS_GLPMCFG` writer"]
pub struct W(crate::W<OTG_FS_GLPMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_FS_GLPMCFG_SPEC>;
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
impl From<crate::W<OTG_FS_GLPMCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_FS_GLPMCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPMEN` reader - LPM support enable"]
pub struct LPMEN_R(crate::FieldReader<bool, bool>);
impl LPMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMEN` writer - LPM support enable"]
pub struct LPMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMEN_W<'a> {
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
#[doc = "Field `LPMACK` reader - LPM token acknowledge enable"]
pub struct LPMACK_R(crate::FieldReader<bool, bool>);
impl LPMACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPMACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPMACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMACK` writer - LPM token acknowledge enable"]
pub struct LPMACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMACK_W<'a> {
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
#[doc = "Field `BESL` reader - Best effort service latency"]
pub struct BESL_R(crate::FieldReader<u8, u8>);
impl BESL_R {
    pub(crate) fn new(bits: u8) -> Self {
        BESL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BESL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BESL` writer - Best effort service latency"]
pub struct BESL_W<'a> {
    w: &'a mut W,
}
impl<'a> BESL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | ((value as u32 & 0x0f) << 2);
        self.w
    }
}
#[doc = "Field `REMWAKE` reader - bRemoteWake value"]
pub struct REMWAKE_R(crate::FieldReader<bool, bool>);
impl REMWAKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REMWAKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REMWAKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REMWAKE` writer - bRemoteWake value"]
pub struct REMWAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> REMWAKE_W<'a> {
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
#[doc = "Field `L1SSEN` reader - L1 Shallow Sleep enable"]
pub struct L1SSEN_R(crate::FieldReader<bool, bool>);
impl L1SSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        L1SSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L1SSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L1SSEN` writer - L1 Shallow Sleep enable"]
pub struct L1SSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> L1SSEN_W<'a> {
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
#[doc = "Field `BESLTHRS` reader - BESL threshold"]
pub struct BESLTHRS_R(crate::FieldReader<u8, u8>);
impl BESLTHRS_R {
    pub(crate) fn new(bits: u8) -> Self {
        BESLTHRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BESLTHRS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BESLTHRS` writer - BESL threshold"]
pub struct BESLTHRS_W<'a> {
    w: &'a mut W,
}
impl<'a> BESLTHRS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `L1DSEN` reader - L1 deep sleep enable"]
pub struct L1DSEN_R(crate::FieldReader<bool, bool>);
impl L1DSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        L1DSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L1DSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L1DSEN` writer - L1 deep sleep enable"]
pub struct L1DSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> L1DSEN_W<'a> {
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
#[doc = "Field `LPMRST` reader - LPM response"]
pub struct LPMRST_R(crate::FieldReader<u8, u8>);
impl LPMRST_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPMRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPMRST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLPSTS` reader - Port sleep status"]
pub struct SLPSTS_R(crate::FieldReader<bool, bool>);
impl SLPSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLPSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLPSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L1RSMOK` reader - Sleep State Resume OK"]
pub struct L1RSMOK_R(crate::FieldReader<bool, bool>);
impl L1RSMOK_R {
    pub(crate) fn new(bits: bool) -> Self {
        L1RSMOK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L1RSMOK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMCHIDX` reader - LPM Channel Index"]
pub struct LPMCHIDX_R(crate::FieldReader<u8, u8>);
impl LPMCHIDX_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPMCHIDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPMCHIDX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMCHIDX` writer - LPM Channel Index"]
pub struct LPMCHIDX_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMCHIDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 17)) | ((value as u32 & 0x0f) << 17);
        self.w
    }
}
#[doc = "Field `LPMRCNT` reader - LPM retry count"]
pub struct LPMRCNT_R(crate::FieldReader<u8, u8>);
impl LPMRCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPMRCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPMRCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMRCNT` writer - LPM retry count"]
pub struct LPMRCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMRCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | ((value as u32 & 0x07) << 21);
        self.w
    }
}
#[doc = "Field `SNDLPM` reader - Send LPM transaction"]
pub struct SNDLPM_R(crate::FieldReader<bool, bool>);
impl SNDLPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SNDLPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SNDLPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SNDLPM` writer - Send LPM transaction"]
pub struct SNDLPM_W<'a> {
    w: &'a mut W,
}
impl<'a> SNDLPM_W<'a> {
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
#[doc = "Field `LPMRCNTSTS` reader - LPM retry count status"]
pub struct LPMRCNTSTS_R(crate::FieldReader<u8, u8>);
impl LPMRCNTSTS_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPMRCNTSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPMRCNTSTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENBESL` reader - Enable best effort service latency"]
pub struct ENBESL_R(crate::FieldReader<bool, bool>);
impl ENBESL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENBESL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENBESL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENBESL` writer - Enable best effort service latency"]
pub struct ENBESL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBESL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LPM support enable"]
    #[inline(always)]
    pub fn lpmen(&self) -> LPMEN_R {
        LPMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LPM token acknowledge enable"]
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - Best effort service latency"]
    #[inline(always)]
    pub fn besl(&self) -> BESL_R {
        BESL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - bRemoteWake value"]
    #[inline(always)]
    pub fn remwake(&self) -> REMWAKE_R {
        REMWAKE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - L1 Shallow Sleep enable"]
    #[inline(always)]
    pub fn l1ssen(&self) -> L1SSEN_R {
        L1SSEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - BESL threshold"]
    #[inline(always)]
    pub fn beslthrs(&self) -> BESLTHRS_R {
        BESLTHRS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - L1 deep sleep enable"]
    #[inline(always)]
    pub fn l1dsen(&self) -> L1DSEN_R {
        L1DSEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - LPM response"]
    #[inline(always)]
    pub fn lpmrst(&self) -> LPMRST_R {
        LPMRST_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Port sleep status"]
    #[inline(always)]
    pub fn slpsts(&self) -> SLPSTS_R {
        SLPSTS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Sleep State Resume OK"]
    #[inline(always)]
    pub fn l1rsmok(&self) -> L1RSMOK_R {
        L1RSMOK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:20 - LPM Channel Index"]
    #[inline(always)]
    pub fn lpmchidx(&self) -> LPMCHIDX_R {
        LPMCHIDX_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:23 - LPM retry count"]
    #[inline(always)]
    pub fn lpmrcnt(&self) -> LPMRCNT_R {
        LPMRCNT_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bit 24 - Send LPM transaction"]
    #[inline(always)]
    pub fn sndlpm(&self) -> SNDLPM_R {
        SNDLPM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:27 - LPM retry count status"]
    #[inline(always)]
    pub fn lpmrcntsts(&self) -> LPMRCNTSTS_R {
        LPMRCNTSTS_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bit 28 - Enable best effort service latency"]
    #[inline(always)]
    pub fn enbesl(&self) -> ENBESL_R {
        ENBESL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPM support enable"]
    #[inline(always)]
    pub fn lpmen(&mut self) -> LPMEN_W {
        LPMEN_W { w: self }
    }
    #[doc = "Bit 1 - LPM token acknowledge enable"]
    #[inline(always)]
    pub fn lpmack(&mut self) -> LPMACK_W {
        LPMACK_W { w: self }
    }
    #[doc = "Bits 2:5 - Best effort service latency"]
    #[inline(always)]
    pub fn besl(&mut self) -> BESL_W {
        BESL_W { w: self }
    }
    #[doc = "Bit 6 - bRemoteWake value"]
    #[inline(always)]
    pub fn remwake(&mut self) -> REMWAKE_W {
        REMWAKE_W { w: self }
    }
    #[doc = "Bit 7 - L1 Shallow Sleep enable"]
    #[inline(always)]
    pub fn l1ssen(&mut self) -> L1SSEN_W {
        L1SSEN_W { w: self }
    }
    #[doc = "Bits 8:11 - BESL threshold"]
    #[inline(always)]
    pub fn beslthrs(&mut self) -> BESLTHRS_W {
        BESLTHRS_W { w: self }
    }
    #[doc = "Bit 12 - L1 deep sleep enable"]
    #[inline(always)]
    pub fn l1dsen(&mut self) -> L1DSEN_W {
        L1DSEN_W { w: self }
    }
    #[doc = "Bits 17:20 - LPM Channel Index"]
    #[inline(always)]
    pub fn lpmchidx(&mut self) -> LPMCHIDX_W {
        LPMCHIDX_W { w: self }
    }
    #[doc = "Bits 21:23 - LPM retry count"]
    #[inline(always)]
    pub fn lpmrcnt(&mut self) -> LPMRCNT_W {
        LPMRCNT_W { w: self }
    }
    #[doc = "Bit 24 - Send LPM transaction"]
    #[inline(always)]
    pub fn sndlpm(&mut self) -> SNDLPM_W {
        SNDLPM_W { w: self }
    }
    #[doc = "Bit 28 - Enable best effort service latency"]
    #[inline(always)]
    pub fn enbesl(&mut self) -> ENBESL_W {
        ENBESL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG core LPM configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_glpmcfg](index.html) module"]
pub struct OTG_FS_GLPMCFG_SPEC;
impl crate::RegisterSpec for OTG_FS_GLPMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_fs_glpmcfg::R](R) reader structure"]
impl crate::Readable for OTG_FS_GLPMCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_fs_glpmcfg::W](W) writer structure"]
impl crate::Writable for OTG_FS_GLPMCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_FS_GLPMCFG to value 0x0200_0400"]
impl crate::Resettable for OTG_FS_GLPMCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0400
    }
}
