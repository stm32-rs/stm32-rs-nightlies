#[doc = "Register `MDIER4` reader"]
pub struct R(crate::R<MDIER4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIER4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIER4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIER4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDIER4` writer"]
pub struct W(crate::W<MDIER4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDIER4_SPEC>;
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
impl From<crate::W<MDIER4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDIER4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MUPDDE` reader - MUPDDE"]
pub struct MUPDDE_R(crate::FieldReader<bool, bool>);
impl MUPDDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUPDDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUPDDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUPDDE` writer - MUPDDE"]
pub struct MUPDDE_W<'a> {
    w: &'a mut W,
}
impl<'a> MUPDDE_W<'a> {
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
#[doc = "Field `SYNCDE` reader - SYNCDE"]
pub struct SYNCDE_R(crate::FieldReader<bool, bool>);
impl SYNCDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNCDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCDE` writer - SYNCDE"]
pub struct SYNCDE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCDE_W<'a> {
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
#[doc = "Field `MREPDE` reader - MREPDE"]
pub struct MREPDE_R(crate::FieldReader<bool, bool>);
impl MREPDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MREPDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MREPDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MREPDE` writer - MREPDE"]
pub struct MREPDE_W<'a> {
    w: &'a mut W,
}
impl<'a> MREPDE_W<'a> {
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
#[doc = "Field `MCMP4DE` reader - MCMP4DE"]
pub struct MCMP4DE_R(crate::FieldReader<bool, bool>);
impl MCMP4DE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCMP4DE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCMP4DE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCMP4DE` writer - MCMP4DE"]
pub struct MCMP4DE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP4DE_W<'a> {
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
#[doc = "Field `MCMP3DE` reader - MCMP3DE"]
pub struct MCMP3DE_R(crate::FieldReader<bool, bool>);
impl MCMP3DE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCMP3DE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCMP3DE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCMP3DE` writer - MCMP3DE"]
pub struct MCMP3DE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP3DE_W<'a> {
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
#[doc = "Field `MCMP2DE` reader - MCMP2DE"]
pub struct MCMP2DE_R(crate::FieldReader<bool, bool>);
impl MCMP2DE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCMP2DE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCMP2DE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCMP2DE` writer - MCMP2DE"]
pub struct MCMP2DE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP2DE_W<'a> {
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
#[doc = "Field `MCMP1DE` reader - MCMP1DE"]
pub struct MCMP1DE_R(crate::FieldReader<bool, bool>);
impl MCMP1DE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCMP1DE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCMP1DE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCMP1DE` writer - MCMP1DE"]
pub struct MCMP1DE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP1DE_W<'a> {
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
#[doc = "Field `MUPDIE` reader - MUPDIE"]
pub struct MUPDIE_R(crate::FieldReader<bool, bool>);
impl MUPDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUPDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUPDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUPDIE` writer - MUPDIE"]
pub struct MUPDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MUPDIE_W<'a> {
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
#[doc = "Field `SYNCIE` reader - SYNCIE"]
pub struct SYNCIE_R(crate::FieldReader<bool, bool>);
impl SYNCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCIE` writer - SYNCIE"]
pub struct SYNCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCIE_W<'a> {
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
#[doc = "Field `MREPIE` reader - MREPIE"]
pub struct MREPIE_R(crate::FieldReader<bool, bool>);
impl MREPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MREPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MREPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MREPIE` writer - MREPIE"]
pub struct MREPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MREPIE_W<'a> {
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
#[doc = "Field `MCMP4IE` reader - MCMP4IE"]
pub struct MCMP4IE_R(crate::FieldReader<bool, bool>);
impl MCMP4IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCMP4IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCMP4IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCMP4IE` writer - MCMP4IE"]
pub struct MCMP4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP4IE_W<'a> {
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
#[doc = "Field `MCMP3IE` reader - MCMP3IE"]
pub struct MCMP3IE_R(crate::FieldReader<bool, bool>);
impl MCMP3IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCMP3IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCMP3IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCMP3IE` writer - MCMP3IE"]
pub struct MCMP3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP3IE_W<'a> {
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
#[doc = "Field `MCMP2IE` reader - MCMP2IE"]
pub struct MCMP2IE_R(crate::FieldReader<bool, bool>);
impl MCMP2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCMP2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCMP2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCMP2IE` writer - MCMP2IE"]
pub struct MCMP2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP2IE_W<'a> {
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
#[doc = "Field `MCMP1IE` reader - MCMP1IE"]
pub struct MCMP1IE_R(crate::FieldReader<bool, bool>);
impl MCMP1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCMP1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCMP1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCMP1IE` writer - MCMP1IE"]
pub struct MCMP1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP1IE_W<'a> {
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
    #[doc = "Bit 22 - MUPDDE"]
    #[inline(always)]
    pub fn mupdde(&self) -> MUPDDE_R {
        MUPDDE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SYNCDE"]
    #[inline(always)]
    pub fn syncde(&self) -> SYNCDE_R {
        SYNCDE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - MREPDE"]
    #[inline(always)]
    pub fn mrepde(&self) -> MREPDE_R {
        MREPDE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - MCMP4DE"]
    #[inline(always)]
    pub fn mcmp4de(&self) -> MCMP4DE_R {
        MCMP4DE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - MCMP3DE"]
    #[inline(always)]
    pub fn mcmp3de(&self) -> MCMP3DE_R {
        MCMP3DE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MCMP2DE"]
    #[inline(always)]
    pub fn mcmp2de(&self) -> MCMP2DE_R {
        MCMP2DE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - MCMP1DE"]
    #[inline(always)]
    pub fn mcmp1de(&self) -> MCMP1DE_R {
        MCMP1DE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MUPDIE"]
    #[inline(always)]
    pub fn mupdie(&self) -> MUPDIE_R {
        MUPDIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SYNCIE"]
    #[inline(always)]
    pub fn syncie(&self) -> SYNCIE_R {
        SYNCIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MREPIE"]
    #[inline(always)]
    pub fn mrepie(&self) -> MREPIE_R {
        MREPIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MCMP4IE"]
    #[inline(always)]
    pub fn mcmp4ie(&self) -> MCMP4IE_R {
        MCMP4IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MCMP3IE"]
    #[inline(always)]
    pub fn mcmp3ie(&self) -> MCMP3IE_R {
        MCMP3IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - MCMP2IE"]
    #[inline(always)]
    pub fn mcmp2ie(&self) -> MCMP2IE_R {
        MCMP2IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - MCMP1IE"]
    #[inline(always)]
    pub fn mcmp1ie(&self) -> MCMP1IE_R {
        MCMP1IE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 22 - MUPDDE"]
    #[inline(always)]
    pub fn mupdde(&mut self) -> MUPDDE_W {
        MUPDDE_W { w: self }
    }
    #[doc = "Bit 21 - SYNCDE"]
    #[inline(always)]
    pub fn syncde(&mut self) -> SYNCDE_W {
        SYNCDE_W { w: self }
    }
    #[doc = "Bit 20 - MREPDE"]
    #[inline(always)]
    pub fn mrepde(&mut self) -> MREPDE_W {
        MREPDE_W { w: self }
    }
    #[doc = "Bit 19 - MCMP4DE"]
    #[inline(always)]
    pub fn mcmp4de(&mut self) -> MCMP4DE_W {
        MCMP4DE_W { w: self }
    }
    #[doc = "Bit 18 - MCMP3DE"]
    #[inline(always)]
    pub fn mcmp3de(&mut self) -> MCMP3DE_W {
        MCMP3DE_W { w: self }
    }
    #[doc = "Bit 17 - MCMP2DE"]
    #[inline(always)]
    pub fn mcmp2de(&mut self) -> MCMP2DE_W {
        MCMP2DE_W { w: self }
    }
    #[doc = "Bit 16 - MCMP1DE"]
    #[inline(always)]
    pub fn mcmp1de(&mut self) -> MCMP1DE_W {
        MCMP1DE_W { w: self }
    }
    #[doc = "Bit 6 - MUPDIE"]
    #[inline(always)]
    pub fn mupdie(&mut self) -> MUPDIE_W {
        MUPDIE_W { w: self }
    }
    #[doc = "Bit 5 - SYNCIE"]
    #[inline(always)]
    pub fn syncie(&mut self) -> SYNCIE_W {
        SYNCIE_W { w: self }
    }
    #[doc = "Bit 4 - MREPIE"]
    #[inline(always)]
    pub fn mrepie(&mut self) -> MREPIE_W {
        MREPIE_W { w: self }
    }
    #[doc = "Bit 3 - MCMP4IE"]
    #[inline(always)]
    pub fn mcmp4ie(&mut self) -> MCMP4IE_W {
        MCMP4IE_W { w: self }
    }
    #[doc = "Bit 2 - MCMP3IE"]
    #[inline(always)]
    pub fn mcmp3ie(&mut self) -> MCMP3IE_W {
        MCMP3IE_W { w: self }
    }
    #[doc = "Bit 1 - MCMP2IE"]
    #[inline(always)]
    pub fn mcmp2ie(&mut self) -> MCMP2IE_W {
        MCMP2IE_W { w: self }
    }
    #[doc = "Bit 0 - MCMP1IE"]
    #[inline(always)]
    pub fn mcmp1ie(&mut self) -> MCMP1IE_W {
        MCMP1IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIER4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdier4](index.html) module"]
pub struct MDIER4_SPEC;
impl crate::RegisterSpec for MDIER4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdier4::R](R) reader structure"]
impl crate::Readable for MDIER4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdier4::W](W) writer structure"]
impl crate::Writable for MDIER4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDIER4 to value 0"]
impl crate::Resettable for MDIER4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
