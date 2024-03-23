#[doc = "Register `AHB2SMENR` reader"]
pub type R = crate::R<AHB2SMENRrs>;
#[doc = "Register `AHB2SMENR` writer"]
pub type W = crate::W<AHB2SMENRrs>;
#[doc = "IO port A clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOASMEN {
    #[doc = "0: IO port x clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: IO port x clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<GPIOASMEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOASMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOASMEN` reader - IO port A clocks enable during Sleep and Stop modes"]
pub type GPIOASMEN_R = crate::BitReader<GPIOASMEN>;
impl GPIOASMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIOASMEN {
        match self.bits {
            false => GPIOASMEN::Disabled,
            true => GPIOASMEN::Enabled,
        }
    }
    #[doc = "IO port x clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOASMEN::Disabled
    }
    #[doc = "IO port x clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOASMEN::Enabled
    }
}
#[doc = "Field `GPIOASMEN` writer - IO port A clocks enable during Sleep and Stop modes"]
pub type GPIOASMEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOASMEN>;
impl<'a, REG> GPIOASMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IO port x clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOASMEN::Disabled)
    }
    #[doc = "IO port x clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOASMEN::Enabled)
    }
}
#[doc = "Field `GPIOBSMEN` reader - IO port B clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_R as GPIOBSMEN_R;
#[doc = "Field `GPIOCSMEN` reader - IO port C clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_R as GPIOCSMEN_R;
#[doc = "Field `GPIODSMEN` reader - IO port D clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_R as GPIODSMEN_R;
#[doc = "Field `GPIOESMEN` reader - IO port E clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_R as GPIOESMEN_R;
#[doc = "Field `GPIOFSMEN` reader - IO port F clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_R as GPIOFSMEN_R;
#[doc = "Field `GPIOGSMEN` reader - IO port G clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_R as GPIOGSMEN_R;
#[doc = "Field `GPIOHSMEN` reader - IO port H clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_R as GPIOHSMEN_R;
#[doc = "Field `GPIOISMEN` reader - IO port I clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_R as GPIOISMEN_R;
#[doc = "Field `GPIOBSMEN` writer - IO port B clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_W as GPIOBSMEN_W;
#[doc = "Field `GPIOCSMEN` writer - IO port C clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_W as GPIOCSMEN_W;
#[doc = "Field `GPIODSMEN` writer - IO port D clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_W as GPIODSMEN_W;
#[doc = "Field `GPIOESMEN` writer - IO port E clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_W as GPIOESMEN_W;
#[doc = "Field `GPIOFSMEN` writer - IO port F clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_W as GPIOFSMEN_W;
#[doc = "Field `GPIOGSMEN` writer - IO port G clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_W as GPIOGSMEN_W;
#[doc = "Field `GPIOHSMEN` writer - IO port H clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_W as GPIOHSMEN_W;
#[doc = "Field `GPIOISMEN` writer - IO port I clocks enable during Sleep and Stop modes"]
pub use GPIOASMEN_W as GPIOISMEN_W;
#[doc = "SRAM2 interface clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2SMEN {
    #[doc = "0: SRAMx clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: SRAMx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<SRAM2SMEN> for bool {
    #[inline(always)]
    fn from(variant: SRAM2SMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM2SMEN` reader - SRAM2 interface clocks enable during Sleep and Stop modes"]
pub type SRAM2SMEN_R = crate::BitReader<SRAM2SMEN>;
impl SRAM2SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAM2SMEN {
        match self.bits {
            false => SRAM2SMEN::Disabled,
            true => SRAM2SMEN::Enabled,
        }
    }
    #[doc = "SRAMx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRAM2SMEN::Disabled
    }
    #[doc = "SRAMx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRAM2SMEN::Enabled
    }
}
#[doc = "Field `SRAM2SMEN` writer - SRAM2 interface clocks enable during Sleep and Stop modes"]
pub type SRAM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG, SRAM2SMEN>;
impl<'a, REG> SRAM2SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAMx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2SMEN::Disabled)
    }
    #[doc = "SRAMx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2SMEN::Enabled)
    }
}
#[doc = "Field `SRAM3SMEN` reader - SRAM2 interface clocks enable during Sleep and Stop modes"]
pub use SRAM2SMEN_R as SRAM3SMEN_R;
#[doc = "Field `SRAM3SMEN` writer - SRAM2 interface clocks enable during Sleep and Stop modes"]
pub use SRAM2SMEN_W as SRAM3SMEN_W;
#[doc = "OTG full speed clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTGFSSMEN {
    #[doc = "0: USB OTG full speed clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: USB OTG full speed clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<OTGFSSMEN> for bool {
    #[inline(always)]
    fn from(variant: OTGFSSMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTGFSSMEN` reader - OTG full speed clocks enable during Sleep and Stop modes"]
pub type OTGFSSMEN_R = crate::BitReader<OTGFSSMEN>;
impl OTGFSSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OTGFSSMEN {
        match self.bits {
            false => OTGFSSMEN::Disabled,
            true => OTGFSSMEN::Enabled,
        }
    }
    #[doc = "USB OTG full speed clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OTGFSSMEN::Disabled
    }
    #[doc = "USB OTG full speed clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OTGFSSMEN::Enabled
    }
}
#[doc = "Field `OTGFSSMEN` writer - OTG full speed clocks enable during Sleep and Stop modes"]
pub type OTGFSSMEN_W<'a, REG> = crate::BitWriter<'a, REG, OTGFSSMEN>;
impl<'a, REG> OTGFSSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB OTG full speed clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OTGFSSMEN::Disabled)
    }
    #[doc = "USB OTG full speed clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OTGFSSMEN::Enabled)
    }
}
#[doc = "ADC clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCSMEN {
    #[doc = "0: ADC clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: ADC clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<ADCSMEN> for bool {
    #[inline(always)]
    fn from(variant: ADCSMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCSMEN` reader - ADC clocks enable during Sleep and Stop modes"]
pub type ADCSMEN_R = crate::BitReader<ADCSMEN>;
impl ADCSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADCSMEN {
        match self.bits {
            false => ADCSMEN::Disabled,
            true => ADCSMEN::Enabled,
        }
    }
    #[doc = "ADC clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCSMEN::Disabled
    }
    #[doc = "ADC clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADCSMEN::Enabled
    }
}
#[doc = "Field `ADCSMEN` writer - ADC clocks enable during Sleep and Stop modes"]
pub type ADCSMEN_W<'a, REG> = crate::BitWriter<'a, REG, ADCSMEN>;
impl<'a, REG> ADCSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSMEN::Disabled)
    }
    #[doc = "ADC clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSMEN::Enabled)
    }
}
#[doc = "DCMI clock enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMISMEN {
    #[doc = "0: DCMI/PSSI clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: DCMI/PSSI clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<DCMISMEN> for bool {
    #[inline(always)]
    fn from(variant: DCMISMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMISMEN` reader - DCMI clock enable during Sleep and Stop modes"]
pub type DCMISMEN_R = crate::BitReader<DCMISMEN>;
impl DCMISMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCMISMEN {
        match self.bits {
            false => DCMISMEN::Disabled,
            true => DCMISMEN::Enabled,
        }
    }
    #[doc = "DCMI/PSSI clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCMISMEN::Disabled
    }
    #[doc = "DCMI/PSSI clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCMISMEN::Enabled
    }
}
#[doc = "Field `DCMISMEN` writer - DCMI clock enable during Sleep and Stop modes"]
pub type DCMISMEN_W<'a, REG> = crate::BitWriter<'a, REG, DCMISMEN>;
impl<'a, REG> DCMISMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DCMI/PSSI clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMISMEN::Disabled)
    }
    #[doc = "DCMI/PSSI clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMISMEN::Enabled)
    }
}
#[doc = "PKA clocks enable during Sleep and Stop modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PKASMEN {
    #[doc = "0: PKA clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: PKA clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<PKASMEN> for bool {
    #[inline(always)]
    fn from(variant: PKASMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PKASMEN` reader - PKA clocks enable during Sleep and Stop modes"]
pub type PKASMEN_R = crate::BitReader<PKASMEN>;
impl PKASMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PKASMEN {
        match self.bits {
            false => PKASMEN::Disabled,
            true => PKASMEN::Enabled,
        }
    }
    #[doc = "PKA clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PKASMEN::Disabled
    }
    #[doc = "PKA clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PKASMEN::Enabled
    }
}
#[doc = "Field `PKASMEN` writer - PKA clocks enable during Sleep and Stop modes"]
pub type PKASMEN_W<'a, REG> = crate::BitWriter<'a, REG, PKASMEN>;
impl<'a, REG> PKASMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PKA clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PKASMEN::Disabled)
    }
    #[doc = "PKA clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PKASMEN::Enabled)
    }
}
#[doc = "AES accelerator clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESSMEN {
    #[doc = "0: AES clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: AES clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<AESSMEN> for bool {
    #[inline(always)]
    fn from(variant: AESSMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESSMEN` reader - AES accelerator clocks enable during Sleep and Stop modes"]
pub type AESSMEN_R = crate::BitReader<AESSMEN>;
impl AESSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AESSMEN {
        match self.bits {
            false => AESSMEN::Disabled,
            true => AESSMEN::Enabled,
        }
    }
    #[doc = "AES clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AESSMEN::Disabled
    }
    #[doc = "AES clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AESSMEN::Enabled
    }
}
#[doc = "Field `AESSMEN` writer - AES accelerator clocks enable during Sleep and Stop modes"]
pub type AESSMEN_W<'a, REG> = crate::BitWriter<'a, REG, AESSMEN>;
impl<'a, REG> AESSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AES clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AESSMEN::Disabled)
    }
    #[doc = "AES clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AESSMEN::Enabled)
    }
}
#[doc = "HASH clock enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HASHSMEN {
    #[doc = "0: HASH clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: HASH clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<HASHSMEN> for bool {
    #[inline(always)]
    fn from(variant: HASHSMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASHSMEN` reader - HASH clock enable during Sleep and Stop modes"]
pub type HASHSMEN_R = crate::BitReader<HASHSMEN>;
impl HASHSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HASHSMEN {
        match self.bits {
            false => HASHSMEN::Disabled,
            true => HASHSMEN::Enabled,
        }
    }
    #[doc = "HASH clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HASHSMEN::Disabled
    }
    #[doc = "HASH clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HASHSMEN::Enabled
    }
}
#[doc = "Field `HASHSMEN` writer - HASH clock enable during Sleep and Stop modes"]
pub type HASHSMEN_W<'a, REG> = crate::BitWriter<'a, REG, HASHSMEN>;
impl<'a, REG> HASHSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HASH clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HASHSMEN::Disabled)
    }
    #[doc = "HASH clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HASHSMEN::Enabled)
    }
}
#[doc = "Random Number Generator clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGSMEN {
    #[doc = "0: Random Number Generator clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: Random Number Generator clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<RNGSMEN> for bool {
    #[inline(always)]
    fn from(variant: RNGSMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNGSMEN` reader - Random Number Generator clocks enable during Sleep and Stop modes"]
pub type RNGSMEN_R = crate::BitReader<RNGSMEN>;
impl RNGSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RNGSMEN {
        match self.bits {
            false => RNGSMEN::Disabled,
            true => RNGSMEN::Enabled,
        }
    }
    #[doc = "Random Number Generator clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RNGSMEN::Disabled
    }
    #[doc = "Random Number Generator clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RNGSMEN::Enabled
    }
}
#[doc = "Field `RNGSMEN` writer - Random Number Generator clocks enable during Sleep and Stop modes"]
pub type RNGSMEN_W<'a, REG> = crate::BitWriter<'a, REG, RNGSMEN>;
impl<'a, REG> RNGSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Random Number Generator clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSMEN::Disabled)
    }
    #[doc = "Random Number Generator clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSMEN::Enabled)
    }
}
#[doc = "OctoSPI IO manager clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSPIMSMEN {
    #[doc = "0: OCTOSPIM clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: OCTOSPIM clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<OSPIMSMEN> for bool {
    #[inline(always)]
    fn from(variant: OSPIMSMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSPIMSMEN` reader - OctoSPI IO manager clocks enable during Sleep and Stop modes"]
pub type OSPIMSMEN_R = crate::BitReader<OSPIMSMEN>;
impl OSPIMSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSPIMSMEN {
        match self.bits {
            false => OSPIMSMEN::Disabled,
            true => OSPIMSMEN::Enabled,
        }
    }
    #[doc = "OCTOSPIM clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSPIMSMEN::Disabled
    }
    #[doc = "OCTOSPIM clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSPIMSMEN::Enabled
    }
}
#[doc = "Field `OSPIMSMEN` writer - OctoSPI IO manager clocks enable during Sleep and Stop modes"]
pub type OSPIMSMEN_W<'a, REG> = crate::BitWriter<'a, REG, OSPIMSMEN>;
impl<'a, REG> OSPIMSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OCTOSPIM clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSPIMSMEN::Disabled)
    }
    #[doc = "OCTOSPIM clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSPIMSMEN::Enabled)
    }
}
#[doc = "SDMMC1 clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMC1SMEN {
    #[doc = "0: SDMMCx clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: SDMMCx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<SDMMC1SMEN> for bool {
    #[inline(always)]
    fn from(variant: SDMMC1SMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMMC1SMEN` reader - SDMMC1 clocks enable during Sleep and Stop modes"]
pub type SDMMC1SMEN_R = crate::BitReader<SDMMC1SMEN>;
impl SDMMC1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDMMC1SMEN {
        match self.bits {
            false => SDMMC1SMEN::Disabled,
            true => SDMMC1SMEN::Enabled,
        }
    }
    #[doc = "SDMMCx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDMMC1SMEN::Disabled
    }
    #[doc = "SDMMCx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDMMC1SMEN::Enabled
    }
}
#[doc = "Field `SDMMC1SMEN` writer - SDMMC1 clocks enable during Sleep and Stop modes"]
pub type SDMMC1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, SDMMC1SMEN>;
impl<'a, REG> SDMMC1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDMMCx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1SMEN::Disabled)
    }
    #[doc = "SDMMCx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1SMEN::Enabled)
    }
}
#[doc = "Field `SDMMC2SMEN` reader - SDMMC2 clocks enable during Sleep and Stop modes"]
pub use SDMMC1SMEN_R as SDMMC2SMEN_R;
#[doc = "Field `SDMMC2SMEN` writer - SDMMC2 clocks enable during Sleep and Stop modes"]
pub use SDMMC1SMEN_W as SDMMC2SMEN_W;
impl R {
    #[doc = "Bit 0 - IO port A clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GPIODSMEN_R {
        GPIODSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioesmen(&self) -> GPIOESMEN_R {
        GPIOESMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiofsmen(&self) -> GPIOFSMEN_R {
        GPIOFSMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiogsmen(&self) -> GPIOGSMEN_R {
        GPIOGSMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiohsmen(&self) -> GPIOHSMEN_R {
        GPIOHSMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IO port I clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioismen(&self) -> GPIOISMEN_R {
        GPIOISMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM2 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sram2smen(&self) -> SRAM2SMEN_R {
        SRAM2SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SRAM2 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sram3smen(&self) -> SRAM3SMEN_R {
        SRAM3SMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - OTG full speed clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn otgfssmen(&self) -> OTGFSSMEN_R {
        OTGFSSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn adcsmen(&self) -> ADCSMEN_R {
        ADCSMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DCMI clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dcmismen(&self) -> DCMISMEN_R {
        DCMISMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PKA clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn pkasmen(&self) -> PKASMEN_R {
        PKASMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - AES accelerator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn aessmen(&self) -> AESSMEN_R {
        AESSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HASH clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn hashsmen(&self) -> HASHSMEN_R {
        HASHSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Random Number Generator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn rngsmen(&self) -> RNGSMEN_R {
        RNGSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - OctoSPI IO manager clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn ospimsmen(&self) -> OSPIMSMEN_R {
        OSPIMSMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - SDMMC1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sdmmc1smen(&self) -> SDMMC1SMEN_R {
        SDMMC1SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SDMMC2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sdmmc2smen(&self) -> SDMMC2SMEN_R {
        SDMMC2SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<AHB2SMENRrs> {
        GPIOASMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<AHB2SMENRrs> {
        GPIOBSMEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<AHB2SMENRrs> {
        GPIOCSMEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - IO port D clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodsmen(&mut self) -> GPIODSMEN_W<AHB2SMENRrs> {
        GPIODSMEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpioesmen(&mut self) -> GPIOESMEN_W<AHB2SMENRrs> {
        GPIOESMEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - IO port F clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofsmen(&mut self) -> GPIOFSMEN_W<AHB2SMENRrs> {
        GPIOFSMEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - IO port G clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiogsmen(&mut self) -> GPIOGSMEN_W<AHB2SMENRrs> {
        GPIOGSMEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - IO port H clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohsmen(&mut self) -> GPIOHSMEN_W<AHB2SMENRrs> {
        GPIOHSMEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - IO port I clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gpioismen(&mut self) -> GPIOISMEN_W<AHB2SMENRrs> {
        GPIOISMEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - SRAM2 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sram2smen(&mut self) -> SRAM2SMEN_W<AHB2SMENRrs> {
        SRAM2SMEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - SRAM2 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sram3smen(&mut self) -> SRAM3SMEN_W<AHB2SMENRrs> {
        SRAM3SMEN_W::new(self, 10)
    }
    #[doc = "Bit 12 - OTG full speed clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn otgfssmen(&mut self) -> OTGFSSMEN_W<AHB2SMENRrs> {
        OTGFSSMEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - ADC clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn adcsmen(&mut self) -> ADCSMEN_W<AHB2SMENRrs> {
        ADCSMEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - DCMI clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dcmismen(&mut self) -> DCMISMEN_W<AHB2SMENRrs> {
        DCMISMEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - PKA clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn pkasmen(&mut self) -> PKASMEN_W<AHB2SMENRrs> {
        PKASMEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - AES accelerator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn aessmen(&mut self) -> AESSMEN_W<AHB2SMENRrs> {
        AESSMEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - HASH clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn hashsmen(&mut self) -> HASHSMEN_W<AHB2SMENRrs> {
        HASHSMEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Random Number Generator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn rngsmen(&mut self) -> RNGSMEN_W<AHB2SMENRrs> {
        RNGSMEN_W::new(self, 18)
    }
    #[doc = "Bit 20 - OctoSPI IO manager clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn ospimsmen(&mut self) -> OSPIMSMEN_W<AHB2SMENRrs> {
        OSPIMSMEN_W::new(self, 20)
    }
    #[doc = "Bit 22 - SDMMC1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1smen(&mut self) -> SDMMC1SMEN_W<AHB2SMENRrs> {
        SDMMC1SMEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - SDMMC2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2smen(&mut self) -> SDMMC2SMEN_W<AHB2SMENRrs> {
        SDMMC2SMEN_W::new(self, 23)
    }
}
#[doc = "AHB2 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2smenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2smenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2SMENRrs;
impl crate::RegisterSpec for AHB2SMENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2smenr::R`](R) reader structure"]
impl crate::Readable for AHB2SMENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb2smenr::W`](W) writer structure"]
impl crate::Writable for AHB2SMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB2SMENR to value 0x0057_77ff"]
impl crate::Resettable for AHB2SMENRrs {
    const RESET_VALUE: u32 = 0x0057_77ff;
}
