#[doc = "Register `TIMBDIER` reader"]
pub struct R(crate::R<TIMBDIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMBDIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMBDIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMBDIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMBDIER` writer"]
pub struct W(crate::W<TIMBDIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMBDIER_SPEC>;
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
impl From<crate::W<TIMBDIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMBDIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLYPRTDE` reader - DLYPRTDE"]
pub struct DLYPRTDE_R(crate::FieldReader<bool, bool>);
impl DLYPRTDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DLYPRTDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYPRTDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYPRTDE` writer - DLYPRTDE"]
pub struct DLYPRTDE_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYPRTDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `RSTDE` reader - RSTDE"]
pub struct RSTDE_R(crate::FieldReader<bool, bool>);
impl RSTDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTDE` writer - RSTDE"]
pub struct RSTDE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `RSTx2DE` reader - RSTx2DE"]
pub struct RSTX2DE_R(crate::FieldReader<bool, bool>);
impl RSTX2DE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTX2DE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTX2DE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTx2DE` writer - RSTx2DE"]
pub struct RSTX2DE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTX2DE_W<'a> {
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
#[doc = "Field `SETx2DE` reader - SETx2DE"]
pub struct SETX2DE_R(crate::FieldReader<bool, bool>);
impl SETX2DE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETX2DE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETX2DE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETx2DE` writer - SETx2DE"]
pub struct SETX2DE_W<'a> {
    w: &'a mut W,
}
impl<'a> SETX2DE_W<'a> {
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
#[doc = "Field `RSTx1DE` reader - RSTx1DE"]
pub struct RSTX1DE_R(crate::FieldReader<bool, bool>);
impl RSTX1DE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTX1DE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTX1DE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTx1DE` writer - RSTx1DE"]
pub struct RSTX1DE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTX1DE_W<'a> {
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
#[doc = "Field `SET1xDE` reader - SET1xDE"]
pub struct SET1XDE_R(crate::FieldReader<bool, bool>);
impl SET1XDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SET1XDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SET1XDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SET1xDE` writer - SET1xDE"]
pub struct SET1XDE_W<'a> {
    w: &'a mut W,
}
impl<'a> SET1XDE_W<'a> {
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
#[doc = "Field `CPT2DE` reader - CPT2DE"]
pub struct CPT2DE_R(crate::FieldReader<bool, bool>);
impl CPT2DE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPT2DE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPT2DE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPT2DE` writer - CPT2DE"]
pub struct CPT2DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPT2DE_W<'a> {
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
#[doc = "Field `CPT1DE` reader - CPT1DE"]
pub struct CPT1DE_R(crate::FieldReader<bool, bool>);
impl CPT1DE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPT1DE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPT1DE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPT1DE` writer - CPT1DE"]
pub struct CPT1DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPT1DE_W<'a> {
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
#[doc = "Field `UPDDE` reader - UPDDE"]
pub struct UPDDE_R(crate::FieldReader<bool, bool>);
impl UPDDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UPDDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPDDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPDDE` writer - UPDDE"]
pub struct UPDDE_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDDE_W<'a> {
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
#[doc = "Field `REPDE` reader - REPDE"]
pub struct REPDE_R(crate::FieldReader<bool, bool>);
impl REPDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REPDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REPDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REPDE` writer - REPDE"]
pub struct REPDE_W<'a> {
    w: &'a mut W,
}
impl<'a> REPDE_W<'a> {
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
#[doc = "Field `CMP4DE` reader - CMP4DE"]
pub struct CMP4DE_R(crate::FieldReader<bool, bool>);
impl CMP4DE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP4DE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP4DE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP4DE` writer - CMP4DE"]
pub struct CMP4DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP4DE_W<'a> {
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
#[doc = "Field `CMP3DE` reader - CMP3DE"]
pub struct CMP3DE_R(crate::FieldReader<bool, bool>);
impl CMP3DE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP3DE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP3DE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP3DE` writer - CMP3DE"]
pub struct CMP3DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3DE_W<'a> {
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
#[doc = "Field `CMP2DE` reader - CMP2DE"]
pub struct CMP2DE_R(crate::FieldReader<bool, bool>);
impl CMP2DE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP2DE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP2DE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP2DE` writer - CMP2DE"]
pub struct CMP2DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2DE_W<'a> {
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
#[doc = "Field `CMP1DE` reader - CMP1DE"]
pub struct CMP1DE_R(crate::FieldReader<bool, bool>);
impl CMP1DE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP1DE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP1DE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP1DE` writer - CMP1DE"]
pub struct CMP1DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1DE_W<'a> {
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
#[doc = "Field `DLYPRTIE` reader - DLYPRTIE"]
pub struct DLYPRTIE_R(crate::FieldReader<bool, bool>);
impl DLYPRTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DLYPRTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYPRTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYPRTIE` writer - DLYPRTIE"]
pub struct DLYPRTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYPRTIE_W<'a> {
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
#[doc = "Field `RSTIE` reader - RSTIE"]
pub struct RSTIE_R(crate::FieldReader<bool, bool>);
impl RSTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTIE` writer - RSTIE"]
pub struct RSTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIE_W<'a> {
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
#[doc = "Field `RSTx2IE` reader - RSTx2IE"]
pub struct RSTX2IE_R(crate::FieldReader<bool, bool>);
impl RSTX2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTX2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTX2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTx2IE` writer - RSTx2IE"]
pub struct RSTX2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTX2IE_W<'a> {
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
#[doc = "Field `SETx2IE` reader - SETx2IE"]
pub struct SETX2IE_R(crate::FieldReader<bool, bool>);
impl SETX2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETX2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETX2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETx2IE` writer - SETx2IE"]
pub struct SETX2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SETX2IE_W<'a> {
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
#[doc = "Field `RSTx1IE` reader - RSTx1IE"]
pub struct RSTX1IE_R(crate::FieldReader<bool, bool>);
impl RSTX1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTX1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTX1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTx1IE` writer - RSTx1IE"]
pub struct RSTX1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTX1IE_W<'a> {
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
#[doc = "Field `SET1xIE` reader - SET1xIE"]
pub struct SET1XIE_R(crate::FieldReader<bool, bool>);
impl SET1XIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SET1XIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SET1XIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SET1xIE` writer - SET1xIE"]
pub struct SET1XIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SET1XIE_W<'a> {
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
#[doc = "Field `CPT2IE` reader - CPT2IE"]
pub struct CPT2IE_R(crate::FieldReader<bool, bool>);
impl CPT2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPT2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPT2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPT2IE` writer - CPT2IE"]
pub struct CPT2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPT2IE_W<'a> {
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
#[doc = "Field `CPT1IE` reader - CPT1IE"]
pub struct CPT1IE_R(crate::FieldReader<bool, bool>);
impl CPT1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPT1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPT1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPT1IE` writer - CPT1IE"]
pub struct CPT1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPT1IE_W<'a> {
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
#[doc = "Field `UPDIE` reader - UPDIE"]
pub struct UPDIE_R(crate::FieldReader<bool, bool>);
impl UPDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UPDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPDIE` writer - UPDIE"]
pub struct UPDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDIE_W<'a> {
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
#[doc = "Field `REPIE` reader - REPIE"]
pub struct REPIE_R(crate::FieldReader<bool, bool>);
impl REPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REPIE` writer - REPIE"]
pub struct REPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> REPIE_W<'a> {
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
#[doc = "Field `CMP4IE` reader - CMP4IE"]
pub struct CMP4IE_R(crate::FieldReader<bool, bool>);
impl CMP4IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP4IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP4IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP4IE` writer - CMP4IE"]
pub struct CMP4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP4IE_W<'a> {
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
#[doc = "Field `CMP3IE` reader - CMP3IE"]
pub struct CMP3IE_R(crate::FieldReader<bool, bool>);
impl CMP3IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP3IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP3IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP3IE` writer - CMP3IE"]
pub struct CMP3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3IE_W<'a> {
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
#[doc = "Field `CMP2IE` reader - CMP2IE"]
pub struct CMP2IE_R(crate::FieldReader<bool, bool>);
impl CMP2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP2IE` writer - CMP2IE"]
pub struct CMP2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2IE_W<'a> {
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
#[doc = "Field `CMP1IE` reader - CMP1IE"]
pub struct CMP1IE_R(crate::FieldReader<bool, bool>);
impl CMP1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP1IE` writer - CMP1IE"]
pub struct CMP1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1IE_W<'a> {
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
impl R {
    #[doc = "Bit 30 - DLYPRTDE"]
    #[inline(always)]
    pub fn dlyprtde(&self) -> DLYPRTDE_R {
        DLYPRTDE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - RSTDE"]
    #[inline(always)]
    pub fn rstde(&self) -> RSTDE_R {
        RSTDE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - RSTx2DE"]
    #[inline(always)]
    pub fn rstx2de(&self) -> RSTX2DE_R {
        RSTX2DE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - SETx2DE"]
    #[inline(always)]
    pub fn setx2de(&self) -> SETX2DE_R {
        SETX2DE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - RSTx1DE"]
    #[inline(always)]
    pub fn rstx1de(&self) -> RSTX1DE_R {
        RSTX1DE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - SET1xDE"]
    #[inline(always)]
    pub fn set1x_de(&self) -> SET1XDE_R {
        SET1XDE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - CPT2DE"]
    #[inline(always)]
    pub fn cpt2de(&self) -> CPT2DE_R {
        CPT2DE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CPT1DE"]
    #[inline(always)]
    pub fn cpt1de(&self) -> CPT1DE_R {
        CPT1DE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - UPDDE"]
    #[inline(always)]
    pub fn updde(&self) -> UPDDE_R {
        UPDDE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 20 - REPDE"]
    #[inline(always)]
    pub fn repde(&self) -> REPDE_R {
        REPDE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CMP4DE"]
    #[inline(always)]
    pub fn cmp4de(&self) -> CMP4DE_R {
        CMP4DE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CMP3DE"]
    #[inline(always)]
    pub fn cmp3de(&self) -> CMP3DE_R {
        CMP3DE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CMP2DE"]
    #[inline(always)]
    pub fn cmp2de(&self) -> CMP2DE_R {
        CMP2DE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CMP1DE"]
    #[inline(always)]
    pub fn cmp1de(&self) -> CMP1DE_R {
        CMP1DE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DLYPRTIE"]
    #[inline(always)]
    pub fn dlyprtie(&self) -> DLYPRTIE_R {
        DLYPRTIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RSTIE"]
    #[inline(always)]
    pub fn rstie(&self) -> RSTIE_R {
        RSTIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RSTx2IE"]
    #[inline(always)]
    pub fn rstx2ie(&self) -> RSTX2IE_R {
        RSTX2IE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SETx2IE"]
    #[inline(always)]
    pub fn setx2ie(&self) -> SETX2IE_R {
        SETX2IE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RSTx1IE"]
    #[inline(always)]
    pub fn rstx1ie(&self) -> RSTX1IE_R {
        RSTX1IE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SET1xIE"]
    #[inline(always)]
    pub fn set1x_ie(&self) -> SET1XIE_R {
        SET1XIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CPT2IE"]
    #[inline(always)]
    pub fn cpt2ie(&self) -> CPT2IE_R {
        CPT2IE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CPT1IE"]
    #[inline(always)]
    pub fn cpt1ie(&self) -> CPT1IE_R {
        CPT1IE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UPDIE"]
    #[inline(always)]
    pub fn updie(&self) -> UPDIE_R {
        UPDIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 4 - REPIE"]
    #[inline(always)]
    pub fn repie(&self) -> REPIE_R {
        REPIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CMP4IE"]
    #[inline(always)]
    pub fn cmp4ie(&self) -> CMP4IE_R {
        CMP4IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CMP3IE"]
    #[inline(always)]
    pub fn cmp3ie(&self) -> CMP3IE_R {
        CMP3IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - CMP2IE"]
    #[inline(always)]
    pub fn cmp2ie(&self) -> CMP2IE_R {
        CMP2IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CMP1IE"]
    #[inline(always)]
    pub fn cmp1ie(&self) -> CMP1IE_R {
        CMP1IE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - DLYPRTDE"]
    #[inline(always)]
    pub fn dlyprtde(&mut self) -> DLYPRTDE_W {
        DLYPRTDE_W { w: self }
    }
    #[doc = "Bit 29 - RSTDE"]
    #[inline(always)]
    pub fn rstde(&mut self) -> RSTDE_W {
        RSTDE_W { w: self }
    }
    #[doc = "Bit 28 - RSTx2DE"]
    #[inline(always)]
    pub fn rstx2de(&mut self) -> RSTX2DE_W {
        RSTX2DE_W { w: self }
    }
    #[doc = "Bit 27 - SETx2DE"]
    #[inline(always)]
    pub fn setx2de(&mut self) -> SETX2DE_W {
        SETX2DE_W { w: self }
    }
    #[doc = "Bit 26 - RSTx1DE"]
    #[inline(always)]
    pub fn rstx1de(&mut self) -> RSTX1DE_W {
        RSTX1DE_W { w: self }
    }
    #[doc = "Bit 25 - SET1xDE"]
    #[inline(always)]
    pub fn set1x_de(&mut self) -> SET1XDE_W {
        SET1XDE_W { w: self }
    }
    #[doc = "Bit 24 - CPT2DE"]
    #[inline(always)]
    pub fn cpt2de(&mut self) -> CPT2DE_W {
        CPT2DE_W { w: self }
    }
    #[doc = "Bit 23 - CPT1DE"]
    #[inline(always)]
    pub fn cpt1de(&mut self) -> CPT1DE_W {
        CPT1DE_W { w: self }
    }
    #[doc = "Bit 22 - UPDDE"]
    #[inline(always)]
    pub fn updde(&mut self) -> UPDDE_W {
        UPDDE_W { w: self }
    }
    #[doc = "Bit 20 - REPDE"]
    #[inline(always)]
    pub fn repde(&mut self) -> REPDE_W {
        REPDE_W { w: self }
    }
    #[doc = "Bit 19 - CMP4DE"]
    #[inline(always)]
    pub fn cmp4de(&mut self) -> CMP4DE_W {
        CMP4DE_W { w: self }
    }
    #[doc = "Bit 18 - CMP3DE"]
    #[inline(always)]
    pub fn cmp3de(&mut self) -> CMP3DE_W {
        CMP3DE_W { w: self }
    }
    #[doc = "Bit 17 - CMP2DE"]
    #[inline(always)]
    pub fn cmp2de(&mut self) -> CMP2DE_W {
        CMP2DE_W { w: self }
    }
    #[doc = "Bit 16 - CMP1DE"]
    #[inline(always)]
    pub fn cmp1de(&mut self) -> CMP1DE_W {
        CMP1DE_W { w: self }
    }
    #[doc = "Bit 14 - DLYPRTIE"]
    #[inline(always)]
    pub fn dlyprtie(&mut self) -> DLYPRTIE_W {
        DLYPRTIE_W { w: self }
    }
    #[doc = "Bit 13 - RSTIE"]
    #[inline(always)]
    pub fn rstie(&mut self) -> RSTIE_W {
        RSTIE_W { w: self }
    }
    #[doc = "Bit 12 - RSTx2IE"]
    #[inline(always)]
    pub fn rstx2ie(&mut self) -> RSTX2IE_W {
        RSTX2IE_W { w: self }
    }
    #[doc = "Bit 11 - SETx2IE"]
    #[inline(always)]
    pub fn setx2ie(&mut self) -> SETX2IE_W {
        SETX2IE_W { w: self }
    }
    #[doc = "Bit 10 - RSTx1IE"]
    #[inline(always)]
    pub fn rstx1ie(&mut self) -> RSTX1IE_W {
        RSTX1IE_W { w: self }
    }
    #[doc = "Bit 9 - SET1xIE"]
    #[inline(always)]
    pub fn set1x_ie(&mut self) -> SET1XIE_W {
        SET1XIE_W { w: self }
    }
    #[doc = "Bit 8 - CPT2IE"]
    #[inline(always)]
    pub fn cpt2ie(&mut self) -> CPT2IE_W {
        CPT2IE_W { w: self }
    }
    #[doc = "Bit 7 - CPT1IE"]
    #[inline(always)]
    pub fn cpt1ie(&mut self) -> CPT1IE_W {
        CPT1IE_W { w: self }
    }
    #[doc = "Bit 6 - UPDIE"]
    #[inline(always)]
    pub fn updie(&mut self) -> UPDIE_W {
        UPDIE_W { w: self }
    }
    #[doc = "Bit 4 - REPIE"]
    #[inline(always)]
    pub fn repie(&mut self) -> REPIE_W {
        REPIE_W { w: self }
    }
    #[doc = "Bit 3 - CMP4IE"]
    #[inline(always)]
    pub fn cmp4ie(&mut self) -> CMP4IE_W {
        CMP4IE_W { w: self }
    }
    #[doc = "Bit 2 - CMP3IE"]
    #[inline(always)]
    pub fn cmp3ie(&mut self) -> CMP3IE_W {
        CMP3IE_W { w: self }
    }
    #[doc = "Bit 1 - CMP2IE"]
    #[inline(always)]
    pub fn cmp2ie(&mut self) -> CMP2IE_W {
        CMP2IE_W { w: self }
    }
    #[doc = "Bit 0 - CMP1IE"]
    #[inline(always)]
    pub fn cmp1ie(&mut self) -> CMP1IE_W {
        CMP1IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMxDIER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timbdier](index.html) module"]
pub struct TIMBDIER_SPEC;
impl crate::RegisterSpec for TIMBDIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timbdier::R](R) reader structure"]
impl crate::Readable for TIMBDIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timbdier::W](W) writer structure"]
impl crate::Writable for TIMBDIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMBDIER to value 0"]
impl crate::Resettable for TIMBDIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
