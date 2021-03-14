#[doc = "Reader of register SMPR"]
pub type R = crate::R<u32, super::SMPR>;
#[doc = "Writer for register SMPR"]
pub type W = crate::W<u32, super::SMPR>;
#[doc = "Register SMPR `reset()`'s with value 0"]
impl crate::ResetValue for super::SMPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Sampling time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMP_A {
    #[doc = "0: 1.5 ADC clock cycles"]
    CYCLES1_5 = 0,
    #[doc = "1: 3.5 ADC clock cycles"]
    CYCLES3_5 = 1,
    #[doc = "2: 7.5 ADC clock cycles"]
    CYCLES7_5 = 2,
    #[doc = "3: 12.5 ADC clock cycles"]
    CYCLES12_5 = 3,
    #[doc = "4: 19.5 ADC clock cycles"]
    CYCLES19_5 = 4,
    #[doc = "5: 39.5 ADC clock cycles"]
    CYCLES39_5 = 5,
    #[doc = "6: 79.5 ADC clock cycles"]
    CYCLES79_5 = 6,
    #[doc = "7: 160.5 ADC clock cycles"]
    CYCLES160_5 = 7,
}
impl From<SMP_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SMP`"]
pub type SMP_R = crate::R<u8, SMP_A>;
impl SMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP_A {
        match self.bits {
            0 => SMP_A::CYCLES1_5,
            1 => SMP_A::CYCLES3_5,
            2 => SMP_A::CYCLES7_5,
            3 => SMP_A::CYCLES12_5,
            4 => SMP_A::CYCLES19_5,
            5 => SMP_A::CYCLES39_5,
            6 => SMP_A::CYCLES79_5,
            7 => SMP_A::CYCLES160_5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES1_5`"]
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SMP_A::CYCLES1_5
    }
    #[doc = "Checks if the value of the field is `CYCLES3_5`"]
    #[inline(always)]
    pub fn is_cycles3_5(&self) -> bool {
        *self == SMP_A::CYCLES3_5
    }
    #[doc = "Checks if the value of the field is `CYCLES7_5`"]
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == SMP_A::CYCLES7_5
    }
    #[doc = "Checks if the value of the field is `CYCLES12_5`"]
    #[inline(always)]
    pub fn is_cycles12_5(&self) -> bool {
        *self == SMP_A::CYCLES12_5
    }
    #[doc = "Checks if the value of the field is `CYCLES19_5`"]
    #[inline(always)]
    pub fn is_cycles19_5(&self) -> bool {
        *self == SMP_A::CYCLES19_5
    }
    #[doc = "Checks if the value of the field is `CYCLES39_5`"]
    #[inline(always)]
    pub fn is_cycles39_5(&self) -> bool {
        *self == SMP_A::CYCLES39_5
    }
    #[doc = "Checks if the value of the field is `CYCLES79_5`"]
    #[inline(always)]
    pub fn is_cycles79_5(&self) -> bool {
        *self == SMP_A::CYCLES79_5
    }
    #[doc = "Checks if the value of the field is `CYCLES160_5`"]
    #[inline(always)]
    pub fn is_cycles160_5(&self) -> bool {
        *self == SMP_A::CYCLES160_5
    }
}
#[doc = "Write proxy for field `SMP`"]
pub struct SMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP_A::CYCLES1_5)
    }
    #[doc = "3.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles3_5(self) -> &'a mut W {
        self.variant(SMP_A::CYCLES3_5)
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut W {
        self.variant(SMP_A::CYCLES7_5)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP_A::CYCLES12_5)
    }
    #[doc = "19.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles19_5(self) -> &'a mut W {
        self.variant(SMP_A::CYCLES19_5)
    }
    #[doc = "39.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles39_5(self) -> &'a mut W {
        self.variant(SMP_A::CYCLES39_5)
    }
    #[doc = "79.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles79_5(self) -> &'a mut W {
        self.variant(SMP_A::CYCLES79_5)
    }
    #[doc = "160.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles160_5(self) -> &'a mut W {
        self.variant(SMP_A::CYCLES160_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Sampling time selection"]
    #[inline(always)]
    pub fn smp(&self) -> SMP_R {
        SMP_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sampling time selection"]
    #[inline(always)]
    pub fn smp(&mut self) -> SMP_W {
        SMP_W { w: self }
    }
}
