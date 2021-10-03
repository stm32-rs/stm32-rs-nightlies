#[doc = "Register `RCC_PLL4CR` reader"]
pub struct R(crate::R<RCC_PLL4CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_PLL4CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_PLL4CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_PLL4CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_PLL4CR` writer"]
pub struct W(crate::W<RCC_PLL4CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_PLL4CR_SPEC>;
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
impl From<crate::W<RCC_PLL4CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_PLL4CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLON` reader - PLLON"]
pub struct PLLON_R(crate::FieldReader<bool, bool>);
impl PLLON_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLON` writer - PLLON"]
pub struct PLLON_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLON_W<'a> {
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
#[doc = "Field `PLL4RDY` reader - PLL4RDY"]
pub struct PLL4RDY_R(crate::FieldReader<bool, bool>);
impl PLL4RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL4RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL4RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSCG_CTRL` reader - SSCG_CTRL"]
pub struct SSCG_CTRL_R(crate::FieldReader<bool, bool>);
impl SSCG_CTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSCG_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSCG_CTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSCG_CTRL` writer - SSCG_CTRL"]
pub struct SSCG_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SSCG_CTRL_W<'a> {
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
#[doc = "Field `DIVPEN` reader - DIVPEN"]
pub struct DIVPEN_R(crate::FieldReader<bool, bool>);
impl DIVPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIVPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVPEN` writer - DIVPEN"]
pub struct DIVPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVPEN_W<'a> {
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
#[doc = "Field `DIVQEN` reader - DIVQEN"]
pub struct DIVQEN_R(crate::FieldReader<bool, bool>);
impl DIVQEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIVQEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVQEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVQEN` writer - DIVQEN"]
pub struct DIVQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVQEN_W<'a> {
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
#[doc = "Field `DIVREN` reader - DIVREN"]
pub struct DIVREN_R(crate::FieldReader<bool, bool>);
impl DIVREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIVREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVREN` writer - DIVREN"]
pub struct DIVREN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVREN_W<'a> {
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
    #[doc = "Bit 0 - PLLON"]
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PLL4RDY"]
    #[inline(always)]
    pub fn pll4rdy(&self) -> PLL4RDY_R {
        PLL4RDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SSCG_CTRL"]
    #[inline(always)]
    pub fn sscg_ctrl(&self) -> SSCG_CTRL_R {
        SSCG_CTRL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DIVPEN"]
    #[inline(always)]
    pub fn divpen(&self) -> DIVPEN_R {
        DIVPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DIVQEN"]
    #[inline(always)]
    pub fn divqen(&self) -> DIVQEN_R {
        DIVQEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DIVREN"]
    #[inline(always)]
    pub fn divren(&self) -> DIVREN_R {
        DIVREN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLLON"]
    #[inline(always)]
    pub fn pllon(&mut self) -> PLLON_W {
        PLLON_W { w: self }
    }
    #[doc = "Bit 2 - SSCG_CTRL"]
    #[inline(always)]
    pub fn sscg_ctrl(&mut self) -> SSCG_CTRL_W {
        SSCG_CTRL_W { w: self }
    }
    #[doc = "Bit 4 - DIVPEN"]
    #[inline(always)]
    pub fn divpen(&mut self) -> DIVPEN_W {
        DIVPEN_W { w: self }
    }
    #[doc = "Bit 5 - DIVQEN"]
    #[inline(always)]
    pub fn divqen(&mut self) -> DIVQEN_W {
        DIVQEN_W { w: self }
    }
    #[doc = "Bit 6 - DIVREN"]
    #[inline(always)]
    pub fn divren(&mut self) -> DIVREN_W {
        DIVREN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the PLL4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll4cr](index.html) module"]
pub struct RCC_PLL4CR_SPEC;
impl crate::RegisterSpec for RCC_PLL4CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_pll4cr::R](R) reader structure"]
impl crate::Readable for RCC_PLL4CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_pll4cr::W](W) writer structure"]
impl crate::Writable for RCC_PLL4CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_PLL4CR to value 0"]
impl crate::Resettable for RCC_PLL4CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
