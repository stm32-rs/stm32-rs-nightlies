#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Volatile data execution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDE_A {
    #[doc = "0: Volatile data segment cannot be executed if VDS = 0"]
    NOTEXECUTABLE = 0,
    #[doc = "1: Volatile data segment is declared executable whatever VDS bit value"]
    EXECUTABLE = 1,
}
impl From<VDE_A> for bool {
    #[inline(always)]
    fn from(variant: VDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VDE`"]
pub type VDE_R = crate::R<bool, VDE_A>;
impl VDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDE_A {
        match self.bits {
            false => VDE_A::NOTEXECUTABLE,
            true => VDE_A::EXECUTABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTEXECUTABLE`"]
    #[inline(always)]
    pub fn is_not_executable(&self) -> bool {
        *self == VDE_A::NOTEXECUTABLE
    }
    #[doc = "Checks if the value of the field is `EXECUTABLE`"]
    #[inline(always)]
    pub fn is_executable(&self) -> bool {
        *self == VDE_A::EXECUTABLE
    }
}
#[doc = "Volatile data execution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDE_AW {
    #[doc = "0: Resets volatile data execution bit"]
    RESET = 0,
}
impl From<VDE_AW> for bool {
    #[inline(always)]
    fn from(variant: VDE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `VDE`"]
pub struct VDE_W<'a> {
    w: &'a mut W,
}
impl<'a> VDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Resets volatile data execution bit"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(VDE_AW::RESET)
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
#[doc = "Volatile data shared\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDS_A {
    #[doc = "0: Volatile data segment is not shared and cannot be hit by a non protected executable code when the Firewall is closed"]
    NOTSHARED = 0,
    #[doc = "1: Volatile data segment is shared with non protected application code"]
    SHARED = 1,
}
impl From<VDS_A> for bool {
    #[inline(always)]
    fn from(variant: VDS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VDS`"]
pub type VDS_R = crate::R<bool, VDS_A>;
impl VDS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDS_A {
        match self.bits {
            false => VDS_A::NOTSHARED,
            true => VDS_A::SHARED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSHARED`"]
    #[inline(always)]
    pub fn is_not_shared(&self) -> bool {
        *self == VDS_A::NOTSHARED
    }
    #[doc = "Checks if the value of the field is `SHARED`"]
    #[inline(always)]
    pub fn is_shared(&self) -> bool {
        *self == VDS_A::SHARED
    }
}
#[doc = "Volatile data shared\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDS_AW {
    #[doc = "0: Resets volatile data shared bit"]
    RESET = 0,
}
impl From<VDS_AW> for bool {
    #[inline(always)]
    fn from(variant: VDS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `VDS`"]
pub struct VDS_W<'a> {
    w: &'a mut W,
}
impl<'a> VDS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Resets volatile data shared bit"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(VDS_AW::RESET)
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
#[doc = "Firewall pre alarm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPA_A {
    #[doc = "0: Any code executed outside the protected segment when the Firewall is opened will generate a system reset"]
    PREARMRESET = 0,
    #[doc = "1: Any code executed outside the protected segment will close the Firewall"]
    PREARMSET = 1,
}
impl From<FPA_A> for bool {
    #[inline(always)]
    fn from(variant: FPA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FPA`"]
pub type FPA_R = crate::R<bool, FPA_A>;
impl FPA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPA_A {
        match self.bits {
            false => FPA_A::PREARMRESET,
            true => FPA_A::PREARMSET,
        }
    }
    #[doc = "Checks if the value of the field is `PREARMRESET`"]
    #[inline(always)]
    pub fn is_pre_arm_reset(&self) -> bool {
        *self == FPA_A::PREARMRESET
    }
    #[doc = "Checks if the value of the field is `PREARMSET`"]
    #[inline(always)]
    pub fn is_pre_arm_set(&self) -> bool {
        *self == FPA_A::PREARMSET
    }
}
#[doc = "Write proxy for field `FPA`"]
pub struct FPA_W<'a> {
    w: &'a mut W,
}
impl<'a> FPA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Any code executed outside the protected segment when the Firewall is opened will generate a system reset"]
    #[inline(always)]
    pub fn pre_arm_reset(self) -> &'a mut W {
        self.variant(FPA_A::PREARMRESET)
    }
    #[doc = "Any code executed outside the protected segment will close the Firewall"]
    #[inline(always)]
    pub fn pre_arm_set(self) -> &'a mut W {
        self.variant(FPA_A::PREARMSET)
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
    #[doc = "Bit 2 - Volatile data execution"]
    #[inline(always)]
    pub fn vde(&self) -> VDE_R {
        VDE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Volatile data shared"]
    #[inline(always)]
    pub fn vds(&self) -> VDS_R {
        VDS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Firewall pre alarm"]
    #[inline(always)]
    pub fn fpa(&self) -> FPA_R {
        FPA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Volatile data execution"]
    #[inline(always)]
    pub fn vde(&mut self) -> VDE_W {
        VDE_W { w: self }
    }
    #[doc = "Bit 1 - Volatile data shared"]
    #[inline(always)]
    pub fn vds(&mut self) -> VDS_W {
        VDS_W { w: self }
    }
    #[doc = "Bit 0 - Firewall pre alarm"]
    #[inline(always)]
    pub fn fpa(&mut self) -> FPA_W {
        FPA_W { w: self }
    }
}
