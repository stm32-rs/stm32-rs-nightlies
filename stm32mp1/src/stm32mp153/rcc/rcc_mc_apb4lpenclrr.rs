#[doc = "Register `RCC_MC_APB4LPENCLRR` reader"]
pub struct R(crate::R<RCC_MC_APB4LPENCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_APB4LPENCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_APB4LPENCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_APB4LPENCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MC_APB4LPENCLRR` writer"]
pub struct W(crate::W<RCC_MC_APB4LPENCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_APB4LPENCLRR_SPEC>;
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
impl From<crate::W<RCC_MC_APB4LPENCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_APB4LPENCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LTDCLPEN` reader - LTDCLPEN"]
pub struct LTDCLPEN_R(crate::FieldReader<bool, bool>);
impl LTDCLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LTDCLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LTDCLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LTDCLPEN` writer - LTDCLPEN"]
pub struct LTDCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LTDCLPEN_W<'a> {
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
#[doc = "Field `DSILPEN` reader - DSILPEN"]
pub struct DSILPEN_R(crate::FieldReader<bool, bool>);
impl DSILPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSILPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSILPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSILPEN` writer - DSILPEN"]
pub struct DSILPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSILPEN_W<'a> {
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
#[doc = "Field `DDRPERFMLPEN` reader - DDRPERFMLPEN"]
pub struct DDRPERFMLPEN_R(crate::FieldReader<bool, bool>);
impl DDRPERFMLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRPERFMLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRPERFMLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRPERFMLPEN` writer - DDRPERFMLPEN"]
pub struct DDRPERFMLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRPERFMLPEN_W<'a> {
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
#[doc = "Field `USBPHYLPEN` reader - USBPHYLPEN"]
pub struct USBPHYLPEN_R(crate::FieldReader<bool, bool>);
impl USBPHYLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBPHYLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBPHYLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBPHYLPEN` writer - USBPHYLPEN"]
pub struct USBPHYLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPHYLPEN_W<'a> {
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
#[doc = "Field `STGENROLPEN` reader - STGENROLPEN"]
pub struct STGENROLPEN_R(crate::FieldReader<bool, bool>);
impl STGENROLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        STGENROLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STGENROLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STGENROLPEN` writer - STGENROLPEN"]
pub struct STGENROLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STGENROLPEN_W<'a> {
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
#[doc = "Field `STGENROSTPEN` reader - STGENROSTPEN"]
pub struct STGENROSTPEN_R(crate::FieldReader<bool, bool>);
impl STGENROSTPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        STGENROSTPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STGENROSTPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STGENROSTPEN` writer - STGENROSTPEN"]
pub struct STGENROSTPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STGENROSTPEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - LTDCLPEN"]
    #[inline(always)]
    pub fn ltdclpen(&self) -> LTDCLPEN_R {
        LTDCLPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - DSILPEN"]
    #[inline(always)]
    pub fn dsilpen(&self) -> DSILPEN_R {
        DSILPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DDRPERFMLPEN"]
    #[inline(always)]
    pub fn ddrperfmlpen(&self) -> DDRPERFMLPEN_R {
        DDRPERFMLPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USBPHYLPEN"]
    #[inline(always)]
    pub fn usbphylpen(&self) -> USBPHYLPEN_R {
        USBPHYLPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - STGENROLPEN"]
    #[inline(always)]
    pub fn stgenrolpen(&self) -> STGENROLPEN_R {
        STGENROLPEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - STGENROSTPEN"]
    #[inline(always)]
    pub fn stgenrostpen(&self) -> STGENROSTPEN_R {
        STGENROSTPEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LTDCLPEN"]
    #[inline(always)]
    pub fn ltdclpen(&mut self) -> LTDCLPEN_W {
        LTDCLPEN_W { w: self }
    }
    #[doc = "Bit 4 - DSILPEN"]
    #[inline(always)]
    pub fn dsilpen(&mut self) -> DSILPEN_W {
        DSILPEN_W { w: self }
    }
    #[doc = "Bit 8 - DDRPERFMLPEN"]
    #[inline(always)]
    pub fn ddrperfmlpen(&mut self) -> DDRPERFMLPEN_W {
        DDRPERFMLPEN_W { w: self }
    }
    #[doc = "Bit 16 - USBPHYLPEN"]
    #[inline(always)]
    pub fn usbphylpen(&mut self) -> USBPHYLPEN_W {
        USBPHYLPEN_W { w: self }
    }
    #[doc = "Bit 20 - STGENROLPEN"]
    #[inline(always)]
    pub fn stgenrolpen(&mut self) -> STGENROLPEN_W {
        STGENROLPEN_W { w: self }
    }
    #[doc = "Bit 21 - STGENROSTPEN"]
    #[inline(always)]
    pub fn stgenrostpen(&mut self) -> STGENROSTPEN_W {
        STGENROSTPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb4lpenclrr](index.html) module"]
pub struct RCC_MC_APB4LPENCLRR_SPEC;
impl crate::RegisterSpec for RCC_MC_APB4LPENCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mc_apb4lpenclrr::R](R) reader structure"]
impl crate::Readable for RCC_MC_APB4LPENCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb4lpenclrr::W](W) writer structure"]
impl crate::Writable for RCC_MC_APB4LPENCLRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MC_APB4LPENCLRR to value 0x0011_0111"]
impl crate::Resettable for RCC_MC_APB4LPENCLRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0011_0111
    }
}
