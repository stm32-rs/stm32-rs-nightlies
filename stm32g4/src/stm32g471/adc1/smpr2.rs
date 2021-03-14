#[doc = "Reader of register SMPR2"]
pub type R = crate::R<u32, super::SMPR2>;
#[doc = "Writer for register SMPR2"]
pub type W = crate::W<u32, super::SMPR2>;
#[doc = "Register SMPR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SMPR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel 18 sampling time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMP18_A {
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
impl From<SMP18_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP18_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SMP18`"]
pub type SMP18_R = crate::R<u8, SMP18_A>;
impl SMP18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP18_A {
        match self.bits {
            0 => SMP18_A::CYCLES2_5,
            1 => SMP18_A::CYCLES6_5,
            2 => SMP18_A::CYCLES12_5,
            3 => SMP18_A::CYCLES24_5,
            4 => SMP18_A::CYCLES47_5,
            5 => SMP18_A::CYCLES92_5,
            6 => SMP18_A::CYCLES247_5,
            7 => SMP18_A::CYCLES640_5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES2_5`"]
    #[inline(always)]
    pub fn is_cycles2_5(&self) -> bool {
        *self == SMP18_A::CYCLES2_5
    }
    #[doc = "Checks if the value of the field is `CYCLES6_5`"]
    #[inline(always)]
    pub fn is_cycles6_5(&self) -> bool {
        *self == SMP18_A::CYCLES6_5
    }
    #[doc = "Checks if the value of the field is `CYCLES12_5`"]
    #[inline(always)]
    pub fn is_cycles12_5(&self) -> bool {
        *self == SMP18_A::CYCLES12_5
    }
    #[doc = "Checks if the value of the field is `CYCLES24_5`"]
    #[inline(always)]
    pub fn is_cycles24_5(&self) -> bool {
        *self == SMP18_A::CYCLES24_5
    }
    #[doc = "Checks if the value of the field is `CYCLES47_5`"]
    #[inline(always)]
    pub fn is_cycles47_5(&self) -> bool {
        *self == SMP18_A::CYCLES47_5
    }
    #[doc = "Checks if the value of the field is `CYCLES92_5`"]
    #[inline(always)]
    pub fn is_cycles92_5(&self) -> bool {
        *self == SMP18_A::CYCLES92_5
    }
    #[doc = "Checks if the value of the field is `CYCLES247_5`"]
    #[inline(always)]
    pub fn is_cycles247_5(&self) -> bool {
        *self == SMP18_A::CYCLES247_5
    }
    #[doc = "Checks if the value of the field is `CYCLES640_5`"]
    #[inline(always)]
    pub fn is_cycles640_5(&self) -> bool {
        *self == SMP18_A::CYCLES640_5
    }
}
#[doc = "Write proxy for field `SMP18`"]
pub struct SMP18_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP18_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Channel 17 sampling time selection"]
pub type SMP17_A = SMP18_A;
#[doc = "Reader of field `SMP17`"]
pub type SMP17_R = crate::R<u8, SMP18_A>;
#[doc = "Write proxy for field `SMP17`"]
pub struct SMP17_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP17_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
#[doc = "Channel 16 sampling time selection"]
pub type SMP16_A = SMP18_A;
#[doc = "Reader of field `SMP16`"]
pub type SMP16_R = crate::R<u8, SMP18_A>;
#[doc = "Write proxy for field `SMP16`"]
pub struct SMP16_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP16_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Channel 15 sampling time selection"]
pub type SMP15_A = SMP18_A;
#[doc = "Reader of field `SMP15`"]
pub type SMP15_R = crate::R<u8, SMP18_A>;
#[doc = "Write proxy for field `SMP15`"]
pub struct SMP15_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP15_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x07 << 15)) | (((value as u32) & 0x07) << 15);
        self.w
    }
}
#[doc = "Channel 14 sampling time selection"]
pub type SMP14_A = SMP18_A;
#[doc = "Reader of field `SMP14`"]
pub type SMP14_R = crate::R<u8, SMP18_A>;
#[doc = "Write proxy for field `SMP14`"]
pub struct SMP14_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP14_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Channel 13 sampling time selection"]
pub type SMP13_A = SMP18_A;
#[doc = "Reader of field `SMP13`"]
pub type SMP13_R = crate::R<u8, SMP18_A>;
#[doc = "Write proxy for field `SMP13`"]
pub struct SMP13_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP13_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Channel 11 sampling time selection"]
pub type SMP12_A = SMP18_A;
#[doc = "Reader of field `SMP12`"]
pub type SMP12_R = crate::R<u8, SMP18_A>;
#[doc = "Write proxy for field `SMP12`"]
pub struct SMP12_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP12_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Channel 12 sampling time selection"]
pub type SMP11_A = SMP18_A;
#[doc = "Reader of field `SMP11`"]
pub type SMP11_R = crate::R<u8, SMP18_A>;
#[doc = "Write proxy for field `SMP11`"]
pub struct SMP11_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP11_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Channel 10 sampling time selection"]
pub type SMP10_A = SMP18_A;
#[doc = "Reader of field `SMP10`"]
pub type SMP10_R = crate::R<u8, SMP18_A>;
#[doc = "Write proxy for field `SMP10`"]
pub struct SMP10_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP10_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
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
}
