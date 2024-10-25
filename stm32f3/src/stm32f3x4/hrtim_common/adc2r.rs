///Register `ADC2R` reader
pub type R = crate::R<ADC2Rrs>;
///Register `ADC2R` writer
pub type W = crate::W<ADC2Rrs>;
/**ADC trigger 2 on Master Compare 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD2MC1 {
    ///0: No generation of ADC trigger on master compare event
    Disabled = 0,
    ///1: Generation of ADC trigger on master compare event
    Enabled = 1,
}
impl From<AD2MC1> for bool {
    #[inline(always)]
    fn from(variant: AD2MC1) -> Self {
        variant as u8 != 0
    }
}
///Field `AD2MC1` reader - ADC trigger 2 on Master Compare 1
pub type AD2MC1_R = crate::BitReader<AD2MC1>;
impl AD2MC1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AD2MC1 {
        match self.bits {
            false => AD2MC1::Disabled,
            true => AD2MC1::Enabled,
        }
    }
    ///No generation of ADC trigger on master compare event
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD2MC1::Disabled
    }
    ///Generation of ADC trigger on master compare event
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD2MC1::Enabled
    }
}
///Field `AD2MC1` writer - ADC trigger 2 on Master Compare 1
pub type AD2MC1_W<'a, REG> = crate::BitWriter<'a, REG, AD2MC1>;
impl<'a, REG> AD2MC1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No generation of ADC trigger on master compare event
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AD2MC1::Disabled)
    }
    ///Generation of ADC trigger on master compare event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AD2MC1::Enabled)
    }
}
///Field `AD2MC2` reader - ADC trigger 2 on Master Compare 2
pub use AD2MC1_R as AD2MC2_R;
///Field `AD2MC3` reader - ADC trigger 2 on Master Compare 3
pub use AD2MC1_R as AD2MC3_R;
///Field `AD2MC4` reader - ADC trigger 2 on Master Compare 4
pub use AD2MC1_R as AD2MC4_R;
///Field `AD2MC2` writer - ADC trigger 2 on Master Compare 2
pub use AD2MC1_W as AD2MC2_W;
///Field `AD2MC3` writer - ADC trigger 2 on Master Compare 3
pub use AD2MC1_W as AD2MC3_W;
///Field `AD2MC4` writer - ADC trigger 2 on Master Compare 4
pub use AD2MC1_W as AD2MC4_W;
/**ADC trigger 2 on Master Period

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD2MPER {
    ///0: No generation of ADC trigger on timer period event
    Disabled = 0,
    ///1: Generation of ADC trigger on timer period event
    Enabled = 1,
}
impl From<AD2MPER> for bool {
    #[inline(always)]
    fn from(variant: AD2MPER) -> Self {
        variant as u8 != 0
    }
}
///Field `AD2MPER` reader - ADC trigger 2 on Master Period
pub type AD2MPER_R = crate::BitReader<AD2MPER>;
impl AD2MPER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AD2MPER {
        match self.bits {
            false => AD2MPER::Disabled,
            true => AD2MPER::Enabled,
        }
    }
    ///No generation of ADC trigger on timer period event
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD2MPER::Disabled
    }
    ///Generation of ADC trigger on timer period event
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD2MPER::Enabled
    }
}
///Field `AD2MPER` writer - ADC trigger 2 on Master Period
pub type AD2MPER_W<'a, REG> = crate::BitWriter<'a, REG, AD2MPER>;
impl<'a, REG> AD2MPER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No generation of ADC trigger on timer period event
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AD2MPER::Disabled)
    }
    ///Generation of ADC trigger on timer period event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AD2MPER::Enabled)
    }
}
/**ADC trigger 2 on External Event 6

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD2EEV6 {
    ///0: No generation of ADC trigger on external event
    Disabled = 0,
    ///1: Generation of ADC trigger on external event
    Enabled = 1,
}
impl From<AD2EEV6> for bool {
    #[inline(always)]
    fn from(variant: AD2EEV6) -> Self {
        variant as u8 != 0
    }
}
///Field `AD2EEV6` reader - ADC trigger 2 on External Event 6
pub type AD2EEV6_R = crate::BitReader<AD2EEV6>;
impl AD2EEV6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AD2EEV6 {
        match self.bits {
            false => AD2EEV6::Disabled,
            true => AD2EEV6::Enabled,
        }
    }
    ///No generation of ADC trigger on external event
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD2EEV6::Disabled
    }
    ///Generation of ADC trigger on external event
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD2EEV6::Enabled
    }
}
///Field `AD2EEV6` writer - ADC trigger 2 on External Event 6
pub type AD2EEV6_W<'a, REG> = crate::BitWriter<'a, REG, AD2EEV6>;
impl<'a, REG> AD2EEV6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No generation of ADC trigger on external event
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AD2EEV6::Disabled)
    }
    ///Generation of ADC trigger on external event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AD2EEV6::Enabled)
    }
}
///Field `AD2EEV7` reader - ADC trigger 2 on External Event 7
pub use AD2EEV6_R as AD2EEV7_R;
///Field `AD2EEV8` reader - ADC trigger 2 on External Event 8
pub use AD2EEV6_R as AD2EEV8_R;
///Field `AD2EEV9` reader - ADC trigger 2 on External Event 9
pub use AD2EEV6_R as AD2EEV9_R;
///Field `AD2EEV10` reader - ADC trigger 2 on External Event 10
pub use AD2EEV6_R as AD2EEV10_R;
///Field `AD2EEV7` writer - ADC trigger 2 on External Event 7
pub use AD2EEV6_W as AD2EEV7_W;
///Field `AD2EEV8` writer - ADC trigger 2 on External Event 8
pub use AD2EEV6_W as AD2EEV8_W;
///Field `AD2EEV9` writer - ADC trigger 2 on External Event 9
pub use AD2EEV6_W as AD2EEV9_W;
///Field `AD2EEV10` writer - ADC trigger 2 on External Event 10
pub use AD2EEV6_W as AD2EEV10_W;
/**ADC trigger 2 on Timer A compare 2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD2TAC2 {
    ///0: No generation of ADC trigger on timer compare event
    Disabled = 0,
    ///1: Generation of ADC trigger on timer compare event
    Enabled = 1,
}
impl From<AD2TAC2> for bool {
    #[inline(always)]
    fn from(variant: AD2TAC2) -> Self {
        variant as u8 != 0
    }
}
///Field `AD2TAC2` reader - ADC trigger 2 on Timer A compare 2
pub type AD2TAC2_R = crate::BitReader<AD2TAC2>;
impl AD2TAC2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AD2TAC2 {
        match self.bits {
            false => AD2TAC2::Disabled,
            true => AD2TAC2::Enabled,
        }
    }
    ///No generation of ADC trigger on timer compare event
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD2TAC2::Disabled
    }
    ///Generation of ADC trigger on timer compare event
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD2TAC2::Enabled
    }
}
///Field `AD2TAC2` writer - ADC trigger 2 on Timer A compare 2
pub type AD2TAC2_W<'a, REG> = crate::BitWriter<'a, REG, AD2TAC2>;
impl<'a, REG> AD2TAC2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No generation of ADC trigger on timer compare event
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AD2TAC2::Disabled)
    }
    ///Generation of ADC trigger on timer compare event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AD2TAC2::Enabled)
    }
}
///Field `AD2TAPER` reader - ADC trigger 2 on Timer A Period
pub use AD2MPER_R as AD2TAPER_R;
///Field `AD2TBPER` reader - ADC trigger 2 on Timer B Period
pub use AD2MPER_R as AD2TBPER_R;
///Field `AD2TCPER` reader - ADC trigger 2 on Timer C Period
pub use AD2MPER_R as AD2TCPER_R;
///Field `AD2TAPER` writer - ADC trigger 2 on Timer A Period
pub use AD2MPER_W as AD2TAPER_W;
///Field `AD2TBPER` writer - ADC trigger 2 on Timer B Period
pub use AD2MPER_W as AD2TBPER_W;
///Field `AD2TCPER` writer - ADC trigger 2 on Timer C Period
pub use AD2MPER_W as AD2TCPER_W;
///Field `AD2TAC3` reader - ADC trigger 2 on Timer A compare 3
pub use AD2TAC2_R as AD2TAC3_R;
///Field `AD2TAC4` reader - ADC trigger 2 on Timer A compare 4
pub use AD2TAC2_R as AD2TAC4_R;
///Field `AD2TBC2` reader - ADC trigger 2 on Timer B compare 2
pub use AD2TAC2_R as AD2TBC2_R;
///Field `AD2TBC3` reader - ADC trigger 2 on Timer B compare 3
pub use AD2TAC2_R as AD2TBC3_R;
///Field `AD2TBC4` reader - ADC trigger 2 on Timer B compare 4
pub use AD2TAC2_R as AD2TBC4_R;
///Field `AD2TCC2` reader - ADC trigger 2 on Timer C compare 2
pub use AD2TAC2_R as AD2TCC2_R;
///Field `AD2TCC3` reader - ADC trigger 2 on Timer C compare 3
pub use AD2TAC2_R as AD2TCC3_R;
///Field `AD2TCC4` reader - ADC trigger 2 on Timer C compare 4
pub use AD2TAC2_R as AD2TCC4_R;
///Field `AD2TAC3` writer - ADC trigger 2 on Timer A compare 3
pub use AD2TAC2_W as AD2TAC3_W;
///Field `AD2TAC4` writer - ADC trigger 2 on Timer A compare 4
pub use AD2TAC2_W as AD2TAC4_W;
///Field `AD2TBC2` writer - ADC trigger 2 on Timer B compare 2
pub use AD2TAC2_W as AD2TBC2_W;
///Field `AD2TBC3` writer - ADC trigger 2 on Timer B compare 3
pub use AD2TAC2_W as AD2TBC3_W;
///Field `AD2TBC4` writer - ADC trigger 2 on Timer B compare 4
pub use AD2TAC2_W as AD2TBC4_W;
///Field `AD2TCC2` writer - ADC trigger 2 on Timer C compare 2
pub use AD2TAC2_W as AD2TCC2_W;
///Field `AD2TCC3` writer - ADC trigger 2 on Timer C compare 3
pub use AD2TAC2_W as AD2TCC3_W;
///Field `AD2TCC4` writer - ADC trigger 2 on Timer C compare 4
pub use AD2TAC2_W as AD2TCC4_W;
/**ADC trigger 2 on Timer C Reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD2TCRST {
    ///0: No generation of ADC trigger on timer reset and roll-over
    Disabled = 0,
    ///1: Generation of ADC trigger on timer reset and roll-over
    Enabled = 1,
}
impl From<AD2TCRST> for bool {
    #[inline(always)]
    fn from(variant: AD2TCRST) -> Self {
        variant as u8 != 0
    }
}
///Field `AD2TCRST` reader - ADC trigger 2 on Timer C Reset
pub type AD2TCRST_R = crate::BitReader<AD2TCRST>;
impl AD2TCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AD2TCRST {
        match self.bits {
            false => AD2TCRST::Disabled,
            true => AD2TCRST::Enabled,
        }
    }
    ///No generation of ADC trigger on timer reset and roll-over
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD2TCRST::Disabled
    }
    ///Generation of ADC trigger on timer reset and roll-over
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD2TCRST::Enabled
    }
}
///Field `AD2TCRST` writer - ADC trigger 2 on Timer C Reset
pub type AD2TCRST_W<'a, REG> = crate::BitWriter<'a, REG, AD2TCRST>;
impl<'a, REG> AD2TCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No generation of ADC trigger on timer reset and roll-over
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AD2TCRST::Disabled)
    }
    ///Generation of ADC trigger on timer reset and roll-over
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AD2TCRST::Enabled)
    }
}
///Field `AD2TDPER` reader - ADC trigger 2 on Timer D Period
pub use AD2MPER_R as AD2TDPER_R;
///Field `AD2TDPER` writer - ADC trigger 2 on Timer D Period
pub use AD2MPER_W as AD2TDPER_W;
///Field `AD2TDC2` reader - ADC trigger 2 on Timer D compare 2
pub use AD2TAC2_R as AD2TDC2_R;
///Field `AD2TDC3` reader - ADC trigger 2 on Timer D compare 3
pub use AD2TAC2_R as AD2TDC3_R;
///Field `AD2TDC4` reader - ADC trigger 2 on Timer D compare 4
pub use AD2TAC2_R as AD2TDC4_R;
///Field `AD2TEC2` reader - ADC trigger 2 on Timer E compare 2
pub use AD2TAC2_R as AD2TEC2_R;
///Field `AD2TEC3` reader - ADC trigger 2 on Timer E compare 3
pub use AD2TAC2_R as AD2TEC3_R;
///Field `AD2TEC4` reader - ADC trigger 2 on Timer E compare 4
pub use AD2TAC2_R as AD2TEC4_R;
///Field `AD2TDC2` writer - ADC trigger 2 on Timer D compare 2
pub use AD2TAC2_W as AD2TDC2_W;
///Field `AD2TDC3` writer - ADC trigger 2 on Timer D compare 3
pub use AD2TAC2_W as AD2TDC3_W;
///Field `AD2TDC4` writer - ADC trigger 2 on Timer D compare 4
pub use AD2TAC2_W as AD2TDC4_W;
///Field `AD2TEC2` writer - ADC trigger 2 on Timer E compare 2
pub use AD2TAC2_W as AD2TEC2_W;
///Field `AD2TEC3` writer - ADC trigger 2 on Timer E compare 3
pub use AD2TAC2_W as AD2TEC3_W;
///Field `AD2TEC4` writer - ADC trigger 2 on Timer E compare 4
pub use AD2TAC2_W as AD2TEC4_W;
///Field `AD2TDRST` reader - ADC trigger 2 on Timer D Reset
pub use AD2TCRST_R as AD2TDRST_R;
///Field `AD2TERST` reader - ADC trigger 2 on Timer E Reset
pub use AD2TCRST_R as AD2TERST_R;
///Field `AD2TDRST` writer - ADC trigger 2 on Timer D Reset
pub use AD2TCRST_W as AD2TDRST_W;
///Field `AD2TERST` writer - ADC trigger 2 on Timer E Reset
pub use AD2TCRST_W as AD2TERST_W;
impl R {
    ///Bit 0 - ADC trigger 2 on Master Compare 1
    #[inline(always)]
    pub fn ad2mc1(&self) -> AD2MC1_R {
        AD2MC1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ADC trigger 2 on Master Compare 2
    #[inline(always)]
    pub fn ad2mc2(&self) -> AD2MC2_R {
        AD2MC2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ADC trigger 2 on Master Compare 3
    #[inline(always)]
    pub fn ad2mc3(&self) -> AD2MC3_R {
        AD2MC3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ADC trigger 2 on Master Compare 4
    #[inline(always)]
    pub fn ad2mc4(&self) -> AD2MC4_R {
        AD2MC4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ADC trigger 2 on Master Period
    #[inline(always)]
    pub fn ad2mper(&self) -> AD2MPER_R {
        AD2MPER_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ADC trigger 2 on External Event 6
    #[inline(always)]
    pub fn ad2eev6(&self) -> AD2EEV6_R {
        AD2EEV6_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - ADC trigger 2 on External Event 7
    #[inline(always)]
    pub fn ad2eev7(&self) -> AD2EEV7_R {
        AD2EEV7_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - ADC trigger 2 on External Event 8
    #[inline(always)]
    pub fn ad2eev8(&self) -> AD2EEV8_R {
        AD2EEV8_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ADC trigger 2 on External Event 9
    #[inline(always)]
    pub fn ad2eev9(&self) -> AD2EEV9_R {
        AD2EEV9_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ADC trigger 2 on External Event 10
    #[inline(always)]
    pub fn ad2eev10(&self) -> AD2EEV10_R {
        AD2EEV10_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ADC trigger 2 on Timer A compare 2
    #[inline(always)]
    pub fn ad2tac2(&self) -> AD2TAC2_R {
        AD2TAC2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - ADC trigger 2 on Timer A compare 3
    #[inline(always)]
    pub fn ad2tac3(&self) -> AD2TAC3_R {
        AD2TAC3_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - ADC trigger 2 on Timer A compare 4
    #[inline(always)]
    pub fn ad2tac4(&self) -> AD2TAC4_R {
        AD2TAC4_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ADC trigger 2 on Timer A Period
    #[inline(always)]
    pub fn ad2taper(&self) -> AD2TAPER_R {
        AD2TAPER_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ADC trigger 2 on Timer B compare 2
    #[inline(always)]
    pub fn ad2tbc2(&self) -> AD2TBC2_R {
        AD2TBC2_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - ADC trigger 2 on Timer B compare 3
    #[inline(always)]
    pub fn ad2tbc3(&self) -> AD2TBC3_R {
        AD2TBC3_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - ADC trigger 2 on Timer B compare 4
    #[inline(always)]
    pub fn ad2tbc4(&self) -> AD2TBC4_R {
        AD2TBC4_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ADC trigger 2 on Timer B Period
    #[inline(always)]
    pub fn ad2tbper(&self) -> AD2TBPER_R {
        AD2TBPER_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ADC trigger 2 on Timer C compare 2
    #[inline(always)]
    pub fn ad2tcc2(&self) -> AD2TCC2_R {
        AD2TCC2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - ADC trigger 2 on Timer C compare 3
    #[inline(always)]
    pub fn ad2tcc3(&self) -> AD2TCC3_R {
        AD2TCC3_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - ADC trigger 2 on Timer C compare 4
    #[inline(always)]
    pub fn ad2tcc4(&self) -> AD2TCC4_R {
        AD2TCC4_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ADC trigger 2 on Timer C Period
    #[inline(always)]
    pub fn ad2tcper(&self) -> AD2TCPER_R {
        AD2TCPER_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ADC trigger 2 on Timer C Reset
    #[inline(always)]
    pub fn ad2tcrst(&self) -> AD2TCRST_R {
        AD2TCRST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ADC trigger 2 on Timer D compare 2
    #[inline(always)]
    pub fn ad2tdc2(&self) -> AD2TDC2_R {
        AD2TDC2_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - ADC trigger 2 on Timer D compare 3
    #[inline(always)]
    pub fn ad2tdc3(&self) -> AD2TDC3_R {
        AD2TDC3_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - ADC trigger 2 on Timer D compare 4
    #[inline(always)]
    pub fn ad2tdc4(&self) -> AD2TDC4_R {
        AD2TDC4_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - ADC trigger 2 on Timer D Period
    #[inline(always)]
    pub fn ad2tdper(&self) -> AD2TDPER_R {
        AD2TDPER_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - ADC trigger 2 on Timer D Reset
    #[inline(always)]
    pub fn ad2tdrst(&self) -> AD2TDRST_R {
        AD2TDRST_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - ADC trigger 2 on Timer E compare 2
    #[inline(always)]
    pub fn ad2tec2(&self) -> AD2TEC2_R {
        AD2TEC2_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - ADC trigger 2 on Timer E compare 3
    #[inline(always)]
    pub fn ad2tec3(&self) -> AD2TEC3_R {
        AD2TEC3_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - ADC trigger 2 on Timer E compare 4
    #[inline(always)]
    pub fn ad2tec4(&self) -> AD2TEC4_R {
        AD2TEC4_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - ADC trigger 2 on Timer E Reset
    #[inline(always)]
    pub fn ad2terst(&self) -> AD2TERST_R {
        AD2TERST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC2R")
            .field("ad2tcrst", &self.ad2tcrst())
            .field("ad2terst", &self.ad2terst())
            .field("ad2tac2", &self.ad2tac2())
            .field("ad2tec4", &self.ad2tec4())
            .field("ad2tec3", &self.ad2tec3())
            .field("ad2tec2", &self.ad2tec2())
            .field("ad2tdrst", &self.ad2tdrst())
            .field("ad2mper", &self.ad2mper())
            .field("ad2tdper", &self.ad2tdper())
            .field("ad2tdc4", &self.ad2tdc4())
            .field("ad2tdc3", &self.ad2tdc3())
            .field("ad2tdc2", &self.ad2tdc2())
            .field("ad2tcper", &self.ad2tcper())
            .field("ad2tcc4", &self.ad2tcc4())
            .field("ad2tcc3", &self.ad2tcc3())
            .field("ad2tcc2", &self.ad2tcc2())
            .field("ad2tbper", &self.ad2tbper())
            .field("ad2tbc4", &self.ad2tbc4())
            .field("ad2tbc3", &self.ad2tbc3())
            .field("ad2tbc2", &self.ad2tbc2())
            .field("ad2taper", &self.ad2taper())
            .field("ad2tac4", &self.ad2tac4())
            .field("ad2tac3", &self.ad2tac3())
            .field("ad2eev6", &self.ad2eev6())
            .field("ad2eev10", &self.ad2eev10())
            .field("ad2eev9", &self.ad2eev9())
            .field("ad2eev8", &self.ad2eev8())
            .field("ad2eev7", &self.ad2eev7())
            .field("ad2mc1", &self.ad2mc1())
            .field("ad2mc4", &self.ad2mc4())
            .field("ad2mc3", &self.ad2mc3())
            .field("ad2mc2", &self.ad2mc2())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADC trigger 2 on Master Compare 1
    #[inline(always)]
    #[must_use]
    pub fn ad2mc1(&mut self) -> AD2MC1_W<ADC2Rrs> {
        AD2MC1_W::new(self, 0)
    }
    ///Bit 1 - ADC trigger 2 on Master Compare 2
    #[inline(always)]
    #[must_use]
    pub fn ad2mc2(&mut self) -> AD2MC2_W<ADC2Rrs> {
        AD2MC2_W::new(self, 1)
    }
    ///Bit 2 - ADC trigger 2 on Master Compare 3
    #[inline(always)]
    #[must_use]
    pub fn ad2mc3(&mut self) -> AD2MC3_W<ADC2Rrs> {
        AD2MC3_W::new(self, 2)
    }
    ///Bit 3 - ADC trigger 2 on Master Compare 4
    #[inline(always)]
    #[must_use]
    pub fn ad2mc4(&mut self) -> AD2MC4_W<ADC2Rrs> {
        AD2MC4_W::new(self, 3)
    }
    ///Bit 4 - ADC trigger 2 on Master Period
    #[inline(always)]
    #[must_use]
    pub fn ad2mper(&mut self) -> AD2MPER_W<ADC2Rrs> {
        AD2MPER_W::new(self, 4)
    }
    ///Bit 5 - ADC trigger 2 on External Event 6
    #[inline(always)]
    #[must_use]
    pub fn ad2eev6(&mut self) -> AD2EEV6_W<ADC2Rrs> {
        AD2EEV6_W::new(self, 5)
    }
    ///Bit 6 - ADC trigger 2 on External Event 7
    #[inline(always)]
    #[must_use]
    pub fn ad2eev7(&mut self) -> AD2EEV7_W<ADC2Rrs> {
        AD2EEV7_W::new(self, 6)
    }
    ///Bit 7 - ADC trigger 2 on External Event 8
    #[inline(always)]
    #[must_use]
    pub fn ad2eev8(&mut self) -> AD2EEV8_W<ADC2Rrs> {
        AD2EEV8_W::new(self, 7)
    }
    ///Bit 8 - ADC trigger 2 on External Event 9
    #[inline(always)]
    #[must_use]
    pub fn ad2eev9(&mut self) -> AD2EEV9_W<ADC2Rrs> {
        AD2EEV9_W::new(self, 8)
    }
    ///Bit 9 - ADC trigger 2 on External Event 10
    #[inline(always)]
    #[must_use]
    pub fn ad2eev10(&mut self) -> AD2EEV10_W<ADC2Rrs> {
        AD2EEV10_W::new(self, 9)
    }
    ///Bit 10 - ADC trigger 2 on Timer A compare 2
    #[inline(always)]
    #[must_use]
    pub fn ad2tac2(&mut self) -> AD2TAC2_W<ADC2Rrs> {
        AD2TAC2_W::new(self, 10)
    }
    ///Bit 11 - ADC trigger 2 on Timer A compare 3
    #[inline(always)]
    #[must_use]
    pub fn ad2tac3(&mut self) -> AD2TAC3_W<ADC2Rrs> {
        AD2TAC3_W::new(self, 11)
    }
    ///Bit 12 - ADC trigger 2 on Timer A compare 4
    #[inline(always)]
    #[must_use]
    pub fn ad2tac4(&mut self) -> AD2TAC4_W<ADC2Rrs> {
        AD2TAC4_W::new(self, 12)
    }
    ///Bit 13 - ADC trigger 2 on Timer A Period
    #[inline(always)]
    #[must_use]
    pub fn ad2taper(&mut self) -> AD2TAPER_W<ADC2Rrs> {
        AD2TAPER_W::new(self, 13)
    }
    ///Bit 14 - ADC trigger 2 on Timer B compare 2
    #[inline(always)]
    #[must_use]
    pub fn ad2tbc2(&mut self) -> AD2TBC2_W<ADC2Rrs> {
        AD2TBC2_W::new(self, 14)
    }
    ///Bit 15 - ADC trigger 2 on Timer B compare 3
    #[inline(always)]
    #[must_use]
    pub fn ad2tbc3(&mut self) -> AD2TBC3_W<ADC2Rrs> {
        AD2TBC3_W::new(self, 15)
    }
    ///Bit 16 - ADC trigger 2 on Timer B compare 4
    #[inline(always)]
    #[must_use]
    pub fn ad2tbc4(&mut self) -> AD2TBC4_W<ADC2Rrs> {
        AD2TBC4_W::new(self, 16)
    }
    ///Bit 17 - ADC trigger 2 on Timer B Period
    #[inline(always)]
    #[must_use]
    pub fn ad2tbper(&mut self) -> AD2TBPER_W<ADC2Rrs> {
        AD2TBPER_W::new(self, 17)
    }
    ///Bit 18 - ADC trigger 2 on Timer C compare 2
    #[inline(always)]
    #[must_use]
    pub fn ad2tcc2(&mut self) -> AD2TCC2_W<ADC2Rrs> {
        AD2TCC2_W::new(self, 18)
    }
    ///Bit 19 - ADC trigger 2 on Timer C compare 3
    #[inline(always)]
    #[must_use]
    pub fn ad2tcc3(&mut self) -> AD2TCC3_W<ADC2Rrs> {
        AD2TCC3_W::new(self, 19)
    }
    ///Bit 20 - ADC trigger 2 on Timer C compare 4
    #[inline(always)]
    #[must_use]
    pub fn ad2tcc4(&mut self) -> AD2TCC4_W<ADC2Rrs> {
        AD2TCC4_W::new(self, 20)
    }
    ///Bit 21 - ADC trigger 2 on Timer C Period
    #[inline(always)]
    #[must_use]
    pub fn ad2tcper(&mut self) -> AD2TCPER_W<ADC2Rrs> {
        AD2TCPER_W::new(self, 21)
    }
    ///Bit 22 - ADC trigger 2 on Timer C Reset
    #[inline(always)]
    #[must_use]
    pub fn ad2tcrst(&mut self) -> AD2TCRST_W<ADC2Rrs> {
        AD2TCRST_W::new(self, 22)
    }
    ///Bit 23 - ADC trigger 2 on Timer D compare 2
    #[inline(always)]
    #[must_use]
    pub fn ad2tdc2(&mut self) -> AD2TDC2_W<ADC2Rrs> {
        AD2TDC2_W::new(self, 23)
    }
    ///Bit 24 - ADC trigger 2 on Timer D compare 3
    #[inline(always)]
    #[must_use]
    pub fn ad2tdc3(&mut self) -> AD2TDC3_W<ADC2Rrs> {
        AD2TDC3_W::new(self, 24)
    }
    ///Bit 25 - ADC trigger 2 on Timer D compare 4
    #[inline(always)]
    #[must_use]
    pub fn ad2tdc4(&mut self) -> AD2TDC4_W<ADC2Rrs> {
        AD2TDC4_W::new(self, 25)
    }
    ///Bit 26 - ADC trigger 2 on Timer D Period
    #[inline(always)]
    #[must_use]
    pub fn ad2tdper(&mut self) -> AD2TDPER_W<ADC2Rrs> {
        AD2TDPER_W::new(self, 26)
    }
    ///Bit 27 - ADC trigger 2 on Timer D Reset
    #[inline(always)]
    #[must_use]
    pub fn ad2tdrst(&mut self) -> AD2TDRST_W<ADC2Rrs> {
        AD2TDRST_W::new(self, 27)
    }
    ///Bit 28 - ADC trigger 2 on Timer E compare 2
    #[inline(always)]
    #[must_use]
    pub fn ad2tec2(&mut self) -> AD2TEC2_W<ADC2Rrs> {
        AD2TEC2_W::new(self, 28)
    }
    ///Bit 29 - ADC trigger 2 on Timer E compare 3
    #[inline(always)]
    #[must_use]
    pub fn ad2tec3(&mut self) -> AD2TEC3_W<ADC2Rrs> {
        AD2TEC3_W::new(self, 29)
    }
    ///Bit 30 - ADC trigger 2 on Timer E compare 4
    #[inline(always)]
    #[must_use]
    pub fn ad2tec4(&mut self) -> AD2TEC4_W<ADC2Rrs> {
        AD2TEC4_W::new(self, 30)
    }
    ///Bit 31 - ADC trigger 2 on Timer E Reset
    #[inline(always)]
    #[must_use]
    pub fn ad2terst(&mut self) -> AD2TERST_W<ADC2Rrs> {
        AD2TERST_W::new(self, 31)
    }
}
/**ADC Trigger 2 Register

You can [`read`](crate::Reg::read) this register and get [`adc2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_Common:ADC2R)*/
pub struct ADC2Rrs;
impl crate::RegisterSpec for ADC2Rrs {
    type Ux = u32;
}
///`read()` method returns [`adc2r::R`](R) reader structure
impl crate::Readable for ADC2Rrs {}
///`write(|w| ..)` method takes [`adc2r::W`](W) writer structure
impl crate::Writable for ADC2Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC2R to value 0
impl crate::Resettable for ADC2Rrs {
    const RESET_VALUE: u32 = 0;
}
