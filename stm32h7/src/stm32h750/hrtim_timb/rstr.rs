///Register `RSTR` reader
pub type R = crate::R<RSTRrs>;
///Register `RSTR` writer
pub type W = crate::W<RSTRrs>;
/**Timer A Update reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPDT {
    ///0: Update event has no effect
    NoEffect = 0,
    ///1: Timer X counter is reset upon update event
    ResetCounter = 1,
}
impl From<UPDT> for bool {
    #[inline(always)]
    fn from(variant: UPDT) -> Self {
        variant as u8 != 0
    }
}
///Field `UPDT` reader - Timer A Update reset
pub type UPDT_R = crate::BitReader<UPDT>;
impl UPDT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UPDT {
        match self.bits {
            false => UPDT::NoEffect,
            true => UPDT::ResetCounter,
        }
    }
    ///Update event has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == UPDT::NoEffect
    }
    ///Timer X counter is reset upon update event
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        *self == UPDT::ResetCounter
    }
}
///Field `UPDT` writer - Timer A Update reset
pub type UPDT_W<'a, REG> = crate::BitWriter<'a, REG, UPDT>;
impl<'a, REG> UPDT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Update event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(UPDT::NoEffect)
    }
    ///Timer X counter is reset upon update event
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut crate::W<REG> {
        self.variant(UPDT::ResetCounter)
    }
}
/**Timer A compare 2 reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP2 {
    ///0: Timer X compare Z event has no effect
    NoEffect = 0,
    ///1: Timer X counter is reset upon timer X compare Z event
    ResetCounter = 1,
}
impl From<CMP2> for bool {
    #[inline(always)]
    fn from(variant: CMP2) -> Self {
        variant as u8 != 0
    }
}
///Field `CMP2` reader - Timer A compare 2 reset
pub type CMP2_R = crate::BitReader<CMP2>;
impl CMP2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMP2 {
        match self.bits {
            false => CMP2::NoEffect,
            true => CMP2::ResetCounter,
        }
    }
    ///Timer X compare Z event has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CMP2::NoEffect
    }
    ///Timer X counter is reset upon timer X compare Z event
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        *self == CMP2::ResetCounter
    }
}
///Field `CMP2` writer - Timer A compare 2 reset
pub type CMP2_W<'a, REG> = crate::BitWriter<'a, REG, CMP2>;
impl<'a, REG> CMP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timer X compare Z event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(CMP2::NoEffect)
    }
    ///Timer X counter is reset upon timer X compare Z event
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut crate::W<REG> {
        self.variant(CMP2::ResetCounter)
    }
}
///Field `CMP4` reader - Timer A compare 4 reset
pub use CMP2_R as CMP4_R;
///Field `CMP4` writer - Timer A compare 4 reset
pub use CMP2_W as CMP4_W;
/**Master timer Period

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPER {
    ///0: Master timer period event has no effect
    NoEffect = 0,
    ///1: Timer X counter is reset upon master timer period event
    ResetCounter = 1,
}
impl From<MSTPER> for bool {
    #[inline(always)]
    fn from(variant: MSTPER) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPER` reader - Master timer Period
pub type MSTPER_R = crate::BitReader<MSTPER>;
impl MSTPER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPER {
        match self.bits {
            false => MSTPER::NoEffect,
            true => MSTPER::ResetCounter,
        }
    }
    ///Master timer period event has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTPER::NoEffect
    }
    ///Timer X counter is reset upon master timer period event
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        *self == MSTPER::ResetCounter
    }
}
///Field `MSTPER` writer - Master timer Period
pub type MSTPER_W<'a, REG> = crate::BitWriter<'a, REG, MSTPER>;
impl<'a, REG> MSTPER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Master timer period event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPER::NoEffect)
    }
    ///Timer X counter is reset upon master timer period event
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPER::ResetCounter)
    }
}
/**Master compare 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTCMP1 {
    ///0: Master timer compare Z event has no effect
    NoEffect = 0,
    ///1: Timer X counter is reset upon master timer compare Z event
    ResetCounter = 1,
}
impl From<MSTCMP1> for bool {
    #[inline(always)]
    fn from(variant: MSTCMP1) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTCMP1` reader - Master compare 1
pub type MSTCMP1_R = crate::BitReader<MSTCMP1>;
impl MSTCMP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTCMP1 {
        match self.bits {
            false => MSTCMP1::NoEffect,
            true => MSTCMP1::ResetCounter,
        }
    }
    ///Master timer compare Z event has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTCMP1::NoEffect
    }
    ///Timer X counter is reset upon master timer compare Z event
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        *self == MSTCMP1::ResetCounter
    }
}
///Field `MSTCMP1` writer - Master compare 1
pub type MSTCMP1_W<'a, REG> = crate::BitWriter<'a, REG, MSTCMP1>;
impl<'a, REG> MSTCMP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Master timer compare Z event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(MSTCMP1::NoEffect)
    }
    ///Timer X counter is reset upon master timer compare Z event
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut crate::W<REG> {
        self.variant(MSTCMP1::ResetCounter)
    }
}
///Field `MSTCMP2` reader - Master compare 2
pub use MSTCMP1_R as MSTCMP2_R;
///Field `MSTCMP3` reader - Master compare 3
pub use MSTCMP1_R as MSTCMP3_R;
///Field `MSTCMP4` reader - Master compare 4
pub use MSTCMP1_R as MSTCMP4_R;
///Field `MSTCMP2` writer - Master compare 2
pub use MSTCMP1_W as MSTCMP2_W;
///Field `MSTCMP3` writer - Master compare 3
pub use MSTCMP1_W as MSTCMP3_W;
///Field `MSTCMP4` writer - Master compare 4
pub use MSTCMP1_W as MSTCMP4_W;
/**External Event 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTEVNT1 {
    ///0: External event Z has no effect
    NoEffect = 0,
    ///1: Timer X counter is reset upon external event Z
    ResetCounter = 1,
}
impl From<EXTEVNT1> for bool {
    #[inline(always)]
    fn from(variant: EXTEVNT1) -> Self {
        variant as u8 != 0
    }
}
///Field `EXTEVNT1` reader - External Event 1
pub type EXTEVNT1_R = crate::BitReader<EXTEVNT1>;
impl EXTEVNT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXTEVNT1 {
        match self.bits {
            false => EXTEVNT1::NoEffect,
            true => EXTEVNT1::ResetCounter,
        }
    }
    ///External event Z has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == EXTEVNT1::NoEffect
    }
    ///Timer X counter is reset upon external event Z
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        *self == EXTEVNT1::ResetCounter
    }
}
///Field `EXTEVNT1` writer - External Event 1
pub type EXTEVNT1_W<'a, REG> = crate::BitWriter<'a, REG, EXTEVNT1>;
impl<'a, REG> EXTEVNT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///External event Z has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEVNT1::NoEffect)
    }
    ///Timer X counter is reset upon external event Z
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEVNT1::ResetCounter)
    }
}
///Field `EXTEVNT2` reader - External Event 2
pub use EXTEVNT1_R as EXTEVNT2_R;
///Field `EXTEVNT3` reader - External Event 3
pub use EXTEVNT1_R as EXTEVNT3_R;
///Field `EXTEVNT4` reader - External Event 4
pub use EXTEVNT1_R as EXTEVNT4_R;
///Field `EXTEVNT5` reader - External Event 5
pub use EXTEVNT1_R as EXTEVNT5_R;
///Field `EXTEVNT6` reader - External Event 6
pub use EXTEVNT1_R as EXTEVNT6_R;
///Field `EXTEVNT7` reader - External Event 7
pub use EXTEVNT1_R as EXTEVNT7_R;
///Field `EXTEVNT8` reader - External Event 8
pub use EXTEVNT1_R as EXTEVNT8_R;
///Field `EXTEVNT9` reader - External Event 9
pub use EXTEVNT1_R as EXTEVNT9_R;
///Field `EXTEVNT10` reader - External Event 10
pub use EXTEVNT1_R as EXTEVNT10_R;
///Field `EXTEVNT2` writer - External Event 2
pub use EXTEVNT1_W as EXTEVNT2_W;
///Field `EXTEVNT3` writer - External Event 3
pub use EXTEVNT1_W as EXTEVNT3_W;
///Field `EXTEVNT4` writer - External Event 4
pub use EXTEVNT1_W as EXTEVNT4_W;
///Field `EXTEVNT5` writer - External Event 5
pub use EXTEVNT1_W as EXTEVNT5_W;
///Field `EXTEVNT6` writer - External Event 6
pub use EXTEVNT1_W as EXTEVNT6_W;
///Field `EXTEVNT7` writer - External Event 7
pub use EXTEVNT1_W as EXTEVNT7_W;
///Field `EXTEVNT8` writer - External Event 8
pub use EXTEVNT1_W as EXTEVNT8_W;
///Field `EXTEVNT9` writer - External Event 9
pub use EXTEVNT1_W as EXTEVNT9_W;
///Field `EXTEVNT10` writer - External Event 10
pub use EXTEVNT1_W as EXTEVNT10_W;
/**Timer A Compare 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMACMP1 {
    ///0: Timer Y compare Z event has no effect
    NoEffect = 0,
    ///1: Timer X counter is reset upon timer Y compare Z event
    ResetCounter = 1,
}
impl From<TIMACMP1> for bool {
    #[inline(always)]
    fn from(variant: TIMACMP1) -> Self {
        variant as u8 != 0
    }
}
///Field `TIMACMP1` reader - Timer A Compare 1
pub type TIMACMP1_R = crate::BitReader<TIMACMP1>;
impl TIMACMP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIMACMP1 {
        match self.bits {
            false => TIMACMP1::NoEffect,
            true => TIMACMP1::ResetCounter,
        }
    }
    ///Timer Y compare Z event has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TIMACMP1::NoEffect
    }
    ///Timer X counter is reset upon timer Y compare Z event
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        *self == TIMACMP1::ResetCounter
    }
}
///Field `TIMACMP1` writer - Timer A Compare 1
pub type TIMACMP1_W<'a, REG> = crate::BitWriter<'a, REG, TIMACMP1>;
impl<'a, REG> TIMACMP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timer Y compare Z event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TIMACMP1::NoEffect)
    }
    ///Timer X counter is reset upon timer Y compare Z event
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut crate::W<REG> {
        self.variant(TIMACMP1::ResetCounter)
    }
}
///Field `TIMACMP2` reader - Timer A Compare 2
pub use TIMACMP1_R as TIMACMP2_R;
///Field `TIMACMP4` reader - Timer A Compare 4
pub use TIMACMP1_R as TIMACMP4_R;
///Field `TIMCCMP1` reader - Timer C Compare 1
pub use TIMACMP1_R as TIMCCMP1_R;
///Field `TIMCCMP2` reader - Timer C Compare 2
pub use TIMACMP1_R as TIMCCMP2_R;
///Field `TIMCCMP4` reader - Timer C Compare 4
pub use TIMACMP1_R as TIMCCMP4_R;
///Field `TIMDCMP1` reader - Timer D Compare 1
pub use TIMACMP1_R as TIMDCMP1_R;
///Field `TIMDCMP2` reader - Timer D Compare 2
pub use TIMACMP1_R as TIMDCMP2_R;
///Field `TIMDCMP4` reader - Timer D Compare 4
pub use TIMACMP1_R as TIMDCMP4_R;
///Field `TIMECMP1` reader - Timer E Compare 1
pub use TIMACMP1_R as TIMECMP1_R;
///Field `TIMECMP2` reader - Timer E Compare 2
pub use TIMACMP1_R as TIMECMP2_R;
///Field `TIMECMP4` reader - Timer E Compare 4
pub use TIMACMP1_R as TIMECMP4_R;
///Field `TIMACMP2` writer - Timer A Compare 2
pub use TIMACMP1_W as TIMACMP2_W;
///Field `TIMACMP4` writer - Timer A Compare 4
pub use TIMACMP1_W as TIMACMP4_W;
///Field `TIMCCMP1` writer - Timer C Compare 1
pub use TIMACMP1_W as TIMCCMP1_W;
///Field `TIMCCMP2` writer - Timer C Compare 2
pub use TIMACMP1_W as TIMCCMP2_W;
///Field `TIMCCMP4` writer - Timer C Compare 4
pub use TIMACMP1_W as TIMCCMP4_W;
///Field `TIMDCMP1` writer - Timer D Compare 1
pub use TIMACMP1_W as TIMDCMP1_W;
///Field `TIMDCMP2` writer - Timer D Compare 2
pub use TIMACMP1_W as TIMDCMP2_W;
///Field `TIMDCMP4` writer - Timer D Compare 4
pub use TIMACMP1_W as TIMDCMP4_W;
///Field `TIMECMP1` writer - Timer E Compare 1
pub use TIMACMP1_W as TIMECMP1_W;
///Field `TIMECMP2` writer - Timer E Compare 2
pub use TIMACMP1_W as TIMECMP2_W;
///Field `TIMECMP4` writer - Timer E Compare 4
pub use TIMACMP1_W as TIMECMP4_W;
impl R {
    ///Bit 1 - Timer A Update reset
    #[inline(always)]
    pub fn updt(&self) -> UPDT_R {
        UPDT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Timer A compare 2 reset
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timer A compare 4 reset
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Master timer Period
    #[inline(always)]
    pub fn mstper(&self) -> MSTPER_R {
        MSTPER_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Master compare 1
    #[inline(always)]
    pub fn mstcmp1(&self) -> MSTCMP1_R {
        MSTCMP1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Master compare 2
    #[inline(always)]
    pub fn mstcmp2(&self) -> MSTCMP2_R {
        MSTCMP2_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Master compare 3
    #[inline(always)]
    pub fn mstcmp3(&self) -> MSTCMP3_R {
        MSTCMP3_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Master compare 4
    #[inline(always)]
    pub fn mstcmp4(&self) -> MSTCMP4_R {
        MSTCMP4_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - External Event 1
    #[inline(always)]
    pub fn extevnt1(&self) -> EXTEVNT1_R {
        EXTEVNT1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - External Event 2
    #[inline(always)]
    pub fn extevnt2(&self) -> EXTEVNT2_R {
        EXTEVNT2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - External Event 3
    #[inline(always)]
    pub fn extevnt3(&self) -> EXTEVNT3_R {
        EXTEVNT3_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - External Event 4
    #[inline(always)]
    pub fn extevnt4(&self) -> EXTEVNT4_R {
        EXTEVNT4_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - External Event 5
    #[inline(always)]
    pub fn extevnt5(&self) -> EXTEVNT5_R {
        EXTEVNT5_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - External Event 6
    #[inline(always)]
    pub fn extevnt6(&self) -> EXTEVNT6_R {
        EXTEVNT6_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - External Event 7
    #[inline(always)]
    pub fn extevnt7(&self) -> EXTEVNT7_R {
        EXTEVNT7_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - External Event 8
    #[inline(always)]
    pub fn extevnt8(&self) -> EXTEVNT8_R {
        EXTEVNT8_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - External Event 9
    #[inline(always)]
    pub fn extevnt9(&self) -> EXTEVNT9_R {
        EXTEVNT9_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - External Event 10
    #[inline(always)]
    pub fn extevnt10(&self) -> EXTEVNT10_R {
        EXTEVNT10_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Timer A Compare 1
    #[inline(always)]
    pub fn timacmp1(&self) -> TIMACMP1_R {
        TIMACMP1_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Timer A Compare 2
    #[inline(always)]
    pub fn timacmp2(&self) -> TIMACMP2_R {
        TIMACMP2_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Timer A Compare 4
    #[inline(always)]
    pub fn timacmp4(&self) -> TIMACMP4_R {
        TIMACMP4_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Timer C Compare 1
    #[inline(always)]
    pub fn timccmp1(&self) -> TIMCCMP1_R {
        TIMCCMP1_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Timer C Compare 2
    #[inline(always)]
    pub fn timccmp2(&self) -> TIMCCMP2_R {
        TIMCCMP2_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Timer C Compare 4
    #[inline(always)]
    pub fn timccmp4(&self) -> TIMCCMP4_R {
        TIMCCMP4_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Timer D Compare 1
    #[inline(always)]
    pub fn timdcmp1(&self) -> TIMDCMP1_R {
        TIMDCMP1_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Timer D Compare 2
    #[inline(always)]
    pub fn timdcmp2(&self) -> TIMDCMP2_R {
        TIMDCMP2_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Timer D Compare 4
    #[inline(always)]
    pub fn timdcmp4(&self) -> TIMDCMP4_R {
        TIMDCMP4_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Timer E Compare 1
    #[inline(always)]
    pub fn timecmp1(&self) -> TIMECMP1_R {
        TIMECMP1_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Timer E Compare 2
    #[inline(always)]
    pub fn timecmp2(&self) -> TIMECMP2_R {
        TIMECMP2_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Timer E Compare 4
    #[inline(always)]
    pub fn timecmp4(&self) -> TIMECMP4_R {
        TIMECMP4_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSTR")
            .field("timacmp1", &self.timacmp1())
            .field("timecmp4", &self.timecmp4())
            .field("timecmp2", &self.timecmp2())
            .field("timecmp1", &self.timecmp1())
            .field("timdcmp4", &self.timdcmp4())
            .field("timdcmp2", &self.timdcmp2())
            .field("timdcmp1", &self.timdcmp1())
            .field("timccmp4", &self.timccmp4())
            .field("timccmp2", &self.timccmp2())
            .field("timccmp1", &self.timccmp1())
            .field("timacmp4", &self.timacmp4())
            .field("timacmp2", &self.timacmp2())
            .field("extevnt1", &self.extevnt1())
            .field("extevnt10", &self.extevnt10())
            .field("extevnt9", &self.extevnt9())
            .field("extevnt8", &self.extevnt8())
            .field("extevnt7", &self.extevnt7())
            .field("extevnt6", &self.extevnt6())
            .field("extevnt5", &self.extevnt5())
            .field("extevnt4", &self.extevnt4())
            .field("extevnt3", &self.extevnt3())
            .field("extevnt2", &self.extevnt2())
            .field("mstcmp1", &self.mstcmp1())
            .field("mstcmp4", &self.mstcmp4())
            .field("mstcmp3", &self.mstcmp3())
            .field("mstcmp2", &self.mstcmp2())
            .field("mstper", &self.mstper())
            .field("cmp2", &self.cmp2())
            .field("cmp4", &self.cmp4())
            .field("updt", &self.updt())
            .finish()
    }
}
impl W {
    ///Bit 1 - Timer A Update reset
    #[inline(always)]
    pub fn updt(&mut self) -> UPDT_W<RSTRrs> {
        UPDT_W::new(self, 1)
    }
    ///Bit 2 - Timer A compare 2 reset
    #[inline(always)]
    pub fn cmp2(&mut self) -> CMP2_W<RSTRrs> {
        CMP2_W::new(self, 2)
    }
    ///Bit 3 - Timer A compare 4 reset
    #[inline(always)]
    pub fn cmp4(&mut self) -> CMP4_W<RSTRrs> {
        CMP4_W::new(self, 3)
    }
    ///Bit 4 - Master timer Period
    #[inline(always)]
    pub fn mstper(&mut self) -> MSTPER_W<RSTRrs> {
        MSTPER_W::new(self, 4)
    }
    ///Bit 5 - Master compare 1
    #[inline(always)]
    pub fn mstcmp1(&mut self) -> MSTCMP1_W<RSTRrs> {
        MSTCMP1_W::new(self, 5)
    }
    ///Bit 6 - Master compare 2
    #[inline(always)]
    pub fn mstcmp2(&mut self) -> MSTCMP2_W<RSTRrs> {
        MSTCMP2_W::new(self, 6)
    }
    ///Bit 7 - Master compare 3
    #[inline(always)]
    pub fn mstcmp3(&mut self) -> MSTCMP3_W<RSTRrs> {
        MSTCMP3_W::new(self, 7)
    }
    ///Bit 8 - Master compare 4
    #[inline(always)]
    pub fn mstcmp4(&mut self) -> MSTCMP4_W<RSTRrs> {
        MSTCMP4_W::new(self, 8)
    }
    ///Bit 9 - External Event 1
    #[inline(always)]
    pub fn extevnt1(&mut self) -> EXTEVNT1_W<RSTRrs> {
        EXTEVNT1_W::new(self, 9)
    }
    ///Bit 10 - External Event 2
    #[inline(always)]
    pub fn extevnt2(&mut self) -> EXTEVNT2_W<RSTRrs> {
        EXTEVNT2_W::new(self, 10)
    }
    ///Bit 11 - External Event 3
    #[inline(always)]
    pub fn extevnt3(&mut self) -> EXTEVNT3_W<RSTRrs> {
        EXTEVNT3_W::new(self, 11)
    }
    ///Bit 12 - External Event 4
    #[inline(always)]
    pub fn extevnt4(&mut self) -> EXTEVNT4_W<RSTRrs> {
        EXTEVNT4_W::new(self, 12)
    }
    ///Bit 13 - External Event 5
    #[inline(always)]
    pub fn extevnt5(&mut self) -> EXTEVNT5_W<RSTRrs> {
        EXTEVNT5_W::new(self, 13)
    }
    ///Bit 14 - External Event 6
    #[inline(always)]
    pub fn extevnt6(&mut self) -> EXTEVNT6_W<RSTRrs> {
        EXTEVNT6_W::new(self, 14)
    }
    ///Bit 15 - External Event 7
    #[inline(always)]
    pub fn extevnt7(&mut self) -> EXTEVNT7_W<RSTRrs> {
        EXTEVNT7_W::new(self, 15)
    }
    ///Bit 16 - External Event 8
    #[inline(always)]
    pub fn extevnt8(&mut self) -> EXTEVNT8_W<RSTRrs> {
        EXTEVNT8_W::new(self, 16)
    }
    ///Bit 17 - External Event 9
    #[inline(always)]
    pub fn extevnt9(&mut self) -> EXTEVNT9_W<RSTRrs> {
        EXTEVNT9_W::new(self, 17)
    }
    ///Bit 18 - External Event 10
    #[inline(always)]
    pub fn extevnt10(&mut self) -> EXTEVNT10_W<RSTRrs> {
        EXTEVNT10_W::new(self, 18)
    }
    ///Bit 19 - Timer A Compare 1
    #[inline(always)]
    pub fn timacmp1(&mut self) -> TIMACMP1_W<RSTRrs> {
        TIMACMP1_W::new(self, 19)
    }
    ///Bit 20 - Timer A Compare 2
    #[inline(always)]
    pub fn timacmp2(&mut self) -> TIMACMP2_W<RSTRrs> {
        TIMACMP2_W::new(self, 20)
    }
    ///Bit 21 - Timer A Compare 4
    #[inline(always)]
    pub fn timacmp4(&mut self) -> TIMACMP4_W<RSTRrs> {
        TIMACMP4_W::new(self, 21)
    }
    ///Bit 22 - Timer C Compare 1
    #[inline(always)]
    pub fn timccmp1(&mut self) -> TIMCCMP1_W<RSTRrs> {
        TIMCCMP1_W::new(self, 22)
    }
    ///Bit 23 - Timer C Compare 2
    #[inline(always)]
    pub fn timccmp2(&mut self) -> TIMCCMP2_W<RSTRrs> {
        TIMCCMP2_W::new(self, 23)
    }
    ///Bit 24 - Timer C Compare 4
    #[inline(always)]
    pub fn timccmp4(&mut self) -> TIMCCMP4_W<RSTRrs> {
        TIMCCMP4_W::new(self, 24)
    }
    ///Bit 25 - Timer D Compare 1
    #[inline(always)]
    pub fn timdcmp1(&mut self) -> TIMDCMP1_W<RSTRrs> {
        TIMDCMP1_W::new(self, 25)
    }
    ///Bit 26 - Timer D Compare 2
    #[inline(always)]
    pub fn timdcmp2(&mut self) -> TIMDCMP2_W<RSTRrs> {
        TIMDCMP2_W::new(self, 26)
    }
    ///Bit 27 - Timer D Compare 4
    #[inline(always)]
    pub fn timdcmp4(&mut self) -> TIMDCMP4_W<RSTRrs> {
        TIMDCMP4_W::new(self, 27)
    }
    ///Bit 28 - Timer E Compare 1
    #[inline(always)]
    pub fn timecmp1(&mut self) -> TIMECMP1_W<RSTRrs> {
        TIMECMP1_W::new(self, 28)
    }
    ///Bit 29 - Timer E Compare 2
    #[inline(always)]
    pub fn timecmp2(&mut self) -> TIMECMP2_W<RSTRrs> {
        TIMECMP2_W::new(self, 29)
    }
    ///Bit 30 - Timer E Compare 4
    #[inline(always)]
    pub fn timecmp4(&mut self) -> TIMECMP4_W<RSTRrs> {
        TIMECMP4_W::new(self, 30)
    }
}
/**TimerA Reset Register

You can [`read`](crate::Reg::read) this register and get [`rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#HRTIM_TIMB:RSTR)*/
pub struct RSTRrs;
impl crate::RegisterSpec for RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`rstr::R`](R) reader structure
impl crate::Readable for RSTRrs {}
///`write(|w| ..)` method takes [`rstr::W`](W) writer structure
impl crate::Writable for RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RSTR to value 0
impl crate::Resettable for RSTRrs {}
