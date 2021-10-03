#[doc = "Register `SAI_AIM` reader"]
pub struct R(crate::R<SAI_AIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_AIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_AIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_AIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAI_AIM` writer"]
pub struct W(crate::W<SAI_AIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_AIM_SPEC>;
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
impl From<crate::W<SAI_AIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_AIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVRUDRIE` reader - OVRUDRIE"]
pub struct OVRUDRIE_R(crate::FieldReader<bool, bool>);
impl OVRUDRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRUDRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRUDRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRUDRIE` writer - OVRUDRIE"]
pub struct OVRUDRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRUDRIE_W<'a> {
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
#[doc = "Field `MUTEDETIE` reader - MUTEDETIE"]
pub struct MUTEDETIE_R(crate::FieldReader<bool, bool>);
impl MUTEDETIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUTEDETIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUTEDETIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUTEDETIE` writer - MUTEDETIE"]
pub struct MUTEDETIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTEDETIE_W<'a> {
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
#[doc = "Field `WCKCFGIE` reader - WCKCFGIE"]
pub struct WCKCFGIE_R(crate::FieldReader<bool, bool>);
impl WCKCFGIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WCKCFGIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WCKCFGIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WCKCFGIE` writer - WCKCFGIE"]
pub struct WCKCFGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WCKCFGIE_W<'a> {
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
#[doc = "Field `FREQIE` reader - FREQIE"]
pub struct FREQIE_R(crate::FieldReader<bool, bool>);
impl FREQIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FREQIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREQIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQIE` writer - FREQIE"]
pub struct FREQIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQIE_W<'a> {
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
#[doc = "Field `CNRDYIE` reader - CNRDYIE"]
pub struct CNRDYIE_R(crate::FieldReader<bool, bool>);
impl CNRDYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNRDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNRDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNRDYIE` writer - CNRDYIE"]
pub struct CNRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CNRDYIE_W<'a> {
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
#[doc = "Field `AFSDETIE` reader - AFSDETIE"]
pub struct AFSDETIE_R(crate::FieldReader<bool, bool>);
impl AFSDETIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AFSDETIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFSDETIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFSDETIE` writer - AFSDETIE"]
pub struct AFSDETIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSDETIE_W<'a> {
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
#[doc = "Field `LFSDETIE` reader - LFSDETIE"]
pub struct LFSDETIE_R(crate::FieldReader<bool, bool>);
impl LFSDETIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LFSDETIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFSDETIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFSDETIE` writer - LFSDETIE"]
pub struct LFSDETIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LFSDETIE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - OVRUDRIE"]
    #[inline(always)]
    pub fn ovrudrie(&self) -> OVRUDRIE_R {
        OVRUDRIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MUTEDETIE"]
    #[inline(always)]
    pub fn mutedetie(&self) -> MUTEDETIE_R {
        MUTEDETIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WCKCFGIE"]
    #[inline(always)]
    pub fn wckcfgie(&self) -> WCKCFGIE_R {
        WCKCFGIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FREQIE"]
    #[inline(always)]
    pub fn freqie(&self) -> FREQIE_R {
        FREQIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CNRDYIE"]
    #[inline(always)]
    pub fn cnrdyie(&self) -> CNRDYIE_R {
        CNRDYIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AFSDETIE"]
    #[inline(always)]
    pub fn afsdetie(&self) -> AFSDETIE_R {
        AFSDETIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LFSDETIE"]
    #[inline(always)]
    pub fn lfsdetie(&self) -> LFSDETIE_R {
        LFSDETIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OVRUDRIE"]
    #[inline(always)]
    pub fn ovrudrie(&mut self) -> OVRUDRIE_W {
        OVRUDRIE_W { w: self }
    }
    #[doc = "Bit 1 - MUTEDETIE"]
    #[inline(always)]
    pub fn mutedetie(&mut self) -> MUTEDETIE_W {
        MUTEDETIE_W { w: self }
    }
    #[doc = "Bit 2 - WCKCFGIE"]
    #[inline(always)]
    pub fn wckcfgie(&mut self) -> WCKCFGIE_W {
        WCKCFGIE_W { w: self }
    }
    #[doc = "Bit 3 - FREQIE"]
    #[inline(always)]
    pub fn freqie(&mut self) -> FREQIE_W {
        FREQIE_W { w: self }
    }
    #[doc = "Bit 4 - CNRDYIE"]
    #[inline(always)]
    pub fn cnrdyie(&mut self) -> CNRDYIE_W {
        CNRDYIE_W { w: self }
    }
    #[doc = "Bit 5 - AFSDETIE"]
    #[inline(always)]
    pub fn afsdetie(&mut self) -> AFSDETIE_W {
        AFSDETIE_W { w: self }
    }
    #[doc = "Bit 6 - LFSDETIE"]
    #[inline(always)]
    pub fn lfsdetie(&mut self) -> LFSDETIE_W {
        LFSDETIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_aim](index.html) module"]
pub struct SAI_AIM_SPEC;
impl crate::RegisterSpec for SAI_AIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sai_aim::R](R) reader structure"]
impl crate::Readable for SAI_AIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sai_aim::W](W) writer structure"]
impl crate::Writable for SAI_AIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAI_AIM to value 0"]
impl crate::Resettable for SAI_AIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
