#[doc = "Register `RCC_MP_APB3ENCLRR` reader"]
pub struct R(crate::R<RCC_MP_APB3ENCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_APB3ENCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_APB3ENCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_APB3ENCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_APB3ENCLRR` writer"]
pub struct W(crate::W<RCC_MP_APB3ENCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_APB3ENCLRR_SPEC>;
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
impl From<crate::W<RCC_MP_APB3ENCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_APB3ENCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPTIM2EN` reader - LPTIM2EN"]
pub struct LPTIM2EN_R(crate::FieldReader<bool, bool>);
impl LPTIM2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM2EN` writer - LPTIM2EN"]
pub struct LPTIM2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2EN_W<'a> {
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
#[doc = "Field `LPTIM3EN` reader - LPTIM3EN"]
pub struct LPTIM3EN_R(crate::FieldReader<bool, bool>);
impl LPTIM3EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM3EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM3EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM3EN` writer - LPTIM3EN"]
pub struct LPTIM3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3EN_W<'a> {
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
#[doc = "Field `LPTIM4EN` reader - LPTIM4EN"]
pub struct LPTIM4EN_R(crate::FieldReader<bool, bool>);
impl LPTIM4EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM4EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM4EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM4EN` writer - LPTIM4EN"]
pub struct LPTIM4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM4EN_W<'a> {
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
#[doc = "Field `LPTIM5EN` reader - LPTIM5EN"]
pub struct LPTIM5EN_R(crate::FieldReader<bool, bool>);
impl LPTIM5EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM5EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM5EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM5EN` writer - LPTIM5EN"]
pub struct LPTIM5EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM5EN_W<'a> {
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
#[doc = "Field `SAI4EN` reader - SAI4EN"]
pub struct SAI4EN_R(crate::FieldReader<bool, bool>);
impl SAI4EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI4EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI4EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI4EN` writer - SAI4EN"]
pub struct SAI4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI4EN_W<'a> {
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
#[doc = "Field `SYSCFGEN` reader - SYSCFGEN"]
pub struct SYSCFGEN_R(crate::FieldReader<bool, bool>);
impl SYSCFGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSCFGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSCFGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSCFGEN` writer - SYSCFGEN"]
pub struct SYSCFGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGEN_W<'a> {
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
#[doc = "Field `VREFEN` reader - VREFEN"]
pub struct VREFEN_R(crate::FieldReader<bool, bool>);
impl VREFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREFEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREFEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREFEN` writer - VREFEN"]
pub struct VREFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFEN_W<'a> {
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
#[doc = "Field `DTSEN` reader - DTSEN"]
pub struct DTSEN_R(crate::FieldReader<bool, bool>);
impl DTSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTSEN` writer - DTSEN"]
pub struct DTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTSEN_W<'a> {
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
#[doc = "Field `HDPEN` reader - HDPEN"]
pub struct HDPEN_R(crate::FieldReader<bool, bool>);
impl HDPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HDPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDPEN` writer - HDPEN"]
pub struct HDPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HDPEN_W<'a> {
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
    #[doc = "Bit 0 - LPTIM2EN"]
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LPTIM3EN"]
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LPTIM4EN"]
    #[inline(always)]
    pub fn lptim4en(&self) -> LPTIM4EN_R {
        LPTIM4EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LPTIM5EN"]
    #[inline(always)]
    pub fn lptim5en(&self) -> LPTIM5EN_R {
        LPTIM5EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SAI4EN"]
    #[inline(always)]
    pub fn sai4en(&self) -> SAI4EN_R {
        SAI4EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SYSCFGEN"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - VREFEN"]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DTSEN"]
    #[inline(always)]
    pub fn dtsen(&self) -> DTSEN_R {
        DTSEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - HDPEN"]
    #[inline(always)]
    pub fn hdpen(&self) -> HDPEN_R {
        HDPEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPTIM2EN"]
    #[inline(always)]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W {
        LPTIM2EN_W { w: self }
    }
    #[doc = "Bit 1 - LPTIM3EN"]
    #[inline(always)]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W {
        LPTIM3EN_W { w: self }
    }
    #[doc = "Bit 2 - LPTIM4EN"]
    #[inline(always)]
    pub fn lptim4en(&mut self) -> LPTIM4EN_W {
        LPTIM4EN_W { w: self }
    }
    #[doc = "Bit 3 - LPTIM5EN"]
    #[inline(always)]
    pub fn lptim5en(&mut self) -> LPTIM5EN_W {
        LPTIM5EN_W { w: self }
    }
    #[doc = "Bit 8 - SAI4EN"]
    #[inline(always)]
    pub fn sai4en(&mut self) -> SAI4EN_W {
        SAI4EN_W { w: self }
    }
    #[doc = "Bit 11 - SYSCFGEN"]
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W {
        SYSCFGEN_W { w: self }
    }
    #[doc = "Bit 13 - VREFEN"]
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W {
        VREFEN_W { w: self }
    }
    #[doc = "Bit 16 - DTSEN"]
    #[inline(always)]
    pub fn dtsen(&mut self) -> DTSEN_W {
        DTSEN_W { w: self }
    }
    #[doc = "Bit 20 - HDPEN"]
    #[inline(always)]
    pub fn hdpen(&mut self) -> HDPEN_W {
        HDPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb3enclrr](index.html) module"]
pub struct RCC_MP_APB3ENCLRR_SPEC;
impl crate::RegisterSpec for RCC_MP_APB3ENCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_apb3enclrr::R](R) reader structure"]
impl crate::Readable for RCC_MP_APB3ENCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb3enclrr::W](W) writer structure"]
impl crate::Writable for RCC_MP_APB3ENCLRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MP_APB3ENCLRR to value 0"]
impl crate::Resettable for RCC_MP_APB3ENCLRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
