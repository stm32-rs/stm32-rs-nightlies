#[doc = "Register `RCC_AHB5RSTSETR` reader"]
pub struct R(crate::R<RCC_AHB5RSTSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB5RSTSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB5RSTSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB5RSTSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_AHB5RSTSETR` writer"]
pub struct W(crate::W<RCC_AHB5RSTSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB5RSTSETR_SPEC>;
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
impl From<crate::W<RCC_AHB5RSTSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB5RSTSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOZRST` reader - GPIOZRST"]
pub struct GPIOZRST_R(crate::FieldReader<bool, bool>);
impl GPIOZRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOZRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOZRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOZRST` writer - GPIOZRST"]
pub struct GPIOZRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOZRST_W<'a> {
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
#[doc = "Field `CRYP1RST` reader - CRYP1RST"]
pub struct CRYP1RST_R(crate::FieldReader<bool, bool>);
impl CRYP1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRYP1RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYP1RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRYP1RST` writer - CRYP1RST"]
pub struct CRYP1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYP1RST_W<'a> {
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
#[doc = "Field `HASH1RST` reader - HASH1RST"]
pub struct HASH1RST_R(crate::FieldReader<bool, bool>);
impl HASH1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        HASH1RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HASH1RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HASH1RST` writer - HASH1RST"]
pub struct HASH1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH1RST_W<'a> {
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
#[doc = "Field `RNG1RST` reader - RNG1RST"]
pub struct RNG1RST_R(crate::FieldReader<bool, bool>);
impl RNG1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNG1RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNG1RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNG1RST` writer - RNG1RST"]
pub struct RNG1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG1RST_W<'a> {
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
#[doc = "Field `AXIMCRST` reader - AXIMCRST"]
pub struct AXIMCRST_R(crate::FieldReader<bool, bool>);
impl AXIMCRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        AXIMCRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AXIMCRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AXIMCRST` writer - AXIMCRST"]
pub struct AXIMCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIMCRST_W<'a> {
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
impl R {
    #[doc = "Bit 0 - GPIOZRST"]
    #[inline(always)]
    pub fn gpiozrst(&self) -> GPIOZRST_R {
        GPIOZRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRYP1RST"]
    #[inline(always)]
    pub fn cryp1rst(&self) -> CRYP1RST_R {
        CRYP1RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HASH1RST"]
    #[inline(always)]
    pub fn hash1rst(&self) -> HASH1RST_R {
        HASH1RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RNG1RST"]
    #[inline(always)]
    pub fn rng1rst(&self) -> RNG1RST_R {
        RNG1RST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 16 - AXIMCRST"]
    #[inline(always)]
    pub fn aximcrst(&self) -> AXIMCRST_R {
        AXIMCRST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOZRST"]
    #[inline(always)]
    pub fn gpiozrst(&mut self) -> GPIOZRST_W {
        GPIOZRST_W { w: self }
    }
    #[doc = "Bit 4 - CRYP1RST"]
    #[inline(always)]
    pub fn cryp1rst(&mut self) -> CRYP1RST_W {
        CRYP1RST_W { w: self }
    }
    #[doc = "Bit 5 - HASH1RST"]
    #[inline(always)]
    pub fn hash1rst(&mut self) -> HASH1RST_W {
        HASH1RST_W { w: self }
    }
    #[doc = "Bit 6 - RNG1RST"]
    #[inline(always)]
    pub fn rng1rst(&mut self) -> RNG1RST_W {
        RNG1RST_W { w: self }
    }
    #[doc = "Bit 16 - AXIMCRST"]
    #[inline(always)]
    pub fn aximcrst(&mut self) -> AXIMCRST_W {
        AXIMCRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb5rstsetr](index.html) module"]
pub struct RCC_AHB5RSTSETR_SPEC;
impl crate::RegisterSpec for RCC_AHB5RSTSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ahb5rstsetr::R](R) reader structure"]
impl crate::Readable for RCC_AHB5RSTSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ahb5rstsetr::W](W) writer structure"]
impl crate::Writable for RCC_AHB5RSTSETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_AHB5RSTSETR to value 0"]
impl crate::Resettable for RCC_AHB5RSTSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
