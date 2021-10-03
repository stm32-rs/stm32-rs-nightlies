#[doc = "Register `SCSR` reader"]
pub struct R(crate::R<SCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCSR` writer"]
pub struct W(crate::W<SCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCSR_SPEC>;
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
impl From<crate::W<SCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PKA SRAM busy by erase operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PKASRAMBSY_A {
    #[doc = "0: No PKA SRAM erase operation is ongoing"]
    IDLE = 0,
    #[doc = "1: PKA SRAM erase operation is ongoing"]
    BUSY = 1,
}
impl From<PKASRAMBSY_A> for bool {
    #[inline(always)]
    fn from(variant: PKASRAMBSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PKASRAMBSY` reader - PKA SRAM busy by erase operation"]
pub struct PKASRAMBSY_R(crate::FieldReader<bool, PKASRAMBSY_A>);
impl PKASRAMBSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKASRAMBSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PKASRAMBSY_A {
        match self.bits {
            false => PKASRAMBSY_A::IDLE,
            true => PKASRAMBSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == PKASRAMBSY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == PKASRAMBSY_A::BUSY
    }
}
impl core::ops::Deref for PKASRAMBSY_R {
    type Target = crate::FieldReader<bool, PKASRAMBSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SRAM1, SRAM2 and PKA SRAM busy by erase operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAMBSY_A {
    #[doc = "0: No SRAM1 or SRAM2 erase operation is ongoing"]
    IDLE = 0,
    #[doc = "1: SRAM1 or SRAM2 erase operation is ongoing"]
    BUSY = 1,
}
impl From<SRAMBSY_A> for bool {
    #[inline(always)]
    fn from(variant: SRAMBSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAMBSY` reader - SRAM1, SRAM2 and PKA SRAM busy by erase operation"]
pub struct SRAMBSY_R(crate::FieldReader<bool, SRAMBSY_A>);
impl SRAMBSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAMBSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAMBSY_A {
        match self.bits {
            false => SRAMBSY_A::IDLE,
            true => SRAMBSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == SRAMBSY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == SRAMBSY_A::BUSY
    }
}
impl core::ops::Deref for SRAMBSY_R {
    type Target = crate::FieldReader<bool, SRAMBSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SRAM2 erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM2ER_A {
    #[doc = "1: Start SRAM2 erase operation"]
    ERASE = 1,
}
impl From<SRAM2ER_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM2ER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM2ER` reader - SRAM2 erase"]
pub struct SRAM2ER_R(crate::FieldReader<bool, SRAM2ER_A>);
impl SRAM2ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM2ER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRAM2ER_A> {
        match self.bits {
            true => Some(SRAM2ER_A::ERASE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ERASE`"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        **self == SRAM2ER_A::ERASE
    }
}
impl core::ops::Deref for SRAM2ER_R {
    type Target = crate::FieldReader<bool, SRAM2ER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM2ER` writer - SRAM2 erase"]
pub struct SRAM2ER_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2ER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM2ER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Start SRAM2 erase operation"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(SRAM2ER_A::ERASE)
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
impl R {
    #[doc = "Bit 8 - PKA SRAM busy by erase operation"]
    #[inline(always)]
    pub fn pkasrambsy(&self) -> PKASRAMBSY_R {
        PKASRAMBSY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SRAM1, SRAM2 and PKA SRAM busy by erase operation"]
    #[inline(always)]
    pub fn srambsy(&self) -> SRAMBSY_R {
        SRAMBSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SRAM2 erase"]
    #[inline(always)]
    pub fn sram2er(&self) -> SRAM2ER_R {
        SRAM2ER_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM2 erase"]
    #[inline(always)]
    pub fn sram2er(&mut self) -> SRAM2ER_W {
        SRAM2ER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scsr](index.html) module"]
pub struct SCSR_SPEC;
impl crate::RegisterSpec for SCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scsr::R](R) reader structure"]
impl crate::Readable for SCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scsr::W](W) writer structure"]
impl crate::Writable for SCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCSR to value 0"]
impl crate::Resettable for SCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
