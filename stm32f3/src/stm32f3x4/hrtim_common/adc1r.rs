#[doc = "Register `ADC1R` reader"]
pub type R = crate::R<ADC1Rrs>;
#[doc = "Register `ADC1R` writer"]
pub type W = crate::W<ADC1Rrs>;
#[doc = "ADC trigger 1 on Master Compare 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD1MC1 {
    #[doc = "0: No generation of ADC trigger on master compare event"]
    Disabled = 0,
    #[doc = "1: Generation of ADC trigger on master compare event"]
    Enabled = 1,
}
impl From<AD1MC1> for bool {
    #[inline(always)]
    fn from(variant: AD1MC1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AD1MC1` reader - ADC trigger 1 on Master Compare 1"]
pub type AD1MC1_R = crate::BitReader<AD1MC1>;
impl AD1MC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AD1MC1 {
        match self.bits {
            false => AD1MC1::Disabled,
            true => AD1MC1::Enabled,
        }
    }
    #[doc = "No generation of ADC trigger on master compare event"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD1MC1::Disabled
    }
    #[doc = "Generation of ADC trigger on master compare event"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD1MC1::Enabled
    }
}
#[doc = "Field `AD1MC1` writer - ADC trigger 1 on Master Compare 1"]
pub type AD1MC1_W<'a, REG> = crate::BitWriter<'a, REG, AD1MC1>;
impl<'a, REG> AD1MC1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No generation of ADC trigger on master compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AD1MC1::Disabled)
    }
    #[doc = "Generation of ADC trigger on master compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AD1MC1::Enabled)
    }
}
#[doc = "Field `AD1MC2` reader - ADC trigger 1 on Master Compare 2"]
pub use AD1MC1_R as AD1MC2_R;
#[doc = "Field `AD1MC3` reader - ADC trigger 1 on Master Compare 3"]
pub use AD1MC1_R as AD1MC3_R;
#[doc = "Field `AD1MC4` reader - ADC trigger 1 on Master Compare 4"]
pub use AD1MC1_R as AD1MC4_R;
#[doc = "Field `AD1MC2` writer - ADC trigger 1 on Master Compare 2"]
pub use AD1MC1_W as AD1MC2_W;
#[doc = "Field `AD1MC3` writer - ADC trigger 1 on Master Compare 3"]
pub use AD1MC1_W as AD1MC3_W;
#[doc = "Field `AD1MC4` writer - ADC trigger 1 on Master Compare 4"]
pub use AD1MC1_W as AD1MC4_W;
#[doc = "ADC trigger 1 on Master Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD1MPER {
    #[doc = "0: No generation of ADC trigger on timer period event"]
    Disabled = 0,
    #[doc = "1: Generation of ADC trigger on timer period event"]
    Enabled = 1,
}
impl From<AD1MPER> for bool {
    #[inline(always)]
    fn from(variant: AD1MPER) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AD1MPER` reader - ADC trigger 1 on Master Period"]
pub type AD1MPER_R = crate::BitReader<AD1MPER>;
impl AD1MPER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AD1MPER {
        match self.bits {
            false => AD1MPER::Disabled,
            true => AD1MPER::Enabled,
        }
    }
    #[doc = "No generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD1MPER::Disabled
    }
    #[doc = "Generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD1MPER::Enabled
    }
}
#[doc = "Field `AD1MPER` writer - ADC trigger 1 on Master Period"]
pub type AD1MPER_W<'a, REG> = crate::BitWriter<'a, REG, AD1MPER>;
impl<'a, REG> AD1MPER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AD1MPER::Disabled)
    }
    #[doc = "Generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AD1MPER::Enabled)
    }
}
#[doc = "ADC trigger 1 on External Event 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD1EEV1 {
    #[doc = "0: No generation of ADC trigger on external event"]
    Disabled = 0,
    #[doc = "1: Generation of ADC trigger on external event"]
    Enabled = 1,
}
impl From<AD1EEV1> for bool {
    #[inline(always)]
    fn from(variant: AD1EEV1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AD1EEV1` reader - ADC trigger 1 on External Event 1"]
pub type AD1EEV1_R = crate::BitReader<AD1EEV1>;
impl AD1EEV1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AD1EEV1 {
        match self.bits {
            false => AD1EEV1::Disabled,
            true => AD1EEV1::Enabled,
        }
    }
    #[doc = "No generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD1EEV1::Disabled
    }
    #[doc = "Generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD1EEV1::Enabled
    }
}
#[doc = "Field `AD1EEV1` writer - ADC trigger 1 on External Event 1"]
pub type AD1EEV1_W<'a, REG> = crate::BitWriter<'a, REG, AD1EEV1>;
impl<'a, REG> AD1EEV1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AD1EEV1::Disabled)
    }
    #[doc = "Generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AD1EEV1::Enabled)
    }
}
#[doc = "Field `AD1EEV2` reader - ADC trigger 1 on External Event 2"]
pub use AD1EEV1_R as AD1EEV2_R;
#[doc = "Field `AD1EEV3` reader - ADC trigger 1 on External Event 3"]
pub use AD1EEV1_R as AD1EEV3_R;
#[doc = "Field `AD1EEV4` reader - ADC trigger 1 on External Event 4"]
pub use AD1EEV1_R as AD1EEV4_R;
#[doc = "Field `AD1EEV5` reader - ADC trigger 1 on External Event 5"]
pub use AD1EEV1_R as AD1EEV5_R;
#[doc = "Field `AD1EEV2` writer - ADC trigger 1 on External Event 2"]
pub use AD1EEV1_W as AD1EEV2_W;
#[doc = "Field `AD1EEV3` writer - ADC trigger 1 on External Event 3"]
pub use AD1EEV1_W as AD1EEV3_W;
#[doc = "Field `AD1EEV4` writer - ADC trigger 1 on External Event 4"]
pub use AD1EEV1_W as AD1EEV4_W;
#[doc = "Field `AD1EEV5` writer - ADC trigger 1 on External Event 5"]
pub use AD1EEV1_W as AD1EEV5_W;
#[doc = "ADC trigger 1 on Timer A compare 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD1TAC2 {
    #[doc = "0: No generation of ADC trigger on timer compare event"]
    Disabled = 0,
    #[doc = "1: Generation of ADC trigger on timer compare event"]
    Enabled = 1,
}
impl From<AD1TAC2> for bool {
    #[inline(always)]
    fn from(variant: AD1TAC2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AD1TAC2` reader - ADC trigger 1 on Timer A compare 2"]
pub type AD1TAC2_R = crate::BitReader<AD1TAC2>;
impl AD1TAC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AD1TAC2 {
        match self.bits {
            false => AD1TAC2::Disabled,
            true => AD1TAC2::Enabled,
        }
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD1TAC2::Disabled
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD1TAC2::Enabled
    }
}
#[doc = "Field `AD1TAC2` writer - ADC trigger 1 on Timer A compare 2"]
pub type AD1TAC2_W<'a, REG> = crate::BitWriter<'a, REG, AD1TAC2>;
impl<'a, REG> AD1TAC2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AD1TAC2::Disabled)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AD1TAC2::Enabled)
    }
}
#[doc = "Field `AD1TAPER` reader - ADC trigger 1 on Timer A Period"]
pub use AD1MPER_R as AD1TAPER_R;
#[doc = "Field `AD1TAPER` writer - ADC trigger 1 on Timer A Period"]
pub use AD1MPER_W as AD1TAPER_W;
#[doc = "Field `AD1TAC3` reader - ADC trigger 1 on Timer A compare 3"]
pub use AD1TAC2_R as AD1TAC3_R;
#[doc = "Field `AD1TAC4` reader - ADC trigger 1 on Timer A compare 4"]
pub use AD1TAC2_R as AD1TAC4_R;
#[doc = "Field `AD1TAC3` writer - ADC trigger 1 on Timer A compare 3"]
pub use AD1TAC2_W as AD1TAC3_W;
#[doc = "Field `AD1TAC4` writer - ADC trigger 1 on Timer A compare 4"]
pub use AD1TAC2_W as AD1TAC4_W;
#[doc = "ADC trigger 1 on Timer A Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD1TARST {
    #[doc = "0: No generation of ADC trigger on timer reset and roll-over"]
    Disabled = 0,
    #[doc = "1: Generation of ADC trigger on timer reset and roll-over"]
    Enabled = 1,
}
impl From<AD1TARST> for bool {
    #[inline(always)]
    fn from(variant: AD1TARST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AD1TARST` reader - ADC trigger 1 on Timer A Reset"]
pub type AD1TARST_R = crate::BitReader<AD1TARST>;
impl AD1TARST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AD1TARST {
        match self.bits {
            false => AD1TARST::Disabled,
            true => AD1TARST::Enabled,
        }
    }
    #[doc = "No generation of ADC trigger on timer reset and roll-over"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD1TARST::Disabled
    }
    #[doc = "Generation of ADC trigger on timer reset and roll-over"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD1TARST::Enabled
    }
}
#[doc = "Field `AD1TARST` writer - ADC trigger 1 on Timer A Reset"]
pub type AD1TARST_W<'a, REG> = crate::BitWriter<'a, REG, AD1TARST>;
impl<'a, REG> AD1TARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No generation of ADC trigger on timer reset and roll-over"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AD1TARST::Disabled)
    }
    #[doc = "Generation of ADC trigger on timer reset and roll-over"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AD1TARST::Enabled)
    }
}
#[doc = "Field `AD1TBPER` reader - ADC trigger 1 on Timer B Period"]
pub use AD1MPER_R as AD1TBPER_R;
#[doc = "Field `AD1TCPER` reader - ADC trigger 1 on Timer C Period"]
pub use AD1MPER_R as AD1TCPER_R;
#[doc = "Field `AD1TDPER` reader - ADC trigger 1 on Timer D Period"]
pub use AD1MPER_R as AD1TDPER_R;
#[doc = "Field `AD1TEPER` reader - ADC trigger 1 on Timer E Period"]
pub use AD1MPER_R as AD1TEPER_R;
#[doc = "Field `AD1TBPER` writer - ADC trigger 1 on Timer B Period"]
pub use AD1MPER_W as AD1TBPER_W;
#[doc = "Field `AD1TCPER` writer - ADC trigger 1 on Timer C Period"]
pub use AD1MPER_W as AD1TCPER_W;
#[doc = "Field `AD1TDPER` writer - ADC trigger 1 on Timer D Period"]
pub use AD1MPER_W as AD1TDPER_W;
#[doc = "Field `AD1TEPER` writer - ADC trigger 1 on Timer E Period"]
pub use AD1MPER_W as AD1TEPER_W;
#[doc = "Field `AD1TBC2` reader - ADC trigger 1 on Timer B compare 2"]
pub use AD1TAC2_R as AD1TBC2_R;
#[doc = "Field `AD1TBC3` reader - ADC trigger 1 on Timer B compare 3"]
pub use AD1TAC2_R as AD1TBC3_R;
#[doc = "Field `AD1TBC4` reader - ADC trigger 1 on Timer B compare 4"]
pub use AD1TAC2_R as AD1TBC4_R;
#[doc = "Field `AD1TCC2` reader - ADC trigger 1 on Timer C compare 2"]
pub use AD1TAC2_R as AD1TCC2_R;
#[doc = "Field `AD1TCC3` reader - ADC trigger 1 on Timer C compare 3"]
pub use AD1TAC2_R as AD1TCC3_R;
#[doc = "Field `AD1TCC4` reader - ADC trigger 1 on Timer C compare 4"]
pub use AD1TAC2_R as AD1TCC4_R;
#[doc = "Field `AD1TDC2` reader - ADC trigger 1 on Timer D compare 2"]
pub use AD1TAC2_R as AD1TDC2_R;
#[doc = "Field `AD1TDC3` reader - ADC trigger 1 on Timer D compare 3"]
pub use AD1TAC2_R as AD1TDC3_R;
#[doc = "Field `AD1TDC4` reader - ADC trigger 1 on Timer D compare 4"]
pub use AD1TAC2_R as AD1TDC4_R;
#[doc = "Field `AD1TEC2` reader - ADC trigger 1 on Timer E compare 2"]
pub use AD1TAC2_R as AD1TEC2_R;
#[doc = "Field `AD1TEC3` reader - ADC trigger 1 on Timer E compare 3"]
pub use AD1TAC2_R as AD1TEC3_R;
#[doc = "Field `AD1TEC4` reader - ADC trigger 1 on Timer E compare 4"]
pub use AD1TAC2_R as AD1TEC4_R;
#[doc = "Field `AD1TBC2` writer - ADC trigger 1 on Timer B compare 2"]
pub use AD1TAC2_W as AD1TBC2_W;
#[doc = "Field `AD1TBC3` writer - ADC trigger 1 on Timer B compare 3"]
pub use AD1TAC2_W as AD1TBC3_W;
#[doc = "Field `AD1TBC4` writer - ADC trigger 1 on Timer B compare 4"]
pub use AD1TAC2_W as AD1TBC4_W;
#[doc = "Field `AD1TCC2` writer - ADC trigger 1 on Timer C compare 2"]
pub use AD1TAC2_W as AD1TCC2_W;
#[doc = "Field `AD1TCC3` writer - ADC trigger 1 on Timer C compare 3"]
pub use AD1TAC2_W as AD1TCC3_W;
#[doc = "Field `AD1TCC4` writer - ADC trigger 1 on Timer C compare 4"]
pub use AD1TAC2_W as AD1TCC4_W;
#[doc = "Field `AD1TDC2` writer - ADC trigger 1 on Timer D compare 2"]
pub use AD1TAC2_W as AD1TDC2_W;
#[doc = "Field `AD1TDC3` writer - ADC trigger 1 on Timer D compare 3"]
pub use AD1TAC2_W as AD1TDC3_W;
#[doc = "Field `AD1TDC4` writer - ADC trigger 1 on Timer D compare 4"]
pub use AD1TAC2_W as AD1TDC4_W;
#[doc = "Field `AD1TEC2` writer - ADC trigger 1 on Timer E compare 2"]
pub use AD1TAC2_W as AD1TEC2_W;
#[doc = "Field `AD1TEC3` writer - ADC trigger 1 on Timer E compare 3"]
pub use AD1TAC2_W as AD1TEC3_W;
#[doc = "Field `AD1TEC4` writer - ADC trigger 1 on Timer E compare 4"]
pub use AD1TAC2_W as AD1TEC4_W;
#[doc = "Field `AD1TBRST` reader - ADC trigger 1 on Timer B Reset"]
pub use AD1TARST_R as AD1TBRST_R;
#[doc = "Field `AD1TBRST` writer - ADC trigger 1 on Timer B Reset"]
pub use AD1TARST_W as AD1TBRST_W;
impl R {
    #[doc = "Bit 0 - ADC trigger 1 on Master Compare 1"]
    #[inline(always)]
    pub fn ad1mc1(&self) -> AD1MC1_R {
        AD1MC1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC trigger 1 on Master Compare 2"]
    #[inline(always)]
    pub fn ad1mc2(&self) -> AD1MC2_R {
        AD1MC2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC trigger 1 on Master Compare 3"]
    #[inline(always)]
    pub fn ad1mc3(&self) -> AD1MC3_R {
        AD1MC3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC trigger 1 on Master Compare 4"]
    #[inline(always)]
    pub fn ad1mc4(&self) -> AD1MC4_R {
        AD1MC4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC trigger 1 on Master Period"]
    #[inline(always)]
    pub fn ad1mper(&self) -> AD1MPER_R {
        AD1MPER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC trigger 1 on External Event 1"]
    #[inline(always)]
    pub fn ad1eev1(&self) -> AD1EEV1_R {
        AD1EEV1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC trigger 1 on External Event 2"]
    #[inline(always)]
    pub fn ad1eev2(&self) -> AD1EEV2_R {
        AD1EEV2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC trigger 1 on External Event 3"]
    #[inline(always)]
    pub fn ad1eev3(&self) -> AD1EEV3_R {
        AD1EEV3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC trigger 1 on External Event 4"]
    #[inline(always)]
    pub fn ad1eev4(&self) -> AD1EEV4_R {
        AD1EEV4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC trigger 1 on External Event 5"]
    #[inline(always)]
    pub fn ad1eev5(&self) -> AD1EEV5_R {
        AD1EEV5_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC trigger 1 on Timer A compare 2"]
    #[inline(always)]
    pub fn ad1tac2(&self) -> AD1TAC2_R {
        AD1TAC2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ADC trigger 1 on Timer A compare 3"]
    #[inline(always)]
    pub fn ad1tac3(&self) -> AD1TAC3_R {
        AD1TAC3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ADC trigger 1 on Timer A compare 4"]
    #[inline(always)]
    pub fn ad1tac4(&self) -> AD1TAC4_R {
        AD1TAC4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC trigger 1 on Timer A Period"]
    #[inline(always)]
    pub fn ad1taper(&self) -> AD1TAPER_R {
        AD1TAPER_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC trigger 1 on Timer A Reset"]
    #[inline(always)]
    pub fn ad1tarst(&self) -> AD1TARST_R {
        AD1TARST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC trigger 1 on Timer B compare 2"]
    #[inline(always)]
    pub fn ad1tbc2(&self) -> AD1TBC2_R {
        AD1TBC2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC trigger 1 on Timer B compare 3"]
    #[inline(always)]
    pub fn ad1tbc3(&self) -> AD1TBC3_R {
        AD1TBC3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC trigger 1 on Timer B compare 4"]
    #[inline(always)]
    pub fn ad1tbc4(&self) -> AD1TBC4_R {
        AD1TBC4_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC trigger 1 on Timer B Period"]
    #[inline(always)]
    pub fn ad1tbper(&self) -> AD1TBPER_R {
        AD1TBPER_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADC trigger 1 on Timer B Reset"]
    #[inline(always)]
    pub fn ad1tbrst(&self) -> AD1TBRST_R {
        AD1TBRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC trigger 1 on Timer C compare 2"]
    #[inline(always)]
    pub fn ad1tcc2(&self) -> AD1TCC2_R {
        AD1TCC2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ADC trigger 1 on Timer C compare 3"]
    #[inline(always)]
    pub fn ad1tcc3(&self) -> AD1TCC3_R {
        AD1TCC3_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ADC trigger 1 on Timer C compare 4"]
    #[inline(always)]
    pub fn ad1tcc4(&self) -> AD1TCC4_R {
        AD1TCC4_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ADC trigger 1 on Timer C Period"]
    #[inline(always)]
    pub fn ad1tcper(&self) -> AD1TCPER_R {
        AD1TCPER_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ADC trigger 1 on Timer D compare 2"]
    #[inline(always)]
    pub fn ad1tdc2(&self) -> AD1TDC2_R {
        AD1TDC2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ADC trigger 1 on Timer D compare 3"]
    #[inline(always)]
    pub fn ad1tdc3(&self) -> AD1TDC3_R {
        AD1TDC3_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - ADC trigger 1 on Timer D compare 4"]
    #[inline(always)]
    pub fn ad1tdc4(&self) -> AD1TDC4_R {
        AD1TDC4_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - ADC trigger 1 on Timer D Period"]
    #[inline(always)]
    pub fn ad1tdper(&self) -> AD1TDPER_R {
        AD1TDPER_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - ADC trigger 1 on Timer E compare 2"]
    #[inline(always)]
    pub fn ad1tec2(&self) -> AD1TEC2_R {
        AD1TEC2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ADC trigger 1 on Timer E compare 3"]
    #[inline(always)]
    pub fn ad1tec3(&self) -> AD1TEC3_R {
        AD1TEC3_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ADC trigger 1 on Timer E compare 4"]
    #[inline(always)]
    pub fn ad1tec4(&self) -> AD1TEC4_R {
        AD1TEC4_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ADC trigger 1 on Timer E Period"]
    #[inline(always)]
    pub fn ad1teper(&self) -> AD1TEPER_R {
        AD1TEPER_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC trigger 1 on Master Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn ad1mc1(&mut self) -> AD1MC1_W<ADC1Rrs> {
        AD1MC1_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC trigger 1 on Master Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn ad1mc2(&mut self) -> AD1MC2_W<ADC1Rrs> {
        AD1MC2_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC trigger 1 on Master Compare 3"]
    #[inline(always)]
    #[must_use]
    pub fn ad1mc3(&mut self) -> AD1MC3_W<ADC1Rrs> {
        AD1MC3_W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC trigger 1 on Master Compare 4"]
    #[inline(always)]
    #[must_use]
    pub fn ad1mc4(&mut self) -> AD1MC4_W<ADC1Rrs> {
        AD1MC4_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC trigger 1 on Master Period"]
    #[inline(always)]
    #[must_use]
    pub fn ad1mper(&mut self) -> AD1MPER_W<ADC1Rrs> {
        AD1MPER_W::new(self, 4)
    }
    #[doc = "Bit 5 - ADC trigger 1 on External Event 1"]
    #[inline(always)]
    #[must_use]
    pub fn ad1eev1(&mut self) -> AD1EEV1_W<ADC1Rrs> {
        AD1EEV1_W::new(self, 5)
    }
    #[doc = "Bit 6 - ADC trigger 1 on External Event 2"]
    #[inline(always)]
    #[must_use]
    pub fn ad1eev2(&mut self) -> AD1EEV2_W<ADC1Rrs> {
        AD1EEV2_W::new(self, 6)
    }
    #[doc = "Bit 7 - ADC trigger 1 on External Event 3"]
    #[inline(always)]
    #[must_use]
    pub fn ad1eev3(&mut self) -> AD1EEV3_W<ADC1Rrs> {
        AD1EEV3_W::new(self, 7)
    }
    #[doc = "Bit 8 - ADC trigger 1 on External Event 4"]
    #[inline(always)]
    #[must_use]
    pub fn ad1eev4(&mut self) -> AD1EEV4_W<ADC1Rrs> {
        AD1EEV4_W::new(self, 8)
    }
    #[doc = "Bit 9 - ADC trigger 1 on External Event 5"]
    #[inline(always)]
    #[must_use]
    pub fn ad1eev5(&mut self) -> AD1EEV5_W<ADC1Rrs> {
        AD1EEV5_W::new(self, 9)
    }
    #[doc = "Bit 10 - ADC trigger 1 on Timer A compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tac2(&mut self) -> AD1TAC2_W<ADC1Rrs> {
        AD1TAC2_W::new(self, 10)
    }
    #[doc = "Bit 11 - ADC trigger 1 on Timer A compare 3"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tac3(&mut self) -> AD1TAC3_W<ADC1Rrs> {
        AD1TAC3_W::new(self, 11)
    }
    #[doc = "Bit 12 - ADC trigger 1 on Timer A compare 4"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tac4(&mut self) -> AD1TAC4_W<ADC1Rrs> {
        AD1TAC4_W::new(self, 12)
    }
    #[doc = "Bit 13 - ADC trigger 1 on Timer A Period"]
    #[inline(always)]
    #[must_use]
    pub fn ad1taper(&mut self) -> AD1TAPER_W<ADC1Rrs> {
        AD1TAPER_W::new(self, 13)
    }
    #[doc = "Bit 14 - ADC trigger 1 on Timer A Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tarst(&mut self) -> AD1TARST_W<ADC1Rrs> {
        AD1TARST_W::new(self, 14)
    }
    #[doc = "Bit 15 - ADC trigger 1 on Timer B compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tbc2(&mut self) -> AD1TBC2_W<ADC1Rrs> {
        AD1TBC2_W::new(self, 15)
    }
    #[doc = "Bit 16 - ADC trigger 1 on Timer B compare 3"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tbc3(&mut self) -> AD1TBC3_W<ADC1Rrs> {
        AD1TBC3_W::new(self, 16)
    }
    #[doc = "Bit 17 - ADC trigger 1 on Timer B compare 4"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tbc4(&mut self) -> AD1TBC4_W<ADC1Rrs> {
        AD1TBC4_W::new(self, 17)
    }
    #[doc = "Bit 18 - ADC trigger 1 on Timer B Period"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tbper(&mut self) -> AD1TBPER_W<ADC1Rrs> {
        AD1TBPER_W::new(self, 18)
    }
    #[doc = "Bit 19 - ADC trigger 1 on Timer B Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tbrst(&mut self) -> AD1TBRST_W<ADC1Rrs> {
        AD1TBRST_W::new(self, 19)
    }
    #[doc = "Bit 20 - ADC trigger 1 on Timer C compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tcc2(&mut self) -> AD1TCC2_W<ADC1Rrs> {
        AD1TCC2_W::new(self, 20)
    }
    #[doc = "Bit 21 - ADC trigger 1 on Timer C compare 3"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tcc3(&mut self) -> AD1TCC3_W<ADC1Rrs> {
        AD1TCC3_W::new(self, 21)
    }
    #[doc = "Bit 22 - ADC trigger 1 on Timer C compare 4"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tcc4(&mut self) -> AD1TCC4_W<ADC1Rrs> {
        AD1TCC4_W::new(self, 22)
    }
    #[doc = "Bit 23 - ADC trigger 1 on Timer C Period"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tcper(&mut self) -> AD1TCPER_W<ADC1Rrs> {
        AD1TCPER_W::new(self, 23)
    }
    #[doc = "Bit 24 - ADC trigger 1 on Timer D compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tdc2(&mut self) -> AD1TDC2_W<ADC1Rrs> {
        AD1TDC2_W::new(self, 24)
    }
    #[doc = "Bit 25 - ADC trigger 1 on Timer D compare 3"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tdc3(&mut self) -> AD1TDC3_W<ADC1Rrs> {
        AD1TDC3_W::new(self, 25)
    }
    #[doc = "Bit 26 - ADC trigger 1 on Timer D compare 4"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tdc4(&mut self) -> AD1TDC4_W<ADC1Rrs> {
        AD1TDC4_W::new(self, 26)
    }
    #[doc = "Bit 27 - ADC trigger 1 on Timer D Period"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tdper(&mut self) -> AD1TDPER_W<ADC1Rrs> {
        AD1TDPER_W::new(self, 27)
    }
    #[doc = "Bit 28 - ADC trigger 1 on Timer E compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tec2(&mut self) -> AD1TEC2_W<ADC1Rrs> {
        AD1TEC2_W::new(self, 28)
    }
    #[doc = "Bit 29 - ADC trigger 1 on Timer E compare 3"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tec3(&mut self) -> AD1TEC3_W<ADC1Rrs> {
        AD1TEC3_W::new(self, 29)
    }
    #[doc = "Bit 30 - ADC trigger 1 on Timer E compare 4"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tec4(&mut self) -> AD1TEC4_W<ADC1Rrs> {
        AD1TEC4_W::new(self, 30)
    }
    #[doc = "Bit 31 - ADC trigger 1 on Timer E Period"]
    #[inline(always)]
    #[must_use]
    pub fn ad1teper(&mut self) -> AD1TEPER_W<ADC1Rrs> {
        AD1TEPER_W::new(self, 31)
    }
}
#[doc = "ADC Trigger 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC1Rrs;
impl crate::RegisterSpec for ADC1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc1r::R`](R) reader structure"]
impl crate::Readable for ADC1Rrs {}
#[doc = "`write(|w| ..)` method takes [`adc1r::W`](W) writer structure"]
impl crate::Writable for ADC1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC1R to value 0"]
impl crate::Resettable for ADC1Rrs {
    const RESET_VALUE: u32 = 0;
}
