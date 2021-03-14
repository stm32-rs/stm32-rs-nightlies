#[doc = "Reader of register PWRCR"]
pub type R = crate::R<u32, super::PWRCR>;
#[doc = "Writer for register PWRCR"]
pub type W = crate::W<u32, super::PWRCR>;
#[doc = "Register PWRCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PWRCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Overdrive enable, this bit allows to activate the LDO regulator overdrive mode. This bit must be written only in VOS1 voltage scaling mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODEN_A {
    #[doc = "0: Overdrive mode disabled"]
    DISABLED = 0,
    #[doc = "1: Overdrive mode enabled (the LDO generates VOS0 for VCORE)"]
    ENABLED = 1,
}
impl From<ODEN_A> for bool {
    #[inline(always)]
    fn from(variant: ODEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ODEN`"]
pub type ODEN_R = crate::R<bool, ODEN_A>;
impl ODEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ODEN_A {
        match self.bits {
            false => ODEN_A::DISABLED,
            true => ODEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ODEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ODEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `ODEN`"]
pub struct ODEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ODEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Overdrive mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ODEN_A::DISABLED)
    }
    #[doc = "Overdrive mode enabled (the LDO generates VOS0 for VCORE)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ODEN_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - Overdrive enable, this bit allows to activate the LDO regulator overdrive mode. This bit must be written only in VOS1 voltage scaling mode"]
    #[inline(always)]
    pub fn oden(&self) -> ODEN_R {
        ODEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overdrive enable, this bit allows to activate the LDO regulator overdrive mode. This bit must be written only in VOS1 voltage scaling mode"]
    #[inline(always)]
    pub fn oden(&mut self) -> ODEN_W {
        ODEN_W { w: self }
    }
}
