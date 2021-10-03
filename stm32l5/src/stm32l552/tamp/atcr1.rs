#[doc = "Register `ATCR1` reader"]
pub struct R(crate::R<ATCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATCR1` writer"]
pub struct W(crate::W<ATCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATCR1_SPEC>;
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
impl From<crate::W<ATCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAMP1AM` reader - TAMP1AM"]
pub struct TAMP1AM_R(crate::FieldReader<bool, bool>);
impl TAMP1AM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1AM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP1AM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP1AM` writer - TAMP1AM"]
pub struct TAMP1AM_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1AM_W<'a> {
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
#[doc = "Field `TAMP2AM` reader - TAMP2AM"]
pub struct TAMP2AM_R(crate::FieldReader<bool, bool>);
impl TAMP2AM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP2AM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP2AM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP2AM` writer - TAMP2AM"]
pub struct TAMP2AM_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2AM_W<'a> {
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
#[doc = "Field `TAMP3AM` reader - TAMP3AM"]
pub struct TAMP3AM_R(crate::FieldReader<bool, bool>);
impl TAMP3AM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP3AM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP3AM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP3AM` writer - TAMP3AM"]
pub struct TAMP3AM_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP3AM_W<'a> {
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
#[doc = "Field `TAMP4AM` reader - TAMP4AM"]
pub struct TAMP4AM_R(crate::FieldReader<bool, bool>);
impl TAMP4AM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP4AM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP4AM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP4AM` writer - TAMP4AM"]
pub struct TAMP4AM_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP4AM_W<'a> {
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
#[doc = "Field `TAMP5AM` reader - TAMP5AM"]
pub struct TAMP5AM_R(crate::FieldReader<bool, bool>);
impl TAMP5AM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP5AM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP5AM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP5AM` writer - TAMP5AM"]
pub struct TAMP5AM_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP5AM_W<'a> {
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
#[doc = "Field `TAMP6AM` reader - TAMP6AM"]
pub struct TAMP6AM_R(crate::FieldReader<bool, bool>);
impl TAMP6AM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP6AM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP6AM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP6AM` writer - TAMP6AM"]
pub struct TAMP6AM_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP6AM_W<'a> {
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
#[doc = "Field `TAMP7AM` reader - TAMP7AM"]
pub struct TAMP7AM_R(crate::FieldReader<bool, bool>);
impl TAMP7AM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP7AM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP7AM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP7AM` writer - TAMP7AM"]
pub struct TAMP7AM_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP7AM_W<'a> {
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
#[doc = "Field `TAMP8AM` reader - TAMP8AM"]
pub struct TAMP8AM_R(crate::FieldReader<bool, bool>);
impl TAMP8AM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP8AM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP8AM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP8AM` writer - TAMP8AM"]
pub struct TAMP8AM_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP8AM_W<'a> {
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
#[doc = "Field `ATOSEL1` reader - ATOSEL1"]
pub struct ATOSEL1_R(crate::FieldReader<u8, u8>);
impl ATOSEL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATOSEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATOSEL1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATOSEL1` writer - ATOSEL1"]
pub struct ATOSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `ATOSEL2` reader - ATOSEL2"]
pub struct ATOSEL2_R(crate::FieldReader<u8, u8>);
impl ATOSEL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATOSEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATOSEL2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATOSEL2` writer - ATOSEL2"]
pub struct ATOSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `ATOSEL3` reader - ATOSEL3"]
pub struct ATOSEL3_R(crate::FieldReader<u8, u8>);
impl ATOSEL3_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATOSEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATOSEL3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATOSEL3` writer - ATOSEL3"]
pub struct ATOSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSEL3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `ATOSEL4` reader - ATOSEL4"]
pub struct ATOSEL4_R(crate::FieldReader<u8, u8>);
impl ATOSEL4_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATOSEL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATOSEL4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATOSEL4` writer - ATOSEL4"]
pub struct ATOSEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSEL4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `ATCKSEL` reader - ATCKSEL"]
pub struct ATCKSEL_R(crate::FieldReader<u8, u8>);
impl ATCKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATCKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATCKSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATCKSEL` writer - ATCKSEL"]
pub struct ATCKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ATCKSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `ATPER` reader - ATPER"]
pub struct ATPER_R(crate::FieldReader<u8, u8>);
impl ATPER_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATPER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATPER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATPER` writer - ATPER"]
pub struct ATPER_W<'a> {
    w: &'a mut W,
}
impl<'a> ATPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `ATOSHARE` reader - ATOSHARE"]
pub struct ATOSHARE_R(crate::FieldReader<bool, bool>);
impl ATOSHARE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ATOSHARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATOSHARE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATOSHARE` writer - ATOSHARE"]
pub struct ATOSHARE_W<'a> {
    w: &'a mut W,
}
impl<'a> ATOSHARE_W<'a> {
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
#[doc = "Field `FLTEN` reader - FLTEN"]
pub struct FLTEN_R(crate::FieldReader<bool, bool>);
impl FLTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLTEN` writer - FLTEN"]
pub struct FLTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTEN_W<'a> {
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
    #[doc = "Bit 0 - TAMP1AM"]
    #[inline(always)]
    pub fn tamp1am(&self) -> TAMP1AM_R {
        TAMP1AM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TAMP2AM"]
    #[inline(always)]
    pub fn tamp2am(&self) -> TAMP2AM_R {
        TAMP2AM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TAMP3AM"]
    #[inline(always)]
    pub fn tamp3am(&self) -> TAMP3AM_R {
        TAMP3AM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TAMP4AM"]
    #[inline(always)]
    pub fn tamp4am(&self) -> TAMP4AM_R {
        TAMP4AM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TAMP5AM"]
    #[inline(always)]
    pub fn tamp5am(&self) -> TAMP5AM_R {
        TAMP5AM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TAMP6AM"]
    #[inline(always)]
    pub fn tamp6am(&self) -> TAMP6AM_R {
        TAMP6AM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TAMP7AM"]
    #[inline(always)]
    pub fn tamp7am(&self) -> TAMP7AM_R {
        TAMP7AM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TAMP8AM"]
    #[inline(always)]
    pub fn tamp8am(&self) -> TAMP8AM_R {
        TAMP8AM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - ATOSEL1"]
    #[inline(always)]
    pub fn atosel1(&self) -> ATOSEL1_R {
        ATOSEL1_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - ATOSEL2"]
    #[inline(always)]
    pub fn atosel2(&self) -> ATOSEL2_R {
        ATOSEL2_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - ATOSEL3"]
    #[inline(always)]
    pub fn atosel3(&self) -> ATOSEL3_R {
        ATOSEL3_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - ATOSEL4"]
    #[inline(always)]
    pub fn atosel4(&self) -> ATOSEL4_R {
        ATOSEL4_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - ATCKSEL"]
    #[inline(always)]
    pub fn atcksel(&self) -> ATCKSEL_R {
        ATCKSEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - ATPER"]
    #[inline(always)]
    pub fn atper(&self) -> ATPER_R {
        ATPER_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 30 - ATOSHARE"]
    #[inline(always)]
    pub fn atoshare(&self) -> ATOSHARE_R {
        ATOSHARE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - FLTEN"]
    #[inline(always)]
    pub fn flten(&self) -> FLTEN_R {
        FLTEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMP1AM"]
    #[inline(always)]
    pub fn tamp1am(&mut self) -> TAMP1AM_W {
        TAMP1AM_W { w: self }
    }
    #[doc = "Bit 1 - TAMP2AM"]
    #[inline(always)]
    pub fn tamp2am(&mut self) -> TAMP2AM_W {
        TAMP2AM_W { w: self }
    }
    #[doc = "Bit 2 - TAMP3AM"]
    #[inline(always)]
    pub fn tamp3am(&mut self) -> TAMP3AM_W {
        TAMP3AM_W { w: self }
    }
    #[doc = "Bit 3 - TAMP4AM"]
    #[inline(always)]
    pub fn tamp4am(&mut self) -> TAMP4AM_W {
        TAMP4AM_W { w: self }
    }
    #[doc = "Bit 4 - TAMP5AM"]
    #[inline(always)]
    pub fn tamp5am(&mut self) -> TAMP5AM_W {
        TAMP5AM_W { w: self }
    }
    #[doc = "Bit 5 - TAMP6AM"]
    #[inline(always)]
    pub fn tamp6am(&mut self) -> TAMP6AM_W {
        TAMP6AM_W { w: self }
    }
    #[doc = "Bit 6 - TAMP7AM"]
    #[inline(always)]
    pub fn tamp7am(&mut self) -> TAMP7AM_W {
        TAMP7AM_W { w: self }
    }
    #[doc = "Bit 7 - TAMP8AM"]
    #[inline(always)]
    pub fn tamp8am(&mut self) -> TAMP8AM_W {
        TAMP8AM_W { w: self }
    }
    #[doc = "Bits 8:9 - ATOSEL1"]
    #[inline(always)]
    pub fn atosel1(&mut self) -> ATOSEL1_W {
        ATOSEL1_W { w: self }
    }
    #[doc = "Bits 10:11 - ATOSEL2"]
    #[inline(always)]
    pub fn atosel2(&mut self) -> ATOSEL2_W {
        ATOSEL2_W { w: self }
    }
    #[doc = "Bits 12:13 - ATOSEL3"]
    #[inline(always)]
    pub fn atosel3(&mut self) -> ATOSEL3_W {
        ATOSEL3_W { w: self }
    }
    #[doc = "Bits 14:15 - ATOSEL4"]
    #[inline(always)]
    pub fn atosel4(&mut self) -> ATOSEL4_W {
        ATOSEL4_W { w: self }
    }
    #[doc = "Bits 16:17 - ATCKSEL"]
    #[inline(always)]
    pub fn atcksel(&mut self) -> ATCKSEL_W {
        ATCKSEL_W { w: self }
    }
    #[doc = "Bits 24:25 - ATPER"]
    #[inline(always)]
    pub fn atper(&mut self) -> ATPER_W {
        ATPER_W { w: self }
    }
    #[doc = "Bit 30 - ATOSHARE"]
    #[inline(always)]
    pub fn atoshare(&mut self) -> ATOSHARE_W {
        ATOSHARE_W { w: self }
    }
    #[doc = "Bit 31 - FLTEN"]
    #[inline(always)]
    pub fn flten(&mut self) -> FLTEN_W {
        FLTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP active tamper control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atcr1](index.html) module"]
pub struct ATCR1_SPEC;
impl crate::RegisterSpec for ATCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atcr1::R](R) reader structure"]
impl crate::Readable for ATCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atcr1::W](W) writer structure"]
impl crate::Writable for ATCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ATCR1 to value 0x0007_0000"]
impl crate::Resettable for ATCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0007_0000
    }
}
