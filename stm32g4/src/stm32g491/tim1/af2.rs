#[doc = "Register `AF2` reader"]
pub struct R(crate::R<AF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AF2` writer"]
pub struct W(crate::W<AF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AF2_SPEC>;
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
impl From<crate::W<AF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCRSEL` reader - OCREF_CLR source selection"]
pub struct OCRSEL_R(crate::FieldReader<u8, u8>);
impl OCRSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        OCRSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCRSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCRSEL` writer - OCREF_CLR source selection"]
pub struct OCRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OCRSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `BK2CMP4P` reader - BRK2 COMP4 input polarity"]
pub struct BK2CMP4P_R(crate::FieldReader<bool, bool>);
impl BK2CMP4P_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2CMP4P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BK2CMP4P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2CMP4P` writer - BRK2 COMP4 input polarity"]
pub struct BK2CMP4P_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2CMP4P_W<'a> {
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
#[doc = "Field `BK2CMP3P` reader - BRK2 COMP3 input polarity"]
pub struct BK2CMP3P_R(crate::FieldReader<bool, bool>);
impl BK2CMP3P_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2CMP3P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BK2CMP3P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2CMP3P` writer - BRK2 COMP3 input polarity"]
pub struct BK2CMP3P_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2CMP3P_W<'a> {
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
#[doc = "Field `BK2CMP2P` reader - BRK2 COMP2 input polarity"]
pub struct BK2CMP2P_R(crate::FieldReader<bool, bool>);
impl BK2CMP2P_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2CMP2P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BK2CMP2P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2CMP2P` writer - BRK2 COMP2 input polarity"]
pub struct BK2CMP2P_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2CMP2P_W<'a> {
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
#[doc = "Field `BK2CMP1P` reader - BRK2 COMP1 input polarity"]
pub struct BK2CMP1P_R(crate::FieldReader<bool, bool>);
impl BK2CMP1P_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2CMP1P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BK2CMP1P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2CMP1P` writer - BRK2 COMP1 input polarity"]
pub struct BK2CMP1P_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2CMP1P_W<'a> {
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
#[doc = "Field `BK2INP` reader - BRK2 BKIN input polarity"]
pub struct BK2INP_R(crate::FieldReader<bool, bool>);
impl BK2INP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2INP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BK2INP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2INP` writer - BRK2 BKIN input polarity"]
pub struct BK2INP_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2INP_W<'a> {
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
#[doc = "Field `BK2CMP7E` reader - BRK2 COMP7 enable"]
pub struct BK2CMP7E_R(crate::FieldReader<bool, bool>);
impl BK2CMP7E_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2CMP7E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BK2CMP7E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2CMP7E` writer - BRK2 COMP7 enable"]
pub struct BK2CMP7E_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2CMP7E_W<'a> {
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
#[doc = "Field `BK2CMP6E` reader - BRK2 COMP6 enable"]
pub struct BK2CMP6E_R(crate::FieldReader<bool, bool>);
impl BK2CMP6E_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2CMP6E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BK2CMP6E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2CMP6E` writer - BRK2 COMP6 enable"]
pub struct BK2CMP6E_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2CMP6E_W<'a> {
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
#[doc = "Field `BK2CMP5E` reader - BRK2 COMP5 enable"]
pub struct BK2CMP5E_R(crate::FieldReader<bool, bool>);
impl BK2CMP5E_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2CMP5E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BK2CMP5E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2CMP5E` writer - BRK2 COMP5 enable"]
pub struct BK2CMP5E_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2CMP5E_W<'a> {
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
#[doc = "Field `BK2CMP4E` reader - BRK2 COMP4 enable"]
pub struct BK2CMP4E_R(crate::FieldReader<bool, bool>);
impl BK2CMP4E_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2CMP4E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BK2CMP4E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2CMP4E` writer - BRK2 COMP4 enable"]
pub struct BK2CMP4E_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2CMP4E_W<'a> {
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
#[doc = "Field `BK2CMP3E` reader - BRK2 COMP3 enable"]
pub struct BK2CMP3E_R(crate::FieldReader<bool, bool>);
impl BK2CMP3E_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2CMP3E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BK2CMP3E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2CMP3E` writer - BRK2 COMP3 enable"]
pub struct BK2CMP3E_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2CMP3E_W<'a> {
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
#[doc = "Field `BK2CMP2E` reader - BRK2 COMP2 enable"]
pub struct BK2CMP2E_R(crate::FieldReader<bool, bool>);
impl BK2CMP2E_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2CMP2E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BK2CMP2E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2CMP2E` writer - BRK2 COMP2 enable"]
pub struct BK2CMP2E_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2CMP2E_W<'a> {
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
#[doc = "Field `BK2CMP1E` reader - BRK2 COMP1 enable"]
pub struct BK2CMP1E_R(crate::FieldReader<bool, bool>);
impl BK2CMP1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2CMP1E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BK2CMP1E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2CMP1E` writer - BRK2 COMP1 enable"]
pub struct BK2CMP1E_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2CMP1E_W<'a> {
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
#[doc = "Field `BKINE` reader - BRK BKIN input enable"]
pub struct BKINE_R(crate::FieldReader<bool, bool>);
impl BKINE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKINE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKINE` writer - BRK BKIN input enable"]
pub struct BKINE_W<'a> {
    w: &'a mut W,
}
impl<'a> BKINE_W<'a> {
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
    #[doc = "Bits 16:18 - OCREF_CLR source selection"]
    #[inline(always)]
    pub fn ocrsel(&self) -> OCRSEL_R {
        OCRSEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 13 - BRK2 COMP4 input polarity"]
    #[inline(always)]
    pub fn bk2cmp4p(&self) -> BK2CMP4P_R {
        BK2CMP4P_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - BRK2 COMP3 input polarity"]
    #[inline(always)]
    pub fn bk2cmp3p(&self) -> BK2CMP3P_R {
        BK2CMP3P_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity"]
    #[inline(always)]
    pub fn bk2cmp2p(&self) -> BK2CMP2P_R {
        BK2CMP2P_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity"]
    #[inline(always)]
    pub fn bk2cmp1p(&self) -> BK2CMP1P_R {
        BK2CMP1P_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BRK2 BKIN input polarity"]
    #[inline(always)]
    pub fn bk2inp(&self) -> BK2INP_R {
        BK2INP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BRK2 COMP7 enable"]
    #[inline(always)]
    pub fn bk2cmp7e(&self) -> BK2CMP7E_R {
        BK2CMP7E_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BRK2 COMP6 enable"]
    #[inline(always)]
    pub fn bk2cmp6e(&self) -> BK2CMP6E_R {
        BK2CMP6E_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - BRK2 COMP5 enable"]
    #[inline(always)]
    pub fn bk2cmp5e(&self) -> BK2CMP5E_R {
        BK2CMP5E_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BRK2 COMP4 enable"]
    #[inline(always)]
    pub fn bk2cmp4e(&self) -> BK2CMP4E_R {
        BK2CMP4E_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BRK2 COMP3 enable"]
    #[inline(always)]
    pub fn bk2cmp3e(&self) -> BK2CMP3E_R {
        BK2CMP3E_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable"]
    #[inline(always)]
    pub fn bk2cmp2e(&self) -> BK2CMP2E_R {
        BK2CMP2E_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable"]
    #[inline(always)]
    pub fn bk2cmp1e(&self) -> BK2CMP1E_R {
        BK2CMP1E_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:18 - OCREF_CLR source selection"]
    #[inline(always)]
    pub fn ocrsel(&mut self) -> OCRSEL_W {
        OCRSEL_W { w: self }
    }
    #[doc = "Bit 13 - BRK2 COMP4 input polarity"]
    #[inline(always)]
    pub fn bk2cmp4p(&mut self) -> BK2CMP4P_W {
        BK2CMP4P_W { w: self }
    }
    #[doc = "Bit 12 - BRK2 COMP3 input polarity"]
    #[inline(always)]
    pub fn bk2cmp3p(&mut self) -> BK2CMP3P_W {
        BK2CMP3P_W { w: self }
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity"]
    #[inline(always)]
    pub fn bk2cmp2p(&mut self) -> BK2CMP2P_W {
        BK2CMP2P_W { w: self }
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity"]
    #[inline(always)]
    pub fn bk2cmp1p(&mut self) -> BK2CMP1P_W {
        BK2CMP1P_W { w: self }
    }
    #[doc = "Bit 9 - BRK2 BKIN input polarity"]
    #[inline(always)]
    pub fn bk2inp(&mut self) -> BK2INP_W {
        BK2INP_W { w: self }
    }
    #[doc = "Bit 7 - BRK2 COMP7 enable"]
    #[inline(always)]
    pub fn bk2cmp7e(&mut self) -> BK2CMP7E_W {
        BK2CMP7E_W { w: self }
    }
    #[doc = "Bit 6 - BRK2 COMP6 enable"]
    #[inline(always)]
    pub fn bk2cmp6e(&mut self) -> BK2CMP6E_W {
        BK2CMP6E_W { w: self }
    }
    #[doc = "Bit 5 - BRK2 COMP5 enable"]
    #[inline(always)]
    pub fn bk2cmp5e(&mut self) -> BK2CMP5E_W {
        BK2CMP5E_W { w: self }
    }
    #[doc = "Bit 4 - BRK2 COMP4 enable"]
    #[inline(always)]
    pub fn bk2cmp4e(&mut self) -> BK2CMP4E_W {
        BK2CMP4E_W { w: self }
    }
    #[doc = "Bit 3 - BRK2 COMP3 enable"]
    #[inline(always)]
    pub fn bk2cmp3e(&mut self) -> BK2CMP3E_W {
        BK2CMP3E_W { w: self }
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable"]
    #[inline(always)]
    pub fn bk2cmp2e(&mut self) -> BK2CMP2E_W {
        BK2CMP2E_W { w: self }
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable"]
    #[inline(always)]
    pub fn bk2cmp1e(&mut self) -> BK2CMP1E_W {
        BK2CMP1E_W { w: self }
    }
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    pub fn bkine(&mut self) -> BKINE_W {
        BKINE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM alternate function option register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [af2](index.html) module"]
pub struct AF2_SPEC;
impl crate::RegisterSpec for AF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [af2::R](R) reader structure"]
impl crate::Readable for AF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [af2::W](W) writer structure"]
impl crate::Writable for AF2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AF2 to value 0"]
impl crate::Resettable for AF2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
