#[doc = "Register `RCC_APB3RSTSETR` reader"]
pub struct R(crate::R<RCC_APB3RSTSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB3RSTSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB3RSTSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB3RSTSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_APB3RSTSETR` writer"]
pub struct W(crate::W<RCC_APB3RSTSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB3RSTSETR_SPEC>;
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
impl From<crate::W<RCC_APB3RSTSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB3RSTSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPTIM2RST` reader - LPTIM2RST"]
pub struct LPTIM2RST_R(crate::FieldReader<bool, bool>);
impl LPTIM2RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM2RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM2RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM2RST` writer - LPTIM2RST"]
pub struct LPTIM2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2RST_W<'a> {
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
#[doc = "Field `LPTIM3RST` reader - LPTIM3RST"]
pub struct LPTIM3RST_R(crate::FieldReader<bool, bool>);
impl LPTIM3RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM3RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM3RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM3RST` writer - LPTIM3RST"]
pub struct LPTIM3RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3RST_W<'a> {
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
#[doc = "Field `LPTIM4RST` reader - LPTIM4RST"]
pub struct LPTIM4RST_R(crate::FieldReader<bool, bool>);
impl LPTIM4RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM4RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM4RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM4RST` writer - LPTIM4RST"]
pub struct LPTIM4RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM4RST_W<'a> {
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
#[doc = "Field `LPTIM5RST` reader - LPTIM5RST"]
pub struct LPTIM5RST_R(crate::FieldReader<bool, bool>);
impl LPTIM5RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM5RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM5RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM5RST` writer - LPTIM5RST"]
pub struct LPTIM5RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM5RST_W<'a> {
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
#[doc = "Field `SAI4RST` reader - SAI4RST"]
pub struct SAI4RST_R(crate::FieldReader<bool, bool>);
impl SAI4RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI4RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI4RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI4RST` writer - SAI4RST"]
pub struct SAI4RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI4RST_W<'a> {
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
#[doc = "Field `SYSCFGRST` reader - SYSCFGRST"]
pub struct SYSCFGRST_R(crate::FieldReader<bool, bool>);
impl SYSCFGRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSCFGRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSCFGRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSCFGRST` writer - SYSCFGRST"]
pub struct SYSCFGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGRST_W<'a> {
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
#[doc = "Field `VREFRST` reader - VREFRST"]
pub struct VREFRST_R(crate::FieldReader<bool, bool>);
impl VREFRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREFRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREFRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREFRST` writer - VREFRST"]
pub struct VREFRST_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFRST_W<'a> {
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
#[doc = "Field `DTSRST` reader - DTSRST"]
pub struct DTSRST_R(crate::FieldReader<bool, bool>);
impl DTSRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTSRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTSRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTSRST` writer - DTSRST"]
pub struct DTSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DTSRST_W<'a> {
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
    #[doc = "Bit 0 - LPTIM2RST"]
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LPTIM3RST"]
    #[inline(always)]
    pub fn lptim3rst(&self) -> LPTIM3RST_R {
        LPTIM3RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LPTIM4RST"]
    #[inline(always)]
    pub fn lptim4rst(&self) -> LPTIM4RST_R {
        LPTIM4RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LPTIM5RST"]
    #[inline(always)]
    pub fn lptim5rst(&self) -> LPTIM5RST_R {
        LPTIM5RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SAI4RST"]
    #[inline(always)]
    pub fn sai4rst(&self) -> SAI4RST_R {
        SAI4RST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SYSCFGRST"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - VREFRST"]
    #[inline(always)]
    pub fn vrefrst(&self) -> VREFRST_R {
        VREFRST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DTSRST"]
    #[inline(always)]
    pub fn dtsrst(&self) -> DTSRST_R {
        DTSRST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPTIM2RST"]
    #[inline(always)]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W {
        LPTIM2RST_W { w: self }
    }
    #[doc = "Bit 1 - LPTIM3RST"]
    #[inline(always)]
    pub fn lptim3rst(&mut self) -> LPTIM3RST_W {
        LPTIM3RST_W { w: self }
    }
    #[doc = "Bit 2 - LPTIM4RST"]
    #[inline(always)]
    pub fn lptim4rst(&mut self) -> LPTIM4RST_W {
        LPTIM4RST_W { w: self }
    }
    #[doc = "Bit 3 - LPTIM5RST"]
    #[inline(always)]
    pub fn lptim5rst(&mut self) -> LPTIM5RST_W {
        LPTIM5RST_W { w: self }
    }
    #[doc = "Bit 8 - SAI4RST"]
    #[inline(always)]
    pub fn sai4rst(&mut self) -> SAI4RST_W {
        SAI4RST_W { w: self }
    }
    #[doc = "Bit 11 - SYSCFGRST"]
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W {
        SYSCFGRST_W { w: self }
    }
    #[doc = "Bit 13 - VREFRST"]
    #[inline(always)]
    pub fn vrefrst(&mut self) -> VREFRST_W {
        VREFRST_W { w: self }
    }
    #[doc = "Bit 16 - DTSRST"]
    #[inline(always)]
    pub fn dtsrst(&mut self) -> DTSRST_W {
        DTSRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to activate the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb3rstsetr](index.html) module"]
pub struct RCC_APB3RSTSETR_SPEC;
impl crate::RegisterSpec for RCC_APB3RSTSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_apb3rstsetr::R](R) reader structure"]
impl crate::Readable for RCC_APB3RSTSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_apb3rstsetr::W](W) writer structure"]
impl crate::Writable for RCC_APB3RSTSETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_APB3RSTSETR to value 0"]
impl crate::Resettable for RCC_APB3RSTSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
