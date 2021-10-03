#[doc = "Register `ICR` reader"]
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXMSGDISCCF` reader - TXMSGDISCCF"]
pub struct TXMSGDISCCF_R(crate::FieldReader<bool, bool>);
impl TXMSGDISCCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMSGDISCCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMSGDISCCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMSGDISCCF` writer - TXMSGDISCCF"]
pub struct TXMSGDISCCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMSGDISCCF_W<'a> {
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
#[doc = "Field `TXMSGSENTCF` reader - TXMSGSENTCF"]
pub struct TXMSGSENTCF_R(crate::FieldReader<bool, bool>);
impl TXMSGSENTCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMSGSENTCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMSGSENTCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMSGSENTCF` writer - TXMSGSENTCF"]
pub struct TXMSGSENTCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMSGSENTCF_W<'a> {
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
#[doc = "Field `TXMSGABTCF` reader - TXMSGABTCF"]
pub struct TXMSGABTCF_R(crate::FieldReader<bool, bool>);
impl TXMSGABTCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMSGABTCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMSGABTCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMSGABTCF` writer - TXMSGABTCF"]
pub struct TXMSGABTCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMSGABTCF_W<'a> {
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
#[doc = "Field `HRSTDISCCF` reader - HRSTDISCCF"]
pub struct HRSTDISCCF_R(crate::FieldReader<bool, bool>);
impl HRSTDISCCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HRSTDISCCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HRSTDISCCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HRSTDISCCF` writer - HRSTDISCCF"]
pub struct HRSTDISCCF_W<'a> {
    w: &'a mut W,
}
impl<'a> HRSTDISCCF_W<'a> {
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
#[doc = "Field `HRSTSENTCF` reader - HRSTSENTCF"]
pub struct HRSTSENTCF_R(crate::FieldReader<bool, bool>);
impl HRSTSENTCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HRSTSENTCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HRSTSENTCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HRSTSENTCF` writer - HRSTSENTCF"]
pub struct HRSTSENTCF_W<'a> {
    w: &'a mut W,
}
impl<'a> HRSTSENTCF_W<'a> {
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
#[doc = "Field `TXUNDCF` reader - TXUNDCF"]
pub struct TXUNDCF_R(crate::FieldReader<bool, bool>);
impl TXUNDCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUNDCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUNDCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUNDCF` writer - TXUNDCF"]
pub struct TXUNDCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNDCF_W<'a> {
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
#[doc = "Field `RXORDDETCF` reader - RXORDDETCF"]
pub struct RXORDDETCF_R(crate::FieldReader<bool, bool>);
impl RXORDDETCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXORDDETCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXORDDETCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXORDDETCF` writer - RXORDDETCF"]
pub struct RXORDDETCF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXORDDETCF_W<'a> {
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
#[doc = "Field `RXHRSTDETCF` reader - RXHRSTDETCF"]
pub struct RXHRSTDETCF_R(crate::FieldReader<bool, bool>);
impl RXHRSTDETCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXHRSTDETCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXHRSTDETCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXHRSTDETCF` writer - RXHRSTDETCF"]
pub struct RXHRSTDETCF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXHRSTDETCF_W<'a> {
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
#[doc = "Field `RXOVRCF` reader - RXOVRCF"]
pub struct RXOVRCF_R(crate::FieldReader<bool, bool>);
impl RXOVRCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVRCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOVRCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOVRCF` writer - RXOVRCF"]
pub struct RXOVRCF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVRCF_W<'a> {
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
#[doc = "Field `RXMSGENDCF` reader - RXMSGENDCF"]
pub struct RXMSGENDCF_R(crate::FieldReader<bool, bool>);
impl RXMSGENDCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXMSGENDCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXMSGENDCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXMSGENDCF` writer - RXMSGENDCF"]
pub struct RXMSGENDCF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMSGENDCF_W<'a> {
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
#[doc = "Field `TYPECEVT1CF` reader - TYPECEVT1CF"]
pub struct TYPECEVT1CF_R(crate::FieldReader<bool, bool>);
impl TYPECEVT1CF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPECEVT1CF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TYPECEVT1CF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPECEVT1CF` writer - TYPECEVT1CF"]
pub struct TYPECEVT1CF_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPECEVT1CF_W<'a> {
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
#[doc = "Field `TYPECEVT2CF` reader - TYPECEVT2CF"]
pub struct TYPECEVT2CF_R(crate::FieldReader<bool, bool>);
impl TYPECEVT2CF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPECEVT2CF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TYPECEVT2CF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPECEVT2CF` writer - TYPECEVT2CF"]
pub struct TYPECEVT2CF_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPECEVT2CF_W<'a> {
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
#[doc = "Field `FRSEVTCF` reader - FRSEVTCF"]
pub struct FRSEVTCF_R(crate::FieldReader<bool, bool>);
impl FRSEVTCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRSEVTCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRSEVTCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRSEVTCF` writer - FRSEVTCF"]
pub struct FRSEVTCF_W<'a> {
    w: &'a mut W,
}
impl<'a> FRSEVTCF_W<'a> {
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
    #[doc = "Bit 1 - TXMSGDISCCF"]
    #[inline(always)]
    pub fn txmsgdisccf(&self) -> TXMSGDISCCF_R {
        TXMSGDISCCF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TXMSGSENTCF"]
    #[inline(always)]
    pub fn txmsgsentcf(&self) -> TXMSGSENTCF_R {
        TXMSGSENTCF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TXMSGABTCF"]
    #[inline(always)]
    pub fn txmsgabtcf(&self) -> TXMSGABTCF_R {
        TXMSGABTCF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HRSTDISCCF"]
    #[inline(always)]
    pub fn hrstdisccf(&self) -> HRSTDISCCF_R {
        HRSTDISCCF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HRSTSENTCF"]
    #[inline(always)]
    pub fn hrstsentcf(&self) -> HRSTSENTCF_R {
        HRSTSENTCF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TXUNDCF"]
    #[inline(always)]
    pub fn txundcf(&self) -> TXUNDCF_R {
        TXUNDCF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RXORDDETCF"]
    #[inline(always)]
    pub fn rxorddetcf(&self) -> RXORDDETCF_R {
        RXORDDETCF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RXHRSTDETCF"]
    #[inline(always)]
    pub fn rxhrstdetcf(&self) -> RXHRSTDETCF_R {
        RXHRSTDETCF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RXOVRCF"]
    #[inline(always)]
    pub fn rxovrcf(&self) -> RXOVRCF_R {
        RXOVRCF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RXMSGENDCF"]
    #[inline(always)]
    pub fn rxmsgendcf(&self) -> RXMSGENDCF_R {
        RXMSGENDCF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TYPECEVT1CF"]
    #[inline(always)]
    pub fn typecevt1cf(&self) -> TYPECEVT1CF_R {
        TYPECEVT1CF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TYPECEVT2CF"]
    #[inline(always)]
    pub fn typecevt2cf(&self) -> TYPECEVT2CF_R {
        TYPECEVT2CF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 20 - FRSEVTCF"]
    #[inline(always)]
    pub fn frsevtcf(&self) -> FRSEVTCF_R {
        FRSEVTCF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TXMSGDISCCF"]
    #[inline(always)]
    pub fn txmsgdisccf(&mut self) -> TXMSGDISCCF_W {
        TXMSGDISCCF_W { w: self }
    }
    #[doc = "Bit 2 - TXMSGSENTCF"]
    #[inline(always)]
    pub fn txmsgsentcf(&mut self) -> TXMSGSENTCF_W {
        TXMSGSENTCF_W { w: self }
    }
    #[doc = "Bit 3 - TXMSGABTCF"]
    #[inline(always)]
    pub fn txmsgabtcf(&mut self) -> TXMSGABTCF_W {
        TXMSGABTCF_W { w: self }
    }
    #[doc = "Bit 4 - HRSTDISCCF"]
    #[inline(always)]
    pub fn hrstdisccf(&mut self) -> HRSTDISCCF_W {
        HRSTDISCCF_W { w: self }
    }
    #[doc = "Bit 5 - HRSTSENTCF"]
    #[inline(always)]
    pub fn hrstsentcf(&mut self) -> HRSTSENTCF_W {
        HRSTSENTCF_W { w: self }
    }
    #[doc = "Bit 6 - TXUNDCF"]
    #[inline(always)]
    pub fn txundcf(&mut self) -> TXUNDCF_W {
        TXUNDCF_W { w: self }
    }
    #[doc = "Bit 9 - RXORDDETCF"]
    #[inline(always)]
    pub fn rxorddetcf(&mut self) -> RXORDDETCF_W {
        RXORDDETCF_W { w: self }
    }
    #[doc = "Bit 10 - RXHRSTDETCF"]
    #[inline(always)]
    pub fn rxhrstdetcf(&mut self) -> RXHRSTDETCF_W {
        RXHRSTDETCF_W { w: self }
    }
    #[doc = "Bit 11 - RXOVRCF"]
    #[inline(always)]
    pub fn rxovrcf(&mut self) -> RXOVRCF_W {
        RXOVRCF_W { w: self }
    }
    #[doc = "Bit 12 - RXMSGENDCF"]
    #[inline(always)]
    pub fn rxmsgendcf(&mut self) -> RXMSGENDCF_W {
        RXMSGENDCF_W { w: self }
    }
    #[doc = "Bit 14 - TYPECEVT1CF"]
    #[inline(always)]
    pub fn typecevt1cf(&mut self) -> TYPECEVT1CF_W {
        TYPECEVT1CF_W { w: self }
    }
    #[doc = "Bit 15 - TYPECEVT2CF"]
    #[inline(always)]
    pub fn typecevt2cf(&mut self) -> TYPECEVT2CF_W {
        TYPECEVT2CF_W { w: self }
    }
    #[doc = "Bit 20 - FRSEVTCF"]
    #[inline(always)]
    pub fn frsevtcf(&mut self) -> FRSEVTCF_W {
        FRSEVTCF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCPD Interrupt Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr::R](R) reader structure"]
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
