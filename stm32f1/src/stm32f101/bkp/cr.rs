#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Tamper pin enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPE_A {
    #[doc = "0: The TAMPER pin is free for general purpose I/O"]
    GENERAL = 0,
    #[doc = "1: Tamper alternate I/O function is activated"]
    ALTERNATE = 1,
}
impl From<TPE_A> for bool {
    #[inline(always)]
    fn from(variant: TPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPE` reader - Tamper pin enable"]
pub struct TPE_R(crate::FieldReader<bool, TPE_A>);
impl TPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPE_A {
        match self.bits {
            false => TPE_A::GENERAL,
            true => TPE_A::ALTERNATE,
        }
    }
    #[doc = "Checks if the value of the field is `GENERAL`"]
    #[inline(always)]
    pub fn is_general(&self) -> bool {
        **self == TPE_A::GENERAL
    }
    #[doc = "Checks if the value of the field is `ALTERNATE`"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        **self == TPE_A::ALTERNATE
    }
}
impl core::ops::Deref for TPE_R {
    type Target = crate::FieldReader<bool, TPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPE` writer - Tamper pin enable"]
pub struct TPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The TAMPER pin is free for general purpose I/O"]
    #[inline(always)]
    pub fn general(self) -> &'a mut W {
        self.variant(TPE_A::GENERAL)
    }
    #[doc = "Tamper alternate I/O function is activated"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(TPE_A::ALTERNATE)
    }
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
#[doc = "Tamper pin active level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPAL_A {
    #[doc = "0: A high level on the TAMPER pin resets all data backup registers (if TPE bit is set)"]
    HIGH = 0,
    #[doc = "1: A low level on the TAMPER pin resets all data backup registers (if TPE bit is set)"]
    LOW = 1,
}
impl From<TPAL_A> for bool {
    #[inline(always)]
    fn from(variant: TPAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPAL` reader - Tamper pin active level"]
pub struct TPAL_R(crate::FieldReader<bool, TPAL_A>);
impl TPAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TPAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPAL_A {
        match self.bits {
            false => TPAL_A::HIGH,
            true => TPAL_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == TPAL_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == TPAL_A::LOW
    }
}
impl core::ops::Deref for TPAL_R {
    type Target = crate::FieldReader<bool, TPAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPAL` writer - Tamper pin active level"]
pub struct TPAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TPAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A high level on the TAMPER pin resets all data backup registers (if TPE bit is set)"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(TPAL_A::HIGH)
    }
    #[doc = "A low level on the TAMPER pin resets all data backup registers (if TPE bit is set)"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(TPAL_A::LOW)
    }
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
impl R {
    #[doc = "Bit 0 - Tamper pin enable"]
    #[inline(always)]
    pub fn tpe(&self) -> TPE_R {
        TPE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Tamper pin active level"]
    #[inline(always)]
    pub fn tpal(&self) -> TPAL_R {
        TPAL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper pin enable"]
    #[inline(always)]
    pub fn tpe(&mut self) -> TPE_W {
        TPE_W { w: self }
    }
    #[doc = "Bit 1 - Tamper pin active level"]
    #[inline(always)]
    pub fn tpal(&mut self) -> TPAL_W {
        TPAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup control register (BKP_CR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
