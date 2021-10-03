#[doc = "Register `OSPEEDR` reader"]
pub struct R(crate::R<OSPEEDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSPEEDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSPEEDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSPEEDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSPEEDR` writer"]
pub struct W(crate::W<OSPEEDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSPEEDR_SPEC>;
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
impl From<crate::W<OSPEEDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSPEEDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port x configuration bits (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSPEEDR3_A {
    #[doc = "0: Low speed"]
    LOWSPEED = 0,
    #[doc = "1: Medium speed"]
    MEDIUMSPEED = 1,
    #[doc = "2: High speed"]
    HIGHSPEED = 2,
    #[doc = "3: Very high speed"]
    VERYHIGHSPEED = 3,
}
impl From<OSPEEDR3_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPEEDR3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OSPEEDR3` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR3_R(crate::FieldReader<u8, OSPEEDR3_A>);
impl OSPEEDR3_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSPEEDR3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSPEEDR3_A {
        match self.bits {
            0 => OSPEEDR3_A::LOWSPEED,
            1 => OSPEEDR3_A::MEDIUMSPEED,
            2 => OSPEEDR3_A::HIGHSPEED,
            3 => OSPEEDR3_A::VERYHIGHSPEED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOWSPEED`"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        **self == OSPEEDR3_A::LOWSPEED
    }
    #[doc = "Checks if the value of the field is `MEDIUMSPEED`"]
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        **self == OSPEEDR3_A::MEDIUMSPEED
    }
    #[doc = "Checks if the value of the field is `HIGHSPEED`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        **self == OSPEEDR3_A::HIGHSPEED
    }
    #[doc = "Checks if the value of the field is `VERYHIGHSPEED`"]
    #[inline(always)]
    pub fn is_very_high_speed(&self) -> bool {
        **self == OSPEEDR3_A::VERYHIGHSPEED
    }
}
impl core::ops::Deref for OSPEEDR3_R {
    type Target = crate::FieldReader<u8, OSPEEDR3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEEDR3` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR3_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEEDR3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEEDR3_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEEDR3_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR3_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR3_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr3(&self) -> OSPEEDR3_R {
        OSPEEDR3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr3(&mut self) -> OSPEEDR3_W {
        OSPEEDR3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ospeedr](index.html) module"]
pub struct OSPEEDR_SPEC;
impl crate::RegisterSpec for OSPEEDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ospeedr::R](R) reader structure"]
impl crate::Readable for OSPEEDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ospeedr::W](W) writer structure"]
impl crate::Writable for OSPEEDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSPEEDR to value 0"]
impl crate::Resettable for OSPEEDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
