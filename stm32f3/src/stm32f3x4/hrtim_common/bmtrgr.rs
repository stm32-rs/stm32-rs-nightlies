#[doc = "Register `BMTRGR` reader"]
pub type R = crate::R<BMTRGRrs>;
#[doc = "Register `BMTRGR` writer"]
pub type W = crate::W<BMTRGRrs>;
#[doc = "SW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SW {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Trigger immediate burst mode operation"]
    Trigger = 1,
}
impl From<SW> for bool {
    #[inline(always)]
    fn from(variant: SW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW` reader - SW"]
pub type SW_R = crate::BitReader<SW>;
impl SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SW {
        match self.bits {
            false => SW::NoEffect,
            true => SW::Trigger,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SW::NoEffect
    }
    #[doc = "Trigger immediate burst mode operation"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == SW::Trigger
    }
}
#[doc = "Field `SW` writer - SW"]
pub type SW_W<'a, REG> = crate::BitWriter<'a, REG, SW>;
impl<'a, REG> SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SW::NoEffect)
    }
    #[doc = "Trigger immediate burst mode operation"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(SW::Trigger)
    }
}
#[doc = "MSTRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTRST {
    #[doc = "0: Master timer reset/roll-over event has no effect"]
    NoEffect = 0,
    #[doc = "1: Master timer reset/roll-over event triggers a burst mode entry"]
    Trigger = 1,
}
impl From<MSTRST> for bool {
    #[inline(always)]
    fn from(variant: MSTRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTRST` reader - MSTRST"]
pub type MSTRST_R = crate::BitReader<MSTRST>;
impl MSTRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSTRST {
        match self.bits {
            false => MSTRST::NoEffect,
            true => MSTRST::Trigger,
        }
    }
    #[doc = "Master timer reset/roll-over event has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTRST::NoEffect
    }
    #[doc = "Master timer reset/roll-over event triggers a burst mode entry"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == MSTRST::Trigger
    }
}
#[doc = "Field `MSTRST` writer - MSTRST"]
pub type MSTRST_W<'a, REG> = crate::BitWriter<'a, REG, MSTRST>;
impl<'a, REG> MSTRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master timer reset/roll-over event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(MSTRST::NoEffect)
    }
    #[doc = "Master timer reset/roll-over event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(MSTRST::Trigger)
    }
}
#[doc = "MSTREP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTREP {
    #[doc = "0: Master timer repetition event has no effect"]
    NoEffect = 0,
    #[doc = "1: Master timer repetition event triggers a burst mode entry"]
    Trigger = 1,
}
impl From<MSTREP> for bool {
    #[inline(always)]
    fn from(variant: MSTREP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTREP` reader - MSTREP"]
pub type MSTREP_R = crate::BitReader<MSTREP>;
impl MSTREP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSTREP {
        match self.bits {
            false => MSTREP::NoEffect,
            true => MSTREP::Trigger,
        }
    }
    #[doc = "Master timer repetition event has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTREP::NoEffect
    }
    #[doc = "Master timer repetition event triggers a burst mode entry"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == MSTREP::Trigger
    }
}
#[doc = "Field `MSTREP` writer - MSTREP"]
pub type MSTREP_W<'a, REG> = crate::BitWriter<'a, REG, MSTREP>;
impl<'a, REG> MSTREP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master timer repetition event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(MSTREP::NoEffect)
    }
    #[doc = "Master timer repetition event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(MSTREP::Trigger)
    }
}
#[doc = "MSTCMP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTCMP1 {
    #[doc = "0: Master timer compare X event has no effect"]
    NoEffect = 0,
    #[doc = "1: Master timer compare X event triggers a burst mode entry"]
    Trigger = 1,
}
impl From<MSTCMP1> for bool {
    #[inline(always)]
    fn from(variant: MSTCMP1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTCMP1` reader - MSTCMP1"]
pub type MSTCMP1_R = crate::BitReader<MSTCMP1>;
impl MSTCMP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSTCMP1 {
        match self.bits {
            false => MSTCMP1::NoEffect,
            true => MSTCMP1::Trigger,
        }
    }
    #[doc = "Master timer compare X event has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTCMP1::NoEffect
    }
    #[doc = "Master timer compare X event triggers a burst mode entry"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == MSTCMP1::Trigger
    }
}
#[doc = "Field `MSTCMP1` writer - MSTCMP1"]
pub type MSTCMP1_W<'a, REG> = crate::BitWriter<'a, REG, MSTCMP1>;
impl<'a, REG> MSTCMP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master timer compare X event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(MSTCMP1::NoEffect)
    }
    #[doc = "Master timer compare X event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(MSTCMP1::Trigger)
    }
}
#[doc = "Field `MSTCMP2` reader - MSTCMP2"]
pub use MSTCMP1_R as MSTCMP2_R;
#[doc = "Field `MSTCMP3` reader - MSTCMP3"]
pub use MSTCMP1_R as MSTCMP3_R;
#[doc = "Field `MSTCMP4` reader - MSTCMP4"]
pub use MSTCMP1_R as MSTCMP4_R;
#[doc = "Field `MSTCMP2` writer - MSTCMP2"]
pub use MSTCMP1_W as MSTCMP2_W;
#[doc = "Field `MSTCMP3` writer - MSTCMP3"]
pub use MSTCMP1_W as MSTCMP3_W;
#[doc = "Field `MSTCMP4` writer - MSTCMP4"]
pub use MSTCMP1_W as MSTCMP4_W;
#[doc = "TARST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TARST {
    #[doc = "0: Timer X reset/roll-over event has no effect"]
    NoEffect = 0,
    #[doc = "1: Timer X reset/roll-over event triggers a burst mode entry"]
    Trigger = 1,
}
impl From<TARST> for bool {
    #[inline(always)]
    fn from(variant: TARST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TARST` reader - TARST"]
pub type TARST_R = crate::BitReader<TARST>;
impl TARST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TARST {
        match self.bits {
            false => TARST::NoEffect,
            true => TARST::Trigger,
        }
    }
    #[doc = "Timer X reset/roll-over event has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TARST::NoEffect
    }
    #[doc = "Timer X reset/roll-over event triggers a burst mode entry"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == TARST::Trigger
    }
}
#[doc = "Field `TARST` writer - TARST"]
pub type TARST_W<'a, REG> = crate::BitWriter<'a, REG, TARST>;
impl<'a, REG> TARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer X reset/roll-over event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TARST::NoEffect)
    }
    #[doc = "Timer X reset/roll-over event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TARST::Trigger)
    }
}
#[doc = "TAREP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAREP {
    #[doc = "0: Timer X repetition event has no effect"]
    NoEffect = 0,
    #[doc = "1: Timer X repetition event triggers a burst mode entry"]
    Trigger = 1,
}
impl From<TAREP> for bool {
    #[inline(always)]
    fn from(variant: TAREP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAREP` reader - TAREP"]
pub type TAREP_R = crate::BitReader<TAREP>;
impl TAREP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAREP {
        match self.bits {
            false => TAREP::NoEffect,
            true => TAREP::Trigger,
        }
    }
    #[doc = "Timer X repetition event has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TAREP::NoEffect
    }
    #[doc = "Timer X repetition event triggers a burst mode entry"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == TAREP::Trigger
    }
}
#[doc = "Field `TAREP` writer - TAREP"]
pub type TAREP_W<'a, REG> = crate::BitWriter<'a, REG, TAREP>;
impl<'a, REG> TAREP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer X repetition event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TAREP::NoEffect)
    }
    #[doc = "Timer X repetition event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TAREP::Trigger)
    }
}
#[doc = "TACMP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TACMP1 {
    #[doc = "0: Timer X compare Y event has no effect"]
    NoEffect = 0,
    #[doc = "1: Timer X compare Y event triggers a burst mode entry"]
    Trigger = 1,
}
impl From<TACMP1> for bool {
    #[inline(always)]
    fn from(variant: TACMP1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TACMP1` reader - TACMP1"]
pub type TACMP1_R = crate::BitReader<TACMP1>;
impl TACMP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TACMP1 {
        match self.bits {
            false => TACMP1::NoEffect,
            true => TACMP1::Trigger,
        }
    }
    #[doc = "Timer X compare Y event has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TACMP1::NoEffect
    }
    #[doc = "Timer X compare Y event triggers a burst mode entry"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == TACMP1::Trigger
    }
}
#[doc = "Field `TACMP1` writer - TACMP1"]
pub type TACMP1_W<'a, REG> = crate::BitWriter<'a, REG, TACMP1>;
impl<'a, REG> TACMP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer X compare Y event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TACMP1::NoEffect)
    }
    #[doc = "Timer X compare Y event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TACMP1::Trigger)
    }
}
#[doc = "Field `TACMP2` reader - TACMP2"]
pub use TACMP1_R as TACMP2_R;
#[doc = "Field `TBCMP1` reader - TBCMP1"]
pub use TACMP1_R as TBCMP1_R;
#[doc = "Field `TBCMP2` reader - TBCMP2"]
pub use TACMP1_R as TBCMP2_R;
#[doc = "Field `TCCMP1` reader - TCCMP1"]
pub use TACMP1_R as TCCMP1_R;
#[doc = "Field `TCCMP2` reader - TCCMP2"]
pub use TACMP1_R as TCCMP2_R;
#[doc = "Field `TDCMP1` reader - TDCMP1"]
pub use TACMP1_R as TDCMP1_R;
#[doc = "Field `TDCMP2` reader - TDCMP2"]
pub use TACMP1_R as TDCMP2_R;
#[doc = "Field `TECMP1` reader - TECMP1"]
pub use TACMP1_R as TECMP1_R;
#[doc = "Field `TECMP2` reader - TECMP2"]
pub use TACMP1_R as TECMP2_R;
#[doc = "Field `TACMP2` writer - TACMP2"]
pub use TACMP1_W as TACMP2_W;
#[doc = "Field `TBCMP1` writer - TBCMP1"]
pub use TACMP1_W as TBCMP1_W;
#[doc = "Field `TBCMP2` writer - TBCMP2"]
pub use TACMP1_W as TBCMP2_W;
#[doc = "Field `TCCMP1` writer - TCCMP1"]
pub use TACMP1_W as TCCMP1_W;
#[doc = "Field `TCCMP2` writer - TCCMP2"]
pub use TACMP1_W as TCCMP2_W;
#[doc = "Field `TDCMP1` writer - TDCMP1"]
pub use TACMP1_W as TDCMP1_W;
#[doc = "Field `TDCMP2` writer - TDCMP2"]
pub use TACMP1_W as TDCMP2_W;
#[doc = "Field `TECMP1` writer - TECMP1"]
pub use TACMP1_W as TECMP1_W;
#[doc = "Field `TECMP2` writer - TECMP2"]
pub use TACMP1_W as TECMP2_W;
#[doc = "Field `TBREP` reader - TBREP"]
pub use TAREP_R as TBREP_R;
#[doc = "Field `TCREP` reader - TCREP"]
pub use TAREP_R as TCREP_R;
#[doc = "Field `TDREP` reader - TDREP"]
pub use TAREP_R as TDREP_R;
#[doc = "Field `TEREP` reader - TEREP"]
pub use TAREP_R as TEREP_R;
#[doc = "Field `TBREP` writer - TBREP"]
pub use TAREP_W as TBREP_W;
#[doc = "Field `TCREP` writer - TCREP"]
pub use TAREP_W as TCREP_W;
#[doc = "Field `TDREP` writer - TDREP"]
pub use TAREP_W as TDREP_W;
#[doc = "Field `TEREP` writer - TEREP"]
pub use TAREP_W as TEREP_W;
#[doc = "Field `TBRST` reader - TBRST"]
pub use TARST_R as TBRST_R;
#[doc = "Field `TCRST` reader - TCRST"]
pub use TARST_R as TCRST_R;
#[doc = "Field `TDRST` reader - TDRST"]
pub use TARST_R as TDRST_R;
#[doc = "Field `TERST` reader - TERST"]
pub use TARST_R as TERST_R;
#[doc = "Field `TBRST` writer - TBRST"]
pub use TARST_W as TBRST_W;
#[doc = "Field `TCRST` writer - TCRST"]
pub use TARST_W as TCRST_W;
#[doc = "Field `TDRST` writer - TDRST"]
pub use TARST_W as TDRST_W;
#[doc = "Field `TERST` writer - TERST"]
pub use TARST_W as TERST_W;
#[doc = "TAEEV7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAEEV7 {
    #[doc = "0: Timer X period following external event Y has no effect"]
    NoEffect = 0,
    #[doc = "1: Timer X period following external event Y triggers a burst mode entry"]
    Trigger = 1,
}
impl From<TAEEV7> for bool {
    #[inline(always)]
    fn from(variant: TAEEV7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAEEV7` reader - TAEEV7"]
pub type TAEEV7_R = crate::BitReader<TAEEV7>;
impl TAEEV7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAEEV7 {
        match self.bits {
            false => TAEEV7::NoEffect,
            true => TAEEV7::Trigger,
        }
    }
    #[doc = "Timer X period following external event Y has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TAEEV7::NoEffect
    }
    #[doc = "Timer X period following external event Y triggers a burst mode entry"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == TAEEV7::Trigger
    }
}
#[doc = "Field `TAEEV7` writer - TAEEV7"]
pub type TAEEV7_W<'a, REG> = crate::BitWriter<'a, REG, TAEEV7>;
impl<'a, REG> TAEEV7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer X period following external event Y has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TAEEV7::NoEffect)
    }
    #[doc = "Timer X period following external event Y triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TAEEV7::Trigger)
    }
}
#[doc = "Field `TDEEV8` reader - TDEEV8"]
pub use TAEEV7_R as TDEEV8_R;
#[doc = "Field `TDEEV8` writer - TDEEV8"]
pub use TAEEV7_W as TDEEV8_W;
#[doc = "EEV7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEV7 {
    #[doc = "0: External event X has no effect"]
    NoEffect = 0,
    #[doc = "1: External event X triggers a burst mode entry"]
    Trigger = 1,
}
impl From<EEV7> for bool {
    #[inline(always)]
    fn from(variant: EEV7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEV7` reader - EEV7"]
pub type EEV7_R = crate::BitReader<EEV7>;
impl EEV7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EEV7 {
        match self.bits {
            false => EEV7::NoEffect,
            true => EEV7::Trigger,
        }
    }
    #[doc = "External event X has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == EEV7::NoEffect
    }
    #[doc = "External event X triggers a burst mode entry"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == EEV7::Trigger
    }
}
#[doc = "Field `EEV7` writer - EEV7"]
pub type EEV7_W<'a, REG> = crate::BitWriter<'a, REG, EEV7>;
impl<'a, REG> EEV7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External event X has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(EEV7::NoEffect)
    }
    #[doc = "External event X triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(EEV7::Trigger)
    }
}
#[doc = "Field `EEV8` reader - EEV8"]
pub use EEV7_R as EEV8_R;
#[doc = "Field `EEV8` writer - EEV8"]
pub use EEV7_W as EEV8_W;
#[doc = "OCHPEV\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCHPEV {
    #[doc = "0: Rising edge on an on-chip event has no effect"]
    NoEffect = 0,
    #[doc = "1: Rising edge on an on-chip event triggers a burst mode entry"]
    Trigger = 1,
}
impl From<OCHPEV> for bool {
    #[inline(always)]
    fn from(variant: OCHPEV) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCHPEV` reader - OCHPEV"]
pub type OCHPEV_R = crate::BitReader<OCHPEV>;
impl OCHPEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OCHPEV {
        match self.bits {
            false => OCHPEV::NoEffect,
            true => OCHPEV::Trigger,
        }
    }
    #[doc = "Rising edge on an on-chip event has no effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == OCHPEV::NoEffect
    }
    #[doc = "Rising edge on an on-chip event triggers a burst mode entry"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == OCHPEV::Trigger
    }
}
#[doc = "Field `OCHPEV` writer - OCHPEV"]
pub type OCHPEV_W<'a, REG> = crate::BitWriter<'a, REG, OCHPEV>;
impl<'a, REG> OCHPEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge on an on-chip event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(OCHPEV::NoEffect)
    }
    #[doc = "Rising edge on an on-chip event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(OCHPEV::Trigger)
    }
}
impl R {
    #[doc = "Bit 0 - SW"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MSTRST"]
    #[inline(always)]
    pub fn mstrst(&self) -> MSTRST_R {
        MSTRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSTREP"]
    #[inline(always)]
    pub fn mstrep(&self) -> MSTREP_R {
        MSTREP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MSTCMP1"]
    #[inline(always)]
    pub fn mstcmp1(&self) -> MSTCMP1_R {
        MSTCMP1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MSTCMP2"]
    #[inline(always)]
    pub fn mstcmp2(&self) -> MSTCMP2_R {
        MSTCMP2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MSTCMP3"]
    #[inline(always)]
    pub fn mstcmp3(&self) -> MSTCMP3_R {
        MSTCMP3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MSTCMP4"]
    #[inline(always)]
    pub fn mstcmp4(&self) -> MSTCMP4_R {
        MSTCMP4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TARST"]
    #[inline(always)]
    pub fn tarst(&self) -> TARST_R {
        TARST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TAREP"]
    #[inline(always)]
    pub fn tarep(&self) -> TAREP_R {
        TAREP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TACMP1"]
    #[inline(always)]
    pub fn tacmp1(&self) -> TACMP1_R {
        TACMP1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TACMP2"]
    #[inline(always)]
    pub fn tacmp2(&self) -> TACMP2_R {
        TACMP2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TBRST"]
    #[inline(always)]
    pub fn tbrst(&self) -> TBRST_R {
        TBRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TBREP"]
    #[inline(always)]
    pub fn tbrep(&self) -> TBREP_R {
        TBREP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TBCMP1"]
    #[inline(always)]
    pub fn tbcmp1(&self) -> TBCMP1_R {
        TBCMP1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TBCMP2"]
    #[inline(always)]
    pub fn tbcmp2(&self) -> TBCMP2_R {
        TBCMP2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TCRST"]
    #[inline(always)]
    pub fn tcrst(&self) -> TCRST_R {
        TCRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TCREP"]
    #[inline(always)]
    pub fn tcrep(&self) -> TCREP_R {
        TCREP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TCCMP1"]
    #[inline(always)]
    pub fn tccmp1(&self) -> TCCMP1_R {
        TCCMP1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TCCMP2"]
    #[inline(always)]
    pub fn tccmp2(&self) -> TCCMP2_R {
        TCCMP2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TDRST"]
    #[inline(always)]
    pub fn tdrst(&self) -> TDRST_R {
        TDRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TDREP"]
    #[inline(always)]
    pub fn tdrep(&self) -> TDREP_R {
        TDREP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TDCMP1"]
    #[inline(always)]
    pub fn tdcmp1(&self) -> TDCMP1_R {
        TDCMP1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TDCMP2"]
    #[inline(always)]
    pub fn tdcmp2(&self) -> TDCMP2_R {
        TDCMP2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TERST"]
    #[inline(always)]
    pub fn terst(&self) -> TERST_R {
        TERST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TEREP"]
    #[inline(always)]
    pub fn terep(&self) -> TEREP_R {
        TEREP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TECMP1"]
    #[inline(always)]
    pub fn tecmp1(&self) -> TECMP1_R {
        TECMP1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TECMP2"]
    #[inline(always)]
    pub fn tecmp2(&self) -> TECMP2_R {
        TECMP2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TAEEV7"]
    #[inline(always)]
    pub fn taeev7(&self) -> TAEEV7_R {
        TAEEV7_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - TDEEV8"]
    #[inline(always)]
    pub fn tdeev8(&self) -> TDEEV8_R {
        TDEEV8_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - EEV7"]
    #[inline(always)]
    pub fn eev7(&self) -> EEV7_R {
        EEV7_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - EEV8"]
    #[inline(always)]
    pub fn eev8(&self) -> EEV8_R {
        EEV8_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - OCHPEV"]
    #[inline(always)]
    pub fn ochpev(&self) -> OCHPEV_R {
        OCHPEV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SW"]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<BMTRGRrs> {
        SW_W::new(self, 0)
    }
    #[doc = "Bit 1 - MSTRST"]
    #[inline(always)]
    #[must_use]
    pub fn mstrst(&mut self) -> MSTRST_W<BMTRGRrs> {
        MSTRST_W::new(self, 1)
    }
    #[doc = "Bit 2 - MSTREP"]
    #[inline(always)]
    #[must_use]
    pub fn mstrep(&mut self) -> MSTREP_W<BMTRGRrs> {
        MSTREP_W::new(self, 2)
    }
    #[doc = "Bit 3 - MSTCMP1"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp1(&mut self) -> MSTCMP1_W<BMTRGRrs> {
        MSTCMP1_W::new(self, 3)
    }
    #[doc = "Bit 4 - MSTCMP2"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp2(&mut self) -> MSTCMP2_W<BMTRGRrs> {
        MSTCMP2_W::new(self, 4)
    }
    #[doc = "Bit 5 - MSTCMP3"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp3(&mut self) -> MSTCMP3_W<BMTRGRrs> {
        MSTCMP3_W::new(self, 5)
    }
    #[doc = "Bit 6 - MSTCMP4"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp4(&mut self) -> MSTCMP4_W<BMTRGRrs> {
        MSTCMP4_W::new(self, 6)
    }
    #[doc = "Bit 7 - TARST"]
    #[inline(always)]
    #[must_use]
    pub fn tarst(&mut self) -> TARST_W<BMTRGRrs> {
        TARST_W::new(self, 7)
    }
    #[doc = "Bit 8 - TAREP"]
    #[inline(always)]
    #[must_use]
    pub fn tarep(&mut self) -> TAREP_W<BMTRGRrs> {
        TAREP_W::new(self, 8)
    }
    #[doc = "Bit 9 - TACMP1"]
    #[inline(always)]
    #[must_use]
    pub fn tacmp1(&mut self) -> TACMP1_W<BMTRGRrs> {
        TACMP1_W::new(self, 9)
    }
    #[doc = "Bit 10 - TACMP2"]
    #[inline(always)]
    #[must_use]
    pub fn tacmp2(&mut self) -> TACMP2_W<BMTRGRrs> {
        TACMP2_W::new(self, 10)
    }
    #[doc = "Bit 11 - TBRST"]
    #[inline(always)]
    #[must_use]
    pub fn tbrst(&mut self) -> TBRST_W<BMTRGRrs> {
        TBRST_W::new(self, 11)
    }
    #[doc = "Bit 12 - TBREP"]
    #[inline(always)]
    #[must_use]
    pub fn tbrep(&mut self) -> TBREP_W<BMTRGRrs> {
        TBREP_W::new(self, 12)
    }
    #[doc = "Bit 13 - TBCMP1"]
    #[inline(always)]
    #[must_use]
    pub fn tbcmp1(&mut self) -> TBCMP1_W<BMTRGRrs> {
        TBCMP1_W::new(self, 13)
    }
    #[doc = "Bit 14 - TBCMP2"]
    #[inline(always)]
    #[must_use]
    pub fn tbcmp2(&mut self) -> TBCMP2_W<BMTRGRrs> {
        TBCMP2_W::new(self, 14)
    }
    #[doc = "Bit 15 - TCRST"]
    #[inline(always)]
    #[must_use]
    pub fn tcrst(&mut self) -> TCRST_W<BMTRGRrs> {
        TCRST_W::new(self, 15)
    }
    #[doc = "Bit 16 - TCREP"]
    #[inline(always)]
    #[must_use]
    pub fn tcrep(&mut self) -> TCREP_W<BMTRGRrs> {
        TCREP_W::new(self, 16)
    }
    #[doc = "Bit 17 - TCCMP1"]
    #[inline(always)]
    #[must_use]
    pub fn tccmp1(&mut self) -> TCCMP1_W<BMTRGRrs> {
        TCCMP1_W::new(self, 17)
    }
    #[doc = "Bit 18 - TCCMP2"]
    #[inline(always)]
    #[must_use]
    pub fn tccmp2(&mut self) -> TCCMP2_W<BMTRGRrs> {
        TCCMP2_W::new(self, 18)
    }
    #[doc = "Bit 19 - TDRST"]
    #[inline(always)]
    #[must_use]
    pub fn tdrst(&mut self) -> TDRST_W<BMTRGRrs> {
        TDRST_W::new(self, 19)
    }
    #[doc = "Bit 20 - TDREP"]
    #[inline(always)]
    #[must_use]
    pub fn tdrep(&mut self) -> TDREP_W<BMTRGRrs> {
        TDREP_W::new(self, 20)
    }
    #[doc = "Bit 21 - TDCMP1"]
    #[inline(always)]
    #[must_use]
    pub fn tdcmp1(&mut self) -> TDCMP1_W<BMTRGRrs> {
        TDCMP1_W::new(self, 21)
    }
    #[doc = "Bit 22 - TDCMP2"]
    #[inline(always)]
    #[must_use]
    pub fn tdcmp2(&mut self) -> TDCMP2_W<BMTRGRrs> {
        TDCMP2_W::new(self, 22)
    }
    #[doc = "Bit 23 - TERST"]
    #[inline(always)]
    #[must_use]
    pub fn terst(&mut self) -> TERST_W<BMTRGRrs> {
        TERST_W::new(self, 23)
    }
    #[doc = "Bit 24 - TEREP"]
    #[inline(always)]
    #[must_use]
    pub fn terep(&mut self) -> TEREP_W<BMTRGRrs> {
        TEREP_W::new(self, 24)
    }
    #[doc = "Bit 25 - TECMP1"]
    #[inline(always)]
    #[must_use]
    pub fn tecmp1(&mut self) -> TECMP1_W<BMTRGRrs> {
        TECMP1_W::new(self, 25)
    }
    #[doc = "Bit 26 - TECMP2"]
    #[inline(always)]
    #[must_use]
    pub fn tecmp2(&mut self) -> TECMP2_W<BMTRGRrs> {
        TECMP2_W::new(self, 26)
    }
    #[doc = "Bit 27 - TAEEV7"]
    #[inline(always)]
    #[must_use]
    pub fn taeev7(&mut self) -> TAEEV7_W<BMTRGRrs> {
        TAEEV7_W::new(self, 27)
    }
    #[doc = "Bit 28 - TDEEV8"]
    #[inline(always)]
    #[must_use]
    pub fn tdeev8(&mut self) -> TDEEV8_W<BMTRGRrs> {
        TDEEV8_W::new(self, 28)
    }
    #[doc = "Bit 29 - EEV7"]
    #[inline(always)]
    #[must_use]
    pub fn eev7(&mut self) -> EEV7_W<BMTRGRrs> {
        EEV7_W::new(self, 29)
    }
    #[doc = "Bit 30 - EEV8"]
    #[inline(always)]
    #[must_use]
    pub fn eev8(&mut self) -> EEV8_W<BMTRGRrs> {
        EEV8_W::new(self, 30)
    }
    #[doc = "Bit 31 - OCHPEV"]
    #[inline(always)]
    #[must_use]
    pub fn ochpev(&mut self) -> OCHPEV_W<BMTRGRrs> {
        OCHPEV_W::new(self, 31)
    }
}
#[doc = "BMTRGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmtrgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmtrgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BMTRGRrs;
impl crate::RegisterSpec for BMTRGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmtrgr::R`](R) reader structure"]
impl crate::Readable for BMTRGRrs {}
#[doc = "`write(|w| ..)` method takes [`bmtrgr::W`](W) writer structure"]
impl crate::Writable for BMTRGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BMTRGR to value 0"]
impl crate::Resettable for BMTRGRrs {
    const RESET_VALUE: u32 = 0;
}
