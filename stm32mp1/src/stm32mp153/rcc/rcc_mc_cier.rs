#[doc = "Register `RCC_MC_CIER` reader"]
pub struct R(crate::R<RCC_MC_CIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_CIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_CIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_CIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MC_CIER` writer"]
pub struct W(crate::W<RCC_MC_CIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_CIER_SPEC>;
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
impl From<crate::W<RCC_MC_CIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_CIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSIRDYIE` reader - LSIRDYIE"]
pub struct LSIRDYIE_R(crate::FieldReader<bool, bool>);
impl LSIRDYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSIRDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSIRDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSIRDYIE` writer - LSIRDYIE"]
pub struct LSIRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSIRDYIE_W<'a> {
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
#[doc = "Field `LSERDYIE` reader - LSERDYIE"]
pub struct LSERDYIE_R(crate::FieldReader<bool, bool>);
impl LSERDYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSERDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSERDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSERDYIE` writer - LSERDYIE"]
pub struct LSERDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSERDYIE_W<'a> {
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
#[doc = "Field `HSIRDYIE` reader - HSIRDYIE"]
pub struct HSIRDYIE_R(crate::FieldReader<bool, bool>);
impl HSIRDYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSIRDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSIRDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSIRDYIE` writer - HSIRDYIE"]
pub struct HSIRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIRDYIE_W<'a> {
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
#[doc = "Field `HSERDYIE` reader - HSERDYIE"]
pub struct HSERDYIE_R(crate::FieldReader<bool, bool>);
impl HSERDYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSERDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSERDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSERDYIE` writer - HSERDYIE"]
pub struct HSERDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSERDYIE_W<'a> {
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
#[doc = "Field `CSIRDYIE` reader - CSIRDYIE"]
pub struct CSIRDYIE_R(crate::FieldReader<bool, bool>);
impl CSIRDYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSIRDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSIRDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSIRDYIE` writer - CSIRDYIE"]
pub struct CSIRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CSIRDYIE_W<'a> {
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
#[doc = "Field `PLL1DYIE` reader - PLL1DYIE"]
pub struct PLL1DYIE_R(crate::FieldReader<bool, bool>);
impl PLL1DYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL1DYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL1DYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL1DYIE` writer - PLL1DYIE"]
pub struct PLL1DYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL1DYIE_W<'a> {
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
#[doc = "Field `PLL2DYIE` reader - PLL2DYIE"]
pub struct PLL2DYIE_R(crate::FieldReader<bool, bool>);
impl PLL2DYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL2DYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL2DYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL2DYIE` writer - PLL2DYIE"]
pub struct PLL2DYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL2DYIE_W<'a> {
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
#[doc = "Field `PLL3DYIE` reader - PLL3DYIE"]
pub struct PLL3DYIE_R(crate::FieldReader<bool, bool>);
impl PLL3DYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL3DYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL3DYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL3DYIE` writer - PLL3DYIE"]
pub struct PLL3DYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL3DYIE_W<'a> {
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
#[doc = "Field `PLL4DYIE` reader - PLL4DYIE"]
pub struct PLL4DYIE_R(crate::FieldReader<bool, bool>);
impl PLL4DYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL4DYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL4DYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL4DYIE` writer - PLL4DYIE"]
pub struct PLL4DYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL4DYIE_W<'a> {
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
#[doc = "Field `LSECSSIE` reader - LSECSSIE"]
pub struct LSECSSIE_R(crate::FieldReader<bool, bool>);
impl LSECSSIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSECSSIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSECSSIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSECSSIE` writer - LSECSSIE"]
pub struct LSECSSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSECSSIE_W<'a> {
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
#[doc = "Field `WKUPIE` reader - WKUPIE"]
pub struct WKUPIE_R(crate::FieldReader<bool, bool>);
impl WKUPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPIE` writer - WKUPIE"]
pub struct WKUPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPIE_W<'a> {
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
    #[doc = "Bit 0 - LSIRDYIE"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSERDYIE"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSIRDYIE"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HSERDYIE"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CSIRDYIE"]
    #[inline(always)]
    pub fn csirdyie(&self) -> CSIRDYIE_R {
        CSIRDYIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PLL1DYIE"]
    #[inline(always)]
    pub fn pll1dyie(&self) -> PLL1DYIE_R {
        PLL1DYIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PLL2DYIE"]
    #[inline(always)]
    pub fn pll2dyie(&self) -> PLL2DYIE_R {
        PLL2DYIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PLL3DYIE"]
    #[inline(always)]
    pub fn pll3dyie(&self) -> PLL3DYIE_R {
        PLL3DYIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PLL4DYIE"]
    #[inline(always)]
    pub fn pll4dyie(&self) -> PLL4DYIE_R {
        PLL4DYIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LSECSSIE"]
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - WKUPIE"]
    #[inline(always)]
    pub fn wkupie(&self) -> WKUPIE_R {
        WKUPIE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSIRDYIE"]
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W {
        LSIRDYIE_W { w: self }
    }
    #[doc = "Bit 1 - LSERDYIE"]
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W {
        LSERDYIE_W { w: self }
    }
    #[doc = "Bit 2 - HSIRDYIE"]
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W {
        HSIRDYIE_W { w: self }
    }
    #[doc = "Bit 3 - HSERDYIE"]
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W {
        HSERDYIE_W { w: self }
    }
    #[doc = "Bit 4 - CSIRDYIE"]
    #[inline(always)]
    pub fn csirdyie(&mut self) -> CSIRDYIE_W {
        CSIRDYIE_W { w: self }
    }
    #[doc = "Bit 8 - PLL1DYIE"]
    #[inline(always)]
    pub fn pll1dyie(&mut self) -> PLL1DYIE_W {
        PLL1DYIE_W { w: self }
    }
    #[doc = "Bit 9 - PLL2DYIE"]
    #[inline(always)]
    pub fn pll2dyie(&mut self) -> PLL2DYIE_W {
        PLL2DYIE_W { w: self }
    }
    #[doc = "Bit 10 - PLL3DYIE"]
    #[inline(always)]
    pub fn pll3dyie(&mut self) -> PLL3DYIE_W {
        PLL3DYIE_W { w: self }
    }
    #[doc = "Bit 11 - PLL4DYIE"]
    #[inline(always)]
    pub fn pll4dyie(&mut self) -> PLL4DYIE_W {
        PLL4DYIE_W { w: self }
    }
    #[doc = "Bit 16 - LSECSSIE"]
    #[inline(always)]
    pub fn lsecssie(&mut self) -> LSECSSIE_W {
        LSECSSIE_W { w: self }
    }
    #[doc = "Bit 20 - WKUPIE"]
    #[inline(always)]
    pub fn wkupie(&mut self) -> WKUPIE_W {
        WKUPIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register shall be used by the MCU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_cier](index.html) module"]
pub struct RCC_MC_CIER_SPEC;
impl crate::RegisterSpec for RCC_MC_CIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mc_cier::R](R) reader structure"]
impl crate::Readable for RCC_MC_CIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mc_cier::W](W) writer structure"]
impl crate::Writable for RCC_MC_CIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MC_CIER to value 0"]
impl crate::Resettable for RCC_MC_CIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
