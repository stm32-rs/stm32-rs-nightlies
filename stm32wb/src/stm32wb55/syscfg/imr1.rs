#[doc = "Register `IMR1` reader"]
pub struct R(crate::R<IMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR1` writer"]
pub struct W(crate::W<IMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR1_SPEC>;
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
impl From<crate::W<IMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM1IM` reader - Peripheral TIM1 interrupt mask to CPU1"]
pub struct TIM1IM_R(crate::FieldReader<bool, bool>);
impl TIM1IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM1IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM1IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM1IM` writer - Peripheral TIM1 interrupt mask to CPU1"]
pub struct TIM1IM_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1IM_W<'a> {
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
#[doc = "Field `TIM16IM` reader - Peripheral TIM16 interrupt mask to CPU1"]
pub struct TIM16IM_R(crate::FieldReader<bool, bool>);
impl TIM16IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM16IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM16IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM16IM` writer - Peripheral TIM16 interrupt mask to CPU1"]
pub struct TIM16IM_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16IM_W<'a> {
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
#[doc = "Field `TIM17IM` reader - Peripheral TIM17 interrupt mask to CPU1"]
pub struct TIM17IM_R(crate::FieldReader<bool, bool>);
impl TIM17IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM17IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM17IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM17IM` writer - Peripheral TIM17 interrupt mask to CPU1"]
pub struct TIM17IM_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17IM_W<'a> {
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
#[doc = "Field `EXIT5IM` reader - Peripheral EXIT5 interrupt mask to CPU1"]
pub struct EXIT5IM_R(crate::FieldReader<bool, bool>);
impl EXIT5IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXIT5IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXIT5IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXIT5IM` writer - Peripheral EXIT5 interrupt mask to CPU1"]
pub struct EXIT5IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXIT5IM_W<'a> {
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
#[doc = "Field `EXIT6IM` reader - Peripheral EXIT6 interrupt mask to CPU1"]
pub struct EXIT6IM_R(crate::FieldReader<bool, bool>);
impl EXIT6IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXIT6IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXIT6IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXIT6IM` writer - Peripheral EXIT6 interrupt mask to CPU1"]
pub struct EXIT6IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXIT6IM_W<'a> {
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
#[doc = "Field `EXIT7IM` reader - Peripheral EXIT7 interrupt mask to CPU1"]
pub struct EXIT7IM_R(crate::FieldReader<bool, bool>);
impl EXIT7IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXIT7IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXIT7IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXIT7IM` writer - Peripheral EXIT7 interrupt mask to CPU1"]
pub struct EXIT7IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXIT7IM_W<'a> {
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
#[doc = "Field `EXIT8IM` reader - Peripheral EXIT8 interrupt mask to CPU1"]
pub struct EXIT8IM_R(crate::FieldReader<bool, bool>);
impl EXIT8IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXIT8IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXIT8IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXIT8IM` writer - Peripheral EXIT8 interrupt mask to CPU1"]
pub struct EXIT8IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXIT8IM_W<'a> {
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
#[doc = "Field `EXIT9IM` reader - Peripheral EXIT9 interrupt mask to CPU1"]
pub struct EXIT9IM_R(crate::FieldReader<bool, bool>);
impl EXIT9IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXIT9IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXIT9IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXIT9IM` writer - Peripheral EXIT9 interrupt mask to CPU1"]
pub struct EXIT9IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXIT9IM_W<'a> {
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
#[doc = "Field `EXIT10IM` reader - Peripheral EXIT10 interrupt mask to CPU1"]
pub struct EXIT10IM_R(crate::FieldReader<bool, bool>);
impl EXIT10IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXIT10IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXIT10IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXIT10IM` writer - Peripheral EXIT10 interrupt mask to CPU1"]
pub struct EXIT10IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXIT10IM_W<'a> {
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
#[doc = "Field `EXIT11IM` reader - Peripheral EXIT11 interrupt mask to CPU1"]
pub struct EXIT11IM_R(crate::FieldReader<bool, bool>);
impl EXIT11IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXIT11IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXIT11IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXIT11IM` writer - Peripheral EXIT11 interrupt mask to CPU1"]
pub struct EXIT11IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXIT11IM_W<'a> {
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
#[doc = "Field `EXIT12IM` reader - Peripheral EXIT12 interrupt mask to CPU1"]
pub struct EXIT12IM_R(crate::FieldReader<bool, bool>);
impl EXIT12IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXIT12IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXIT12IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXIT12IM` writer - Peripheral EXIT12 interrupt mask to CPU1"]
pub struct EXIT12IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXIT12IM_W<'a> {
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
#[doc = "Field `EXIT13IM` reader - Peripheral EXIT13 interrupt mask to CPU1"]
pub struct EXIT13IM_R(crate::FieldReader<bool, bool>);
impl EXIT13IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXIT13IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXIT13IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXIT13IM` writer - Peripheral EXIT13 interrupt mask to CPU1"]
pub struct EXIT13IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXIT13IM_W<'a> {
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
#[doc = "Field `EXIT14IM` reader - Peripheral EXIT14 interrupt mask to CPU1"]
pub struct EXIT14IM_R(crate::FieldReader<bool, bool>);
impl EXIT14IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXIT14IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXIT14IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXIT14IM` writer - Peripheral EXIT14 interrupt mask to CPU1"]
pub struct EXIT14IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXIT14IM_W<'a> {
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
#[doc = "Field `EXIT15IM` reader - Peripheral EXIT15 interrupt mask to CPU1"]
pub struct EXIT15IM_R(crate::FieldReader<bool, bool>);
impl EXIT15IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXIT15IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXIT15IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXIT15IM` writer - Peripheral EXIT15 interrupt mask to CPU1"]
pub struct EXIT15IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXIT15IM_W<'a> {
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
    #[doc = "Bit 13 - Peripheral TIM1 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn tim1im(&self) -> TIM1IM_R {
        TIM1IM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Peripheral TIM16 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn tim16im(&self) -> TIM16IM_R {
        TIM16IM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Peripheral TIM17 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn tim17im(&self) -> TIM17IM_R {
        TIM17IM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Peripheral EXIT5 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit5im(&self) -> EXIT5IM_R {
        EXIT5IM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Peripheral EXIT6 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit6im(&self) -> EXIT6IM_R {
        EXIT6IM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Peripheral EXIT7 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit7im(&self) -> EXIT7IM_R {
        EXIT7IM_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Peripheral EXIT8 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit8im(&self) -> EXIT8IM_R {
        EXIT8IM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Peripheral EXIT9 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit9im(&self) -> EXIT9IM_R {
        EXIT9IM_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Peripheral EXIT10 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit10im(&self) -> EXIT10IM_R {
        EXIT10IM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Peripheral EXIT11 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit11im(&self) -> EXIT11IM_R {
        EXIT11IM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Peripheral EXIT12 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit12im(&self) -> EXIT12IM_R {
        EXIT12IM_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Peripheral EXIT13 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit13im(&self) -> EXIT13IM_R {
        EXIT13IM_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Peripheral EXIT14 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit14im(&self) -> EXIT14IM_R {
        EXIT14IM_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Peripheral EXIT15 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit15im(&self) -> EXIT15IM_R {
        EXIT15IM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Peripheral TIM1 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn tim1im(&mut self) -> TIM1IM_W {
        TIM1IM_W { w: self }
    }
    #[doc = "Bit 14 - Peripheral TIM16 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn tim16im(&mut self) -> TIM16IM_W {
        TIM16IM_W { w: self }
    }
    #[doc = "Bit 15 - Peripheral TIM17 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn tim17im(&mut self) -> TIM17IM_W {
        TIM17IM_W { w: self }
    }
    #[doc = "Bit 21 - Peripheral EXIT5 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit5im(&mut self) -> EXIT5IM_W {
        EXIT5IM_W { w: self }
    }
    #[doc = "Bit 22 - Peripheral EXIT6 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit6im(&mut self) -> EXIT6IM_W {
        EXIT6IM_W { w: self }
    }
    #[doc = "Bit 23 - Peripheral EXIT7 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit7im(&mut self) -> EXIT7IM_W {
        EXIT7IM_W { w: self }
    }
    #[doc = "Bit 24 - Peripheral EXIT8 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit8im(&mut self) -> EXIT8IM_W {
        EXIT8IM_W { w: self }
    }
    #[doc = "Bit 25 - Peripheral EXIT9 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit9im(&mut self) -> EXIT9IM_W {
        EXIT9IM_W { w: self }
    }
    #[doc = "Bit 26 - Peripheral EXIT10 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit10im(&mut self) -> EXIT10IM_W {
        EXIT10IM_W { w: self }
    }
    #[doc = "Bit 27 - Peripheral EXIT11 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit11im(&mut self) -> EXIT11IM_W {
        EXIT11IM_W { w: self }
    }
    #[doc = "Bit 28 - Peripheral EXIT12 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit12im(&mut self) -> EXIT12IM_W {
        EXIT12IM_W { w: self }
    }
    #[doc = "Bit 29 - Peripheral EXIT13 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit13im(&mut self) -> EXIT13IM_W {
        EXIT13IM_W { w: self }
    }
    #[doc = "Bit 30 - Peripheral EXIT14 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit14im(&mut self) -> EXIT14IM_W {
        EXIT14IM_W { w: self }
    }
    #[doc = "Bit 31 - Peripheral EXIT15 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit15im(&mut self) -> EXIT15IM_W {
        EXIT15IM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU1 interrupt mask register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr1](index.html) module"]
pub struct IMR1_SPEC;
impl crate::RegisterSpec for IMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr1::R](R) reader structure"]
impl crate::Readable for IMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr1::W](W) writer structure"]
impl crate::Writable for IMR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMR1 to value 0"]
impl crate::Resettable for IMR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
