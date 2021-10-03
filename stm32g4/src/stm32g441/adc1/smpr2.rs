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
#[doc = "Channel 18 sampling time selection"]
pub type SMP18_A = SMP10_A;
#[doc = "Field `SMP18` reader - Channel 18 sampling time selection"]
pub type SMP18_R = SMP10_R;
#[doc = "Field `SMP18` writer - Channel 18 sampling time selection"]
pub struct SMP18_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP18_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES2_5)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles6_5(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES6_5)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES12_5)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles24_5(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES24_5)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles47_5(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES47_5)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles92_5(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES92_5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles247_5(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES247_5)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles640_5(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES640_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Channel 17 sampling time selection"]
pub type SMP17_A = SMP10_A;
#[doc = "Field `SMP17` reader - Channel 17 sampling time selection"]
pub type SMP17_R = SMP10_R;
#[doc = "Field `SMP17` writer - Channel 17 sampling time selection"]
pub struct SMP17_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP17_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP17_A::CYCLES2_5)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles6_5(self) -> &'a mut W {
        self.variant(SMP17_A::CYCLES6_5)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP17_A::CYCLES12_5)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles24_5(self) -> &'a mut W {
        self.variant(SMP17_A::CYCLES24_5)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles47_5(self) -> &'a mut W {
        self.variant(SMP17_A::CYCLES47_5)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles92_5(self) -> &'a mut W {
        self.variant(SMP17_A::CYCLES92_5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles247_5(self) -> &'a mut W {
        self.variant(SMP17_A::CYCLES247_5)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles640_5(self) -> &'a mut W {
        self.variant(SMP17_A::CYCLES640_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | ((value as u32 & 0x07) << 21);
        self.w
    }
}
#[doc = "Channel 16 sampling time selection"]
pub type SMP16_A = SMP10_A;
#[doc = "Field `SMP16` reader - Channel 16 sampling time selection"]
pub type SMP16_R = SMP10_R;
#[doc = "Field `SMP16` writer - Channel 16 sampling time selection"]
pub struct SMP16_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP16_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP16_A::CYCLES2_5)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles6_5(self) -> &'a mut W {
        self.variant(SMP16_A::CYCLES6_5)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP16_A::CYCLES12_5)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles24_5(self) -> &'a mut W {
        self.variant(SMP16_A::CYCLES24_5)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles47_5(self) -> &'a mut W {
        self.variant(SMP16_A::CYCLES47_5)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles92_5(self) -> &'a mut W {
        self.variant(SMP16_A::CYCLES92_5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles247_5(self) -> &'a mut W {
        self.variant(SMP16_A::CYCLES247_5)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles640_5(self) -> &'a mut W {
        self.variant(SMP16_A::CYCLES640_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Channel 15 sampling time selection"]
pub type SMP15_A = SMP10_A;
#[doc = "Field `SMP15` reader - Channel 15 sampling time selection"]
pub type SMP15_R = SMP10_R;
#[doc = "Field `SMP15` writer - Channel 15 sampling time selection"]
pub struct SMP15_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP15_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP15_A::CYCLES2_5)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles6_5(self) -> &'a mut W {
        self.variant(SMP15_A::CYCLES6_5)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP15_A::CYCLES12_5)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles24_5(self) -> &'a mut W {
        self.variant(SMP15_A::CYCLES24_5)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles47_5(self) -> &'a mut W {
        self.variant(SMP15_A::CYCLES47_5)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles92_5(self) -> &'a mut W {
        self.variant(SMP15_A::CYCLES92_5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles247_5(self) -> &'a mut W {
        self.variant(SMP15_A::CYCLES247_5)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles640_5(self) -> &'a mut W {
        self.variant(SMP15_A::CYCLES640_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | ((value as u32 & 0x07) << 15);
        self.w
    }
}
#[doc = "Channel 14 sampling time selection"]
pub type SMP14_A = SMP10_A;
#[doc = "Field `SMP14` reader - Channel 14 sampling time selection"]
pub type SMP14_R = SMP10_R;
#[doc = "Field `SMP14` writer - Channel 14 sampling time selection"]
pub struct SMP14_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP14_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP14_A::CYCLES2_5)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles6_5(self) -> &'a mut W {
        self.variant(SMP14_A::CYCLES6_5)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP14_A::CYCLES12_5)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles24_5(self) -> &'a mut W {
        self.variant(SMP14_A::CYCLES24_5)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles47_5(self) -> &'a mut W {
        self.variant(SMP14_A::CYCLES47_5)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles92_5(self) -> &'a mut W {
        self.variant(SMP14_A::CYCLES92_5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles247_5(self) -> &'a mut W {
        self.variant(SMP14_A::CYCLES247_5)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles640_5(self) -> &'a mut W {
        self.variant(SMP14_A::CYCLES640_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Channel 13 sampling time selection"]
pub type SMP13_A = SMP10_A;
#[doc = "Field `SMP13` reader - Channel 13 sampling time selection"]
pub type SMP13_R = SMP10_R;
#[doc = "Field `SMP13` writer - Channel 13 sampling time selection"]
pub struct SMP13_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP13_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP13_A::CYCLES2_5)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles6_5(self) -> &'a mut W {
        self.variant(SMP13_A::CYCLES6_5)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP13_A::CYCLES12_5)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles24_5(self) -> &'a mut W {
        self.variant(SMP13_A::CYCLES24_5)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles47_5(self) -> &'a mut W {
        self.variant(SMP13_A::CYCLES47_5)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles92_5(self) -> &'a mut W {
        self.variant(SMP13_A::CYCLES92_5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles247_5(self) -> &'a mut W {
        self.variant(SMP13_A::CYCLES247_5)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles640_5(self) -> &'a mut W {
        self.variant(SMP13_A::CYCLES640_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Channel 11 sampling time selection"]
pub type SMP12_A = SMP10_A;
#[doc = "Field `SMP12` reader - Channel 11 sampling time selection"]
pub type SMP12_R = SMP10_R;
#[doc = "Field `SMP12` writer - Channel 11 sampling time selection"]
pub struct SMP12_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP12_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP12_A::CYCLES2_5)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles6_5(self) -> &'a mut W {
        self.variant(SMP12_A::CYCLES6_5)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP12_A::CYCLES12_5)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles24_5(self) -> &'a mut W {
        self.variant(SMP12_A::CYCLES24_5)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles47_5(self) -> &'a mut W {
        self.variant(SMP12_A::CYCLES47_5)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles92_5(self) -> &'a mut W {
        self.variant(SMP12_A::CYCLES92_5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles247_5(self) -> &'a mut W {
        self.variant(SMP12_A::CYCLES247_5)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles640_5(self) -> &'a mut W {
        self.variant(SMP12_A::CYCLES640_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Channel 12 sampling time selection"]
pub type SMP11_A = SMP10_A;
#[doc = "Field `SMP11` reader - Channel 12 sampling time selection"]
pub type SMP11_R = SMP10_R;
#[doc = "Field `SMP11` writer - Channel 12 sampling time selection"]
pub struct SMP11_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP11_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP11_A::CYCLES2_5)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles6_5(self) -> &'a mut W {
        self.variant(SMP11_A::CYCLES6_5)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP11_A::CYCLES12_5)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles24_5(self) -> &'a mut W {
        self.variant(SMP11_A::CYCLES24_5)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles47_5(self) -> &'a mut W {
        self.variant(SMP11_A::CYCLES47_5)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles92_5(self) -> &'a mut W {
        self.variant(SMP11_A::CYCLES92_5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles247_5(self) -> &'a mut W {
        self.variant(SMP11_A::CYCLES247_5)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles640_5(self) -> &'a mut W {
        self.variant(SMP11_A::CYCLES640_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Channel 10 sampling time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMP10_A {
    #[doc = "0: 2.5 ADC clock cycles"]
    CYCLES2_5 = 0,
    #[doc = "1: 6.5 ADC clock cycles"]
    CYCLES6_5 = 1,
    #[doc = "2: 12.5 ADC clock cycles"]
    CYCLES12_5 = 2,
    #[doc = "3: 24.5 ADC clock cycles"]
    CYCLES24_5 = 3,
    #[doc = "4: 47.5 ADC clock cycles"]
    CYCLES47_5 = 4,
    #[doc = "5: 92.5 ADC clock cycles"]
    CYCLES92_5 = 5,
    #[doc = "6: 247.5 ADC clock cycles"]
    CYCLES247_5 = 6,
    #[doc = "7: 640.5 ADC clock cycles"]
    CYCLES640_5 = 7,
}
impl From<SMP10_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP10_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SMP10` reader - Channel 10 sampling time selection"]
pub struct SMP10_R(crate::FieldReader<u8, SMP10_A>);
impl SMP10_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP10_A {
        match self.bits {
            0 => SMP10_A::CYCLES2_5,
            1 => SMP10_A::CYCLES6_5,
            2 => SMP10_A::CYCLES12_5,
            3 => SMP10_A::CYCLES24_5,
            4 => SMP10_A::CYCLES47_5,
            5 => SMP10_A::CYCLES92_5,
            6 => SMP10_A::CYCLES247_5,
            7 => SMP10_A::CYCLES640_5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES2_5`"]
    #[inline(always)]
    pub fn is_cycles2_5(&self) -> bool {
        **self == SMP10_A::CYCLES2_5
    }
    #[doc = "Checks if the value of the field is `CYCLES6_5`"]
    #[inline(always)]
    pub fn is_cycles6_5(&self) -> bool {
        **self == SMP10_A::CYCLES6_5
    }
    #[doc = "Checks if the value of the field is `CYCLES12_5`"]
    #[inline(always)]
    pub fn is_cycles12_5(&self) -> bool {
        **self == SMP10_A::CYCLES12_5
    }
    #[doc = "Checks if the value of the field is `CYCLES24_5`"]
    #[inline(always)]
    pub fn is_cycles24_5(&self) -> bool {
        **self == SMP10_A::CYCLES24_5
    }
    #[doc = "Checks if the value of the field is `CYCLES47_5`"]
    #[inline(always)]
    pub fn is_cycles47_5(&self) -> bool {
        **self == SMP10_A::CYCLES47_5
    }
    #[doc = "Checks if the value of the field is `CYCLES92_5`"]
    #[inline(always)]
    pub fn is_cycles92_5(&self) -> bool {
        **self == SMP10_A::CYCLES92_5
    }
    #[doc = "Checks if the value of the field is `CYCLES247_5`"]
    #[inline(always)]
    pub fn is_cycles247_5(&self) -> bool {
        **self == SMP10_A::CYCLES247_5
    }
    #[doc = "Checks if the value of the field is `CYCLES640_5`"]
    #[inline(always)]
    pub fn is_cycles640_5(&self) -> bool {
        **self == SMP10_A::CYCLES640_5
    }
}
impl core::ops::Deref for SMP10_R {
    type Target = crate::FieldReader<u8, SMP10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP10` writer - Channel 10 sampling time selection"]
pub struct SMP10_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP10_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP10_A::CYCLES2_5)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles6_5(self) -> &'a mut W {
        self.variant(SMP10_A::CYCLES6_5)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP10_A::CYCLES12_5)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles24_5(self) -> &'a mut W {
        self.variant(SMP10_A::CYCLES24_5)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles47_5(self) -> &'a mut W {
        self.variant(SMP10_A::CYCLES47_5)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles92_5(self) -> &'a mut W {
        self.variant(SMP10_A::CYCLES92_5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles247_5(self) -> &'a mut W {
        self.variant(SMP10_A::CYCLES247_5)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles640_5(self) -> &'a mut W {
        self.variant(SMP10_A::CYCLES640_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:26 - Channel 18 sampling time selection"]
    #[inline(always)]
    pub fn smp18(&self) -> SMP18_R {
        SMP18_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 21:23 - Channel 17 sampling time selection"]
    #[inline(always)]
    pub fn smp17(&self) -> SMP17_R {
        SMP17_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 18:20 - Channel 16 sampling time selection"]
    #[inline(always)]
    pub fn smp16(&self) -> SMP16_R {
        SMP16_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 15:17 - Channel 15 sampling time selection"]
    #[inline(always)]
    pub fn smp15(&self) -> SMP15_R {
        SMP15_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Channel 14 sampling time selection"]
    #[inline(always)]
    pub fn smp14(&self) -> SMP14_R {
        SMP14_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - Channel 13 sampling time selection"]
    #[inline(always)]
    pub fn smp13(&self) -> SMP13_R {
        SMP13_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - Channel 11 sampling time selection"]
    #[inline(always)]
    pub fn smp12(&self) -> SMP12_R {
        SMP12_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Channel 12 sampling time selection"]
    #[inline(always)]
    pub fn smp11(&self) -> SMP11_R {
        SMP11_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - Channel 10 sampling time selection"]
    #[inline(always)]
    pub fn smp10(&self) -> SMP10_R {
        SMP10_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 24:26 - Channel 18 sampling time selection"]
    #[inline(always)]
    pub fn smp18(&mut self) -> SMP18_W {
        SMP18_W { w: self }
    }
    #[doc = "Bits 21:23 - Channel 17 sampling time selection"]
    #[inline(always)]
    pub fn smp17(&mut self) -> SMP17_W {
        SMP17_W { w: self }
    }
    #[doc = "Bits 18:20 - Channel 16 sampling time selection"]
    #[inline(always)]
    pub fn smp16(&mut self) -> SMP16_W {
        SMP16_W { w: self }
    }
    #[doc = "Bits 15:17 - Channel 15 sampling time selection"]
    #[inline(always)]
    pub fn smp15(&mut self) -> SMP15_W {
        SMP15_W { w: self }
    }
    #[doc = "Bits 12:14 - Channel 14 sampling time selection"]
    #[inline(always)]
    pub fn smp14(&mut self) -> SMP14_W {
        SMP14_W { w: self }
    }
    #[doc = "Bits 9:11 - Channel 13 sampling time selection"]
    #[inline(always)]
    pub fn smp13(&mut self) -> SMP13_W {
        SMP13_W { w: self }
    }
    #[doc = "Bits 6:8 - Channel 11 sampling time selection"]
    #[inline(always)]
    pub fn smp12(&mut self) -> SMP12_W {
        SMP12_W { w: self }
    }
    #[doc = "Bits 3:5 - Channel 12 sampling time selection"]
    #[inline(always)]
    pub fn smp11(&mut self) -> SMP11_W {
        SMP11_W { w: self }
    }
    #[doc = "Bits 0:2 - Channel 10 sampling time selection"]
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
#[doc = "sample time register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr2](index.html) module"]
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
