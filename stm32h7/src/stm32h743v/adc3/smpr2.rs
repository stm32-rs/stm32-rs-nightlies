#[doc = "Register `SMPR2` reader"]
pub struct R(crate::R<SMPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPR2` writer"]
pub struct W(crate::W<SMPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR2_SPEC>;
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
impl From<crate::W<SMPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ADC channel 19 sampling time"]
pub type SMP19_A = SMP10_A;
#[doc = "Field `SMP19` reader - ADC channel 19 sampling time"]
pub type SMP19_R = SMP10_R;
#[doc = "Field `SMP19` writer - ADC channel 19 sampling time"]
pub struct SMP19_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP19_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP19_A::CYCLES1_5)
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP19_A::CYCLES2_5)
    }
    #[doc = "8.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles8_5(self) -> &'a mut W {
        self.variant(SMP19_A::CYCLES8_5)
    }
    #[doc = "16.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles16_5(self) -> &'a mut W {
        self.variant(SMP19_A::CYCLES16_5)
    }
    #[doc = "32.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles32_5(self) -> &'a mut W {
        self.variant(SMP19_A::CYCLES32_5)
    }
    #[doc = "64.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles64_5(self) -> &'a mut W {
        self.variant(SMP19_A::CYCLES64_5)
    }
    #[doc = "387.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles387_5(self) -> &'a mut W {
        self.variant(SMP19_A::CYCLES387_5)
    }
    #[doc = "810.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles810_5(self) -> &'a mut W {
        self.variant(SMP19_A::CYCLES810_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | ((value as u32 & 0x07) << 27);
        self.w
    }
}
#[doc = "ADC channel 18 sampling time selection"]
pub type SMP18_A = SMP10_A;
#[doc = "Field `SMP18` reader - ADC channel 18 sampling time selection"]
pub type SMP18_R = SMP10_R;
#[doc = "Field `SMP18` writer - ADC channel 18 sampling time selection"]
pub struct SMP18_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP18_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES1_5)
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES2_5)
    }
    #[doc = "8.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles8_5(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES8_5)
    }
    #[doc = "16.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles16_5(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES16_5)
    }
    #[doc = "32.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles32_5(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES32_5)
    }
    #[doc = "64.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles64_5(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES64_5)
    }
    #[doc = "387.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles387_5(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES387_5)
    }
    #[doc = "810.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles810_5(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES810_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "ADC channel 17 sampling time selection"]
pub type SMP17_A = SMP10_A;
#[doc = "Field `SMP17` reader - ADC channel 17 sampling time selection"]
pub type SMP17_R = SMP10_R;
#[doc = "Field `SMP17` writer - ADC channel 17 sampling time selection"]
pub struct SMP17_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP17_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP17_A::CYCLES1_5)
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP17_A::CYCLES2_5)
    }
    #[doc = "8.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles8_5(self) -> &'a mut W {
        self.variant(SMP17_A::CYCLES8_5)
    }
    #[doc = "16.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles16_5(self) -> &'a mut W {
        self.variant(SMP17_A::CYCLES16_5)
    }
    #[doc = "32.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles32_5(self) -> &'a mut W {
        self.variant(SMP17_A::CYCLES32_5)
    }
    #[doc = "64.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles64_5(self) -> &'a mut W {
        self.variant(SMP17_A::CYCLES64_5)
    }
    #[doc = "387.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles387_5(self) -> &'a mut W {
        self.variant(SMP17_A::CYCLES387_5)
    }
    #[doc = "810.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles810_5(self) -> &'a mut W {
        self.variant(SMP17_A::CYCLES810_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | ((value as u32 & 0x07) << 21);
        self.w
    }
}
#[doc = "ADC channel 16 sampling time selection"]
pub type SMP16_A = SMP10_A;
#[doc = "Field `SMP16` reader - ADC channel 16 sampling time selection"]
pub type SMP16_R = SMP10_R;
#[doc = "Field `SMP16` writer - ADC channel 16 sampling time selection"]
pub struct SMP16_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP16_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP16_A::CYCLES1_5)
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP16_A::CYCLES2_5)
    }
    #[doc = "8.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles8_5(self) -> &'a mut W {
        self.variant(SMP16_A::CYCLES8_5)
    }
    #[doc = "16.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles16_5(self) -> &'a mut W {
        self.variant(SMP16_A::CYCLES16_5)
    }
    #[doc = "32.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles32_5(self) -> &'a mut W {
        self.variant(SMP16_A::CYCLES32_5)
    }
    #[doc = "64.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles64_5(self) -> &'a mut W {
        self.variant(SMP16_A::CYCLES64_5)
    }
    #[doc = "387.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles387_5(self) -> &'a mut W {
        self.variant(SMP16_A::CYCLES387_5)
    }
    #[doc = "810.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles810_5(self) -> &'a mut W {
        self.variant(SMP16_A::CYCLES810_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "ADC channel 15 sampling time selection"]
pub type SMP15_A = SMP10_A;
#[doc = "Field `SMP15` reader - ADC channel 15 sampling time selection"]
pub type SMP15_R = SMP10_R;
#[doc = "Field `SMP15` writer - ADC channel 15 sampling time selection"]
pub struct SMP15_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP15_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP15_A::CYCLES1_5)
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP15_A::CYCLES2_5)
    }
    #[doc = "8.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles8_5(self) -> &'a mut W {
        self.variant(SMP15_A::CYCLES8_5)
    }
    #[doc = "16.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles16_5(self) -> &'a mut W {
        self.variant(SMP15_A::CYCLES16_5)
    }
    #[doc = "32.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles32_5(self) -> &'a mut W {
        self.variant(SMP15_A::CYCLES32_5)
    }
    #[doc = "64.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles64_5(self) -> &'a mut W {
        self.variant(SMP15_A::CYCLES64_5)
    }
    #[doc = "387.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles387_5(self) -> &'a mut W {
        self.variant(SMP15_A::CYCLES387_5)
    }
    #[doc = "810.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles810_5(self) -> &'a mut W {
        self.variant(SMP15_A::CYCLES810_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | ((value as u32 & 0x07) << 15);
        self.w
    }
}
#[doc = "ADC channel 14 sampling time selection"]
pub type SMP14_A = SMP10_A;
#[doc = "Field `SMP14` reader - ADC channel 14 sampling time selection"]
pub type SMP14_R = SMP10_R;
#[doc = "Field `SMP14` writer - ADC channel 14 sampling time selection"]
pub struct SMP14_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP14_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP14_A::CYCLES1_5)
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP14_A::CYCLES2_5)
    }
    #[doc = "8.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles8_5(self) -> &'a mut W {
        self.variant(SMP14_A::CYCLES8_5)
    }
    #[doc = "16.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles16_5(self) -> &'a mut W {
        self.variant(SMP14_A::CYCLES16_5)
    }
    #[doc = "32.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles32_5(self) -> &'a mut W {
        self.variant(SMP14_A::CYCLES32_5)
    }
    #[doc = "64.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles64_5(self) -> &'a mut W {
        self.variant(SMP14_A::CYCLES64_5)
    }
    #[doc = "387.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles387_5(self) -> &'a mut W {
        self.variant(SMP14_A::CYCLES387_5)
    }
    #[doc = "810.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles810_5(self) -> &'a mut W {
        self.variant(SMP14_A::CYCLES810_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "ADC channel 13 sampling time selection"]
pub type SMP13_A = SMP10_A;
#[doc = "Field `SMP13` reader - ADC channel 13 sampling time selection"]
pub type SMP13_R = SMP10_R;
#[doc = "Field `SMP13` writer - ADC channel 13 sampling time selection"]
pub struct SMP13_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP13_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP13_A::CYCLES1_5)
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP13_A::CYCLES2_5)
    }
    #[doc = "8.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles8_5(self) -> &'a mut W {
        self.variant(SMP13_A::CYCLES8_5)
    }
    #[doc = "16.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles16_5(self) -> &'a mut W {
        self.variant(SMP13_A::CYCLES16_5)
    }
    #[doc = "32.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles32_5(self) -> &'a mut W {
        self.variant(SMP13_A::CYCLES32_5)
    }
    #[doc = "64.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles64_5(self) -> &'a mut W {
        self.variant(SMP13_A::CYCLES64_5)
    }
    #[doc = "387.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles387_5(self) -> &'a mut W {
        self.variant(SMP13_A::CYCLES387_5)
    }
    #[doc = "810.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles810_5(self) -> &'a mut W {
        self.variant(SMP13_A::CYCLES810_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "ADC channel 12 sampling time selection"]
pub type SMP12_A = SMP10_A;
#[doc = "Field `SMP12` reader - ADC channel 12 sampling time selection"]
pub type SMP12_R = SMP10_R;
#[doc = "Field `SMP12` writer - ADC channel 12 sampling time selection"]
pub struct SMP12_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP12_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP12_A::CYCLES1_5)
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP12_A::CYCLES2_5)
    }
    #[doc = "8.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles8_5(self) -> &'a mut W {
        self.variant(SMP12_A::CYCLES8_5)
    }
    #[doc = "16.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles16_5(self) -> &'a mut W {
        self.variant(SMP12_A::CYCLES16_5)
    }
    #[doc = "32.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles32_5(self) -> &'a mut W {
        self.variant(SMP12_A::CYCLES32_5)
    }
    #[doc = "64.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles64_5(self) -> &'a mut W {
        self.variant(SMP12_A::CYCLES64_5)
    }
    #[doc = "387.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles387_5(self) -> &'a mut W {
        self.variant(SMP12_A::CYCLES387_5)
    }
    #[doc = "810.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles810_5(self) -> &'a mut W {
        self.variant(SMP12_A::CYCLES810_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "ADC channel 11 sampling time selection"]
pub type SMP11_A = SMP10_A;
#[doc = "Field `SMP11` reader - ADC channel 11 sampling time selection"]
pub type SMP11_R = SMP10_R;
#[doc = "Field `SMP11` writer - ADC channel 11 sampling time selection"]
pub struct SMP11_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP11_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP11_A::CYCLES1_5)
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP11_A::CYCLES2_5)
    }
    #[doc = "8.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles8_5(self) -> &'a mut W {
        self.variant(SMP11_A::CYCLES8_5)
    }
    #[doc = "16.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles16_5(self) -> &'a mut W {
        self.variant(SMP11_A::CYCLES16_5)
    }
    #[doc = "32.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles32_5(self) -> &'a mut W {
        self.variant(SMP11_A::CYCLES32_5)
    }
    #[doc = "64.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles64_5(self) -> &'a mut W {
        self.variant(SMP11_A::CYCLES64_5)
    }
    #[doc = "387.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles387_5(self) -> &'a mut W {
        self.variant(SMP11_A::CYCLES387_5)
    }
    #[doc = "810.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles810_5(self) -> &'a mut W {
        self.variant(SMP11_A::CYCLES810_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "ADC channel 10 sampling time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMP10_A {
    #[doc = "0: 1.5 ADC clock cycles"]
    CYCLES1_5 = 0,
    #[doc = "1: 2.5 ADC clock cycles"]
    CYCLES2_5 = 1,
    #[doc = "2: 8.5 ADC clock cycles"]
    CYCLES8_5 = 2,
    #[doc = "3: 16.5 ADC clock cycles"]
    CYCLES16_5 = 3,
    #[doc = "4: 32.5 ADC clock cycles"]
    CYCLES32_5 = 4,
    #[doc = "5: 64.5 ADC clock cycles"]
    CYCLES64_5 = 5,
    #[doc = "6: 387.5 ADC clock cycles"]
    CYCLES387_5 = 6,
    #[doc = "7: 810.5 ADC clock cycles"]
    CYCLES810_5 = 7,
}
impl From<SMP10_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP10_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SMP10` reader - ADC channel 10 sampling time selection"]
pub struct SMP10_R(crate::FieldReader<u8, SMP10_A>);
impl SMP10_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP10_A {
        match self.bits {
            0 => SMP10_A::CYCLES1_5,
            1 => SMP10_A::CYCLES2_5,
            2 => SMP10_A::CYCLES8_5,
            3 => SMP10_A::CYCLES16_5,
            4 => SMP10_A::CYCLES32_5,
            5 => SMP10_A::CYCLES64_5,
            6 => SMP10_A::CYCLES387_5,
            7 => SMP10_A::CYCLES810_5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES1_5`"]
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        **self == SMP10_A::CYCLES1_5
    }
    #[doc = "Checks if the value of the field is `CYCLES2_5`"]
    #[inline(always)]
    pub fn is_cycles2_5(&self) -> bool {
        **self == SMP10_A::CYCLES2_5
    }
    #[doc = "Checks if the value of the field is `CYCLES8_5`"]
    #[inline(always)]
    pub fn is_cycles8_5(&self) -> bool {
        **self == SMP10_A::CYCLES8_5
    }
    #[doc = "Checks if the value of the field is `CYCLES16_5`"]
    #[inline(always)]
    pub fn is_cycles16_5(&self) -> bool {
        **self == SMP10_A::CYCLES16_5
    }
    #[doc = "Checks if the value of the field is `CYCLES32_5`"]
    #[inline(always)]
    pub fn is_cycles32_5(&self) -> bool {
        **self == SMP10_A::CYCLES32_5
    }
    #[doc = "Checks if the value of the field is `CYCLES64_5`"]
    #[inline(always)]
    pub fn is_cycles64_5(&self) -> bool {
        **self == SMP10_A::CYCLES64_5
    }
    #[doc = "Checks if the value of the field is `CYCLES387_5`"]
    #[inline(always)]
    pub fn is_cycles387_5(&self) -> bool {
        **self == SMP10_A::CYCLES387_5
    }
    #[doc = "Checks if the value of the field is `CYCLES810_5`"]
    #[inline(always)]
    pub fn is_cycles810_5(&self) -> bool {
        **self == SMP10_A::CYCLES810_5
    }
}
impl core::ops::Deref for SMP10_R {
    type Target = crate::FieldReader<u8, SMP10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP10` writer - ADC channel 10 sampling time selection"]
pub struct SMP10_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP10_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP10_A::CYCLES1_5)
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP10_A::CYCLES2_5)
    }
    #[doc = "8.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles8_5(self) -> &'a mut W {
        self.variant(SMP10_A::CYCLES8_5)
    }
    #[doc = "16.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles16_5(self) -> &'a mut W {
        self.variant(SMP10_A::CYCLES16_5)
    }
    #[doc = "32.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles32_5(self) -> &'a mut W {
        self.variant(SMP10_A::CYCLES32_5)
    }
    #[doc = "64.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles64_5(self) -> &'a mut W {
        self.variant(SMP10_A::CYCLES64_5)
    }
    #[doc = "387.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles387_5(self) -> &'a mut W {
        self.variant(SMP10_A::CYCLES387_5)
    }
    #[doc = "810.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles810_5(self) -> &'a mut W {
        self.variant(SMP10_A::CYCLES810_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:29 - ADC channel 19 sampling time"]
    #[inline(always)]
    pub fn smp19(&self) -> SMP19_R {
        SMP19_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - ADC channel 18 sampling time selection"]
    #[inline(always)]
    pub fn smp18(&self) -> SMP18_R {
        SMP18_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 21:23 - ADC channel 17 sampling time selection"]
    #[inline(always)]
    pub fn smp17(&self) -> SMP17_R {
        SMP17_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 18:20 - ADC channel 16 sampling time selection"]
    #[inline(always)]
    pub fn smp16(&self) -> SMP16_R {
        SMP16_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 15:17 - ADC channel 15 sampling time selection"]
    #[inline(always)]
    pub fn smp15(&self) -> SMP15_R {
        SMP15_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - ADC channel 14 sampling time selection"]
    #[inline(always)]
    pub fn smp14(&self) -> SMP14_R {
        SMP14_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - ADC channel 13 sampling time selection"]
    #[inline(always)]
    pub fn smp13(&self) -> SMP13_R {
        SMP13_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - ADC channel 12 sampling time selection"]
    #[inline(always)]
    pub fn smp12(&self) -> SMP12_R {
        SMP12_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - ADC channel 11 sampling time selection"]
    #[inline(always)]
    pub fn smp11(&self) -> SMP11_R {
        SMP11_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - ADC channel 10 sampling time selection"]
    #[inline(always)]
    pub fn smp10(&self) -> SMP10_R {
        SMP10_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 27:29 - ADC channel 19 sampling time"]
    #[inline(always)]
    pub fn smp19(&mut self) -> SMP19_W {
        SMP19_W { w: self }
    }
    #[doc = "Bits 24:26 - ADC channel 18 sampling time selection"]
    #[inline(always)]
    pub fn smp18(&mut self) -> SMP18_W {
        SMP18_W { w: self }
    }
    #[doc = "Bits 21:23 - ADC channel 17 sampling time selection"]
    #[inline(always)]
    pub fn smp17(&mut self) -> SMP17_W {
        SMP17_W { w: self }
    }
    #[doc = "Bits 18:20 - ADC channel 16 sampling time selection"]
    #[inline(always)]
    pub fn smp16(&mut self) -> SMP16_W {
        SMP16_W { w: self }
    }
    #[doc = "Bits 15:17 - ADC channel 15 sampling time selection"]
    #[inline(always)]
    pub fn smp15(&mut self) -> SMP15_W {
        SMP15_W { w: self }
    }
    #[doc = "Bits 12:14 - ADC channel 14 sampling time selection"]
    #[inline(always)]
    pub fn smp14(&mut self) -> SMP14_W {
        SMP14_W { w: self }
    }
    #[doc = "Bits 9:11 - ADC channel 13 sampling time selection"]
    #[inline(always)]
    pub fn smp13(&mut self) -> SMP13_W {
        SMP13_W { w: self }
    }
    #[doc = "Bits 6:8 - ADC channel 12 sampling time selection"]
    #[inline(always)]
    pub fn smp12(&mut self) -> SMP12_W {
        SMP12_W { w: self }
    }
    #[doc = "Bits 3:5 - ADC channel 11 sampling time selection"]
    #[inline(always)]
    pub fn smp11(&mut self) -> SMP11_W {
        SMP11_W { w: self }
    }
    #[doc = "Bits 0:2 - ADC channel 10 sampling time selection"]
    #[inline(always)]
    pub fn smp10(&mut self) -> SMP10_W {
        SMP10_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC sampling time register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr2](index.html) module"]
pub struct SMPR2_SPEC;
impl crate::RegisterSpec for SMPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpr2::R](R) reader structure"]
impl crate::Readable for SMPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpr2::W](W) writer structure"]
impl crate::Writable for SMPR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMPR2 to value 0"]
impl crate::Resettable for SMPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
