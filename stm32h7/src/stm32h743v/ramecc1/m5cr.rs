#[doc = "Register `M5CR` reader"]
pub struct R(crate::R<M5CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M5CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M5CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M5CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M5CR` writer"]
pub struct W(crate::W<M5CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M5CR_SPEC>;
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
impl From<crate::W<M5CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M5CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECCELEN` reader - ECC error context latching enable"]
pub struct ECCELEN_R(crate::FieldReader<bool, bool>);
impl ECCELEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCELEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCELEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCELEN` writer - ECC error context latching enable"]
pub struct ECCELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCELEN_W<'a> {
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
#[doc = "Field `ECCDEBWIE` reader - ECC double error on byte write interrupt enable"]
pub struct ECCDEBWIE_R(crate::FieldReader<bool, bool>);
impl ECCDEBWIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCDEBWIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCDEBWIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCDEBWIE` writer - ECC double error on byte write interrupt enable"]
pub struct ECCDEBWIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCDEBWIE_W<'a> {
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
#[doc = "Field `ECCDEIE` reader - ECC double error interrupt enable"]
pub struct ECCDEIE_R(crate::FieldReader<bool, bool>);
impl ECCDEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCDEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCDEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCDEIE` writer - ECC double error interrupt enable"]
pub struct ECCDEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCDEIE_W<'a> {
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
#[doc = "Field `ECCSEIE` reader - ECC single error interrupt enable"]
pub struct ECCSEIE_R(crate::FieldReader<bool, bool>);
impl ECCSEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCSEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCSEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCSEIE` writer - ECC single error interrupt enable"]
pub struct ECCSEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCSEIE_W<'a> {
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
impl R {
    #[doc = "Bit 5 - ECC error context latching enable"]
    #[inline(always)]
    pub fn eccelen(&self) -> ECCELEN_R {
        ECCELEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ECC double error on byte write interrupt enable"]
    #[inline(always)]
    pub fn eccdebwie(&self) -> ECCDEBWIE_R {
        ECCDEBWIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ECC double error interrupt enable"]
    #[inline(always)]
    pub fn eccdeie(&self) -> ECCDEIE_R {
        ECCDEIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ECC single error interrupt enable"]
    #[inline(always)]
    pub fn eccseie(&self) -> ECCSEIE_R {
        ECCSEIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - ECC error context latching enable"]
    #[inline(always)]
    pub fn eccelen(&mut self) -> ECCELEN_W {
        ECCELEN_W { w: self }
    }
    #[doc = "Bit 4 - ECC double error on byte write interrupt enable"]
    #[inline(always)]
    pub fn eccdebwie(&mut self) -> ECCDEBWIE_W {
        ECCDEBWIE_W { w: self }
    }
    #[doc = "Bit 3 - ECC double error interrupt enable"]
    #[inline(always)]
    pub fn eccdeie(&mut self) -> ECCDEIE_W {
        ECCDEIE_W { w: self }
    }
    #[doc = "Bit 2 - ECC single error interrupt enable"]
    #[inline(always)]
    pub fn eccseie(&mut self) -> ECCSEIE_W {
        ECCSEIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAMECC monitor 5 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m5cr](index.html) module"]
pub struct M5CR_SPEC;
impl crate::RegisterSpec for M5CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m5cr::R](R) reader structure"]
impl crate::Readable for M5CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m5cr::W](W) writer structure"]
impl crate::Writable for M5CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M5CR to value 0"]
impl crate::Resettable for M5CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
