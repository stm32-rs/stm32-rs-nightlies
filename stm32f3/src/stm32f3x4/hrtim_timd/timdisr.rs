#[doc = "Reader of register TIMDISR"]
pub type R = crate::R<u32, super::TIMDISR>;
#[doc = "Output 2 Copy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O2CPY_A {
    #[doc = "0: Output is inactive"]
    INACTIVE = 0,
    #[doc = "1: Output is active"]
    ACTIVE = 1,
}
impl From<O2CPY_A> for bool {
    #[inline(always)]
    fn from(variant: O2CPY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `O2CPY`"]
pub type O2CPY_R = crate::R<bool, O2CPY_A>;
impl O2CPY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> O2CPY_A {
        match self.bits {
            false => O2CPY_A::INACTIVE,
            true => O2CPY_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == O2CPY_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == O2CPY_A::ACTIVE
    }
}
#[doc = "Output 1 Copy"]
pub type O1CPY_A = O2CPY_A;
#[doc = "Reader of field `O1CPY`"]
pub type O1CPY_R = crate::R<bool, O2CPY_A>;
#[doc = "Output 2 State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O2STAT_A {
    #[doc = "0: Output was inactive"]
    INACTIVE = 0,
    #[doc = "1: Output was active"]
    ACTIVE = 1,
}
impl From<O2STAT_A> for bool {
    #[inline(always)]
    fn from(variant: O2STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `O2STAT`"]
pub type O2STAT_R = crate::R<bool, O2STAT_A>;
impl O2STAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> O2STAT_A {
        match self.bits {
            false => O2STAT_A::INACTIVE,
            true => O2STAT_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == O2STAT_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == O2STAT_A::ACTIVE
    }
}
#[doc = "Output 1 State"]
pub type O1STAT_A = O2STAT_A;
#[doc = "Reader of field `O1STAT`"]
pub type O1STAT_R = crate::R<bool, O2STAT_A>;
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
#[doc = "Reader of field `IPPSTAT`"]
pub type IPPSTAT_R = crate::R<bool, IPPSTAT_A>;
impl IPPSTAT_R {
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
        *self == IPPSTAT_A::OUTPUT1ACTIVE
    }
    #[doc = "Checks if the value of the field is `OUTPUT2ACTIVE`"]
    #[inline(always)]
    pub fn is_output2active(&self) -> bool {
        *self == IPPSTAT_A::OUTPUT2ACTIVE
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
#[doc = "Reader of field `CPPSTAT`"]
pub type CPPSTAT_R = crate::R<bool, CPPSTAT_A>;
impl CPPSTAT_R {
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
        *self == CPPSTAT_A::OUTPUT1ACTIVE
    }
    #[doc = "Checks if the value of the field is `OUTPUT2ACTIVE`"]
    #[inline(always)]
    pub fn is_output2active(&self) -> bool {
        *self == CPPSTAT_A::OUTPUT2ACTIVE
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
#[doc = "Reader of field `DLYPRT`"]
pub type DLYPRT_R = crate::R<bool, DLYPRT_A>;
impl DLYPRT_R {
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
        *self == DLYPRT_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == DLYPRT_A::ACTIVE
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
#[doc = "Reader of field `RST`"]
pub type RST_R = crate::R<bool, RST_A>;
impl RST_R {
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
        *self == RST_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == RST_A::EVENT
    }
}
#[doc = "Output 2 Reset Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTX2_A {
    #[doc = "0: No Tx output reset interrupt occurred"]
    NOEVENT = 0,
    #[doc = "1: Tx output reset interrupt occurred"]
    EVENT = 1,
}
impl From<RSTX2_A> for bool {
    #[inline(always)]
    fn from(variant: RSTX2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSTx2`"]
pub type RSTX2_R = crate::R<bool, RSTX2_A>;
impl RSTX2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTX2_A {
        match self.bits {
            false => RSTX2_A::NOEVENT,
            true => RSTX2_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == RSTX2_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == RSTX2_A::EVENT
    }
}
#[doc = "Output 2 Set Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETX2_A {
    #[doc = "0: No Tx output set interrupt occurred"]
    NOEVENT = 0,
    #[doc = "1: Tx output set interrupt occurred"]
    EVENT = 1,
}
impl From<SETX2_A> for bool {
    #[inline(always)]
    fn from(variant: SETX2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SETx2`"]
pub type SETX2_R = crate::R<bool, SETX2_A>;
impl SETX2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SETX2_A {
        match self.bits {
            false => SETX2_A::NOEVENT,
            true => SETX2_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == SETX2_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == SETX2_A::EVENT
    }
}
#[doc = "Output 1 Reset Interrupt Flag"]
pub type RSTX1_A = RSTX2_A;
#[doc = "Reader of field `RSTx1`"]
pub type RSTX1_R = crate::R<bool, RSTX2_A>;
#[doc = "Output 1 Set Interrupt Flag"]
pub type SETX1_A = SETX2_A;
#[doc = "Reader of field `SETx1`"]
pub type SETX1_R = crate::R<bool, SETX2_A>;
#[doc = "Capture2 Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPT2_A {
    #[doc = "0: No timer x capture reset interrupt occurred"]
    NOEVENT = 0,
    #[doc = "1: Timer x capture reset interrupt occurred"]
    EVENT = 1,
}
impl From<CPT2_A> for bool {
    #[inline(always)]
    fn from(variant: CPT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPT2`"]
pub type CPT2_R = crate::R<bool, CPT2_A>;
impl CPT2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPT2_A {
        match self.bits {
            false => CPT2_A::NOEVENT,
            true => CPT2_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == CPT2_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CPT2_A::EVENT
    }
}
#[doc = "Capture1 Interrupt Flag"]
pub type CPT1_A = CPT2_A;
#[doc = "Reader of field `CPT1`"]
pub type CPT1_R = crate::R<bool, CPT2_A>;
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
#[doc = "Reader of field `UPD`"]
pub type UPD_R = crate::R<bool, UPD_A>;
impl UPD_R {
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
        *self == UPD_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == UPD_A::EVENT
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
#[doc = "Reader of field `REP`"]
pub type REP_R = crate::R<bool, REP_A>;
impl REP_R {
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
        *self == REP_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == REP_A::EVENT
    }
}
#[doc = "Compare 4 Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP4_A {
    #[doc = "0: No compare interrupt occurred"]
    NOEVENT = 0,
    #[doc = "1: Compare interrupt occurred"]
    EVENT = 1,
}
impl From<CMP4_A> for bool {
    #[inline(always)]
    fn from(variant: CMP4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMP4`"]
pub type CMP4_R = crate::R<bool, CMP4_A>;
impl CMP4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP4_A {
        match self.bits {
            false => CMP4_A::NOEVENT,
            true => CMP4_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == CMP4_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CMP4_A::EVENT
    }
}
#[doc = "Compare 3 Interrupt Flag"]
pub type CMP3_A = CMP4_A;
#[doc = "Reader of field `CMP3`"]
pub type CMP3_R = crate::R<bool, CMP4_A>;
#[doc = "Compare 2 Interrupt Flag"]
pub type CMP2_A = CMP4_A;
#[doc = "Reader of field `CMP2`"]
pub type CMP2_R = crate::R<bool, CMP4_A>;
#[doc = "Compare 1 Interrupt Flag"]
pub type CMP1_A = CMP4_A;
#[doc = "Reader of field `CMP1`"]
pub type CMP1_R = crate::R<bool, CMP4_A>;
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
