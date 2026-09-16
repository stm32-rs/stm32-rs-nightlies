///Register `ADC2R` reader
pub type R = crate::R<ADC2Rrs>;
///Register `ADC2R` writer
pub type W = crate::W<ADC2Rrs>;
/**ADC trigger 2 on Master Compare %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MC1 {
    ///0: No generation of ADC trigger on master compare event
    Disabled = 0,
    ///1: Generation of ADC trigger on master compare event
    Enabled = 1,
}
impl From<MC1> for bool {
    #[inline(always)]
    fn from(variant: MC1) -> Self {
        variant as u8 != 0
    }
}
///Field `MC(1-4)` reader - ADC trigger 2 on Master Compare %s
pub type MC_R = crate::BitReader<MC1>;
impl MC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MC1 {
        match self.bits {
            false => MC1::Disabled,
            true => MC1::Enabled,
        }
    }
    ///No generation of ADC trigger on master compare event
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MC1::Disabled
    }
    ///Generation of ADC trigger on master compare event
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MC1::Enabled
    }
}
///Field `MC(1-4)` writer - ADC trigger 2 on Master Compare %s
pub type MC_W<'a, REG> = crate::BitWriter<'a, REG, MC1>;
impl<'a, REG> MC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No generation of ADC trigger on master compare event
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MC1::Disabled)
    }
    ///Generation of ADC trigger on master compare event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MC1::Enabled)
    }
}
/**ADC trigger 2 on Master Period

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPER {
    ///0: No generation of ADC trigger on timer period event
    Disabled = 0,
    ///1: Generation of ADC trigger on timer period event
    Enabled = 1,
}
impl From<MPER> for bool {
    #[inline(always)]
    fn from(variant: MPER) -> Self {
        variant as u8 != 0
    }
}
///Field `MPER` reader - ADC trigger 2 on Master Period
pub type MPER_R = crate::BitReader<MPER>;
impl MPER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MPER {
        match self.bits {
            false => MPER::Disabled,
            true => MPER::Enabled,
        }
    }
    ///No generation of ADC trigger on timer period event
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MPER::Disabled
    }
    ///Generation of ADC trigger on timer period event
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MPER::Enabled
    }
}
///Field `MPER` writer - ADC trigger 2 on Master Period
pub type MPER_W<'a, REG> = crate::BitWriter<'a, REG, MPER>;
impl<'a, REG> MPER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No generation of ADC trigger on timer period event
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MPER::Disabled)
    }
    ///Generation of ADC trigger on timer period event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MPER::Enabled)
    }
}
/**ADC trigger 2 on External Event %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEV6 {
    ///0: No generation of ADC trigger on external event
    Disabled = 0,
    ///1: Generation of ADC trigger on external event
    Enabled = 1,
}
impl From<EEV6> for bool {
    #[inline(always)]
    fn from(variant: EEV6) -> Self {
        variant as u8 != 0
    }
}
///Field `EEV(6-10)` reader - ADC trigger 2 on External Event %s
pub type EEV_R = crate::BitReader<EEV6>;
impl EEV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EEV6 {
        match self.bits {
            false => EEV6::Disabled,
            true => EEV6::Enabled,
        }
    }
    ///No generation of ADC trigger on external event
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EEV6::Disabled
    }
    ///Generation of ADC trigger on external event
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EEV6::Enabled
    }
}
///Field `EEV(6-10)` writer - ADC trigger 2 on External Event %s
pub type EEV_W<'a, REG> = crate::BitWriter<'a, REG, EEV6>;
impl<'a, REG> EEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No generation of ADC trigger on external event
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EEV6::Disabled)
    }
    ///Generation of ADC trigger on external event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EEV6::Enabled)
    }
}
/**ADC trigger 2 on Timer A compare 2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AC2 {
    ///0: No generation of ADC trigger on timer compare event
    Disabled = 0,
    ///1: Generation of ADC trigger on timer compare event
    Enabled = 1,
}
impl From<AC2> for bool {
    #[inline(always)]
    fn from(variant: AC2) -> Self {
        variant as u8 != 0
    }
}
///Field `AC2` reader - ADC trigger 2 on Timer A compare 2
pub type AC2_R = crate::BitReader<AC2>;
impl AC2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AC2 {
        match self.bits {
            false => AC2::Disabled,
            true => AC2::Enabled,
        }
    }
    ///No generation of ADC trigger on timer compare event
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AC2::Disabled
    }
    ///Generation of ADC trigger on timer compare event
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AC2::Enabled
    }
}
///Field `AC2` writer - ADC trigger 2 on Timer A compare 2
pub type AC2_W<'a, REG> = crate::BitWriter<'a, REG, AC2>;
impl<'a, REG> AC2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No generation of ADC trigger on timer compare event
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AC2::Disabled)
    }
    ///Generation of ADC trigger on timer compare event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AC2::Enabled)
    }
}
///Field `AC3` reader - ADC trigger 2 on Timer A compare 3
pub use AC2_R as AC3_R;
///Field `AC4` reader - ADC trigger 2 on Timer A compare 4
pub use AC2_R as AC4_R;
///Field `BC2` reader - ADC trigger 2 on Timer B compare 2
pub use AC2_R as BC2_R;
///Field `BC3` reader - ADC trigger 2 on Timer B compare 3
pub use AC2_R as BC3_R;
///Field `BC4` reader - ADC trigger 2 on Timer B compare 4
pub use AC2_R as BC4_R;
///Field `CC2` reader - ADC trigger 2 on Timer C compare 2
pub use AC2_R as CC2_R;
///Field `CC3` reader - ADC trigger 2 on Timer C compare 3
pub use AC2_R as CC3_R;
///Field `CC4` reader - ADC trigger 2 on Timer C compare 4
pub use AC2_R as CC4_R;
///Field `AC3` writer - ADC trigger 2 on Timer A compare 3
pub use AC2_W as AC3_W;
///Field `AC4` writer - ADC trigger 2 on Timer A compare 4
pub use AC2_W as AC4_W;
///Field `BC2` writer - ADC trigger 2 on Timer B compare 2
pub use AC2_W as BC2_W;
///Field `BC3` writer - ADC trigger 2 on Timer B compare 3
pub use AC2_W as BC3_W;
///Field `BC4` writer - ADC trigger 2 on Timer B compare 4
pub use AC2_W as BC4_W;
///Field `CC2` writer - ADC trigger 2 on Timer C compare 2
pub use AC2_W as CC2_W;
///Field `CC3` writer - ADC trigger 2 on Timer C compare 3
pub use AC2_W as CC3_W;
///Field `CC4` writer - ADC trigger 2 on Timer C compare 4
pub use AC2_W as CC4_W;
///Field `APER` reader - ADC trigger 2 on Timer A Period
pub use MPER_R as APER_R;
///Field `BPER` reader - ADC trigger 2 on Timer B Period
pub use MPER_R as BPER_R;
///Field `CPER` reader - ADC trigger 2 on Timer C Period
pub use MPER_R as CPER_R;
///Field `APER` writer - ADC trigger 2 on Timer A Period
pub use MPER_W as APER_W;
///Field `BPER` writer - ADC trigger 2 on Timer B Period
pub use MPER_W as BPER_W;
///Field `CPER` writer - ADC trigger 2 on Timer C Period
pub use MPER_W as CPER_W;
/**ADC trigger 2 on Timer C Reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRST {
    ///0: No generation of ADC trigger on timer reset and roll-over
    Disabled = 0,
    ///1: Generation of ADC trigger on timer reset and roll-over
    Enabled = 1,
}
impl From<CRST> for bool {
    #[inline(always)]
    fn from(variant: CRST) -> Self {
        variant as u8 != 0
    }
}
///Field `CRST` reader - ADC trigger 2 on Timer C Reset
pub type CRST_R = crate::BitReader<CRST>;
impl CRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRST {
        match self.bits {
            false => CRST::Disabled,
            true => CRST::Enabled,
        }
    }
    ///No generation of ADC trigger on timer reset and roll-over
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRST::Disabled
    }
    ///Generation of ADC trigger on timer reset and roll-over
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRST::Enabled
    }
}
///Field `CRST` writer - ADC trigger 2 on Timer C Reset
pub type CRST_W<'a, REG> = crate::BitWriter<'a, REG, CRST>;
impl<'a, REG> CRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No generation of ADC trigger on timer reset and roll-over
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRST::Disabled)
    }
    ///Generation of ADC trigger on timer reset and roll-over
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRST::Enabled)
    }
}
///Field `DC2` reader - ADC trigger 2 on Timer D compare 2
pub use AC2_R as DC2_R;
///Field `DC3` reader - ADC trigger 2 on Timer D compare 3
pub use AC2_R as DC3_R;
///Field `DC4` reader - ADC trigger 2 on Timer D compare 4
pub use AC2_R as DC4_R;
///Field `EC2` reader - ADC trigger 2 on Timer E compare 2
pub use AC2_R as EC2_R;
///Field `EC3` reader - ADC trigger 2 on Timer E compare 3
pub use AC2_R as EC3_R;
///Field `EC4` reader - ADC trigger 2 on Timer E compare 4
pub use AC2_R as EC4_R;
///Field `DC2` writer - ADC trigger 2 on Timer D compare 2
pub use AC2_W as DC2_W;
///Field `DC3` writer - ADC trigger 2 on Timer D compare 3
pub use AC2_W as DC3_W;
///Field `DC4` writer - ADC trigger 2 on Timer D compare 4
pub use AC2_W as DC4_W;
///Field `EC2` writer - ADC trigger 2 on Timer E compare 2
pub use AC2_W as EC2_W;
///Field `EC3` writer - ADC trigger 2 on Timer E compare 3
pub use AC2_W as EC3_W;
///Field `EC4` writer - ADC trigger 2 on Timer E compare 4
pub use AC2_W as EC4_W;
///Field `DRST` reader - ADC trigger 2 on Timer D Reset
pub use CRST_R as DRST_R;
///Field `ERST` reader - ADC trigger 2 on Timer E Reset
pub use CRST_R as ERST_R;
///Field `DRST` writer - ADC trigger 2 on Timer D Reset
pub use CRST_W as DRST_W;
///Field `ERST` writer - ADC trigger 2 on Timer E Reset
pub use CRST_W as ERST_W;
///Field `DPER` reader - ADC trigger 2 on Timer D Period
pub use MPER_R as DPER_R;
///Field `DPER` writer - ADC trigger 2 on Timer D Period
pub use MPER_W as DPER_W;
impl R {
    ///ADC trigger 2 on Master Compare (1-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MC1` field.</div>
    #[inline(always)]
    pub fn mc(&self, n: u8) -> MC_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        MC_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///ADC trigger 2 on Master Compare (1-4)
    #[inline(always)]
    pub fn mc_iter(&self) -> impl Iterator<Item = MC_R> + '_ {
        (0..4).map(move |n| MC_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - ADC trigger 2 on Master Compare 1
    #[inline(always)]
    pub fn mc1(&self) -> MC_R {
        MC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ADC trigger 2 on Master Compare 2
    #[inline(always)]
    pub fn mc2(&self) -> MC_R {
        MC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ADC trigger 2 on Master Compare 3
    #[inline(always)]
    pub fn mc3(&self) -> MC_R {
        MC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ADC trigger 2 on Master Compare 4
    #[inline(always)]
    pub fn mc4(&self) -> MC_R {
        MC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ADC trigger 2 on Master Period
    #[inline(always)]
    pub fn mper(&self) -> MPER_R {
        MPER_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///ADC trigger 2 on External Event (6-10)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EEV6` field.</div>
    #[inline(always)]
    pub fn eev(&self, n: u8) -> EEV_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        EEV_R::new(((self.bits >> (n + 5)) & 1) != 0)
    }
    ///Iterator for array of:
    ///ADC trigger 2 on External Event (6-10)
    #[inline(always)]
    pub fn eev_iter(&self) -> impl Iterator<Item = EEV_R> + '_ {
        (0..5).map(move |n| EEV_R::new(((self.bits >> (n + 5)) & 1) != 0))
    }
    ///Bit 5 - ADC trigger 2 on External Event 6
    #[inline(always)]
    pub fn eev6(&self) -> EEV_R {
        EEV_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - ADC trigger 2 on External Event 7
    #[inline(always)]
    pub fn eev7(&self) -> EEV_R {
        EEV_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - ADC trigger 2 on External Event 8
    #[inline(always)]
    pub fn eev8(&self) -> EEV_R {
        EEV_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ADC trigger 2 on External Event 9
    #[inline(always)]
    pub fn eev9(&self) -> EEV_R {
        EEV_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ADC trigger 2 on External Event 10
    #[inline(always)]
    pub fn eev10(&self) -> EEV_R {
        EEV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ADC trigger 2 on Timer A compare 2
    #[inline(always)]
    pub fn ac2(&self) -> AC2_R {
        AC2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - ADC trigger 2 on Timer A compare 3
    #[inline(always)]
    pub fn ac3(&self) -> AC3_R {
        AC3_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - ADC trigger 2 on Timer A compare 4
    #[inline(always)]
    pub fn ac4(&self) -> AC4_R {
        AC4_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ADC trigger 2 on Timer A Period
    #[inline(always)]
    pub fn aper(&self) -> APER_R {
        APER_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ADC trigger 2 on Timer B compare 2
    #[inline(always)]
    pub fn bc2(&self) -> BC2_R {
        BC2_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - ADC trigger 2 on Timer B compare 3
    #[inline(always)]
    pub fn bc3(&self) -> BC3_R {
        BC3_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - ADC trigger 2 on Timer B compare 4
    #[inline(always)]
    pub fn bc4(&self) -> BC4_R {
        BC4_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ADC trigger 2 on Timer B Period
    #[inline(always)]
    pub fn bper(&self) -> BPER_R {
        BPER_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ADC trigger 2 on Timer C compare 2
    #[inline(always)]
    pub fn cc2(&self) -> CC2_R {
        CC2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - ADC trigger 2 on Timer C compare 3
    #[inline(always)]
    pub fn cc3(&self) -> CC3_R {
        CC3_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - ADC trigger 2 on Timer C compare 4
    #[inline(always)]
    pub fn cc4(&self) -> CC4_R {
        CC4_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ADC trigger 2 on Timer C Period
    #[inline(always)]
    pub fn cper(&self) -> CPER_R {
        CPER_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ADC trigger 2 on Timer C Reset
    #[inline(always)]
    pub fn crst(&self) -> CRST_R {
        CRST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ADC trigger 2 on Timer D compare 2
    #[inline(always)]
    pub fn dc2(&self) -> DC2_R {
        DC2_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - ADC trigger 2 on Timer D compare 3
    #[inline(always)]
    pub fn dc3(&self) -> DC3_R {
        DC3_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - ADC trigger 2 on Timer D compare 4
    #[inline(always)]
    pub fn dc4(&self) -> DC4_R {
        DC4_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - ADC trigger 2 on Timer D Period
    #[inline(always)]
    pub fn dper(&self) -> DPER_R {
        DPER_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - ADC trigger 2 on Timer D Reset
    #[inline(always)]
    pub fn drst(&self) -> DRST_R {
        DRST_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - ADC trigger 2 on Timer E compare 2
    #[inline(always)]
    pub fn ec2(&self) -> EC2_R {
        EC2_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - ADC trigger 2 on Timer E compare 3
    #[inline(always)]
    pub fn ec3(&self) -> EC3_R {
        EC3_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - ADC trigger 2 on Timer E compare 4
    #[inline(always)]
    pub fn ec4(&self) -> EC4_R {
        EC4_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - ADC trigger 2 on Timer E Reset
    #[inline(always)]
    pub fn erst(&self) -> ERST_R {
        ERST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC2R")
            .field("crst", &self.crst())
            .field("erst", &self.erst())
            .field("ac2", &self.ac2())
            .field("ec4", &self.ec4())
            .field("ec3", &self.ec3())
            .field("ec2", &self.ec2())
            .field("drst", &self.drst())
            .field("mper", &self.mper())
            .field("dper", &self.dper())
            .field("dc4", &self.dc4())
            .field("dc3", &self.dc3())
            .field("dc2", &self.dc2())
            .field("cper", &self.cper())
            .field("cc4", &self.cc4())
            .field("cc3", &self.cc3())
            .field("cc2", &self.cc2())
            .field("bper", &self.bper())
            .field("bc4", &self.bc4())
            .field("bc3", &self.bc3())
            .field("bc2", &self.bc2())
            .field("aper", &self.aper())
            .field("ac4", &self.ac4())
            .field("ac3", &self.ac3())
            .field("eev6", &self.eev6())
            .field("eev7", &self.eev7())
            .field("eev8", &self.eev8())
            .field("eev9", &self.eev9())
            .field("eev10", &self.eev10())
            .field("mc1", &self.mc1())
            .field("mc2", &self.mc2())
            .field("mc3", &self.mc3())
            .field("mc4", &self.mc4())
            .finish()
    }
}
impl W {
    ///ADC trigger 2 on Master Compare (1-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MC1` field.</div>
    #[inline(always)]
    pub fn mc(&mut self, n: u8) -> MC_W<'_, ADC2Rrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        MC_W::new(self, n)
    }
    ///Bit 0 - ADC trigger 2 on Master Compare 1
    #[inline(always)]
    pub fn mc1(&mut self) -> MC_W<'_, ADC2Rrs> {
        MC_W::new(self, 0)
    }
    ///Bit 1 - ADC trigger 2 on Master Compare 2
    #[inline(always)]
    pub fn mc2(&mut self) -> MC_W<'_, ADC2Rrs> {
        MC_W::new(self, 1)
    }
    ///Bit 2 - ADC trigger 2 on Master Compare 3
    #[inline(always)]
    pub fn mc3(&mut self) -> MC_W<'_, ADC2Rrs> {
        MC_W::new(self, 2)
    }
    ///Bit 3 - ADC trigger 2 on Master Compare 4
    #[inline(always)]
    pub fn mc4(&mut self) -> MC_W<'_, ADC2Rrs> {
        MC_W::new(self, 3)
    }
    ///Bit 4 - ADC trigger 2 on Master Period
    #[inline(always)]
    pub fn mper(&mut self) -> MPER_W<'_, ADC2Rrs> {
        MPER_W::new(self, 4)
    }
    ///ADC trigger 2 on External Event (6-10)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EEV6` field.</div>
    #[inline(always)]
    pub fn eev(&mut self, n: u8) -> EEV_W<'_, ADC2Rrs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        EEV_W::new(self, n + 5)
    }
    ///Bit 5 - ADC trigger 2 on External Event 6
    #[inline(always)]
    pub fn eev6(&mut self) -> EEV_W<'_, ADC2Rrs> {
        EEV_W::new(self, 5)
    }
    ///Bit 6 - ADC trigger 2 on External Event 7
    #[inline(always)]
    pub fn eev7(&mut self) -> EEV_W<'_, ADC2Rrs> {
        EEV_W::new(self, 6)
    }
    ///Bit 7 - ADC trigger 2 on External Event 8
    #[inline(always)]
    pub fn eev8(&mut self) -> EEV_W<'_, ADC2Rrs> {
        EEV_W::new(self, 7)
    }
    ///Bit 8 - ADC trigger 2 on External Event 9
    #[inline(always)]
    pub fn eev9(&mut self) -> EEV_W<'_, ADC2Rrs> {
        EEV_W::new(self, 8)
    }
    ///Bit 9 - ADC trigger 2 on External Event 10
    #[inline(always)]
    pub fn eev10(&mut self) -> EEV_W<'_, ADC2Rrs> {
        EEV_W::new(self, 9)
    }
    ///Bit 10 - ADC trigger 2 on Timer A compare 2
    #[inline(always)]
    pub fn ac2(&mut self) -> AC2_W<'_, ADC2Rrs> {
        AC2_W::new(self, 10)
    }
    ///Bit 11 - ADC trigger 2 on Timer A compare 3
    #[inline(always)]
    pub fn ac3(&mut self) -> AC3_W<'_, ADC2Rrs> {
        AC3_W::new(self, 11)
    }
    ///Bit 12 - ADC trigger 2 on Timer A compare 4
    #[inline(always)]
    pub fn ac4(&mut self) -> AC4_W<'_, ADC2Rrs> {
        AC4_W::new(self, 12)
    }
    ///Bit 13 - ADC trigger 2 on Timer A Period
    #[inline(always)]
    pub fn aper(&mut self) -> APER_W<'_, ADC2Rrs> {
        APER_W::new(self, 13)
    }
    ///Bit 14 - ADC trigger 2 on Timer B compare 2
    #[inline(always)]
    pub fn bc2(&mut self) -> BC2_W<'_, ADC2Rrs> {
        BC2_W::new(self, 14)
    }
    ///Bit 15 - ADC trigger 2 on Timer B compare 3
    #[inline(always)]
    pub fn bc3(&mut self) -> BC3_W<'_, ADC2Rrs> {
        BC3_W::new(self, 15)
    }
    ///Bit 16 - ADC trigger 2 on Timer B compare 4
    #[inline(always)]
    pub fn bc4(&mut self) -> BC4_W<'_, ADC2Rrs> {
        BC4_W::new(self, 16)
    }
    ///Bit 17 - ADC trigger 2 on Timer B Period
    #[inline(always)]
    pub fn bper(&mut self) -> BPER_W<'_, ADC2Rrs> {
        BPER_W::new(self, 17)
    }
    ///Bit 18 - ADC trigger 2 on Timer C compare 2
    #[inline(always)]
    pub fn cc2(&mut self) -> CC2_W<'_, ADC2Rrs> {
        CC2_W::new(self, 18)
    }
    ///Bit 19 - ADC trigger 2 on Timer C compare 3
    #[inline(always)]
    pub fn cc3(&mut self) -> CC3_W<'_, ADC2Rrs> {
        CC3_W::new(self, 19)
    }
    ///Bit 20 - ADC trigger 2 on Timer C compare 4
    #[inline(always)]
    pub fn cc4(&mut self) -> CC4_W<'_, ADC2Rrs> {
        CC4_W::new(self, 20)
    }
    ///Bit 21 - ADC trigger 2 on Timer C Period
    #[inline(always)]
    pub fn cper(&mut self) -> CPER_W<'_, ADC2Rrs> {
        CPER_W::new(self, 21)
    }
    ///Bit 22 - ADC trigger 2 on Timer C Reset
    #[inline(always)]
    pub fn crst(&mut self) -> CRST_W<'_, ADC2Rrs> {
        CRST_W::new(self, 22)
    }
    ///Bit 23 - ADC trigger 2 on Timer D compare 2
    #[inline(always)]
    pub fn dc2(&mut self) -> DC2_W<'_, ADC2Rrs> {
        DC2_W::new(self, 23)
    }
    ///Bit 24 - ADC trigger 2 on Timer D compare 3
    #[inline(always)]
    pub fn dc3(&mut self) -> DC3_W<'_, ADC2Rrs> {
        DC3_W::new(self, 24)
    }
    ///Bit 25 - ADC trigger 2 on Timer D compare 4
    #[inline(always)]
    pub fn dc4(&mut self) -> DC4_W<'_, ADC2Rrs> {
        DC4_W::new(self, 25)
    }
    ///Bit 26 - ADC trigger 2 on Timer D Period
    #[inline(always)]
    pub fn dper(&mut self) -> DPER_W<'_, ADC2Rrs> {
        DPER_W::new(self, 26)
    }
    ///Bit 27 - ADC trigger 2 on Timer D Reset
    #[inline(always)]
    pub fn drst(&mut self) -> DRST_W<'_, ADC2Rrs> {
        DRST_W::new(self, 27)
    }
    ///Bit 28 - ADC trigger 2 on Timer E compare 2
    #[inline(always)]
    pub fn ec2(&mut self) -> EC2_W<'_, ADC2Rrs> {
        EC2_W::new(self, 28)
    }
    ///Bit 29 - ADC trigger 2 on Timer E compare 3
    #[inline(always)]
    pub fn ec3(&mut self) -> EC3_W<'_, ADC2Rrs> {
        EC3_W::new(self, 29)
    }
    ///Bit 30 - ADC trigger 2 on Timer E compare 4
    #[inline(always)]
    pub fn ec4(&mut self) -> EC4_W<'_, ADC2Rrs> {
        EC4_W::new(self, 30)
    }
    ///Bit 31 - ADC trigger 2 on Timer E Reset
    #[inline(always)]
    pub fn erst(&mut self) -> ERST_W<'_, ADC2Rrs> {
        ERST_W::new(self, 31)
    }
}
/**ADC Trigger 2 Register

You can [`read`](crate::Reg::read) this register and get [`adc2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#HRTIM_Common:ADC2R)*/
pub struct ADC2Rrs;
impl crate::RegisterSpec for ADC2Rrs {
    type Ux = u32;
}
///`read()` method returns [`adc2r::R`](R) reader structure
impl crate::Readable for ADC2Rrs {}
///`write(|w| ..)` method takes [`adc2r::W`](W) writer structure
impl crate::Writable for ADC2Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADC2R to value 0
impl crate::Resettable for ADC2Rrs {}
