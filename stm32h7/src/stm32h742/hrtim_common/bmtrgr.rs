///Register `BMTRGR` reader
pub type R = crate::R<BMTRGRrs>;
///Register `BMTRGR` writer
pub type W = crate::W<BMTRGRrs>;
/**SW

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SW {
    ///0: No effect
    NoEffect = 0,
    ///1: Trigger immediate burst mode operation
    Trigger = 1,
}
impl From<SW> for bool {
    #[inline(always)]
    fn from(variant: SW) -> Self {
        variant as u8 != 0
    }
}
///Field `SW` reader - SW
pub type SW_R = crate::BitReader<SW>;
impl SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SW {
        match self.bits {
            false => SW::NoEffect,
            true => SW::Trigger,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SW::NoEffect
    }
    ///Trigger immediate burst mode operation
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == SW::Trigger
    }
}
///Field `SW` writer - SW
pub type SW_W<'a, REG> = crate::BitWriter<'a, REG, SW>;
impl<'a, REG> SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SW::NoEffect)
    }
    ///Trigger immediate burst mode operation
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Trigger)
    }
}
/**MSTRST

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTRST {
    ///0: Master timer reset/roll-over event has no effect
    NoEffect = 0,
    ///1: Master timer reset/roll-over event triggers a burst mode entry
    Trigger = 1,
}
impl From<MSTRST> for bool {
    #[inline(always)]
    fn from(variant: MSTRST) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTRST` reader - MSTRST
pub type MSTRST_R = crate::BitReader<MSTRST>;
impl MSTRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTRST {
        match self.bits {
            false => MSTRST::NoEffect,
            true => MSTRST::Trigger,
        }
    }
    ///Master timer reset/roll-over event has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTRST::NoEffect
    }
    ///Master timer reset/roll-over event triggers a burst mode entry
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == MSTRST::Trigger
    }
}
///Field `MSTRST` writer - MSTRST
pub type MSTRST_W<'a, REG> = crate::BitWriter<'a, REG, MSTRST>;
impl<'a, REG> MSTRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Master timer reset/roll-over event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(MSTRST::NoEffect)
    }
    ///Master timer reset/roll-over event triggers a burst mode entry
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(MSTRST::Trigger)
    }
}
/**MSTREP

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTREP {
    ///0: Master timer repetition event has no effect
    NoEffect = 0,
    ///1: Master timer repetition event triggers a burst mode entry
    Trigger = 1,
}
impl From<MSTREP> for bool {
    #[inline(always)]
    fn from(variant: MSTREP) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTREP` reader - MSTREP
pub type MSTREP_R = crate::BitReader<MSTREP>;
impl MSTREP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTREP {
        match self.bits {
            false => MSTREP::NoEffect,
            true => MSTREP::Trigger,
        }
    }
    ///Master timer repetition event has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTREP::NoEffect
    }
    ///Master timer repetition event triggers a burst mode entry
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == MSTREP::Trigger
    }
}
///Field `MSTREP` writer - MSTREP
pub type MSTREP_W<'a, REG> = crate::BitWriter<'a, REG, MSTREP>;
impl<'a, REG> MSTREP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Master timer repetition event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(MSTREP::NoEffect)
    }
    ///Master timer repetition event triggers a burst mode entry
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(MSTREP::Trigger)
    }
}
/**MSTCMP1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTCMP1 {
    ///0: Master timer compare X event has no effect
    NoEffect = 0,
    ///1: Master timer compare X event triggers a burst mode entry
    Trigger = 1,
}
impl From<MSTCMP1> for bool {
    #[inline(always)]
    fn from(variant: MSTCMP1) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTCMP1` reader - MSTCMP1
pub type MSTCMP1_R = crate::BitReader<MSTCMP1>;
impl MSTCMP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTCMP1 {
        match self.bits {
            false => MSTCMP1::NoEffect,
            true => MSTCMP1::Trigger,
        }
    }
    ///Master timer compare X event has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTCMP1::NoEffect
    }
    ///Master timer compare X event triggers a burst mode entry
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == MSTCMP1::Trigger
    }
}
///Field `MSTCMP1` writer - MSTCMP1
pub type MSTCMP1_W<'a, REG> = crate::BitWriter<'a, REG, MSTCMP1>;
impl<'a, REG> MSTCMP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Master timer compare X event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(MSTCMP1::NoEffect)
    }
    ///Master timer compare X event triggers a burst mode entry
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(MSTCMP1::Trigger)
    }
}
///Field `MSTCMP2` reader - MSTCMP2
pub use MSTCMP1_R as MSTCMP2_R;
///Field `MSTCMP3` reader - MSTCMP3
pub use MSTCMP1_R as MSTCMP3_R;
///Field `MSTCMP4` reader - MSTCMP4
pub use MSTCMP1_R as MSTCMP4_R;
///Field `MSTCMP2` writer - MSTCMP2
pub use MSTCMP1_W as MSTCMP2_W;
///Field `MSTCMP3` writer - MSTCMP3
pub use MSTCMP1_W as MSTCMP3_W;
///Field `MSTCMP4` writer - MSTCMP4
pub use MSTCMP1_W as MSTCMP4_W;
/**TARST

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TARST {
    ///0: Timer X reset/roll-over event has no effect
    NoEffect = 0,
    ///1: Timer X reset/roll-over event triggers a burst mode entry
    Trigger = 1,
}
impl From<TARST> for bool {
    #[inline(always)]
    fn from(variant: TARST) -> Self {
        variant as u8 != 0
    }
}
///Field `TARST` reader - TARST
pub type TARST_R = crate::BitReader<TARST>;
impl TARST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TARST {
        match self.bits {
            false => TARST::NoEffect,
            true => TARST::Trigger,
        }
    }
    ///Timer X reset/roll-over event has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TARST::NoEffect
    }
    ///Timer X reset/roll-over event triggers a burst mode entry
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == TARST::Trigger
    }
}
///Field `TARST` writer - TARST
pub type TARST_W<'a, REG> = crate::BitWriter<'a, REG, TARST>;
impl<'a, REG> TARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timer X reset/roll-over event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TARST::NoEffect)
    }
    ///Timer X reset/roll-over event triggers a burst mode entry
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TARST::Trigger)
    }
}
/**TAREP

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAREP {
    ///0: Timer X repetition event has no effect
    NoEffect = 0,
    ///1: Timer X repetition event triggers a burst mode entry
    Trigger = 1,
}
impl From<TAREP> for bool {
    #[inline(always)]
    fn from(variant: TAREP) -> Self {
        variant as u8 != 0
    }
}
///Field `TAREP` reader - TAREP
pub type TAREP_R = crate::BitReader<TAREP>;
impl TAREP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAREP {
        match self.bits {
            false => TAREP::NoEffect,
            true => TAREP::Trigger,
        }
    }
    ///Timer X repetition event has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TAREP::NoEffect
    }
    ///Timer X repetition event triggers a burst mode entry
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == TAREP::Trigger
    }
}
///Field `TAREP` writer - TAREP
pub type TAREP_W<'a, REG> = crate::BitWriter<'a, REG, TAREP>;
impl<'a, REG> TAREP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timer X repetition event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TAREP::NoEffect)
    }
    ///Timer X repetition event triggers a burst mode entry
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TAREP::Trigger)
    }
}
/**TACMP1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TACMP1 {
    ///0: Timer X compare Y event has no effect
    NoEffect = 0,
    ///1: Timer X compare Y event triggers a burst mode entry
    Trigger = 1,
}
impl From<TACMP1> for bool {
    #[inline(always)]
    fn from(variant: TACMP1) -> Self {
        variant as u8 != 0
    }
}
///Field `TACMP1` reader - TACMP1
pub type TACMP1_R = crate::BitReader<TACMP1>;
impl TACMP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TACMP1 {
        match self.bits {
            false => TACMP1::NoEffect,
            true => TACMP1::Trigger,
        }
    }
    ///Timer X compare Y event has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TACMP1::NoEffect
    }
    ///Timer X compare Y event triggers a burst mode entry
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == TACMP1::Trigger
    }
}
///Field `TACMP1` writer - TACMP1
pub type TACMP1_W<'a, REG> = crate::BitWriter<'a, REG, TACMP1>;
impl<'a, REG> TACMP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timer X compare Y event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TACMP1::NoEffect)
    }
    ///Timer X compare Y event triggers a burst mode entry
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TACMP1::Trigger)
    }
}
///Field `TACMP2` reader - TACMP2
pub use TACMP1_R as TACMP2_R;
///Field `TBCMP1` reader - TBCMP1
pub use TACMP1_R as TBCMP1_R;
///Field `TBCMP2` reader - TBCMP2
pub use TACMP1_R as TBCMP2_R;
///Field `TCCMP1` reader - TCCMP1
pub use TACMP1_R as TCCMP1_R;
///Field `TCCMP2` reader - TCCMP2
pub use TACMP1_R as TCCMP2_R;
///Field `TDCMP1` reader - TDCMP1
pub use TACMP1_R as TDCMP1_R;
///Field `TDCMP2` reader - TDCMP2
pub use TACMP1_R as TDCMP2_R;
///Field `TECMP1` reader - TECMP1
pub use TACMP1_R as TECMP1_R;
///Field `TECMP2` reader - TECMP2
pub use TACMP1_R as TECMP2_R;
///Field `TACMP2` writer - TACMP2
pub use TACMP1_W as TACMP2_W;
///Field `TBCMP1` writer - TBCMP1
pub use TACMP1_W as TBCMP1_W;
///Field `TBCMP2` writer - TBCMP2
pub use TACMP1_W as TBCMP2_W;
///Field `TCCMP1` writer - TCCMP1
pub use TACMP1_W as TCCMP1_W;
///Field `TCCMP2` writer - TCCMP2
pub use TACMP1_W as TCCMP2_W;
///Field `TDCMP1` writer - TDCMP1
pub use TACMP1_W as TDCMP1_W;
///Field `TDCMP2` writer - TDCMP2
pub use TACMP1_W as TDCMP2_W;
///Field `TECMP1` writer - TECMP1
pub use TACMP1_W as TECMP1_W;
///Field `TECMP2` writer - TECMP2
pub use TACMP1_W as TECMP2_W;
///Field `TBREP` reader - TBREP
pub use TAREP_R as TBREP_R;
///Field `TCREP` reader - TCREP
pub use TAREP_R as TCREP_R;
///Field `TDREP` reader - TDREP
pub use TAREP_R as TDREP_R;
///Field `TEREP` reader - TEREP
pub use TAREP_R as TEREP_R;
///Field `TBREP` writer - TBREP
pub use TAREP_W as TBREP_W;
///Field `TCREP` writer - TCREP
pub use TAREP_W as TCREP_W;
///Field `TDREP` writer - TDREP
pub use TAREP_W as TDREP_W;
///Field `TEREP` writer - TEREP
pub use TAREP_W as TEREP_W;
///Field `TBRST` reader - TBRST
pub use TARST_R as TBRST_R;
///Field `TCRST` reader - TCRST
pub use TARST_R as TCRST_R;
///Field `TDRST` reader - TDRST
pub use TARST_R as TDRST_R;
///Field `TERST` reader - TERST
pub use TARST_R as TERST_R;
///Field `TBRST` writer - TBRST
pub use TARST_W as TBRST_W;
///Field `TCRST` writer - TCRST
pub use TARST_W as TCRST_W;
///Field `TDRST` writer - TDRST
pub use TARST_W as TDRST_W;
///Field `TERST` writer - TERST
pub use TARST_W as TERST_W;
///Field `TAEEV7` reader - Timer A period following External Event 7
pub type TAEEV7_R = crate::BitReader;
///Field `TAEEV7` writer - Timer A period following External Event 7
pub type TAEEV7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TDEEV8` reader - Timer D period following External Event 8
pub type TDEEV8_R = crate::BitReader;
///Field `TDEEV8` writer - Timer D period following External Event 8
pub type TDEEV8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EEV7` reader - External Event 7 (TIMA filters applied)
pub type EEV7_R = crate::BitReader;
///Field `EEV7` writer - External Event 7 (TIMA filters applied)
pub type EEV7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EEV8` reader - External Event 8 (TIMD filters applied)
pub type EEV8_R = crate::BitReader;
///Field `EEV8` writer - External Event 8 (TIMD filters applied)
pub type EEV8_W<'a, REG> = crate::BitWriter<'a, REG>;
/**OCHPEV

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCHPEV {
    ///0: Rising edge on an on-chip event has no effect
    NoEffect = 0,
    ///1: Rising edge on an on-chip event triggers a burst mode entry
    Trigger = 1,
}
impl From<OCHPEV> for bool {
    #[inline(always)]
    fn from(variant: OCHPEV) -> Self {
        variant as u8 != 0
    }
}
///Field `OCHPEV` reader - OCHPEV
pub type OCHPEV_R = crate::BitReader<OCHPEV>;
impl OCHPEV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OCHPEV {
        match self.bits {
            false => OCHPEV::NoEffect,
            true => OCHPEV::Trigger,
        }
    }
    ///Rising edge on an on-chip event has no effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == OCHPEV::NoEffect
    }
    ///Rising edge on an on-chip event triggers a burst mode entry
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == OCHPEV::Trigger
    }
}
///Field `OCHPEV` writer - OCHPEV
pub type OCHPEV_W<'a, REG> = crate::BitWriter<'a, REG, OCHPEV>;
impl<'a, REG> OCHPEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Rising edge on an on-chip event has no effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(OCHPEV::NoEffect)
    }
    ///Rising edge on an on-chip event triggers a burst mode entry
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(OCHPEV::Trigger)
    }
}
impl R {
    ///Bit 0 - SW
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MSTRST
    #[inline(always)]
    pub fn mstrst(&self) -> MSTRST_R {
        MSTRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSTREP
    #[inline(always)]
    pub fn mstrep(&self) -> MSTREP_R {
        MSTREP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MSTCMP1
    #[inline(always)]
    pub fn mstcmp1(&self) -> MSTCMP1_R {
        MSTCMP1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MSTCMP2
    #[inline(always)]
    pub fn mstcmp2(&self) -> MSTCMP2_R {
        MSTCMP2_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MSTCMP3
    #[inline(always)]
    pub fn mstcmp3(&self) -> MSTCMP3_R {
        MSTCMP3_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MSTCMP4
    #[inline(always)]
    pub fn mstcmp4(&self) -> MSTCMP4_R {
        MSTCMP4_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TARST
    #[inline(always)]
    pub fn tarst(&self) -> TARST_R {
        TARST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TAREP
    #[inline(always)]
    pub fn tarep(&self) -> TAREP_R {
        TAREP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TACMP1
    #[inline(always)]
    pub fn tacmp1(&self) -> TACMP1_R {
        TACMP1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - TACMP2
    #[inline(always)]
    pub fn tacmp2(&self) -> TACMP2_R {
        TACMP2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TBRST
    #[inline(always)]
    pub fn tbrst(&self) -> TBRST_R {
        TBRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TBREP
    #[inline(always)]
    pub fn tbrep(&self) -> TBREP_R {
        TBREP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TBCMP1
    #[inline(always)]
    pub fn tbcmp1(&self) -> TBCMP1_R {
        TBCMP1_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - TBCMP2
    #[inline(always)]
    pub fn tbcmp2(&self) -> TBCMP2_R {
        TBCMP2_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TCRST
    #[inline(always)]
    pub fn tcrst(&self) -> TCRST_R {
        TCRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - TCREP
    #[inline(always)]
    pub fn tcrep(&self) -> TCREP_R {
        TCREP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TCCMP1
    #[inline(always)]
    pub fn tccmp1(&self) -> TCCMP1_R {
        TCCMP1_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TCCMP2
    #[inline(always)]
    pub fn tccmp2(&self) -> TCCMP2_R {
        TCCMP2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - TDRST
    #[inline(always)]
    pub fn tdrst(&self) -> TDRST_R {
        TDRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - TDREP
    #[inline(always)]
    pub fn tdrep(&self) -> TDREP_R {
        TDREP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - TDCMP1
    #[inline(always)]
    pub fn tdcmp1(&self) -> TDCMP1_R {
        TDCMP1_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - TDCMP2
    #[inline(always)]
    pub fn tdcmp2(&self) -> TDCMP2_R {
        TDCMP2_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - TERST
    #[inline(always)]
    pub fn terst(&self) -> TERST_R {
        TERST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - TEREP
    #[inline(always)]
    pub fn terep(&self) -> TEREP_R {
        TEREP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - TECMP1
    #[inline(always)]
    pub fn tecmp1(&self) -> TECMP1_R {
        TECMP1_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - TECMP2
    #[inline(always)]
    pub fn tecmp2(&self) -> TECMP2_R {
        TECMP2_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Timer A period following External Event 7
    #[inline(always)]
    pub fn taeev7(&self) -> TAEEV7_R {
        TAEEV7_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Timer D period following External Event 8
    #[inline(always)]
    pub fn tdeev8(&self) -> TDEEV8_R {
        TDEEV8_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - External Event 7 (TIMA filters applied)
    #[inline(always)]
    pub fn eev7(&self) -> EEV7_R {
        EEV7_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - External Event 8 (TIMD filters applied)
    #[inline(always)]
    pub fn eev8(&self) -> EEV8_R {
        EEV8_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - OCHPEV
    #[inline(always)]
    pub fn ochpev(&self) -> OCHPEV_R {
        OCHPEV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BMTRGR")
            .field("ochpev", &self.ochpev())
            .field("tacmp1", &self.tacmp1())
            .field("tecmp2", &self.tecmp2())
            .field("tecmp1", &self.tecmp1())
            .field("tarep", &self.tarep())
            .field("terep", &self.terep())
            .field("tarst", &self.tarst())
            .field("terst", &self.terst())
            .field("tdcmp2", &self.tdcmp2())
            .field("tdcmp1", &self.tdcmp1())
            .field("tdrep", &self.tdrep())
            .field("tdrst", &self.tdrst())
            .field("tccmp2", &self.tccmp2())
            .field("tccmp1", &self.tccmp1())
            .field("tcrep", &self.tcrep())
            .field("tcrst", &self.tcrst())
            .field("tbcmp2", &self.tbcmp2())
            .field("tbcmp1", &self.tbcmp1())
            .field("tbrep", &self.tbrep())
            .field("tbrst", &self.tbrst())
            .field("tacmp2", &self.tacmp2())
            .field("mstcmp1", &self.mstcmp1())
            .field("mstcmp4", &self.mstcmp4())
            .field("mstcmp3", &self.mstcmp3())
            .field("mstcmp2", &self.mstcmp2())
            .field("mstrep", &self.mstrep())
            .field("mstrst", &self.mstrst())
            .field("sw", &self.sw())
            .field("taeev7", &self.taeev7())
            .field("tdeev8", &self.tdeev8())
            .field("eev7", &self.eev7())
            .field("eev8", &self.eev8())
            .finish()
    }
}
impl W {
    ///Bit 0 - SW
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W<'_, BMTRGRrs> {
        SW_W::new(self, 0)
    }
    ///Bit 1 - MSTRST
    #[inline(always)]
    pub fn mstrst(&mut self) -> MSTRST_W<'_, BMTRGRrs> {
        MSTRST_W::new(self, 1)
    }
    ///Bit 2 - MSTREP
    #[inline(always)]
    pub fn mstrep(&mut self) -> MSTREP_W<'_, BMTRGRrs> {
        MSTREP_W::new(self, 2)
    }
    ///Bit 3 - MSTCMP1
    #[inline(always)]
    pub fn mstcmp1(&mut self) -> MSTCMP1_W<'_, BMTRGRrs> {
        MSTCMP1_W::new(self, 3)
    }
    ///Bit 4 - MSTCMP2
    #[inline(always)]
    pub fn mstcmp2(&mut self) -> MSTCMP2_W<'_, BMTRGRrs> {
        MSTCMP2_W::new(self, 4)
    }
    ///Bit 5 - MSTCMP3
    #[inline(always)]
    pub fn mstcmp3(&mut self) -> MSTCMP3_W<'_, BMTRGRrs> {
        MSTCMP3_W::new(self, 5)
    }
    ///Bit 6 - MSTCMP4
    #[inline(always)]
    pub fn mstcmp4(&mut self) -> MSTCMP4_W<'_, BMTRGRrs> {
        MSTCMP4_W::new(self, 6)
    }
    ///Bit 7 - TARST
    #[inline(always)]
    pub fn tarst(&mut self) -> TARST_W<'_, BMTRGRrs> {
        TARST_W::new(self, 7)
    }
    ///Bit 8 - TAREP
    #[inline(always)]
    pub fn tarep(&mut self) -> TAREP_W<'_, BMTRGRrs> {
        TAREP_W::new(self, 8)
    }
    ///Bit 9 - TACMP1
    #[inline(always)]
    pub fn tacmp1(&mut self) -> TACMP1_W<'_, BMTRGRrs> {
        TACMP1_W::new(self, 9)
    }
    ///Bit 10 - TACMP2
    #[inline(always)]
    pub fn tacmp2(&mut self) -> TACMP2_W<'_, BMTRGRrs> {
        TACMP2_W::new(self, 10)
    }
    ///Bit 11 - TBRST
    #[inline(always)]
    pub fn tbrst(&mut self) -> TBRST_W<'_, BMTRGRrs> {
        TBRST_W::new(self, 11)
    }
    ///Bit 12 - TBREP
    #[inline(always)]
    pub fn tbrep(&mut self) -> TBREP_W<'_, BMTRGRrs> {
        TBREP_W::new(self, 12)
    }
    ///Bit 13 - TBCMP1
    #[inline(always)]
    pub fn tbcmp1(&mut self) -> TBCMP1_W<'_, BMTRGRrs> {
        TBCMP1_W::new(self, 13)
    }
    ///Bit 14 - TBCMP2
    #[inline(always)]
    pub fn tbcmp2(&mut self) -> TBCMP2_W<'_, BMTRGRrs> {
        TBCMP2_W::new(self, 14)
    }
    ///Bit 15 - TCRST
    #[inline(always)]
    pub fn tcrst(&mut self) -> TCRST_W<'_, BMTRGRrs> {
        TCRST_W::new(self, 15)
    }
    ///Bit 16 - TCREP
    #[inline(always)]
    pub fn tcrep(&mut self) -> TCREP_W<'_, BMTRGRrs> {
        TCREP_W::new(self, 16)
    }
    ///Bit 17 - TCCMP1
    #[inline(always)]
    pub fn tccmp1(&mut self) -> TCCMP1_W<'_, BMTRGRrs> {
        TCCMP1_W::new(self, 17)
    }
    ///Bit 18 - TCCMP2
    #[inline(always)]
    pub fn tccmp2(&mut self) -> TCCMP2_W<'_, BMTRGRrs> {
        TCCMP2_W::new(self, 18)
    }
    ///Bit 19 - TDRST
    #[inline(always)]
    pub fn tdrst(&mut self) -> TDRST_W<'_, BMTRGRrs> {
        TDRST_W::new(self, 19)
    }
    ///Bit 20 - TDREP
    #[inline(always)]
    pub fn tdrep(&mut self) -> TDREP_W<'_, BMTRGRrs> {
        TDREP_W::new(self, 20)
    }
    ///Bit 21 - TDCMP1
    #[inline(always)]
    pub fn tdcmp1(&mut self) -> TDCMP1_W<'_, BMTRGRrs> {
        TDCMP1_W::new(self, 21)
    }
    ///Bit 22 - TDCMP2
    #[inline(always)]
    pub fn tdcmp2(&mut self) -> TDCMP2_W<'_, BMTRGRrs> {
        TDCMP2_W::new(self, 22)
    }
    ///Bit 23 - TERST
    #[inline(always)]
    pub fn terst(&mut self) -> TERST_W<'_, BMTRGRrs> {
        TERST_W::new(self, 23)
    }
    ///Bit 24 - TEREP
    #[inline(always)]
    pub fn terep(&mut self) -> TEREP_W<'_, BMTRGRrs> {
        TEREP_W::new(self, 24)
    }
    ///Bit 25 - TECMP1
    #[inline(always)]
    pub fn tecmp1(&mut self) -> TECMP1_W<'_, BMTRGRrs> {
        TECMP1_W::new(self, 25)
    }
    ///Bit 26 - TECMP2
    #[inline(always)]
    pub fn tecmp2(&mut self) -> TECMP2_W<'_, BMTRGRrs> {
        TECMP2_W::new(self, 26)
    }
    ///Bit 27 - Timer A period following External Event 7
    #[inline(always)]
    pub fn taeev7(&mut self) -> TAEEV7_W<'_, BMTRGRrs> {
        TAEEV7_W::new(self, 27)
    }
    ///Bit 28 - Timer D period following External Event 8
    #[inline(always)]
    pub fn tdeev8(&mut self) -> TDEEV8_W<'_, BMTRGRrs> {
        TDEEV8_W::new(self, 28)
    }
    ///Bit 29 - External Event 7 (TIMA filters applied)
    #[inline(always)]
    pub fn eev7(&mut self) -> EEV7_W<'_, BMTRGRrs> {
        EEV7_W::new(self, 29)
    }
    ///Bit 30 - External Event 8 (TIMD filters applied)
    #[inline(always)]
    pub fn eev8(&mut self) -> EEV8_W<'_, BMTRGRrs> {
        EEV8_W::new(self, 30)
    }
    ///Bit 31 - OCHPEV
    #[inline(always)]
    pub fn ochpev(&mut self) -> OCHPEV_W<'_, BMTRGRrs> {
        OCHPEV_W::new(self, 31)
    }
}
/**BMTRG

You can [`read`](crate::Reg::read) this register and get [`bmtrgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmtrgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#HRTIM_Common:BMTRGR)*/
pub struct BMTRGRrs;
impl crate::RegisterSpec for BMTRGRrs {
    type Ux = u32;
}
///`read()` method returns [`bmtrgr::R`](R) reader structure
impl crate::Readable for BMTRGRrs {}
///`write(|w| ..)` method takes [`bmtrgr::W`](W) writer structure
impl crate::Writable for BMTRGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BMTRGR to value 0
impl crate::Resettable for BMTRGRrs {}
