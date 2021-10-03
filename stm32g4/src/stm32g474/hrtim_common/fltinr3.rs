#[doc = "Register `FLTINR3` reader"]
pub struct R(crate::R<FLTINR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTINR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTINR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTINR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLTINR3` writer"]
pub struct W(crate::W<FLTINR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTINR3_SPEC>;
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
impl From<crate::W<FLTINR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTINR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLT4RSTM` reader - FLT4RSTM"]
pub struct FLT4RSTM_R(crate::FieldReader<bool, bool>);
impl FLT4RSTM_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT4RSTM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT4RSTM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT4RSTM` writer - FLT4RSTM"]
pub struct FLT4RSTM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT4RSTM_W<'a> {
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
#[doc = "Field `FLT4CRES` reader - FLT4CRES"]
pub struct FLT4CRES_R(crate::FieldReader<bool, bool>);
impl FLT4CRES_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT4CRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT4CRES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT4CRES` writer - FLT4CRES"]
pub struct FLT4CRES_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT4CRES_W<'a> {
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
#[doc = "Field `FLT4CNT` reader - FLT4CNT"]
pub struct FLT4CNT_R(crate::FieldReader<u8, u8>);
impl FLT4CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLT4CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT4CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT4CNT` writer - FLT4CNT"]
pub struct FLT4CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT4CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 26)) | ((value as u32 & 0x0f) << 26);
        self.w
    }
}
#[doc = "Field `FLT4BLKS` reader - FLT4BLKS"]
pub struct FLT4BLKS_R(crate::FieldReader<bool, bool>);
impl FLT4BLKS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT4BLKS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT4BLKS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT4BLKS` writer - FLT4BLKS"]
pub struct FLT4BLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT4BLKS_W<'a> {
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
#[doc = "Field `FLT4BLKE` reader - FLT4BLKE"]
pub struct FLT4BLKE_R(crate::FieldReader<bool, bool>);
impl FLT4BLKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT4BLKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT4BLKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT4BLKE` writer - FLT4BLKE"]
pub struct FLT4BLKE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT4BLKE_W<'a> {
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
#[doc = "Field `FLT3RSTM` reader - FLT3RSTM"]
pub struct FLT3RSTM_R(crate::FieldReader<bool, bool>);
impl FLT3RSTM_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT3RSTM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT3RSTM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT3RSTM` writer - FLT3RSTM"]
pub struct FLT3RSTM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3RSTM_W<'a> {
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
#[doc = "Field `FLT3CRES` reader - FLT3CRES"]
pub struct FLT3CRES_R(crate::FieldReader<bool, bool>);
impl FLT3CRES_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT3CRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT3CRES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT3CRES` writer - FLT3CRES"]
pub struct FLT3CRES_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3CRES_W<'a> {
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
#[doc = "Field `FLT3CNT` reader - FLT3CNT"]
pub struct FLT3CNT_R(crate::FieldReader<u8, u8>);
impl FLT3CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLT3CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT3CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT3CNT` writer - FLT3CNT"]
pub struct FLT3CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | ((value as u32 & 0x0f) << 18);
        self.w
    }
}
#[doc = "Field `FLT3BLKS` reader - FLT3BLKS"]
pub struct FLT3BLKS_R(crate::FieldReader<bool, bool>);
impl FLT3BLKS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT3BLKS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT3BLKS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT3BLKS` writer - FLT3BLKS"]
pub struct FLT3BLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3BLKS_W<'a> {
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
#[doc = "Field `FLT3BLKE` reader - FLT3BLKE"]
pub struct FLT3BLKE_R(crate::FieldReader<bool, bool>);
impl FLT3BLKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT3BLKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT3BLKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT3BLKE` writer - FLT3BLKE"]
pub struct FLT3BLKE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3BLKE_W<'a> {
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
#[doc = "Field `FLT2RSTM` reader - FLT2RSTM"]
pub struct FLT2RSTM_R(crate::FieldReader<bool, bool>);
impl FLT2RSTM_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT2RSTM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT2RSTM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT2RSTM` writer - FLT2RSTM"]
pub struct FLT2RSTM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2RSTM_W<'a> {
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
#[doc = "Field `FLT2CRES` reader - FLT2CRES"]
pub struct FLT2CRES_R(crate::FieldReader<bool, bool>);
impl FLT2CRES_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT2CRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT2CRES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT2CRES` writer - FLT2CRES"]
pub struct FLT2CRES_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2CRES_W<'a> {
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
#[doc = "Field `FLT2CNT` reader - FLT2CNT"]
pub struct FLT2CNT_R(crate::FieldReader<u8, u8>);
impl FLT2CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLT2CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT2CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT2CNT` writer - FLT2CNT"]
pub struct FLT2CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | ((value as u32 & 0x0f) << 10);
        self.w
    }
}
#[doc = "Field `FLT2BLKS` reader - FLT2BLKS"]
pub struct FLT2BLKS_R(crate::FieldReader<bool, bool>);
impl FLT2BLKS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT2BLKS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT2BLKS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT2BLKS` writer - FLT2BLKS"]
pub struct FLT2BLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2BLKS_W<'a> {
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
#[doc = "Field `FLT2BLKE` reader - FLT2BLKE"]
pub struct FLT2BLKE_R(crate::FieldReader<bool, bool>);
impl FLT2BLKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT2BLKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT2BLKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT2BLKE` writer - FLT2BLKE"]
pub struct FLT2BLKE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2BLKE_W<'a> {
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
#[doc = "Field `FLT1RSTM` reader - FLT1RSTM"]
pub struct FLT1RSTM_R(crate::FieldReader<bool, bool>);
impl FLT1RSTM_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT1RSTM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT1RSTM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT1RSTM` writer - FLT1RSTM"]
pub struct FLT1RSTM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1RSTM_W<'a> {
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
#[doc = "Field `FLT1CRES` reader - FLT1CRES"]
pub struct FLT1CRES_R(crate::FieldReader<bool, bool>);
impl FLT1CRES_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT1CRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT1CRES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT1CRES` writer - FLT1CRES"]
pub struct FLT1CRES_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1CRES_W<'a> {
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
#[doc = "Field `FLT1CNT` reader - FLT1CNT"]
pub struct FLT1CNT_R(crate::FieldReader<u8, u8>);
impl FLT1CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLT1CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT1CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT1CNT` writer - FLT1CNT"]
pub struct FLT1CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | ((value as u32 & 0x0f) << 2);
        self.w
    }
}
#[doc = "Field `FLT1BLKS` reader - FLT1BLKS"]
pub struct FLT1BLKS_R(crate::FieldReader<bool, bool>);
impl FLT1BLKS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT1BLKS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT1BLKS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT1BLKS` writer - FLT1BLKS"]
pub struct FLT1BLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1BLKS_W<'a> {
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
#[doc = "Field `FLT1BLKE` reader - FLT1BLKE"]
pub struct FLT1BLKE_R(crate::FieldReader<bool, bool>);
impl FLT1BLKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT1BLKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT1BLKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT1BLKE` writer - FLT1BLKE"]
pub struct FLT1BLKE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1BLKE_W<'a> {
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
    #[doc = "Bit 31 - FLT4RSTM"]
    #[inline(always)]
    pub fn flt4rstm(&self) -> FLT4RSTM_R {
        FLT4RSTM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - FLT4CRES"]
    #[inline(always)]
    pub fn flt4cres(&self) -> FLT4CRES_R {
        FLT4CRES_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 26:29 - FLT4CNT"]
    #[inline(always)]
    pub fn flt4cnt(&self) -> FLT4CNT_R {
        FLT4CNT_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 25 - FLT4BLKS"]
    #[inline(always)]
    pub fn flt4blks(&self) -> FLT4BLKS_R {
        FLT4BLKS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - FLT4BLKE"]
    #[inline(always)]
    pub fn flt4blke(&self) -> FLT4BLKE_R {
        FLT4BLKE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - FLT3RSTM"]
    #[inline(always)]
    pub fn flt3rstm(&self) -> FLT3RSTM_R {
        FLT3RSTM_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - FLT3CRES"]
    #[inline(always)]
    pub fn flt3cres(&self) -> FLT3CRES_R {
        FLT3CRES_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 18:21 - FLT3CNT"]
    #[inline(always)]
    pub fn flt3cnt(&self) -> FLT3CNT_R {
        FLT3CNT_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - FLT3BLKS"]
    #[inline(always)]
    pub fn flt3blks(&self) -> FLT3BLKS_R {
        FLT3BLKS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - FLT3BLKE"]
    #[inline(always)]
    pub fn flt3blke(&self) -> FLT3BLKE_R {
        FLT3BLKE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - FLT2RSTM"]
    #[inline(always)]
    pub fn flt2rstm(&self) -> FLT2RSTM_R {
        FLT2RSTM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - FLT2CRES"]
    #[inline(always)]
    pub fn flt2cres(&self) -> FLT2CRES_R {
        FLT2CRES_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 10:13 - FLT2CNT"]
    #[inline(always)]
    pub fn flt2cnt(&self) -> FLT2CNT_R {
        FLT2CNT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - FLT2BLKS"]
    #[inline(always)]
    pub fn flt2blks(&self) -> FLT2BLKS_R {
        FLT2BLKS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FLT2BLKE"]
    #[inline(always)]
    pub fn flt2blke(&self) -> FLT2BLKE_R {
        FLT2BLKE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - FLT1RSTM"]
    #[inline(always)]
    pub fn flt1rstm(&self) -> FLT1RSTM_R {
        FLT1RSTM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FLT1CRES"]
    #[inline(always)]
    pub fn flt1cres(&self) -> FLT1CRES_R {
        FLT1CRES_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - FLT1CNT"]
    #[inline(always)]
    pub fn flt1cnt(&self) -> FLT1CNT_R {
        FLT1CNT_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 1 - FLT1BLKS"]
    #[inline(always)]
    pub fn flt1blks(&self) -> FLT1BLKS_R {
        FLT1BLKS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - FLT1BLKE"]
    #[inline(always)]
    pub fn flt1blke(&self) -> FLT1BLKE_R {
        FLT1BLKE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - FLT4RSTM"]
    #[inline(always)]
    pub fn flt4rstm(&mut self) -> FLT4RSTM_W {
        FLT4RSTM_W { w: self }
    }
    #[doc = "Bit 30 - FLT4CRES"]
    #[inline(always)]
    pub fn flt4cres(&mut self) -> FLT4CRES_W {
        FLT4CRES_W { w: self }
    }
    #[doc = "Bits 26:29 - FLT4CNT"]
    #[inline(always)]
    pub fn flt4cnt(&mut self) -> FLT4CNT_W {
        FLT4CNT_W { w: self }
    }
    #[doc = "Bit 25 - FLT4BLKS"]
    #[inline(always)]
    pub fn flt4blks(&mut self) -> FLT4BLKS_W {
        FLT4BLKS_W { w: self }
    }
    #[doc = "Bit 24 - FLT4BLKE"]
    #[inline(always)]
    pub fn flt4blke(&mut self) -> FLT4BLKE_W {
        FLT4BLKE_W { w: self }
    }
    #[doc = "Bit 23 - FLT3RSTM"]
    #[inline(always)]
    pub fn flt3rstm(&mut self) -> FLT3RSTM_W {
        FLT3RSTM_W { w: self }
    }
    #[doc = "Bit 22 - FLT3CRES"]
    #[inline(always)]
    pub fn flt3cres(&mut self) -> FLT3CRES_W {
        FLT3CRES_W { w: self }
    }
    #[doc = "Bits 18:21 - FLT3CNT"]
    #[inline(always)]
    pub fn flt3cnt(&mut self) -> FLT3CNT_W {
        FLT3CNT_W { w: self }
    }
    #[doc = "Bit 17 - FLT3BLKS"]
    #[inline(always)]
    pub fn flt3blks(&mut self) -> FLT3BLKS_W {
        FLT3BLKS_W { w: self }
    }
    #[doc = "Bit 16 - FLT3BLKE"]
    #[inline(always)]
    pub fn flt3blke(&mut self) -> FLT3BLKE_W {
        FLT3BLKE_W { w: self }
    }
    #[doc = "Bit 15 - FLT2RSTM"]
    #[inline(always)]
    pub fn flt2rstm(&mut self) -> FLT2RSTM_W {
        FLT2RSTM_W { w: self }
    }
    #[doc = "Bit 14 - FLT2CRES"]
    #[inline(always)]
    pub fn flt2cres(&mut self) -> FLT2CRES_W {
        FLT2CRES_W { w: self }
    }
    #[doc = "Bits 10:13 - FLT2CNT"]
    #[inline(always)]
    pub fn flt2cnt(&mut self) -> FLT2CNT_W {
        FLT2CNT_W { w: self }
    }
    #[doc = "Bit 9 - FLT2BLKS"]
    #[inline(always)]
    pub fn flt2blks(&mut self) -> FLT2BLKS_W {
        FLT2BLKS_W { w: self }
    }
    #[doc = "Bit 8 - FLT2BLKE"]
    #[inline(always)]
    pub fn flt2blke(&mut self) -> FLT2BLKE_W {
        FLT2BLKE_W { w: self }
    }
    #[doc = "Bit 7 - FLT1RSTM"]
    #[inline(always)]
    pub fn flt1rstm(&mut self) -> FLT1RSTM_W {
        FLT1RSTM_W { w: self }
    }
    #[doc = "Bit 6 - FLT1CRES"]
    #[inline(always)]
    pub fn flt1cres(&mut self) -> FLT1CRES_W {
        FLT1CRES_W { w: self }
    }
    #[doc = "Bits 2:5 - FLT1CNT"]
    #[inline(always)]
    pub fn flt1cnt(&mut self) -> FLT1CNT_W {
        FLT1CNT_W { w: self }
    }
    #[doc = "Bit 1 - FLT1BLKS"]
    #[inline(always)]
    pub fn flt1blks(&mut self) -> FLT1BLKS_W {
        FLT1BLKS_W { w: self }
    }
    #[doc = "Bit 0 - FLT1BLKE"]
    #[inline(always)]
    pub fn flt1blke(&mut self) -> FLT1BLKE_W {
        FLT1BLKE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRTIM Fault Input Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltinr3](index.html) module"]
pub struct FLTINR3_SPEC;
impl crate::RegisterSpec for FLTINR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fltinr3::R](R) reader structure"]
impl crate::Readable for FLTINR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fltinr3::W](W) writer structure"]
impl crate::Writable for FLTINR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLTINR3 to value 0"]
impl crate::Resettable for FLTINR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
