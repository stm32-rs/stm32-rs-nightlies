#[doc = "Register `TIMCISR` reader"]
pub type R = crate::R<TIMCISRrs>;
#[doc = "Compare 1 Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP1 {
    #[doc = "0: No compare interrupt occurred"]
    NoEvent = 0,
    #[doc = "1: Compare interrupt occurred"]
    Event = 1,
}
impl From<CMP1> for bool {
    #[inline(always)]
    fn from(variant: CMP1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP1` reader - Compare 1 Interrupt Flag"]
pub type CMP1_R = crate::BitReader<CMP1>;
impl CMP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMP1 {
        match self.bits {
            false => CMP1::NoEvent,
            true => CMP1::Event,
        }
    }
    #[doc = "No compare interrupt occurred"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == CMP1::NoEvent
    }
    #[doc = "Compare interrupt occurred"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CMP1::Event
    }
}
#[doc = "Field `CMP2` reader - Compare 2 Interrupt Flag"]
pub use CMP1_R as CMP2_R;
#[doc = "Field `CMP3` reader - Compare 3 Interrupt Flag"]
pub use CMP1_R as CMP3_R;
#[doc = "Field `CMP4` reader - Compare 4 Interrupt Flag"]
pub use CMP1_R as CMP4_R;
#[doc = "Repetition Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REP {
    #[doc = "0: No timer repetition interrupt occurred"]
    NoEvent = 0,
    #[doc = "1: Timer repetition interrupt occurred"]
    Event = 1,
}
impl From<REP> for bool {
    #[inline(always)]
    fn from(variant: REP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REP` reader - Repetition Interrupt Flag"]
pub type REP_R = crate::BitReader<REP>;
impl REP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REP {
        match self.bits {
            false => REP::NoEvent,
            true => REP::Event,
        }
    }
    #[doc = "No timer repetition interrupt occurred"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == REP::NoEvent
    }
    #[doc = "Timer repetition interrupt occurred"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == REP::Event
    }
}
#[doc = "Update Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPD {
    #[doc = "0: No timer update interrupt occurred"]
    NoEvent = 0,
    #[doc = "1: Timer update interrupt occurred"]
    Event = 1,
}
impl From<UPD> for bool {
    #[inline(always)]
    fn from(variant: UPD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPD` reader - Update Interrupt Flag"]
pub type UPD_R = crate::BitReader<UPD>;
impl UPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UPD {
        match self.bits {
            false => UPD::NoEvent,
            true => UPD::Event,
        }
    }
    #[doc = "No timer update interrupt occurred"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == UPD::NoEvent
    }
    #[doc = "Timer update interrupt occurred"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == UPD::Event
    }
}
#[doc = "Capture1 Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPT1 {
    #[doc = "0: No timer x capture reset interrupt occurred"]
    NoEvent = 0,
    #[doc = "1: Timer x capture reset interrupt occurred"]
    Event = 1,
}
impl From<CPT1> for bool {
    #[inline(always)]
    fn from(variant: CPT1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPT1` reader - Capture1 Interrupt Flag"]
pub type CPT1_R = crate::BitReader<CPT1>;
impl CPT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPT1 {
        match self.bits {
            false => CPT1::NoEvent,
            true => CPT1::Event,
        }
    }
    #[doc = "No timer x capture reset interrupt occurred"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == CPT1::NoEvent
    }
    #[doc = "Timer x capture reset interrupt occurred"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CPT1::Event
    }
}
#[doc = "Field `CPT2` reader - Capture2 Interrupt Flag"]
pub use CPT1_R as CPT2_R;
#[doc = "Output 1 Set Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETX1 {
    #[doc = "0: No Tx output set interrupt occurred"]
    NoEvent = 0,
    #[doc = "1: Tx output set interrupt occurred"]
    Event = 1,
}
impl From<SETX1> for bool {
    #[inline(always)]
    fn from(variant: SETX1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETx1` reader - Output 1 Set Interrupt Flag"]
pub type SETX1_R = crate::BitReader<SETX1>;
impl SETX1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SETX1 {
        match self.bits {
            false => SETX1::NoEvent,
            true => SETX1::Event,
        }
    }
    #[doc = "No Tx output set interrupt occurred"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == SETX1::NoEvent
    }
    #[doc = "Tx output set interrupt occurred"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == SETX1::Event
    }
}
#[doc = "Output 1 Reset Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTX1 {
    #[doc = "0: No Tx output reset interrupt occurred"]
    NoEvent = 0,
    #[doc = "1: Tx output reset interrupt occurred"]
    Event = 1,
}
impl From<RSTX1> for bool {
    #[inline(always)]
    fn from(variant: RSTX1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTx1` reader - Output 1 Reset Interrupt Flag"]
pub type RSTX1_R = crate::BitReader<RSTX1>;
impl RSTX1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSTX1 {
        match self.bits {
            false => RSTX1::NoEvent,
            true => RSTX1::Event,
        }
    }
    #[doc = "No Tx output reset interrupt occurred"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == RSTX1::NoEvent
    }
    #[doc = "Tx output reset interrupt occurred"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == RSTX1::Event
    }
}
#[doc = "Field `RSTx2` reader - Output 2 Reset Interrupt Flag"]
pub use RSTX1_R as RSTX2_R;
#[doc = "Field `SETx2` reader - Output 2 Set Interrupt Flag"]
pub use SETX1_R as SETX2_R;
#[doc = "Reset Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RST {
    #[doc = "0: No TIMx counter reset/roll-over interrupt occurred"]
    NoEvent = 0,
    #[doc = "1: TIMx counter reset/roll-over interrupt occurred"]
    Event = 1,
}
impl From<RST> for bool {
    #[inline(always)]
    fn from(variant: RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST` reader - Reset Interrupt Flag"]
pub type RST_R = crate::BitReader<RST>;
impl RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RST {
        match self.bits {
            false => RST::NoEvent,
            true => RST::Event,
        }
    }
    #[doc = "No TIMx counter reset/roll-over interrupt occurred"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == RST::NoEvent
    }
    #[doc = "TIMx counter reset/roll-over interrupt occurred"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == RST::Event
    }
}
#[doc = "Delayed Protection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYPRT {
    #[doc = "0: Not in delayed idle or balanced idle mode"]
    Inactive = 0,
    #[doc = "1: Delayed idle or balanced idle mode entry"]
    Active = 1,
}
impl From<DLYPRT> for bool {
    #[inline(always)]
    fn from(variant: DLYPRT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLYPRT` reader - Delayed Protection Flag"]
pub type DLYPRT_R = crate::BitReader<DLYPRT>;
impl DLYPRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DLYPRT {
        match self.bits {
            false => DLYPRT::Inactive,
            true => DLYPRT::Active,
        }
    }
    #[doc = "Not in delayed idle or balanced idle mode"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DLYPRT::Inactive
    }
    #[doc = "Delayed idle or balanced idle mode entry"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == DLYPRT::Active
    }
}
#[doc = "Current Push Pull Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPPSTAT {
    #[doc = "0: Signal applied on output 1 and output 2 forced inactive"]
    Output1active = 0,
    #[doc = "1: Signal applied on output 2 and output 1 forced inactive"]
    Output2active = 1,
}
impl From<CPPSTAT> for bool {
    #[inline(always)]
    fn from(variant: CPPSTAT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPPSTAT` reader - Current Push Pull Status"]
pub type CPPSTAT_R = crate::BitReader<CPPSTAT>;
impl CPPSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPPSTAT {
        match self.bits {
            false => CPPSTAT::Output1active,
            true => CPPSTAT::Output2active,
        }
    }
    #[doc = "Signal applied on output 1 and output 2 forced inactive"]
    #[inline(always)]
    pub fn is_output1active(&self) -> bool {
        *self == CPPSTAT::Output1active
    }
    #[doc = "Signal applied on output 2 and output 1 forced inactive"]
    #[inline(always)]
    pub fn is_output2active(&self) -> bool {
        *self == CPPSTAT::Output2active
    }
}
#[doc = "Idle Push Pull Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPPSTAT {
    #[doc = "0: Protection occurred when the output 1 was active and output 2 forced inactive"]
    Output1active = 0,
    #[doc = "1: Protection occurred when the output 2 was active and output 1 forced inactive"]
    Output2active = 1,
}
impl From<IPPSTAT> for bool {
    #[inline(always)]
    fn from(variant: IPPSTAT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IPPSTAT` reader - Idle Push Pull Status"]
pub type IPPSTAT_R = crate::BitReader<IPPSTAT>;
impl IPPSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IPPSTAT {
        match self.bits {
            false => IPPSTAT::Output1active,
            true => IPPSTAT::Output2active,
        }
    }
    #[doc = "Protection occurred when the output 1 was active and output 2 forced inactive"]
    #[inline(always)]
    pub fn is_output1active(&self) -> bool {
        *self == IPPSTAT::Output1active
    }
    #[doc = "Protection occurred when the output 2 was active and output 1 forced inactive"]
    #[inline(always)]
    pub fn is_output2active(&self) -> bool {
        *self == IPPSTAT::Output2active
    }
}
#[doc = "Output 1 State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum O1STAT {
    #[doc = "0: Output was inactive"]
    Inactive = 0,
    #[doc = "1: Output was active"]
    Active = 1,
}
impl From<O1STAT> for bool {
    #[inline(always)]
    fn from(variant: O1STAT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `O1STAT` reader - Output 1 State"]
pub type O1STAT_R = crate::BitReader<O1STAT>;
impl O1STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> O1STAT {
        match self.bits {
            false => O1STAT::Inactive,
            true => O1STAT::Active,
        }
    }
    #[doc = "Output was inactive"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == O1STAT::Inactive
    }
    #[doc = "Output was active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == O1STAT::Active
    }
}
#[doc = "Field `O2STAT` reader - Output 2 State"]
pub use O1STAT_R as O2STAT_R;
#[doc = "Output 1 Copy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum O1CPY {
    #[doc = "0: Output is inactive"]
    Inactive = 0,
    #[doc = "1: Output is active"]
    Active = 1,
}
impl From<O1CPY> for bool {
    #[inline(always)]
    fn from(variant: O1CPY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `O1CPY` reader - Output 1 Copy"]
pub type O1CPY_R = crate::BitReader<O1CPY>;
impl O1CPY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> O1CPY {
        match self.bits {
            false => O1CPY::Inactive,
            true => O1CPY::Active,
        }
    }
    #[doc = "Output is inactive"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == O1CPY::Inactive
    }
    #[doc = "Output is active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == O1CPY::Active
    }
}
#[doc = "Field `O2CPY` reader - Output 2 Copy"]
pub use O1CPY_R as O2CPY_R;
impl R {
    #[doc = "Bit 0 - Compare 1 Interrupt Flag"]
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare 2 Interrupt Flag"]
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare 3 Interrupt Flag"]
    #[inline(always)]
    pub fn cmp3(&self) -> CMP3_R {
        CMP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare 4 Interrupt Flag"]
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Repetition Interrupt Flag"]
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Update Interrupt Flag"]
    #[inline(always)]
    pub fn upd(&self) -> UPD_R {
        UPD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture1 Interrupt Flag"]
    #[inline(always)]
    pub fn cpt1(&self) -> CPT1_R {
        CPT1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture2 Interrupt Flag"]
    #[inline(always)]
    pub fn cpt2(&self) -> CPT2_R {
        CPT2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output 1 Set Interrupt Flag"]
    #[inline(always)]
    pub fn setx1(&self) -> SETX1_R {
        SETX1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output 1 Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rstx1(&self) -> RSTX1_R {
        RSTX1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output 2 Set Interrupt Flag"]
    #[inline(always)]
    pub fn setx2(&self) -> SETX2_R {
        SETX2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Output 2 Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rstx2(&self) -> RSTX2_R {
        RSTX2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Delayed Protection Flag"]
    #[inline(always)]
    pub fn dlyprt(&self) -> DLYPRT_R {
        DLYPRT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Current Push Pull Status"]
    #[inline(always)]
    pub fn cppstat(&self) -> CPPSTAT_R {
        CPPSTAT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Idle Push Pull Status"]
    #[inline(always)]
    pub fn ippstat(&self) -> IPPSTAT_R {
        IPPSTAT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output 1 State"]
    #[inline(always)]
    pub fn o1stat(&self) -> O1STAT_R {
        O1STAT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output 2 State"]
    #[inline(always)]
    pub fn o2stat(&self) -> O2STAT_R {
        O2STAT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output 1 Copy"]
    #[inline(always)]
    pub fn o1cpy(&self) -> O1CPY_R {
        O1CPY_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Output 2 Copy"]
    #[inline(always)]
    pub fn o2cpy(&self) -> O2CPY_R {
        O2CPY_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "Timerx Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timcisr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMCISRrs;
impl crate::RegisterSpec for TIMCISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timcisr::R`](R) reader structure"]
impl crate::Readable for TIMCISRrs {}
#[doc = "`reset()` method sets TIMCISR to value 0"]
impl crate::Resettable for TIMCISRrs {
    const RESET_VALUE: u32 = 0;
}
