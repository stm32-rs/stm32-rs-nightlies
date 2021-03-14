#[doc = "Reader of register RTCCR"]
pub type R = crate::R<u32, super::RTCCR>;
#[doc = "Writer for register RTCCR"]
pub type W = crate::W<u32, super::RTCCR>;
#[doc = "Register RTCCR `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAL`"]
pub type CAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAL`"]
pub struct CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `CCO`"]
pub type CCO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCO`"]
pub struct CCO_W<'a> {
    w: &'a mut W,
}
impl<'a> CCO_W<'a> {
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
#[doc = "Alarm or second output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASOE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Setting this bit outputs either the RTC Alarm pulse signal or the Second pulse signal on the TAMPER pin depending on the ASOS bit"]
    ENABLED = 1,
}
impl From<ASOE_A> for bool {
    #[inline(always)]
    fn from(variant: ASOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ASOE`"]
pub type ASOE_R = crate::R<bool, ASOE_A>;
impl ASOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASOE_A {
        match self.bits {
            false => ASOE_A::DISABLED,
            true => ASOE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ASOE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ASOE_A::ENABLED
    }
}
#[doc = "Write proxy for field `ASOE`"]
pub struct ASOE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ASOE_A::DISABLED)
    }
    #[doc = "Setting this bit outputs either the RTC Alarm pulse signal or the Second pulse signal on the TAMPER pin depending on the ASOS bit"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ASOE_A::ENABLED)
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
#[doc = "Alarm or second output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASOS_A {
    #[doc = "0: RTC Alarm pulse output selected"]
    ALARM = 0,
    #[doc = "1: RTC Second pulse output selected"]
    SECOND = 1,
}
impl From<ASOS_A> for bool {
    #[inline(always)]
    fn from(variant: ASOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ASOS`"]
pub type ASOS_R = crate::R<bool, ASOS_A>;
impl ASOS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASOS_A {
        match self.bits {
            false => ASOS_A::ALARM,
            true => ASOS_A::SECOND,
        }
    }
    #[doc = "Checks if the value of the field is `ALARM`"]
    #[inline(always)]
    pub fn is_alarm(&self) -> bool {
        *self == ASOS_A::ALARM
    }
    #[doc = "Checks if the value of the field is `SECOND`"]
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == ASOS_A::SECOND
    }
}
#[doc = "Write proxy for field `ASOS`"]
pub struct ASOS_W<'a> {
    w: &'a mut W,
}
impl<'a> ASOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASOS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RTC Alarm pulse output selected"]
    #[inline(always)]
    pub fn alarm(self) -> &'a mut W {
        self.variant(ASOS_A::ALARM)
    }
    #[doc = "RTC Second pulse output selected"]
    #[inline(always)]
    pub fn second(self) -> &'a mut W {
        self.variant(ASOS_A::SECOND)
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
impl R {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Calibration Clock Output"]
    #[inline(always)]
    pub fn cco(&self) -> CCO_R {
        CCO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Alarm or second output enable"]
    #[inline(always)]
    pub fn asoe(&self) -> ASOE_R {
        ASOE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Alarm or second output selection"]
    #[inline(always)]
    pub fn asos(&self) -> ASOS_R {
        ASOS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W {
        CAL_W { w: self }
    }
    #[doc = "Bit 7 - Calibration Clock Output"]
    #[inline(always)]
    pub fn cco(&mut self) -> CCO_W {
        CCO_W { w: self }
    }
    #[doc = "Bit 8 - Alarm or second output enable"]
    #[inline(always)]
    pub fn asoe(&mut self) -> ASOE_W {
        ASOE_W { w: self }
    }
    #[doc = "Bit 9 - Alarm or second output selection"]
    #[inline(always)]
    pub fn asos(&mut self) -> ASOS_W {
        ASOS_W { w: self }
    }
}
