#[doc = "Register `C2AHB3SMENR` reader"]
pub struct R(crate::R<C2AHB3SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2AHB3SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2AHB3SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2AHB3SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2AHB3SMENR` writer"]
pub struct W(crate::W<C2AHB3SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2AHB3SMENR_SPEC>;
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
impl From<crate::W<C2AHB3SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2AHB3SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASHSMEN` reader - Flash interface clocks enable during CPU2 sleep modes"]
pub struct FLASHSMEN_R(crate::FieldReader<bool, bool>);
impl FLASHSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASHSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASHSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASHSMEN` writer - Flash interface clocks enable during CPU2 sleep modes"]
pub struct FLASHSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHSMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `SRAM2SMEN` reader - SRAM2a and SRAM2b memory interface clocks enable during CPU2 sleep modes"]
pub struct SRAM2SMEN_R(crate::FieldReader<bool, bool>);
impl SRAM2SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM2SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM2SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM2SMEN` writer - SRAM2a and SRAM2b memory interface clocks enable during CPU2 sleep modes"]
pub struct SRAM2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `RNGSMEN` reader - True RNG clocks enable during CPU2 sleep modes"]
pub struct RNGSMEN_R(crate::FieldReader<bool, bool>);
impl RNGSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNGSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNGSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNGSMEN` writer - True RNG clocks enable during CPU2 sleep modes"]
pub struct RNGSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGSMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `AES2SMEN` reader - AES2 accelerator clocks enable during CPU2 sleep modes"]
pub struct AES2SMEN_R(crate::FieldReader<bool, bool>);
impl AES2SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AES2SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AES2SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AES2SMEN` writer - AES2 accelerator clocks enable during CPU2 sleep modes"]
pub struct AES2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AES2SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `PKASMEN` reader - PKA accelerator clocks enable during CPU2 sleep modes"]
pub struct PKASMEN_R(crate::FieldReader<bool, bool>);
impl PKASMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKASMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKASMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKASMEN` writer - PKA accelerator clocks enable during CPU2 sleep modes"]
pub struct PKASMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PKASMEN_W<'a> {
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
    #[doc = "Bit 25 - Flash interface clocks enable during CPU2 sleep modes"]
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - SRAM2a and SRAM2b memory interface clocks enable during CPU2 sleep modes"]
    #[inline(always)]
    pub fn sram2smen(&self) -> SRAM2SMEN_R {
        SRAM2SMEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 18 - True RNG clocks enable during CPU2 sleep modes"]
    #[inline(always)]
    pub fn rngsmen(&self) -> RNGSMEN_R {
        RNGSMEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - AES2 accelerator clocks enable during CPU2 sleep modes"]
    #[inline(always)]
    pub fn aes2smen(&self) -> AES2SMEN_R {
        AES2SMEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PKA accelerator clocks enable during CPU2 sleep modes"]
    #[inline(always)]
    pub fn pkasmen(&self) -> PKASMEN_R {
        PKASMEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - Flash interface clocks enable during CPU2 sleep modes"]
    #[inline(always)]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W {
        FLASHSMEN_W { w: self }
    }
    #[doc = "Bit 24 - SRAM2a and SRAM2b memory interface clocks enable during CPU2 sleep modes"]
    #[inline(always)]
    pub fn sram2smen(&mut self) -> SRAM2SMEN_W {
        SRAM2SMEN_W { w: self }
    }
    #[doc = "Bit 18 - True RNG clocks enable during CPU2 sleep modes"]
    #[inline(always)]
    pub fn rngsmen(&mut self) -> RNGSMEN_W {
        RNGSMEN_W { w: self }
    }
    #[doc = "Bit 17 - AES2 accelerator clocks enable during CPU2 sleep modes"]
    #[inline(always)]
    pub fn aes2smen(&mut self) -> AES2SMEN_W {
        AES2SMEN_W { w: self }
    }
    #[doc = "Bit 16 - PKA accelerator clocks enable during CPU2 sleep modes"]
    #[inline(always)]
    pub fn pkasmen(&mut self) -> PKASMEN_W {
        PKASMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 AHB3 peripheral clocks enable in Sleep and Stop modes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2ahb3smenr](index.html) module"]
pub struct C2AHB3SMENR_SPEC;
impl crate::RegisterSpec for C2AHB3SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2ahb3smenr::R](R) reader structure"]
impl crate::Readable for C2AHB3SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2ahb3smenr::W](W) writer structure"]
impl crate::Writable for C2AHB3SMENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2AHB3SMENR to value 0x0307_0000"]
impl crate::Resettable for C2AHB3SMENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0307_0000
    }
}