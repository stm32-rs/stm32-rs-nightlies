#[doc = "Reader of register CR2"]
pub type R = crate::R<u32, super::CR2>;
#[doc = "Writer for register CR2"]
pub type W = crate::W<u32, super::CR2>;
#[doc = "Register CR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Master mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MMS_A {
    #[doc = "0: Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO)"]
    RESET = 0,
    #[doc = "1: Enable - the Counter enable signal, CNT_EN, is used as trigger output (TRGO)"]
    ENABLE = 1,
    #[doc = "2: Update - The update event is selected as trigger output (TRGO)"]
    UPDATE = 2,
    #[doc = "3: Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred"]
    COMPAREPULSE = 3,
    #[doc = "4: OC1REF signal is used as trigger output (TRGO)"]
    OC1REF = 4,
    #[doc = "5: OC2REF signal is used as trigger output (TRGO)"]
    OC2REF = 5,
}
impl From<MMS_A> for u8 {
    #[inline(always)]
    fn from(variant: MMS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MMS`"]
pub type MMS_R = crate::R<u8, MMS_A>;
impl MMS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MMS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MMS_A::RESET),
            1 => Val(MMS_A::ENABLE),
            2 => Val(MMS_A::UPDATE),
            3 => Val(MMS_A::COMPAREPULSE),
            4 => Val(MMS_A::OC1REF),
            5 => Val(MMS_A::OC2REF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == MMS_A::RESET
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MMS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == MMS_A::UPDATE
    }
    #[doc = "Checks if the value of the field is `COMPAREPULSE`"]
    #[inline(always)]
    pub fn is_compare_pulse(&self) -> bool {
        *self == MMS_A::COMPAREPULSE
    }
    #[doc = "Checks if the value of the field is `OC1REF`"]
    #[inline(always)]
    pub fn is_oc1ref(&self) -> bool {
        *self == MMS_A::OC1REF
    }
    #[doc = "Checks if the value of the field is `OC2REF`"]
    #[inline(always)]
    pub fn is_oc2ref(&self) -> bool {
        *self == MMS_A::OC2REF
    }
}
#[doc = "Write proxy for field `MMS`"]
pub struct MMS_W<'a> {
    w: &'a mut W,
}
impl<'a> MMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MMS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(MMS_A::RESET)
    }
    #[doc = "Enable - the Counter enable signal, CNT_EN, is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MMS_A::ENABLE)
    }
    #[doc = "Update - The update event is selected as trigger output (TRGO)"]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(MMS_A::UPDATE)
    }
    #[doc = "Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred"]
    #[inline(always)]
    pub fn compare_pulse(self) -> &'a mut W {
        self.variant(MMS_A::COMPAREPULSE)
    }
    #[doc = "OC1REF signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn oc1ref(self) -> &'a mut W {
        self.variant(MMS_A::OC1REF)
    }
    #[doc = "OC2REF signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn oc2ref(self) -> &'a mut W {
        self.variant(MMS_A::OC2REF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W {
        MMS_W { w: self }
    }
}
