#[doc = "Register `CIER` reader"]
pub struct R(crate::R<CIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIER` writer"]
pub struct W(crate::W<CIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIER_SPEC>;
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
impl From<crate::W<CIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSIRDYIE` reader - LSI ready interrupt enable"]
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
#[doc = "Field `LSIRDYIE` writer - LSI ready interrupt enable"]
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
#[doc = "Field `LSERDYIE` reader - LSE ready interrupt enable"]
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
#[doc = "Field `LSERDYIE` writer - LSE ready interrupt enable"]
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
#[doc = "Field `MSIRDYIE` reader - MSI ready interrupt enable"]
pub struct MSIRDYIE_R(crate::FieldReader<bool, bool>);
impl MSIRDYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSIRDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSIRDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSIRDYIE` writer - MSI ready interrupt enable"]
pub struct MSIRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MSIRDYIE_W<'a> {
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
#[doc = "Field `HSIRDYIE` reader - HSI ready interrupt enable"]
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
#[doc = "Field `HSIRDYIE` writer - HSI ready interrupt enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `HSERDYIE` reader - HSE ready interrupt enable"]
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
#[doc = "Field `HSERDYIE` writer - HSE ready interrupt enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `PLLRDYIE` reader - PLL ready interrupt enable"]
pub struct PLLRDYIE_R(crate::FieldReader<bool, bool>);
impl PLLRDYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLRDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLRDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLRDYIE` writer - PLL ready interrupt enable"]
pub struct PLLRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLRDYIE_W<'a> {
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
#[doc = "Field `PLLSAI1RDYIE` reader - PLLSAI1 ready interrupt enable"]
pub struct PLLSAI1RDYIE_R(crate::FieldReader<bool, bool>);
impl PLLSAI1RDYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLSAI1RDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSAI1RDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSAI1RDYIE` writer - PLLSAI1 ready interrupt enable"]
pub struct PLLSAI1RDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI1RDYIE_W<'a> {
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
#[doc = "Field `PLLSAI2RDYIE` reader - PLLSAI2 ready interrupt enable"]
pub struct PLLSAI2RDYIE_R(crate::FieldReader<bool, bool>);
impl PLLSAI2RDYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLSAI2RDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSAI2RDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSAI2RDYIE` writer - PLLSAI2 ready interrupt enable"]
pub struct PLLSAI2RDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2RDYIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `LSECSSIE` reader - LSE clock security system interrupt enable"]
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
#[doc = "Field `LSECSSIE` writer - LSE clock security system interrupt enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `HSI48RDYIE` reader - HSI48 ready interrupt enable"]
pub struct HSI48RDYIE_R(crate::FieldReader<bool, bool>);
impl HSI48RDYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSI48RDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI48RDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSI48RDYIE` writer - HSI48 ready interrupt enable"]
pub struct HSI48RDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI48RDYIE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - LSI ready interrupt enable"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MSI ready interrupt enable"]
    #[inline(always)]
    pub fn msirdyie(&self) -> MSIRDYIE_R {
        MSIRDYIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HSI ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PLL ready interrupt enable"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLLSAI1 ready interrupt enable"]
    #[inline(always)]
    pub fn pllsai1rdyie(&self) -> PLLSAI1RDYIE_R {
        PLLSAI1RDYIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PLLSAI2 ready interrupt enable"]
    #[inline(always)]
    pub fn pllsai2rdyie(&self) -> PLLSAI2RDYIE_R {
        PLLSAI2RDYIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LSE clock security system interrupt enable"]
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HSI48 ready interrupt enable"]
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready interrupt enable"]
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W {
        LSIRDYIE_W { w: self }
    }
    #[doc = "Bit 1 - LSE ready interrupt enable"]
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W {
        LSERDYIE_W { w: self }
    }
    #[doc = "Bit 2 - MSI ready interrupt enable"]
    #[inline(always)]
    pub fn msirdyie(&mut self) -> MSIRDYIE_W {
        MSIRDYIE_W { w: self }
    }
    #[doc = "Bit 3 - HSI ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W {
        HSIRDYIE_W { w: self }
    }
    #[doc = "Bit 4 - HSE ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W {
        HSERDYIE_W { w: self }
    }
    #[doc = "Bit 5 - PLL ready interrupt enable"]
    #[inline(always)]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W {
        PLLRDYIE_W { w: self }
    }
    #[doc = "Bit 6 - PLLSAI1 ready interrupt enable"]
    #[inline(always)]
    pub fn pllsai1rdyie(&mut self) -> PLLSAI1RDYIE_W {
        PLLSAI1RDYIE_W { w: self }
    }
    #[doc = "Bit 7 - PLLSAI2 ready interrupt enable"]
    #[inline(always)]
    pub fn pllsai2rdyie(&mut self) -> PLLSAI2RDYIE_W {
        PLLSAI2RDYIE_W { w: self }
    }
    #[doc = "Bit 9 - LSE clock security system interrupt enable"]
    #[inline(always)]
    pub fn lsecssie(&mut self) -> LSECSSIE_W {
        LSECSSIE_W { w: self }
    }
    #[doc = "Bit 10 - HSI48 ready interrupt enable"]
    #[inline(always)]
    pub fn hsi48rdyie(&mut self) -> HSI48RDYIE_W {
        HSI48RDYIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cier](index.html) module"]
pub struct CIER_SPEC;
impl crate::RegisterSpec for CIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cier::R](R) reader structure"]
impl crate::Readable for CIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cier::W](W) writer structure"]
impl crate::Writable for CIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CIER to value 0"]
impl crate::Resettable for CIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
