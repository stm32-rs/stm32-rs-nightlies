#[doc = "Register `RCC_MC_CIFR` reader"]
pub struct R(crate::R<RCC_MC_CIFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_CIFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_CIFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_CIFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MC_CIFR` writer"]
pub struct W(crate::W<RCC_MC_CIFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_CIFR_SPEC>;
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
impl From<crate::W<RCC_MC_CIFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_CIFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSIRDYF` reader - LSIRDYF"]
pub struct LSIRDYF_R(crate::FieldReader<bool, bool>);
impl LSIRDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSIRDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSIRDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSIRDYF` writer - LSIRDYF"]
pub struct LSIRDYF_W<'a> {
    w: &'a mut W,
}
impl<'a> LSIRDYF_W<'a> {
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
#[doc = "Field `LSERDYF` reader - LSERDYF"]
pub struct LSERDYF_R(crate::FieldReader<bool, bool>);
impl LSERDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSERDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSERDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSERDYF` writer - LSERDYF"]
pub struct LSERDYF_W<'a> {
    w: &'a mut W,
}
impl<'a> LSERDYF_W<'a> {
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
#[doc = "Field `HSIRDYF` reader - HSIRDYF"]
pub struct HSIRDYF_R(crate::FieldReader<bool, bool>);
impl HSIRDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSIRDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSIRDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSIRDYF` writer - HSIRDYF"]
pub struct HSIRDYF_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIRDYF_W<'a> {
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
#[doc = "Field `HSERDYF` reader - HSERDYF"]
pub struct HSERDYF_R(crate::FieldReader<bool, bool>);
impl HSERDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSERDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSERDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSERDYF` writer - HSERDYF"]
pub struct HSERDYF_W<'a> {
    w: &'a mut W,
}
impl<'a> HSERDYF_W<'a> {
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
#[doc = "Field `CSIRDYF` reader - CSIRDYF"]
pub struct CSIRDYF_R(crate::FieldReader<bool, bool>);
impl CSIRDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSIRDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSIRDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSIRDYF` writer - CSIRDYF"]
pub struct CSIRDYF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSIRDYF_W<'a> {
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
#[doc = "Field `PLL1DYF` reader - PLL1DYF"]
pub struct PLL1DYF_R(crate::FieldReader<bool, bool>);
impl PLL1DYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL1DYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL1DYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL1DYF` writer - PLL1DYF"]
pub struct PLL1DYF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL1DYF_W<'a> {
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
#[doc = "Field `PLL2DYF` reader - PLL2DYF"]
pub struct PLL2DYF_R(crate::FieldReader<bool, bool>);
impl PLL2DYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL2DYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL2DYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL2DYF` writer - PLL2DYF"]
pub struct PLL2DYF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL2DYF_W<'a> {
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
#[doc = "Field `PLL3DYF` reader - PLL3DYF"]
pub struct PLL3DYF_R(crate::FieldReader<bool, bool>);
impl PLL3DYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL3DYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL3DYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL3DYF` writer - PLL3DYF"]
pub struct PLL3DYF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL3DYF_W<'a> {
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
#[doc = "Field `PLL4DYF` reader - PLL4DYF"]
pub struct PLL4DYF_R(crate::FieldReader<bool, bool>);
impl PLL4DYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL4DYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL4DYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL4DYF` writer - PLL4DYF"]
pub struct PLL4DYF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL4DYF_W<'a> {
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
#[doc = "Field `LSECSSF` reader - LSECSSF"]
pub struct LSECSSF_R(crate::FieldReader<bool, bool>);
impl LSECSSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSECSSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSECSSF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSECSSF` writer - LSECSSF"]
pub struct LSECSSF_W<'a> {
    w: &'a mut W,
}
impl<'a> LSECSSF_W<'a> {
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
#[doc = "Field `WKUPF` reader - WKUPF"]
pub struct WKUPF_R(crate::FieldReader<bool, bool>);
impl WKUPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPF` writer - WKUPF"]
pub struct WKUPF_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPF_W<'a> {
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
    #[doc = "Bit 0 - LSIRDYF"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSERDYF"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSIRDYF"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HSERDYF"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CSIRDYF"]
    #[inline(always)]
    pub fn csirdyf(&self) -> CSIRDYF_R {
        CSIRDYF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PLL1DYF"]
    #[inline(always)]
    pub fn pll1dyf(&self) -> PLL1DYF_R {
        PLL1DYF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PLL2DYF"]
    #[inline(always)]
    pub fn pll2dyf(&self) -> PLL2DYF_R {
        PLL2DYF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PLL3DYF"]
    #[inline(always)]
    pub fn pll3dyf(&self) -> PLL3DYF_R {
        PLL3DYF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PLL4DYF"]
    #[inline(always)]
    pub fn pll4dyf(&self) -> PLL4DYF_R {
        PLL4DYF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LSECSSF"]
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - WKUPF"]
    #[inline(always)]
    pub fn wkupf(&self) -> WKUPF_R {
        WKUPF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSIRDYF"]
    #[inline(always)]
    pub fn lsirdyf(&mut self) -> LSIRDYF_W {
        LSIRDYF_W { w: self }
    }
    #[doc = "Bit 1 - LSERDYF"]
    #[inline(always)]
    pub fn lserdyf(&mut self) -> LSERDYF_W {
        LSERDYF_W { w: self }
    }
    #[doc = "Bit 2 - HSIRDYF"]
    #[inline(always)]
    pub fn hsirdyf(&mut self) -> HSIRDYF_W {
        HSIRDYF_W { w: self }
    }
    #[doc = "Bit 3 - HSERDYF"]
    #[inline(always)]
    pub fn hserdyf(&mut self) -> HSERDYF_W {
        HSERDYF_W { w: self }
    }
    #[doc = "Bit 4 - CSIRDYF"]
    #[inline(always)]
    pub fn csirdyf(&mut self) -> CSIRDYF_W {
        CSIRDYF_W { w: self }
    }
    #[doc = "Bit 8 - PLL1DYF"]
    #[inline(always)]
    pub fn pll1dyf(&mut self) -> PLL1DYF_W {
        PLL1DYF_W { w: self }
    }
    #[doc = "Bit 9 - PLL2DYF"]
    #[inline(always)]
    pub fn pll2dyf(&mut self) -> PLL2DYF_W {
        PLL2DYF_W { w: self }
    }
    #[doc = "Bit 10 - PLL3DYF"]
    #[inline(always)]
    pub fn pll3dyf(&mut self) -> PLL3DYF_W {
        PLL3DYF_W { w: self }
    }
    #[doc = "Bit 11 - PLL4DYF"]
    #[inline(always)]
    pub fn pll4dyf(&mut self) -> PLL4DYF_W {
        PLL4DYF_W { w: self }
    }
    #[doc = "Bit 16 - LSECSSF"]
    #[inline(always)]
    pub fn lsecssf(&mut self) -> LSECSSF_W {
        LSECSSF_W { w: self }
    }
    #[doc = "Bit 20 - WKUPF"]
    #[inline(always)]
    pub fn wkupf(&mut self) -> WKUPF_W {
        WKUPF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register shall be used by the MCU in order to read and clear the interrupt flags.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_cifr](index.html) module"]
pub struct RCC_MC_CIFR_SPEC;
impl crate::RegisterSpec for RCC_MC_CIFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mc_cifr::R](R) reader structure"]
impl crate::Readable for RCC_MC_CIFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mc_cifr::W](W) writer structure"]
impl crate::Writable for RCC_MC_CIFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MC_CIFR to value 0"]
impl crate::Resettable for RCC_MC_CIFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
