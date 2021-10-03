#[doc = "Register `RNG_CR` reader"]
pub struct R(crate::R<RNG_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNG_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RNG_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RNG_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RNG_CR` writer"]
pub struct W(crate::W<RNG_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RNG_CR_SPEC>;
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
impl From<crate::W<RNG_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RNG_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RNGEN` reader - Random number generator enable"]
pub struct RNGEN_R(crate::FieldReader<bool, bool>);
impl RNGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNGEN` writer - Random number generator enable"]
pub struct RNGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGEN_W<'a> {
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
#[doc = "Field `IE` reader - Interrupt enable"]
pub struct IE_R(crate::FieldReader<bool, bool>);
impl IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IE` writer - Interrupt enable"]
pub struct IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_W<'a> {
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
#[doc = "Field `CED` reader - Clock error detection Note: The clock error detection can be used only when ck_rc48 or ck_pll1_q (ck_pll1_q = 48MHz) source is selected otherwise, CED bit must be equal to 1. The clock error detection cannot be enabled nor disabled on the fly when RNG peripheral is enabled, to enable or disable CED the RNG must be disabled."]
pub struct CED_R(crate::FieldReader<bool, bool>);
impl CED_R {
    pub(crate) fn new(bits: bool) -> Self {
        CED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CED` writer - Clock error detection Note: The clock error detection can be used only when ck_rc48 or ck_pll1_q (ck_pll1_q = 48MHz) source is selected otherwise, CED bit must be equal to 1. The clock error detection cannot be enabled nor disabled on the fly when RNG peripheral is enabled, to enable or disable CED the RNG must be disabled."]
pub struct CED_W<'a> {
    w: &'a mut W,
}
impl<'a> CED_W<'a> {
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
#[doc = "Field `RNG_CONFIG3` reader - RNG configuration 3"]
pub struct RNG_CONFIG3_R(crate::FieldReader<u8, u8>);
impl RNG_CONFIG3_R {
    pub(crate) fn new(bits: u8) -> Self {
        RNG_CONFIG3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNG_CONFIG3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNG_CONFIG3` writer - RNG configuration 3"]
pub struct RNG_CONFIG3_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG_CONFIG3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `NISTC` reader - Non NIST compliant"]
pub struct NISTC_R(crate::FieldReader<bool, bool>);
impl NISTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        NISTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NISTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NISTC` writer - Non NIST compliant"]
pub struct NISTC_W<'a> {
    w: &'a mut W,
}
impl<'a> NISTC_W<'a> {
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
#[doc = "Field `RNG_CONFIG2` reader - RNG configuration 2"]
pub struct RNG_CONFIG2_R(crate::FieldReader<u8, u8>);
impl RNG_CONFIG2_R {
    pub(crate) fn new(bits: u8) -> Self {
        RNG_CONFIG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNG_CONFIG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNG_CONFIG2` writer - RNG configuration 2"]
pub struct RNG_CONFIG2_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG_CONFIG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
#[doc = "Field `CLKDIV` reader - Clock divider factor"]
pub struct CLKDIV_R(crate::FieldReader<u8, u8>);
impl CLKDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKDIV` writer - Clock divider factor"]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `RNG_CONFIG1` reader - RNG configuration 1"]
pub struct RNG_CONFIG1_R(crate::FieldReader<u8, u8>);
impl RNG_CONFIG1_R {
    pub(crate) fn new(bits: u8) -> Self {
        RNG_CONFIG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNG_CONFIG1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNG_CONFIG1` writer - RNG configuration 1"]
pub struct RNG_CONFIG1_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG_CONFIG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 20)) | ((value as u32 & 0x3f) << 20);
        self.w
    }
}
#[doc = "Field `CONDRST` reader - Conditioning soft reset"]
pub struct CONDRST_R(crate::FieldReader<bool, bool>);
impl CONDRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONDRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONDRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONDRST` writer - Conditioning soft reset"]
pub struct CONDRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CONDRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `CONFIGLOCK` reader - RNG Config Lock"]
pub struct CONFIGLOCK_R(crate::FieldReader<bool, bool>);
impl CONFIGLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONFIGLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONFIGLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONFIGLOCK` writer - RNG Config Lock"]
pub struct CONFIGLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFIGLOCK_W<'a> {
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
    #[doc = "Bit 2 - Random number generator enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clock error detection Note: The clock error detection can be used only when ck_rc48 or ck_pll1_q (ck_pll1_q = 48MHz) source is selected otherwise, CED bit must be equal to 1. The clock error detection cannot be enabled nor disabled on the fly when RNG peripheral is enabled, to enable or disable CED the RNG must be disabled."]
    #[inline(always)]
    pub fn ced(&self) -> CED_R {
        CED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - RNG configuration 3"]
    #[inline(always)]
    pub fn rng_config3(&self) -> RNG_CONFIG3_R {
        RNG_CONFIG3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Non NIST compliant"]
    #[inline(always)]
    pub fn nistc(&self) -> NISTC_R {
        NISTC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - RNG configuration 2"]
    #[inline(always)]
    pub fn rng_config2(&self) -> RNG_CONFIG2_R {
        RNG_CONFIG2_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 16:19 - Clock divider factor"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:25 - RNG configuration 1"]
    #[inline(always)]
    pub fn rng_config1(&self) -> RNG_CONFIG1_R {
        RNG_CONFIG1_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Conditioning soft reset"]
    #[inline(always)]
    pub fn condrst(&self) -> CONDRST_R {
        CONDRST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - RNG Config Lock"]
    #[inline(always)]
    pub fn configlock(&self) -> CONFIGLOCK_R {
        CONFIGLOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Random number generator enable"]
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W {
        RNGEN_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bit 5 - Clock error detection Note: The clock error detection can be used only when ck_rc48 or ck_pll1_q (ck_pll1_q = 48MHz) source is selected otherwise, CED bit must be equal to 1. The clock error detection cannot be enabled nor disabled on the fly when RNG peripheral is enabled, to enable or disable CED the RNG must be disabled."]
    #[inline(always)]
    pub fn ced(&mut self) -> CED_W {
        CED_W { w: self }
    }
    #[doc = "Bits 8:11 - RNG configuration 3"]
    #[inline(always)]
    pub fn rng_config3(&mut self) -> RNG_CONFIG3_W {
        RNG_CONFIG3_W { w: self }
    }
    #[doc = "Bit 12 - Non NIST compliant"]
    #[inline(always)]
    pub fn nistc(&mut self) -> NISTC_W {
        NISTC_W { w: self }
    }
    #[doc = "Bits 13:15 - RNG configuration 2"]
    #[inline(always)]
    pub fn rng_config2(&mut self) -> RNG_CONFIG2_W {
        RNG_CONFIG2_W { w: self }
    }
    #[doc = "Bits 16:19 - Clock divider factor"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
    #[doc = "Bits 20:25 - RNG configuration 1"]
    #[inline(always)]
    pub fn rng_config1(&mut self) -> RNG_CONFIG1_W {
        RNG_CONFIG1_W { w: self }
    }
    #[doc = "Bit 30 - Conditioning soft reset"]
    #[inline(always)]
    pub fn condrst(&mut self) -> CONDRST_W {
        CONDRST_W { w: self }
    }
    #[doc = "Bit 31 - RNG Config Lock"]
    #[inline(always)]
    pub fn configlock(&mut self) -> CONFIGLOCK_W {
        CONFIGLOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RNG control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_cr](index.html) module"]
pub struct RNG_CR_SPEC;
impl crate::RegisterSpec for RNG_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rng_cr::R](R) reader structure"]
impl crate::Readable for RNG_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rng_cr::W](W) writer structure"]
impl crate::Writable for RNG_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RNG_CR to value 0"]
impl crate::Resettable for RNG_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
