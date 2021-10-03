#[doc = "Register `SUBGHZSPICR` reader"]
pub struct R(crate::R<SUBGHZSPICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUBGHZSPICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUBGHZSPICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUBGHZSPICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUBGHZSPICR` writer"]
pub struct W(crate::W<SUBGHZSPICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUBGHZSPICR_SPEC>;
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
impl From<crate::W<SUBGHZSPICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUBGHZSPICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "sub-GHz SPI NSS control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSS_A {
    #[doc = "0: Sub-GHz SPI NSS signal at level low"]
    LOW = 0,
    #[doc = "1: Sub-GHz SPI NSS signal is at level high"]
    HIGH = 1,
}
impl From<NSS_A> for bool {
    #[inline(always)]
    fn from(variant: NSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NSS` reader - sub-GHz SPI NSS control"]
pub struct NSS_R(crate::FieldReader<bool, NSS_A>);
impl NSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSS_A {
        match self.bits {
            false => NSS_A::LOW,
            true => NSS_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == NSS_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == NSS_A::HIGH
    }
}
impl core::ops::Deref for NSS_R {
    type Target = crate::FieldReader<bool, NSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSS` writer - sub-GHz SPI NSS control"]
pub struct NSS_W<'a> {
    w: &'a mut W,
}
impl<'a> NSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sub-GHz SPI NSS signal at level low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(NSS_A::LOW)
    }
    #[doc = "Sub-GHz SPI NSS signal is at level high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(NSS_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - sub-GHz SPI NSS control"]
    #[inline(always)]
    pub fn nss(&self) -> NSS_R {
        NSS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - sub-GHz SPI NSS control"]
    #[inline(always)]
    pub fn nss(&mut self) -> NSS_W {
        NSS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power SPI3 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subghzspicr](index.html) module"]
pub struct SUBGHZSPICR_SPEC;
impl crate::RegisterSpec for SUBGHZSPICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [subghzspicr::R](R) reader structure"]
impl crate::Readable for SUBGHZSPICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [subghzspicr::W](W) writer structure"]
impl crate::Writable for SUBGHZSPICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SUBGHZSPICR to value 0x8000"]
impl crate::Resettable for SUBGHZSPICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
