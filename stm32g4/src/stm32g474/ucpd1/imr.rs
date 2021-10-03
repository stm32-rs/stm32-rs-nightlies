#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR` writer"]
pub struct W(crate::W<IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR_SPEC>;
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
impl From<crate::W<IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXISIE` reader - TXISIE"]
pub struct TXISIE_R(crate::FieldReader<bool, bool>);
impl TXISIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXISIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXISIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXISIE` writer - TXISIE"]
pub struct TXISIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXISIE_W<'a> {
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
#[doc = "Field `TXMSGDISCIE` reader - TXMSGDISCIE"]
pub struct TXMSGDISCIE_R(crate::FieldReader<bool, bool>);
impl TXMSGDISCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMSGDISCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMSGDISCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMSGDISCIE` writer - TXMSGDISCIE"]
pub struct TXMSGDISCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMSGDISCIE_W<'a> {
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
#[doc = "Field `TXMSGSENTIE` reader - TXMSGSENTIE"]
pub struct TXMSGSENTIE_R(crate::FieldReader<bool, bool>);
impl TXMSGSENTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMSGSENTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMSGSENTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMSGSENTIE` writer - TXMSGSENTIE"]
pub struct TXMSGSENTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMSGSENTIE_W<'a> {
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
#[doc = "Field `TXMSGABTIE` reader - TXMSGABTIE"]
pub struct TXMSGABTIE_R(crate::FieldReader<bool, bool>);
impl TXMSGABTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMSGABTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMSGABTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMSGABTIE` writer - TXMSGABTIE"]
pub struct TXMSGABTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMSGABTIE_W<'a> {
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
#[doc = "Field `HRSTDISCIE` reader - HRSTDISCIE"]
pub struct HRSTDISCIE_R(crate::FieldReader<bool, bool>);
impl HRSTDISCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HRSTDISCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HRSTDISCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HRSTDISCIE` writer - HRSTDISCIE"]
pub struct HRSTDISCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HRSTDISCIE_W<'a> {
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
#[doc = "Field `HRSTSENTIE` reader - HRSTSENTIE"]
pub struct HRSTSENTIE_R(crate::FieldReader<bool, bool>);
impl HRSTSENTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HRSTSENTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HRSTSENTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HRSTSENTIE` writer - HRSTSENTIE"]
pub struct HRSTSENTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HRSTSENTIE_W<'a> {
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
#[doc = "Field `TXUNDIE` reader - TXUNDIE"]
pub struct TXUNDIE_R(crate::FieldReader<bool, bool>);
impl TXUNDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUNDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUNDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUNDIE` writer - TXUNDIE"]
pub struct TXUNDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNDIE_W<'a> {
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
#[doc = "Field `RXNEIE` reader - RXNEIE"]
pub struct RXNEIE_R(crate::FieldReader<bool, bool>);
impl RXNEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXNEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXNEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXNEIE` writer - RXNEIE"]
pub struct RXNEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXNEIE_W<'a> {
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
#[doc = "Field `RXORDDETIE` reader - RXORDDETIE"]
pub struct RXORDDETIE_R(crate::FieldReader<bool, bool>);
impl RXORDDETIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXORDDETIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXORDDETIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXORDDETIE` writer - RXORDDETIE"]
pub struct RXORDDETIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXORDDETIE_W<'a> {
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
#[doc = "Field `RXHRSTDETIE` reader - RXHRSTDETIE"]
pub struct RXHRSTDETIE_R(crate::FieldReader<bool, bool>);
impl RXHRSTDETIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXHRSTDETIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXHRSTDETIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXHRSTDETIE` writer - RXHRSTDETIE"]
pub struct RXHRSTDETIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXHRSTDETIE_W<'a> {
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
#[doc = "Field `RXOVRIE` reader - RXOVRIE"]
pub struct RXOVRIE_R(crate::FieldReader<bool, bool>);
impl RXOVRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOVRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOVRIE` writer - RXOVRIE"]
pub struct RXOVRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVRIE_W<'a> {
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
#[doc = "Field `RXMSGENDIE` reader - RXMSGENDIE"]
pub struct RXMSGENDIE_R(crate::FieldReader<bool, bool>);
impl RXMSGENDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXMSGENDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXMSGENDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXMSGENDIE` writer - RXMSGENDIE"]
pub struct RXMSGENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMSGENDIE_W<'a> {
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
#[doc = "Field `TYPECEVT1IE` reader - TYPECEVT1IE"]
pub struct TYPECEVT1IE_R(crate::FieldReader<bool, bool>);
impl TYPECEVT1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPECEVT1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TYPECEVT1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPECEVT1IE` writer - TYPECEVT1IE"]
pub struct TYPECEVT1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPECEVT1IE_W<'a> {
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
#[doc = "Field `TYPECEVT2IE` reader - TYPECEVT2IE"]
pub struct TYPECEVT2IE_R(crate::FieldReader<bool, bool>);
impl TYPECEVT2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPECEVT2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TYPECEVT2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPECEVT2IE` writer - TYPECEVT2IE"]
pub struct TYPECEVT2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPECEVT2IE_W<'a> {
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
#[doc = "Field `FRSEVTIE` reader - FRSEVTIE"]
pub struct FRSEVTIE_R(crate::FieldReader<bool, bool>);
impl FRSEVTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRSEVTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRSEVTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRSEVTIE` writer - FRSEVTIE"]
pub struct FRSEVTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRSEVTIE_W<'a> {
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
    #[doc = "Bit 0 - TXISIE"]
    #[inline(always)]
    pub fn txisie(&self) -> TXISIE_R {
        TXISIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TXMSGDISCIE"]
    #[inline(always)]
    pub fn txmsgdiscie(&self) -> TXMSGDISCIE_R {
        TXMSGDISCIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TXMSGSENTIE"]
    #[inline(always)]
    pub fn txmsgsentie(&self) -> TXMSGSENTIE_R {
        TXMSGSENTIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TXMSGABTIE"]
    #[inline(always)]
    pub fn txmsgabtie(&self) -> TXMSGABTIE_R {
        TXMSGABTIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HRSTDISCIE"]
    #[inline(always)]
    pub fn hrstdiscie(&self) -> HRSTDISCIE_R {
        HRSTDISCIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HRSTSENTIE"]
    #[inline(always)]
    pub fn hrstsentie(&self) -> HRSTSENTIE_R {
        HRSTSENTIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TXUNDIE"]
    #[inline(always)]
    pub fn txundie(&self) -> TXUNDIE_R {
        TXUNDIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RXNEIE"]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RXORDDETIE"]
    #[inline(always)]
    pub fn rxorddetie(&self) -> RXORDDETIE_R {
        RXORDDETIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RXHRSTDETIE"]
    #[inline(always)]
    pub fn rxhrstdetie(&self) -> RXHRSTDETIE_R {
        RXHRSTDETIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RXOVRIE"]
    #[inline(always)]
    pub fn rxovrie(&self) -> RXOVRIE_R {
        RXOVRIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RXMSGENDIE"]
    #[inline(always)]
    pub fn rxmsgendie(&self) -> RXMSGENDIE_R {
        RXMSGENDIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TYPECEVT1IE"]
    #[inline(always)]
    pub fn typecevt1ie(&self) -> TYPECEVT1IE_R {
        TYPECEVT1IE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TYPECEVT2IE"]
    #[inline(always)]
    pub fn typecevt2ie(&self) -> TYPECEVT2IE_R {
        TYPECEVT2IE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 20 - FRSEVTIE"]
    #[inline(always)]
    pub fn frsevtie(&self) -> FRSEVTIE_R {
        FRSEVTIE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXISIE"]
    #[inline(always)]
    pub fn txisie(&mut self) -> TXISIE_W {
        TXISIE_W { w: self }
    }
    #[doc = "Bit 1 - TXMSGDISCIE"]
    #[inline(always)]
    pub fn txmsgdiscie(&mut self) -> TXMSGDISCIE_W {
        TXMSGDISCIE_W { w: self }
    }
    #[doc = "Bit 2 - TXMSGSENTIE"]
    #[inline(always)]
    pub fn txmsgsentie(&mut self) -> TXMSGSENTIE_W {
        TXMSGSENTIE_W { w: self }
    }
    #[doc = "Bit 3 - TXMSGABTIE"]
    #[inline(always)]
    pub fn txmsgabtie(&mut self) -> TXMSGABTIE_W {
        TXMSGABTIE_W { w: self }
    }
    #[doc = "Bit 4 - HRSTDISCIE"]
    #[inline(always)]
    pub fn hrstdiscie(&mut self) -> HRSTDISCIE_W {
        HRSTDISCIE_W { w: self }
    }
    #[doc = "Bit 5 - HRSTSENTIE"]
    #[inline(always)]
    pub fn hrstsentie(&mut self) -> HRSTSENTIE_W {
        HRSTSENTIE_W { w: self }
    }
    #[doc = "Bit 6 - TXUNDIE"]
    #[inline(always)]
    pub fn txundie(&mut self) -> TXUNDIE_W {
        TXUNDIE_W { w: self }
    }
    #[doc = "Bit 8 - RXNEIE"]
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W {
        RXNEIE_W { w: self }
    }
    #[doc = "Bit 9 - RXORDDETIE"]
    #[inline(always)]
    pub fn rxorddetie(&mut self) -> RXORDDETIE_W {
        RXORDDETIE_W { w: self }
    }
    #[doc = "Bit 10 - RXHRSTDETIE"]
    #[inline(always)]
    pub fn rxhrstdetie(&mut self) -> RXHRSTDETIE_W {
        RXHRSTDETIE_W { w: self }
    }
    #[doc = "Bit 11 - RXOVRIE"]
    #[inline(always)]
    pub fn rxovrie(&mut self) -> RXOVRIE_W {
        RXOVRIE_W { w: self }
    }
    #[doc = "Bit 12 - RXMSGENDIE"]
    #[inline(always)]
    pub fn rxmsgendie(&mut self) -> RXMSGENDIE_W {
        RXMSGENDIE_W { w: self }
    }
    #[doc = "Bit 14 - TYPECEVT1IE"]
    #[inline(always)]
    pub fn typecevt1ie(&mut self) -> TYPECEVT1IE_W {
        TYPECEVT1IE_W { w: self }
    }
    #[doc = "Bit 15 - TYPECEVT2IE"]
    #[inline(always)]
    pub fn typecevt2ie(&mut self) -> TYPECEVT2IE_W {
        TYPECEVT2IE_W { w: self }
    }
    #[doc = "Bit 20 - FRSEVTIE"]
    #[inline(always)]
    pub fn frsevtie(&mut self) -> FRSEVTIE_W {
        FRSEVTIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCPD Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr::W](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
