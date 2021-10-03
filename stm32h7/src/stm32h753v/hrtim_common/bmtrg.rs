#[doc = "Register `BMTRG` reader"]
pub struct R(crate::R<BMTRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMTRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMTRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMTRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMTRG` writer"]
pub struct W(crate::W<BMTRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMTRG_SPEC>;
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
impl From<crate::W<BMTRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMTRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCHPEV` reader - OCHPEV"]
pub struct OCHPEV_R(crate::FieldReader<bool, bool>);
impl OCHPEV_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCHPEV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCHPEV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCHPEV` writer - OCHPEV"]
pub struct OCHPEV_W<'a> {
    w: &'a mut W,
}
impl<'a> OCHPEV_W<'a> {
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
#[doc = "Field `TECMP2` reader - TECMP2"]
pub struct TECMP2_R(crate::FieldReader<bool, bool>);
impl TECMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TECMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TECMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TECMP2` writer - TECMP2"]
pub struct TECMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TECMP2_W<'a> {
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
#[doc = "Field `TECMP1` reader - TECMP1"]
pub struct TECMP1_R(crate::FieldReader<bool, bool>);
impl TECMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TECMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TECMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TECMP1` writer - TECMP1"]
pub struct TECMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TECMP1_W<'a> {
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
#[doc = "Field `TEREP` reader - TEREP"]
pub struct TEREP_R(crate::FieldReader<bool, bool>);
impl TEREP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEREP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEREP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEREP` writer - TEREP"]
pub struct TEREP_W<'a> {
    w: &'a mut W,
}
impl<'a> TEREP_W<'a> {
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
#[doc = "Field `TERST` reader - TERST"]
pub struct TERST_R(crate::FieldReader<bool, bool>);
impl TERST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TERST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TERST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TERST` writer - TERST"]
pub struct TERST_W<'a> {
    w: &'a mut W,
}
impl<'a> TERST_W<'a> {
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
#[doc = "Field `TDCMP2` reader - TDCMP2"]
pub struct TDCMP2_R(crate::FieldReader<bool, bool>);
impl TDCMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDCMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDCMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDCMP2` writer - TDCMP2"]
pub struct TDCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCMP2_W<'a> {
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
#[doc = "Field `TDCMP1` reader - TDCMP1"]
pub struct TDCMP1_R(crate::FieldReader<bool, bool>);
impl TDCMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDCMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDCMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDCMP1` writer - TDCMP1"]
pub struct TDCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCMP1_W<'a> {
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
#[doc = "Field `TDREP` reader - TDREP"]
pub struct TDREP_R(crate::FieldReader<bool, bool>);
impl TDREP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDREP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDREP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDREP` writer - TDREP"]
pub struct TDREP_W<'a> {
    w: &'a mut W,
}
impl<'a> TDREP_W<'a> {
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
#[doc = "Field `TDRST` reader - TDRST"]
pub struct TDRST_R(crate::FieldReader<bool, bool>);
impl TDRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDRST` writer - TDRST"]
pub struct TDRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TDRST_W<'a> {
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
#[doc = "Field `TCCMP2` reader - TCCMP2"]
pub struct TCCMP2_R(crate::FieldReader<bool, bool>);
impl TCCMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCCMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCCMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCCMP2` writer - TCCMP2"]
pub struct TCCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCMP2_W<'a> {
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
#[doc = "Field `TCCMP1` reader - TCCMP1"]
pub struct TCCMP1_R(crate::FieldReader<bool, bool>);
impl TCCMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCCMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCCMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCCMP1` writer - TCCMP1"]
pub struct TCCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCMP1_W<'a> {
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
#[doc = "Field `TCREP` reader - TCREP"]
pub struct TCREP_R(crate::FieldReader<bool, bool>);
impl TCREP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCREP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCREP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCREP` writer - TCREP"]
pub struct TCREP_W<'a> {
    w: &'a mut W,
}
impl<'a> TCREP_W<'a> {
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
#[doc = "Field `TCRST` reader - TCRST"]
pub struct TCRST_R(crate::FieldReader<bool, bool>);
impl TCRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCRST` writer - TCRST"]
pub struct TCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TCRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `TBCMP2` reader - TBCMP2"]
pub struct TBCMP2_R(crate::FieldReader<bool, bool>);
impl TBCMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBCMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBCMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBCMP2` writer - TBCMP2"]
pub struct TBCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCMP2_W<'a> {
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
#[doc = "Field `TBCMP1` reader - TBCMP1"]
pub struct TBCMP1_R(crate::FieldReader<bool, bool>);
impl TBCMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBCMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBCMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBCMP1` writer - TBCMP1"]
pub struct TBCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCMP1_W<'a> {
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
#[doc = "Field `TBREP` reader - TBREP"]
pub struct TBREP_R(crate::FieldReader<bool, bool>);
impl TBREP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBREP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBREP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBREP` writer - TBREP"]
pub struct TBREP_W<'a> {
    w: &'a mut W,
}
impl<'a> TBREP_W<'a> {
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
#[doc = "Field `TBRST` reader - TBRST"]
pub struct TBRST_R(crate::FieldReader<bool, bool>);
impl TBRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBRST` writer - TBRST"]
pub struct TBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TBRST_W<'a> {
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
#[doc = "Field `TACMP2` reader - TACMP2"]
pub struct TACMP2_R(crate::FieldReader<bool, bool>);
impl TACMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TACMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TACMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TACMP2` writer - TACMP2"]
pub struct TACMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TACMP2_W<'a> {
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
#[doc = "Field `TACMP1` reader - TACMP1"]
pub struct TACMP1_R(crate::FieldReader<bool, bool>);
impl TACMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TACMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TACMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TACMP1` writer - TACMP1"]
pub struct TACMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TACMP1_W<'a> {
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
#[doc = "Field `TAREP` reader - TAREP"]
pub struct TAREP_R(crate::FieldReader<bool, bool>);
impl TAREP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAREP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAREP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAREP` writer - TAREP"]
pub struct TAREP_W<'a> {
    w: &'a mut W,
}
impl<'a> TAREP_W<'a> {
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
#[doc = "Field `TARST` reader - TARST"]
pub struct TARST_R(crate::FieldReader<bool, bool>);
impl TARST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TARST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TARST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TARST` writer - TARST"]
pub struct TARST_W<'a> {
    w: &'a mut W,
}
impl<'a> TARST_W<'a> {
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
#[doc = "Field `MSTCMP4` reader - MSTCMP4"]
pub struct MSTCMP4_R(crate::FieldReader<bool, bool>);
impl MSTCMP4_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTCMP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSTCMP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTCMP4` writer - MSTCMP4"]
pub struct MSTCMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTCMP4_W<'a> {
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
#[doc = "Field `MSTCMP3` reader - MSTCMP3"]
pub struct MSTCMP3_R(crate::FieldReader<bool, bool>);
impl MSTCMP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTCMP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSTCMP3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTCMP3` writer - MSTCMP3"]
pub struct MSTCMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTCMP3_W<'a> {
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
#[doc = "Field `MSTCMP2` reader - MSTCMP2"]
pub struct MSTCMP2_R(crate::FieldReader<bool, bool>);
impl MSTCMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTCMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSTCMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTCMP2` writer - MSTCMP2"]
pub struct MSTCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTCMP2_W<'a> {
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
#[doc = "Field `MSTCMP1` reader - MSTCMP1"]
pub struct MSTCMP1_R(crate::FieldReader<bool, bool>);
impl MSTCMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTCMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSTCMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTCMP1` writer - MSTCMP1"]
pub struct MSTCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTCMP1_W<'a> {
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
#[doc = "Field `MSTREP` reader - MSTREP"]
pub struct MSTREP_R(crate::FieldReader<bool, bool>);
impl MSTREP_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTREP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSTREP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTREP` writer - MSTREP"]
pub struct MSTREP_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTREP_W<'a> {
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
#[doc = "Field `MSTRST` reader - MSTRST"]
pub struct MSTRST_R(crate::FieldReader<bool, bool>);
impl MSTRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSTRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTRST` writer - MSTRST"]
pub struct MSTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTRST_W<'a> {
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
#[doc = "Field `SW` reader - SW"]
pub struct SW_R(crate::FieldReader<bool, bool>);
impl SW_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW` writer - SW"]
pub struct SW_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_W<'a> {
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
    #[doc = "Bit 31 - OCHPEV"]
    #[inline(always)]
    pub fn ochpev(&self) -> OCHPEV_R {
        OCHPEV_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TECMP2"]
    #[inline(always)]
    pub fn tecmp2(&self) -> TECMP2_R {
        TECMP2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TECMP1"]
    #[inline(always)]
    pub fn tecmp1(&self) -> TECMP1_R {
        TECMP1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TEREP"]
    #[inline(always)]
    pub fn terep(&self) -> TEREP_R {
        TEREP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TERST"]
    #[inline(always)]
    pub fn terst(&self) -> TERST_R {
        TERST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - TDCMP2"]
    #[inline(always)]
    pub fn tdcmp2(&self) -> TDCMP2_R {
        TDCMP2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - TDCMP1"]
    #[inline(always)]
    pub fn tdcmp1(&self) -> TDCMP1_R {
        TDCMP1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - TDREP"]
    #[inline(always)]
    pub fn tdrep(&self) -> TDREP_R {
        TDREP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - TDRST"]
    #[inline(always)]
    pub fn tdrst(&self) -> TDRST_R {
        TDRST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TCCMP2"]
    #[inline(always)]
    pub fn tccmp2(&self) -> TCCMP2_R {
        TCCMP2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TCCMP1"]
    #[inline(always)]
    pub fn tccmp1(&self) -> TCCMP1_R {
        TCCMP1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TCREP"]
    #[inline(always)]
    pub fn tcrep(&self) -> TCREP_R {
        TCREP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TCRST"]
    #[inline(always)]
    pub fn tcrst(&self) -> TCRST_R {
        TCRST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TBCMP2"]
    #[inline(always)]
    pub fn tbcmp2(&self) -> TBCMP2_R {
        TBCMP2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TBCMP1"]
    #[inline(always)]
    pub fn tbcmp1(&self) -> TBCMP1_R {
        TBCMP1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TBREP"]
    #[inline(always)]
    pub fn tbrep(&self) -> TBREP_R {
        TBREP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TBRST"]
    #[inline(always)]
    pub fn tbrst(&self) -> TBRST_R {
        TBRST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TACMP2"]
    #[inline(always)]
    pub fn tacmp2(&self) -> TACMP2_R {
        TACMP2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TACMP1"]
    #[inline(always)]
    pub fn tacmp1(&self) -> TACMP1_R {
        TACMP1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TAREP"]
    #[inline(always)]
    pub fn tarep(&self) -> TAREP_R {
        TAREP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TARST"]
    #[inline(always)]
    pub fn tarst(&self) -> TARST_R {
        TARST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MSTCMP4"]
    #[inline(always)]
    pub fn mstcmp4(&self) -> MSTCMP4_R {
        MSTCMP4_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MSTCMP3"]
    #[inline(always)]
    pub fn mstcmp3(&self) -> MSTCMP3_R {
        MSTCMP3_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MSTCMP2"]
    #[inline(always)]
    pub fn mstcmp2(&self) -> MSTCMP2_R {
        MSTCMP2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MSTCMP1"]
    #[inline(always)]
    pub fn mstcmp1(&self) -> MSTCMP1_R {
        MSTCMP1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MSTREP"]
    #[inline(always)]
    pub fn mstrep(&self) -> MSTREP_R {
        MSTREP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - MSTRST"]
    #[inline(always)]
    pub fn mstrst(&self) -> MSTRST_R {
        MSTRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SW"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - OCHPEV"]
    #[inline(always)]
    pub fn ochpev(&mut self) -> OCHPEV_W {
        OCHPEV_W { w: self }
    }
    #[doc = "Bit 26 - TECMP2"]
    #[inline(always)]
    pub fn tecmp2(&mut self) -> TECMP2_W {
        TECMP2_W { w: self }
    }
    #[doc = "Bit 25 - TECMP1"]
    #[inline(always)]
    pub fn tecmp1(&mut self) -> TECMP1_W {
        TECMP1_W { w: self }
    }
    #[doc = "Bit 24 - TEREP"]
    #[inline(always)]
    pub fn terep(&mut self) -> TEREP_W {
        TEREP_W { w: self }
    }
    #[doc = "Bit 23 - TERST"]
    #[inline(always)]
    pub fn terst(&mut self) -> TERST_W {
        TERST_W { w: self }
    }
    #[doc = "Bit 22 - TDCMP2"]
    #[inline(always)]
    pub fn tdcmp2(&mut self) -> TDCMP2_W {
        TDCMP2_W { w: self }
    }
    #[doc = "Bit 21 - TDCMP1"]
    #[inline(always)]
    pub fn tdcmp1(&mut self) -> TDCMP1_W {
        TDCMP1_W { w: self }
    }
    #[doc = "Bit 20 - TDREP"]
    #[inline(always)]
    pub fn tdrep(&mut self) -> TDREP_W {
        TDREP_W { w: self }
    }
    #[doc = "Bit 19 - TDRST"]
    #[inline(always)]
    pub fn tdrst(&mut self) -> TDRST_W {
        TDRST_W { w: self }
    }
    #[doc = "Bit 18 - TCCMP2"]
    #[inline(always)]
    pub fn tccmp2(&mut self) -> TCCMP2_W {
        TCCMP2_W { w: self }
    }
    #[doc = "Bit 17 - TCCMP1"]
    #[inline(always)]
    pub fn tccmp1(&mut self) -> TCCMP1_W {
        TCCMP1_W { w: self }
    }
    #[doc = "Bit 16 - TCREP"]
    #[inline(always)]
    pub fn tcrep(&mut self) -> TCREP_W {
        TCREP_W { w: self }
    }
    #[doc = "Bit 15 - TCRST"]
    #[inline(always)]
    pub fn tcrst(&mut self) -> TCRST_W {
        TCRST_W { w: self }
    }
    #[doc = "Bit 14 - TBCMP2"]
    #[inline(always)]
    pub fn tbcmp2(&mut self) -> TBCMP2_W {
        TBCMP2_W { w: self }
    }
    #[doc = "Bit 13 - TBCMP1"]
    #[inline(always)]
    pub fn tbcmp1(&mut self) -> TBCMP1_W {
        TBCMP1_W { w: self }
    }
    #[doc = "Bit 12 - TBREP"]
    #[inline(always)]
    pub fn tbrep(&mut self) -> TBREP_W {
        TBREP_W { w: self }
    }
    #[doc = "Bit 11 - TBRST"]
    #[inline(always)]
    pub fn tbrst(&mut self) -> TBRST_W {
        TBRST_W { w: self }
    }
    #[doc = "Bit 10 - TACMP2"]
    #[inline(always)]
    pub fn tacmp2(&mut self) -> TACMP2_W {
        TACMP2_W { w: self }
    }
    #[doc = "Bit 9 - TACMP1"]
    #[inline(always)]
    pub fn tacmp1(&mut self) -> TACMP1_W {
        TACMP1_W { w: self }
    }
    #[doc = "Bit 8 - TAREP"]
    #[inline(always)]
    pub fn tarep(&mut self) -> TAREP_W {
        TAREP_W { w: self }
    }
    #[doc = "Bit 7 - TARST"]
    #[inline(always)]
    pub fn tarst(&mut self) -> TARST_W {
        TARST_W { w: self }
    }
    #[doc = "Bit 6 - MSTCMP4"]
    #[inline(always)]
    pub fn mstcmp4(&mut self) -> MSTCMP4_W {
        MSTCMP4_W { w: self }
    }
    #[doc = "Bit 5 - MSTCMP3"]
    #[inline(always)]
    pub fn mstcmp3(&mut self) -> MSTCMP3_W {
        MSTCMP3_W { w: self }
    }
    #[doc = "Bit 4 - MSTCMP2"]
    #[inline(always)]
    pub fn mstcmp2(&mut self) -> MSTCMP2_W {
        MSTCMP2_W { w: self }
    }
    #[doc = "Bit 3 - MSTCMP1"]
    #[inline(always)]
    pub fn mstcmp1(&mut self) -> MSTCMP1_W {
        MSTCMP1_W { w: self }
    }
    #[doc = "Bit 2 - MSTREP"]
    #[inline(always)]
    pub fn mstrep(&mut self) -> MSTREP_W {
        MSTREP_W { w: self }
    }
    #[doc = "Bit 1 - MSTRST"]
    #[inline(always)]
    pub fn mstrst(&mut self) -> MSTRST_W {
        MSTRST_W { w: self }
    }
    #[doc = "Bit 0 - SW"]
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W {
        SW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BMTRG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmtrg](index.html) module"]
pub struct BMTRG_SPEC;
impl crate::RegisterSpec for BMTRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmtrg::R](R) reader structure"]
impl crate::Readable for BMTRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmtrg::W](W) writer structure"]
impl crate::Writable for BMTRG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BMTRG to value 0"]
impl crate::Resettable for BMTRG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
