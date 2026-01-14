///Register `AHB2RSTR` reader
pub type R = crate::R<AHB2RSTRrs>;
///Register `AHB2RSTR` writer
pub type W = crate::W<AHB2RSTRrs>;
/**IO port A reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOARST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset GPIO port x
    Reset = 1,
}
impl From<GPIOARST> for bool {
    #[inline(always)]
    fn from(variant: GPIOARST) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOARST` reader - IO port A reset
pub type GPIOARST_R = crate::BitReader<GPIOARST>;
impl GPIOARST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOARST {
        match self.bits {
            false => GPIOARST::NoEffect,
            true => GPIOARST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == GPIOARST::NoEffect
    }
    ///Reset GPIO port x
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == GPIOARST::Reset
    }
}
///Field `GPIOARST` writer - IO port A reset
pub type GPIOARST_W<'a, REG> = crate::BitWriter<'a, REG, GPIOARST>;
impl<'a, REG> GPIOARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOARST::NoEffect)
    }
    ///Reset GPIO port x
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOARST::Reset)
    }
}
///Field `GPIOBRST` reader - IO port B reset
pub use GPIOARST_R as GPIOBRST_R;
///Field `GPIOCRST` reader - IO port C reset
pub use GPIOARST_R as GPIOCRST_R;
///Field `GPIODRST` reader - IO port D reset
pub use GPIOARST_R as GPIODRST_R;
///Field `GPIOERST` reader - IO port E reset
pub use GPIOARST_R as GPIOERST_R;
///Field `GPIOFRST` reader - IO port F reset
pub use GPIOARST_R as GPIOFRST_R;
///Field `GPIOGRST` reader - IO port G reset
pub use GPIOARST_R as GPIOGRST_R;
///Field `GPIOHRST` reader - IO port H reset
pub use GPIOARST_R as GPIOHRST_R;
///Field `GPIOIRST` reader - IO port I reset
pub use GPIOARST_R as GPIOIRST_R;
///Field `GPIOBRST` writer - IO port B reset
pub use GPIOARST_W as GPIOBRST_W;
///Field `GPIOCRST` writer - IO port C reset
pub use GPIOARST_W as GPIOCRST_W;
///Field `GPIODRST` writer - IO port D reset
pub use GPIOARST_W as GPIODRST_W;
///Field `GPIOERST` writer - IO port E reset
pub use GPIOARST_W as GPIOERST_W;
///Field `GPIOFRST` writer - IO port F reset
pub use GPIOARST_W as GPIOFRST_W;
///Field `GPIOGRST` writer - IO port G reset
pub use GPIOARST_W as GPIOGRST_W;
///Field `GPIOHRST` writer - IO port H reset
pub use GPIOARST_W as GPIOHRST_W;
///Field `GPIOIRST` writer - IO port I reset
pub use GPIOARST_W as GPIOIRST_W;
/**USB OTG FS reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTGFSRST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset USB OTG FS
    Reset = 1,
}
impl From<OTGFSRST> for bool {
    #[inline(always)]
    fn from(variant: OTGFSRST) -> Self {
        variant as u8 != 0
    }
}
///Field `OTGFSRST` reader - USB OTG FS reset
pub type OTGFSRST_R = crate::BitReader<OTGFSRST>;
impl OTGFSRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OTGFSRST {
        match self.bits {
            false => OTGFSRST::NoEffect,
            true => OTGFSRST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == OTGFSRST::NoEffect
    }
    ///Reset USB OTG FS
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OTGFSRST::Reset
    }
}
///Field `OTGFSRST` writer - USB OTG FS reset
pub type OTGFSRST_W<'a, REG> = crate::BitWriter<'a, REG, OTGFSRST>;
impl<'a, REG> OTGFSRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(OTGFSRST::NoEffect)
    }
    ///Reset USB OTG FS
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(OTGFSRST::Reset)
    }
}
/**ADC reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCRST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset ADC
    Reset = 1,
}
impl From<ADCRST> for bool {
    #[inline(always)]
    fn from(variant: ADCRST) -> Self {
        variant as u8 != 0
    }
}
///Field `ADCRST` reader - ADC reset
pub type ADCRST_R = crate::BitReader<ADCRST>;
impl ADCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADCRST {
        match self.bits {
            false => ADCRST::NoEffect,
            true => ADCRST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == ADCRST::NoEffect
    }
    ///Reset ADC
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ADCRST::Reset
    }
}
///Field `ADCRST` writer - ADC reset
pub type ADCRST_W<'a, REG> = crate::BitWriter<'a, REG, ADCRST>;
impl<'a, REG> ADCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(ADCRST::NoEffect)
    }
    ///Reset ADC
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(ADCRST::Reset)
    }
}
/**Digital Camera Interface reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMIRST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset DCMI/PSSI interface
    Reset = 1,
}
impl From<DCMIRST> for bool {
    #[inline(always)]
    fn from(variant: DCMIRST) -> Self {
        variant as u8 != 0
    }
}
///Field `DCMIRST` reader - Digital Camera Interface reset
pub type DCMIRST_R = crate::BitReader<DCMIRST>;
impl DCMIRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DCMIRST {
        match self.bits {
            false => DCMIRST::NoEffect,
            true => DCMIRST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DCMIRST::NoEffect
    }
    ///Reset DCMI/PSSI interface
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DCMIRST::Reset
    }
}
///Field `DCMIRST` writer - Digital Camera Interface reset
pub type DCMIRST_W<'a, REG> = crate::BitWriter<'a, REG, DCMIRST>;
impl<'a, REG> DCMIRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(DCMIRST::NoEffect)
    }
    ///Reset DCMI/PSSI interface
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DCMIRST::Reset)
    }
}
/**PKA reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PKARST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset PKA
    Reset = 1,
}
impl From<PKARST> for bool {
    #[inline(always)]
    fn from(variant: PKARST) -> Self {
        variant as u8 != 0
    }
}
///Field `PKARST` reader - PKA reset
pub type PKARST_R = crate::BitReader<PKARST>;
impl PKARST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PKARST {
        match self.bits {
            false => PKARST::NoEffect,
            true => PKARST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PKARST::NoEffect
    }
    ///Reset PKA
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PKARST::Reset
    }
}
///Field `PKARST` writer - PKA reset
pub type PKARST_W<'a, REG> = crate::BitWriter<'a, REG, PKARST>;
impl<'a, REG> PKARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(PKARST::NoEffect)
    }
    ///Reset PKA
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(PKARST::Reset)
    }
}
/**AES hardware accelerator reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESRST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset AES
    Reset = 1,
}
impl From<AESRST> for bool {
    #[inline(always)]
    fn from(variant: AESRST) -> Self {
        variant as u8 != 0
    }
}
///Field `AESRST` reader - AES hardware accelerator reset
pub type AESRST_R = crate::BitReader<AESRST>;
impl AESRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AESRST {
        match self.bits {
            false => AESRST::NoEffect,
            true => AESRST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == AESRST::NoEffect
    }
    ///Reset AES
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == AESRST::Reset
    }
}
///Field `AESRST` writer - AES hardware accelerator reset
pub type AESRST_W<'a, REG> = crate::BitWriter<'a, REG, AESRST>;
impl<'a, REG> AESRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(AESRST::NoEffect)
    }
    ///Reset AES
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(AESRST::Reset)
    }
}
/**Hash reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HASHRST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset HASH
    Reset = 1,
}
impl From<HASHRST> for bool {
    #[inline(always)]
    fn from(variant: HASHRST) -> Self {
        variant as u8 != 0
    }
}
///Field `HASHRST` reader - Hash reset
pub type HASHRST_R = crate::BitReader<HASHRST>;
impl HASHRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HASHRST {
        match self.bits {
            false => HASHRST::NoEffect,
            true => HASHRST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == HASHRST::NoEffect
    }
    ///Reset HASH
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == HASHRST::Reset
    }
}
///Field `HASHRST` writer - Hash reset
pub type HASHRST_W<'a, REG> = crate::BitWriter<'a, REG, HASHRST>;
impl<'a, REG> HASHRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(HASHRST::NoEffect)
    }
    ///Reset HASH
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(HASHRST::Reset)
    }
}
/**Random number generator reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGRST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset RNG
    Reset = 1,
}
impl From<RNGRST> for bool {
    #[inline(always)]
    fn from(variant: RNGRST) -> Self {
        variant as u8 != 0
    }
}
///Field `RNGRST` reader - Random number generator reset
pub type RNGRST_R = crate::BitReader<RNGRST>;
impl RNGRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RNGRST {
        match self.bits {
            false => RNGRST::NoEffect,
            true => RNGRST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RNGRST::NoEffect
    }
    ///Reset RNG
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RNGRST::Reset
    }
}
///Field `RNGRST` writer - Random number generator reset
pub type RNGRST_W<'a, REG> = crate::BitWriter<'a, REG, RNGRST>;
impl<'a, REG> RNGRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(RNGRST::NoEffect)
    }
    ///Reset RNG
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(RNGRST::Reset)
    }
}
/**OCTOSPI IO manager reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSPIMRST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset OctoSPI IO manager
    Reset = 1,
}
impl From<OSPIMRST> for bool {
    #[inline(always)]
    fn from(variant: OSPIMRST) -> Self {
        variant as u8 != 0
    }
}
///Field `OSPIMRST` reader - OCTOSPI IO manager reset
pub type OSPIMRST_R = crate::BitReader<OSPIMRST>;
impl OSPIMRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSPIMRST {
        match self.bits {
            false => OSPIMRST::NoEffect,
            true => OSPIMRST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == OSPIMRST::NoEffect
    }
    ///Reset OctoSPI IO manager
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OSPIMRST::Reset
    }
}
///Field `OSPIMRST` writer - OCTOSPI IO manager reset
pub type OSPIMRST_W<'a, REG> = crate::BitWriter<'a, REG, OSPIMRST>;
impl<'a, REG> OSPIMRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(OSPIMRST::NoEffect)
    }
    ///Reset OctoSPI IO manager
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(OSPIMRST::Reset)
    }
}
/**SDMMC1 reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMC1RST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset SDMMC1
    Reset = 1,
}
impl From<SDMMC1RST> for bool {
    #[inline(always)]
    fn from(variant: SDMMC1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `SDMMC1RST` reader - SDMMC1 reset
pub type SDMMC1RST_R = crate::BitReader<SDMMC1RST>;
impl SDMMC1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDMMC1RST {
        match self.bits {
            false => SDMMC1RST::NoEffect,
            true => SDMMC1RST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SDMMC1RST::NoEffect
    }
    ///Reset SDMMC1
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SDMMC1RST::Reset
    }
}
///Field `SDMMC1RST` writer - SDMMC1 reset
pub type SDMMC1RST_W<'a, REG> = crate::BitWriter<'a, REG, SDMMC1RST>;
impl<'a, REG> SDMMC1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1RST::NoEffect)
    }
    ///Reset SDMMC1
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1RST::Reset)
    }
}
/**SDMMC2 reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMC2RST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset SDMMC2
    Reset = 1,
}
impl From<SDMMC2RST> for bool {
    #[inline(always)]
    fn from(variant: SDMMC2RST) -> Self {
        variant as u8 != 0
    }
}
///Field `SDMMC2RST` reader - SDMMC2 reset
pub type SDMMC2RST_R = crate::BitReader<SDMMC2RST>;
impl SDMMC2RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDMMC2RST {
        match self.bits {
            false => SDMMC2RST::NoEffect,
            true => SDMMC2RST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SDMMC2RST::NoEffect
    }
    ///Reset SDMMC2
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SDMMC2RST::Reset
    }
}
///Field `SDMMC2RST` writer - SDMMC2 reset
pub type SDMMC2RST_W<'a, REG> = crate::BitWriter<'a, REG, SDMMC2RST>;
impl<'a, REG> SDMMC2RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC2RST::NoEffect)
    }
    ///Reset SDMMC2
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC2RST::Reset)
    }
}
impl R {
    ///Bit 0 - IO port A reset
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B reset
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C reset
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port D reset
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port E reset
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IO port F reset
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port G reset
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IO port H reset
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - IO port I reset
    #[inline(always)]
    pub fn gpioirst(&self) -> GPIOIRST_R {
        GPIOIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - USB OTG FS reset
    #[inline(always)]
    pub fn otgfsrst(&self) -> OTGFSRST_R {
        OTGFSRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ADC reset
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Digital Camera Interface reset
    #[inline(always)]
    pub fn dcmirst(&self) -> DCMIRST_R {
        DCMIRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - PKA reset
    #[inline(always)]
    pub fn pkarst(&self) -> PKARST_R {
        PKARST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - AES hardware accelerator reset
    #[inline(always)]
    pub fn aesrst(&self) -> AESRST_R {
        AESRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Hash reset
    #[inline(always)]
    pub fn hashrst(&self) -> HASHRST_R {
        HASHRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Random number generator reset
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - OCTOSPI IO manager reset
    #[inline(always)]
    pub fn ospimrst(&self) -> OSPIMRST_R {
        OSPIMRST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - SDMMC1 reset
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - SDMMC2 reset
    #[inline(always)]
    pub fn sdmmc2rst(&self) -> SDMMC2RST_R {
        SDMMC2RST_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2RSTR")
            .field("gpioarst", &self.gpioarst())
            .field("gpiobrst", &self.gpiobrst())
            .field("gpiocrst", &self.gpiocrst())
            .field("gpiodrst", &self.gpiodrst())
            .field("gpioerst", &self.gpioerst())
            .field("gpiofrst", &self.gpiofrst())
            .field("gpiogrst", &self.gpiogrst())
            .field("gpiohrst", &self.gpiohrst())
            .field("gpioirst", &self.gpioirst())
            .field("otgfsrst", &self.otgfsrst())
            .field("adcrst", &self.adcrst())
            .field("dcmirst", &self.dcmirst())
            .field("aesrst", &self.aesrst())
            .field("hashrst", &self.hashrst())
            .field("rngrst", &self.rngrst())
            .field("ospimrst", &self.ospimrst())
            .field("sdmmc1rst", &self.sdmmc1rst())
            .field("sdmmc2rst", &self.sdmmc2rst())
            .field("pkarst", &self.pkarst())
            .finish()
    }
}
impl W {
    ///Bit 0 - IO port A reset
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GPIOARST_W<'_, AHB2RSTRrs> {
        GPIOARST_W::new(self, 0)
    }
    ///Bit 1 - IO port B reset
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<'_, AHB2RSTRrs> {
        GPIOBRST_W::new(self, 1)
    }
    ///Bit 2 - IO port C reset
    #[inline(always)]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<'_, AHB2RSTRrs> {
        GPIOCRST_W::new(self, 2)
    }
    ///Bit 3 - IO port D reset
    #[inline(always)]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<'_, AHB2RSTRrs> {
        GPIODRST_W::new(self, 3)
    }
    ///Bit 4 - IO port E reset
    #[inline(always)]
    pub fn gpioerst(&mut self) -> GPIOERST_W<'_, AHB2RSTRrs> {
        GPIOERST_W::new(self, 4)
    }
    ///Bit 5 - IO port F reset
    #[inline(always)]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<'_, AHB2RSTRrs> {
        GPIOFRST_W::new(self, 5)
    }
    ///Bit 6 - IO port G reset
    #[inline(always)]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<'_, AHB2RSTRrs> {
        GPIOGRST_W::new(self, 6)
    }
    ///Bit 7 - IO port H reset
    #[inline(always)]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<'_, AHB2RSTRrs> {
        GPIOHRST_W::new(self, 7)
    }
    ///Bit 8 - IO port I reset
    #[inline(always)]
    pub fn gpioirst(&mut self) -> GPIOIRST_W<'_, AHB2RSTRrs> {
        GPIOIRST_W::new(self, 8)
    }
    ///Bit 12 - USB OTG FS reset
    #[inline(always)]
    pub fn otgfsrst(&mut self) -> OTGFSRST_W<'_, AHB2RSTRrs> {
        OTGFSRST_W::new(self, 12)
    }
    ///Bit 13 - ADC reset
    #[inline(always)]
    pub fn adcrst(&mut self) -> ADCRST_W<'_, AHB2RSTRrs> {
        ADCRST_W::new(self, 13)
    }
    ///Bit 14 - Digital Camera Interface reset
    #[inline(always)]
    pub fn dcmirst(&mut self) -> DCMIRST_W<'_, AHB2RSTRrs> {
        DCMIRST_W::new(self, 14)
    }
    ///Bit 15 - PKA reset
    #[inline(always)]
    pub fn pkarst(&mut self) -> PKARST_W<'_, AHB2RSTRrs> {
        PKARST_W::new(self, 15)
    }
    ///Bit 16 - AES hardware accelerator reset
    #[inline(always)]
    pub fn aesrst(&mut self) -> AESRST_W<'_, AHB2RSTRrs> {
        AESRST_W::new(self, 16)
    }
    ///Bit 17 - Hash reset
    #[inline(always)]
    pub fn hashrst(&mut self) -> HASHRST_W<'_, AHB2RSTRrs> {
        HASHRST_W::new(self, 17)
    }
    ///Bit 18 - Random number generator reset
    #[inline(always)]
    pub fn rngrst(&mut self) -> RNGRST_W<'_, AHB2RSTRrs> {
        RNGRST_W::new(self, 18)
    }
    ///Bit 20 - OCTOSPI IO manager reset
    #[inline(always)]
    pub fn ospimrst(&mut self) -> OSPIMRST_W<'_, AHB2RSTRrs> {
        OSPIMRST_W::new(self, 20)
    }
    ///Bit 22 - SDMMC1 reset
    #[inline(always)]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W<'_, AHB2RSTRrs> {
        SDMMC1RST_W::new(self, 22)
    }
    ///Bit 23 - SDMMC2 reset
    #[inline(always)]
    pub fn sdmmc2rst(&mut self) -> SDMMC2RST_W<'_, AHB2RSTRrs> {
        SDMMC2RST_W::new(self, 23)
    }
}
/**AHB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#RCC:AHB2RSTR)*/
pub struct AHB2RSTRrs;
impl crate::RegisterSpec for AHB2RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2rstr::R`](R) reader structure
impl crate::Readable for AHB2RSTRrs {}
///`write(|w| ..)` method takes [`ahb2rstr::W`](W) writer structure
impl crate::Writable for AHB2RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2RSTR to value 0
impl crate::Resettable for AHB2RSTRrs {}
