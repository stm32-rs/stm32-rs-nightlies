#[doc = "Reader of register C1_APB1HLPENR"]
pub type R = crate::R<u32, super::C1_APB1HLPENR>;
#[doc = "Writer for register C1_APB1HLPENR"]
pub type W = crate::W<u32, super::C1_APB1HLPENR>;
#[doc = "Register C1_APB1HLPENR `reset()`'s with value 0"]
impl crate::ResetValue for super::C1_APB1HLPENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock Recovery System peripheral clock enable during CSleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRSLPEN_A {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    ENABLED = 1,
}
impl From<CRSLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRSLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRSLPEN`"]
pub type CRSLPEN_R = crate::R<bool, CRSLPEN_A>;
impl CRSLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRSLPEN_A {
        match self.bits {
            false => CRSLPEN_A::DISABLED,
            true => CRSLPEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRSLPEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRSLPEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `CRSLPEN`"]
pub struct CRSLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRSLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRSLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRSLPEN_A::ENABLED)
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
#[doc = "SWPMI Peripheral Clocks Enable During CSleep Mode"]
pub type SWPLPEN_A = CRSLPEN_A;
#[doc = "Reader of field `SWPLPEN`"]
pub type SWPLPEN_R = crate::R<bool, CRSLPEN_A>;
#[doc = "Write proxy for field `SWPLPEN`"]
pub struct SWPLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWPLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRSLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRSLPEN_A::ENABLED)
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
#[doc = "OPAMP peripheral clock enable during CSleep mode"]
pub type OPAMPLPEN_A = CRSLPEN_A;
#[doc = "Reader of field `OPAMPLPEN`"]
pub type OPAMPLPEN_R = crate::R<bool, CRSLPEN_A>;
#[doc = "Write proxy for field `OPAMPLPEN`"]
pub struct OPAMPLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAMPLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAMPLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRSLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRSLPEN_A::ENABLED)
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
#[doc = "MDIOS peripheral clock enable during CSleep mode"]
pub type MDIOSLPEN_A = CRSLPEN_A;
#[doc = "Reader of field `MDIOSLPEN`"]
pub type MDIOSLPEN_R = crate::R<bool, CRSLPEN_A>;
#[doc = "Write proxy for field `MDIOSLPEN`"]
pub struct MDIOSLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIOSLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDIOSLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRSLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRSLPEN_A::ENABLED)
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
#[doc = "FDCAN Peripheral Clocks Enable During CSleep Mode"]
pub type FDCANLPEN_A = CRSLPEN_A;
#[doc = "Reader of field `FDCANLPEN`"]
pub type FDCANLPEN_R = crate::R<bool, CRSLPEN_A>;
#[doc = "Write proxy for field `FDCANLPEN`"]
pub struct FDCANLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCANLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDCANLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRSLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRSLPEN_A::ENABLED)
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
    #[doc = "Bit 1 - Clock Recovery System peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn crslpen(&self) -> CRSLPEN_R {
        CRSLPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SWPMI Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn swplpen(&self) -> SWPLPEN_R {
        SWPLPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OPAMP peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn opamplpen(&self) -> OPAMPLPEN_R {
        OPAMPLPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MDIOS peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn mdioslpen(&self) -> MDIOSLPEN_R {
        MDIOSLPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FDCAN Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn fdcanlpen(&self) -> FDCANLPEN_R {
        FDCANLPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Clock Recovery System peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn crslpen(&mut self) -> CRSLPEN_W {
        CRSLPEN_W { w: self }
    }
    #[doc = "Bit 2 - SWPMI Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn swplpen(&mut self) -> SWPLPEN_W {
        SWPLPEN_W { w: self }
    }
    #[doc = "Bit 4 - OPAMP peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn opamplpen(&mut self) -> OPAMPLPEN_W {
        OPAMPLPEN_W { w: self }
    }
    #[doc = "Bit 5 - MDIOS peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn mdioslpen(&mut self) -> MDIOSLPEN_W {
        MDIOSLPEN_W { w: self }
    }
    #[doc = "Bit 8 - FDCAN Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn fdcanlpen(&mut self) -> FDCANLPEN_W {
        FDCANLPEN_W { w: self }
    }
}
