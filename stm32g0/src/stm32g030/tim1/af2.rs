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
#[doc = "Field `BK2INE` reader - BRK2 BKIN input enable"]
pub struct BK2INE_R(crate::FieldReader<bool, bool>);
impl BK2INE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2INE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BK2INE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2INE` writer - BRK2 BKIN input enable"]
pub struct BK2INE_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2INE_W<'a> {
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
#[doc = "Field `BK2DFBK0E` reader - BRK2 DFSDM_BREAK0 enable"]
pub struct BK2DFBK0E_R(crate::FieldReader<bool, bool>);
impl BK2DFBK0E_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2DFBK0E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BK2DFBK0E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2DFBK0E` writer - BRK2 DFSDM_BREAK0 enable"]
pub struct BK2DFBK0E_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2DFBK0E_W<'a> {
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
impl R {
    #[doc = "Bit 0 - BRK2 BKIN input enable"]
    #[inline(always)]
    pub fn bk2ine(&self) -> BK2INE_R {
        BK2INE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable"]
    #[inline(always)]
    pub fn bk2cmp1e(&self) -> BK2CMP1E_R {
        BK2CMP1E_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable"]
    #[inline(always)]
    pub fn bk2cmp2e(&self) -> BK2CMP2E_R {
        BK2CMP2E_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BRK2 DFSDM_BREAK0 enable"]
    #[inline(always)]
    pub fn bk2dfbk0e(&self) -> BK2DFBK0E_R {
        BK2DFBK0E_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BRK2 BKIN input polarity"]
    #[inline(always)]
    pub fn bk2inp(&self) -> BK2INP_R {
        BK2INP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity"]
    #[inline(always)]
    pub fn bk2cmp1p(&self) -> BK2CMP1P_R {
        BK2CMP1P_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity"]
    #[inline(always)]
    pub fn bk2cmp2p(&self) -> BK2CMP2P_R {
        BK2CMP2P_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRK2 BKIN input enable"]
    #[inline(always)]
    pub fn bk2ine(&mut self) -> BK2INE_W {
        BK2INE_W { w: self }
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable"]
    #[inline(always)]
    pub fn bk2cmp1e(&mut self) -> BK2CMP1E_W {
        BK2CMP1E_W { w: self }
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable"]
    #[inline(always)]
    pub fn bk2cmp2e(&mut self) -> BK2CMP2E_W {
        BK2CMP2E_W { w: self }
    }
    #[doc = "Bit 8 - BRK2 DFSDM_BREAK0 enable"]
    #[inline(always)]
    pub fn bk2dfbk0e(&mut self) -> BK2DFBK0E_W {
        BK2DFBK0E_W { w: self }
    }
    #[doc = "Bit 9 - BRK2 BKIN input polarity"]
    #[inline(always)]
    pub fn bk2inp(&mut self) -> BK2INP_W {
        BK2INP_W { w: self }
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity"]
    #[inline(always)]
    pub fn bk2cmp1p(&mut self) -> BK2CMP1P_W {
        BK2CMP1P_W { w: self }
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity"]
    #[inline(always)]
    pub fn bk2cmp2p(&mut self) -> BK2CMP2P_W {
        BK2CMP2P_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [af2](index.html) module"]
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
#[doc = "`reset()` method sets AF2 to value 0x01"]
impl crate::Resettable for AF2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
