///Register `AHB2ENR` reader
pub type R = crate::R<AHB2ENRrs>;
///Register `AHB2ENR` writer
pub type W = crate::W<AHB2ENRrs>;
/**IO port A clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOAEN {
    ///0: IO port x clock disabled
    Disabled = 0,
    ///1: IO port x clock enabled
    Enabled = 1,
}
impl From<GPIOAEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOAEN` reader - IO port A clock enable
pub type GPIOAEN_R = crate::BitReader<GPIOAEN>;
impl GPIOAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOAEN {
        match self.bits {
            false => GPIOAEN::Disabled,
            true => GPIOAEN::Enabled,
        }
    }
    ///IO port x clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOAEN::Disabled
    }
    ///IO port x clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOAEN::Enabled
    }
}
///Field `GPIOAEN` writer - IO port A clock enable
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOAEN>;
impl<'a, REG> GPIOAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///IO port x clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Disabled)
    }
    ///IO port x clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Enabled)
    }
}
///Field `GPIOBEN` reader - IO port B clock enable
pub use GPIOAEN_R as GPIOBEN_R;
///Field `GPIOCEN` reader - IO port C clock enable
pub use GPIOAEN_R as GPIOCEN_R;
///Field `GPIODEN` reader - IO port D clock enable
pub use GPIOAEN_R as GPIODEN_R;
///Field `GPIOEEN` reader - IO port E clock enable
pub use GPIOAEN_R as GPIOEEN_R;
///Field `GPIOFEN` reader - IO port F clock enable
pub use GPIOAEN_R as GPIOFEN_R;
///Field `GPIOGEN` reader - IO port G clock enable
pub use GPIOAEN_R as GPIOGEN_R;
///Field `GPIOHEN` reader - IO port H clock enable
pub use GPIOAEN_R as GPIOHEN_R;
///Field `GPIOIEN` reader - IO port I clock enable
pub use GPIOAEN_R as GPIOIEN_R;
///Field `GPIOBEN` writer - IO port B clock enable
pub use GPIOAEN_W as GPIOBEN_W;
///Field `GPIOCEN` writer - IO port C clock enable
pub use GPIOAEN_W as GPIOCEN_W;
///Field `GPIODEN` writer - IO port D clock enable
pub use GPIOAEN_W as GPIODEN_W;
///Field `GPIOEEN` writer - IO port E clock enable
pub use GPIOAEN_W as GPIOEEN_W;
///Field `GPIOFEN` writer - IO port F clock enable
pub use GPIOAEN_W as GPIOFEN_W;
///Field `GPIOGEN` writer - IO port G clock enable
pub use GPIOAEN_W as GPIOGEN_W;
///Field `GPIOHEN` writer - IO port H clock enable
pub use GPIOAEN_W as GPIOHEN_W;
///Field `GPIOIEN` writer - IO port I clock enable
pub use GPIOAEN_W as GPIOIEN_W;
/**OTG full speed clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTGFSEN {
    ///0: USB OTG full speed clock disabled
    Disabled = 0,
    ///1: USB OTG full speed clock enabled
    Enabled = 1,
}
impl From<OTGFSEN> for bool {
    #[inline(always)]
    fn from(variant: OTGFSEN) -> Self {
        variant as u8 != 0
    }
}
///Field `OTGFSEN` reader - OTG full speed clock enable
pub type OTGFSEN_R = crate::BitReader<OTGFSEN>;
impl OTGFSEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OTGFSEN {
        match self.bits {
            false => OTGFSEN::Disabled,
            true => OTGFSEN::Enabled,
        }
    }
    ///USB OTG full speed clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OTGFSEN::Disabled
    }
    ///USB OTG full speed clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OTGFSEN::Enabled
    }
}
///Field `OTGFSEN` writer - OTG full speed clock enable
pub type OTGFSEN_W<'a, REG> = crate::BitWriter<'a, REG, OTGFSEN>;
impl<'a, REG> OTGFSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///USB OTG full speed clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OTGFSEN::Disabled)
    }
    ///USB OTG full speed clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OTGFSEN::Enabled)
    }
}
/**ADC clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCEN {
    ///0: ADC clock disabled
    Disabled = 0,
    ///1: ADC clock enabled
    Enabled = 1,
}
impl From<ADCEN> for bool {
    #[inline(always)]
    fn from(variant: ADCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ADCEN` reader - ADC clock enable
pub type ADCEN_R = crate::BitReader<ADCEN>;
impl ADCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADCEN {
        match self.bits {
            false => ADCEN::Disabled,
            true => ADCEN::Enabled,
        }
    }
    ///ADC clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCEN::Disabled
    }
    ///ADC clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADCEN::Enabled
    }
}
///Field `ADCEN` writer - ADC clock enable
pub type ADCEN_W<'a, REG> = crate::BitWriter<'a, REG, ADCEN>;
impl<'a, REG> ADCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADCEN::Disabled)
    }
    ///ADC clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADCEN::Enabled)
    }
}
/**DCMI clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMIEN {
    ///0: DCMI/PSSI clock disabled
    Disabled = 0,
    ///1: DCMI/PSSI clock enabled
    Enabled = 1,
}
impl From<DCMIEN> for bool {
    #[inline(always)]
    fn from(variant: DCMIEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DCMIEN` reader - DCMI clock enable
pub type DCMIEN_R = crate::BitReader<DCMIEN>;
impl DCMIEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DCMIEN {
        match self.bits {
            false => DCMIEN::Disabled,
            true => DCMIEN::Enabled,
        }
    }
    ///DCMI/PSSI clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCMIEN::Disabled
    }
    ///DCMI/PSSI clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCMIEN::Enabled
    }
}
///Field `DCMIEN` writer - DCMI clock enable
pub type DCMIEN_W<'a, REG> = crate::BitWriter<'a, REG, DCMIEN>;
impl<'a, REG> DCMIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DCMI/PSSI clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMIEN::Disabled)
    }
    ///DCMI/PSSI clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMIEN::Enabled)
    }
}
/**PKA clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PKAEN {
    ///0: PKA clock disabled
    Disabled = 0,
    ///1: PKA clock enabled
    Enabled = 1,
}
impl From<PKAEN> for bool {
    #[inline(always)]
    fn from(variant: PKAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PKAEN` reader - PKA clock enable
pub type PKAEN_R = crate::BitReader<PKAEN>;
impl PKAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PKAEN {
        match self.bits {
            false => PKAEN::Disabled,
            true => PKAEN::Enabled,
        }
    }
    ///PKA clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PKAEN::Disabled
    }
    ///PKA clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PKAEN::Enabled
    }
}
///Field `PKAEN` writer - PKA clock enable
pub type PKAEN_W<'a, REG> = crate::BitWriter<'a, REG, PKAEN>;
impl<'a, REG> PKAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PKA clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PKAEN::Disabled)
    }
    ///PKA clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PKAEN::Enabled)
    }
}
/**AES accelerator clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESEN {
    ///0: AES clock disabled
    Disabled = 0,
    ///1: AES clock enabled
    Enabled = 1,
}
impl From<AESEN> for bool {
    #[inline(always)]
    fn from(variant: AESEN) -> Self {
        variant as u8 != 0
    }
}
///Field `AESEN` reader - AES accelerator clock enable
pub type AESEN_R = crate::BitReader<AESEN>;
impl AESEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AESEN {
        match self.bits {
            false => AESEN::Disabled,
            true => AESEN::Enabled,
        }
    }
    ///AES clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AESEN::Disabled
    }
    ///AES clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AESEN::Enabled
    }
}
///Field `AESEN` writer - AES accelerator clock enable
pub type AESEN_W<'a, REG> = crate::BitWriter<'a, REG, AESEN>;
impl<'a, REG> AESEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AES clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AESEN::Disabled)
    }
    ///AES clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AESEN::Enabled)
    }
}
/**HASH clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HASHEN {
    ///0: HASH clock disabled
    Disabled = 0,
    ///1: HASH clock enabled
    Enabled = 1,
}
impl From<HASHEN> for bool {
    #[inline(always)]
    fn from(variant: HASHEN) -> Self {
        variant as u8 != 0
    }
}
///Field `HASHEN` reader - HASH clock enable
pub type HASHEN_R = crate::BitReader<HASHEN>;
impl HASHEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HASHEN {
        match self.bits {
            false => HASHEN::Disabled,
            true => HASHEN::Enabled,
        }
    }
    ///HASH clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HASHEN::Disabled
    }
    ///HASH clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HASHEN::Enabled
    }
}
///Field `HASHEN` writer - HASH clock enable
pub type HASHEN_W<'a, REG> = crate::BitWriter<'a, REG, HASHEN>;
impl<'a, REG> HASHEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HASH clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HASHEN::Disabled)
    }
    ///HASH clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HASHEN::Enabled)
    }
}
/**Random Number Generator clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGEN {
    ///0: Random Number Generator clock disabled
    Disabled = 0,
    ///1: Random Number Generator clock enabled
    Enabled = 1,
}
impl From<RNGEN> for bool {
    #[inline(always)]
    fn from(variant: RNGEN) -> Self {
        variant as u8 != 0
    }
}
///Field `RNGEN` reader - Random Number Generator clock enable
pub type RNGEN_R = crate::BitReader<RNGEN>;
impl RNGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RNGEN {
        match self.bits {
            false => RNGEN::Disabled,
            true => RNGEN::Enabled,
        }
    }
    ///Random Number Generator clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RNGEN::Disabled
    }
    ///Random Number Generator clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RNGEN::Enabled
    }
}
///Field `RNGEN` writer - Random Number Generator clock enable
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG, RNGEN>;
impl<'a, REG> RNGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Random Number Generator clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RNGEN::Disabled)
    }
    ///Random Number Generator clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RNGEN::Enabled)
    }
}
/**OctoSPI IO manager clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSPIMEN {
    ///0: OctoSPI IO manager clock disabled
    Disabled = 0,
    ///1: OctoSPI IO manager clock enabled
    Enabled = 1,
}
impl From<OSPIMEN> for bool {
    #[inline(always)]
    fn from(variant: OSPIMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `OSPIMEN` reader - OctoSPI IO manager clock enable
pub type OSPIMEN_R = crate::BitReader<OSPIMEN>;
impl OSPIMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSPIMEN {
        match self.bits {
            false => OSPIMEN::Disabled,
            true => OSPIMEN::Enabled,
        }
    }
    ///OctoSPI IO manager clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSPIMEN::Disabled
    }
    ///OctoSPI IO manager clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSPIMEN::Enabled
    }
}
///Field `OSPIMEN` writer - OctoSPI IO manager clock enable
pub type OSPIMEN_W<'a, REG> = crate::BitWriter<'a, REG, OSPIMEN>;
impl<'a, REG> OSPIMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OctoSPI IO manager clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSPIMEN::Disabled)
    }
    ///OctoSPI IO manager clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSPIMEN::Enabled)
    }
}
/**SDMMC1 clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMC1EN {
    ///0: SDMMCx clock disabled
    Disabled = 0,
    ///1: SDMMCx clock enabled
    Enabled = 1,
}
impl From<SDMMC1EN> for bool {
    #[inline(always)]
    fn from(variant: SDMMC1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `SDMMC1EN` reader - SDMMC1 clock enable
pub type SDMMC1EN_R = crate::BitReader<SDMMC1EN>;
impl SDMMC1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDMMC1EN {
        match self.bits {
            false => SDMMC1EN::Disabled,
            true => SDMMC1EN::Enabled,
        }
    }
    ///SDMMCx clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDMMC1EN::Disabled
    }
    ///SDMMCx clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDMMC1EN::Enabled
    }
}
///Field `SDMMC1EN` writer - SDMMC1 clock enable
pub type SDMMC1EN_W<'a, REG> = crate::BitWriter<'a, REG, SDMMC1EN>;
impl<'a, REG> SDMMC1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SDMMCx clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1EN::Disabled)
    }
    ///SDMMCx clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1EN::Enabled)
    }
}
///Field `SDMMC2EN` reader - SDMMC2 clock enable
pub use SDMMC1EN_R as SDMMC2EN_R;
///Field `SDMMC2EN` writer - SDMMC2 clock enable
pub use SDMMC1EN_W as SDMMC2EN_W;
impl R {
    ///Bit 0 - IO port A clock enable
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B clock enable
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C clock enable
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port D clock enable
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port E clock enable
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IO port F clock enable
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port G clock enable
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IO port H clock enable
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - IO port I clock enable
    #[inline(always)]
    pub fn gpioien(&self) -> GPIOIEN_R {
        GPIOIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - OTG full speed clock enable
    #[inline(always)]
    pub fn otgfsen(&self) -> OTGFSEN_R {
        OTGFSEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ADC clock enable
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - DCMI clock enable
    #[inline(always)]
    pub fn dcmien(&self) -> DCMIEN_R {
        DCMIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - PKA clock enable
    #[inline(always)]
    pub fn pkaen(&self) -> PKAEN_R {
        PKAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - AES accelerator clock enable
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HASH clock enable
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Random Number Generator clock enable
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - OctoSPI IO manager clock enable
    #[inline(always)]
    pub fn ospimen(&self) -> OSPIMEN_R {
        OSPIMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - SDMMC1 clock enable
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - SDMMC2 clock enable
    #[inline(always)]
    pub fn sdmmc2en(&self) -> SDMMC2EN_R {
        SDMMC2EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2ENR")
            .field("gpioaen", &self.gpioaen())
            .field("gpioben", &self.gpioben())
            .field("gpiocen", &self.gpiocen())
            .field("gpioden", &self.gpioden())
            .field("gpioeen", &self.gpioeen())
            .field("gpiofen", &self.gpiofen())
            .field("gpiogen", &self.gpiogen())
            .field("gpiohen", &self.gpiohen())
            .field("gpioien", &self.gpioien())
            .field("otgfsen", &self.otgfsen())
            .field("adcen", &self.adcen())
            .field("dcmien", &self.dcmien())
            .field("aesen", &self.aesen())
            .field("hashen", &self.hashen())
            .field("rngen", &self.rngen())
            .field("ospimen", &self.ospimen())
            .field("sdmmc1en", &self.sdmmc1en())
            .field("sdmmc2en", &self.sdmmc2en())
            .field("pkaen", &self.pkaen())
            .finish()
    }
}
impl W {
    ///Bit 0 - IO port A clock enable
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<'_, AHB2ENRrs> {
        GPIOAEN_W::new(self, 0)
    }
    ///Bit 1 - IO port B clock enable
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W<'_, AHB2ENRrs> {
        GPIOBEN_W::new(self, 1)
    }
    ///Bit 2 - IO port C clock enable
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<'_, AHB2ENRrs> {
        GPIOCEN_W::new(self, 2)
    }
    ///Bit 3 - IO port D clock enable
    #[inline(always)]
    pub fn gpioden(&mut self) -> GPIODEN_W<'_, AHB2ENRrs> {
        GPIODEN_W::new(self, 3)
    }
    ///Bit 4 - IO port E clock enable
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<'_, AHB2ENRrs> {
        GPIOEEN_W::new(self, 4)
    }
    ///Bit 5 - IO port F clock enable
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<'_, AHB2ENRrs> {
        GPIOFEN_W::new(self, 5)
    }
    ///Bit 6 - IO port G clock enable
    #[inline(always)]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<'_, AHB2ENRrs> {
        GPIOGEN_W::new(self, 6)
    }
    ///Bit 7 - IO port H clock enable
    #[inline(always)]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<'_, AHB2ENRrs> {
        GPIOHEN_W::new(self, 7)
    }
    ///Bit 8 - IO port I clock enable
    #[inline(always)]
    pub fn gpioien(&mut self) -> GPIOIEN_W<'_, AHB2ENRrs> {
        GPIOIEN_W::new(self, 8)
    }
    ///Bit 12 - OTG full speed clock enable
    #[inline(always)]
    pub fn otgfsen(&mut self) -> OTGFSEN_W<'_, AHB2ENRrs> {
        OTGFSEN_W::new(self, 12)
    }
    ///Bit 13 - ADC clock enable
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W<'_, AHB2ENRrs> {
        ADCEN_W::new(self, 13)
    }
    ///Bit 14 - DCMI clock enable
    #[inline(always)]
    pub fn dcmien(&mut self) -> DCMIEN_W<'_, AHB2ENRrs> {
        DCMIEN_W::new(self, 14)
    }
    ///Bit 15 - PKA clock enable
    #[inline(always)]
    pub fn pkaen(&mut self) -> PKAEN_W<'_, AHB2ENRrs> {
        PKAEN_W::new(self, 15)
    }
    ///Bit 16 - AES accelerator clock enable
    #[inline(always)]
    pub fn aesen(&mut self) -> AESEN_W<'_, AHB2ENRrs> {
        AESEN_W::new(self, 16)
    }
    ///Bit 17 - HASH clock enable
    #[inline(always)]
    pub fn hashen(&mut self) -> HASHEN_W<'_, AHB2ENRrs> {
        HASHEN_W::new(self, 17)
    }
    ///Bit 18 - Random Number Generator clock enable
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<'_, AHB2ENRrs> {
        RNGEN_W::new(self, 18)
    }
    ///Bit 20 - OctoSPI IO manager clock enable
    #[inline(always)]
    pub fn ospimen(&mut self) -> OSPIMEN_W<'_, AHB2ENRrs> {
        OSPIMEN_W::new(self, 20)
    }
    ///Bit 22 - SDMMC1 clock enable
    #[inline(always)]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<'_, AHB2ENRrs> {
        SDMMC1EN_W::new(self, 22)
    }
    ///Bit 23 - SDMMC2 clock enable
    #[inline(always)]
    pub fn sdmmc2en(&mut self) -> SDMMC2EN_W<'_, AHB2ENRrs> {
        SDMMC2EN_W::new(self, 23)
    }
}
/**AHB2 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#RCC:AHB2ENR)*/
pub struct AHB2ENRrs;
impl crate::RegisterSpec for AHB2ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2enr::R`](R) reader structure
impl crate::Readable for AHB2ENRrs {}
///`write(|w| ..)` method takes [`ahb2enr::W`](W) writer structure
impl crate::Writable for AHB2ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2ENR to value 0
impl crate::Resettable for AHB2ENRrs {}
