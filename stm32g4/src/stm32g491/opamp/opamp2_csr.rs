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
#[doc = "Operational amplifier Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAEN_A {
    #[doc = "0: OpAmp disabled"]
    DISABLED = 0,
    #[doc = "1: OpAmp enabled"]
    ENABLED = 1,
}
impl From<OPAEN_A> for bool {
    #[inline(always)]
    fn from(variant: OPAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OPAEN`"]
pub type OPAEN_R = crate::R<bool, OPAEN_A>;
impl OPAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPAEN_A {
        match self.bits {
            false => OPAEN_A::DISABLED,
            true => OPAEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OPAEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OPAEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `OPAEN`"]
pub struct OPAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "OpAmp disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OPAEN_A::DISABLED)
    }
    #[doc = "OpAmp enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OPAEN_A::ENABLED)
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
    #[doc = "0: Non-inverting input connected configured inputs"]
    NORMAL = 0,
    #[doc = "1: Non-inverting input connected to calibration reference voltage"]
    CALIBRATIONVERIFICATION = 1,
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
            true => FORCE_VP_A::CALIBRATIONVERIFICATION,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == FORCE_VP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `CALIBRATIONVERIFICATION`"]
    #[inline(always)]
    pub fn is_calibration_verification(&self) -> bool {
        *self == FORCE_VP_A::CALIBRATIONVERIFICATION
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
    #[doc = "Non-inverting input connected configured inputs"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(FORCE_VP_A::NORMAL)
    }
    #[doc = "Non-inverting input connected to calibration reference voltage"]
    #[inline(always)]
    pub fn calibration_verification(self) -> &'a mut W {
        self.variant(FORCE_VP_A::CALIBRATIONVERIFICATION)
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
#[doc = "VP_SEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VP_SEL_A {
    #[doc = "0: VINP0 connected to VINP input"]
    VINP0 = 0,
    #[doc = "1: VINP1 connected to VINP input"]
    VINP1 = 1,
    #[doc = "2: VINP2 connected to VINP input"]
    VINP2 = 2,
    #[doc = "3: VINP3 connected to VINP input"]
    VINP3 = 3,
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
    pub fn variant(&self) -> VP_SEL_A {
        match self.bits {
            0 => VP_SEL_A::VINP0,
            1 => VP_SEL_A::VINP1,
            2 => VP_SEL_A::VINP2,
            3 => VP_SEL_A::VINP3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VINP0`"]
    #[inline(always)]
    pub fn is_vinp0(&self) -> bool {
        *self == VP_SEL_A::VINP0
    }
    #[doc = "Checks if the value of the field is `VINP1`"]
    #[inline(always)]
    pub fn is_vinp1(&self) -> bool {
        *self == VP_SEL_A::VINP1
    }
    #[doc = "Checks if the value of the field is `VINP2`"]
    #[inline(always)]
    pub fn is_vinp2(&self) -> bool {
        *self == VP_SEL_A::VINP2
    }
    #[doc = "Checks if the value of the field is `VINP3`"]
    #[inline(always)]
    pub fn is_vinp3(&self) -> bool {
        *self == VP_SEL_A::VINP3
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
        {
            self.bits(variant.into())
        }
    }
    #[doc = "VINP0 connected to VINP input"]
    #[inline(always)]
    pub fn vinp0(self) -> &'a mut W {
        self.variant(VP_SEL_A::VINP0)
    }
    #[doc = "VINP1 connected to VINP input"]
    #[inline(always)]
    pub fn vinp1(self) -> &'a mut W {
        self.variant(VP_SEL_A::VINP1)
    }
    #[doc = "VINP2 connected to VINP input"]
    #[inline(always)]
    pub fn vinp2(self) -> &'a mut W {
        self.variant(VP_SEL_A::VINP2)
    }
    #[doc = "VINP3 connected to VINP input"]
    #[inline(always)]
    pub fn vinp3(self) -> &'a mut W {
        self.variant(VP_SEL_A::VINP3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "USERTRIM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USERTRIM_A {
    #[doc = "0: Factory trim used"]
    FACTORY = 0,
    #[doc = "1: User trim used"]
    USER = 1,
}
impl From<USERTRIM_A> for bool {
    #[inline(always)]
    fn from(variant: USERTRIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USERTRIM`"]
pub type USERTRIM_R = crate::R<bool, USERTRIM_A>;
impl USERTRIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USERTRIM_A {
        match self.bits {
            false => USERTRIM_A::FACTORY,
            true => USERTRIM_A::USER,
        }
    }
    #[doc = "Checks if the value of the field is `FACTORY`"]
    #[inline(always)]
    pub fn is_factory(&self) -> bool {
        *self == USERTRIM_A::FACTORY
    }
    #[doc = "Checks if the value of the field is `USER`"]
    #[inline(always)]
    pub fn is_user(&self) -> bool {
        *self == USERTRIM_A::USER
    }
}
#[doc = "Write proxy for field `USERTRIM`"]
pub struct USERTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> USERTRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USERTRIM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Factory trim used"]
    #[inline(always)]
    pub fn factory(self) -> &'a mut W {
        self.variant(USERTRIM_A::FACTORY)
    }
    #[doc = "User trim used"]
    #[inline(always)]
    pub fn user(self) -> &'a mut W {
        self.variant(USERTRIM_A::USER)
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
#[doc = "VM_SEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VM_SEL_A {
    #[doc = "0: VINM0 connected to VINM input"]
    VINM0 = 0,
    #[doc = "1: VINM1 connected to VINM input"]
    VINM1 = 1,
    #[doc = "2: Feedback resistor connected to VINM (PGA mode)"]
    PGA = 2,
    #[doc = "3: OpAmp output connected to VINM (Follower mode)"]
    OUTPUT = 3,
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
            0 => VM_SEL_A::VINM0,
            1 => VM_SEL_A::VINM1,
            2 => VM_SEL_A::PGA,
            3 => VM_SEL_A::OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VINM0`"]
    #[inline(always)]
    pub fn is_vinm0(&self) -> bool {
        *self == VM_SEL_A::VINM0
    }
    #[doc = "Checks if the value of the field is `VINM1`"]
    #[inline(always)]
    pub fn is_vinm1(&self) -> bool {
        *self == VM_SEL_A::VINM1
    }
    #[doc = "Checks if the value of the field is `PGA`"]
    #[inline(always)]
    pub fn is_pga(&self) -> bool {
        *self == VM_SEL_A::PGA
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == VM_SEL_A::OUTPUT
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
    #[doc = "VINM0 connected to VINM input"]
    #[inline(always)]
    pub fn vinm0(self) -> &'a mut W {
        self.variant(VM_SEL_A::VINM0)
    }
    #[doc = "VINM1 connected to VINM input"]
    #[inline(always)]
    pub fn vinm1(self) -> &'a mut W {
        self.variant(VM_SEL_A::VINM1)
    }
    #[doc = "Feedback resistor connected to VINM (PGA mode)"]
    #[inline(always)]
    pub fn pga(self) -> &'a mut W {
        self.variant(VM_SEL_A::PGA)
    }
    #[doc = "OpAmp output connected to VINM (Follower mode)"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(VM_SEL_A::OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "OPAHSM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAHSM_A {
    #[doc = "0: OpAmp in normal mode"]
    NORMAL = 0,
    #[doc = "1: OpAmp in high speed mode"]
    HIGHSPEED = 1,
}
impl From<OPAHSM_A> for bool {
    #[inline(always)]
    fn from(variant: OPAHSM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OPAHSM`"]
pub type OPAHSM_R = crate::R<bool, OPAHSM_A>;
impl OPAHSM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPAHSM_A {
        match self.bits {
            false => OPAHSM_A::NORMAL,
            true => OPAHSM_A::HIGHSPEED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == OPAHSM_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `HIGHSPEED`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == OPAHSM_A::HIGHSPEED
    }
}
#[doc = "Write proxy for field `OPAHSM`"]
pub struct OPAHSM_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAHSM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAHSM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "OpAmp in normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(OPAHSM_A::NORMAL)
    }
    #[doc = "OpAmp in high speed mode"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OPAHSM_A::HIGHSPEED)
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
#[doc = "OPAINTOEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAINTOEN_A {
    #[doc = "0: Output is connected to the output Pin"]
    OUTPUTPIN = 0,
    #[doc = "1: Output is connected internally to ADC channel"]
    ADCCHANNEL = 1,
}
impl From<OPAINTOEN_A> for bool {
    #[inline(always)]
    fn from(variant: OPAINTOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OPAINTOEN`"]
pub type OPAINTOEN_R = crate::R<bool, OPAINTOEN_A>;
impl OPAINTOEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPAINTOEN_A {
        match self.bits {
            false => OPAINTOEN_A::OUTPUTPIN,
            true => OPAINTOEN_A::ADCCHANNEL,
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUTPIN`"]
    #[inline(always)]
    pub fn is_output_pin(&self) -> bool {
        *self == OPAINTOEN_A::OUTPUTPIN
    }
    #[doc = "Checks if the value of the field is `ADCCHANNEL`"]
    #[inline(always)]
    pub fn is_adcchannel(&self) -> bool {
        *self == OPAINTOEN_A::ADCCHANNEL
    }
}
#[doc = "Write proxy for field `OPAINTOEN`"]
pub struct OPAINTOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAINTOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAINTOEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output is connected to the output Pin"]
    #[inline(always)]
    pub fn output_pin(self) -> &'a mut W {
        self.variant(OPAINTOEN_A::OUTPUTPIN)
    }
    #[doc = "Output is connected internally to ADC channel"]
    #[inline(always)]
    pub fn adcchannel(self) -> &'a mut W {
        self.variant(OPAINTOEN_A::ADCCHANNEL)
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
#[doc = "CALON\n\nValue on reset: 0"]
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
#[doc = "CALSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CALSEL_A {
    #[doc = "0: 0.033*VDDA applied to OPAMP inputs during calibration"]
    PERCENT3_3 = 0,
    #[doc = "1: 0.1*VDDA applied to OPAMP inputs during calibration"]
    PERCENT10 = 1,
    #[doc = "2: 0.5*VDDA applied to OPAMP inputs during calibration"]
    PERCENT50 = 2,
    #[doc = "3: 0.9*VDDA applied to OPAMP inputs during calibration"]
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
    #[doc = "0.033*VDDA applied to OPAMP inputs during calibration"]
    #[inline(always)]
    pub fn percent3_3(self) -> &'a mut W {
        self.variant(CALSEL_A::PERCENT3_3)
    }
    #[doc = "0.1*VDDA applied to OPAMP inputs during calibration"]
    #[inline(always)]
    pub fn percent10(self) -> &'a mut W {
        self.variant(CALSEL_A::PERCENT10)
    }
    #[doc = "0.5*VDDA applied to OPAMP inputs during calibration"]
    #[inline(always)]
    pub fn percent50(self) -> &'a mut W {
        self.variant(CALSEL_A::PERCENT50)
    }
    #[doc = "0.9*VDDA applied to OPAMP inputs during calibration"]
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
#[doc = "PGA_GAIN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PGA_GAIN_A {
    #[doc = "0: Gain 2"]
    GAIN2 = 0,
    #[doc = "1: Gain 4"]
    GAIN4 = 1,
    #[doc = "2: Gain 8"]
    GAIN8 = 2,
    #[doc = "3: Gain 16"]
    GAIN16 = 3,
    #[doc = "4: Gain 32"]
    GAIN32 = 4,
    #[doc = "5: Gain 64"]
    GAIN64 = 5,
    #[doc = "8: Gain 2, input/bias connected to VINM0 or inverting gain"]
    GAIN2_INPUTVINM0 = 8,
    #[doc = "9: Gain 4, input/bias connected to VINM0 or inverting gain"]
    GAIN4_INPUTVINM0 = 9,
    #[doc = "10: Gain 8, input/bias connected to VINM0 or inverting gain"]
    GAIN8_INPUTVINM0 = 10,
    #[doc = "11: Gain 16, input/bias connected to VINM0 or inverting gain"]
    GAIN16_INPUTVINM0 = 11,
    #[doc = "12: Gain 32, input/bias connected to VINM0 or inverting gain"]
    GAIN32_INPUTVINM0 = 12,
    #[doc = "13: Gain 64, input/bias connected to VINM0 or inverting gain"]
    GAIN64_INPUTVINM0 = 13,
    #[doc = "16: Gain 2, with filtering on VINM0"]
    GAIN2_FILTERINGVINM0 = 16,
    #[doc = "17: Gain 4, with filtering on VINM0"]
    GAIN4_FILTERINGVINM0 = 17,
    #[doc = "18: Gain 8, with filtering on VINM0"]
    GAIN8_FILTERINGVINM0 = 18,
    #[doc = "19: Gain 16, with filtering on VINM0"]
    GAIN16_FILTERINGVINM0 = 19,
    #[doc = "20: Gain 32, with filtering on VINM0"]
    GAIN32_FILTERINGVINM0 = 20,
    #[doc = "21: Gain 64, with filtering on VINM0"]
    GAIN64_FILTERINGVINM0 = 21,
    #[doc = "24: Gain 2, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    GAIN2_INPUTVINM0FILTERINGVINM1 = 24,
    #[doc = "25: Gain 4, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    GAIN4_INPUTVINM0FILTERINGVINM1 = 25,
    #[doc = "26: Gain 8, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    GAIN8_INPUTVINM0FILTERINGVINM1 = 26,
    #[doc = "27: Gain 16, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    GAIN16_INPUTVINM0FILTERINGVINM1 = 27,
    #[doc = "28: Gain 32, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    GAIN32_INPUTVINM0FILTERINGVINM1 = 28,
    #[doc = "29: Gain 64, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    GAIN64_INPUTVINM0FILTERINGVINM1 = 29,
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
            3 => Val(PGA_GAIN_A::GAIN16),
            4 => Val(PGA_GAIN_A::GAIN32),
            5 => Val(PGA_GAIN_A::GAIN64),
            8 => Val(PGA_GAIN_A::GAIN2_INPUTVINM0),
            9 => Val(PGA_GAIN_A::GAIN4_INPUTVINM0),
            10 => Val(PGA_GAIN_A::GAIN8_INPUTVINM0),
            11 => Val(PGA_GAIN_A::GAIN16_INPUTVINM0),
            12 => Val(PGA_GAIN_A::GAIN32_INPUTVINM0),
            13 => Val(PGA_GAIN_A::GAIN64_INPUTVINM0),
            16 => Val(PGA_GAIN_A::GAIN2_FILTERINGVINM0),
            17 => Val(PGA_GAIN_A::GAIN4_FILTERINGVINM0),
            18 => Val(PGA_GAIN_A::GAIN8_FILTERINGVINM0),
            19 => Val(PGA_GAIN_A::GAIN16_FILTERINGVINM0),
            20 => Val(PGA_GAIN_A::GAIN32_FILTERINGVINM0),
            21 => Val(PGA_GAIN_A::GAIN64_FILTERINGVINM0),
            24 => Val(PGA_GAIN_A::GAIN2_INPUTVINM0FILTERINGVINM1),
            25 => Val(PGA_GAIN_A::GAIN4_INPUTVINM0FILTERINGVINM1),
            26 => Val(PGA_GAIN_A::GAIN8_INPUTVINM0FILTERINGVINM1),
            27 => Val(PGA_GAIN_A::GAIN16_INPUTVINM0FILTERINGVINM1),
            28 => Val(PGA_GAIN_A::GAIN32_INPUTVINM0FILTERINGVINM1),
            29 => Val(PGA_GAIN_A::GAIN64_INPUTVINM0FILTERINGVINM1),
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
    #[doc = "Checks if the value of the field is `GAIN32`"]
    #[inline(always)]
    pub fn is_gain32(&self) -> bool {
        *self == PGA_GAIN_A::GAIN32
    }
    #[doc = "Checks if the value of the field is `GAIN64`"]
    #[inline(always)]
    pub fn is_gain64(&self) -> bool {
        *self == PGA_GAIN_A::GAIN64
    }
    #[doc = "Checks if the value of the field is `GAIN2_INPUTVINM0`"]
    #[inline(always)]
    pub fn is_gain2_input_vinm0(&self) -> bool {
        *self == PGA_GAIN_A::GAIN2_INPUTVINM0
    }
    #[doc = "Checks if the value of the field is `GAIN4_INPUTVINM0`"]
    #[inline(always)]
    pub fn is_gain4_input_vinm0(&self) -> bool {
        *self == PGA_GAIN_A::GAIN4_INPUTVINM0
    }
    #[doc = "Checks if the value of the field is `GAIN8_INPUTVINM0`"]
    #[inline(always)]
    pub fn is_gain8_input_vinm0(&self) -> bool {
        *self == PGA_GAIN_A::GAIN8_INPUTVINM0
    }
    #[doc = "Checks if the value of the field is `GAIN16_INPUTVINM0`"]
    #[inline(always)]
    pub fn is_gain16_input_vinm0(&self) -> bool {
        *self == PGA_GAIN_A::GAIN16_INPUTVINM0
    }
    #[doc = "Checks if the value of the field is `GAIN32_INPUTVINM0`"]
    #[inline(always)]
    pub fn is_gain32_input_vinm0(&self) -> bool {
        *self == PGA_GAIN_A::GAIN32_INPUTVINM0
    }
    #[doc = "Checks if the value of the field is `GAIN64_INPUTVINM0`"]
    #[inline(always)]
    pub fn is_gain64_input_vinm0(&self) -> bool {
        *self == PGA_GAIN_A::GAIN64_INPUTVINM0
    }
    #[doc = "Checks if the value of the field is `GAIN2_FILTERINGVINM0`"]
    #[inline(always)]
    pub fn is_gain2_filtering_vinm0(&self) -> bool {
        *self == PGA_GAIN_A::GAIN2_FILTERINGVINM0
    }
    #[doc = "Checks if the value of the field is `GAIN4_FILTERINGVINM0`"]
    #[inline(always)]
    pub fn is_gain4_filtering_vinm0(&self) -> bool {
        *self == PGA_GAIN_A::GAIN4_FILTERINGVINM0
    }
    #[doc = "Checks if the value of the field is `GAIN8_FILTERINGVINM0`"]
    #[inline(always)]
    pub fn is_gain8_filtering_vinm0(&self) -> bool {
        *self == PGA_GAIN_A::GAIN8_FILTERINGVINM0
    }
    #[doc = "Checks if the value of the field is `GAIN16_FILTERINGVINM0`"]
    #[inline(always)]
    pub fn is_gain16_filtering_vinm0(&self) -> bool {
        *self == PGA_GAIN_A::GAIN16_FILTERINGVINM0
    }
    #[doc = "Checks if the value of the field is `GAIN32_FILTERINGVINM0`"]
    #[inline(always)]
    pub fn is_gain32_filtering_vinm0(&self) -> bool {
        *self == PGA_GAIN_A::GAIN32_FILTERINGVINM0
    }
    #[doc = "Checks if the value of the field is `GAIN64_FILTERINGVINM0`"]
    #[inline(always)]
    pub fn is_gain64_filtering_vinm0(&self) -> bool {
        *self == PGA_GAIN_A::GAIN64_FILTERINGVINM0
    }
    #[doc = "Checks if the value of the field is `GAIN2_INPUTVINM0FILTERINGVINM1`"]
    #[inline(always)]
    pub fn is_gain2_input_vinm0filtering_vinm1(&self) -> bool {
        *self == PGA_GAIN_A::GAIN2_INPUTVINM0FILTERINGVINM1
    }
    #[doc = "Checks if the value of the field is `GAIN4_INPUTVINM0FILTERINGVINM1`"]
    #[inline(always)]
    pub fn is_gain4_input_vinm0filtering_vinm1(&self) -> bool {
        *self == PGA_GAIN_A::GAIN4_INPUTVINM0FILTERINGVINM1
    }
    #[doc = "Checks if the value of the field is `GAIN8_INPUTVINM0FILTERINGVINM1`"]
    #[inline(always)]
    pub fn is_gain8_input_vinm0filtering_vinm1(&self) -> bool {
        *self == PGA_GAIN_A::GAIN8_INPUTVINM0FILTERINGVINM1
    }
    #[doc = "Checks if the value of the field is `GAIN16_INPUTVINM0FILTERINGVINM1`"]
    #[inline(always)]
    pub fn is_gain16_input_vinm0filtering_vinm1(&self) -> bool {
        *self == PGA_GAIN_A::GAIN16_INPUTVINM0FILTERINGVINM1
    }
    #[doc = "Checks if the value of the field is `GAIN32_INPUTVINM0FILTERINGVINM1`"]
    #[inline(always)]
    pub fn is_gain32_input_vinm0filtering_vinm1(&self) -> bool {
        *self == PGA_GAIN_A::GAIN32_INPUTVINM0FILTERINGVINM1
    }
    #[doc = "Checks if the value of the field is `GAIN64_INPUTVINM0FILTERINGVINM1`"]
    #[inline(always)]
    pub fn is_gain64_input_vinm0filtering_vinm1(&self) -> bool {
        *self == PGA_GAIN_A::GAIN64_INPUTVINM0FILTERINGVINM1
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
    #[doc = "Gain 32"]
    #[inline(always)]
    pub fn gain32(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN32)
    }
    #[doc = "Gain 64"]
    #[inline(always)]
    pub fn gain64(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN64)
    }
    #[doc = "Gain 2, input/bias connected to VINM0 or inverting gain"]
    #[inline(always)]
    pub fn gain2_input_vinm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN2_INPUTVINM0)
    }
    #[doc = "Gain 4, input/bias connected to VINM0 or inverting gain"]
    #[inline(always)]
    pub fn gain4_input_vinm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN4_INPUTVINM0)
    }
    #[doc = "Gain 8, input/bias connected to VINM0 or inverting gain"]
    #[inline(always)]
    pub fn gain8_input_vinm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN8_INPUTVINM0)
    }
    #[doc = "Gain 16, input/bias connected to VINM0 or inverting gain"]
    #[inline(always)]
    pub fn gain16_input_vinm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN16_INPUTVINM0)
    }
    #[doc = "Gain 32, input/bias connected to VINM0 or inverting gain"]
    #[inline(always)]
    pub fn gain32_input_vinm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN32_INPUTVINM0)
    }
    #[doc = "Gain 64, input/bias connected to VINM0 or inverting gain"]
    #[inline(always)]
    pub fn gain64_input_vinm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN64_INPUTVINM0)
    }
    #[doc = "Gain 2, with filtering on VINM0"]
    #[inline(always)]
    pub fn gain2_filtering_vinm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN2_FILTERINGVINM0)
    }
    #[doc = "Gain 4, with filtering on VINM0"]
    #[inline(always)]
    pub fn gain4_filtering_vinm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN4_FILTERINGVINM0)
    }
    #[doc = "Gain 8, with filtering on VINM0"]
    #[inline(always)]
    pub fn gain8_filtering_vinm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN8_FILTERINGVINM0)
    }
    #[doc = "Gain 16, with filtering on VINM0"]
    #[inline(always)]
    pub fn gain16_filtering_vinm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN16_FILTERINGVINM0)
    }
    #[doc = "Gain 32, with filtering on VINM0"]
    #[inline(always)]
    pub fn gain32_filtering_vinm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN32_FILTERINGVINM0)
    }
    #[doc = "Gain 64, with filtering on VINM0"]
    #[inline(always)]
    pub fn gain64_filtering_vinm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN64_FILTERINGVINM0)
    }
    #[doc = "Gain 2, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    #[inline(always)]
    pub fn gain2_input_vinm0filtering_vinm1(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN2_INPUTVINM0FILTERINGVINM1)
    }
    #[doc = "Gain 4, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    #[inline(always)]
    pub fn gain4_input_vinm0filtering_vinm1(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN4_INPUTVINM0FILTERINGVINM1)
    }
    #[doc = "Gain 8, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    #[inline(always)]
    pub fn gain8_input_vinm0filtering_vinm1(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN8_INPUTVINM0FILTERINGVINM1)
    }
    #[doc = "Gain 16, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    #[inline(always)]
    pub fn gain16_input_vinm0filtering_vinm1(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN16_INPUTVINM0FILTERINGVINM1)
    }
    #[doc = "Gain 32, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    #[inline(always)]
    pub fn gain32_input_vinm0filtering_vinm1(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN32_INPUTVINM0FILTERINGVINM1)
    }
    #[doc = "Gain 64, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
    #[inline(always)]
    pub fn gain64_input_vinm0filtering_vinm1(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::GAIN64_INPUTVINM0FILTERINGVINM1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 14)) | (((value as u32) & 0x1f) << 14);
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
#[doc = "Reader of field `CALOUT`"]
pub type CALOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CALOUT`"]
pub struct CALOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CALOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "LOCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "0: CSR is read-write"]
    READWRITE = 0,
    #[doc = "1: CSR is read-only, can only be cleared by system reset"]
    READONLY = 1,
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
            false => LOCK_A::READWRITE,
            true => LOCK_A::READONLY,
        }
    }
    #[doc = "Checks if the value of the field is `READWRITE`"]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        *self == LOCK_A::READWRITE
    }
    #[doc = "Checks if the value of the field is `READONLY`"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == LOCK_A::READONLY
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
    #[doc = "CSR is read-write"]
    #[inline(always)]
    pub fn read_write(self) -> &'a mut W {
        self.variant(LOCK_A::READWRITE)
    }
    #[doc = "CSR is read-only, can only be cleared by system reset"]
    #[inline(always)]
    pub fn read_only(self) -> &'a mut W {
        self.variant(LOCK_A::READONLY)
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
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    pub fn opaen(&self) -> OPAEN_R {
        OPAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FORCE_VP"]
    #[inline(always)]
    pub fn force_vp(&self) -> FORCE_VP_R {
        FORCE_VP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - VP_SEL"]
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - USERTRIM"]
    #[inline(always)]
    pub fn usertrim(&self) -> USERTRIM_R {
        USERTRIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - VM_SEL"]
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - OPAHSM"]
    #[inline(always)]
    pub fn opahsm(&self) -> OPAHSM_R {
        OPAHSM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - OPAINTOEN"]
    #[inline(always)]
    pub fn opaintoen(&self) -> OPAINTOEN_R {
        OPAINTOEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CALON"]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - CALSEL"]
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:18 - PGA_GAIN"]
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - TRIMOFFSETP"]
    #[inline(always)]
    pub fn trimoffsetp(&self) -> TRIMOFFSETP_R {
        TRIMOFFSETP_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - TRIMOFFSETN"]
    #[inline(always)]
    pub fn trimoffsetn(&self) -> TRIMOFFSETN_R {
        TRIMOFFSETN_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - CALOUT"]
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    pub fn opaen(&mut self) -> OPAEN_W {
        OPAEN_W { w: self }
    }
    #[doc = "Bit 1 - FORCE_VP"]
    #[inline(always)]
    pub fn force_vp(&mut self) -> FORCE_VP_W {
        FORCE_VP_W { w: self }
    }
    #[doc = "Bits 2:3 - VP_SEL"]
    #[inline(always)]
    pub fn vp_sel(&mut self) -> VP_SEL_W {
        VP_SEL_W { w: self }
    }
    #[doc = "Bit 4 - USERTRIM"]
    #[inline(always)]
    pub fn usertrim(&mut self) -> USERTRIM_W {
        USERTRIM_W { w: self }
    }
    #[doc = "Bits 5:6 - VM_SEL"]
    #[inline(always)]
    pub fn vm_sel(&mut self) -> VM_SEL_W {
        VM_SEL_W { w: self }
    }
    #[doc = "Bit 7 - OPAHSM"]
    #[inline(always)]
    pub fn opahsm(&mut self) -> OPAHSM_W {
        OPAHSM_W { w: self }
    }
    #[doc = "Bit 8 - OPAINTOEN"]
    #[inline(always)]
    pub fn opaintoen(&mut self) -> OPAINTOEN_W {
        OPAINTOEN_W { w: self }
    }
    #[doc = "Bit 11 - CALON"]
    #[inline(always)]
    pub fn calon(&mut self) -> CALON_W {
        CALON_W { w: self }
    }
    #[doc = "Bits 12:13 - CALSEL"]
    #[inline(always)]
    pub fn calsel(&mut self) -> CALSEL_W {
        CALSEL_W { w: self }
    }
    #[doc = "Bits 14:18 - PGA_GAIN"]
    #[inline(always)]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W {
        PGA_GAIN_W { w: self }
    }
    #[doc = "Bits 19:23 - TRIMOFFSETP"]
    #[inline(always)]
    pub fn trimoffsetp(&mut self) -> TRIMOFFSETP_W {
        TRIMOFFSETP_W { w: self }
    }
    #[doc = "Bits 24:28 - TRIMOFFSETN"]
    #[inline(always)]
    pub fn trimoffsetn(&mut self) -> TRIMOFFSETN_W {
        TRIMOFFSETN_W { w: self }
    }
    #[doc = "Bit 30 - CALOUT"]
    #[inline(always)]
    pub fn calout(&mut self) -> CALOUT_W {
        CALOUT_W { w: self }
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
}
