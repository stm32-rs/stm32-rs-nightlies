#[doc = "Register `RCC_MP_MLAHBLPENSETR` reader"]
pub struct R(crate::R<RCC_MP_MLAHBLPENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_MLAHBLPENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_MLAHBLPENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_MLAHBLPENSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_MLAHBLPENSETR` writer"]
pub struct W(crate::W<RCC_MP_MLAHBLPENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_MLAHBLPENSETR_SPEC>;
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
impl From<crate::W<RCC_MP_MLAHBLPENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_MLAHBLPENSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM1LPEN` reader - SRAM1LPEN"]
pub struct SRAM1LPEN_R(crate::FieldReader<bool, bool>);
impl SRAM1LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM1LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM1LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM1LPEN` writer - SRAM1LPEN"]
pub struct SRAM1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1LPEN_W<'a> {
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
#[doc = "Field `SRAM2LPEN` reader - SRAM2LPEN"]
pub struct SRAM2LPEN_R(crate::FieldReader<bool, bool>);
impl SRAM2LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM2LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM2LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM2LPEN` writer - SRAM2LPEN"]
pub struct SRAM2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2LPEN_W<'a> {
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
#[doc = "Field `SRAM34LPEN` reader - SRAM34LPEN"]
pub struct SRAM34LPEN_R(crate::FieldReader<bool, bool>);
impl SRAM34LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM34LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM34LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM34LPEN` writer - SRAM34LPEN"]
pub struct SRAM34LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM34LPEN_W<'a> {
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
#[doc = "Field `RETRAMLPEN` reader - RETRAMLPEN"]
pub struct RETRAMLPEN_R(crate::FieldReader<bool, bool>);
impl RETRAMLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RETRAMLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RETRAMLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RETRAMLPEN` writer - RETRAMLPEN"]
pub struct RETRAMLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRAMLPEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - SRAM1LPEN"]
    #[inline(always)]
    pub fn sram1lpen(&self) -> SRAM1LPEN_R {
        SRAM1LPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SRAM2LPEN"]
    #[inline(always)]
    pub fn sram2lpen(&self) -> SRAM2LPEN_R {
        SRAM2LPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SRAM34LPEN"]
    #[inline(always)]
    pub fn sram34lpen(&self) -> SRAM34LPEN_R {
        SRAM34LPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RETRAMLPEN"]
    #[inline(always)]
    pub fn retramlpen(&self) -> RETRAMLPEN_R {
        RETRAMLPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM1LPEN"]
    #[inline(always)]
    pub fn sram1lpen(&mut self) -> SRAM1LPEN_W {
        SRAM1LPEN_W { w: self }
    }
    #[doc = "Bit 1 - SRAM2LPEN"]
    #[inline(always)]
    pub fn sram2lpen(&mut self) -> SRAM2LPEN_W {
        SRAM2LPEN_W { w: self }
    }
    #[doc = "Bit 2 - SRAM34LPEN"]
    #[inline(always)]
    pub fn sram34lpen(&mut self) -> SRAM34LPEN_W {
        SRAM34LPEN_W { w: self }
    }
    #[doc = "Bit 4 - RETRAMLPEN"]
    #[inline(always)]
    pub fn retramlpen(&mut self) -> RETRAMLPEN_W {
        RETRAMLPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by the MPU in order to set the PERxLPEN bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_mlahblpensetr](index.html) module"]
pub struct RCC_MP_MLAHBLPENSETR_SPEC;
impl crate::RegisterSpec for RCC_MP_MLAHBLPENSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_mlahblpensetr::R](R) reader structure"]
impl crate::Readable for RCC_MP_MLAHBLPENSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_mlahblpensetr::W](W) writer structure"]
impl crate::Writable for RCC_MP_MLAHBLPENSETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MP_MLAHBLPENSETR to value 0x17"]
impl crate::Resettable for RCC_MP_MLAHBLPENSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x17
    }
}
