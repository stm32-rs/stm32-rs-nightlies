#[doc = "Reader of register DIFSEL"]
pub type R = crate::R<u32, super::DIFSEL>;
#[doc = "Writer for register DIFSEL"]
pub type W = crate::W<u32, super::DIFSEL>;
#[doc = "Register DIFSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::DIFSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ADC channel differential or single-ended mode for channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIFSEL0_A {
    #[doc = "0: Input channel is configured in single-ended mode"]
    SINGLEENDED = 0,
    #[doc = "1: Input channel is configured in differential mode"]
    DIFFERENTIAL = 1,
}
impl From<DIFSEL0_A> for bool {
    #[inline(always)]
    fn from(variant: DIFSEL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIFSEL0`"]
pub type DIFSEL0_R = crate::R<bool, DIFSEL0_A>;
impl DIFSEL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIFSEL0_A {
        match self.bits {
            false => DIFSEL0_A::SINGLEENDED,
            true => DIFSEL0_A::DIFFERENTIAL,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLEENDED`"]
    #[inline(always)]
    pub fn is_single_ended(&self) -> bool {
        *self == DIFSEL0_A::SINGLEENDED
    }
    #[doc = "Checks if the value of the field is `DIFFERENTIAL`"]
    #[inline(always)]
    pub fn is_differential(&self) -> bool {
        *self == DIFSEL0_A::DIFFERENTIAL
    }
}
#[doc = "Write proxy for field `DIFSEL0`"]
pub struct DIFSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL0_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL0_A::DIFFERENTIAL)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "ADC channel differential or single-ended mode for channel"]
pub type DIFSEL1_A = DIFSEL0_A;
#[doc = "Reader of field `DIFSEL1`"]
pub type DIFSEL1_R = crate::R<bool, DIFSEL0_A>;
#[doc = "Write proxy for field `DIFSEL1`"]
pub struct DIFSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL0_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL0_A::DIFFERENTIAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "ADC channel differential or single-ended mode for channel"]
pub type DIFSEL2_A = DIFSEL0_A;
#[doc = "Reader of field `DIFSEL2`"]
pub type DIFSEL2_R = crate::R<bool, DIFSEL0_A>;
#[doc = "Write proxy for field `DIFSEL2`"]
pub struct DIFSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL0_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL0_A::DIFFERENTIAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "ADC channel differential or single-ended mode for channel"]
pub type DIFSEL3_A = DIFSEL0_A;
#[doc = "Reader of field `DIFSEL3`"]
pub type DIFSEL3_R = crate::R<bool, DIFSEL0_A>;
#[doc = "Write proxy for field `DIFSEL3`"]
pub struct DIFSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL0_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL0_A::DIFFERENTIAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "ADC channel differential or single-ended mode for channel"]
pub type DIFSEL4_A = DIFSEL0_A;
#[doc = "Reader of field `DIFSEL4`"]
pub type DIFSEL4_R = crate::R<bool, DIFSEL0_A>;
#[doc = "Write proxy for field `DIFSEL4`"]
pub struct DIFSEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL0_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL0_A::DIFFERENTIAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "ADC channel differential or single-ended mode for channel"]
pub type DIFSEL5_A = DIFSEL0_A;
#[doc = "Reader of field `DIFSEL5`"]
pub type DIFSEL5_R = crate::R<bool, DIFSEL0_A>;
#[doc = "Write proxy for field `DIFSEL5`"]
pub struct DIFSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL0_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL0_A::DIFFERENTIAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "ADC channel differential or single-ended mode for channel"]
pub type DIFSEL6_A = DIFSEL0_A;
#[doc = "Reader of field `DIFSEL6`"]
pub type DIFSEL6_R = crate::R<bool, DIFSEL0_A>;
#[doc = "Write proxy for field `DIFSEL6`"]
pub struct DIFSEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL0_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL0_A::DIFFERENTIAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "ADC channel differential or single-ended mode for channel"]
pub type DIFSEL7_A = DIFSEL0_A;
#[doc = "Reader of field `DIFSEL7`"]
pub type DIFSEL7_R = crate::R<bool, DIFSEL0_A>;
#[doc = "Write proxy for field `DIFSEL7`"]
pub struct DIFSEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL0_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL0_A::DIFFERENTIAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "ADC channel differential or single-ended mode for channel"]
pub type DIFSEL8_A = DIFSEL0_A;
#[doc = "Reader of field `DIFSEL8`"]
pub type DIFSEL8_R = crate::R<bool, DIFSEL0_A>;
#[doc = "Write proxy for field `DIFSEL8`"]
pub struct DIFSEL8_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL0_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL0_A::DIFFERENTIAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "ADC channel differential or single-ended mode for channel"]
pub type DIFSEL9_A = DIFSEL0_A;
#[doc = "Reader of field `DIFSEL9`"]
pub type DIFSEL9_R = crate::R<bool, DIFSEL0_A>;
#[doc = "Write proxy for field `DIFSEL9`"]
pub struct DIFSEL9_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL0_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL0_A::DIFFERENTIAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "ADC channel differential or single-ended mode for channel"]
pub type DIFSEL10_A = DIFSEL0_A;
#[doc = "Reader of field `DIFSEL10`"]
pub type DIFSEL10_R = crate::R<bool, DIFSEL0_A>;
#[doc = "Write proxy for field `DIFSEL10`"]
pub struct DIFSEL10_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL0_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL0_A::DIFFERENTIAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "ADC channel differential or single-ended mode for channel"]
pub type DIFSEL11_A = DIFSEL0_A;
#[doc = "Reader of field `DIFSEL11`"]
pub type DIFSEL11_R = crate::R<bool, DIFSEL0_A>;
#[doc = "Write proxy for field `DIFSEL11`"]
pub struct DIFSEL11_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL0_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL0_A::DIFFERENTIAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "ADC channel differential or single-ended mode for channel"]
pub type DIFSEL12_A = DIFSEL0_A;
#[doc = "Reader of field `DIFSEL12`"]
pub type DIFSEL12_R = crate::R<bool, DIFSEL0_A>;
#[doc = "Write proxy for field `DIFSEL12`"]
pub struct DIFSEL12_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL0_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL0_A::DIFFERENTIAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "ADC channel differential or single-ended mode for channel"]
pub type DIFSEL13_A = DIFSEL0_A;
#[doc = "Reader of field `DIFSEL13`"]
pub type DIFSEL13_R = crate::R<bool, DIFSEL0_A>;
#[doc = "Write proxy for field `DIFSEL13`"]
pub struct DIFSEL13_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL0_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL0_A::DIFFERENTIAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "ADC channel differential or single-ended mode for channel"]
pub type DIFSEL14_A = DIFSEL0_A;
#[doc = "Reader of field `DIFSEL14`"]
pub type DIFSEL14_R = crate::R<bool, DIFSEL0_A>;
#[doc = "Write proxy for field `DIFSEL14`"]
pub struct DIFSEL14_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL0_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL0_A::DIFFERENTIAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "ADC channel differential or single-ended mode for channel"]
pub type DIFSEL15_A = DIFSEL0_A;
#[doc = "Reader of field `DIFSEL15`"]
pub type DIFSEL15_R = crate::R<bool, DIFSEL0_A>;
#[doc = "Write proxy for field `DIFSEL15`"]
pub struct DIFSEL15_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL0_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL0_A::DIFFERENTIAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "ADC channel differential or single-ended mode for channel"]
pub type DIFSEL16_A = DIFSEL0_A;
#[doc = "Reader of field `DIFSEL16`"]
pub type DIFSEL16_R = crate::R<bool, DIFSEL0_A>;
#[doc = "Write proxy for field `DIFSEL16`"]
pub struct DIFSEL16_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL0_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL0_A::DIFFERENTIAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "ADC channel differential or single-ended mode for channel"]
pub type DIFSEL17_A = DIFSEL0_A;
#[doc = "Reader of field `DIFSEL17`"]
pub type DIFSEL17_R = crate::R<bool, DIFSEL0_A>;
#[doc = "Write proxy for field `DIFSEL17`"]
pub struct DIFSEL17_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL0_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL0_A::DIFFERENTIAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "ADC channel differential or single-ended mode for channel"]
pub type DIFSEL18_A = DIFSEL0_A;
#[doc = "Reader of field `DIFSEL18`"]
pub type DIFSEL18_R = crate::R<bool, DIFSEL0_A>;
#[doc = "Write proxy for field `DIFSEL18`"]
pub struct DIFSEL18_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL0_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL0_A::DIFFERENTIAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "ADC channel differential or single-ended mode for channel"]
pub type DIFSEL19_A = DIFSEL0_A;
#[doc = "Reader of field `DIFSEL19`"]
pub type DIFSEL19_R = crate::R<bool, DIFSEL0_A>;
#[doc = "Write proxy for field `DIFSEL19`"]
pub struct DIFSEL19_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL0_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL0_A::DIFFERENTIAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel0(&self) -> DIFSEL0_R {
        DIFSEL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel1(&self) -> DIFSEL1_R {
        DIFSEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel2(&self) -> DIFSEL2_R {
        DIFSEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel3(&self) -> DIFSEL3_R {
        DIFSEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel4(&self) -> DIFSEL4_R {
        DIFSEL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel5(&self) -> DIFSEL5_R {
        DIFSEL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel6(&self) -> DIFSEL6_R {
        DIFSEL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel7(&self) -> DIFSEL7_R {
        DIFSEL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel8(&self) -> DIFSEL8_R {
        DIFSEL8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel9(&self) -> DIFSEL9_R {
        DIFSEL9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel10(&self) -> DIFSEL10_R {
        DIFSEL10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel11(&self) -> DIFSEL11_R {
        DIFSEL11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel12(&self) -> DIFSEL12_R {
        DIFSEL12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel13(&self) -> DIFSEL13_R {
        DIFSEL13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel14(&self) -> DIFSEL14_R {
        DIFSEL14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel15(&self) -> DIFSEL15_R {
        DIFSEL15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel16(&self) -> DIFSEL16_R {
        DIFSEL16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel17(&self) -> DIFSEL17_R {
        DIFSEL17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel18(&self) -> DIFSEL18_R {
        DIFSEL18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel19(&self) -> DIFSEL19_R {
        DIFSEL19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel0(&mut self) -> DIFSEL0_W {
        DIFSEL0_W { w: self }
    }
    #[doc = "Bit 1 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel1(&mut self) -> DIFSEL1_W {
        DIFSEL1_W { w: self }
    }
    #[doc = "Bit 2 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel2(&mut self) -> DIFSEL2_W {
        DIFSEL2_W { w: self }
    }
    #[doc = "Bit 3 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel3(&mut self) -> DIFSEL3_W {
        DIFSEL3_W { w: self }
    }
    #[doc = "Bit 4 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel4(&mut self) -> DIFSEL4_W {
        DIFSEL4_W { w: self }
    }
    #[doc = "Bit 5 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel5(&mut self) -> DIFSEL5_W {
        DIFSEL5_W { w: self }
    }
    #[doc = "Bit 6 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel6(&mut self) -> DIFSEL6_W {
        DIFSEL6_W { w: self }
    }
    #[doc = "Bit 7 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel7(&mut self) -> DIFSEL7_W {
        DIFSEL7_W { w: self }
    }
    #[doc = "Bit 8 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel8(&mut self) -> DIFSEL8_W {
        DIFSEL8_W { w: self }
    }
    #[doc = "Bit 9 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel9(&mut self) -> DIFSEL9_W {
        DIFSEL9_W { w: self }
    }
    #[doc = "Bit 10 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel10(&mut self) -> DIFSEL10_W {
        DIFSEL10_W { w: self }
    }
    #[doc = "Bit 11 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel11(&mut self) -> DIFSEL11_W {
        DIFSEL11_W { w: self }
    }
    #[doc = "Bit 12 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel12(&mut self) -> DIFSEL12_W {
        DIFSEL12_W { w: self }
    }
    #[doc = "Bit 13 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel13(&mut self) -> DIFSEL13_W {
        DIFSEL13_W { w: self }
    }
    #[doc = "Bit 14 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel14(&mut self) -> DIFSEL14_W {
        DIFSEL14_W { w: self }
    }
    #[doc = "Bit 15 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel15(&mut self) -> DIFSEL15_W {
        DIFSEL15_W { w: self }
    }
    #[doc = "Bit 16 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel16(&mut self) -> DIFSEL16_W {
        DIFSEL16_W { w: self }
    }
    #[doc = "Bit 17 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel17(&mut self) -> DIFSEL17_W {
        DIFSEL17_W { w: self }
    }
    #[doc = "Bit 18 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel18(&mut self) -> DIFSEL18_W {
        DIFSEL18_W { w: self }
    }
    #[doc = "Bit 19 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel19(&mut self) -> DIFSEL19_W {
        DIFSEL19_W { w: self }
    }
}
