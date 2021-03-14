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
    #[doc = "0: 1.5 cycles"]
    CYCLES1_5 = 0,
    #[doc = "1: 7.5 cycles"]
    CYCLES7_5 = 1,
    #[doc = "2: 13.5 cycles"]
    CYCLES13_5 = 2,
    #[doc = "3: 28.5 cycles"]
    CYCLES28_5 = 3,
    #[doc = "4: 41.5 cycles"]
    CYCLES41_5 = 4,
    #[doc = "5: 55.5 cycles"]
    CYCLES55_5 = 5,
    #[doc = "6: 71.5 cycles"]
    CYCLES71_5 = 6,
    #[doc = "7: 239.5 cycles"]
    CYCLES239_5 = 7,
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
            1 => SMP_A::CYCLES7_5,
            2 => SMP_A::CYCLES13_5,
            3 => SMP_A::CYCLES28_5,
            4 => SMP_A::CYCLES41_5,
            5 => SMP_A::CYCLES55_5,
            6 => SMP_A::CYCLES71_5,
            7 => SMP_A::CYCLES239_5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES1_5`"]
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SMP_A::CYCLES1_5
    }
    #[doc = "Checks if the value of the field is `CYCLES7_5`"]
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == SMP_A::CYCLES7_5
    }
    #[doc = "Checks if the value of the field is `CYCLES13_5`"]
    #[inline(always)]
    pub fn is_cycles13_5(&self) -> bool {
        *self == SMP_A::CYCLES13_5
    }
    #[doc = "Checks if the value of the field is `CYCLES28_5`"]
    #[inline(always)]
    pub fn is_cycles28_5(&self) -> bool {
        *self == SMP_A::CYCLES28_5
    }
    #[doc = "Checks if the value of the field is `CYCLES41_5`"]
    #[inline(always)]
    pub fn is_cycles41_5(&self) -> bool {
        *self == SMP_A::CYCLES41_5
    }
    #[doc = "Checks if the value of the field is `CYCLES55_5`"]
    #[inline(always)]
    pub fn is_cycles55_5(&self) -> bool {
        *self == SMP_A::CYCLES55_5
    }
    #[doc = "Checks if the value of the field is `CYCLES71_5`"]
    #[inline(always)]
    pub fn is_cycles71_5(&self) -> bool {
        *self == SMP_A::CYCLES71_5
    }
    #[doc = "Checks if the value of the field is `CYCLES239_5`"]
    #[inline(always)]
    pub fn is_cycles239_5(&self) -> bool {
        *self == SMP_A::CYCLES239_5
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
    #[doc = "1.5 cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP_A::CYCLES1_5)
    }
    #[doc = "7.5 cycles"]
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut W {
        self.variant(SMP_A::CYCLES7_5)
    }
    #[doc = "13.5 cycles"]
    #[inline(always)]
    pub fn cycles13_5(self) -> &'a mut W {
        self.variant(SMP_A::CYCLES13_5)
    }
    #[doc = "28.5 cycles"]
    #[inline(always)]
    pub fn cycles28_5(self) -> &'a mut W {
        self.variant(SMP_A::CYCLES28_5)
    }
    #[doc = "41.5 cycles"]
    #[inline(always)]
    pub fn cycles41_5(self) -> &'a mut W {
        self.variant(SMP_A::CYCLES41_5)
    }
    #[doc = "55.5 cycles"]
    #[inline(always)]
    pub fn cycles55_5(self) -> &'a mut W {
        self.variant(SMP_A::CYCLES55_5)
    }
    #[doc = "71.5 cycles"]
    #[inline(always)]
    pub fn cycles71_5(self) -> &'a mut W {
        self.variant(SMP_A::CYCLES71_5)
    }
    #[doc = "239.5 cycles"]
    #[inline(always)]
    pub fn cycles239_5(self) -> &'a mut W {
        self.variant(SMP_A::CYCLES239_5)
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
