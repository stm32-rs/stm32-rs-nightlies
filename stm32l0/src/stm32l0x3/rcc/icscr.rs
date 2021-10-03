#[doc = "Register `ICSCR` reader"]
pub struct R(crate::R<ICSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICSCR` writer"]
pub struct W(crate::W<ICSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICSCR_SPEC>;
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
impl From<crate::W<ICSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSITRIM` reader - MSI clock trimming"]
pub struct MSITRIM_R(crate::FieldReader<u8, u8>);
impl MSITRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        MSITRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSITRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSITRIM` writer - MSI clock trimming"]
pub struct MSITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MSITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `MSICAL` reader - MSI clock calibration"]
pub struct MSICAL_R(crate::FieldReader<u8, u8>);
impl MSICAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        MSICAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSICAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "MSI clock ranges\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSIRANGE_A {
    #[doc = "0: range 0 around 65.536 kHz"]
    RANGE0 = 0,
    #[doc = "1: range 1 around 131.072 kHz"]
    RANGE1 = 1,
    #[doc = "2: range 2 around 262.144 kHz"]
    RANGE2 = 2,
    #[doc = "3: range 3 around 524.288 kHz"]
    RANGE3 = 3,
    #[doc = "4: range 4 around 1.048 MHz"]
    RANGE4 = 4,
    #[doc = "5: range 5 around 2.097 MHz (reset value)"]
    RANGE5 = 5,
    #[doc = "6: range 6 around 4.194 MHz"]
    RANGE6 = 6,
    #[doc = "7: not allowed"]
    RANGE7 = 7,
}
impl From<MSIRANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: MSIRANGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MSIRANGE` reader - MSI clock ranges"]
pub struct MSIRANGE_R(crate::FieldReader<u8, MSIRANGE_A>);
impl MSIRANGE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MSIRANGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSIRANGE_A {
        match self.bits {
            0 => MSIRANGE_A::RANGE0,
            1 => MSIRANGE_A::RANGE1,
            2 => MSIRANGE_A::RANGE2,
            3 => MSIRANGE_A::RANGE3,
            4 => MSIRANGE_A::RANGE4,
            5 => MSIRANGE_A::RANGE5,
            6 => MSIRANGE_A::RANGE6,
            7 => MSIRANGE_A::RANGE7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RANGE0`"]
    #[inline(always)]
    pub fn is_range0(&self) -> bool {
        **self == MSIRANGE_A::RANGE0
    }
    #[doc = "Checks if the value of the field is `RANGE1`"]
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        **self == MSIRANGE_A::RANGE1
    }
    #[doc = "Checks if the value of the field is `RANGE2`"]
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        **self == MSIRANGE_A::RANGE2
    }
    #[doc = "Checks if the value of the field is `RANGE3`"]
    #[inline(always)]
    pub fn is_range3(&self) -> bool {
        **self == MSIRANGE_A::RANGE3
    }
    #[doc = "Checks if the value of the field is `RANGE4`"]
    #[inline(always)]
    pub fn is_range4(&self) -> bool {
        **self == MSIRANGE_A::RANGE4
    }
    #[doc = "Checks if the value of the field is `RANGE5`"]
    #[inline(always)]
    pub fn is_range5(&self) -> bool {
        **self == MSIRANGE_A::RANGE5
    }
    #[doc = "Checks if the value of the field is `RANGE6`"]
    #[inline(always)]
    pub fn is_range6(&self) -> bool {
        **self == MSIRANGE_A::RANGE6
    }
    #[doc = "Checks if the value of the field is `RANGE7`"]
    #[inline(always)]
    pub fn is_range7(&self) -> bool {
        **self == MSIRANGE_A::RANGE7
    }
}
impl core::ops::Deref for MSIRANGE_R {
    type Target = crate::FieldReader<u8, MSIRANGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSIRANGE` writer - MSI clock ranges"]
pub struct MSIRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> MSIRANGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSIRANGE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "range 0 around 65.536 kHz"]
    #[inline(always)]
    pub fn range0(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE0)
    }
    #[doc = "range 1 around 131.072 kHz"]
    #[inline(always)]
    pub fn range1(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE1)
    }
    #[doc = "range 2 around 262.144 kHz"]
    #[inline(always)]
    pub fn range2(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE2)
    }
    #[doc = "range 3 around 524.288 kHz"]
    #[inline(always)]
    pub fn range3(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE3)
    }
    #[doc = "range 4 around 1.048 MHz"]
    #[inline(always)]
    pub fn range4(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE4)
    }
    #[doc = "range 5 around 2.097 MHz (reset value)"]
    #[inline(always)]
    pub fn range5(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE5)
    }
    #[doc = "range 6 around 4.194 MHz"]
    #[inline(always)]
    pub fn range6(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE6)
    }
    #[doc = "not allowed"]
    #[inline(always)]
    pub fn range7(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
#[doc = "Field `HSI16TRIM` reader - High speed internal clock trimming"]
pub struct HSI16TRIM_R(crate::FieldReader<u8, u8>);
impl HSI16TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSI16TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI16TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSI16TRIM` writer - High speed internal clock trimming"]
pub struct HSI16TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI16TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `HSI16CAL` reader - nternal high speed clock calibration"]
pub struct HSI16CAL_R(crate::FieldReader<u8, u8>);
impl HSI16CAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSI16CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI16CAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 24:31 - MSI clock trimming"]
    #[inline(always)]
    pub fn msitrim(&self) -> MSITRIM_R {
        MSITRIM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - MSI clock calibration"]
    #[inline(always)]
    pub fn msical(&self) -> MSICAL_R {
        MSICAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 13:15 - MSI clock ranges"]
    #[inline(always)]
    pub fn msirange(&self) -> MSIRANGE_R {
        MSIRANGE_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 8:12 - High speed internal clock trimming"]
    #[inline(always)]
    pub fn hsi16trim(&self) -> HSI16TRIM_R {
        HSI16TRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:7 - nternal high speed clock calibration"]
    #[inline(always)]
    pub fn hsi16cal(&self) -> HSI16CAL_R {
        HSI16CAL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - MSI clock trimming"]
    #[inline(always)]
    pub fn msitrim(&mut self) -> MSITRIM_W {
        MSITRIM_W { w: self }
    }
    #[doc = "Bits 13:15 - MSI clock ranges"]
    #[inline(always)]
    pub fn msirange(&mut self) -> MSIRANGE_W {
        MSIRANGE_W { w: self }
    }
    #[doc = "Bits 8:12 - High speed internal clock trimming"]
    #[inline(always)]
    pub fn hsi16trim(&mut self) -> HSI16TRIM_W {
        HSI16TRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal clock sources calibration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icscr](index.html) module"]
pub struct ICSCR_SPEC;
impl crate::RegisterSpec for ICSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icscr::R](R) reader structure"]
impl crate::Readable for ICSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icscr::W](W) writer structure"]
impl crate::Writable for ICSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICSCR to value 0xb000"]
impl crate::Resettable for ICSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb000
    }
}
