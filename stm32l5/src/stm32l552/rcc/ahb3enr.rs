#[doc = "Reader of register AHB3ENR"]
pub type R = crate::R<u32, super::AHB3ENR>;
#[doc = "Writer for register AHB3ENR"]
pub type W = crate::W<u32, super::AHB3ENR>;
#[doc = "Register AHB3ENR `reset()`'s with value 0"]
impl crate::ResetValue for super::AHB3ENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Flexible memory controller clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMCEN_A {
    #[doc = "0: The selected clock is disabled"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled"]
    ENABLED = 1,
}
impl From<FMCEN_A> for bool {
    #[inline(always)]
    fn from(variant: FMCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FMCEN`"]
pub type FMCEN_R = crate::R<bool, FMCEN_A>;
impl FMCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMCEN_A {
        match self.bits {
            false => FMCEN_A::DISABLED,
            true => FMCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FMCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FMCEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `FMCEN`"]
pub struct FMCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FMCEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FMCEN_A::ENABLED)
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
#[doc = "OSPI1EN"]
pub type OSPI1EN_A = FMCEN_A;
#[doc = "Reader of field `OSPI1EN`"]
pub type OSPI1EN_R = crate::R<bool, FMCEN_A>;
#[doc = "Write proxy for field `OSPI1EN`"]
pub struct OSPI1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPI1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPI1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FMCEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FMCEN_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - Flexible memory controller clock enable"]
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - OSPI1EN"]
    #[inline(always)]
    pub fn ospi1en(&self) -> OSPI1EN_R {
        OSPI1EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible memory controller clock enable"]
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W {
        FMCEN_W { w: self }
    }
    #[doc = "Bit 8 - OSPI1EN"]
    #[inline(always)]
    pub fn ospi1en(&mut self) -> OSPI1EN_W {
        OSPI1EN_W { w: self }
    }
}
