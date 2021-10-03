#[doc = "Register `OTG_FS_GOTGCTL` reader"]
pub struct R(crate::R<OTG_FS_GOTGCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_FS_GOTGCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_FS_GOTGCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_FS_GOTGCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_FS_GOTGCTL` writer"]
pub struct W(crate::W<OTG_FS_GOTGCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_FS_GOTGCTL_SPEC>;
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
impl From<crate::W<OTG_FS_GOTGCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_FS_GOTGCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRQSCS` reader - Session request success"]
pub struct SRQSCS_R(crate::FieldReader<bool, bool>);
impl SRQSCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRQSCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRQSCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRQ` reader - Session request"]
pub struct SRQ_R(crate::FieldReader<bool, bool>);
impl SRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRQ` writer - Session request"]
pub struct SRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SRQ_W<'a> {
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
#[doc = "Field `HNGSCS` reader - Host negotiation success"]
pub struct HNGSCS_R(crate::FieldReader<bool, bool>);
impl HNGSCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        HNGSCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HNGSCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HNPRQ` reader - HNP request"]
pub struct HNPRQ_R(crate::FieldReader<bool, bool>);
impl HNPRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        HNPRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HNPRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HNPRQ` writer - HNP request"]
pub struct HNPRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HNPRQ_W<'a> {
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
#[doc = "Field `HSHNPEN` reader - Host set HNP enable"]
pub struct HSHNPEN_R(crate::FieldReader<bool, bool>);
impl HSHNPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSHNPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSHNPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSHNPEN` writer - Host set HNP enable"]
pub struct HSHNPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSHNPEN_W<'a> {
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
#[doc = "Field `DHNPEN` reader - Device HNP enabled"]
pub struct DHNPEN_R(crate::FieldReader<bool, bool>);
impl DHNPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DHNPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DHNPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DHNPEN` writer - Device HNP enabled"]
pub struct DHNPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DHNPEN_W<'a> {
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
#[doc = "Field `CIDSTS` reader - Connector ID status"]
pub struct CIDSTS_R(crate::FieldReader<bool, bool>);
impl CIDSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CIDSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CIDSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBCT` reader - Long/short debounce time"]
pub struct DBCT_R(crate::FieldReader<bool, bool>);
impl DBCT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBCT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASVLD` reader - A-session valid"]
pub struct ASVLD_R(crate::FieldReader<bool, bool>);
impl ASVLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASVLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASVLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BSVLD` reader - B-session valid"]
pub struct BSVLD_R(crate::FieldReader<bool, bool>);
impl BSVLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BSVLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BSVLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBVALOEN` reader - VBUS valid override enable"]
pub struct VBVALOEN_R(crate::FieldReader<bool, bool>);
impl VBVALOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBVALOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBVALOEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBVALOEN` writer - VBUS valid override enable"]
pub struct VBVALOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBVALOEN_W<'a> {
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
#[doc = "Field `VBVALOVAL` reader - VBUS valid override value"]
pub struct VBVALOVAL_R(crate::FieldReader<bool, bool>);
impl VBVALOVAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBVALOVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBVALOVAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBVALOVAL` writer - VBUS valid override value"]
pub struct VBVALOVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> VBVALOVAL_W<'a> {
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
#[doc = "Field `AVALOEN` reader - A-peripheral session valid override enable"]
pub struct AVALOEN_R(crate::FieldReader<bool, bool>);
impl AVALOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AVALOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVALOEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVALOEN` writer - A-peripheral session valid override enable"]
pub struct AVALOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AVALOEN_W<'a> {
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
#[doc = "Field `AVALOVAL` reader - A-peripheral session valid override value"]
pub struct AVALOVAL_R(crate::FieldReader<bool, bool>);
impl AVALOVAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        AVALOVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVALOVAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVALOVAL` writer - A-peripheral session valid override value"]
pub struct AVALOVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> AVALOVAL_W<'a> {
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
#[doc = "Field `BVALOEN` reader - B-peripheral session valid override enable"]
pub struct BVALOEN_R(crate::FieldReader<bool, bool>);
impl BVALOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BVALOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BVALOEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BVALOEN` writer - B-peripheral session valid override enable"]
pub struct BVALOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BVALOEN_W<'a> {
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
#[doc = "Field `BVALOVAL` reader - B-peripheral session valid override value"]
pub struct BVALOVAL_R(crate::FieldReader<bool, bool>);
impl BVALOVAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        BVALOVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BVALOVAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BVALOVAL` writer - B-peripheral session valid override value"]
pub struct BVALOVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BVALOVAL_W<'a> {
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
#[doc = "Field `EHEN` reader - Embedded host enable"]
pub struct EHEN_R(crate::FieldReader<bool, bool>);
impl EHEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EHEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EHEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EHEN` writer - Embedded host enable"]
pub struct EHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EHEN_W<'a> {
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
#[doc = "Field `OTGVER` reader - OTG version"]
pub struct OTGVER_R(crate::FieldReader<bool, bool>);
impl OTGVER_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTGVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTGVER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTGVER` writer - OTG version"]
pub struct OTGVER_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGVER_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Session request success"]
    #[inline(always)]
    pub fn srqscs(&self) -> SRQSCS_R {
        SRQSCS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Session request"]
    #[inline(always)]
    pub fn srq(&self) -> SRQ_R {
        SRQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Host negotiation success"]
    #[inline(always)]
    pub fn hngscs(&self) -> HNGSCS_R {
        HNGSCS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    pub fn hnprq(&self) -> HNPRQ_R {
        HNPRQ_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Host set HNP enable"]
    #[inline(always)]
    pub fn hshnpen(&self) -> HSHNPEN_R {
        HSHNPEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    pub fn dhnpen(&self) -> DHNPEN_R {
        DHNPEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Connector ID status"]
    #[inline(always)]
    pub fn cidsts(&self) -> CIDSTS_R {
        CIDSTS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Long/short debounce time"]
    #[inline(always)]
    pub fn dbct(&self) -> DBCT_R {
        DBCT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - A-session valid"]
    #[inline(always)]
    pub fn asvld(&self) -> ASVLD_R {
        ASVLD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B-session valid"]
    #[inline(always)]
    pub fn bsvld(&self) -> BSVLD_R {
        BSVLD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 2 - VBUS valid override enable"]
    #[inline(always)]
    pub fn vbvaloen(&self) -> VBVALOEN_R {
        VBVALOEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VBUS valid override value"]
    #[inline(always)]
    pub fn vbvaloval(&self) -> VBVALOVAL_R {
        VBVALOVAL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - A-peripheral session valid override enable"]
    #[inline(always)]
    pub fn avaloen(&self) -> AVALOEN_R {
        AVALOEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - A-peripheral session valid override value"]
    #[inline(always)]
    pub fn avaloval(&self) -> AVALOVAL_R {
        AVALOVAL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B-peripheral session valid override enable"]
    #[inline(always)]
    pub fn bvaloen(&self) -> BVALOEN_R {
        BVALOEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B-peripheral session valid override value"]
    #[inline(always)]
    pub fn bvaloval(&self) -> BVALOVAL_R {
        BVALOVAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Embedded host enable"]
    #[inline(always)]
    pub fn ehen(&self) -> EHEN_R {
        EHEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 20 - OTG version"]
    #[inline(always)]
    pub fn otgver(&self) -> OTGVER_R {
        OTGVER_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Session request"]
    #[inline(always)]
    pub fn srq(&mut self) -> SRQ_W {
        SRQ_W { w: self }
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    pub fn hnprq(&mut self) -> HNPRQ_W {
        HNPRQ_W { w: self }
    }
    #[doc = "Bit 10 - Host set HNP enable"]
    #[inline(always)]
    pub fn hshnpen(&mut self) -> HSHNPEN_W {
        HSHNPEN_W { w: self }
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    pub fn dhnpen(&mut self) -> DHNPEN_W {
        DHNPEN_W { w: self }
    }
    #[doc = "Bit 2 - VBUS valid override enable"]
    #[inline(always)]
    pub fn vbvaloen(&mut self) -> VBVALOEN_W {
        VBVALOEN_W { w: self }
    }
    #[doc = "Bit 3 - VBUS valid override value"]
    #[inline(always)]
    pub fn vbvaloval(&mut self) -> VBVALOVAL_W {
        VBVALOVAL_W { w: self }
    }
    #[doc = "Bit 4 - A-peripheral session valid override enable"]
    #[inline(always)]
    pub fn avaloen(&mut self) -> AVALOEN_W {
        AVALOEN_W { w: self }
    }
    #[doc = "Bit 5 - A-peripheral session valid override value"]
    #[inline(always)]
    pub fn avaloval(&mut self) -> AVALOVAL_W {
        AVALOVAL_W { w: self }
    }
    #[doc = "Bit 6 - B-peripheral session valid override enable"]
    #[inline(always)]
    pub fn bvaloen(&mut self) -> BVALOEN_W {
        BVALOEN_W { w: self }
    }
    #[doc = "Bit 7 - B-peripheral session valid override value"]
    #[inline(always)]
    pub fn bvaloval(&mut self) -> BVALOVAL_W {
        BVALOVAL_W { w: self }
    }
    #[doc = "Bit 12 - Embedded host enable"]
    #[inline(always)]
    pub fn ehen(&mut self) -> EHEN_W {
        EHEN_W { w: self }
    }
    #[doc = "Bit 20 - OTG version"]
    #[inline(always)]
    pub fn otgver(&mut self) -> OTGVER_W {
        OTGVER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS control and status register (OTG_FS_GOTGCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_gotgctl](index.html) module"]
pub struct OTG_FS_GOTGCTL_SPEC;
impl crate::RegisterSpec for OTG_FS_GOTGCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_fs_gotgctl::R](R) reader structure"]
impl crate::Readable for OTG_FS_GOTGCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_fs_gotgctl::W](W) writer structure"]
impl crate::Writable for OTG_FS_GOTGCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_FS_GOTGCTL to value 0x0800"]
impl crate::Resettable for OTG_FS_GOTGCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0800
    }
}
