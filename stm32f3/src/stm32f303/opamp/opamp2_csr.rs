#[doc = "Reader of register OPAMP2_CSR"]
pub type R = crate::R<u32, super::OPAMP2_CSR>;
#[doc = "Writer for register OPAMP2_CSR"]
pub type W = crate::W<u32, super::OPAMP2_CSR>;
#[doc = "Register OPAMP2_CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::OPAMP2_CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "OPAMP2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAMP2EN_A {
    #[doc = "0: OPAMP2 is disabled"]
    DISABLED = 0,
    #[doc = "1: OPAMP2 is enabled"]
    ENABLED = 1,
}
impl From<OPAMP2EN_A> for bool {
    #[inline(always)]
    fn from(variant: OPAMP2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OPAMP2EN`"]
pub type OPAMP2EN_R = crate::R<bool, OPAMP2EN_A>;
impl OPAMP2EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPAMP2EN_A {
        match self.bits {
            false => OPAMP2EN_A::DISABLED,
            true => OPAMP2EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OPAMP2EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OPAMP2EN_A::ENABLED
    }
}
#[doc = "Write proxy for field `OPAMP2EN`"]
pub struct OPAMP2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAMP2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAMP2EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "OPAMP2 is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OPAMP2EN_A::DISABLED)
    }
    #[doc = "OPAMP2 is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OPAMP2EN_A::ENABLED)
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
#[doc = "FORCE_VP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCE_VP_A {
    #[doc = "0: Normal operating mode"]
    NORMAL = 0,
    #[doc = "1: Calibration mode. Non-inverting input connected to calibration reference"]
    CALIBRATION = 1,
}
impl From<FORCE_VP_A> for bool {
    #[inline(always)]
    fn from(variant: FORCE_VP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FORCE_VP`"]
pub type FORCE_VP_R = crate::R<bool, FORCE_VP_A>;
impl FORCE_VP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCE_VP_A {
        match self.bits {
            false => FORCE_VP_A::NORMAL,
            true => FORCE_VP_A::CALIBRATION,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == FORCE_VP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `CALIBRATION`"]
    #[inline(always)]
    pub fn is_calibration(&self) -> bool {
        *self == FORCE_VP_A::CALIBRATION
    }
}
#[doc = "Write proxy for field `FORCE_VP`"]
pub struct FORCE_VP_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_VP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCE_VP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operating mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(FORCE_VP_A::NORMAL)
    }
    #[doc = "Calibration mode. Non-inverting input connected to calibration reference"]
    #[inline(always)]
    pub fn calibration(self) -> &'a mut W {
        self.variant(FORCE_VP_A::CALIBRATION)
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
#[doc = "OPAMP Non inverting input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VP_SEL_A {
    #[doc = "1: PB14 used as OPAMP2 non-inverting input"]
    PB14 = 1,
    #[doc = "2: PB0 used as OPAMP2 non-inverting input"]
    PB0 = 2,
    #[doc = "3: PA7 used as OPAMP2 non-inverting input"]
    PA7 = 3,
}
impl From<VP_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VP_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VP_SEL`"]
pub type VP_SEL_R = crate::R<u8, VP_SEL_A>;
impl VP_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VP_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(VP_SEL_A::PB14),
            2 => Val(VP_SEL_A::PB0),
            3 => Val(VP_SEL_A::PA7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PB14`"]
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        *self == VP_SEL_A::PB14
    }
    #[doc = "Checks if the value of the field is `PB0`"]
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == VP_SEL_A::PB0
    }
    #[doc = "Checks if the value of the field is `PA7`"]
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        *self == VP_SEL_A::PA7
    }
}
#[doc = "Write proxy for field `VP_SEL`"]
pub struct VP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VP_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VP_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PB14 used as OPAMP2 non-inverting input"]
    #[inline(always)]
    pub fn pb14(self) -> &'a mut W {
        self.variant(VP_SEL_A::PB14)
    }
    #[doc = "PB0 used as OPAMP2 non-inverting input"]
    #[inline(always)]
    pub fn pb0(self) -> &'a mut W {
        self.variant(VP_SEL_A::PB0)
    }
    #[doc = "PA7 used as OPAMP2 non-inverting input"]
    #[inline(always)]
    pub fn pa7(self) -> &'a mut W {
        self.variant(VP_SEL_A::PA7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "OPAMP inverting input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VM_SEL_A {
    #[doc = "0: PC5 (VM0) used as OPAMP2 inverting input"]
    PC5 = 0,
    #[doc = "1: PA5 (VM1) used as OPAMP2 inverting input"]
    PA5 = 1,
    #[doc = "2: Resistor feedback output (PGA mode)"]
    PGA = 2,
    #[doc = "3: Follower mode"]
    FOLLOWER = 3,
}
impl From<VM_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VM_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VM_SEL`"]
pub type VM_SEL_R = crate::R<u8, VM_SEL_A>;
impl VM_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VM_SEL_A {
        match self.bits {
            0 => VM_SEL_A::PC5,
            1 => VM_SEL_A::PA5,
            2 => VM_SEL_A::PGA,
            3 => VM_SEL_A::FOLLOWER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PC5`"]
    #[inline(always)]
    pub fn is_pc5(&self) -> bool {
        *self == VM_SEL_A::PC5
    }
    #[doc = "Checks if the value of the field is `PA5`"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == VM_SEL_A::PA5
    }
    #[doc = "Checks if the value of the field is `PGA`"]
    #[inline(always)]
    pub fn is_pga(&self) -> bool {
        *self == VM_SEL_A::PGA
    }
    #[doc = "Checks if the value of the field is `FOLLOWER`"]
    #[inline(always)]
    pub fn is_follower(&self) -> bool {
        *self == VM_SEL_A::FOLLOWER
    }
}
#[doc = "Write proxy for field `VM_SEL`"]
pub struct VM_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VM_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VM_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PC5 (VM0) used as OPAMP2 inverting input"]
    #[inline(always)]
    pub fn pc5(self) -> &'a mut W {
        self.variant(VM_SEL_A::PC5)
    }
    #[doc = "PA5 (VM1) used as OPAMP2 inverting input"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut W {
        self.variant(VM_SEL_A::PA5)
    }
    #[doc = "Resistor feedback output (PGA mode)"]
    #[inline(always)]
    pub fn pga(self) -> &'a mut W {
        self.variant(VM_SEL_A::PGA)
    }
    #[doc = "Follower mode"]
    #[inline(always)]
    pub fn follower(self) -> &'a mut W {
        self.variant(VM_SEL_A::FOLLOWER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Timer controlled Mux mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCM_EN_A {
    #[doc = "0: Timer controlled mux disabled"]
    DISABLED = 0,
    #[doc = "1: Timer controlled mux enabled"]
    ENABLED = 1,
}
impl From<TCM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TCM_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCM_EN`"]
pub type TCM_EN_R = crate::R<bool, TCM_EN_A>;
impl TCM_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCM_EN_A {
        match self.bits {
            false => TCM_EN_A::DISABLED,
            true => TCM_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCM_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCM_EN_A::ENABLED
    }
}
#[doc = "Write proxy for field `TCM_EN`"]
pub struct TCM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TCM_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCM_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer controlled mux disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCM_EN_A::DISABLED)
    }
    #[doc = "Timer controlled mux enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCM_EN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "OPAMP inverting input secondary selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VMS_SEL_A {
    #[doc = "0: PC5 (VM0) used as OPAMP2 inverting input when TCM_EN=1"]
    PC5 = 0,
    #[doc = "1: PA5 (VM1) used as OPAMP2 inverting input when TCM_EN=1"]
    PA5 = 1,
}
impl From<VMS_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: VMS_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VMS_SEL`"]
pub type VMS_SEL_R = crate::R<bool, VMS_SEL_A>;
impl VMS_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VMS_SEL_A {
        match self.bits {
            false => VMS_SEL_A::PC5,
            true => VMS_SEL_A::PA5,
        }
    }
    #[doc = "Checks if the value of the field is `PC5`"]
    #[inline(always)]
    pub fn is_pc5(&self) -> bool {
        *self == VMS_SEL_A::PC5
    }
    #[doc = "Checks if the value of the field is `PA5`"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == VMS_SEL_A::PA5
    }
}
#[doc = "Write proxy for field `VMS_SEL`"]
pub struct VMS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VMS_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VMS_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PC5 (VM0) used as OPAMP2 inverting input when TCM_EN=1"]
    #[inline(always)]
    pub fn pc5(self) -> &'a mut W {
        self.variant(VMS_SEL_A::PC5)
    }
    #[doc = "PA5 (VM1) used as OPAMP2 inverting input when TCM_EN=1"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut W {
        self.variant(VMS_SEL_A::PA5)
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
#[doc = "OPAMP Non inverting input secondary selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VPS_SEL_A {
    #[doc = "1: PB14 used as OPAMP2 non-inverting input when TCM_EN=1"]
    PB14 = 1,
    #[doc = "2: PB0 used as OPAMP2 non-inverting input when TCM_EN=1"]
    PB0 = 2,
    #[doc = "3: PA7 used as OPAMP2 non-inverting input when TCM_EN=1"]
    PA7 = 3,
}
impl From<VPS_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VPS_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VPS_SEL`"]
pub type VPS_SEL_R = crate::R<u8, VPS_SEL_A>;
impl VPS_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VPS_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(VPS_SEL_A::PB14),
            2 => Val(VPS_SEL_A::PB0),
            3 => Val(VPS_SEL_A::PA7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PB14`"]
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        *self == VPS_SEL_A::PB14
    }
    #[doc = "Checks if the value of the field is `PB0`"]
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == VPS_SEL_A::PB0
    }
    #[doc = "Checks if the value of the field is `PA7`"]
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        *self == VPS_SEL_A::PA7
    }
}
#[doc = "Write proxy for field `VPS_SEL`"]
pub struct VPS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VPS_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VPS_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PB14 used as OPAMP2 non-inverting input when TCM_EN=1"]
    #[inline(always)]
    pub fn pb14(self) -> &'a mut W {
        self.variant(VPS_SEL_A::PB14)
    }
    #[doc = "PB0 used as OPAMP2 non-inverting input when TCM_EN=1"]
    #[inline(always)]
    pub fn pb0(self) -> &'a mut W {
        self.variant(VPS_SEL_A::PB0)
    }
    #[doc = "PA7 used as OPAMP2 non-inverting input when TCM_EN=1"]
    #[inline(always)]
    pub fn pa7(self) -> &'a mut W {
        self.variant(VPS_SEL_A::PA7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Calibration mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALON_A {
    #[doc = "0: Calibration mode disabled"]
    DISABLED = 0,
    #[doc = "1: Calibration mode enabled"]
    ENABLED = 1,
}
impl From<CALON_A> for bool {
    #[inline(always)]
    fn from(variant: CALON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CALON`"]
pub type CALON_R = crate::R<bool, CALON_A>;
impl CALON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALON_A {
        match self.bits {
            false => CALON_A::DISABLED,
            true => CALON_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CALON_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CALON_A::ENABLED
    }
}
#[doc = "Write proxy for field `CALON`"]
pub struct CALON_W<'a> {
    w: &'a mut W,
}
impl<'a> CALON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Calibration mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CALON_A::DISABLED)
    }
    #[doc = "Calibration mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CALON_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Calibration selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CALSEL_A {
    #[doc = "0: VREFOPAMP=3.3% VDDA"]
    PERCENT3_3 = 0,
    #[doc = "1: VREFOPAMP=10% VDDA"]
    PERCENT10 = 1,
    #[doc = "2: VREFOPAMP=50% VDDA"]
    PERCENT50 = 2,
    #[doc = "3: VREFOPAMP=90% VDDA"]
    PERCENT90 = 3,
}
impl From<CALSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CALSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CALSEL`"]
pub type CALSEL_R = crate::R<u8, CALSEL_A>;
impl CALSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALSEL_A {
        match self.bits {
            0 => CALSEL_A::PERCENT3_3,
            1 => CALSEL_A::PERCENT10,
            2 => CALSEL_A::PERCENT50,
            3 => CALSEL_A::PERCENT90,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PERCENT3_3`"]
    #[inline(always)]
    pub fn is_percent3_3(&self) -> bool {
        *self == CALSEL_A::PERCENT3_3
    }
    #[doc = "Checks if the value of the field is `PERCENT10`"]
    #[inline(always)]
    pub fn is_percent10(&self) -> bool {
        *self == CALSEL_A::PERCENT10
    }
    #[doc = "Checks if the value of the field is `PERCENT50`"]
    #[inline(always)]
    pub fn is_percent50(&self) -> bool {
        *self == CALSEL_A::PERCENT50
    }
    #[doc = "Checks if the value of the field is `PERCENT90`"]
    #[inline(always)]
    pub fn is_percent90(&self) -> bool {
        *self == CALSEL_A::PERCENT90
    }
}
#[doc = "Write proxy for field `CALSEL`"]
pub struct CALSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CALSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "VREFOPAMP=3.3% VDDA"]
    #[inline(always)]
    pub fn percent3_3(self) -> &'a mut W {
        self.variant(CALSEL_A::PERCENT3_3)
    }
    #[doc = "VREFOPAMP=10% VDDA"]
    #[inline(always)]
    pub fn percent10(self) -> &'a mut W {
        self.variant(CALSEL_A::PERCENT10)
    }
    #[doc = "VREFOPAMP=50% VDDA"]
    #[inline(always)]
    pub fn percent50(self) -> &'a mut W {
        self.variant(CALSEL_A::PERCENT50)
    }
    #[doc = "VREFOPAMP=90% VDDA"]
    #[inline(always)]
    pub fn percent90(self) -> &'a mut W {
        self.variant(CALSEL_A::PERCENT90)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Gain in PGA mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PGA_GAIN_A {
    #[doc = "0: Gain 2"]
    GAIN2 = 0,
    #[doc = "1: Gain 4"]
    GAIN4 = 1,
    #[doc = "2: Gain 8"]
    GAIN8 = 2,
    #[doc = "4: Gain 16"]
    GAIN16 = 4,
    #[doc = "8: Gain 2, feedback connected to VM0"]
    GAIN2_VM0 = 8,
    #[doc = "9: Gain 4, feedback connected to VM0"]
    GAIN4_VM0 = 9,
    #[doc = "10: Gain 8, feedback connected to VM0"]
    GAIN8_VM0 = 10,
    #[doc = "11: Gain 16, feedback connected to VM0"]
    GAIN16_VM0 = 11,
    #[doc = "12: Gain 2, feedback connected to VM1"]
    GAIN2_VM1 = 12,
    #[doc = "13: Gain 4, feedback connected to VM1"]
    GAIN4_VM1 = 13,
    #[doc = "14: Gain 8, feedback connected to VM1"]
    GAIN8_VM1 = 14,
    #[doc = "15: Gain 16, feedback connected to VM1"]
    GAIN16_VM1 = 15,
}
impl From<PGA_GAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: PGA_GAIN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PGA_GAIN`"]
pub type PGA_GAIN_R = crate::R<u8, PGA_GAIN_A>;
impl PGA_GAIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PGA_GAIN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PGA_GAIN_A::GAIN2),
            1 => Val(PGA_GAIN_A::GAIN4),
            2 => Val(PGA_GAIN_A::GAIN8),
            4 => Val(PGA_GAIN_A::GAIN16),
            8 => Val(PGA_GAIN_A::GAIN2_VM0),
            9 => Val(PGA_GAIN_A::GAIN4_VM0),
            10 => Val(PGA_GAIN_A::GAIN8_VM0),
            11 => Val(PGA_GAIN_A::GAIN16_VM0),
            12 => Val(PGA_GAIN_A::GAIN2_VM1),
            13 => Val(PGA_GAIN_A::GAIN4_VM1),
            14 => Val(PGA_GAIN_A::GAIN8_VM1),
            15 => Val(PGA_GAIN_A::GAIN16_VM1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GAIN2`"]
    #[inline(always)]
    pub fn is_gain2(&self) -> bool {
        *self == PGA_GAIN_A::GAIN2
    }
    #[doc = "Checks if the value of the field is `GAIN4`"]
    #[inline(always)]
    pub fn is_gain4(&self) -> bool {
        *self == PGA_GAIN_A::GAIN4
    }
    #[doc = "Checks if the value of the field is `GAIN8`"]
    #[inline(always)]
    pub fn is_gain8(&self) -> bool {
        *self == PGA_GAIN_A::GAIN8
    }
    #[doc = "Checks if the value of the field is `GAIN16`"]
    #[inline(always)]
    pub fn is_gain16(&self) -> bool {
        *self == PGA_GAIN_A::GAIN16
    }
    #[doc = "Checks if the value of the field is `GAIN2_VM0`"]
    #[inline(always)]
    pub fn is_gain2_vm0(&self) -> bool {
        *self == PGA_GAIN_A::GAIN2_VM0
    }
    #[doc = "Checks if the value of the field is `GAIN4_VM0`"]
    #[inline(always)]
    pub fn is_gain4_vm0(&self) -> bool {
        *self == PGA_GAIN_A::GAIN4_VM0
    }
    #[doc = "Checks if the value of the field is `GAIN8_VM0`"]
    #[inline(always)]
    pub fn is_gain8_vm0(&self) -> bool {
        *self == PGA_GAIN_A::GAIN8_VM0
    }
    #[doc = "Checks if the value of the field is `GAIN16_VM0`"]
    #[inline(always)]
    pub fn is_gain16_vm0(&self) -> bool {
        *self == PGA_GAIN_A::GAIN16_VM0
    }
    #[doc = "Checks if the value of the field is `GAIN2_VM1`"]
    #[inline(always)]
    pub fn is_gain2_vm1(&self) -> bool {
        *self == PGA_GAIN_A::GAIN2_VM1
    }
    #[doc = "Checks if the value of the field is `GAIN4_VM1`"]
    #[inline(always)]
    pub fn is_gain4_vm1(&self) -> bool {
        *self == PGA_GAIN_A::GAIN4_VM1
    }
    #[doc = "Checks if the value of the field is `GAIN8_VM1`"]
    #[inline(always)]
    pub fn is_gain8_vm1(&self) -> bool {
        *self == PGA_GAIN_A::GAIN8_VM1
    }
    #[doc = "Checks if the value of the field is `GAIN16_VM1`"]
    #[inline(always)]
    pub fn is_gain16_vm1(&self) -> bool {
        *self == PGA_GAIN_A::GAIN16_VM1
    }
}
#[doc = "Write proxy for field `PGA_GAIN`"]
pub struct PGA_GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> PGA_GAIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PGA_GAIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Gain 2"]
    #[inline(always)]
    pub fn gain2(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN2)
    }
    #[doc = "Gain 4"]
    #[inline(always)]
    pub fn gain4(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN4)
    }
    #[doc = "Gain 8"]
    #[inline(always)]
    pub fn gain8(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN8)
    }
    #[doc = "Gain 16"]
    #[inline(always)]
    pub fn gain16(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN16)
    }
    #[doc = "Gain 2, feedback connected to VM0"]
    #[inline(always)]
    pub fn gain2_vm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN2_VM0)
    }
    #[doc = "Gain 4, feedback connected to VM0"]
    #[inline(always)]
    pub fn gain4_vm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN4_VM0)
    }
    #[doc = "Gain 8, feedback connected to VM0"]
    #[inline(always)]
    pub fn gain8_vm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN8_VM0)
    }
    #[doc = "Gain 16, feedback connected to VM0"]
    #[inline(always)]
    pub fn gain16_vm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN16_VM0)
    }
    #[doc = "Gain 2, feedback connected to VM1"]
    #[inline(always)]
    pub fn gain2_vm1(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN2_VM1)
    }
    #[doc = "Gain 4, feedback connected to VM1"]
    #[inline(always)]
    pub fn gain4_vm1(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN4_VM1)
    }
    #[doc = "Gain 8, feedback connected to VM1"]
    #[inline(always)]
    pub fn gain8_vm1(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN8_VM1)
    }
    #[doc = "Gain 16, feedback connected to VM1"]
    #[inline(always)]
    pub fn gain16_vm1(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN16_VM1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 14)) | (((value as u32) & 0x0f) << 14);
        self.w
    }
}
#[doc = "User trimming enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USER_TRIM_A {
    #[doc = "0: User trimming disabled"]
    DISABLED = 0,
    #[doc = "1: User trimming enabled"]
    ENABLED = 1,
}
impl From<USER_TRIM_A> for bool {
    #[inline(always)]
    fn from(variant: USER_TRIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USER_TRIM`"]
pub type USER_TRIM_R = crate::R<bool, USER_TRIM_A>;
impl USER_TRIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USER_TRIM_A {
        match self.bits {
            false => USER_TRIM_A::DISABLED,
            true => USER_TRIM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USER_TRIM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USER_TRIM_A::ENABLED
    }
}
#[doc = "Write proxy for field `USER_TRIM`"]
pub struct USER_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> USER_TRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USER_TRIM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "User trimming disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USER_TRIM_A::DISABLED)
    }
    #[doc = "User trimming enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USER_TRIM_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `TRIMOFFSETP`"]
pub type TRIMOFFSETP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIMOFFSETP`"]
pub struct TRIMOFFSETP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMOFFSETP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | (((value as u32) & 0x1f) << 19);
        self.w
    }
}
#[doc = "Reader of field `TRIMOFFSETN`"]
pub type TRIMOFFSETN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIMOFFSETN`"]
pub struct TRIMOFFSETN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMOFFSETN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "TSTREF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTREF_A {
    #[doc = "0: VREFOPAMP2 is output"]
    OUTPUT = 0,
    #[doc = "1: VREFOPAMP2 is not output"]
    NOTOUTPUT = 1,
}
impl From<TSTREF_A> for bool {
    #[inline(always)]
    fn from(variant: TSTREF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TSTREF`"]
pub type TSTREF_R = crate::R<bool, TSTREF_A>;
impl TSTREF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSTREF_A {
        match self.bits {
            false => TSTREF_A::OUTPUT,
            true => TSTREF_A::NOTOUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == TSTREF_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `NOTOUTPUT`"]
    #[inline(always)]
    pub fn is_not_output(&self) -> bool {
        *self == TSTREF_A::NOTOUTPUT
    }
}
#[doc = "Write proxy for field `TSTREF`"]
pub struct TSTREF_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSTREF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "VREFOPAMP2 is output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(TSTREF_A::OUTPUT)
    }
    #[doc = "VREFOPAMP2 is not output"]
    #[inline(always)]
    pub fn not_output(self) -> &'a mut W {
        self.variant(TSTREF_A::NOTOUTPUT)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "OPAMP ouput status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCAL_A {
    #[doc = "0: Non-inverting < inverting"]
    LOW = 0,
    #[doc = "1: Non-inverting > inverting"]
    HIGH = 1,
}
impl From<OUTCAL_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCAL`"]
pub type OUTCAL_R = crate::R<bool, OUTCAL_A>;
impl OUTCAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCAL_A {
        match self.bits {
            false => OUTCAL_A::LOW,
            true => OUTCAL_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OUTCAL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OUTCAL_A::HIGH
    }
}
#[doc = "OPAMP lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "0: Comparator CSR bits are read-write"]
    UNLOCKED = 0,
    #[doc = "1: Comparator CSR bits are read-only"]
    LOCKED = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, LOCK_A>;
impl LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::UNLOCKED,
            true => LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK_A::LOCKED
    }
}
#[doc = "Write proxy for field `LOCK`"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Comparator CSR bits are read-write"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCK_A::UNLOCKED)
    }
    #[doc = "Comparator CSR bits are read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCK_A::LOCKED)
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
impl R {
    #[doc = "Bit 0 - OPAMP2 enable"]
    #[inline(always)]
    pub fn opamp2en(&self) -> OPAMP2EN_R {
        OPAMP2EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FORCE_VP"]
    #[inline(always)]
    pub fn force_vp(&self) -> FORCE_VP_R {
        FORCE_VP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - OPAMP Non inverting input selection"]
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 5:6 - OPAMP inverting input selection"]
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Timer controlled Mux mode enable"]
    #[inline(always)]
    pub fn tcm_en(&self) -> TCM_EN_R {
        TCM_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - OPAMP inverting input secondary selection"]
    #[inline(always)]
    pub fn vms_sel(&self) -> VMS_SEL_R {
        VMS_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - OPAMP Non inverting input secondary selection"]
    #[inline(always)]
    pub fn vps_sel(&self) -> VPS_SEL_R {
        VPS_SEL_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Calibration mode enable"]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Calibration selection"]
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:17 - Gain in PGA mode"]
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - User trimming enable"]
    #[inline(always)]
    pub fn user_trim(&self) -> USER_TRIM_R {
        USER_TRIM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 19:23 - Offset trimming value (PMOS)"]
    #[inline(always)]
    pub fn trimoffsetp(&self) -> TRIMOFFSETP_R {
        TRIMOFFSETP_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Offset trimming value (NMOS)"]
    #[inline(always)]
    pub fn trimoffsetn(&self) -> TRIMOFFSETN_R {
        TRIMOFFSETN_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - TSTREF"]
    #[inline(always)]
    pub fn tstref(&self) -> TSTREF_R {
        TSTREF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - OPAMP ouput status flag"]
    #[inline(always)]
    pub fn outcal(&self) -> OUTCAL_R {
        OUTCAL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - OPAMP lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OPAMP2 enable"]
    #[inline(always)]
    pub fn opamp2en(&mut self) -> OPAMP2EN_W {
        OPAMP2EN_W { w: self }
    }
    #[doc = "Bit 1 - FORCE_VP"]
    #[inline(always)]
    pub fn force_vp(&mut self) -> FORCE_VP_W {
        FORCE_VP_W { w: self }
    }
    #[doc = "Bits 2:3 - OPAMP Non inverting input selection"]
    #[inline(always)]
    pub fn vp_sel(&mut self) -> VP_SEL_W {
        VP_SEL_W { w: self }
    }
    #[doc = "Bits 5:6 - OPAMP inverting input selection"]
    #[inline(always)]
    pub fn vm_sel(&mut self) -> VM_SEL_W {
        VM_SEL_W { w: self }
    }
    #[doc = "Bit 7 - Timer controlled Mux mode enable"]
    #[inline(always)]
    pub fn tcm_en(&mut self) -> TCM_EN_W {
        TCM_EN_W { w: self }
    }
    #[doc = "Bit 8 - OPAMP inverting input secondary selection"]
    #[inline(always)]
    pub fn vms_sel(&mut self) -> VMS_SEL_W {
        VMS_SEL_W { w: self }
    }
    #[doc = "Bits 9:10 - OPAMP Non inverting input secondary selection"]
    #[inline(always)]
    pub fn vps_sel(&mut self) -> VPS_SEL_W {
        VPS_SEL_W { w: self }
    }
    #[doc = "Bit 11 - Calibration mode enable"]
    #[inline(always)]
    pub fn calon(&mut self) -> CALON_W {
        CALON_W { w: self }
    }
    #[doc = "Bits 12:13 - Calibration selection"]
    #[inline(always)]
    pub fn calsel(&mut self) -> CALSEL_W {
        CALSEL_W { w: self }
    }
    #[doc = "Bits 14:17 - Gain in PGA mode"]
    #[inline(always)]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W {
        PGA_GAIN_W { w: self }
    }
    #[doc = "Bit 18 - User trimming enable"]
    #[inline(always)]
    pub fn user_trim(&mut self) -> USER_TRIM_W {
        USER_TRIM_W { w: self }
    }
    #[doc = "Bits 19:23 - Offset trimming value (PMOS)"]
    #[inline(always)]
    pub fn trimoffsetp(&mut self) -> TRIMOFFSETP_W {
        TRIMOFFSETP_W { w: self }
    }
    #[doc = "Bits 24:28 - Offset trimming value (NMOS)"]
    #[inline(always)]
    pub fn trimoffsetn(&mut self) -> TRIMOFFSETN_W {
        TRIMOFFSETN_W { w: self }
    }
    #[doc = "Bit 29 - TSTREF"]
    #[inline(always)]
    pub fn tstref(&mut self) -> TSTREF_W {
        TSTREF_W { w: self }
    }
    #[doc = "Bit 31 - OPAMP lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
}
