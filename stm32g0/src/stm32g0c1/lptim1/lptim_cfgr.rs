#[doc = "Register `LPTIM_CFGR` reader"]
pub struct R(crate::R<LPTIM_CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM_CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM_CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM_CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPTIM_CFGR` writer"]
pub struct W(crate::W<LPTIM_CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPTIM_CFGR_SPEC>;
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
impl From<crate::W<LPTIM_CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPTIM_CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock selector The CKSEL bit selects which clock source the LPTIM will use:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKSEL_A {
    #[doc = "0: LPTIM is clocked by internal clock source (APB clock or any of the embedded oscillators)"]
    B_0X0 = 0,
    #[doc = "1: LPTIM is clocked by an external clock source through the LPTIM external Input1"]
    B_0X1 = 1,
}
impl From<CKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKSEL` reader - Clock selector The CKSEL bit selects which clock source the LPTIM will use:"]
pub struct CKSEL_R(crate::FieldReader<bool, CKSEL_A>);
impl CKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKSEL_A {
        match self.bits {
            false => CKSEL_A::B_0X0,
            true => CKSEL_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CKSEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CKSEL_A::B_0X1
    }
}
impl core::ops::Deref for CKSEL_R {
    type Target = crate::FieldReader<bool, CKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKSEL` writer - Clock selector The CKSEL bit selects which clock source the LPTIM will use:"]
pub struct CKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LPTIM is clocked by internal clock source (APB clock or any of the embedded oscillators)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CKSEL_A::B_0X0)
    }
    #[doc = "LPTIM is clocked by an external clock source through the LPTIM external Input1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CKSEL_A::B_0X1)
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
#[doc = "Clock Polarity If LPTIM is clocked by an external clock source: When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKPOL_A {
    #[doc = "0: the rising edge is the active edge used for counting."]
    B_0X0 = 0,
    #[doc = "1: the falling edge is the active edge used for counting"]
    B_0X1 = 1,
    #[doc = "2: both edges are active edges. When both external clock signal edges are considered active ones, the LPTIM must also be clocked by an internal clock source with a frequency equal to at least four times the external clock frequency.If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 3 is active."]
    B_0X2 = 2,
    #[doc = "3: not allowed"]
    B_0X3 = 3,
}
impl From<CKPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: CKPOL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CKPOL` reader - Clock Polarity If LPTIM is clocked by an external clock source: When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes."]
pub struct CKPOL_R(crate::FieldReader<u8, CKPOL_A>);
impl CKPOL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKPOL_A {
        match self.bits {
            0 => CKPOL_A::B_0X0,
            1 => CKPOL_A::B_0X1,
            2 => CKPOL_A::B_0X2,
            3 => CKPOL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CKPOL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CKPOL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == CKPOL_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == CKPOL_A::B_0X3
    }
}
impl core::ops::Deref for CKPOL_R {
    type Target = crate::FieldReader<u8, CKPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKPOL` writer - Clock Polarity If LPTIM is clocked by an external clock source: When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes."]
pub struct CKPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKPOL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "the rising edge is the active edge used for counting."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CKPOL_A::B_0X0)
    }
    #[doc = "the falling edge is the active edge used for counting"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CKPOL_A::B_0X1)
    }
    #[doc = "both edges are active edges. When both external clock signal edges are considered active ones, the LPTIM must also be clocked by an internal clock source with a frequency equal to at least four times the external clock frequency.If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 3 is active."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(CKPOL_A::B_0X2)
    }
    #[doc = "not allowed"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(CKPOL_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKFLT_A {
    #[doc = "0: any external clock signal level change is considered as a valid transition"]
    B_0X0 = 0,
    #[doc = "1: external clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    B_0X1 = 1,
    #[doc = "2: external clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    B_0X2 = 2,
    #[doc = "3: external clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    B_0X3 = 3,
}
impl From<CKFLT_A> for u8 {
    #[inline(always)]
    fn from(variant: CKFLT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CKFLT` reader - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
pub struct CKFLT_R(crate::FieldReader<u8, CKFLT_A>);
impl CKFLT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKFLT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKFLT_A {
        match self.bits {
            0 => CKFLT_A::B_0X0,
            1 => CKFLT_A::B_0X1,
            2 => CKFLT_A::B_0X2,
            3 => CKFLT_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CKFLT_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CKFLT_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == CKFLT_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == CKFLT_A::B_0X3
    }
}
impl core::ops::Deref for CKFLT_R {
    type Target = crate::FieldReader<u8, CKFLT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKFLT` writer - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
pub struct CKFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> CKFLT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKFLT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "any external clock signal level change is considered as a valid transition"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CKFLT_A::B_0X0)
    }
    #[doc = "external clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CKFLT_A::B_0X1)
    }
    #[doc = "external clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(CKFLT_A::B_0X2)
    }
    #[doc = "external clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(CKFLT_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGFLT_A {
    #[doc = "0: any trigger active level change is considered as a valid trigger"]
    B_0X0 = 0,
    #[doc = "1: trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger."]
    B_0X1 = 1,
    #[doc = "2: trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger."]
    B_0X2 = 2,
    #[doc = "3: trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger."]
    B_0X3 = 3,
}
impl From<TRGFLT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGFLT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRGFLT` reader - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
pub struct TRGFLT_R(crate::FieldReader<u8, TRGFLT_A>);
impl TRGFLT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGFLT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGFLT_A {
        match self.bits {
            0 => TRGFLT_A::B_0X0,
            1 => TRGFLT_A::B_0X1,
            2 => TRGFLT_A::B_0X2,
            3 => TRGFLT_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TRGFLT_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TRGFLT_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == TRGFLT_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == TRGFLT_A::B_0X3
    }
}
impl core::ops::Deref for TRGFLT_R {
    type Target = crate::FieldReader<u8, TRGFLT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGFLT` writer - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
pub struct TRGFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGFLT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGFLT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "any trigger active level change is considered as a valid trigger"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TRGFLT_A::B_0X0)
    }
    #[doc = "trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TRGFLT_A::B_0X1)
    }
    #[doc = "trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(TRGFLT_A::B_0X2)
    }
    #[doc = "trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(TRGFLT_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: /1"]
    B_0X0 = 0,
    #[doc = "1: /2"]
    B_0X1 = 1,
    #[doc = "2: /4"]
    B_0X2 = 2,
    #[doc = "3: /8"]
    B_0X3 = 3,
    #[doc = "4: /16"]
    B_0X4 = 4,
    #[doc = "5: /32"]
    B_0X5 = 5,
    #[doc = "6: /64"]
    B_0X6 = 6,
    #[doc = "7: /128"]
    B_0X7 = 7,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRESC` reader - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:"]
pub struct PRESC_R(crate::FieldReader<u8, PRESC_A>);
impl PRESC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRESC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESC_A {
        match self.bits {
            0 => PRESC_A::B_0X0,
            1 => PRESC_A::B_0X1,
            2 => PRESC_A::B_0X2,
            3 => PRESC_A::B_0X3,
            4 => PRESC_A::B_0X4,
            5 => PRESC_A::B_0X5,
            6 => PRESC_A::B_0X6,
            7 => PRESC_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == PRESC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == PRESC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == PRESC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == PRESC_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == PRESC_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == PRESC_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == PRESC_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        **self == PRESC_A::B_0X7
    }
}
impl core::ops::Deref for PRESC_R {
    type Target = crate::FieldReader<u8, PRESC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESC` writer - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:"]
pub struct PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PRESC_A::B_0X0)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PRESC_A::B_0X1)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(PRESC_A::B_0X2)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(PRESC_A::B_0X3)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(PRESC_A::B_0X4)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(PRESC_A::B_0X5)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(PRESC_A::B_0X6)
    }
    #[doc = "/128"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(PRESC_A::B_0X7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Trigger selector The TRIGSEL bits select the trigger source that will serve as a trigger event for the LPTIM among the below 8 available sources: See for details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGSEL_A {
    #[doc = "0: lptim_ext_trig0"]
    B_0X0 = 0,
    #[doc = "1: lptim_ext_trig1"]
    B_0X1 = 1,
    #[doc = "2: lptim_ext_trig2"]
    B_0X2 = 2,
    #[doc = "3: lptim_ext_trig3"]
    B_0X3 = 3,
    #[doc = "4: lptim_ext_trig4"]
    B_0X4 = 4,
    #[doc = "5: lptim_ext_trig5"]
    B_0X5 = 5,
    #[doc = "6: lptim_ext_trig6"]
    B_0X6 = 6,
    #[doc = "7: lptim_ext_trig7"]
    B_0X7 = 7,
}
impl From<TRIGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRIGSEL` reader - Trigger selector The TRIGSEL bits select the trigger source that will serve as a trigger event for the LPTIM among the below 8 available sources: See for details."]
pub struct TRIGSEL_R(crate::FieldReader<u8, TRIGSEL_A>);
impl TRIGSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIGSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGSEL_A {
        match self.bits {
            0 => TRIGSEL_A::B_0X0,
            1 => TRIGSEL_A::B_0X1,
            2 => TRIGSEL_A::B_0X2,
            3 => TRIGSEL_A::B_0X3,
            4 => TRIGSEL_A::B_0X4,
            5 => TRIGSEL_A::B_0X5,
            6 => TRIGSEL_A::B_0X6,
            7 => TRIGSEL_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TRIGSEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TRIGSEL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == TRIGSEL_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == TRIGSEL_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == TRIGSEL_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == TRIGSEL_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == TRIGSEL_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        **self == TRIGSEL_A::B_0X7
    }
}
impl core::ops::Deref for TRIGSEL_R {
    type Target = crate::FieldReader<u8, TRIGSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGSEL` writer - Trigger selector The TRIGSEL bits select the trigger source that will serve as a trigger event for the LPTIM among the below 8 available sources: See for details."]
pub struct TRIGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "lptim_ext_trig0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TRIGSEL_A::B_0X0)
    }
    #[doc = "lptim_ext_trig1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TRIGSEL_A::B_0X1)
    }
    #[doc = "lptim_ext_trig2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(TRIGSEL_A::B_0X2)
    }
    #[doc = "lptim_ext_trig3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(TRIGSEL_A::B_0X3)
    }
    #[doc = "lptim_ext_trig4"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(TRIGSEL_A::B_0X4)
    }
    #[doc = "lptim_ext_trig5"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(TRIGSEL_A::B_0X5)
    }
    #[doc = "lptim_ext_trig6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(TRIGSEL_A::B_0X6)
    }
    #[doc = "lptim_ext_trig7"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(TRIGSEL_A::B_0X7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
#[doc = "Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGEN_A {
    #[doc = "0: software trigger (counting start is initiated by software)"]
    B_0X0 = 0,
    #[doc = "1: rising edge is the active edge"]
    B_0X1 = 1,
    #[doc = "2: falling edge is the active edge"]
    B_0X2 = 2,
    #[doc = "3: both edges are active edges"]
    B_0X3 = 3,
}
impl From<TRIGEN_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRIGEN` reader - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:"]
pub struct TRIGEN_R(crate::FieldReader<u8, TRIGEN_A>);
impl TRIGEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGEN_A {
        match self.bits {
            0 => TRIGEN_A::B_0X0,
            1 => TRIGEN_A::B_0X1,
            2 => TRIGEN_A::B_0X2,
            3 => TRIGEN_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TRIGEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TRIGEN_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == TRIGEN_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == TRIGEN_A::B_0X3
    }
}
impl core::ops::Deref for TRIGEN_R {
    type Target = crate::FieldReader<u8, TRIGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGEN` writer - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:"]
pub struct TRIGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGEN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "software trigger (counting start is initiated by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TRIGEN_A::B_0X0)
    }
    #[doc = "rising edge is the active edge"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TRIGEN_A::B_0X1)
    }
    #[doc = "falling edge is the active edge"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(TRIGEN_A::B_0X2)
    }
    #[doc = "both edges are active edges"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(TRIGEN_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
#[doc = "Timeout enable The TIMOUT bit controls the Timeout feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMOUT_A {
    #[doc = "0: A trigger event arriving when the timer is already started will be ignored"]
    B_0X0 = 0,
    #[doc = "1: A trigger event arriving when the timer is already started will reset and restart the counter"]
    B_0X1 = 1,
}
impl From<TIMOUT_A> for bool {
    #[inline(always)]
    fn from(variant: TIMOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMOUT` reader - Timeout enable The TIMOUT bit controls the Timeout feature"]
pub struct TIMOUT_R(crate::FieldReader<bool, TIMOUT_A>);
impl TIMOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMOUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMOUT_A {
        match self.bits {
            false => TIMOUT_A::B_0X0,
            true => TIMOUT_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TIMOUT_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TIMOUT_A::B_0X1
    }
}
impl core::ops::Deref for TIMOUT_R {
    type Target = crate::FieldReader<bool, TIMOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMOUT` writer - Timeout enable The TIMOUT bit controls the Timeout feature"]
pub struct TIMOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMOUT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A trigger event arriving when the timer is already started will be ignored"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIMOUT_A::B_0X0)
    }
    #[doc = "A trigger event arriving when the timer is already started will reset and restart the counter"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIMOUT_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Waveform shape The WAVE bit controls the output shape\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAVE_A {
    #[doc = "0: Deactivate Set-once mode, PWM or One Pulse waveform depending on how the timer was started, CNTSTRT for PWM or SNGSTRT for One Pulse waveform."]
    B_0X0 = 0,
    #[doc = "1: Activate the Set-once mode"]
    B_0X1 = 1,
}
impl From<WAVE_A> for bool {
    #[inline(always)]
    fn from(variant: WAVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAVE` reader - Waveform shape The WAVE bit controls the output shape"]
pub struct WAVE_R(crate::FieldReader<bool, WAVE_A>);
impl WAVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAVE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAVE_A {
        match self.bits {
            false => WAVE_A::B_0X0,
            true => WAVE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == WAVE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == WAVE_A::B_0X1
    }
}
impl core::ops::Deref for WAVE_R {
    type Target = crate::FieldReader<bool, WAVE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAVE` writer - Waveform shape The WAVE bit controls the output shape"]
pub struct WAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAVE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Deactivate Set-once mode, PWM or One Pulse waveform depending on how the timer was started, CNTSTRT for PWM or SNGSTRT for One Pulse waveform."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WAVE_A::B_0X0)
    }
    #[doc = "Activate the Set-once mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WAVE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Waveform shape polarity The WAVEPOL bit controls the output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAVPOL_A {
    #[doc = "0: The LPTIM output reflects the compare results between LPTIM_CNT and LPTIM_CMP registers"]
    B_0X0 = 0,
    #[doc = "1: The LPTIM output reflects the inverse of the compare results between LPTIM_CNT and LPTIM_CMP registers"]
    B_0X1 = 1,
}
impl From<WAVPOL_A> for bool {
    #[inline(always)]
    fn from(variant: WAVPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAVPOL` reader - Waveform shape polarity The WAVEPOL bit controls the output polarity"]
pub struct WAVPOL_R(crate::FieldReader<bool, WAVPOL_A>);
impl WAVPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAVPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAVPOL_A {
        match self.bits {
            false => WAVPOL_A::B_0X0,
            true => WAVPOL_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == WAVPOL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == WAVPOL_A::B_0X1
    }
}
impl core::ops::Deref for WAVPOL_R {
    type Target = crate::FieldReader<bool, WAVPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAVPOL` writer - Waveform shape polarity The WAVEPOL bit controls the output polarity"]
pub struct WAVPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAVPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The LPTIM output reflects the compare results between LPTIM_CNT and LPTIM_CMP registers"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WAVPOL_A::B_0X0)
    }
    #[doc = "The LPTIM output reflects the inverse of the compare results between LPTIM_CNT and LPTIM_CMP registers"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WAVPOL_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Registers update mode The PRELOAD bit controls the LPTIM_ARR and the LPTIM_CMP registers update modality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRELOAD_A {
    #[doc = "0: Registers are updated after each APB bus write access"]
    B_0X0 = 0,
    #[doc = "1: Registers are updated at the end of the current LPTIM period"]
    B_0X1 = 1,
}
impl From<PRELOAD_A> for bool {
    #[inline(always)]
    fn from(variant: PRELOAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRELOAD` reader - Registers update mode The PRELOAD bit controls the LPTIM_ARR and the LPTIM_CMP registers update modality"]
pub struct PRELOAD_R(crate::FieldReader<bool, PRELOAD_A>);
impl PRELOAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRELOAD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRELOAD_A {
        match self.bits {
            false => PRELOAD_A::B_0X0,
            true => PRELOAD_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == PRELOAD_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == PRELOAD_A::B_0X1
    }
}
impl core::ops::Deref for PRELOAD_R {
    type Target = crate::FieldReader<bool, PRELOAD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRELOAD` writer - Registers update mode The PRELOAD bit controls the LPTIM_ARR and the LPTIM_CMP registers update modality"]
pub struct PRELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PRELOAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRELOAD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Registers are updated after each APB bus write access"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PRELOAD_A::B_0X0)
    }
    #[doc = "Registers are updated at the end of the current LPTIM period"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PRELOAD_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COUNTMODE_A {
    #[doc = "0: the counter is incremented following each internal clock pulse"]
    B_0X0 = 0,
    #[doc = "1: the counter is incremented following each valid clock pulse on the LPTIM external Input1"]
    B_0X1 = 1,
}
impl From<COUNTMODE_A> for bool {
    #[inline(always)]
    fn from(variant: COUNTMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COUNTMODE` reader - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:"]
pub struct COUNTMODE_R(crate::FieldReader<bool, COUNTMODE_A>);
impl COUNTMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COUNTMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COUNTMODE_A {
        match self.bits {
            false => COUNTMODE_A::B_0X0,
            true => COUNTMODE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == COUNTMODE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == COUNTMODE_A::B_0X1
    }
}
impl core::ops::Deref for COUNTMODE_R {
    type Target = crate::FieldReader<bool, COUNTMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNTMODE` writer - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:"]
pub struct COUNTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COUNTMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "the counter is incremented following each internal clock pulse"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(COUNTMODE_A::B_0X0)
    }
    #[doc = "the counter is incremented following each valid clock pulse on the LPTIM external Input1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(COUNTMODE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENC_A {
    #[doc = "0: Encoder mode disabled"]
    B_0X0 = 0,
    #[doc = "1: Encoder mode enabled"]
    B_0X1 = 1,
}
impl From<ENC_A> for bool {
    #[inline(always)]
    fn from(variant: ENC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENC` reader - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub struct ENC_R(crate::FieldReader<bool, ENC_A>);
impl ENC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENC_A {
        match self.bits {
            false => ENC_A::B_0X0,
            true => ENC_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ENC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ENC_A::B_0X1
    }
}
impl core::ops::Deref for ENC_R {
    type Target = crate::FieldReader<bool, ENC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENC` writer - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub struct ENC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Encoder mode disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ENC_A::B_0X0)
    }
    #[doc = "Encoder mode enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ENC_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clock selector The CKSEL bit selects which clock source the LPTIM will use:"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Clock Polarity If LPTIM is clocked by an external clock source: When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes."]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
    #[inline(always)]
    pub fn ckflt(&self) -> CKFLT_R {
        CKFLT_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
    #[inline(always)]
    pub fn trgflt(&self) -> TRGFLT_R {
        TRGFLT_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 9:11 - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 13:15 - Trigger selector The TRIGSEL bits select the trigger source that will serve as a trigger event for the LPTIM among the below 8 available sources: See for details."]
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 17:18 - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:"]
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 19 - Timeout enable The TIMOUT bit controls the Timeout feature"]
    #[inline(always)]
    pub fn timout(&self) -> TIMOUT_R {
        TIMOUT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Waveform shape The WAVE bit controls the output shape"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Waveform shape polarity The WAVEPOL bit controls the output polarity"]
    #[inline(always)]
    pub fn wavpol(&self) -> WAVPOL_R {
        WAVPOL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Registers update mode The PRELOAD bit controls the LPTIM_ARR and the LPTIM_CMP registers update modality"]
    #[inline(always)]
    pub fn preload(&self) -> PRELOAD_R {
        PRELOAD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:"]
    #[inline(always)]
    pub fn countmode(&self) -> COUNTMODE_R {
        COUNTMODE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock selector The CKSEL bit selects which clock source the LPTIM will use:"]
    #[inline(always)]
    pub fn cksel(&mut self) -> CKSEL_W {
        CKSEL_W { w: self }
    }
    #[doc = "Bits 1:2 - Clock Polarity If LPTIM is clocked by an external clock source: When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes."]
    #[inline(always)]
    pub fn ckpol(&mut self) -> CKPOL_W {
        CKPOL_W { w: self }
    }
    #[doc = "Bits 3:4 - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
    #[inline(always)]
    pub fn ckflt(&mut self) -> CKFLT_W {
        CKFLT_W { w: self }
    }
    #[doc = "Bits 6:7 - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
    #[inline(always)]
    pub fn trgflt(&mut self) -> TRGFLT_W {
        TRGFLT_W { w: self }
    }
    #[doc = "Bits 9:11 - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W { w: self }
    }
    #[doc = "Bits 13:15 - Trigger selector The TRIGSEL bits select the trigger source that will serve as a trigger event for the LPTIM among the below 8 available sources: See for details."]
    #[inline(always)]
    pub fn trigsel(&mut self) -> TRIGSEL_W {
        TRIGSEL_W { w: self }
    }
    #[doc = "Bits 17:18 - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:"]
    #[inline(always)]
    pub fn trigen(&mut self) -> TRIGEN_W {
        TRIGEN_W { w: self }
    }
    #[doc = "Bit 19 - Timeout enable The TIMOUT bit controls the Timeout feature"]
    #[inline(always)]
    pub fn timout(&mut self) -> TIMOUT_W {
        TIMOUT_W { w: self }
    }
    #[doc = "Bit 20 - Waveform shape The WAVE bit controls the output shape"]
    #[inline(always)]
    pub fn wave(&mut self) -> WAVE_W {
        WAVE_W { w: self }
    }
    #[doc = "Bit 21 - Waveform shape polarity The WAVEPOL bit controls the output polarity"]
    #[inline(always)]
    pub fn wavpol(&mut self) -> WAVPOL_W {
        WAVPOL_W { w: self }
    }
    #[doc = "Bit 22 - Registers update mode The PRELOAD bit controls the LPTIM_ARR and the LPTIM_CMP registers update modality"]
    #[inline(always)]
    pub fn preload(&mut self) -> PRELOAD_W {
        PRELOAD_W { w: self }
    }
    #[doc = "Bit 23 - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:"]
    #[inline(always)]
    pub fn countmode(&mut self) -> COUNTMODE_W {
        COUNTMODE_W { w: self }
    }
    #[doc = "Bit 24 - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn enc(&mut self) -> ENC_W {
        ENC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_cfgr](index.html) module"]
pub struct LPTIM_CFGR_SPEC;
impl crate::RegisterSpec for LPTIM_CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lptim_cfgr::R](R) reader structure"]
impl crate::Readable for LPTIM_CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lptim_cfgr::W](W) writer structure"]
impl crate::Writable for LPTIM_CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPTIM_CFGR to value 0"]
impl crate::Resettable for LPTIM_CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
