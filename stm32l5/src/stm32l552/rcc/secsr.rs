#[doc = "Register `SECSR` reader"]
pub struct R(crate::R<SECSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECSR` writer"]
pub struct W(crate::W<SECSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECSR_SPEC>;
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
impl From<crate::W<SECSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RMVFSECF` reader - RMVFSECF"]
pub struct RMVFSECF_R(crate::FieldReader<bool, bool>);
impl RMVFSECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RMVFSECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMVFSECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMVFSECF` writer - RMVFSECF"]
pub struct RMVFSECF_W<'a> {
    w: &'a mut W,
}
impl<'a> RMVFSECF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `HSI48SECF` reader - HSI48SECF"]
pub struct HSI48SECF_R(crate::FieldReader<bool, bool>);
impl HSI48SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSI48SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI48SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSI48SECF` writer - HSI48SECF"]
pub struct HSI48SECF_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI48SECF_W<'a> {
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
#[doc = "Field `CLK48MSECF` reader - CLK48MSECF"]
pub struct CLK48MSECF_R(crate::FieldReader<bool, bool>);
impl CLK48MSECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLK48MSECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK48MSECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK48MSECF` writer - CLK48MSECF"]
pub struct CLK48MSECF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK48MSECF_W<'a> {
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
#[doc = "Field `PLLSAI2SECF` reader - PLLSAI2SECF"]
pub struct PLLSAI2SECF_R(crate::FieldReader<bool, bool>);
impl PLLSAI2SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLSAI2SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSAI2SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSAI2SECF` writer - PLLSAI2SECF"]
pub struct PLLSAI2SECF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2SECF_W<'a> {
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
#[doc = "Field `PLLSAI1SECF` reader - PLLSAI1SECF"]
pub struct PLLSAI1SECF_R(crate::FieldReader<bool, bool>);
impl PLLSAI1SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLSAI1SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSAI1SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSAI1SECF` writer - PLLSAI1SECF"]
pub struct PLLSAI1SECF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI1SECF_W<'a> {
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
#[doc = "Field `PLLSECF` reader - PLLSECF"]
pub struct PLLSECF_R(crate::FieldReader<bool, bool>);
impl PLLSECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLSECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSECF` writer - PLLSECF"]
pub struct PLLSECF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSECF_W<'a> {
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
#[doc = "Field `PRESCSECF` reader - PRESCSECF"]
pub struct PRESCSECF_R(crate::FieldReader<bool, bool>);
impl PRESCSECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRESCSECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESCSECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESCSECF` writer - PRESCSECF"]
pub struct PRESCSECF_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCSECF_W<'a> {
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
#[doc = "Field `SYSCLKSECF` reader - SYSCLKSECF"]
pub struct SYSCLKSECF_R(crate::FieldReader<bool, bool>);
impl SYSCLKSECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSCLKSECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSCLKSECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSCLKSECF` writer - SYSCLKSECF"]
pub struct SYSCLKSECF_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCLKSECF_W<'a> {
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
#[doc = "Field `LSESECF` reader - LSESECF"]
pub struct LSESECF_R(crate::FieldReader<bool, bool>);
impl LSESECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSESECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSESECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSESECF` writer - LSESECF"]
pub struct LSESECF_W<'a> {
    w: &'a mut W,
}
impl<'a> LSESECF_W<'a> {
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
#[doc = "Field `LSISECF` reader - LSISECF"]
pub struct LSISECF_R(crate::FieldReader<bool, bool>);
impl LSISECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSISECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSISECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSISECF` writer - LSISECF"]
pub struct LSISECF_W<'a> {
    w: &'a mut W,
}
impl<'a> LSISECF_W<'a> {
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
#[doc = "Field `MSISECF` reader - MSISECF"]
pub struct MSISECF_R(crate::FieldReader<bool, bool>);
impl MSISECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSISECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSISECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSISECF` writer - MSISECF"]
pub struct MSISECF_W<'a> {
    w: &'a mut W,
}
impl<'a> MSISECF_W<'a> {
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
#[doc = "Field `HSESECF` reader - HSESECF"]
pub struct HSESECF_R(crate::FieldReader<bool, bool>);
impl HSESECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSESECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSESECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSESECF` writer - HSESECF"]
pub struct HSESECF_W<'a> {
    w: &'a mut W,
}
impl<'a> HSESECF_W<'a> {
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
#[doc = "Field `HSISECF` reader - HSISECF"]
pub struct HSISECF_R(crate::FieldReader<bool, bool>);
impl HSISECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSISECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSISECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSISECF` writer - HSISECF"]
pub struct HSISECF_W<'a> {
    w: &'a mut W,
}
impl<'a> HSISECF_W<'a> {
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
impl R {
    #[doc = "Bit 12 - RMVFSECF"]
    #[inline(always)]
    pub fn rmvfsecf(&self) -> RMVFSECF_R {
        RMVFSECF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HSI48SECF"]
    #[inline(always)]
    pub fn hsi48secf(&self) -> HSI48SECF_R {
        HSI48SECF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CLK48MSECF"]
    #[inline(always)]
    pub fn clk48msecf(&self) -> CLK48MSECF_R {
        CLK48MSECF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PLLSAI2SECF"]
    #[inline(always)]
    pub fn pllsai2secf(&self) -> PLLSAI2SECF_R {
        PLLSAI2SECF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PLLSAI1SECF"]
    #[inline(always)]
    pub fn pllsai1secf(&self) -> PLLSAI1SECF_R {
        PLLSAI1SECF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PLLSECF"]
    #[inline(always)]
    pub fn pllsecf(&self) -> PLLSECF_R {
        PLLSECF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PRESCSECF"]
    #[inline(always)]
    pub fn prescsecf(&self) -> PRESCSECF_R {
        PRESCSECF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SYSCLKSECF"]
    #[inline(always)]
    pub fn sysclksecf(&self) -> SYSCLKSECF_R {
        SYSCLKSECF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LSESECF"]
    #[inline(always)]
    pub fn lsesecf(&self) -> LSESECF_R {
        LSESECF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LSISECF"]
    #[inline(always)]
    pub fn lsisecf(&self) -> LSISECF_R {
        LSISECF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MSISECF"]
    #[inline(always)]
    pub fn msisecf(&self) -> MSISECF_R {
        MSISECF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - HSESECF"]
    #[inline(always)]
    pub fn hsesecf(&self) -> HSESECF_R {
        HSESECF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - HSISECF"]
    #[inline(always)]
    pub fn hsisecf(&self) -> HSISECF_R {
        HSISECF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - RMVFSECF"]
    #[inline(always)]
    pub fn rmvfsecf(&mut self) -> RMVFSECF_W {
        RMVFSECF_W { w: self }
    }
    #[doc = "Bit 11 - HSI48SECF"]
    #[inline(always)]
    pub fn hsi48secf(&mut self) -> HSI48SECF_W {
        HSI48SECF_W { w: self }
    }
    #[doc = "Bit 10 - CLK48MSECF"]
    #[inline(always)]
    pub fn clk48msecf(&mut self) -> CLK48MSECF_W {
        CLK48MSECF_W { w: self }
    }
    #[doc = "Bit 9 - PLLSAI2SECF"]
    #[inline(always)]
    pub fn pllsai2secf(&mut self) -> PLLSAI2SECF_W {
        PLLSAI2SECF_W { w: self }
    }
    #[doc = "Bit 8 - PLLSAI1SECF"]
    #[inline(always)]
    pub fn pllsai1secf(&mut self) -> PLLSAI1SECF_W {
        PLLSAI1SECF_W { w: self }
    }
    #[doc = "Bit 7 - PLLSECF"]
    #[inline(always)]
    pub fn pllsecf(&mut self) -> PLLSECF_W {
        PLLSECF_W { w: self }
    }
    #[doc = "Bit 6 - PRESCSECF"]
    #[inline(always)]
    pub fn prescsecf(&mut self) -> PRESCSECF_W {
        PRESCSECF_W { w: self }
    }
    #[doc = "Bit 5 - SYSCLKSECF"]
    #[inline(always)]
    pub fn sysclksecf(&mut self) -> SYSCLKSECF_W {
        SYSCLKSECF_W { w: self }
    }
    #[doc = "Bit 4 - LSESECF"]
    #[inline(always)]
    pub fn lsesecf(&mut self) -> LSESECF_W {
        LSESECF_W { w: self }
    }
    #[doc = "Bit 3 - LSISECF"]
    #[inline(always)]
    pub fn lsisecf(&mut self) -> LSISECF_W {
        LSISECF_W { w: self }
    }
    #[doc = "Bit 2 - MSISECF"]
    #[inline(always)]
    pub fn msisecf(&mut self) -> MSISECF_W {
        MSISECF_W { w: self }
    }
    #[doc = "Bit 1 - HSESECF"]
    #[inline(always)]
    pub fn hsesecf(&mut self) -> HSESECF_W {
        HSESECF_W { w: self }
    }
    #[doc = "Bit 0 - HSISECF"]
    #[inline(always)]
    pub fn hsisecf(&mut self) -> HSISECF_W {
        HSISECF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC secure status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secsr](index.html) module"]
pub struct SECSR_SPEC;
impl crate::RegisterSpec for SECSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secsr::R](R) reader structure"]
impl crate::Readable for SECSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secsr::W](W) writer structure"]
impl crate::Writable for SECSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECSR to value 0"]
impl crate::Resettable for SECSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
