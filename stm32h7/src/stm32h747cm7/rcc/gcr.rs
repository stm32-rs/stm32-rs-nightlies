#[doc = "Reader of register GCR"]
pub type R = crate::R<u32, super::GCR>;
#[doc = "Writer for register GCR"]
pub type W = crate::W<u32, super::GCR>;
#[doc = "Register GCR `reset()`'s with value 0"]
impl crate::ResetValue for super::GCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "WWDG1 reset scope control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WW1RSC_A {
    #[doc = "0: Clear WWDG1 scope control"]
    CLEAR = 0,
    #[doc = "1: Set WWDG1 scope control"]
    SET = 1,
}
impl From<WW1RSC_A> for bool {
    #[inline(always)]
    fn from(variant: WW1RSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WW1RSC`"]
pub type WW1RSC_R = crate::R<bool, WW1RSC_A>;
impl WW1RSC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WW1RSC_A {
        match self.bits {
            false => WW1RSC_A::CLEAR,
            true => WW1RSC_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WW1RSC_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == WW1RSC_A::SET
    }
}
#[doc = "Write proxy for field `WW1RSC`"]
pub struct WW1RSC_W<'a> {
    w: &'a mut W,
}
impl<'a> WW1RSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WW1RSC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear WWDG1 scope control"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WW1RSC_A::CLEAR)
    }
    #[doc = "Set WWDG1 scope control"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(WW1RSC_A::SET)
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
#[doc = "Reader of field `WW2RSC`"]
pub type WW2RSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WW2RSC`"]
pub struct WW2RSC_W<'a> {
    w: &'a mut W,
}
impl<'a> WW2RSC_W<'a> {
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
#[doc = "Reader of field `BOOT_C1`"]
pub type BOOT_C1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT_C1`"]
pub struct BOOT_C1_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_C1_W<'a> {
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
#[doc = "Reader of field `BOOT_C2`"]
pub type BOOT_C2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT_C2`"]
pub struct BOOT_C2_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_C2_W<'a> {
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
impl R {
    #[doc = "Bit 0 - WWDG1 reset scope control"]
    #[inline(always)]
    pub fn ww1rsc(&self) -> WW1RSC_R {
        WW1RSC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WWDG2 reset scope control"]
    #[inline(always)]
    pub fn ww2rsc(&self) -> WW2RSC_R {
        WW2RSC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Force allow CPU1 to boot"]
    #[inline(always)]
    pub fn boot_c1(&self) -> BOOT_C1_R {
        BOOT_C1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Force allow CPU2 to boot"]
    #[inline(always)]
    pub fn boot_c2(&self) -> BOOT_C2_R {
        BOOT_C2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WWDG1 reset scope control"]
    #[inline(always)]
    pub fn ww1rsc(&mut self) -> WW1RSC_W {
        WW1RSC_W { w: self }
    }
    #[doc = "Bit 1 - WWDG2 reset scope control"]
    #[inline(always)]
    pub fn ww2rsc(&mut self) -> WW2RSC_W {
        WW2RSC_W { w: self }
    }
    #[doc = "Bit 2 - Force allow CPU1 to boot"]
    #[inline(always)]
    pub fn boot_c1(&mut self) -> BOOT_C1_W {
        BOOT_C1_W { w: self }
    }
    #[doc = "Bit 3 - Force allow CPU2 to boot"]
    #[inline(always)]
    pub fn boot_c2(&mut self) -> BOOT_C2_W {
        BOOT_C2_W { w: self }
    }
}
