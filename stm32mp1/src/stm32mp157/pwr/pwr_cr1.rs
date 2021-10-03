#[doc = "Register `PWR_CR1` reader"]
pub struct R(crate::R<PWR_CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_CR1` writer"]
pub struct W(crate::W<PWR_CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_CR1_SPEC>;
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
impl From<crate::W<PWR_CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPDS` reader - LPDS"]
pub struct LPDS_R(crate::FieldReader<bool, bool>);
impl LPDS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPDS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPDS` writer - LPDS"]
pub struct LPDS_W<'a> {
    w: &'a mut W,
}
impl<'a> LPDS_W<'a> {
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
#[doc = "Field `LPCFG` reader - LPCFG"]
pub struct LPCFG_R(crate::FieldReader<bool, bool>);
impl LPCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPCFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPCFG` writer - LPCFG"]
pub struct LPCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCFG_W<'a> {
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
#[doc = "Field `LVDS` reader - LVDS"]
pub struct LVDS_R(crate::FieldReader<bool, bool>);
impl LVDS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LVDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVDS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVDS` writer - LVDS"]
pub struct LVDS_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDS_W<'a> {
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
#[doc = "Field `PVDEN` reader - PVDEN"]
pub struct PVDEN_R(crate::FieldReader<bool, bool>);
impl PVDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVDEN` writer - PVDEN"]
pub struct PVDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PVDEN_W<'a> {
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
#[doc = "Field `PLS` reader - PLS"]
pub struct PLS_R(crate::FieldReader<u8, u8>);
impl PLS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLS` writer - PLS"]
pub struct PLS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `DBP` reader - DBP"]
pub struct DBP_R(crate::FieldReader<bool, bool>);
impl DBP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBP` writer - DBP"]
pub struct DBP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBP_W<'a> {
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
#[doc = "Field `AVDEN` reader - AVDEN"]
pub struct AVDEN_R(crate::FieldReader<bool, bool>);
impl AVDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AVDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVDEN` writer - AVDEN"]
pub struct AVDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AVDEN_W<'a> {
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
#[doc = "Field `ALS` reader - ALS"]
pub struct ALS_R(crate::FieldReader<u8, u8>);
impl ALS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALS` writer - ALS"]
pub struct ALS_W<'a> {
    w: &'a mut W,
}
impl<'a> ALS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LPDS"]
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LPCFG"]
    #[inline(always)]
    pub fn lpcfg(&self) -> LPCFG_R {
        LPCFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LVDS"]
    #[inline(always)]
    pub fn lvds(&self) -> LVDS_R {
        LVDS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PVDEN"]
    #[inline(always)]
    pub fn pvden(&self) -> PVDEN_R {
        PVDEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - PLS"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - DBP"]
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - AVDEN"]
    #[inline(always)]
    pub fn avden(&self) -> AVDEN_R {
        AVDEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - ALS"]
    #[inline(always)]
    pub fn als(&self) -> ALS_R {
        ALS_R::new(((self.bits >> 17) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LPDS"]
    #[inline(always)]
    pub fn lpds(&mut self) -> LPDS_W {
        LPDS_W { w: self }
    }
    #[doc = "Bit 1 - LPCFG"]
    #[inline(always)]
    pub fn lpcfg(&mut self) -> LPCFG_W {
        LPCFG_W { w: self }
    }
    #[doc = "Bit 2 - LVDS"]
    #[inline(always)]
    pub fn lvds(&mut self) -> LVDS_W {
        LVDS_W { w: self }
    }
    #[doc = "Bit 4 - PVDEN"]
    #[inline(always)]
    pub fn pvden(&mut self) -> PVDEN_W {
        PVDEN_W { w: self }
    }
    #[doc = "Bits 5:7 - PLS"]
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W {
        PLS_W { w: self }
    }
    #[doc = "Bit 8 - DBP"]
    #[inline(always)]
    pub fn dbp(&mut self) -> DBP_W {
        DBP_W { w: self }
    }
    #[doc = "Bit 16 - AVDEN"]
    #[inline(always)]
    pub fn avden(&mut self) -> AVDEN_W {
        AVDEN_W { w: self }
    }
    #[doc = "Bits 17:18 - ALS"]
    #[inline(always)]
    pub fn als(&mut self) -> ALS_W {
        ALS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset on any system reset. This register provides write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_cr1](index.html) module"]
pub struct PWR_CR1_SPEC;
impl crate::RegisterSpec for PWR_CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_cr1::R](R) reader structure"]
impl crate::Readable for PWR_CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_cr1::W](W) writer structure"]
impl crate::Writable for PWR_CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_CR1 to value 0"]
impl crate::Resettable for PWR_CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
