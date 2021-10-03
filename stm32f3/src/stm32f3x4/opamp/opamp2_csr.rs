#[doc = "Register `OPAMP2_CSR` reader"]
pub struct R(crate::R<OPAMP2_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAMP2_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAMP2_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAMP2_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPAMP2_CSR` writer"]
pub struct W(crate::W<OPAMP2_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAMP2_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<OPAMP2_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAMP2_CSR_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `OPAMP2EN` reader - OPAMP2 enable"]
pub struct OPAMP2EN_R(crate::FieldReader<bool, OPAMP2EN_A>);
impl OPAMP2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPAMP2EN_R(crate::FieldReader::new(bits))
    }
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
        **self == OPAMP2EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == OPAMP2EN_A::ENABLED
    }
}
impl core::ops::Deref for OPAMP2EN_R {
    type Target = crate::FieldReader<bool, OPAMP2EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPAMP2EN` writer - OPAMP2 enable"]
pub struct OPAMP2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAMP2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAMP2EN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
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
#[doc = "Field `FORCE_VP` reader - FORCE_VP"]
pub struct FORCE_VP_R(crate::FieldReader<bool, FORCE_VP_A>);
impl FORCE_VP_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_VP_R(crate::FieldReader::new(bits))
    }
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
        **self == FORCE_VP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `CALIBRATION`"]
    #[inline(always)]
    pub fn is_calibration(&self) -> bool {
        **self == FORCE_VP_A::CALIBRATION
    }
}
impl core::ops::Deref for FORCE_VP_R {
    type Target = crate::FieldReader<bool, FORCE_VP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_VP` writer - FORCE_VP"]
pub struct FORCE_VP_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_VP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCE_VP_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
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
#[doc = "Field `VP_SEL` reader - OPAMP Non inverting input selection"]
pub struct VP_SEL_R(crate::FieldReader<u8, VP_SEL_A>);
impl VP_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        VP_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VP_SEL_A> {
        match self.bits {
            1 => Some(VP_SEL_A::PB14),
            2 => Some(VP_SEL_A::PB0),
            3 => Some(VP_SEL_A::PA7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PB14`"]
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        **self == VP_SEL_A::PB14
    }
    #[doc = "Checks if the value of the field is `PB0`"]
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        **self == VP_SEL_A::PB0
    }
    #[doc = "Checks if the value of the field is `PA7`"]
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        **self == VP_SEL_A::PA7
    }
}
impl core::ops::Deref for VP_SEL_R {
    type Target = crate::FieldReader<u8, VP_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VP_SEL` writer - OPAMP Non inverting input selection"]
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
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
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
#[doc = "Field `VM_SEL` reader - OPAMP inverting input selection"]
pub struct VM_SEL_R(crate::FieldReader<u8, VM_SEL_A>);
impl VM_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        VM_SEL_R(crate::FieldReader::new(bits))
    }
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
        **self == VM_SEL_A::PC5
    }
    #[doc = "Checks if the value of the field is `PA5`"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        **self == VM_SEL_A::PA5
    }
    #[doc = "Checks if the value of the field is `PGA`"]
    #[inline(always)]
    pub fn is_pga(&self) -> bool {
        **self == VM_SEL_A::PGA
    }
    #[doc = "Checks if the value of the field is `FOLLOWER`"]
    #[inline(always)]
    pub fn is_follower(&self) -> bool {
        **self == VM_SEL_A::FOLLOWER
    }
}
impl core::ops::Deref for VM_SEL_R {
    type Target = crate::FieldReader<u8, VM_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VM_SEL` writer - OPAMP inverting input selection"]
pub struct VM_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VM_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VM_SEL_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
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
#[doc = "Field `TCM_EN` reader - Timer controlled Mux mode enable"]
pub struct TCM_EN_R(crate::FieldReader<bool, TCM_EN_A>);
impl TCM_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCM_EN_R(crate::FieldReader::new(bits))
    }
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
        **self == TCM_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TCM_EN_A::ENABLED
    }
}
impl core::ops::Deref for TCM_EN_R {
    type Target = crate::FieldReader<bool, TCM_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCM_EN` writer - Timer controlled Mux mode enable"]
pub struct TCM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TCM_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCM_EN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
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
#[doc = "Field `VMS_SEL` reader - OPAMP inverting input secondary selection"]
pub struct VMS_SEL_R(crate::FieldReader<bool, VMS_SEL_A>);
impl VMS_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        VMS_SEL_R(crate::FieldReader::new(bits))
    }
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
        **self == VMS_SEL_A::PC5
    }
    #[doc = "Checks if the value of the field is `PA5`"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        **self == VMS_SEL_A::PA5
    }
}
impl core::ops::Deref for VMS_SEL_R {
    type Target = crate::FieldReader<bool, VMS_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VMS_SEL` writer - OPAMP inverting input secondary selection"]
pub struct VMS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VMS_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VMS_SEL_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
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
#[doc = "Field `VPS_SEL` reader - OPAMP Non inverting input secondary selection"]
pub struct VPS_SEL_R(crate::FieldReader<u8, VPS_SEL_A>);
impl VPS_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        VPS_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VPS_SEL_A> {
        match self.bits {
            1 => Some(VPS_SEL_A::PB14),
            2 => Some(VPS_SEL_A::PB0),
            3 => Some(VPS_SEL_A::PA7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PB14`"]
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        **self == VPS_SEL_A::PB14
    }
    #[doc = "Checks if the value of the field is `PB0`"]
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        **self == VPS_SEL_A::PB0
    }
    #[doc = "Checks if the value of the field is `PA7`"]
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        **self == VPS_SEL_A::PA7
    }
}
impl core::ops::Deref for VPS_SEL_R {
    type Target = crate::FieldReader<u8, VPS_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VPS_SEL` writer - OPAMP Non inverting input secondary selection"]
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
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
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
#[doc = "Field `CALON` reader - Calibration mode enable"]
pub struct CALON_R(crate::FieldReader<bool, CALON_A>);
impl CALON_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALON_R(crate::FieldReader::new(bits))
    }
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
        **self == CALON_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CALON_A::ENABLED
    }
}
impl core::ops::Deref for CALON_R {
    type Target = crate::FieldReader<bool, CALON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALON` writer - Calibration mode enable"]
pub struct CALON_W<'a> {
    w: &'a mut W,
}
impl<'a> CALON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALON_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
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
#[doc = "Field `CALSEL` reader - Calibration selection"]
pub struct CALSEL_R(crate::FieldReader<u8, CALSEL_A>);
impl CALSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CALSEL_R(crate::FieldReader::new(bits))
    }
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
        **self == CALSEL_A::PERCENT3_3
    }
    #[doc = "Checks if the value of the field is `PERCENT10`"]
    #[inline(always)]
    pub fn is_percent10(&self) -> bool {
        **self == CALSEL_A::PERCENT10
    }
    #[doc = "Checks if the value of the field is `PERCENT50`"]
    #[inline(always)]
    pub fn is_percent50(&self) -> bool {
        **self == CALSEL_A::PERCENT50
    }
    #[doc = "Checks if the value of the field is `PERCENT90`"]
    #[inline(always)]
    pub fn is_percent90(&self) -> bool {
        **self == CALSEL_A::PERCENT90
    }
}
impl core::ops::Deref for CALSEL_R {
    type Target = crate::FieldReader<u8, CALSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALSEL` writer - Calibration selection"]
pub struct CALSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CALSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALSEL_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
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
#[doc = "Field `PGA_GAIN` reader - Gain in PGA mode"]
pub struct PGA_GAIN_R(crate::FieldReader<u8, PGA_GAIN_A>);
impl PGA_GAIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        PGA_GAIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PGA_GAIN_A> {
        match self.bits {
            0 => Some(PGA_GAIN_A::GAIN2),
            1 => Some(PGA_GAIN_A::GAIN4),
            2 => Some(PGA_GAIN_A::GAIN8),
            4 => Some(PGA_GAIN_A::GAIN16),
            8 => Some(PGA_GAIN_A::GAIN2_VM0),
            9 => Some(PGA_GAIN_A::GAIN4_VM0),
            10 => Some(PGA_GAIN_A::GAIN8_VM0),
            11 => Some(PGA_GAIN_A::GAIN16_VM0),
            12 => Some(PGA_GAIN_A::GAIN2_VM1),
            13 => Some(PGA_GAIN_A::GAIN4_VM1),
            14 => Some(PGA_GAIN_A::GAIN8_VM1),
            15 => Some(PGA_GAIN_A::GAIN16_VM1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GAIN2`"]
    #[inline(always)]
    pub fn is_gain2(&self) -> bool {
        **self == PGA_GAIN_A::GAIN2
    }
    #[doc = "Checks if the value of the field is `GAIN4`"]
    #[inline(always)]
    pub fn is_gain4(&self) -> bool {
        **self == PGA_GAIN_A::GAIN4
    }
    #[doc = "Checks if the value of the field is `GAIN8`"]
    #[inline(always)]
    pub fn is_gain8(&self) -> bool {
        **self == PGA_GAIN_A::GAIN8
    }
    #[doc = "Checks if the value of the field is `GAIN16`"]
    #[inline(always)]
    pub fn is_gain16(&self) -> bool {
        **self == PGA_GAIN_A::GAIN16
    }
    #[doc = "Checks if the value of the field is `GAIN2_VM0`"]
    #[inline(always)]
    pub fn is_gain2_vm0(&self) -> bool {
        **self == PGA_GAIN_A::GAIN2_VM0
    }
    #[doc = "Checks if the value of the field is `GAIN4_VM0`"]
    #[inline(always)]
    pub fn is_gain4_vm0(&self) -> bool {
        **self == PGA_GAIN_A::GAIN4_VM0
    }
    #[doc = "Checks if the value of the field is `GAIN8_VM0`"]
    #[inline(always)]
    pub fn is_gain8_vm0(&self) -> bool {
        **self == PGA_GAIN_A::GAIN8_VM0
    }
    #[doc = "Checks if the value of the field is `GAIN16_VM0`"]
    #[inline(always)]
    pub fn is_gain16_vm0(&self) -> bool {
        **self == PGA_GAIN_A::GAIN16_VM0
    }
    #[doc = "Checks if the value of the field is `GAIN2_VM1`"]
    #[inline(always)]
    pub fn is_gain2_vm1(&self) -> bool {
        **self == PGA_GAIN_A::GAIN2_VM1
    }
    #[doc = "Checks if the value of the field is `GAIN4_VM1`"]
    #[inline(always)]
    pub fn is_gain4_vm1(&self) -> bool {
        **self == PGA_GAIN_A::GAIN4_VM1
    }
    #[doc = "Checks if the value of the field is `GAIN8_VM1`"]
    #[inline(always)]
    pub fn is_gain8_vm1(&self) -> bool {
        **self == PGA_GAIN_A::GAIN8_VM1
    }
    #[doc = "Checks if the value of the field is `GAIN16_VM1`"]
    #[inline(always)]
    pub fn is_gain16_vm1(&self) -> bool {
        **self == PGA_GAIN_A::GAIN16_VM1
    }
}
impl core::ops::Deref for PGA_GAIN_R {
    type Target = crate::FieldReader<u8, PGA_GAIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGA_GAIN` writer - Gain in PGA mode"]
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
        self.w.bits = (self.w.bits & !(0x0f << 14)) | ((value as u32 & 0x0f) << 14);
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
#[doc = "Field `USER_TRIM` reader - User trimming enable"]
pub struct USER_TRIM_R(crate::FieldReader<bool, USER_TRIM_A>);
impl USER_TRIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        USER_TRIM_R(crate::FieldReader::new(bits))
    }
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
        **self == USER_TRIM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == USER_TRIM_A::ENABLED
    }
}
impl core::ops::Deref for USER_TRIM_R {
    type Target = crate::FieldReader<bool, USER_TRIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USER_TRIM` writer - User trimming enable"]
pub struct USER_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> USER_TRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USER_TRIM_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `TRIMOFFSETP` reader - Offset trimming value (PMOS)"]
pub struct TRIMOFFSETP_R(crate::FieldReader<u8, u8>);
impl TRIMOFFSETP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIMOFFSETP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIMOFFSETP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIMOFFSETP` writer - Offset trimming value (PMOS)"]
pub struct TRIMOFFSETP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMOFFSETP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | ((value as u32 & 0x1f) << 19);
        self.w
    }
}
#[doc = "Field `TRIMOFFSETN` reader - Offset trimming value (NMOS)"]
pub struct TRIMOFFSETN_R(crate::FieldReader<u8, u8>);
impl TRIMOFFSETN_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIMOFFSETN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIMOFFSETN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIMOFFSETN` writer - Offset trimming value (NMOS)"]
pub struct TRIMOFFSETN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMOFFSETN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
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
#[doc = "Field `TSTREF` reader - TSTREF"]
pub struct TSTREF_R(crate::FieldReader<bool, TSTREF_A>);
impl TSTREF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSTREF_R(crate::FieldReader::new(bits))
    }
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
        **self == TSTREF_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `NOTOUTPUT`"]
    #[inline(always)]
    pub fn is_not_output(&self) -> bool {
        **self == TSTREF_A::NOTOUTPUT
    }
}
impl core::ops::Deref for TSTREF_R {
    type Target = crate::FieldReader<bool, TSTREF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTREF` writer - TSTREF"]
pub struct TSTREF_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSTREF_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
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
#[doc = "Field `OUTCAL` reader - OPAMP ouput status flag"]
pub struct OUTCAL_R(crate::FieldReader<bool, OUTCAL_A>);
impl OUTCAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUTCAL_R(crate::FieldReader::new(bits))
    }
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
        **self == OUTCAL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == OUTCAL_A::HIGH
    }
}
impl core::ops::Deref for OUTCAL_R {
    type Target = crate::FieldReader<bool, OUTCAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `LOCK` reader - OPAMP lock"]
pub struct LOCK_R(crate::FieldReader<bool, LOCK_A>);
impl LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
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
        **self == LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == LOCK_A::LOCKED
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK` writer - OPAMP lock"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPAMP2 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opamp2_csr](index.html) module"]
pub struct OPAMP2_CSR_SPEC;
impl crate::RegisterSpec for OPAMP2_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opamp2_csr::R](R) reader structure"]
impl crate::Readable for OPAMP2_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opamp2_csr::W](W) writer structure"]
impl crate::Writable for OPAMP2_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPAMP2_CSR to value 0"]
impl crate::Resettable for OPAMP2_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
