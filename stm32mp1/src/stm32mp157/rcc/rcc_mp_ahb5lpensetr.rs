#[doc = "Register `RCC_MP_AHB5LPENSETR` reader"]
pub struct R(crate::R<RCC_MP_AHB5LPENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_AHB5LPENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_AHB5LPENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_AHB5LPENSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_AHB5LPENSETR` writer"]
pub struct W(crate::W<RCC_MP_AHB5LPENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_AHB5LPENSETR_SPEC>;
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
impl From<crate::W<RCC_MP_AHB5LPENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_AHB5LPENSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOZLPEN` reader - GPIOZLPEN"]
pub struct GPIOZLPEN_R(crate::FieldReader<bool, bool>);
impl GPIOZLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOZLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOZLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOZLPEN` writer - GPIOZLPEN"]
pub struct GPIOZLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOZLPEN_W<'a> {
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
#[doc = "Field `CRYP1LPEN` reader - CRYP1LPEN"]
pub struct CRYP1LPEN_R(crate::FieldReader<bool, bool>);
impl CRYP1LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRYP1LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYP1LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRYP1LPEN` writer - CRYP1LPEN"]
pub struct CRYP1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYP1LPEN_W<'a> {
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
#[doc = "Field `HASH1LPEN` reader - HASH1LPEN"]
pub struct HASH1LPEN_R(crate::FieldReader<bool, bool>);
impl HASH1LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HASH1LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HASH1LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HASH1LPEN` writer - HASH1LPEN"]
pub struct HASH1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH1LPEN_W<'a> {
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
#[doc = "Field `RNG1LPEN` reader - RNG1LPEN"]
pub struct RNG1LPEN_R(crate::FieldReader<bool, bool>);
impl RNG1LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNG1LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNG1LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNG1LPEN` writer - RNG1LPEN"]
pub struct RNG1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG1LPEN_W<'a> {
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
#[doc = "Field `BKPSRAMLPEN` reader - BKPSRAMLPEN"]
pub struct BKPSRAMLPEN_R(crate::FieldReader<bool, bool>);
impl BKPSRAMLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKPSRAMLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKPSRAMLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKPSRAMLPEN` writer - BKPSRAMLPEN"]
pub struct BKPSRAMLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BKPSRAMLPEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - GPIOZLPEN"]
    #[inline(always)]
    pub fn gpiozlpen(&self) -> GPIOZLPEN_R {
        GPIOZLPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRYP1LPEN"]
    #[inline(always)]
    pub fn cryp1lpen(&self) -> CRYP1LPEN_R {
        CRYP1LPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HASH1LPEN"]
    #[inline(always)]
    pub fn hash1lpen(&self) -> HASH1LPEN_R {
        HASH1LPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RNG1LPEN"]
    #[inline(always)]
    pub fn rng1lpen(&self) -> RNG1LPEN_R {
        RNG1LPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BKPSRAMLPEN"]
    #[inline(always)]
    pub fn bkpsramlpen(&self) -> BKPSRAMLPEN_R {
        BKPSRAMLPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOZLPEN"]
    #[inline(always)]
    pub fn gpiozlpen(&mut self) -> GPIOZLPEN_W {
        GPIOZLPEN_W { w: self }
    }
    #[doc = "Bit 4 - CRYP1LPEN"]
    #[inline(always)]
    pub fn cryp1lpen(&mut self) -> CRYP1LPEN_W {
        CRYP1LPEN_W { w: self }
    }
    #[doc = "Bit 5 - HASH1LPEN"]
    #[inline(always)]
    pub fn hash1lpen(&mut self) -> HASH1LPEN_W {
        HASH1LPEN_W { w: self }
    }
    #[doc = "Bit 6 - RNG1LPEN"]
    #[inline(always)]
    pub fn rng1lpen(&mut self) -> RNG1LPEN_W {
        RNG1LPEN_W { w: self }
    }
    #[doc = "Bit 8 - BKPSRAMLPEN"]
    #[inline(always)]
    pub fn bkpsramlpen(&mut self) -> BKPSRAMLPEN_W {
        BKPSRAMLPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb5lpensetr](index.html) module"]
pub struct RCC_MP_AHB5LPENSETR_SPEC;
impl crate::RegisterSpec for RCC_MP_AHB5LPENSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_ahb5lpensetr::R](R) reader structure"]
impl crate::Readable for RCC_MP_AHB5LPENSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb5lpensetr::W](W) writer structure"]
impl crate::Writable for RCC_MP_AHB5LPENSETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MP_AHB5LPENSETR to value 0x0171"]
impl crate::Resettable for RCC_MP_AHB5LPENSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0171
    }
}
