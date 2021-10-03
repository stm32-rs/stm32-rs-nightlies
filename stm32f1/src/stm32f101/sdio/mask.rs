#[doc = "Register `MASK` reader"]
pub struct R(crate::R<MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASK` writer"]
pub struct W(crate::W<MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK_SPEC>;
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
impl From<crate::W<MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCRCFAILIE` reader - CCRCFAILIE"]
pub struct CCRCFAILIE_R(crate::FieldReader<bool, bool>);
impl CCRCFAILIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCRCFAILIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCRCFAILIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCRCFAILIE` writer - CCRCFAILIE"]
pub struct CCRCFAILIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCRCFAILIE_W<'a> {
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
#[doc = "Field `DCRCFAILIE` reader - DCRCFAILIE"]
pub struct DCRCFAILIE_R(crate::FieldReader<bool, bool>);
impl DCRCFAILIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCRCFAILIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCRCFAILIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCRCFAILIE` writer - DCRCFAILIE"]
pub struct DCRCFAILIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRCFAILIE_W<'a> {
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
#[doc = "Field `CTIMEOUTIE` reader - CTIMEOUTIE"]
pub struct CTIMEOUTIE_R(crate::FieldReader<bool, bool>);
impl CTIMEOUTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTIMEOUTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTIMEOUTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIMEOUTIE` writer - CTIMEOUTIE"]
pub struct CTIMEOUTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMEOUTIE_W<'a> {
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
#[doc = "Field `DTIMEOUTIE` reader - DTIMEOUTIE"]
pub struct DTIMEOUTIE_R(crate::FieldReader<bool, bool>);
impl DTIMEOUTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTIMEOUTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTIMEOUTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTIMEOUTIE` writer - DTIMEOUTIE"]
pub struct DTIMEOUTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DTIMEOUTIE_W<'a> {
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
#[doc = "Field `TXUNDERRIE` reader - TXUNDERRIE"]
pub struct TXUNDERRIE_R(crate::FieldReader<bool, bool>);
impl TXUNDERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUNDERRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUNDERRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUNDERRIE` writer - TXUNDERRIE"]
pub struct TXUNDERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNDERRIE_W<'a> {
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
#[doc = "Field `RXOVERRIE` reader - RXOVERRIE"]
pub struct RXOVERRIE_R(crate::FieldReader<bool, bool>);
impl RXOVERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVERRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOVERRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOVERRIE` writer - RXOVERRIE"]
pub struct RXOVERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVERRIE_W<'a> {
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
#[doc = "Field `CMDRENDIE` reader - CMDRENDIE"]
pub struct CMDRENDIE_R(crate::FieldReader<bool, bool>);
impl CMDRENDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDRENDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDRENDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDRENDIE` writer - CMDRENDIE"]
pub struct CMDRENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDRENDIE_W<'a> {
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
#[doc = "Field `CMDSENTIE` reader - CMDSENTIE"]
pub struct CMDSENTIE_R(crate::FieldReader<bool, bool>);
impl CMDSENTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDSENTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDSENTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDSENTIE` writer - CMDSENTIE"]
pub struct CMDSENTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDSENTIE_W<'a> {
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
#[doc = "Field `DATAENDIE` reader - DATAENDIE"]
pub struct DATAENDIE_R(crate::FieldReader<bool, bool>);
impl DATAENDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATAENDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAENDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAENDIE` writer - DATAENDIE"]
pub struct DATAENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAENDIE_W<'a> {
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
#[doc = "Field `STBITERRIE` reader - STBITERRIE"]
pub struct STBITERRIE_R(crate::FieldReader<bool, bool>);
impl STBITERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        STBITERRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STBITERRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STBITERRIE` writer - STBITERRIE"]
pub struct STBITERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> STBITERRIE_W<'a> {
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
#[doc = "Field `DBACKENDIE` reader - DBACKENDIE"]
pub struct DBACKENDIE_R(crate::FieldReader<bool, bool>);
impl DBACKENDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBACKENDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBACKENDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBACKENDIE` writer - DBACKENDIE"]
pub struct DBACKENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBACKENDIE_W<'a> {
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
#[doc = "Field `CMDACTIE` reader - CMDACTIE"]
pub struct CMDACTIE_R(crate::FieldReader<bool, bool>);
impl CMDACTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDACTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDACTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDACTIE` writer - CMDACTIE"]
pub struct CMDACTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDACTIE_W<'a> {
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
#[doc = "Field `TXACTIE` reader - TXACTIE"]
pub struct TXACTIE_R(crate::FieldReader<bool, bool>);
impl TXACTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXACTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXACTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXACTIE` writer - TXACTIE"]
pub struct TXACTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXACTIE_W<'a> {
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
#[doc = "Field `RXACTIE` reader - RXACTIE"]
pub struct RXACTIE_R(crate::FieldReader<bool, bool>);
impl RXACTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXACTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXACTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXACTIE` writer - RXACTIE"]
pub struct RXACTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXACTIE_W<'a> {
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
#[doc = "Field `TXFIFOHEIE` reader - TXFIFOHEIE"]
pub struct TXFIFOHEIE_R(crate::FieldReader<bool, bool>);
impl TXFIFOHEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFOHEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFOHEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFOHEIE` writer - TXFIFOHEIE"]
pub struct TXFIFOHEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFOHEIE_W<'a> {
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
#[doc = "Field `RXFIFOHFIE` reader - RXFIFOHFIE"]
pub struct RXFIFOHFIE_R(crate::FieldReader<bool, bool>);
impl RXFIFOHFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFOHFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFOHFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFOHFIE` writer - RXFIFOHFIE"]
pub struct RXFIFOHFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFOHFIE_W<'a> {
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
#[doc = "Field `TXFIFOFIE` reader - TXFIFOFIE"]
pub struct TXFIFOFIE_R(crate::FieldReader<bool, bool>);
impl TXFIFOFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFOFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFOFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFOFIE` writer - TXFIFOFIE"]
pub struct TXFIFOFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFOFIE_W<'a> {
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
#[doc = "Field `RXFIFOFIE` reader - RXFIFOFIE"]
pub struct RXFIFOFIE_R(crate::FieldReader<bool, bool>);
impl RXFIFOFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFOFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFOFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFOFIE` writer - RXFIFOFIE"]
pub struct RXFIFOFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFOFIE_W<'a> {
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
#[doc = "Field `TXFIFOEIE` reader - TXFIFOEIE"]
pub struct TXFIFOEIE_R(crate::FieldReader<bool, bool>);
impl TXFIFOEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFOEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFOEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFOEIE` writer - TXFIFOEIE"]
pub struct TXFIFOEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFOEIE_W<'a> {
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
#[doc = "Field `RXFIFOEIE` reader - RXFIFOEIE"]
pub struct RXFIFOEIE_R(crate::FieldReader<bool, bool>);
impl RXFIFOEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFOEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFOEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFOEIE` writer - RXFIFOEIE"]
pub struct RXFIFOEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFOEIE_W<'a> {
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
#[doc = "Field `TXDAVLIE` reader - TXDAVLIE"]
pub struct TXDAVLIE_R(crate::FieldReader<bool, bool>);
impl TXDAVLIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXDAVLIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDAVLIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDAVLIE` writer - TXDAVLIE"]
pub struct TXDAVLIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDAVLIE_W<'a> {
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
#[doc = "Field `RXDAVLIE` reader - RXDAVLIE"]
pub struct RXDAVLIE_R(crate::FieldReader<bool, bool>);
impl RXDAVLIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXDAVLIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDAVLIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDAVLIE` writer - RXDAVLIE"]
pub struct RXDAVLIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDAVLIE_W<'a> {
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
#[doc = "Field `SDIOITIE` reader - SDIOITIE"]
pub struct SDIOITIE_R(crate::FieldReader<bool, bool>);
impl SDIOITIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDIOITIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIOITIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIOITIE` writer - SDIOITIE"]
pub struct SDIOITIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIOITIE_W<'a> {
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
#[doc = "Field `CEATENDIE` reader - CEATENDIE"]
pub struct CEATENDIE_R(crate::FieldReader<bool, bool>);
impl CEATENDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEATENDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEATENDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEATENDIE` writer - CEATENDIE"]
pub struct CEATENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CEATENDIE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - CCRCFAILIE"]
    #[inline(always)]
    pub fn ccrcfailie(&self) -> CCRCFAILIE_R {
        CCRCFAILIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DCRCFAILIE"]
    #[inline(always)]
    pub fn dcrcfailie(&self) -> DCRCFAILIE_R {
        DCRCFAILIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CTIMEOUTIE"]
    #[inline(always)]
    pub fn ctimeoutie(&self) -> CTIMEOUTIE_R {
        CTIMEOUTIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DTIMEOUTIE"]
    #[inline(always)]
    pub fn dtimeoutie(&self) -> DTIMEOUTIE_R {
        DTIMEOUTIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TXUNDERRIE"]
    #[inline(always)]
    pub fn txunderrie(&self) -> TXUNDERRIE_R {
        TXUNDERRIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RXOVERRIE"]
    #[inline(always)]
    pub fn rxoverrie(&self) -> RXOVERRIE_R {
        RXOVERRIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CMDRENDIE"]
    #[inline(always)]
    pub fn cmdrendie(&self) -> CMDRENDIE_R {
        CMDRENDIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CMDSENTIE"]
    #[inline(always)]
    pub fn cmdsentie(&self) -> CMDSENTIE_R {
        CMDSENTIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DATAENDIE"]
    #[inline(always)]
    pub fn dataendie(&self) -> DATAENDIE_R {
        DATAENDIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - STBITERRIE"]
    #[inline(always)]
    pub fn stbiterrie(&self) -> STBITERRIE_R {
        STBITERRIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DBACKENDIE"]
    #[inline(always)]
    pub fn dbackendie(&self) -> DBACKENDIE_R {
        DBACKENDIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CMDACTIE"]
    #[inline(always)]
    pub fn cmdactie(&self) -> CMDACTIE_R {
        CMDACTIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TXACTIE"]
    #[inline(always)]
    pub fn txactie(&self) -> TXACTIE_R {
        TXACTIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RXACTIE"]
    #[inline(always)]
    pub fn rxactie(&self) -> RXACTIE_R {
        RXACTIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TXFIFOHEIE"]
    #[inline(always)]
    pub fn txfifoheie(&self) -> TXFIFOHEIE_R {
        TXFIFOHEIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RXFIFOHFIE"]
    #[inline(always)]
    pub fn rxfifohfie(&self) -> RXFIFOHFIE_R {
        RXFIFOHFIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TXFIFOFIE"]
    #[inline(always)]
    pub fn txfifofie(&self) -> TXFIFOFIE_R {
        TXFIFOFIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RXFIFOFIE"]
    #[inline(always)]
    pub fn rxfifofie(&self) -> RXFIFOFIE_R {
        RXFIFOFIE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TXFIFOEIE"]
    #[inline(always)]
    pub fn txfifoeie(&self) -> TXFIFOEIE_R {
        TXFIFOEIE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - RXFIFOEIE"]
    #[inline(always)]
    pub fn rxfifoeie(&self) -> RXFIFOEIE_R {
        RXFIFOEIE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - TXDAVLIE"]
    #[inline(always)]
    pub fn txdavlie(&self) -> TXDAVLIE_R {
        TXDAVLIE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - RXDAVLIE"]
    #[inline(always)]
    pub fn rxdavlie(&self) -> RXDAVLIE_R {
        RXDAVLIE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SDIOITIE"]
    #[inline(always)]
    pub fn sdioitie(&self) -> SDIOITIE_R {
        SDIOITIE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CEATENDIE"]
    #[inline(always)]
    pub fn ceatendie(&self) -> CEATENDIE_R {
        CEATENDIE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCRCFAILIE"]
    #[inline(always)]
    pub fn ccrcfailie(&mut self) -> CCRCFAILIE_W {
        CCRCFAILIE_W { w: self }
    }
    #[doc = "Bit 1 - DCRCFAILIE"]
    #[inline(always)]
    pub fn dcrcfailie(&mut self) -> DCRCFAILIE_W {
        DCRCFAILIE_W { w: self }
    }
    #[doc = "Bit 2 - CTIMEOUTIE"]
    #[inline(always)]
    pub fn ctimeoutie(&mut self) -> CTIMEOUTIE_W {
        CTIMEOUTIE_W { w: self }
    }
    #[doc = "Bit 3 - DTIMEOUTIE"]
    #[inline(always)]
    pub fn dtimeoutie(&mut self) -> DTIMEOUTIE_W {
        DTIMEOUTIE_W { w: self }
    }
    #[doc = "Bit 4 - TXUNDERRIE"]
    #[inline(always)]
    pub fn txunderrie(&mut self) -> TXUNDERRIE_W {
        TXUNDERRIE_W { w: self }
    }
    #[doc = "Bit 5 - RXOVERRIE"]
    #[inline(always)]
    pub fn rxoverrie(&mut self) -> RXOVERRIE_W {
        RXOVERRIE_W { w: self }
    }
    #[doc = "Bit 6 - CMDRENDIE"]
    #[inline(always)]
    pub fn cmdrendie(&mut self) -> CMDRENDIE_W {
        CMDRENDIE_W { w: self }
    }
    #[doc = "Bit 7 - CMDSENTIE"]
    #[inline(always)]
    pub fn cmdsentie(&mut self) -> CMDSENTIE_W {
        CMDSENTIE_W { w: self }
    }
    #[doc = "Bit 8 - DATAENDIE"]
    #[inline(always)]
    pub fn dataendie(&mut self) -> DATAENDIE_W {
        DATAENDIE_W { w: self }
    }
    #[doc = "Bit 9 - STBITERRIE"]
    #[inline(always)]
    pub fn stbiterrie(&mut self) -> STBITERRIE_W {
        STBITERRIE_W { w: self }
    }
    #[doc = "Bit 10 - DBACKENDIE"]
    #[inline(always)]
    pub fn dbackendie(&mut self) -> DBACKENDIE_W {
        DBACKENDIE_W { w: self }
    }
    #[doc = "Bit 11 - CMDACTIE"]
    #[inline(always)]
    pub fn cmdactie(&mut self) -> CMDACTIE_W {
        CMDACTIE_W { w: self }
    }
    #[doc = "Bit 12 - TXACTIE"]
    #[inline(always)]
    pub fn txactie(&mut self) -> TXACTIE_W {
        TXACTIE_W { w: self }
    }
    #[doc = "Bit 13 - RXACTIE"]
    #[inline(always)]
    pub fn rxactie(&mut self) -> RXACTIE_W {
        RXACTIE_W { w: self }
    }
    #[doc = "Bit 14 - TXFIFOHEIE"]
    #[inline(always)]
    pub fn txfifoheie(&mut self) -> TXFIFOHEIE_W {
        TXFIFOHEIE_W { w: self }
    }
    #[doc = "Bit 15 - RXFIFOHFIE"]
    #[inline(always)]
    pub fn rxfifohfie(&mut self) -> RXFIFOHFIE_W {
        RXFIFOHFIE_W { w: self }
    }
    #[doc = "Bit 16 - TXFIFOFIE"]
    #[inline(always)]
    pub fn txfifofie(&mut self) -> TXFIFOFIE_W {
        TXFIFOFIE_W { w: self }
    }
    #[doc = "Bit 17 - RXFIFOFIE"]
    #[inline(always)]
    pub fn rxfifofie(&mut self) -> RXFIFOFIE_W {
        RXFIFOFIE_W { w: self }
    }
    #[doc = "Bit 18 - TXFIFOEIE"]
    #[inline(always)]
    pub fn txfifoeie(&mut self) -> TXFIFOEIE_W {
        TXFIFOEIE_W { w: self }
    }
    #[doc = "Bit 19 - RXFIFOEIE"]
    #[inline(always)]
    pub fn rxfifoeie(&mut self) -> RXFIFOEIE_W {
        RXFIFOEIE_W { w: self }
    }
    #[doc = "Bit 20 - TXDAVLIE"]
    #[inline(always)]
    pub fn txdavlie(&mut self) -> TXDAVLIE_W {
        TXDAVLIE_W { w: self }
    }
    #[doc = "Bit 21 - RXDAVLIE"]
    #[inline(always)]
    pub fn rxdavlie(&mut self) -> RXDAVLIE_W {
        RXDAVLIE_W { w: self }
    }
    #[doc = "Bit 22 - SDIOITIE"]
    #[inline(always)]
    pub fn sdioitie(&mut self) -> SDIOITIE_W {
        SDIOITIE_W { w: self }
    }
    #[doc = "Bit 23 - CEATENDIE"]
    #[inline(always)]
    pub fn ceatendie(&mut self) -> CEATENDIE_W {
        CEATENDIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDIO mask register (SDIO_MASK)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask](index.html) module"]
pub struct MASK_SPEC;
impl crate::RegisterSpec for MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mask::R](R) reader structure"]
impl crate::Readable for MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mask::W](W) writer structure"]
impl crate::Writable for MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MASK to value 0"]
impl crate::Resettable for MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
