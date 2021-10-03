#[doc = "Register `RCC_MC_APB3LPENCLRR` reader"]
pub struct R(crate::R<RCC_MC_APB3LPENCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_APB3LPENCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_APB3LPENCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_APB3LPENCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MC_APB3LPENCLRR` writer"]
pub struct W(crate::W<RCC_MC_APB3LPENCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_APB3LPENCLRR_SPEC>;
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
impl From<crate::W<RCC_MC_APB3LPENCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_APB3LPENCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPTIM2LPEN` reader - LPTIM2LPEN"]
pub struct LPTIM2LPEN_R(crate::FieldReader<bool, bool>);
impl LPTIM2LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM2LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM2LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM2LPEN` writer - LPTIM2LPEN"]
pub struct LPTIM2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2LPEN_W<'a> {
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
#[doc = "Field `LPTIM3LPEN` reader - LPTIM3LPEN"]
pub struct LPTIM3LPEN_R(crate::FieldReader<bool, bool>);
impl LPTIM3LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM3LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM3LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM3LPEN` writer - LPTIM3LPEN"]
pub struct LPTIM3LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3LPEN_W<'a> {
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
#[doc = "Field `LPTIM4LPEN` reader - LPTIM4LPEN"]
pub struct LPTIM4LPEN_R(crate::FieldReader<bool, bool>);
impl LPTIM4LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM4LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM4LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM4LPEN` writer - LPTIM4LPEN"]
pub struct LPTIM4LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM4LPEN_W<'a> {
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
#[doc = "Field `LPTIM5LPEN` reader - LPTIM5LPEN"]
pub struct LPTIM5LPEN_R(crate::FieldReader<bool, bool>);
impl LPTIM5LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM5LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM5LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM5LPEN` writer - LPTIM5LPEN"]
pub struct LPTIM5LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM5LPEN_W<'a> {
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
#[doc = "Field `SAI4LPEN` reader - SAI4LPEN"]
pub struct SAI4LPEN_R(crate::FieldReader<bool, bool>);
impl SAI4LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI4LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI4LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI4LPEN` writer - SAI4LPEN"]
pub struct SAI4LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI4LPEN_W<'a> {
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
#[doc = "Field `SYSCFGLPEN` reader - SYSCFGLPEN"]
pub struct SYSCFGLPEN_R(crate::FieldReader<bool, bool>);
impl SYSCFGLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSCFGLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSCFGLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSCFGLPEN` writer - SYSCFGLPEN"]
pub struct SYSCFGLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGLPEN_W<'a> {
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
#[doc = "Field `VREFLPEN` reader - VREFLPEN"]
pub struct VREFLPEN_R(crate::FieldReader<bool, bool>);
impl VREFLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREFLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREFLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREFLPEN` writer - VREFLPEN"]
pub struct VREFLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFLPEN_W<'a> {
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
#[doc = "Field `DTSLPEN` reader - DTSLPEN"]
pub struct DTSLPEN_R(crate::FieldReader<bool, bool>);
impl DTSLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTSLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTSLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTSLPEN` writer - DTSLPEN"]
pub struct DTSLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTSLPEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - LPTIM2LPEN"]
    #[inline(always)]
    pub fn lptim2lpen(&self) -> LPTIM2LPEN_R {
        LPTIM2LPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LPTIM3LPEN"]
    #[inline(always)]
    pub fn lptim3lpen(&self) -> LPTIM3LPEN_R {
        LPTIM3LPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LPTIM4LPEN"]
    #[inline(always)]
    pub fn lptim4lpen(&self) -> LPTIM4LPEN_R {
        LPTIM4LPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LPTIM5LPEN"]
    #[inline(always)]
    pub fn lptim5lpen(&self) -> LPTIM5LPEN_R {
        LPTIM5LPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SAI4LPEN"]
    #[inline(always)]
    pub fn sai4lpen(&self) -> SAI4LPEN_R {
        SAI4LPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SYSCFGLPEN"]
    #[inline(always)]
    pub fn syscfglpen(&self) -> SYSCFGLPEN_R {
        SYSCFGLPEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - VREFLPEN"]
    #[inline(always)]
    pub fn vreflpen(&self) -> VREFLPEN_R {
        VREFLPEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DTSLPEN"]
    #[inline(always)]
    pub fn dtslpen(&self) -> DTSLPEN_R {
        DTSLPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPTIM2LPEN"]
    #[inline(always)]
    pub fn lptim2lpen(&mut self) -> LPTIM2LPEN_W {
        LPTIM2LPEN_W { w: self }
    }
    #[doc = "Bit 1 - LPTIM3LPEN"]
    #[inline(always)]
    pub fn lptim3lpen(&mut self) -> LPTIM3LPEN_W {
        LPTIM3LPEN_W { w: self }
    }
    #[doc = "Bit 2 - LPTIM4LPEN"]
    #[inline(always)]
    pub fn lptim4lpen(&mut self) -> LPTIM4LPEN_W {
        LPTIM4LPEN_W { w: self }
    }
    #[doc = "Bit 3 - LPTIM5LPEN"]
    #[inline(always)]
    pub fn lptim5lpen(&mut self) -> LPTIM5LPEN_W {
        LPTIM5LPEN_W { w: self }
    }
    #[doc = "Bit 8 - SAI4LPEN"]
    #[inline(always)]
    pub fn sai4lpen(&mut self) -> SAI4LPEN_W {
        SAI4LPEN_W { w: self }
    }
    #[doc = "Bit 11 - SYSCFGLPEN"]
    #[inline(always)]
    pub fn syscfglpen(&mut self) -> SYSCFGLPEN_W {
        SYSCFGLPEN_W { w: self }
    }
    #[doc = "Bit 13 - VREFLPEN"]
    #[inline(always)]
    pub fn vreflpen(&mut self) -> VREFLPEN_W {
        VREFLPEN_W { w: self }
    }
    #[doc = "Bit 16 - DTSLPEN"]
    #[inline(always)]
    pub fn dtslpen(&mut self) -> DTSLPEN_W {
        DTSLPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb3lpenclrr](index.html) module"]
pub struct RCC_MC_APB3LPENCLRR_SPEC;
impl crate::RegisterSpec for RCC_MC_APB3LPENCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mc_apb3lpenclrr::R](R) reader structure"]
impl crate::Readable for RCC_MC_APB3LPENCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb3lpenclrr::W](W) writer structure"]
impl crate::Writable for RCC_MC_APB3LPENCLRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MC_APB3LPENCLRR to value 0x0003_290f"]
impl crate::Resettable for RCC_MC_APB3LPENCLRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_290f
    }
}
