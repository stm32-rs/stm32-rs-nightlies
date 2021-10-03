#[doc = "Register `AF1` reader"]
pub struct R(crate::R<AF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AF1` writer"]
pub struct W(crate::W<AF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AF1_SPEC>;
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
impl From<crate::W<AF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETRSEL` reader - ETR source selection"]
pub struct ETRSEL_R(crate::FieldReader<u8, u8>);
impl ETRSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ETRSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETRSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETRSEL` writer - ETR source selection"]
pub struct ETRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ETRSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 14)) | ((value as u32 & 0x0f) << 14);
        self.w
    }
}
#[doc = "Field `BKCMP4P` reader - BRK COMP4 input polarity"]
pub struct BKCMP4P_R(crate::FieldReader<bool, bool>);
impl BKCMP4P_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKCMP4P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKCMP4P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKCMP4P` writer - BRK COMP4 input polarity"]
pub struct BKCMP4P_W<'a> {
    w: &'a mut W,
}
impl<'a> BKCMP4P_W<'a> {
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
#[doc = "Field `BKCMP3P` reader - BRK COMP3 input polarity"]
pub struct BKCMP3P_R(crate::FieldReader<bool, bool>);
impl BKCMP3P_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKCMP3P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKCMP3P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKCMP3P` writer - BRK COMP3 input polarity"]
pub struct BKCMP3P_W<'a> {
    w: &'a mut W,
}
impl<'a> BKCMP3P_W<'a> {
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
#[doc = "Field `BKCMP2P` reader - BRK COMP2 input polarity"]
pub struct BKCMP2P_R(crate::FieldReader<bool, bool>);
impl BKCMP2P_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKCMP2P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKCMP2P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKCMP2P` writer - BRK COMP2 input polarity"]
pub struct BKCMP2P_W<'a> {
    w: &'a mut W,
}
impl<'a> BKCMP2P_W<'a> {
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
#[doc = "Field `BKCMP1P` reader - BRK COMP1 input polarity"]
pub struct BKCMP1P_R(crate::FieldReader<bool, bool>);
impl BKCMP1P_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKCMP1P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKCMP1P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKCMP1P` writer - BRK COMP1 input polarity"]
pub struct BKCMP1P_W<'a> {
    w: &'a mut W,
}
impl<'a> BKCMP1P_W<'a> {
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
#[doc = "Field `BKINP` reader - BRK BKIN input polarity"]
pub struct BKINP_R(crate::FieldReader<bool, bool>);
impl BKINP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKINP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKINP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKINP` writer - BRK BKIN input polarity"]
pub struct BKINP_W<'a> {
    w: &'a mut W,
}
impl<'a> BKINP_W<'a> {
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
#[doc = "Field `BKCMP7E` reader - BRK COMP7 enable"]
pub struct BKCMP7E_R(crate::FieldReader<bool, bool>);
impl BKCMP7E_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKCMP7E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKCMP7E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKCMP7E` writer - BRK COMP7 enable"]
pub struct BKCMP7E_W<'a> {
    w: &'a mut W,
}
impl<'a> BKCMP7E_W<'a> {
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
#[doc = "Field `BKCMP6E` reader - BRK COMP6 enable"]
pub struct BKCMP6E_R(crate::FieldReader<bool, bool>);
impl BKCMP6E_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKCMP6E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKCMP6E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKCMP6E` writer - BRK COMP6 enable"]
pub struct BKCMP6E_W<'a> {
    w: &'a mut W,
}
impl<'a> BKCMP6E_W<'a> {
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
#[doc = "Field `BKCMP5E` reader - BRK COMP5 enable"]
pub struct BKCMP5E_R(crate::FieldReader<bool, bool>);
impl BKCMP5E_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKCMP5E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKCMP5E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKCMP5E` writer - BRK COMP5 enable"]
pub struct BKCMP5E_W<'a> {
    w: &'a mut W,
}
impl<'a> BKCMP5E_W<'a> {
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
#[doc = "Field `BKCMP4E` reader - BRK COMP4 enable"]
pub struct BKCMP4E_R(crate::FieldReader<bool, bool>);
impl BKCMP4E_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKCMP4E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKCMP4E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKCMP4E` writer - BRK COMP4 enable"]
pub struct BKCMP4E_W<'a> {
    w: &'a mut W,
}
impl<'a> BKCMP4E_W<'a> {
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
#[doc = "Field `BKCMP3E` reader - BRK COMP3 enable"]
pub struct BKCMP3E_R(crate::FieldReader<bool, bool>);
impl BKCMP3E_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKCMP3E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKCMP3E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKCMP3E` writer - BRK COMP3 enable"]
pub struct BKCMP3E_W<'a> {
    w: &'a mut W,
}
impl<'a> BKCMP3E_W<'a> {
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
#[doc = "Field `BKCMP2E` reader - BRK COMP2 enable"]
pub struct BKCMP2E_R(crate::FieldReader<bool, bool>);
impl BKCMP2E_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKCMP2E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKCMP2E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKCMP2E` writer - BRK COMP2 enable"]
pub struct BKCMP2E_W<'a> {
    w: &'a mut W,
}
impl<'a> BKCMP2E_W<'a> {
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
#[doc = "Field `BKCMP1E` reader - BRK COMP1 enable"]
pub struct BKCMP1E_R(crate::FieldReader<bool, bool>);
impl BKCMP1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKCMP1E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKCMP1E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKCMP1E` writer - BRK COMP1 enable"]
pub struct BKCMP1E_W<'a> {
    w: &'a mut W,
}
impl<'a> BKCMP1E_W<'a> {
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
    #[doc = "Bits 14:17 - ETR source selection"]
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - BRK COMP4 input polarity"]
    #[inline(always)]
    pub fn bkcmp4p(&self) -> BKCMP4P_R {
        BKCMP4P_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - BRK COMP3 input polarity"]
    #[inline(always)]
    pub fn bkcmp3p(&self) -> BKCMP3P_R {
        BKCMP3P_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BRK COMP2 input polarity"]
    #[inline(always)]
    pub fn bkcmp2p(&self) -> BKCMP2P_R {
        BKCMP2P_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity"]
    #[inline(always)]
    pub fn bkcmp1p(&self) -> BKCMP1P_R {
        BKCMP1P_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BRK BKIN input polarity"]
    #[inline(always)]
    pub fn bkinp(&self) -> BKINP_R {
        BKINP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BRK COMP7 enable"]
    #[inline(always)]
    pub fn bkcmp7e(&self) -> BKCMP7E_R {
        BKCMP7E_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BRK COMP6 enable"]
    #[inline(always)]
    pub fn bkcmp6e(&self) -> BKCMP6E_R {
        BKCMP6E_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - BRK COMP5 enable"]
    #[inline(always)]
    pub fn bkcmp5e(&self) -> BKCMP5E_R {
        BKCMP5E_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BRK COMP4 enable"]
    #[inline(always)]
    pub fn bkcmp4e(&self) -> BKCMP4E_R {
        BKCMP4E_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BRK COMP3 enable"]
    #[inline(always)]
    pub fn bkcmp3e(&self) -> BKCMP3E_R {
        BKCMP3E_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRK COMP2 enable"]
    #[inline(always)]
    pub fn bkcmp2e(&self) -> BKCMP2E_R {
        BKCMP2E_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - BRK COMP1 enable"]
    #[inline(always)]
    pub fn bkcmp1e(&self) -> BKCMP1E_R {
        BKCMP1E_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 14:17 - ETR source selection"]
    #[inline(always)]
    pub fn etrsel(&mut self) -> ETRSEL_W {
        ETRSEL_W { w: self }
    }
    #[doc = "Bit 13 - BRK COMP4 input polarity"]
    #[inline(always)]
    pub fn bkcmp4p(&mut self) -> BKCMP4P_W {
        BKCMP4P_W { w: self }
    }
    #[doc = "Bit 12 - BRK COMP3 input polarity"]
    #[inline(always)]
    pub fn bkcmp3p(&mut self) -> BKCMP3P_W {
        BKCMP3P_W { w: self }
    }
    #[doc = "Bit 11 - BRK COMP2 input polarity"]
    #[inline(always)]
    pub fn bkcmp2p(&mut self) -> BKCMP2P_W {
        BKCMP2P_W { w: self }
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity"]
    #[inline(always)]
    pub fn bkcmp1p(&mut self) -> BKCMP1P_W {
        BKCMP1P_W { w: self }
    }
    #[doc = "Bit 9 - BRK BKIN input polarity"]
    #[inline(always)]
    pub fn bkinp(&mut self) -> BKINP_W {
        BKINP_W { w: self }
    }
    #[doc = "Bit 7 - BRK COMP7 enable"]
    #[inline(always)]
    pub fn bkcmp7e(&mut self) -> BKCMP7E_W {
        BKCMP7E_W { w: self }
    }
    #[doc = "Bit 6 - BRK COMP6 enable"]
    #[inline(always)]
    pub fn bkcmp6e(&mut self) -> BKCMP6E_W {
        BKCMP6E_W { w: self }
    }
    #[doc = "Bit 5 - BRK COMP5 enable"]
    #[inline(always)]
    pub fn bkcmp5e(&mut self) -> BKCMP5E_W {
        BKCMP5E_W { w: self }
    }
    #[doc = "Bit 4 - BRK COMP4 enable"]
    #[inline(always)]
    pub fn bkcmp4e(&mut self) -> BKCMP4E_W {
        BKCMP4E_W { w: self }
    }
    #[doc = "Bit 3 - BRK COMP3 enable"]
    #[inline(always)]
    pub fn bkcmp3e(&mut self) -> BKCMP3E_W {
        BKCMP3E_W { w: self }
    }
    #[doc = "Bit 2 - BRK COMP2 enable"]
    #[inline(always)]
    pub fn bkcmp2e(&mut self) -> BKCMP2E_W {
        BKCMP2E_W { w: self }
    }
    #[doc = "Bit 1 - BRK COMP1 enable"]
    #[inline(always)]
    pub fn bkcmp1e(&mut self) -> BKCMP1E_W {
        BKCMP1E_W { w: self }
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
#[doc = "TIM alternate function option register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [af1](index.html) module"]
pub struct AF1_SPEC;
impl crate::RegisterSpec for AF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [af1::R](R) reader structure"]
impl crate::Readable for AF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [af1::W](W) writer structure"]
impl crate::Writable for AF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AF1 to value 0"]
impl crate::Resettable for AF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
