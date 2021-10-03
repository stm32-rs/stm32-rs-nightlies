#[doc = "Register `RCC_BDCR` reader"]
pub struct R(crate::R<RCC_BDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_BDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_BDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_BDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_BDCR` writer"]
pub struct W(crate::W<RCC_BDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_BDCR_SPEC>;
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
impl From<crate::W<RCC_BDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_BDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSEON` reader - LSEON"]
pub struct LSEON_R(crate::FieldReader<bool, bool>);
impl LSEON_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSEON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSEON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSEON` writer - LSEON"]
pub struct LSEON_W<'a> {
    w: &'a mut W,
}
impl<'a> LSEON_W<'a> {
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
#[doc = "Field `LSEBYP` reader - LSEBYP"]
pub struct LSEBYP_R(crate::FieldReader<bool, bool>);
impl LSEBYP_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSEBYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSEBYP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSEBYP` writer - LSEBYP"]
pub struct LSEBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> LSEBYP_W<'a> {
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
#[doc = "Field `LSERDY` reader - LSERDY"]
pub struct LSERDY_R(crate::FieldReader<bool, bool>);
impl LSERDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSERDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSERDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIGBYP` reader - DIGBYP"]
pub struct DIGBYP_R(crate::FieldReader<bool, bool>);
impl DIGBYP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIGBYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIGBYP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSEDRV` reader - LSEDRV"]
pub struct LSEDRV_R(crate::FieldReader<u8, u8>);
impl LSEDRV_R {
    pub(crate) fn new(bits: u8) -> Self {
        LSEDRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSEDRV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSEDRV` writer - LSEDRV"]
pub struct LSEDRV_W<'a> {
    w: &'a mut W,
}
impl<'a> LSEDRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `LSECSSON` reader - LSECSSON"]
pub struct LSECSSON_R(crate::FieldReader<bool, bool>);
impl LSECSSON_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSECSSON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSECSSON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSECSSON` writer - LSECSSON"]
pub struct LSECSSON_W<'a> {
    w: &'a mut W,
}
impl<'a> LSECSSON_W<'a> {
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
#[doc = "Field `LSECSSD` reader - LSECSSD"]
pub struct LSECSSD_R(crate::FieldReader<bool, bool>);
impl LSECSSD_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSECSSD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSECSSD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCSRC` reader - RTCSRC"]
pub struct RTCSRC_R(crate::FieldReader<u8, u8>);
impl RTCSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTCSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCSRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCKEN` reader - RTCCKEN"]
pub struct RTCCKEN_R(crate::FieldReader<bool, bool>);
impl RTCCKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCCKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCCKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCKEN` writer - RTCCKEN"]
pub struct RTCCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCKEN_W<'a> {
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
#[doc = "Field `VSWRST` reader - VSWRST"]
pub struct VSWRST_R(crate::FieldReader<bool, bool>);
impl VSWRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        VSWRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSWRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSWRST` writer - VSWRST"]
pub struct VSWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> VSWRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LSEON"]
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSEBYP"]
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LSERDY"]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DIGBYP"]
    #[inline(always)]
    pub fn digbyp(&self) -> DIGBYP_R {
        DIGBYP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - LSEDRV"]
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - LSECSSON"]
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LSECSSD"]
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - RTCSRC"]
    #[inline(always)]
    pub fn rtcsrc(&self) -> RTCSRC_R {
        RTCSRC_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 20 - RTCCKEN"]
    #[inline(always)]
    pub fn rtccken(&self) -> RTCCKEN_R {
        RTCCKEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 31 - VSWRST"]
    #[inline(always)]
    pub fn vswrst(&self) -> VSWRST_R {
        VSWRST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSEON"]
    #[inline(always)]
    pub fn lseon(&mut self) -> LSEON_W {
        LSEON_W { w: self }
    }
    #[doc = "Bit 1 - LSEBYP"]
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LSEBYP_W {
        LSEBYP_W { w: self }
    }
    #[doc = "Bits 4:5 - LSEDRV"]
    #[inline(always)]
    pub fn lsedrv(&mut self) -> LSEDRV_W {
        LSEDRV_W { w: self }
    }
    #[doc = "Bit 8 - LSECSSON"]
    #[inline(always)]
    pub fn lsecsson(&mut self) -> LSECSSON_W {
        LSECSSON_W { w: self }
    }
    #[doc = "Bit 20 - RTCCKEN"]
    #[inline(always)]
    pub fn rtccken(&mut self) -> RTCCKEN_W {
        RTCCKEN_W { w: self }
    }
    #[doc = "Bit 31 - VSWRST"]
    #[inline(always)]
    pub fn vswrst(&mut self) -> VSWRST_W {
        VSWRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the LSE function. Wait states are inserted in case of successive write accesses to this register. The number of wait states may be up to 7 cycles of AHB4 clock.After a system reset, the register RCC_BDCR is write-protected. In order to modify this register, the DBP bit in the PWR control register 1 (PWR_CR1) has to be set to . Bits of RCC_BDCR register are only reset after a backup domain reset: nreset_vsw (see Section10.3.6: Backup domain reset). Any other internal or external reset will not have any effect on these bits.This register is located into the VSW domain. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_bdcr](index.html) module"]
pub struct RCC_BDCR_SPEC;
impl crate::RegisterSpec for RCC_BDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_bdcr::R](R) reader structure"]
impl crate::Readable for RCC_BDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_bdcr::W](W) writer structure"]
impl crate::Writable for RCC_BDCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_BDCR to value 0x20"]
impl crate::Resettable for RCC_BDCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
