///Register `AHB2SMENR` reader
pub type R = crate::R<AHB2SMENRrs>;
///Register `AHB2SMENR` writer
pub type W = crate::W<AHB2SMENRrs>;
/**IO port A clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOASMEN {
    ///0: IO port x clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: IO port x clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<GPIOASMEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOASMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOASMEN` reader - IO port A clocks enable during Sleep and Stop modes
pub type GPIOASMEN_R = crate::BitReader<GPIOASMEN>;
impl GPIOASMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOASMEN {
        match self.bits {
            false => GPIOASMEN::Disabled,
            true => GPIOASMEN::Enabled,
        }
    }
    ///IO port x clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOASMEN::Disabled
    }
    ///IO port x clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOASMEN::Enabled
    }
}
///Field `GPIOASMEN` writer - IO port A clocks enable during Sleep and Stop modes
pub type GPIOASMEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOASMEN>;
impl<'a, REG> GPIOASMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///IO port x clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOASMEN::Disabled)
    }
    ///IO port x clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOASMEN::Enabled)
    }
}
///Field `GPIOBSMEN` reader - IO port B clocks enable during Sleep and Stop modes
pub use GPIOASMEN_R as GPIOBSMEN_R;
///Field `GPIOCSMEN` reader - IO port C clocks enable during Sleep and Stop modes
pub use GPIOASMEN_R as GPIOCSMEN_R;
///Field `GPIODSMEN` reader - IO port D clocks enable during Sleep and Stop modes
pub use GPIOASMEN_R as GPIODSMEN_R;
///Field `GPIOESMEN` reader - IO port E clocks enable during Sleep and Stop modes
pub use GPIOASMEN_R as GPIOESMEN_R;
///Field `GPIOFSMEN` reader - IO port F clocks enable during Sleep and Stop modes
pub use GPIOASMEN_R as GPIOFSMEN_R;
///Field `GPIOGSMEN` reader - IO port G clocks enable during Sleep and Stop modes
pub use GPIOASMEN_R as GPIOGSMEN_R;
///Field `GPIOHSMEN` reader - IO port H clocks enable during Sleep and Stop modes
pub use GPIOASMEN_R as GPIOHSMEN_R;
///Field `GPIOISMEN` reader - IO port I clocks enable during Sleep and Stop modes
pub use GPIOASMEN_R as GPIOISMEN_R;
///Field `GPIOBSMEN` writer - IO port B clocks enable during Sleep and Stop modes
pub use GPIOASMEN_W as GPIOBSMEN_W;
///Field `GPIOCSMEN` writer - IO port C clocks enable during Sleep and Stop modes
pub use GPIOASMEN_W as GPIOCSMEN_W;
///Field `GPIODSMEN` writer - IO port D clocks enable during Sleep and Stop modes
pub use GPIOASMEN_W as GPIODSMEN_W;
///Field `GPIOESMEN` writer - IO port E clocks enable during Sleep and Stop modes
pub use GPIOASMEN_W as GPIOESMEN_W;
///Field `GPIOFSMEN` writer - IO port F clocks enable during Sleep and Stop modes
pub use GPIOASMEN_W as GPIOFSMEN_W;
///Field `GPIOGSMEN` writer - IO port G clocks enable during Sleep and Stop modes
pub use GPIOASMEN_W as GPIOGSMEN_W;
///Field `GPIOHSMEN` writer - IO port H clocks enable during Sleep and Stop modes
pub use GPIOASMEN_W as GPIOHSMEN_W;
///Field `GPIOISMEN` writer - IO port I clocks enable during Sleep and Stop modes
pub use GPIOASMEN_W as GPIOISMEN_W;
/**SRAM2 interface clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2SMEN {
    ///0: SRAMx clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: SRAMx clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<SRAM2SMEN> for bool {
    #[inline(always)]
    fn from(variant: SRAM2SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM2SMEN` reader - SRAM2 interface clocks enable during Sleep and Stop modes
pub type SRAM2SMEN_R = crate::BitReader<SRAM2SMEN>;
impl SRAM2SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRAM2SMEN {
        match self.bits {
            false => SRAM2SMEN::Disabled,
            true => SRAM2SMEN::Enabled,
        }
    }
    ///SRAMx clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRAM2SMEN::Disabled
    }
    ///SRAMx clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRAM2SMEN::Enabled
    }
}
///Field `SRAM2SMEN` writer - SRAM2 interface clocks enable during Sleep and Stop modes
pub type SRAM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG, SRAM2SMEN>;
impl<'a, REG> SRAM2SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SRAMx clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2SMEN::Disabled)
    }
    ///SRAMx clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2SMEN::Enabled)
    }
}
///Field `SRAM3SMEN` reader - SRAM2 interface clocks enable during Sleep and Stop modes
pub use SRAM2SMEN_R as SRAM3SMEN_R;
///Field `SRAM3SMEN` writer - SRAM2 interface clocks enable during Sleep and Stop modes
pub use SRAM2SMEN_W as SRAM3SMEN_W;
/**OTG full speed clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTGFSSMEN {
    ///0: USB OTG full speed clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: USB OTG full speed clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<OTGFSSMEN> for bool {
    #[inline(always)]
    fn from(variant: OTGFSSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `OTGFSSMEN` reader - OTG full speed clocks enable during Sleep and Stop modes
pub type OTGFSSMEN_R = crate::BitReader<OTGFSSMEN>;
impl OTGFSSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OTGFSSMEN {
        match self.bits {
            false => OTGFSSMEN::Disabled,
            true => OTGFSSMEN::Enabled,
        }
    }
    ///USB OTG full speed clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OTGFSSMEN::Disabled
    }
    ///USB OTG full speed clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OTGFSSMEN::Enabled
    }
}
///Field `OTGFSSMEN` writer - OTG full speed clocks enable during Sleep and Stop modes
pub type OTGFSSMEN_W<'a, REG> = crate::BitWriter<'a, REG, OTGFSSMEN>;
impl<'a, REG> OTGFSSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///USB OTG full speed clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OTGFSSMEN::Disabled)
    }
    ///USB OTG full speed clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OTGFSSMEN::Enabled)
    }
}
/**ADC clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCSMEN {
    ///0: ADC clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: ADC clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<ADCSMEN> for bool {
    #[inline(always)]
    fn from(variant: ADCSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ADCSMEN` reader - ADC clocks enable during Sleep and Stop modes
pub type ADCSMEN_R = crate::BitReader<ADCSMEN>;
impl ADCSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADCSMEN {
        match self.bits {
            false => ADCSMEN::Disabled,
            true => ADCSMEN::Enabled,
        }
    }
    ///ADC clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCSMEN::Disabled
    }
    ///ADC clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADCSMEN::Enabled
    }
}
///Field `ADCSMEN` writer - ADC clocks enable during Sleep and Stop modes
pub type ADCSMEN_W<'a, REG> = crate::BitWriter<'a, REG, ADCSMEN>;
impl<'a, REG> ADCSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSMEN::Disabled)
    }
    ///ADC clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSMEN::Enabled)
    }
}
/**DCMI clock enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMISMEN {
    ///0: DCMI/PSSI clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: DCMI/PSSI clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<DCMISMEN> for bool {
    #[inline(always)]
    fn from(variant: DCMISMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DCMISMEN` reader - DCMI clock enable during Sleep and Stop modes
pub type DCMISMEN_R = crate::BitReader<DCMISMEN>;
impl DCMISMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DCMISMEN {
        match self.bits {
            false => DCMISMEN::Disabled,
            true => DCMISMEN::Enabled,
        }
    }
    ///DCMI/PSSI clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCMISMEN::Disabled
    }
    ///DCMI/PSSI clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCMISMEN::Enabled
    }
}
///Field `DCMISMEN` writer - DCMI clock enable during Sleep and Stop modes
pub type DCMISMEN_W<'a, REG> = crate::BitWriter<'a, REG, DCMISMEN>;
impl<'a, REG> DCMISMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DCMI/PSSI clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMISMEN::Disabled)
    }
    ///DCMI/PSSI clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMISMEN::Enabled)
    }
}
/**PKA clocks enable during Sleep and Stop modes

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PKASMEN {
    ///0: PKA clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: PKA clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<PKASMEN> for bool {
    #[inline(always)]
    fn from(variant: PKASMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PKASMEN` reader - PKA clocks enable during Sleep and Stop modes
pub type PKASMEN_R = crate::BitReader<PKASMEN>;
impl PKASMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PKASMEN {
        match self.bits {
            false => PKASMEN::Disabled,
            true => PKASMEN::Enabled,
        }
    }
    ///PKA clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PKASMEN::Disabled
    }
    ///PKA clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PKASMEN::Enabled
    }
}
///Field `PKASMEN` writer - PKA clocks enable during Sleep and Stop modes
pub type PKASMEN_W<'a, REG> = crate::BitWriter<'a, REG, PKASMEN>;
impl<'a, REG> PKASMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PKA clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PKASMEN::Disabled)
    }
    ///PKA clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PKASMEN::Enabled)
    }
}
/**AES accelerator clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESSMEN {
    ///0: AES clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: AES clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<AESSMEN> for bool {
    #[inline(always)]
    fn from(variant: AESSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `AESSMEN` reader - AES accelerator clocks enable during Sleep and Stop modes
pub type AESSMEN_R = crate::BitReader<AESSMEN>;
impl AESSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AESSMEN {
        match self.bits {
            false => AESSMEN::Disabled,
            true => AESSMEN::Enabled,
        }
    }
    ///AES clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AESSMEN::Disabled
    }
    ///AES clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AESSMEN::Enabled
    }
}
///Field `AESSMEN` writer - AES accelerator clocks enable during Sleep and Stop modes
pub type AESSMEN_W<'a, REG> = crate::BitWriter<'a, REG, AESSMEN>;
impl<'a, REG> AESSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AES clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AESSMEN::Disabled)
    }
    ///AES clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AESSMEN::Enabled)
    }
}
/**HASH clock enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HASHSMEN {
    ///0: HASH clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: HASH clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<HASHSMEN> for bool {
    #[inline(always)]
    fn from(variant: HASHSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `HASHSMEN` reader - HASH clock enable during Sleep and Stop modes
pub type HASHSMEN_R = crate::BitReader<HASHSMEN>;
impl HASHSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HASHSMEN {
        match self.bits {
            false => HASHSMEN::Disabled,
            true => HASHSMEN::Enabled,
        }
    }
    ///HASH clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HASHSMEN::Disabled
    }
    ///HASH clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HASHSMEN::Enabled
    }
}
///Field `HASHSMEN` writer - HASH clock enable during Sleep and Stop modes
pub type HASHSMEN_W<'a, REG> = crate::BitWriter<'a, REG, HASHSMEN>;
impl<'a, REG> HASHSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HASH clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HASHSMEN::Disabled)
    }
    ///HASH clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HASHSMEN::Enabled)
    }
}
/**Random Number Generator clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGSMEN {
    ///0: Random Number Generator clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: Random Number Generator clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<RNGSMEN> for bool {
    #[inline(always)]
    fn from(variant: RNGSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `RNGSMEN` reader - Random Number Generator clocks enable during Sleep and Stop modes
pub type RNGSMEN_R = crate::BitReader<RNGSMEN>;
impl RNGSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RNGSMEN {
        match self.bits {
            false => RNGSMEN::Disabled,
            true => RNGSMEN::Enabled,
        }
    }
    ///Random Number Generator clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RNGSMEN::Disabled
    }
    ///Random Number Generator clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RNGSMEN::Enabled
    }
}
///Field `RNGSMEN` writer - Random Number Generator clocks enable during Sleep and Stop modes
pub type RNGSMEN_W<'a, REG> = crate::BitWriter<'a, REG, RNGSMEN>;
impl<'a, REG> RNGSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Random Number Generator clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSMEN::Disabled)
    }
    ///Random Number Generator clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSMEN::Enabled)
    }
}
/**OctoSPI IO manager clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSPIMSMEN {
    ///0: OCTOSPIM clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: OCTOSPIM clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<OSPIMSMEN> for bool {
    #[inline(always)]
    fn from(variant: OSPIMSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `OSPIMSMEN` reader - OctoSPI IO manager clocks enable during Sleep and Stop modes
pub type OSPIMSMEN_R = crate::BitReader<OSPIMSMEN>;
impl OSPIMSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSPIMSMEN {
        match self.bits {
            false => OSPIMSMEN::Disabled,
            true => OSPIMSMEN::Enabled,
        }
    }
    ///OCTOSPIM clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSPIMSMEN::Disabled
    }
    ///OCTOSPIM clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSPIMSMEN::Enabled
    }
}
///Field `OSPIMSMEN` writer - OctoSPI IO manager clocks enable during Sleep and Stop modes
pub type OSPIMSMEN_W<'a, REG> = crate::BitWriter<'a, REG, OSPIMSMEN>;
impl<'a, REG> OSPIMSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OCTOSPIM clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSPIMSMEN::Disabled)
    }
    ///OCTOSPIM clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSPIMSMEN::Enabled)
    }
}
/**SDMMC1 clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMC1SMEN {
    ///0: SDMMCx clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: SDMMCx clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<SDMMC1SMEN> for bool {
    #[inline(always)]
    fn from(variant: SDMMC1SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SDMMC1SMEN` reader - SDMMC1 clocks enable during Sleep and Stop modes
pub type SDMMC1SMEN_R = crate::BitReader<SDMMC1SMEN>;
impl SDMMC1SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDMMC1SMEN {
        match self.bits {
            false => SDMMC1SMEN::Disabled,
            true => SDMMC1SMEN::Enabled,
        }
    }
    ///SDMMCx clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDMMC1SMEN::Disabled
    }
    ///SDMMCx clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDMMC1SMEN::Enabled
    }
}
///Field `SDMMC1SMEN` writer - SDMMC1 clocks enable during Sleep and Stop modes
pub type SDMMC1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, SDMMC1SMEN>;
impl<'a, REG> SDMMC1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SDMMCx clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1SMEN::Disabled)
    }
    ///SDMMCx clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1SMEN::Enabled)
    }
}
///Field `SDMMC2SMEN` reader - SDMMC2 clocks enable during Sleep and Stop modes
pub use SDMMC1SMEN_R as SDMMC2SMEN_R;
///Field `SDMMC2SMEN` writer - SDMMC2 clocks enable during Sleep and Stop modes
pub use SDMMC1SMEN_W as SDMMC2SMEN_W;
impl R {
    ///Bit 0 - IO port A clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port D clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GPIODSMEN_R {
        GPIODSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port E clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioesmen(&self) -> GPIOESMEN_R {
        GPIOESMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IO port F clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiofsmen(&self) -> GPIOFSMEN_R {
        GPIOFSMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port G clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiogsmen(&self) -> GPIOGSMEN_R {
        GPIOGSMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IO port H clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiohsmen(&self) -> GPIOHSMEN_R {
        GPIOHSMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - IO port I clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioismen(&self) -> GPIOISMEN_R {
        GPIOISMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SRAM2 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sram2smen(&self) -> SRAM2SMEN_R {
        SRAM2SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SRAM2 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sram3smen(&self) -> SRAM3SMEN_R {
        SRAM3SMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - OTG full speed clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn otgfssmen(&self) -> OTGFSSMEN_R {
        OTGFSSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ADC clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn adcsmen(&self) -> ADCSMEN_R {
        ADCSMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - DCMI clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dcmismen(&self) -> DCMISMEN_R {
        DCMISMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - PKA clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn pkasmen(&self) -> PKASMEN_R {
        PKASMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - AES accelerator clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn aessmen(&self) -> AESSMEN_R {
        AESSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HASH clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn hashsmen(&self) -> HASHSMEN_R {
        HASHSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Random Number Generator clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn rngsmen(&self) -> RNGSMEN_R {
        RNGSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - OctoSPI IO manager clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn ospimsmen(&self) -> OSPIMSMEN_R {
        OSPIMSMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - SDMMC1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sdmmc1smen(&self) -> SDMMC1SMEN_R {
        SDMMC1SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - SDMMC2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sdmmc2smen(&self) -> SDMMC2SMEN_R {
        SDMMC2SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2SMENR")
            .field("gpioasmen", &self.gpioasmen())
            .field("gpiobsmen", &self.gpiobsmen())
            .field("gpiocsmen", &self.gpiocsmen())
            .field("gpiodsmen", &self.gpiodsmen())
            .field("gpioesmen", &self.gpioesmen())
            .field("gpiofsmen", &self.gpiofsmen())
            .field("gpiogsmen", &self.gpiogsmen())
            .field("gpiohsmen", &self.gpiohsmen())
            .field("gpioismen", &self.gpioismen())
            .field("sram2smen", &self.sram2smen())
            .field("sram3smen", &self.sram3smen())
            .field("otgfssmen", &self.otgfssmen())
            .field("adcsmen", &self.adcsmen())
            .field("dcmismen", &self.dcmismen())
            .field("aessmen", &self.aessmen())
            .field("hashsmen", &self.hashsmen())
            .field("rngsmen", &self.rngsmen())
            .field("ospimsmen", &self.ospimsmen())
            .field("sdmmc1smen", &self.sdmmc1smen())
            .field("sdmmc2smen", &self.sdmmc2smen())
            .field("pkasmen", &self.pkasmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - IO port A clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<'_, AHB2SMENRrs> {
        GPIOASMEN_W::new(self, 0)
    }
    ///Bit 1 - IO port B clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<'_, AHB2SMENRrs> {
        GPIOBSMEN_W::new(self, 1)
    }
    ///Bit 2 - IO port C clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<'_, AHB2SMENRrs> {
        GPIOCSMEN_W::new(self, 2)
    }
    ///Bit 3 - IO port D clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiodsmen(&mut self) -> GPIODSMEN_W<'_, AHB2SMENRrs> {
        GPIODSMEN_W::new(self, 3)
    }
    ///Bit 4 - IO port E clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioesmen(&mut self) -> GPIOESMEN_W<'_, AHB2SMENRrs> {
        GPIOESMEN_W::new(self, 4)
    }
    ///Bit 5 - IO port F clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiofsmen(&mut self) -> GPIOFSMEN_W<'_, AHB2SMENRrs> {
        GPIOFSMEN_W::new(self, 5)
    }
    ///Bit 6 - IO port G clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiogsmen(&mut self) -> GPIOGSMEN_W<'_, AHB2SMENRrs> {
        GPIOGSMEN_W::new(self, 6)
    }
    ///Bit 7 - IO port H clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiohsmen(&mut self) -> GPIOHSMEN_W<'_, AHB2SMENRrs> {
        GPIOHSMEN_W::new(self, 7)
    }
    ///Bit 8 - IO port I clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioismen(&mut self) -> GPIOISMEN_W<'_, AHB2SMENRrs> {
        GPIOISMEN_W::new(self, 8)
    }
    ///Bit 9 - SRAM2 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sram2smen(&mut self) -> SRAM2SMEN_W<'_, AHB2SMENRrs> {
        SRAM2SMEN_W::new(self, 9)
    }
    ///Bit 10 - SRAM2 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sram3smen(&mut self) -> SRAM3SMEN_W<'_, AHB2SMENRrs> {
        SRAM3SMEN_W::new(self, 10)
    }
    ///Bit 12 - OTG full speed clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn otgfssmen(&mut self) -> OTGFSSMEN_W<'_, AHB2SMENRrs> {
        OTGFSSMEN_W::new(self, 12)
    }
    ///Bit 13 - ADC clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn adcsmen(&mut self) -> ADCSMEN_W<'_, AHB2SMENRrs> {
        ADCSMEN_W::new(self, 13)
    }
    ///Bit 14 - DCMI clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dcmismen(&mut self) -> DCMISMEN_W<'_, AHB2SMENRrs> {
        DCMISMEN_W::new(self, 14)
    }
    ///Bit 15 - PKA clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn pkasmen(&mut self) -> PKASMEN_W<'_, AHB2SMENRrs> {
        PKASMEN_W::new(self, 15)
    }
    ///Bit 16 - AES accelerator clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn aessmen(&mut self) -> AESSMEN_W<'_, AHB2SMENRrs> {
        AESSMEN_W::new(self, 16)
    }
    ///Bit 17 - HASH clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn hashsmen(&mut self) -> HASHSMEN_W<'_, AHB2SMENRrs> {
        HASHSMEN_W::new(self, 17)
    }
    ///Bit 18 - Random Number Generator clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn rngsmen(&mut self) -> RNGSMEN_W<'_, AHB2SMENRrs> {
        RNGSMEN_W::new(self, 18)
    }
    ///Bit 20 - OctoSPI IO manager clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn ospimsmen(&mut self) -> OSPIMSMEN_W<'_, AHB2SMENRrs> {
        OSPIMSMEN_W::new(self, 20)
    }
    ///Bit 22 - SDMMC1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sdmmc1smen(&mut self) -> SDMMC1SMEN_W<'_, AHB2SMENRrs> {
        SDMMC1SMEN_W::new(self, 22)
    }
    ///Bit 23 - SDMMC2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sdmmc2smen(&mut self) -> SDMMC2SMEN_W<'_, AHB2SMENRrs> {
        SDMMC2SMEN_W::new(self, 23)
    }
}
/**AHB2 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`ahb2smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#RCC:AHB2SMENR)*/
pub struct AHB2SMENRrs;
impl crate::RegisterSpec for AHB2SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2smenr::R`](R) reader structure
impl crate::Readable for AHB2SMENRrs {}
///`write(|w| ..)` method takes [`ahb2smenr::W`](W) writer structure
impl crate::Writable for AHB2SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2SMENR to value 0x0057_77ff
impl crate::Resettable for AHB2SMENRrs {
    const RESET_VALUE: u32 = 0x0057_77ff;
}
