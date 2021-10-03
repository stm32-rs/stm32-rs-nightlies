#[doc = "Register `OTG_FS_GADPCTL` reader"]
pub struct R(crate::R<OTG_FS_GADPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_FS_GADPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_FS_GADPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_FS_GADPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_FS_GADPCTL` writer"]
pub struct W(crate::W<OTG_FS_GADPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_FS_GADPCTL_SPEC>;
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
impl From<crate::W<OTG_FS_GADPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_FS_GADPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRBDSCHG` reader - Probe discharge"]
pub struct PRBDSCHG_R(crate::FieldReader<u8, u8>);
impl PRBDSCHG_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRBDSCHG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRBDSCHG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRBDSCHG` writer - Probe discharge"]
pub struct PRBDSCHG_W<'a> {
    w: &'a mut W,
}
impl<'a> PRBDSCHG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `PRBDELTA` reader - Probe delta"]
pub struct PRBDELTA_R(crate::FieldReader<u8, u8>);
impl PRBDELTA_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRBDELTA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRBDELTA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRBDELTA` writer - Probe delta"]
pub struct PRBDELTA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRBDELTA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `PRBPER` reader - Probe period"]
pub struct PRBPER_R(crate::FieldReader<u8, u8>);
impl PRBPER_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRBPER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRBPER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRBPER` writer - Probe period"]
pub struct PRBPER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRBPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `RTIM` reader - Ramp time"]
pub struct RTIM_R(crate::FieldReader<u16, u16>);
impl RTIM_R {
    pub(crate) fn new(bits: u16) -> Self {
        RTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTIM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENAPRB` reader - Enable probe"]
pub struct ENAPRB_R(crate::FieldReader<bool, bool>);
impl ENAPRB_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENAPRB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENAPRB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENAPRB` writer - Enable probe"]
pub struct ENAPRB_W<'a> {
    w: &'a mut W,
}
impl<'a> ENAPRB_W<'a> {
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
#[doc = "Field `ENASNS` reader - Enable sense"]
pub struct ENASNS_R(crate::FieldReader<bool, bool>);
impl ENASNS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENASNS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENASNS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENASNS` writer - Enable sense"]
pub struct ENASNS_W<'a> {
    w: &'a mut W,
}
impl<'a> ENASNS_W<'a> {
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
#[doc = "Field `ADPRST` reader - ADP reset"]
pub struct ADPRST_R(crate::FieldReader<bool, bool>);
impl ADPRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPEN` reader - ADP enable"]
pub struct ADPEN_R(crate::FieldReader<bool, bool>);
impl ADPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPEN` writer - ADP enable"]
pub struct ADPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPEN_W<'a> {
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
#[doc = "Field `ADPPRBIF` reader - ADP probe interrupt flag"]
pub struct ADPPRBIF_R(crate::FieldReader<bool, bool>);
impl ADPPRBIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPPRBIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPPRBIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPPRBIF` writer - ADP probe interrupt flag"]
pub struct ADPPRBIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPPRBIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `ADPSNSIF` reader - ADP sense interrupt flag"]
pub struct ADPSNSIF_R(crate::FieldReader<bool, bool>);
impl ADPSNSIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPSNSIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPSNSIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPSNSIF` writer - ADP sense interrupt flag"]
pub struct ADPSNSIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPSNSIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `ADPTOIF` reader - ADP timeout interrupt flag"]
pub struct ADPTOIF_R(crate::FieldReader<bool, bool>);
impl ADPTOIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPTOIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPTOIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPTOIF` writer - ADP timeout interrupt flag"]
pub struct ADPTOIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPTOIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `ADPPRBIM` reader - ADP probe interrupt mask"]
pub struct ADPPRBIM_R(crate::FieldReader<bool, bool>);
impl ADPPRBIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPPRBIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPPRBIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPPRBIM` writer - ADP probe interrupt mask"]
pub struct ADPPRBIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPPRBIM_W<'a> {
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
#[doc = "Field `ADPSNSIM` reader - ADP sense interrupt mask"]
pub struct ADPSNSIM_R(crate::FieldReader<bool, bool>);
impl ADPSNSIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPSNSIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPSNSIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPSNSIM` writer - ADP sense interrupt mask"]
pub struct ADPSNSIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPSNSIM_W<'a> {
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
#[doc = "Field `ADPTOIM` reader - ADP timeout interrupt mask"]
pub struct ADPTOIM_R(crate::FieldReader<bool, bool>);
impl ADPTOIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPTOIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPTOIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPTOIM` writer - ADP timeout interrupt mask"]
pub struct ADPTOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPTOIM_W<'a> {
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
#[doc = "Field `AR` reader - Access request"]
pub struct AR_R(crate::FieldReader<u8, u8>);
impl AR_R {
    pub(crate) fn new(bits: u8) -> Self {
        AR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR` writer - Access request"]
pub struct AR_W<'a> {
    w: &'a mut W,
}
impl<'a> AR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | ((value as u32 & 0x03) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Probe discharge"]
    #[inline(always)]
    pub fn prbdschg(&self) -> PRBDSCHG_R {
        PRBDSCHG_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Probe delta"]
    #[inline(always)]
    pub fn prbdelta(&self) -> PRBDELTA_R {
        PRBDELTA_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Probe period"]
    #[inline(always)]
    pub fn prbper(&self) -> PRBPER_R {
        PRBPER_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:16 - Ramp time"]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 6) & 0x07ff) as u16)
    }
    #[doc = "Bit 17 - Enable probe"]
    #[inline(always)]
    pub fn enaprb(&self) -> ENAPRB_R {
        ENAPRB_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable sense"]
    #[inline(always)]
    pub fn enasns(&self) -> ENASNS_R {
        ENASNS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ADP reset"]
    #[inline(always)]
    pub fn adprst(&self) -> ADPRST_R {
        ADPRST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ADP enable"]
    #[inline(always)]
    pub fn adpen(&self) -> ADPEN_R {
        ADPEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ADP probe interrupt flag"]
    #[inline(always)]
    pub fn adpprbif(&self) -> ADPPRBIF_R {
        ADPPRBIF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ADP sense interrupt flag"]
    #[inline(always)]
    pub fn adpsnsif(&self) -> ADPSNSIF_R {
        ADPSNSIF_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ADP timeout interrupt flag"]
    #[inline(always)]
    pub fn adptoif(&self) -> ADPTOIF_R {
        ADPTOIF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADP probe interrupt mask"]
    #[inline(always)]
    pub fn adpprbim(&self) -> ADPPRBIM_R {
        ADPPRBIM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - ADP sense interrupt mask"]
    #[inline(always)]
    pub fn adpsnsim(&self) -> ADPSNSIM_R {
        ADPSNSIM_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - ADP timeout interrupt mask"]
    #[inline(always)]
    pub fn adptoim(&self) -> ADPTOIM_R {
        ADPTOIM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:28 - Access request"]
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new(((self.bits >> 27) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Probe discharge"]
    #[inline(always)]
    pub fn prbdschg(&mut self) -> PRBDSCHG_W {
        PRBDSCHG_W { w: self }
    }
    #[doc = "Bits 2:3 - Probe delta"]
    #[inline(always)]
    pub fn prbdelta(&mut self) -> PRBDELTA_W {
        PRBDELTA_W { w: self }
    }
    #[doc = "Bits 4:5 - Probe period"]
    #[inline(always)]
    pub fn prbper(&mut self) -> PRBPER_W {
        PRBPER_W { w: self }
    }
    #[doc = "Bit 17 - Enable probe"]
    #[inline(always)]
    pub fn enaprb(&mut self) -> ENAPRB_W {
        ENAPRB_W { w: self }
    }
    #[doc = "Bit 18 - Enable sense"]
    #[inline(always)]
    pub fn enasns(&mut self) -> ENASNS_W {
        ENASNS_W { w: self }
    }
    #[doc = "Bit 20 - ADP enable"]
    #[inline(always)]
    pub fn adpen(&mut self) -> ADPEN_W {
        ADPEN_W { w: self }
    }
    #[doc = "Bit 21 - ADP probe interrupt flag"]
    #[inline(always)]
    pub fn adpprbif(&mut self) -> ADPPRBIF_W {
        ADPPRBIF_W { w: self }
    }
    #[doc = "Bit 22 - ADP sense interrupt flag"]
    #[inline(always)]
    pub fn adpsnsif(&mut self) -> ADPSNSIF_W {
        ADPSNSIF_W { w: self }
    }
    #[doc = "Bit 23 - ADP timeout interrupt flag"]
    #[inline(always)]
    pub fn adptoif(&mut self) -> ADPTOIF_W {
        ADPTOIF_W { w: self }
    }
    #[doc = "Bit 24 - ADP probe interrupt mask"]
    #[inline(always)]
    pub fn adpprbim(&mut self) -> ADPPRBIM_W {
        ADPPRBIM_W { w: self }
    }
    #[doc = "Bit 25 - ADP sense interrupt mask"]
    #[inline(always)]
    pub fn adpsnsim(&mut self) -> ADPSNSIM_W {
        ADPSNSIM_W { w: self }
    }
    #[doc = "Bit 26 - ADP timeout interrupt mask"]
    #[inline(always)]
    pub fn adptoim(&mut self) -> ADPTOIM_W {
        ADPTOIM_W { w: self }
    }
    #[doc = "Bits 27:28 - Access request"]
    #[inline(always)]
    pub fn ar(&mut self) -> AR_W {
        AR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG ADP timer, control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_gadpctl](index.html) module"]
pub struct OTG_FS_GADPCTL_SPEC;
impl crate::RegisterSpec for OTG_FS_GADPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_fs_gadpctl::R](R) reader structure"]
impl crate::Readable for OTG_FS_GADPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_fs_gadpctl::W](W) writer structure"]
impl crate::Writable for OTG_FS_GADPCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_FS_GADPCTL to value 0x0200_0400"]
impl crate::Resettable for OTG_FS_GADPCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0400
    }
}
