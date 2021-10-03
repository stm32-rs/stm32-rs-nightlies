#[doc = "Register `PLLSYSCFGR` reader"]
pub struct R(crate::R<PLLSYSCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLSYSCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLSYSCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLSYSCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLSYSCFGR` writer"]
pub struct W(crate::W<PLLSYSCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLSYSCFGR_SPEC>;
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
impl From<crate::W<PLLSYSCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLSYSCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLSRC` reader - PLL input clock source"]
pub struct PLLSRC_R(crate::FieldReader<u8, u8>);
impl PLLSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSRC` writer - PLL input clock source"]
pub struct PLLSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `PLLM` reader - Division factor M of the PLL input clock divider"]
pub struct PLLM_R(crate::FieldReader<u8, u8>);
impl PLLM_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLM` writer - Division factor M of the PLL input clock divider"]
pub struct PLLM_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `PLLN` reader - PLL frequency multiplication factor N"]
pub struct PLLN_R(crate::FieldReader<u8, u8>);
impl PLLN_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLN` writer - PLL frequency multiplication factor N"]
pub struct PLLN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `PLLPEN` reader - PLLPCLK clock output enable"]
pub struct PLLPEN_R(crate::FieldReader<bool, bool>);
impl PLLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLPEN` writer - PLLPCLK clock output enable"]
pub struct PLLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLPEN_W<'a> {
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
#[doc = "Field `PLLP` reader - PLL VCO division factor P for PLLPCLK clock output"]
pub struct PLLP_R(crate::FieldReader<u8, u8>);
impl PLLP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLP` writer - PLL VCO division factor P for PLLPCLK clock output"]
pub struct PLLP_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | ((value as u32 & 0x1f) << 17);
        self.w
    }
}
#[doc = "Field `PLLQEN` reader - PLLQCLK clock output enable"]
pub struct PLLQEN_R(crate::FieldReader<bool, bool>);
impl PLLQEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLQEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLQEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLQEN` writer - PLLQCLK clock output enable"]
pub struct PLLQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLQEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `PLLQ` reader - PLL VCO division factor Q for PLLQCLK clock output"]
pub struct PLLQ_R(crate::FieldReader<u8, u8>);
impl PLLQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLQ` writer - PLL VCO division factor Q for PLLQCLK clock output"]
pub struct PLLQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | ((value as u32 & 0x07) << 25);
        self.w
    }
}
#[doc = "Field `PLLREN` reader - PLLRCLK clock output enable"]
pub struct PLLREN_R(crate::FieldReader<bool, bool>);
impl PLLREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLREN` writer - PLLRCLK clock output enable"]
pub struct PLLREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `PLLR` reader - PLL VCO division factor R for PLLRCLK clock output"]
pub struct PLLR_R(crate::FieldReader<u8, u8>);
impl PLLR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLR` writer - PLL VCO division factor R for PLLRCLK clock output"]
pub struct PLLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | ((value as u32 & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - PLL input clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Division factor M of the PLL input clock divider"]
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:14 - PLL frequency multiplication factor N"]
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - PLLPCLK clock output enable"]
    #[inline(always)]
    pub fn pllpen(&self) -> PLLPEN_R {
        PLLPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:21 - PLL VCO division factor P for PLLPCLK clock output"]
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - PLLQCLK clock output enable"]
    #[inline(always)]
    pub fn pllqen(&self) -> PLLQEN_R {
        PLLQEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:27 - PLL VCO division factor Q for PLLQCLK clock output"]
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bit 28 - PLLRCLK clock output enable"]
    #[inline(always)]
    pub fn pllren(&self) -> PLLREN_R {
        PLLREN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 29:31 - PLL VCO division factor R for PLLRCLK clock output"]
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PLL input clock source"]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W {
        PLLSRC_W { w: self }
    }
    #[doc = "Bits 4:6 - Division factor M of the PLL input clock divider"]
    #[inline(always)]
    pub fn pllm(&mut self) -> PLLM_W {
        PLLM_W { w: self }
    }
    #[doc = "Bits 8:14 - PLL frequency multiplication factor N"]
    #[inline(always)]
    pub fn plln(&mut self) -> PLLN_W {
        PLLN_W { w: self }
    }
    #[doc = "Bit 16 - PLLPCLK clock output enable"]
    #[inline(always)]
    pub fn pllpen(&mut self) -> PLLPEN_W {
        PLLPEN_W { w: self }
    }
    #[doc = "Bits 17:21 - PLL VCO division factor P for PLLPCLK clock output"]
    #[inline(always)]
    pub fn pllp(&mut self) -> PLLP_W {
        PLLP_W { w: self }
    }
    #[doc = "Bit 24 - PLLQCLK clock output enable"]
    #[inline(always)]
    pub fn pllqen(&mut self) -> PLLQEN_W {
        PLLQEN_W { w: self }
    }
    #[doc = "Bits 25:27 - PLL VCO division factor Q for PLLQCLK clock output"]
    #[inline(always)]
    pub fn pllq(&mut self) -> PLLQ_W {
        PLLQ_W { w: self }
    }
    #[doc = "Bit 28 - PLLRCLK clock output enable"]
    #[inline(always)]
    pub fn pllren(&mut self) -> PLLREN_W {
        PLLREN_W { w: self }
    }
    #[doc = "Bits 29:31 - PLL VCO division factor R for PLLRCLK clock output"]
    #[inline(always)]
    pub fn pllr(&mut self) -> PLLR_W {
        PLLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllsyscfgr](index.html) module"]
pub struct PLLSYSCFGR_SPEC;
impl crate::RegisterSpec for PLLSYSCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllsyscfgr::R](R) reader structure"]
impl crate::Readable for PLLSYSCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllsyscfgr::W](W) writer structure"]
impl crate::Writable for PLLSYSCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLSYSCFGR to value 0x1000"]
impl crate::Resettable for PLLSYSCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000
    }
}
