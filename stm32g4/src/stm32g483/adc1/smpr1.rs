#[doc = "Register `SMPR1` reader"]
pub struct R(crate::R<SMPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPR1` writer"]
pub struct W(crate::W<SMPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR1_SPEC>;
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
impl From<crate::W<SMPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 9 sampling time selection"]
pub type SMP9_A = SMP0_A;
#[doc = "Field `SMP9` reader - Channel 9 sampling time selection"]
pub type SMP9_R = SMP0_R;
#[doc = "Field `SMP9` writer - Channel 9 sampling time selection"]
pub struct SMP9_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP9_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP9_A::CYCLES2_5)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles6_5(self) -> &'a mut W {
        self.variant(SMP9_A::CYCLES6_5)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP9_A::CYCLES12_5)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles24_5(self) -> &'a mut W {
        self.variant(SMP9_A::CYCLES24_5)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles47_5(self) -> &'a mut W {
        self.variant(SMP9_A::CYCLES47_5)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles92_5(self) -> &'a mut W {
        self.variant(SMP9_A::CYCLES92_5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles247_5(self) -> &'a mut W {
        self.variant(SMP9_A::CYCLES247_5)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles640_5(self) -> &'a mut W {
        self.variant(SMP9_A::CYCLES640_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | ((value as u32 & 0x07) << 27);
        self.w
    }
}
#[doc = "Channel 8 sampling time selection"]
pub type SMP8_A = SMP0_A;
#[doc = "Field `SMP8` reader - Channel 8 sampling time selection"]
pub type SMP8_R = SMP0_R;
#[doc = "Field `SMP8` writer - Channel 8 sampling time selection"]
pub struct SMP8_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP8_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP8_A::CYCLES2_5)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles6_5(self) -> &'a mut W {
        self.variant(SMP8_A::CYCLES6_5)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP8_A::CYCLES12_5)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles24_5(self) -> &'a mut W {
        self.variant(SMP8_A::CYCLES24_5)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles47_5(self) -> &'a mut W {
        self.variant(SMP8_A::CYCLES47_5)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles92_5(self) -> &'a mut W {
        self.variant(SMP8_A::CYCLES92_5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles247_5(self) -> &'a mut W {
        self.variant(SMP8_A::CYCLES247_5)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles640_5(self) -> &'a mut W {
        self.variant(SMP8_A::CYCLES640_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Channel 7 sampling time selection"]
pub type SMP7_A = SMP0_A;
#[doc = "Field `SMP7` reader - Channel 7 sampling time selection"]
pub type SMP7_R = SMP0_R;
#[doc = "Field `SMP7` writer - Channel 7 sampling time selection"]
pub struct SMP7_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP7_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP7_A::CYCLES2_5)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles6_5(self) -> &'a mut W {
        self.variant(SMP7_A::CYCLES6_5)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP7_A::CYCLES12_5)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles24_5(self) -> &'a mut W {
        self.variant(SMP7_A::CYCLES24_5)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles47_5(self) -> &'a mut W {
        self.variant(SMP7_A::CYCLES47_5)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles92_5(self) -> &'a mut W {
        self.variant(SMP7_A::CYCLES92_5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles247_5(self) -> &'a mut W {
        self.variant(SMP7_A::CYCLES247_5)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles640_5(self) -> &'a mut W {
        self.variant(SMP7_A::CYCLES640_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | ((value as u32 & 0x07) << 21);
        self.w
    }
}
#[doc = "Channel 6 sampling time selection"]
pub type SMP6_A = SMP0_A;
#[doc = "Field `SMP6` reader - Channel 6 sampling time selection"]
pub type SMP6_R = SMP0_R;
#[doc = "Field `SMP6` writer - Channel 6 sampling time selection"]
pub struct SMP6_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP6_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP6_A::CYCLES2_5)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles6_5(self) -> &'a mut W {
        self.variant(SMP6_A::CYCLES6_5)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP6_A::CYCLES12_5)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles24_5(self) -> &'a mut W {
        self.variant(SMP6_A::CYCLES24_5)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles47_5(self) -> &'a mut W {
        self.variant(SMP6_A::CYCLES47_5)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles92_5(self) -> &'a mut W {
        self.variant(SMP6_A::CYCLES92_5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles247_5(self) -> &'a mut W {
        self.variant(SMP6_A::CYCLES247_5)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles640_5(self) -> &'a mut W {
        self.variant(SMP6_A::CYCLES640_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Channel 5 sampling time selection"]
pub type SMP5_A = SMP0_A;
#[doc = "Field `SMP5` reader - Channel 5 sampling time selection"]
pub type SMP5_R = SMP0_R;
#[doc = "Field `SMP5` writer - Channel 5 sampling time selection"]
pub struct SMP5_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP5_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP5_A::CYCLES2_5)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles6_5(self) -> &'a mut W {
        self.variant(SMP5_A::CYCLES6_5)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP5_A::CYCLES12_5)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles24_5(self) -> &'a mut W {
        self.variant(SMP5_A::CYCLES24_5)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles47_5(self) -> &'a mut W {
        self.variant(SMP5_A::CYCLES47_5)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles92_5(self) -> &'a mut W {
        self.variant(SMP5_A::CYCLES92_5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles247_5(self) -> &'a mut W {
        self.variant(SMP5_A::CYCLES247_5)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles640_5(self) -> &'a mut W {
        self.variant(SMP5_A::CYCLES640_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | ((value as u32 & 0x07) << 15);
        self.w
    }
}
#[doc = "Channel 4 sampling time selection"]
pub type SMP4_A = SMP0_A;
#[doc = "Field `SMP4` reader - Channel 4 sampling time selection"]
pub type SMP4_R = SMP0_R;
#[doc = "Field `SMP4` writer - Channel 4 sampling time selection"]
pub struct SMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP4_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP4_A::CYCLES2_5)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles6_5(self) -> &'a mut W {
        self.variant(SMP4_A::CYCLES6_5)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP4_A::CYCLES12_5)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles24_5(self) -> &'a mut W {
        self.variant(SMP4_A::CYCLES24_5)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles47_5(self) -> &'a mut W {
        self.variant(SMP4_A::CYCLES47_5)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles92_5(self) -> &'a mut W {
        self.variant(SMP4_A::CYCLES92_5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles247_5(self) -> &'a mut W {
        self.variant(SMP4_A::CYCLES247_5)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles640_5(self) -> &'a mut W {
        self.variant(SMP4_A::CYCLES640_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Channel 3 sampling time selection"]
pub type SMP3_A = SMP0_A;
#[doc = "Field `SMP3` reader - Channel 3 sampling time selection"]
pub type SMP3_R = SMP0_R;
#[doc = "Field `SMP3` writer - Channel 3 sampling time selection"]
pub struct SMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP3_A::CYCLES2_5)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles6_5(self) -> &'a mut W {
        self.variant(SMP3_A::CYCLES6_5)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP3_A::CYCLES12_5)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles24_5(self) -> &'a mut W {
        self.variant(SMP3_A::CYCLES24_5)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles47_5(self) -> &'a mut W {
        self.variant(SMP3_A::CYCLES47_5)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles92_5(self) -> &'a mut W {
        self.variant(SMP3_A::CYCLES92_5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles247_5(self) -> &'a mut W {
        self.variant(SMP3_A::CYCLES247_5)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles640_5(self) -> &'a mut W {
        self.variant(SMP3_A::CYCLES640_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Channel 2 sampling time selection"]
pub type SMP2_A = SMP0_A;
#[doc = "Field `SMP2` reader - Channel 2 sampling time selection"]
pub type SMP2_R = SMP0_R;
#[doc = "Field `SMP2` writer - Channel 2 sampling time selection"]
pub struct SMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP2_A::CYCLES2_5)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles6_5(self) -> &'a mut W {
        self.variant(SMP2_A::CYCLES6_5)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP2_A::CYCLES12_5)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles24_5(self) -> &'a mut W {
        self.variant(SMP2_A::CYCLES24_5)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles47_5(self) -> &'a mut W {
        self.variant(SMP2_A::CYCLES47_5)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles92_5(self) -> &'a mut W {
        self.variant(SMP2_A::CYCLES92_5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles247_5(self) -> &'a mut W {
        self.variant(SMP2_A::CYCLES247_5)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles640_5(self) -> &'a mut W {
        self.variant(SMP2_A::CYCLES640_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Channel 1 sampling time selection"]
pub type SMP1_A = SMP0_A;
#[doc = "Field `SMP1` reader - Channel 1 sampling time selection"]
pub type SMP1_R = SMP0_R;
#[doc = "Field `SMP1` writer - Channel 1 sampling time selection"]
pub struct SMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP1_A::CYCLES2_5)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles6_5(self) -> &'a mut W {
        self.variant(SMP1_A::CYCLES6_5)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP1_A::CYCLES12_5)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles24_5(self) -> &'a mut W {
        self.variant(SMP1_A::CYCLES24_5)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles47_5(self) -> &'a mut W {
        self.variant(SMP1_A::CYCLES47_5)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles92_5(self) -> &'a mut W {
        self.variant(SMP1_A::CYCLES92_5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles247_5(self) -> &'a mut W {
        self.variant(SMP1_A::CYCLES247_5)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles640_5(self) -> &'a mut W {
        self.variant(SMP1_A::CYCLES640_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Addition of one clock cycle to the sampling time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPPLUS_A {
    #[doc = "0: 2.5 in SMPR remains 2.5 cycles"]
    NORMAL = 0,
    #[doc = "1: 2.5 in SMPR becomes 3.5 cycles"]
    PLUS1 = 1,
}
impl From<SMPPLUS_A> for bool {
    #[inline(always)]
    fn from(variant: SMPPLUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPPLUS` reader - Addition of one clock cycle to the sampling time"]
pub struct SMPPLUS_R(crate::FieldReader<bool, SMPPLUS_A>);
impl SMPPLUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMPPLUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPPLUS_A {
        match self.bits {
            false => SMPPLUS_A::NORMAL,
            true => SMPPLUS_A::PLUS1,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == SMPPLUS_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `PLUS1`"]
    #[inline(always)]
    pub fn is_plus1(&self) -> bool {
        **self == SMPPLUS_A::PLUS1
    }
}
impl core::ops::Deref for SMPPLUS_R {
    type Target = crate::FieldReader<bool, SMPPLUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPPLUS` writer - Addition of one clock cycle to the sampling time"]
pub struct SMPPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPPLUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPPLUS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "2.5 in SMPR remains 2.5 cycles"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SMPPLUS_A::NORMAL)
    }
    #[doc = "2.5 in SMPR becomes 3.5 cycles"]
    #[inline(always)]
    pub fn plus1(self) -> &'a mut W {
        self.variant(SMPPLUS_A::PLUS1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Channel 0 sampling time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMP0_A {
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
impl From<SMP0_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SMP0` reader - Channel 0 sampling time selection"]
pub struct SMP0_R(crate::FieldReader<u8, SMP0_A>);
impl SMP0_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP0_A {
        match self.bits {
            0 => SMP0_A::CYCLES2_5,
            1 => SMP0_A::CYCLES6_5,
            2 => SMP0_A::CYCLES12_5,
            3 => SMP0_A::CYCLES24_5,
            4 => SMP0_A::CYCLES47_5,
            5 => SMP0_A::CYCLES92_5,
            6 => SMP0_A::CYCLES247_5,
            7 => SMP0_A::CYCLES640_5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES2_5`"]
    #[inline(always)]
    pub fn is_cycles2_5(&self) -> bool {
        **self == SMP0_A::CYCLES2_5
    }
    #[doc = "Checks if the value of the field is `CYCLES6_5`"]
    #[inline(always)]
    pub fn is_cycles6_5(&self) -> bool {
        **self == SMP0_A::CYCLES6_5
    }
    #[doc = "Checks if the value of the field is `CYCLES12_5`"]
    #[inline(always)]
    pub fn is_cycles12_5(&self) -> bool {
        **self == SMP0_A::CYCLES12_5
    }
    #[doc = "Checks if the value of the field is `CYCLES24_5`"]
    #[inline(always)]
    pub fn is_cycles24_5(&self) -> bool {
        **self == SMP0_A::CYCLES24_5
    }
    #[doc = "Checks if the value of the field is `CYCLES47_5`"]
    #[inline(always)]
    pub fn is_cycles47_5(&self) -> bool {
        **self == SMP0_A::CYCLES47_5
    }
    #[doc = "Checks if the value of the field is `CYCLES92_5`"]
    #[inline(always)]
    pub fn is_cycles92_5(&self) -> bool {
        **self == SMP0_A::CYCLES92_5
    }
    #[doc = "Checks if the value of the field is `CYCLES247_5`"]
    #[inline(always)]
    pub fn is_cycles247_5(&self) -> bool {
        **self == SMP0_A::CYCLES247_5
    }
    #[doc = "Checks if the value of the field is `CYCLES640_5`"]
    #[inline(always)]
    pub fn is_cycles640_5(&self) -> bool {
        **self == SMP0_A::CYCLES640_5
    }
}
impl core::ops::Deref for SMP0_R {
    type Target = crate::FieldReader<u8, SMP0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP0` writer - Channel 0 sampling time selection"]
pub struct SMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP0_A::CYCLES2_5)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles6_5(self) -> &'a mut W {
        self.variant(SMP0_A::CYCLES6_5)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP0_A::CYCLES12_5)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles24_5(self) -> &'a mut W {
        self.variant(SMP0_A::CYCLES24_5)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles47_5(self) -> &'a mut W {
        self.variant(SMP0_A::CYCLES47_5)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles92_5(self) -> &'a mut W {
        self.variant(SMP0_A::CYCLES92_5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles247_5(self) -> &'a mut W {
        self.variant(SMP0_A::CYCLES247_5)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles640_5(self) -> &'a mut W {
        self.variant(SMP0_A::CYCLES640_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:29 - Channel 9 sampling time selection"]
    #[inline(always)]
    pub fn smp9(&self) -> SMP9_R {
        SMP9_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - Channel 8 sampling time selection"]
    #[inline(always)]
    pub fn smp8(&self) -> SMP8_R {
        SMP8_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 21:23 - Channel 7 sampling time selection"]
    #[inline(always)]
    pub fn smp7(&self) -> SMP7_R {
        SMP7_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 18:20 - Channel 6 sampling time selection"]
    #[inline(always)]
    pub fn smp6(&self) -> SMP6_R {
        SMP6_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 15:17 - Channel 5 sampling time selection"]
    #[inline(always)]
    pub fn smp5(&self) -> SMP5_R {
        SMP5_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Channel 4 sampling time selection"]
    #[inline(always)]
    pub fn smp4(&self) -> SMP4_R {
        SMP4_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - Channel 3 sampling time selection"]
    #[inline(always)]
    pub fn smp3(&self) -> SMP3_R {
        SMP3_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - Channel 2 sampling time selection"]
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Channel 1 sampling time selection"]
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 31 - Addition of one clock cycle to the sampling time"]
    #[inline(always)]
    pub fn smpplus(&self) -> SMPPLUS_R {
        SMPPLUS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - Channel 0 sampling time selection"]
    #[inline(always)]
    pub fn smp0(&self) -> SMP0_R {
        SMP0_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 27:29 - Channel 9 sampling time selection"]
    #[inline(always)]
    pub fn smp9(&mut self) -> SMP9_W {
        SMP9_W { w: self }
    }
    #[doc = "Bits 24:26 - Channel 8 sampling time selection"]
    #[inline(always)]
    pub fn smp8(&mut self) -> SMP8_W {
        SMP8_W { w: self }
    }
    #[doc = "Bits 21:23 - Channel 7 sampling time selection"]
    #[inline(always)]
    pub fn smp7(&mut self) -> SMP7_W {
        SMP7_W { w: self }
    }
    #[doc = "Bits 18:20 - Channel 6 sampling time selection"]
    #[inline(always)]
    pub fn smp6(&mut self) -> SMP6_W {
        SMP6_W { w: self }
    }
    #[doc = "Bits 15:17 - Channel 5 sampling time selection"]
    #[inline(always)]
    pub fn smp5(&mut self) -> SMP5_W {
        SMP5_W { w: self }
    }
    #[doc = "Bits 12:14 - Channel 4 sampling time selection"]
    #[inline(always)]
    pub fn smp4(&mut self) -> SMP4_W {
        SMP4_W { w: self }
    }
    #[doc = "Bits 9:11 - Channel 3 sampling time selection"]
    #[inline(always)]
    pub fn smp3(&mut self) -> SMP3_W {
        SMP3_W { w: self }
    }
    #[doc = "Bits 6:8 - Channel 2 sampling time selection"]
    #[inline(always)]
    pub fn smp2(&mut self) -> SMP2_W {
        SMP2_W { w: self }
    }
    #[doc = "Bits 3:5 - Channel 1 sampling time selection"]
    #[inline(always)]
    pub fn smp1(&mut self) -> SMP1_W {
        SMP1_W { w: self }
    }
    #[doc = "Bit 31 - Addition of one clock cycle to the sampling time"]
    #[inline(always)]
    pub fn smpplus(&mut self) -> SMPPLUS_W {
        SMPPLUS_W { w: self }
    }
    #[doc = "Bits 0:2 - Channel 0 sampling time selection"]
    #[inline(always)]
    pub fn smp0(&mut self) -> SMP0_W {
        SMP0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sample time register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr1](index.html) module"]
pub struct SMPR1_SPEC;
impl crate::RegisterSpec for SMPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpr1::R](R) reader structure"]
impl crate::Readable for SMPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpr1::W](W) writer structure"]
impl crate::Writable for SMPR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMPR1 to value 0"]
impl crate::Resettable for SMPR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
