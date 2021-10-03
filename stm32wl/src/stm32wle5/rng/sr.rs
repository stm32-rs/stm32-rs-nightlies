#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Seed error interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEIS_A {
    #[doc = "0: No faulty sequence detected"]
    NOFAULT = 0,
    #[doc = "1: At least one faulty sequence has been detected"]
    FAULT = 1,
}
impl From<SEIS_A> for bool {
    #[inline(always)]
    fn from(variant: SEIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEIS` reader - Seed error interrupt status"]
pub struct SEIS_R(crate::FieldReader<bool, SEIS_A>);
impl SEIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEIS_A {
        match self.bits {
            false => SEIS_A::NOFAULT,
            true => SEIS_A::FAULT,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAULT`"]
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        **self == SEIS_A::NOFAULT
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        **self == SEIS_A::FAULT
    }
}
impl core::ops::Deref for SEIS_R {
    type Target = crate::FieldReader<bool, SEIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEIS` writer - Seed error interrupt status"]
pub struct SEIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SEIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No faulty sequence detected"]
    #[inline(always)]
    pub fn no_fault(self) -> &'a mut W {
        self.variant(SEIS_A::NOFAULT)
    }
    #[doc = "At least one faulty sequence has been detected"]
    #[inline(always)]
    pub fn fault(self) -> &'a mut W {
        self.variant(SEIS_A::FAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Clock error interrupt status"]
pub type CEIS_A = CECS_A;
#[doc = "Field `CEIS` reader - Clock error interrupt status"]
pub type CEIS_R = CECS_R;
#[doc = "Field `CEIS` writer - Clock error interrupt status"]
pub struct CEIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The RNG clock is correct (fRNGCLK> fHCLK/32)"]
    #[inline(always)]
    pub fn correct(self) -> &'a mut W {
        self.variant(CEIS_A::CORRECT)
    }
    #[doc = "The RNG clock before internal divider has been detected too slow (fRNGCLK< fHCLK/32)"]
    #[inline(always)]
    pub fn slow(self) -> &'a mut W {
        self.variant(CEIS_A::SLOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Seed error current status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECS_A {
    #[doc = "0: No faulty sequence has currently been detected. If the SEIS bit is set, this means that a faulty sequence was detected and the situation has been recovered"]
    NOFAULT = 0,
    #[doc = "1: At least one faulty sequence has been detected - see ref manual for details"]
    FAULT = 1,
}
impl From<SECS_A> for bool {
    #[inline(always)]
    fn from(variant: SECS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECS` reader - Seed error current status"]
pub struct SECS_R(crate::FieldReader<bool, SECS_A>);
impl SECS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECS_A {
        match self.bits {
            false => SECS_A::NOFAULT,
            true => SECS_A::FAULT,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAULT`"]
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        **self == SECS_A::NOFAULT
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        **self == SECS_A::FAULT
    }
}
impl core::ops::Deref for SECS_R {
    type Target = crate::FieldReader<bool, SECS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Clock error current status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CECS_A {
    #[doc = "0: The RNG clock is correct (fRNGCLK> fHCLK/32)"]
    CORRECT = 0,
    #[doc = "1: The RNG clock before internal divider has been detected too slow (fRNGCLK< fHCLK/32)"]
    SLOW = 1,
}
impl From<CECS_A> for bool {
    #[inline(always)]
    fn from(variant: CECS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CECS` reader - Clock error current status"]
pub struct CECS_R(crate::FieldReader<bool, CECS_A>);
impl CECS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CECS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CECS_A {
        match self.bits {
            false => CECS_A::CORRECT,
            true => CECS_A::SLOW,
        }
    }
    #[doc = "Checks if the value of the field is `CORRECT`"]
    #[inline(always)]
    pub fn is_correct(&self) -> bool {
        **self == CECS_A::CORRECT
    }
    #[doc = "Checks if the value of the field is `SLOW`"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        **self == CECS_A::SLOW
    }
}
impl core::ops::Deref for CECS_R {
    type Target = crate::FieldReader<bool, CECS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Data Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRDY_A {
    #[doc = "0: The RNG_DR register is not yet valid, no random data is available"]
    INVALID = 0,
    #[doc = "1: The RNG_DR register contains valid random data"]
    VALID = 1,
}
impl From<DRDY_A> for bool {
    #[inline(always)]
    fn from(variant: DRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRDY` reader - Data Ready"]
pub struct DRDY_R(crate::FieldReader<bool, DRDY_A>);
impl DRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        DRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRDY_A {
        match self.bits {
            false => DRDY_A::INVALID,
            true => DRDY_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `INVALID`"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        **self == DRDY_A::INVALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        **self == DRDY_A::VALID
    }
}
impl core::ops::Deref for DRDY_R {
    type Target = crate::FieldReader<bool, DRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 6 - Seed error interrupt status"]
    #[inline(always)]
    pub fn seis(&self) -> SEIS_R {
        SEIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clock error interrupt status"]
    #[inline(always)]
    pub fn ceis(&self) -> CEIS_R {
        CEIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Seed error current status"]
    #[inline(always)]
    pub fn secs(&self) -> SECS_R {
        SECS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock error current status"]
    #[inline(always)]
    pub fn cecs(&self) -> CECS_R {
        CECS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Data Ready"]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Seed error interrupt status"]
    #[inline(always)]
    pub fn seis(&mut self) -> SEIS_W {
        SEIS_W { w: self }
    }
    #[doc = "Bit 5 - Clock error interrupt status"]
    #[inline(always)]
    pub fn ceis(&mut self) -> CEIS_W {
        CEIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
