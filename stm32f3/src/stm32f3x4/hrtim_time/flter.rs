#[doc = "Reader of register FLTER"]
pub type R = crate::R<u32, super::FLTER>;
#[doc = "Writer for register FLTER"]
pub type W = crate::W<u32, super::FLTER>;
#[doc = "Register FLTER `reset()`'s with value 0"]
impl crate::ResetValue for super::FLTER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Fault sources Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLTLCK_A {
    #[doc = "0: FLT1EN..FLT5EN bits are read/write"]
    UNLOCKED = 0,
    #[doc = "1: FLT1EN..FLT5EN bits are read only"]
    LOCKED = 1,
}
impl From<FLTLCK_A> for bool {
    #[inline(always)]
    fn from(variant: FLTLCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLTLCK`"]
pub type FLTLCK_R = crate::R<bool, FLTLCK_A>;
impl FLTLCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTLCK_A {
        match self.bits {
            false => FLTLCK_A::UNLOCKED,
            true => FLTLCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == FLTLCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == FLTLCK_A::LOCKED
    }
}
#[doc = "Write proxy for field `FLTLCK`"]
pub struct FLTLCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTLCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTLCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FLT1EN..FLT5EN bits are read/write"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(FLTLCK_A::UNLOCKED)
    }
    #[doc = "FLT1EN..FLT5EN bits are read only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(FLTLCK_A::LOCKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Fault 5 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT5EN_A {
    #[doc = "0: Fault input ignored"]
    IGNORED = 0,
    #[doc = "1: Fault input is active and can disable HRTIM outputs"]
    ACTIVE = 1,
}
impl From<FLT5EN_A> for bool {
    #[inline(always)]
    fn from(variant: FLT5EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLT5EN`"]
pub type FLT5EN_R = crate::R<bool, FLT5EN_A>;
impl FLT5EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT5EN_A {
        match self.bits {
            false => FLT5EN_A::IGNORED,
            true => FLT5EN_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `IGNORED`"]
    #[inline(always)]
    pub fn is_ignored(&self) -> bool {
        *self == FLT5EN_A::IGNORED
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == FLT5EN_A::ACTIVE
    }
}
#[doc = "Write proxy for field `FLT5EN`"]
pub struct FLT5EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT5EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fault input ignored"]
    #[inline(always)]
    pub fn ignored(self) -> &'a mut W {
        self.variant(FLT5EN_A::IGNORED)
    }
    #[doc = "Fault input is active and can disable HRTIM outputs"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(FLT5EN_A::ACTIVE)
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
#[doc = "Fault 4 enable"]
pub type FLT4EN_A = FLT5EN_A;
#[doc = "Reader of field `FLT4EN`"]
pub type FLT4EN_R = crate::R<bool, FLT5EN_A>;
#[doc = "Write proxy for field `FLT4EN`"]
pub struct FLT4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT4EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT4EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fault input ignored"]
    #[inline(always)]
    pub fn ignored(self) -> &'a mut W {
        self.variant(FLT5EN_A::IGNORED)
    }
    #[doc = "Fault input is active and can disable HRTIM outputs"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(FLT5EN_A::ACTIVE)
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
#[doc = "Fault 3 enable"]
pub type FLT3EN_A = FLT5EN_A;
#[doc = "Reader of field `FLT3EN`"]
pub type FLT3EN_R = crate::R<bool, FLT5EN_A>;
#[doc = "Write proxy for field `FLT3EN`"]
pub struct FLT3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT3EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fault input ignored"]
    #[inline(always)]
    pub fn ignored(self) -> &'a mut W {
        self.variant(FLT5EN_A::IGNORED)
    }
    #[doc = "Fault input is active and can disable HRTIM outputs"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(FLT5EN_A::ACTIVE)
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
#[doc = "Fault 2 enable"]
pub type FLT2EN_A = FLT5EN_A;
#[doc = "Reader of field `FLT2EN`"]
pub type FLT2EN_R = crate::R<bool, FLT5EN_A>;
#[doc = "Write proxy for field `FLT2EN`"]
pub struct FLT2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT2EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fault input ignored"]
    #[inline(always)]
    pub fn ignored(self) -> &'a mut W {
        self.variant(FLT5EN_A::IGNORED)
    }
    #[doc = "Fault input is active and can disable HRTIM outputs"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(FLT5EN_A::ACTIVE)
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
#[doc = "Fault 1 enable"]
pub type FLT1EN_A = FLT5EN_A;
#[doc = "Reader of field `FLT1EN`"]
pub type FLT1EN_R = crate::R<bool, FLT5EN_A>;
#[doc = "Write proxy for field `FLT1EN`"]
pub struct FLT1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fault input ignored"]
    #[inline(always)]
    pub fn ignored(self) -> &'a mut W {
        self.variant(FLT5EN_A::IGNORED)
    }
    #[doc = "Fault input is active and can disable HRTIM outputs"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(FLT5EN_A::ACTIVE)
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
    #[doc = "Bit 31 - Fault sources Lock"]
    #[inline(always)]
    pub fn fltlck(&self) -> FLTLCK_R {
        FLTLCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fault 5 enable"]
    #[inline(always)]
    pub fn flt5en(&self) -> FLT5EN_R {
        FLT5EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fault 4 enable"]
    #[inline(always)]
    pub fn flt4en(&self) -> FLT4EN_R {
        FLT4EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fault 3 enable"]
    #[inline(always)]
    pub fn flt3en(&self) -> FLT3EN_R {
        FLT3EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fault 2 enable"]
    #[inline(always)]
    pub fn flt2en(&self) -> FLT2EN_R {
        FLT2EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Fault 1 enable"]
    #[inline(always)]
    pub fn flt1en(&self) -> FLT1EN_R {
        FLT1EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Fault sources Lock"]
    #[inline(always)]
    pub fn fltlck(&mut self) -> FLTLCK_W {
        FLTLCK_W { w: self }
    }
    #[doc = "Bit 4 - Fault 5 enable"]
    #[inline(always)]
    pub fn flt5en(&mut self) -> FLT5EN_W {
        FLT5EN_W { w: self }
    }
    #[doc = "Bit 3 - Fault 4 enable"]
    #[inline(always)]
    pub fn flt4en(&mut self) -> FLT4EN_W {
        FLT4EN_W { w: self }
    }
    #[doc = "Bit 2 - Fault 3 enable"]
    #[inline(always)]
    pub fn flt3en(&mut self) -> FLT3EN_W {
        FLT3EN_W { w: self }
    }
    #[doc = "Bit 1 - Fault 2 enable"]
    #[inline(always)]
    pub fn flt2en(&mut self) -> FLT2EN_W {
        FLT2EN_W { w: self }
    }
    #[doc = "Bit 0 - Fault 1 enable"]
    #[inline(always)]
    pub fn flt1en(&mut self) -> FLT1EN_W {
        FLT1EN_W { w: self }
    }
}
