#[doc = "Reader of register SMPR1"]
pub type R = crate::R<u32, super::SMPR1>;
#[doc = "Writer for register SMPR1"]
pub type W = crate::W<u32, super::SMPR1>;
#[doc = "Register SMPR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SMPR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel 9 sampling time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMP9_A {
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
impl From<SMP9_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP9_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SMP9`"]
pub type SMP9_R = crate::R<u8, SMP9_A>;
impl SMP9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP9_A {
        match self.bits {
            0 => SMP9_A::CYCLES2_5,
            1 => SMP9_A::CYCLES6_5,
            2 => SMP9_A::CYCLES12_5,
            3 => SMP9_A::CYCLES24_5,
            4 => SMP9_A::CYCLES47_5,
            5 => SMP9_A::CYCLES92_5,
            6 => SMP9_A::CYCLES247_5,
            7 => SMP9_A::CYCLES640_5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES2_5`"]
    #[inline(always)]
    pub fn is_cycles2_5(&self) -> bool {
        *self == SMP9_A::CYCLES2_5
    }
    #[doc = "Checks if the value of the field is `CYCLES6_5`"]
    #[inline(always)]
    pub fn is_cycles6_5(&self) -> bool {
        *self == SMP9_A::CYCLES6_5
    }
    #[doc = "Checks if the value of the field is `CYCLES12_5`"]
    #[inline(always)]
    pub fn is_cycles12_5(&self) -> bool {
        *self == SMP9_A::CYCLES12_5
    }
    #[doc = "Checks if the value of the field is `CYCLES24_5`"]
    #[inline(always)]
    pub fn is_cycles24_5(&self) -> bool {
        *self == SMP9_A::CYCLES24_5
    }
    #[doc = "Checks if the value of the field is `CYCLES47_5`"]
    #[inline(always)]
    pub fn is_cycles47_5(&self) -> bool {
        *self == SMP9_A::CYCLES47_5
    }
    #[doc = "Checks if the value of the field is `CYCLES92_5`"]
    #[inline(always)]
    pub fn is_cycles92_5(&self) -> bool {
        *self == SMP9_A::CYCLES92_5
    }
    #[doc = "Checks if the value of the field is `CYCLES247_5`"]
    #[inline(always)]
    pub fn is_cycles247_5(&self) -> bool {
        *self == SMP9_A::CYCLES247_5
    }
    #[doc = "Checks if the value of the field is `CYCLES640_5`"]
    #[inline(always)]
    pub fn is_cycles640_5(&self) -> bool {
        *self == SMP9_A::CYCLES640_5
    }
}
#[doc = "Write proxy for field `SMP9`"]
pub struct SMP9_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP9_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
#[doc = "Channel 8 sampling time selection"]
pub type SMP8_A = SMP9_A;
#[doc = "Reader of field `SMP8`"]
pub type SMP8_R = crate::R<u8, SMP9_A>;
#[doc = "Write proxy for field `SMP8`"]
pub struct SMP8_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP8_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Channel 7 sampling time selection"]
pub type SMP7_A = SMP9_A;
#[doc = "Reader of field `SMP7`"]
pub type SMP7_R = crate::R<u8, SMP9_A>;
#[doc = "Write proxy for field `SMP7`"]
pub struct SMP7_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP7_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
#[doc = "Channel 6 sampling time selection"]
pub type SMP6_A = SMP9_A;
#[doc = "Reader of field `SMP6`"]
pub type SMP6_R = crate::R<u8, SMP9_A>;
#[doc = "Write proxy for field `SMP6`"]
pub struct SMP6_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP6_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Channel 5 sampling time selection"]
pub type SMP5_A = SMP9_A;
#[doc = "Reader of field `SMP5`"]
pub type SMP5_R = crate::R<u8, SMP9_A>;
#[doc = "Write proxy for field `SMP5`"]
pub struct SMP5_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP5_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x07 << 15)) | (((value as u32) & 0x07) << 15);
        self.w
    }
}
#[doc = "Channel 4 sampling time selection"]
pub type SMP4_A = SMP9_A;
#[doc = "Reader of field `SMP4`"]
pub type SMP4_R = crate::R<u8, SMP9_A>;
#[doc = "Write proxy for field `SMP4`"]
pub struct SMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP4_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Channel 3 sampling time selection"]
pub type SMP3_A = SMP9_A;
#[doc = "Reader of field `SMP3`"]
pub type SMP3_R = crate::R<u8, SMP9_A>;
#[doc = "Write proxy for field `SMP3`"]
pub struct SMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Channel 2 sampling time selection"]
pub type SMP2_A = SMP9_A;
#[doc = "Reader of field `SMP2`"]
pub type SMP2_R = crate::R<u8, SMP9_A>;
#[doc = "Write proxy for field `SMP2`"]
pub struct SMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Channel 1 sampling time selection"]
pub type SMP1_A = SMP9_A;
#[doc = "Reader of field `SMP1`"]
pub type SMP1_R = crate::R<u8, SMP9_A>;
#[doc = "Write proxy for field `SMP1`"]
pub struct SMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
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
#[doc = "Reader of field `SMPPLUS`"]
pub type SMPPLUS_R = crate::R<bool, SMPPLUS_A>;
impl SMPPLUS_R {
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
        *self == SMPPLUS_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `PLUS1`"]
    #[inline(always)]
    pub fn is_plus1(&self) -> bool {
        *self == SMPPLUS_A::PLUS1
    }
}
#[doc = "Write proxy for field `SMPPLUS`"]
pub struct SMPPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPPLUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPPLUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Channel 0 sampling time selection"]
pub type SMP0_A = SMP9_A;
#[doc = "Reader of field `SMP0`"]
pub type SMP0_R = crate::R<u8, SMP9_A>;
#[doc = "Write proxy for field `SMP0`"]
pub struct SMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
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
}
