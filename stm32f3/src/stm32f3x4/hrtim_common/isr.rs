#[doc = "Reader of register ISR"]
pub type R = crate::R<u32, super::ISR>;
#[doc = "Writer for register ISR"]
pub type W = crate::W<u32, super::ISR>;
#[doc = "Register ISR `reset()`'s with value 0"]
impl crate::ResetValue for super::ISR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Burst mode Period Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BMPER_A {
    #[doc = "0: No burst mode period interrupt occurred"]
    NOEVENT = 0,
    #[doc = "1: Burst mode period interrupt occured"]
    EVENT = 1,
}
impl From<BMPER_A> for bool {
    #[inline(always)]
    fn from(variant: BMPER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BMPER`"]
pub type BMPER_R = crate::R<bool, BMPER_A>;
impl BMPER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BMPER_A {
        match self.bits {
            false => BMPER_A::NOEVENT,
            true => BMPER_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == BMPER_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == BMPER_A::EVENT
    }
}
#[doc = "DLL Ready Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLLRDY_A {
    #[doc = "0: No DLL calibration ready interrupt occurred"]
    NOEVENT = 0,
    #[doc = "1: DLL calibration ready interrupt occurred"]
    EVENT = 1,
}
impl From<DLLRDY_A> for bool {
    #[inline(always)]
    fn from(variant: DLLRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DLLRDY`"]
pub type DLLRDY_R = crate::R<bool, DLLRDY_A>;
impl DLLRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLLRDY_A {
        match self.bits {
            false => DLLRDY_A::NOEVENT,
            true => DLLRDY_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == DLLRDY_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == DLLRDY_A::EVENT
    }
}
#[doc = "System Fault Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSFLT_A {
    #[doc = "0: No fault interrupt occurred"]
    NOEVENT = 0,
    #[doc = "1: Fault interrupt occurred"]
    EVENT = 1,
}
impl From<SYSFLT_A> for bool {
    #[inline(always)]
    fn from(variant: SYSFLT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSFLT`"]
pub type SYSFLT_R = crate::R<bool, SYSFLT_A>;
impl SYSFLT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSFLT_A {
        match self.bits {
            false => SYSFLT_A::NOEVENT,
            true => SYSFLT_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == SYSFLT_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == SYSFLT_A::EVENT
    }
}
#[doc = "System Fault Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSFLT_AW {
    #[doc = "1: Clear fault interrupt"]
    CLEAR = 1,
}
impl From<SYSFLT_AW> for bool {
    #[inline(always)]
    fn from(variant: SYSFLT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SYSFLT`"]
pub struct SYSFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSFLT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSFLT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear fault interrupt"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SYSFLT_AW::CLEAR)
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
#[doc = "Fault 5 Interrupt Flag"]
pub type FLT5_A = SYSFLT_A;
#[doc = "Reader of field `FLT5`"]
pub type FLT5_R = crate::R<bool, SYSFLT_A>;
#[doc = "Fault 4 Interrupt Flag"]
pub type FLT4_A = SYSFLT_A;
#[doc = "Reader of field `FLT4`"]
pub type FLT4_R = crate::R<bool, SYSFLT_A>;
#[doc = "Fault 3 Interrupt Flag"]
pub type FLT3_A = SYSFLT_A;
#[doc = "Reader of field `FLT3`"]
pub type FLT3_R = crate::R<bool, SYSFLT_A>;
#[doc = "Fault 2 Interrupt Flag"]
pub type FLT2_A = SYSFLT_A;
#[doc = "Reader of field `FLT2`"]
pub type FLT2_R = crate::R<bool, SYSFLT_A>;
#[doc = "Fault 1 Interrupt Flag"]
pub type FLT1_A = SYSFLT_A;
#[doc = "Reader of field `FLT1`"]
pub type FLT1_R = crate::R<bool, SYSFLT_A>;
impl R {
    #[doc = "Bit 17 - Burst mode Period Interrupt Flag"]
    #[inline(always)]
    pub fn bmper(&self) -> BMPER_R {
        BMPER_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DLL Ready Interrupt Flag"]
    #[inline(always)]
    pub fn dllrdy(&self) -> DLLRDY_R {
        DLLRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 5 - System Fault Interrupt Flag"]
    #[inline(always)]
    pub fn sysflt(&self) -> SYSFLT_R {
        SYSFLT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Flag"]
    #[inline(always)]
    pub fn flt5(&self) -> FLT5_R {
        FLT5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Flag"]
    #[inline(always)]
    pub fn flt4(&self) -> FLT4_R {
        FLT4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Flag"]
    #[inline(always)]
    pub fn flt3(&self) -> FLT3_R {
        FLT3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Flag"]
    #[inline(always)]
    pub fn flt2(&self) -> FLT2_R {
        FLT2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Fault 1 Interrupt Flag"]
    #[inline(always)]
    pub fn flt1(&self) -> FLT1_R {
        FLT1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - System Fault Interrupt Flag"]
    #[inline(always)]
    pub fn sysflt(&mut self) -> SYSFLT_W {
        SYSFLT_W { w: self }
    }
}
