#[doc = "Register `SECCFGR` reader"]
pub struct R(crate::R<SECCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECCFGR` writer"]
pub struct W(crate::W<SECCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECCFGR_SPEC>;
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
impl From<crate::W<SECCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSISEC` reader - HSISEC"]
pub struct HSISEC_R(crate::FieldReader<bool, bool>);
impl HSISEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSISEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSISEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSISEC` writer - HSISEC"]
pub struct HSISEC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSISEC_W<'a> {
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
#[doc = "Field `HSESEC` reader - HSESEC"]
pub struct HSESEC_R(crate::FieldReader<bool, bool>);
impl HSESEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSESEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSESEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSESEC` writer - HSESEC"]
pub struct HSESEC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSESEC_W<'a> {
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
#[doc = "Field `MSISEC` reader - MSISEC"]
pub struct MSISEC_R(crate::FieldReader<bool, bool>);
impl MSISEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSISEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSISEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSISEC` writer - MSISEC"]
pub struct MSISEC_W<'a> {
    w: &'a mut W,
}
impl<'a> MSISEC_W<'a> {
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
#[doc = "Field `LSISEC` reader - LSISEC"]
pub struct LSISEC_R(crate::FieldReader<bool, bool>);
impl LSISEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSISEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSISEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSISEC` writer - LSISEC"]
pub struct LSISEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LSISEC_W<'a> {
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
#[doc = "Field `LSESEC` reader - LSESEC"]
pub struct LSESEC_R(crate::FieldReader<bool, bool>);
impl LSESEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSESEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSESEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSESEC` writer - LSESEC"]
pub struct LSESEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LSESEC_W<'a> {
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
#[doc = "Field `SYSCLKSEC` reader - SYSCLKSEC"]
pub struct SYSCLKSEC_R(crate::FieldReader<bool, bool>);
impl SYSCLKSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSCLKSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSCLKSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSCLKSEC` writer - SYSCLKSEC"]
pub struct SYSCLKSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCLKSEC_W<'a> {
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
#[doc = "Field `PRESCSEC` reader - PRESCSEC"]
pub struct PRESCSEC_R(crate::FieldReader<bool, bool>);
impl PRESCSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRESCSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESCSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESCSEC` writer - PRESCSEC"]
pub struct PRESCSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCSEC_W<'a> {
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
#[doc = "Field `PLLSEC` reader - PLLSEC"]
pub struct PLLSEC_R(crate::FieldReader<bool, bool>);
impl PLLSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSEC` writer - PLLSEC"]
pub struct PLLSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSEC_W<'a> {
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
#[doc = "Field `PLLSAI1SEC` reader - PLLSAI1SEC"]
pub struct PLLSAI1SEC_R(crate::FieldReader<bool, bool>);
impl PLLSAI1SEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLSAI1SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSAI1SEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSAI1SEC` writer - PLLSAI1SEC"]
pub struct PLLSAI1SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI1SEC_W<'a> {
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
#[doc = "Field `PLLSAI2SEC` reader - PLLSAI2SEC"]
pub struct PLLSAI2SEC_R(crate::FieldReader<bool, bool>);
impl PLLSAI2SEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLSAI2SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSAI2SEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSAI2SEC` writer - PLLSAI2SEC"]
pub struct PLLSAI2SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2SEC_W<'a> {
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
#[doc = "Field `CLK48MSEC` reader - CLK48MSEC"]
pub struct CLK48MSEC_R(crate::FieldReader<bool, bool>);
impl CLK48MSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLK48MSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK48MSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK48MSEC` writer - CLK48MSEC"]
pub struct CLK48MSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK48MSEC_W<'a> {
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
#[doc = "Field `HSI48SEC` reader - HSI48SEC"]
pub struct HSI48SEC_R(crate::FieldReader<bool, bool>);
impl HSI48SEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSI48SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI48SEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSI48SEC` writer - HSI48SEC"]
pub struct HSI48SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI48SEC_W<'a> {
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
#[doc = "Field `RMVFSEC` reader - RMVFSEC"]
pub struct RMVFSEC_R(crate::FieldReader<bool, bool>);
impl RMVFSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RMVFSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMVFSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMVFSEC` writer - RMVFSEC"]
pub struct RMVFSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> RMVFSEC_W<'a> {
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
impl R {
    #[doc = "Bit 0 - HSISEC"]
    #[inline(always)]
    pub fn hsisec(&self) -> HSISEC_R {
        HSISEC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HSESEC"]
    #[inline(always)]
    pub fn hsesec(&self) -> HSESEC_R {
        HSESEC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MSISEC"]
    #[inline(always)]
    pub fn msisec(&self) -> MSISEC_R {
        MSISEC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LSISEC"]
    #[inline(always)]
    pub fn lsisec(&self) -> LSISEC_R {
        LSISEC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LSESEC"]
    #[inline(always)]
    pub fn lsesec(&self) -> LSESEC_R {
        LSESEC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SYSCLKSEC"]
    #[inline(always)]
    pub fn sysclksec(&self) -> SYSCLKSEC_R {
        SYSCLKSEC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PRESCSEC"]
    #[inline(always)]
    pub fn prescsec(&self) -> PRESCSEC_R {
        PRESCSEC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PLLSEC"]
    #[inline(always)]
    pub fn pllsec(&self) -> PLLSEC_R {
        PLLSEC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PLLSAI1SEC"]
    #[inline(always)]
    pub fn pllsai1sec(&self) -> PLLSAI1SEC_R {
        PLLSAI1SEC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PLLSAI2SEC"]
    #[inline(always)]
    pub fn pllsai2sec(&self) -> PLLSAI2SEC_R {
        PLLSAI2SEC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CLK48MSEC"]
    #[inline(always)]
    pub fn clk48msec(&self) -> CLK48MSEC_R {
        CLK48MSEC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HSI48SEC"]
    #[inline(always)]
    pub fn hsi48sec(&self) -> HSI48SEC_R {
        HSI48SEC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RMVFSEC"]
    #[inline(always)]
    pub fn rmvfsec(&self) -> RMVFSEC_R {
        RMVFSEC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSISEC"]
    #[inline(always)]
    pub fn hsisec(&mut self) -> HSISEC_W {
        HSISEC_W { w: self }
    }
    #[doc = "Bit 1 - HSESEC"]
    #[inline(always)]
    pub fn hsesec(&mut self) -> HSESEC_W {
        HSESEC_W { w: self }
    }
    #[doc = "Bit 2 - MSISEC"]
    #[inline(always)]
    pub fn msisec(&mut self) -> MSISEC_W {
        MSISEC_W { w: self }
    }
    #[doc = "Bit 3 - LSISEC"]
    #[inline(always)]
    pub fn lsisec(&mut self) -> LSISEC_W {
        LSISEC_W { w: self }
    }
    #[doc = "Bit 4 - LSESEC"]
    #[inline(always)]
    pub fn lsesec(&mut self) -> LSESEC_W {
        LSESEC_W { w: self }
    }
    #[doc = "Bit 5 - SYSCLKSEC"]
    #[inline(always)]
    pub fn sysclksec(&mut self) -> SYSCLKSEC_W {
        SYSCLKSEC_W { w: self }
    }
    #[doc = "Bit 6 - PRESCSEC"]
    #[inline(always)]
    pub fn prescsec(&mut self) -> PRESCSEC_W {
        PRESCSEC_W { w: self }
    }
    #[doc = "Bit 7 - PLLSEC"]
    #[inline(always)]
    pub fn pllsec(&mut self) -> PLLSEC_W {
        PLLSEC_W { w: self }
    }
    #[doc = "Bit 8 - PLLSAI1SEC"]
    #[inline(always)]
    pub fn pllsai1sec(&mut self) -> PLLSAI1SEC_W {
        PLLSAI1SEC_W { w: self }
    }
    #[doc = "Bit 9 - PLLSAI2SEC"]
    #[inline(always)]
    pub fn pllsai2sec(&mut self) -> PLLSAI2SEC_W {
        PLLSAI2SEC_W { w: self }
    }
    #[doc = "Bit 10 - CLK48MSEC"]
    #[inline(always)]
    pub fn clk48msec(&mut self) -> CLK48MSEC_W {
        CLK48MSEC_W { w: self }
    }
    #[doc = "Bit 11 - HSI48SEC"]
    #[inline(always)]
    pub fn hsi48sec(&mut self) -> HSI48SEC_W {
        HSI48SEC_W { w: self }
    }
    #[doc = "Bit 12 - RMVFSEC"]
    #[inline(always)]
    pub fn rmvfsec(&mut self) -> RMVFSEC_W {
        RMVFSEC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC secure configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seccfgr](index.html) module"]
pub struct SECCFGR_SPEC;
impl crate::RegisterSpec for SECCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seccfgr::R](R) reader structure"]
impl crate::Readable for SECCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seccfgr::W](W) writer structure"]
impl crate::Writable for SECCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECCFGR to value 0"]
impl crate::Resettable for SECCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
