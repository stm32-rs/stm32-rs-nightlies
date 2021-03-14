#[doc = "Reader of register CSR"]
pub type R = crate::R<u32, super::CSR>;
#[doc = "Writer for register CSR"]
pub type W = crate::W<u32, super::CSR>;
#[doc = "Register CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clear Tamper event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTE_AW {
    #[doc = "1: Reset the TEF Tamper event flag (and the Tamper detector)"]
    RESET = 1,
}
impl From<CTE_AW> for bool {
    #[inline(always)]
    fn from(variant: CTE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CTE`"]
pub struct CTE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the TEF Tamper event flag (and the Tamper detector)"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CTE_AW::RESET)
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
#[doc = "Clear Tamper Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTI_AW {
    #[doc = "1: Clear the Tamper interrupt and the TIF Tamper interrupt flag"]
    CLEAR = 1,
}
impl From<CTI_AW> for bool {
    #[inline(always)]
    fn from(variant: CTI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CTI`"]
pub struct CTI_W<'a> {
    w: &'a mut W,
}
impl<'a> CTI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTI_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the Tamper interrupt and the TIF Tamper interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTI_AW::CLEAR)
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
#[doc = "Tamper Pin interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPIE_A {
    #[doc = "0: Tamper interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Tamper interrupt enabled (the TPE bit must also be set in the BKP_CR register"]
    ENABLED = 1,
}
impl From<TPIE_A> for bool {
    #[inline(always)]
    fn from(variant: TPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TPIE`"]
pub type TPIE_R = crate::R<bool, TPIE_A>;
impl TPIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPIE_A {
        match self.bits {
            false => TPIE_A::DISABLED,
            true => TPIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TPIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TPIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `TPIE`"]
pub struct TPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TPIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tamper interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TPIE_A::DISABLED)
    }
    #[doc = "Tamper interrupt enabled (the TPE bit must also be set in the BKP_CR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TPIE_A::ENABLED)
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
#[doc = "Reader of field `TEF`"]
pub type TEF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIF`"]
pub type TIF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - Tamper Pin interrupt enable"]
    #[inline(always)]
    pub fn tpie(&self) -> TPIE_R {
        TPIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Tamper Event Flag"]
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Tamper Interrupt Flag"]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Tamper event"]
    #[inline(always)]
    pub fn cte(&mut self) -> CTE_W {
        CTE_W { w: self }
    }
    #[doc = "Bit 1 - Clear Tamper Interrupt"]
    #[inline(always)]
    pub fn cti(&mut self) -> CTI_W {
        CTI_W { w: self }
    }
    #[doc = "Bit 2 - Tamper Pin interrupt enable"]
    #[inline(always)]
    pub fn tpie(&mut self) -> TPIE_W {
        TPIE_W { w: self }
    }
}
