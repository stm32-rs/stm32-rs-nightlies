#[doc = "Register `TIMBISR` reader"]
pub struct R(crate::R<TIMBISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMBISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMBISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMBISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Output 2 Copy"]
pub type O2CPY_A = O1CPY_A;
#[doc = "Field `O2CPY` reader - Output 2 Copy"]
pub type O2CPY_R = O1CPY_R;
#[doc = "Output 1 Copy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O1CPY_A {
    #[doc = "0: Output is inactive"]
    INACTIVE = 0,
    #[doc = "1: Output is active"]
    ACTIVE = 1,
}
impl From<O1CPY_A> for bool {
    #[inline(always)]
    fn from(variant: O1CPY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `O1CPY` reader - Output 1 Copy"]
pub struct O1CPY_R(crate::FieldReader<bool, O1CPY_A>);
impl O1CPY_R {
    pub(crate) fn new(bits: bool) -> Self {
        O1CPY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> O1CPY_A {
        match self.bits {
            false => O1CPY_A::INACTIVE,
            true => O1CPY_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == O1CPY_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == O1CPY_A::ACTIVE
    }
}
impl core::ops::Deref for O1CPY_R {
    type Target = crate::FieldReader<bool, O1CPY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Output 2 State"]
pub type O2STAT_A = O1STAT_A;
#[doc = "Field `O2STAT` reader - Output 2 State"]
pub type O2STAT_R = O1STAT_R;
#[doc = "Output 1 State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O1STAT_A {
    #[doc = "0: Output was inactive"]
    INACTIVE = 0,
    #[doc = "1: Output was active"]
    ACTIVE = 1,
}
impl From<O1STAT_A> for bool {
    #[inline(always)]
    fn from(variant: O1STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `O1STAT` reader - Output 1 State"]
pub struct O1STAT_R(crate::FieldReader<bool, O1STAT_A>);
impl O1STAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        O1STAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> O1STAT_A {
        match self.bits {
            false => O1STAT_A::INACTIVE,
            true => O1STAT_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == O1STAT_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == O1STAT_A::ACTIVE
    }
}
impl core::ops::Deref for O1STAT_R {
    type Target = crate::FieldReader<bool, O1STAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Idle Push Pull Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPPSTAT_A {
    #[doc = "0: Protection occurred when the output 1 was active and output 2 forced inactive"]
    OUTPUT1ACTIVE = 0,
    #[doc = "1: Protection occurred when the output 2 was active and output 1 forced inactive"]
    OUTPUT2ACTIVE = 1,
}
impl From<IPPSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: IPPSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IPPSTAT` reader - Idle Push Pull Status"]
pub struct IPPSTAT_R(crate::FieldReader<bool, IPPSTAT_A>);
impl IPPSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPPSTAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPPSTAT_A {
        match self.bits {
            false => IPPSTAT_A::OUTPUT1ACTIVE,
            true => IPPSTAT_A::OUTPUT2ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUT1ACTIVE`"]
    #[inline(always)]
    pub fn is_output1active(&self) -> bool {
        **self == IPPSTAT_A::OUTPUT1ACTIVE
    }
    #[doc = "Checks if the value of the field is `OUTPUT2ACTIVE`"]
    #[inline(always)]
    pub fn is_output2active(&self) -> bool {
        **self == IPPSTAT_A::OUTPUT2ACTIVE
    }
}
impl core::ops::Deref for IPPSTAT_R {
    type Target = crate::FieldReader<bool, IPPSTAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Current Push Pull Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPPSTAT_A {
    #[doc = "0: Signal applied on output 1 and output 2 forced inactive"]
    OUTPUT1ACTIVE = 0,
    #[doc = "1: Signal applied on output 2 and output 1 forced inactive"]
    OUTPUT2ACTIVE = 1,
}
impl From<CPPSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: CPPSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPPSTAT` reader - Current Push Pull Status"]
pub struct CPPSTAT_R(crate::FieldReader<bool, CPPSTAT_A>);
impl CPPSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPPSTAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPPSTAT_A {
        match self.bits {
            false => CPPSTAT_A::OUTPUT1ACTIVE,
            true => CPPSTAT_A::OUTPUT2ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUT1ACTIVE`"]
    #[inline(always)]
    pub fn is_output1active(&self) -> bool {
        **self == CPPSTAT_A::OUTPUT1ACTIVE
    }
    #[doc = "Checks if the value of the field is `OUTPUT2ACTIVE`"]
    #[inline(always)]
    pub fn is_output2active(&self) -> bool {
        **self == CPPSTAT_A::OUTPUT2ACTIVE
    }
}
impl core::ops::Deref for CPPSTAT_R {
    type Target = crate::FieldReader<bool, CPPSTAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Delayed Protection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLYPRT_A {
    #[doc = "0: Not in delayed idle or balanced idle mode"]
    INACTIVE = 0,
    #[doc = "1: Delayed idle or balanced idle mode entry"]
    ACTIVE = 1,
}
impl From<DLYPRT_A> for bool {
    #[inline(always)]
    fn from(variant: DLYPRT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLYPRT` reader - Delayed Protection Flag"]
pub struct DLYPRT_R(crate::FieldReader<bool, DLYPRT_A>);
impl DLYPRT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DLYPRT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLYPRT_A {
        match self.bits {
            false => DLYPRT_A::INACTIVE,
            true => DLYPRT_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == DLYPRT_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == DLYPRT_A::ACTIVE
    }
}
impl core::ops::Deref for DLYPRT_R {
    type Target = crate::FieldReader<bool, DLYPRT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Reset Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RST_A {
    #[doc = "0: No TIMx counter reset/roll-over interrupt occurred"]
    NOEVENT = 0,
    #[doc = "1: TIMx counter reset/roll-over interrupt occurred"]
    EVENT = 1,
}
impl From<RST_A> for bool {
    #[inline(always)]
    fn from(variant: RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST` reader - Reset Interrupt Flag"]
pub struct RST_R(crate::FieldReader<bool, RST_A>);
impl RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RST_A {
        match self.bits {
            false => RST_A::NOEVENT,
            true => RST_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == RST_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == RST_A::EVENT
    }
}
impl core::ops::Deref for RST_R {
    type Target = crate::FieldReader<bool, RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Output 2 Reset Interrupt Flag"]
pub type RSTX2_A = RSTX1_A;
#[doc = "Field `RSTx2` reader - Output 2 Reset Interrupt Flag"]
pub type RSTX2_R = RSTX1_R;
#[doc = "Output 2 Set Interrupt Flag"]
pub type SETX2_A = SETX1_A;
#[doc = "Field `SETx2` reader - Output 2 Set Interrupt Flag"]
pub type SETX2_R = SETX1_R;
#[doc = "Output 1 Reset Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTX1_A {
    #[doc = "0: No Tx output reset interrupt occurred"]
    NOEVENT = 0,
    #[doc = "1: Tx output reset interrupt occurred"]
    EVENT = 1,
}
impl From<RSTX1_A> for bool {
    #[inline(always)]
    fn from(variant: RSTX1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTx1` reader - Output 1 Reset Interrupt Flag"]
pub struct RSTX1_R(crate::FieldReader<bool, RSTX1_A>);
impl RSTX1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTX1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTX1_A {
        match self.bits {
            false => RSTX1_A::NOEVENT,
            true => RSTX1_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == RSTX1_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == RSTX1_A::EVENT
    }
}
impl core::ops::Deref for RSTX1_R {
    type Target = crate::FieldReader<bool, RSTX1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Output 1 Set Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETX1_A {
    #[doc = "0: No Tx output set interrupt occurred"]
    NOEVENT = 0,
    #[doc = "1: Tx output set interrupt occurred"]
    EVENT = 1,
}
impl From<SETX1_A> for bool {
    #[inline(always)]
    fn from(variant: SETX1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETx1` reader - Output 1 Set Interrupt Flag"]
pub struct SETX1_R(crate::FieldReader<bool, SETX1_A>);
impl SETX1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETX1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SETX1_A {
        match self.bits {
            false => SETX1_A::NOEVENT,
            true => SETX1_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == SETX1_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == SETX1_A::EVENT
    }
}
impl core::ops::Deref for SETX1_R {
    type Target = crate::FieldReader<bool, SETX1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Capture2 Interrupt Flag"]
pub type CPT2_A = CPT1_A;
#[doc = "Field `CPT2` reader - Capture2 Interrupt Flag"]
pub type CPT2_R = CPT1_R;
#[doc = "Capture1 Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPT1_A {
    #[doc = "0: No timer x capture reset interrupt occurred"]
    NOEVENT = 0,
    #[doc = "1: Timer x capture reset interrupt occurred"]
    EVENT = 1,
}
impl From<CPT1_A> for bool {
    #[inline(always)]
    fn from(variant: CPT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPT1` reader - Capture1 Interrupt Flag"]
pub struct CPT1_R(crate::FieldReader<bool, CPT1_A>);
impl CPT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPT1_A {
        match self.bits {
            false => CPT1_A::NOEVENT,
            true => CPT1_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == CPT1_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == CPT1_A::EVENT
    }
}
impl core::ops::Deref for CPT1_R {
    type Target = crate::FieldReader<bool, CPT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Update Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPD_A {
    #[doc = "0: No timer update interrupt occurred"]
    NOEVENT = 0,
    #[doc = "1: Timer update interrupt occurred"]
    EVENT = 1,
}
impl From<UPD_A> for bool {
    #[inline(always)]
    fn from(variant: UPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPD` reader - Update Interrupt Flag"]
pub struct UPD_R(crate::FieldReader<bool, UPD_A>);
impl UPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        UPD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPD_A {
        match self.bits {
            false => UPD_A::NOEVENT,
            true => UPD_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == UPD_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == UPD_A::EVENT
    }
}
impl core::ops::Deref for UPD_R {
    type Target = crate::FieldReader<bool, UPD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Repetition Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REP_A {
    #[doc = "0: No timer repetition interrupt occurred"]
    NOEVENT = 0,
    #[doc = "1: Timer repetition interrupt occurred"]
    EVENT = 1,
}
impl From<REP_A> for bool {
    #[inline(always)]
    fn from(variant: REP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REP` reader - Repetition Interrupt Flag"]
pub struct REP_R(crate::FieldReader<bool, REP_A>);
impl REP_R {
    pub(crate) fn new(bits: bool) -> Self {
        REP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REP_A {
        match self.bits {
            false => REP_A::NOEVENT,
            true => REP_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == REP_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == REP_A::EVENT
    }
}
impl core::ops::Deref for REP_R {
    type Target = crate::FieldReader<bool, REP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Compare 4 Interrupt Flag"]
pub type CMP4_A = CMP1_A;
#[doc = "Field `CMP4` reader - Compare 4 Interrupt Flag"]
pub type CMP4_R = CMP1_R;
#[doc = "Compare 3 Interrupt Flag"]
pub type CMP3_A = CMP1_A;
#[doc = "Field `CMP3` reader - Compare 3 Interrupt Flag"]
pub type CMP3_R = CMP1_R;
#[doc = "Compare 2 Interrupt Flag"]
pub type CMP2_A = CMP1_A;
#[doc = "Field `CMP2` reader - Compare 2 Interrupt Flag"]
pub type CMP2_R = CMP1_R;
#[doc = "Compare 1 Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP1_A {
    #[doc = "0: No compare interrupt occurred"]
    NOEVENT = 0,
    #[doc = "1: Compare interrupt occurred"]
    EVENT = 1,
}
impl From<CMP1_A> for bool {
    #[inline(always)]
    fn from(variant: CMP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP1` reader - Compare 1 Interrupt Flag"]
pub struct CMP1_R(crate::FieldReader<bool, CMP1_A>);
impl CMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP1_A {
        match self.bits {
            false => CMP1_A::NOEVENT,
            true => CMP1_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == CMP1_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == CMP1_A::EVENT
    }
}
impl core::ops::Deref for CMP1_R {
    type Target = crate::FieldReader<bool, CMP1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 21 - Output 2 Copy"]
    #[inline(always)]
    pub fn o2cpy(&self) -> O2CPY_R {
        O2CPY_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Output 1 Copy"]
    #[inline(always)]
    pub fn o1cpy(&self) -> O1CPY_R {
        O1CPY_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Output 2 State"]
    #[inline(always)]
    pub fn o2stat(&self) -> O2STAT_R {
        O2STAT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Output 1 State"]
    #[inline(always)]
    pub fn o1stat(&self) -> O1STAT_R {
        O1STAT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Idle Push Pull Status"]
    #[inline(always)]
    pub fn ippstat(&self) -> IPPSTAT_R {
        IPPSTAT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Current Push Pull Status"]
    #[inline(always)]
    pub fn cppstat(&self) -> CPPSTAT_R {
        CPPSTAT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Delayed Protection Flag"]
    #[inline(always)]
    pub fn dlyprt(&self) -> DLYPRT_R {
        DLYPRT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Output 2 Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rstx2(&self) -> RSTX2_R {
        RSTX2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Output 2 Set Interrupt Flag"]
    #[inline(always)]
    pub fn setx2(&self) -> SETX2_R {
        SETX2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Output 1 Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rstx1(&self) -> RSTX1_R {
        RSTX1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Output 1 Set Interrupt Flag"]
    #[inline(always)]
    pub fn setx1(&self) -> SETX1_R {
        SETX1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Capture2 Interrupt Flag"]
    #[inline(always)]
    pub fn cpt2(&self) -> CPT2_R {
        CPT2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Capture1 Interrupt Flag"]
    #[inline(always)]
    pub fn cpt1(&self) -> CPT1_R {
        CPT1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Update Interrupt Flag"]
    #[inline(always)]
    pub fn upd(&self) -> UPD_R {
        UPD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Repetition Interrupt Flag"]
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Compare 4 Interrupt Flag"]
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Compare 3 Interrupt Flag"]
    #[inline(always)]
    pub fn cmp3(&self) -> CMP3_R {
        CMP3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Compare 2 Interrupt Flag"]
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Compare 1 Interrupt Flag"]
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Timerx Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timbisr](index.html) module"]
pub struct TIMBISR_SPEC;
impl crate::RegisterSpec for TIMBISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timbisr::R](R) reader structure"]
impl crate::Readable for TIMBISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMBISR to value 0"]
impl crate::Resettable for TIMBISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
