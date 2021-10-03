#[doc = "Register `RCC_MC_AHB5ENSETR` reader"]
pub struct R(crate::R<RCC_MC_AHB5ENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_AHB5ENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_AHB5ENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_AHB5ENSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MC_AHB5ENSETR` writer"]
pub struct W(crate::W<RCC_MC_AHB5ENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_AHB5ENSETR_SPEC>;
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
impl From<crate::W<RCC_MC_AHB5ENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_AHB5ENSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOZEN` reader - GPIOZEN"]
pub struct GPIOZEN_R(crate::FieldReader<bool, bool>);
impl GPIOZEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOZEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOZEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOZEN` writer - GPIOZEN"]
pub struct GPIOZEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOZEN_W<'a> {
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
#[doc = "Field `CRYP1EN` reader - CRYP1EN"]
pub struct CRYP1EN_R(crate::FieldReader<bool, bool>);
impl CRYP1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRYP1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYP1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRYP1EN` writer - CRYP1EN"]
pub struct CRYP1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYP1EN_W<'a> {
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
#[doc = "Field `HASH1EN` reader - HASH1EN"]
pub struct HASH1EN_R(crate::FieldReader<bool, bool>);
impl HASH1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HASH1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HASH1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HASH1EN` writer - HASH1EN"]
pub struct HASH1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH1EN_W<'a> {
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
#[doc = "Field `RNG1EN` reader - RNG1EN"]
pub struct RNG1EN_R(crate::FieldReader<bool, bool>);
impl RNG1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNG1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNG1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNG1EN` writer - RNG1EN"]
pub struct RNG1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG1EN_W<'a> {
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
#[doc = "Field `BKPSRAMEN` reader - BKPSRAMEN"]
pub struct BKPSRAMEN_R(crate::FieldReader<bool, bool>);
impl BKPSRAMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKPSRAMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKPSRAMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKPSRAMEN` writer - BKPSRAMEN"]
pub struct BKPSRAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BKPSRAMEN_W<'a> {
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
    #[doc = "Bit 0 - GPIOZEN"]
    #[inline(always)]
    pub fn gpiozen(&self) -> GPIOZEN_R {
        GPIOZEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRYP1EN"]
    #[inline(always)]
    pub fn cryp1en(&self) -> CRYP1EN_R {
        CRYP1EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HASH1EN"]
    #[inline(always)]
    pub fn hash1en(&self) -> HASH1EN_R {
        HASH1EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RNG1EN"]
    #[inline(always)]
    pub fn rng1en(&self) -> RNG1EN_R {
        RNG1EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BKPSRAMEN"]
    #[inline(always)]
    pub fn bkpsramen(&self) -> BKPSRAMEN_R {
        BKPSRAMEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOZEN"]
    #[inline(always)]
    pub fn gpiozen(&mut self) -> GPIOZEN_W {
        GPIOZEN_W { w: self }
    }
    #[doc = "Bit 4 - CRYP1EN"]
    #[inline(always)]
    pub fn cryp1en(&mut self) -> CRYP1EN_W {
        CRYP1EN_W { w: self }
    }
    #[doc = "Bit 5 - HASH1EN"]
    #[inline(always)]
    pub fn hash1en(&mut self) -> HASH1EN_W {
        HASH1EN_W { w: self }
    }
    #[doc = "Bit 6 - RNG1EN"]
    #[inline(always)]
    pub fn rng1en(&mut self) -> RNG1EN_W {
        RNG1EN_W { w: self }
    }
    #[doc = "Bit 8 - BKPSRAMEN"]
    #[inline(always)]
    pub fn bkpsramen(&mut self) -> BKPSRAMEN_W {
        BKPSRAMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to set the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb5ensetr](index.html) module"]
pub struct RCC_MC_AHB5ENSETR_SPEC;
impl crate::RegisterSpec for RCC_MC_AHB5ENSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mc_ahb5ensetr::R](R) reader structure"]
impl crate::Readable for RCC_MC_AHB5ENSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb5ensetr::W](W) writer structure"]
impl crate::Writable for RCC_MC_AHB5ENSETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MC_AHB5ENSETR to value 0"]
impl crate::Resettable for RCC_MC_AHB5ENSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
