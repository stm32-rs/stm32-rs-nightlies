#[doc = "Register `AHB2RSTR` reader"]
pub type R = crate::R<AHB2RSTRrs>;
#[doc = "Register `AHB2RSTR` writer"]
pub type W = crate::W<AHB2RSTRrs>;
#[doc = "IO port A reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOARST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset GPIO port x"]
    Reset = 1,
}
impl From<GPIOARST> for bool {
    #[inline(always)]
    fn from(variant: GPIOARST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOARST` reader - IO port A reset"]
pub type GPIOARST_R = crate::BitReader<GPIOARST>;
impl GPIOARST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIOARST {
        match self.bits {
            false => GPIOARST::NoEffect,
            true => GPIOARST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == GPIOARST::NoEffect
    }
    #[doc = "Reset GPIO port x"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == GPIOARST::Reset
    }
}
#[doc = "Field `GPIOARST` writer - IO port A reset"]
pub type GPIOARST_W<'a, REG> = crate::BitWriter<'a, REG, GPIOARST>;
impl<'a, REG> GPIOARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOARST::NoEffect)
    }
    #[doc = "Reset GPIO port x"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOARST::Reset)
    }
}
#[doc = "Field `GPIOBRST` reader - IO port B reset"]
pub use GPIOARST_R as GPIOBRST_R;
#[doc = "Field `GPIOCRST` reader - IO port C reset"]
pub use GPIOARST_R as GPIOCRST_R;
#[doc = "Field `GPIODRST` reader - IO port D reset"]
pub use GPIOARST_R as GPIODRST_R;
#[doc = "Field `GPIOERST` reader - IO port E reset"]
pub use GPIOARST_R as GPIOERST_R;
#[doc = "Field `GPIOFRST` reader - IO port F reset"]
pub use GPIOARST_R as GPIOFRST_R;
#[doc = "Field `GPIOGRST` reader - IO port G reset"]
pub use GPIOARST_R as GPIOGRST_R;
#[doc = "Field `GPIOHRST` reader - IO port H reset"]
pub use GPIOARST_R as GPIOHRST_R;
#[doc = "Field `GPIOIRST` reader - IO port I reset"]
pub use GPIOARST_R as GPIOIRST_R;
#[doc = "Field `GPIOBRST` writer - IO port B reset"]
pub use GPIOARST_W as GPIOBRST_W;
#[doc = "Field `GPIOCRST` writer - IO port C reset"]
pub use GPIOARST_W as GPIOCRST_W;
#[doc = "Field `GPIODRST` writer - IO port D reset"]
pub use GPIOARST_W as GPIODRST_W;
#[doc = "Field `GPIOERST` writer - IO port E reset"]
pub use GPIOARST_W as GPIOERST_W;
#[doc = "Field `GPIOFRST` writer - IO port F reset"]
pub use GPIOARST_W as GPIOFRST_W;
#[doc = "Field `GPIOGRST` writer - IO port G reset"]
pub use GPIOARST_W as GPIOGRST_W;
#[doc = "Field `GPIOHRST` writer - IO port H reset"]
pub use GPIOARST_W as GPIOHRST_W;
#[doc = "Field `GPIOIRST` writer - IO port I reset"]
pub use GPIOARST_W as GPIOIRST_W;
#[doc = "USB OTG FS reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTGFSRST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset USB OTG FS"]
    Reset = 1,
}
impl From<OTGFSRST> for bool {
    #[inline(always)]
    fn from(variant: OTGFSRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTGFSRST` reader - USB OTG FS reset"]
pub type OTGFSRST_R = crate::BitReader<OTGFSRST>;
impl OTGFSRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OTGFSRST {
        match self.bits {
            false => OTGFSRST::NoEffect,
            true => OTGFSRST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == OTGFSRST::NoEffect
    }
    #[doc = "Reset USB OTG FS"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OTGFSRST::Reset
    }
}
#[doc = "Field `OTGFSRST` writer - USB OTG FS reset"]
pub type OTGFSRST_W<'a, REG> = crate::BitWriter<'a, REG, OTGFSRST>;
impl<'a, REG> OTGFSRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(OTGFSRST::NoEffect)
    }
    #[doc = "Reset USB OTG FS"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(OTGFSRST::Reset)
    }
}
#[doc = "ADC reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCRST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset ADC"]
    Reset = 1,
}
impl From<ADCRST> for bool {
    #[inline(always)]
    fn from(variant: ADCRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCRST` reader - ADC reset"]
pub type ADCRST_R = crate::BitReader<ADCRST>;
impl ADCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADCRST {
        match self.bits {
            false => ADCRST::NoEffect,
            true => ADCRST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == ADCRST::NoEffect
    }
    #[doc = "Reset ADC"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ADCRST::Reset
    }
}
#[doc = "Field `ADCRST` writer - ADC reset"]
pub type ADCRST_W<'a, REG> = crate::BitWriter<'a, REG, ADCRST>;
impl<'a, REG> ADCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(ADCRST::NoEffect)
    }
    #[doc = "Reset ADC"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(ADCRST::Reset)
    }
}
#[doc = "Digital Camera Interface reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMIRST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset DCMI/PSSI interface"]
    Reset = 1,
}
impl From<DCMIRST> for bool {
    #[inline(always)]
    fn from(variant: DCMIRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMIRST` reader - Digital Camera Interface reset"]
pub type DCMIRST_R = crate::BitReader<DCMIRST>;
impl DCMIRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCMIRST {
        match self.bits {
            false => DCMIRST::NoEffect,
            true => DCMIRST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DCMIRST::NoEffect
    }
    #[doc = "Reset DCMI/PSSI interface"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DCMIRST::Reset
    }
}
#[doc = "Field `DCMIRST` writer - Digital Camera Interface reset"]
pub type DCMIRST_W<'a, REG> = crate::BitWriter<'a, REG, DCMIRST>;
impl<'a, REG> DCMIRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(DCMIRST::NoEffect)
    }
    #[doc = "Reset DCMI/PSSI interface"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DCMIRST::Reset)
    }
}
#[doc = "PKA reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PKARST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset PKA"]
    Reset = 1,
}
impl From<PKARST> for bool {
    #[inline(always)]
    fn from(variant: PKARST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PKARST` reader - PKA reset"]
pub type PKARST_R = crate::BitReader<PKARST>;
impl PKARST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PKARST {
        match self.bits {
            false => PKARST::NoEffect,
            true => PKARST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PKARST::NoEffect
    }
    #[doc = "Reset PKA"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PKARST::Reset
    }
}
#[doc = "Field `PKARST` writer - PKA reset"]
pub type PKARST_W<'a, REG> = crate::BitWriter<'a, REG, PKARST>;
impl<'a, REG> PKARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(PKARST::NoEffect)
    }
    #[doc = "Reset PKA"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(PKARST::Reset)
    }
}
#[doc = "AES hardware accelerator reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESRST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset AES"]
    Reset = 1,
}
impl From<AESRST> for bool {
    #[inline(always)]
    fn from(variant: AESRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESRST` reader - AES hardware accelerator reset"]
pub type AESRST_R = crate::BitReader<AESRST>;
impl AESRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AESRST {
        match self.bits {
            false => AESRST::NoEffect,
            true => AESRST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == AESRST::NoEffect
    }
    #[doc = "Reset AES"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == AESRST::Reset
    }
}
#[doc = "Field `AESRST` writer - AES hardware accelerator reset"]
pub type AESRST_W<'a, REG> = crate::BitWriter<'a, REG, AESRST>;
impl<'a, REG> AESRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(AESRST::NoEffect)
    }
    #[doc = "Reset AES"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(AESRST::Reset)
    }
}
#[doc = "Hash reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HASHRST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset HASH"]
    Reset = 1,
}
impl From<HASHRST> for bool {
    #[inline(always)]
    fn from(variant: HASHRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASHRST` reader - Hash reset"]
pub type HASHRST_R = crate::BitReader<HASHRST>;
impl HASHRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HASHRST {
        match self.bits {
            false => HASHRST::NoEffect,
            true => HASHRST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == HASHRST::NoEffect
    }
    #[doc = "Reset HASH"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == HASHRST::Reset
    }
}
#[doc = "Field `HASHRST` writer - Hash reset"]
pub type HASHRST_W<'a, REG> = crate::BitWriter<'a, REG, HASHRST>;
impl<'a, REG> HASHRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(HASHRST::NoEffect)
    }
    #[doc = "Reset HASH"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(HASHRST::Reset)
    }
}
#[doc = "Random number generator reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGRST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset RNG"]
    Reset = 1,
}
impl From<RNGRST> for bool {
    #[inline(always)]
    fn from(variant: RNGRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNGRST` reader - Random number generator reset"]
pub type RNGRST_R = crate::BitReader<RNGRST>;
impl RNGRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RNGRST {
        match self.bits {
            false => RNGRST::NoEffect,
            true => RNGRST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RNGRST::NoEffect
    }
    #[doc = "Reset RNG"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RNGRST::Reset
    }
}
#[doc = "Field `RNGRST` writer - Random number generator reset"]
pub type RNGRST_W<'a, REG> = crate::BitWriter<'a, REG, RNGRST>;
impl<'a, REG> RNGRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(RNGRST::NoEffect)
    }
    #[doc = "Reset RNG"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(RNGRST::Reset)
    }
}
#[doc = "OCTOSPI IO manager reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSPIMRST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset OctoSPI IO manager"]
    Reset = 1,
}
impl From<OSPIMRST> for bool {
    #[inline(always)]
    fn from(variant: OSPIMRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSPIMRST` reader - OCTOSPI IO manager reset"]
pub type OSPIMRST_R = crate::BitReader<OSPIMRST>;
impl OSPIMRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSPIMRST {
        match self.bits {
            false => OSPIMRST::NoEffect,
            true => OSPIMRST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == OSPIMRST::NoEffect
    }
    #[doc = "Reset OctoSPI IO manager"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OSPIMRST::Reset
    }
}
#[doc = "Field `OSPIMRST` writer - OCTOSPI IO manager reset"]
pub type OSPIMRST_W<'a, REG> = crate::BitWriter<'a, REG, OSPIMRST>;
impl<'a, REG> OSPIMRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(OSPIMRST::NoEffect)
    }
    #[doc = "Reset OctoSPI IO manager"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(OSPIMRST::Reset)
    }
}
#[doc = "SDMMC1 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMC1RST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset SDMMC1"]
    Reset = 1,
}
impl From<SDMMC1RST> for bool {
    #[inline(always)]
    fn from(variant: SDMMC1RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMMC1RST` reader - SDMMC1 reset"]
pub type SDMMC1RST_R = crate::BitReader<SDMMC1RST>;
impl SDMMC1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDMMC1RST {
        match self.bits {
            false => SDMMC1RST::NoEffect,
            true => SDMMC1RST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SDMMC1RST::NoEffect
    }
    #[doc = "Reset SDMMC1"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SDMMC1RST::Reset
    }
}
#[doc = "Field `SDMMC1RST` writer - SDMMC1 reset"]
pub type SDMMC1RST_W<'a, REG> = crate::BitWriter<'a, REG, SDMMC1RST>;
impl<'a, REG> SDMMC1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1RST::NoEffect)
    }
    #[doc = "Reset SDMMC1"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1RST::Reset)
    }
}
#[doc = "SDMMC2 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMC2RST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset SDMMC2"]
    Reset = 1,
}
impl From<SDMMC2RST> for bool {
    #[inline(always)]
    fn from(variant: SDMMC2RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMMC2RST` reader - SDMMC2 reset"]
pub type SDMMC2RST_R = crate::BitReader<SDMMC2RST>;
impl SDMMC2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDMMC2RST {
        match self.bits {
            false => SDMMC2RST::NoEffect,
            true => SDMMC2RST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SDMMC2RST::NoEffect
    }
    #[doc = "Reset SDMMC2"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SDMMC2RST::Reset
    }
}
#[doc = "Field `SDMMC2RST` writer - SDMMC2 reset"]
pub type SDMMC2RST_W<'a, REG> = crate::BitWriter<'a, REG, SDMMC2RST>;
impl<'a, REG> SDMMC2RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC2RST::NoEffect)
    }
    #[doc = "Reset SDMMC2"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC2RST::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - IO port A reset"]
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B reset"]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C reset"]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D reset"]
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E reset"]
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F reset"]
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G reset"]
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H reset"]
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IO port I reset"]
    #[inline(always)]
    pub fn gpioirst(&self) -> GPIOIRST_R {
        GPIOIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - USB OTG FS reset"]
    #[inline(always)]
    pub fn otgfsrst(&self) -> OTGFSRST_R {
        OTGFSRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC reset"]
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Digital Camera Interface reset"]
    #[inline(always)]
    pub fn dcmirst(&self) -> DCMIRST_R {
        DCMIRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PKA reset"]
    #[inline(always)]
    pub fn pkarst(&self) -> PKARST_R {
        PKARST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - AES hardware accelerator reset"]
    #[inline(always)]
    pub fn aesrst(&self) -> AESRST_R {
        AESRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Hash reset"]
    #[inline(always)]
    pub fn hashrst(&self) -> HASHRST_R {
        HASHRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Random number generator reset"]
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - OCTOSPI IO manager reset"]
    #[inline(always)]
    pub fn ospimrst(&self) -> OSPIMRST_R {
        OSPIMRST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - SDMMC1 reset"]
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SDMMC2 reset"]
    #[inline(always)]
    pub fn sdmmc2rst(&self) -> SDMMC2RST_R {
        SDMMC2RST_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GPIOARST_W<AHB2RSTRrs> {
        GPIOARST_W::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<AHB2RSTRrs> {
        GPIOBRST_W::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<AHB2RSTRrs> {
        GPIOCRST_W::new(self, 2)
    }
    #[doc = "Bit 3 - IO port D reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<AHB2RSTRrs> {
        GPIODRST_W::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioerst(&mut self) -> GPIOERST_W<AHB2RSTRrs> {
        GPIOERST_W::new(self, 4)
    }
    #[doc = "Bit 5 - IO port F reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<AHB2RSTRrs> {
        GPIOFRST_W::new(self, 5)
    }
    #[doc = "Bit 6 - IO port G reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<AHB2RSTRrs> {
        GPIOGRST_W::new(self, 6)
    }
    #[doc = "Bit 7 - IO port H reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<AHB2RSTRrs> {
        GPIOHRST_W::new(self, 7)
    }
    #[doc = "Bit 8 - IO port I reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioirst(&mut self) -> GPIOIRST_W<AHB2RSTRrs> {
        GPIOIRST_W::new(self, 8)
    }
    #[doc = "Bit 12 - USB OTG FS reset"]
    #[inline(always)]
    #[must_use]
    pub fn otgfsrst(&mut self) -> OTGFSRST_W<AHB2RSTRrs> {
        OTGFSRST_W::new(self, 12)
    }
    #[doc = "Bit 13 - ADC reset"]
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> ADCRST_W<AHB2RSTRrs> {
        ADCRST_W::new(self, 13)
    }
    #[doc = "Bit 14 - Digital Camera Interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn dcmirst(&mut self) -> DCMIRST_W<AHB2RSTRrs> {
        DCMIRST_W::new(self, 14)
    }
    #[doc = "Bit 15 - PKA reset"]
    #[inline(always)]
    #[must_use]
    pub fn pkarst(&mut self) -> PKARST_W<AHB2RSTRrs> {
        PKARST_W::new(self, 15)
    }
    #[doc = "Bit 16 - AES hardware accelerator reset"]
    #[inline(always)]
    #[must_use]
    pub fn aesrst(&mut self) -> AESRST_W<AHB2RSTRrs> {
        AESRST_W::new(self, 16)
    }
    #[doc = "Bit 17 - Hash reset"]
    #[inline(always)]
    #[must_use]
    pub fn hashrst(&mut self) -> HASHRST_W<AHB2RSTRrs> {
        HASHRST_W::new(self, 17)
    }
    #[doc = "Bit 18 - Random number generator reset"]
    #[inline(always)]
    #[must_use]
    pub fn rngrst(&mut self) -> RNGRST_W<AHB2RSTRrs> {
        RNGRST_W::new(self, 18)
    }
    #[doc = "Bit 20 - OCTOSPI IO manager reset"]
    #[inline(always)]
    #[must_use]
    pub fn ospimrst(&mut self) -> OSPIMRST_W<AHB2RSTRrs> {
        OSPIMRST_W::new(self, 20)
    }
    #[doc = "Bit 22 - SDMMC1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W<AHB2RSTRrs> {
        SDMMC1RST_W::new(self, 22)
    }
    #[doc = "Bit 23 - SDMMC2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2rst(&mut self) -> SDMMC2RST_W<AHB2RSTRrs> {
        SDMMC2RST_W::new(self, 23)
    }
}
#[doc = "AHB2 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2RSTRrs;
impl crate::RegisterSpec for AHB2RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2rstr::R`](R) reader structure"]
impl crate::Readable for AHB2RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb2rstr::W`](W) writer structure"]
impl crate::Writable for AHB2RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB2RSTR to value 0"]
impl crate::Resettable for AHB2RSTRrs {
    const RESET_VALUE: u32 = 0;
}
