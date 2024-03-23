#[doc = "Register `AHB2ENR` reader"]
pub type R = crate::R<AHB2ENRrs>;
#[doc = "Register `AHB2ENR` writer"]
pub type W = crate::W<AHB2ENRrs>;
#[doc = "IO port A clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOAEN {
    #[doc = "0: IO port x clock disabled"]
    Disabled = 0,
    #[doc = "1: IO port x clock enabled"]
    Enabled = 1,
}
impl From<GPIOAEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOAEN` reader - IO port A clock enable"]
pub type GPIOAEN_R = crate::BitReader<GPIOAEN>;
impl GPIOAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIOAEN {
        match self.bits {
            false => GPIOAEN::Disabled,
            true => GPIOAEN::Enabled,
        }
    }
    #[doc = "IO port x clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOAEN::Disabled
    }
    #[doc = "IO port x clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOAEN::Enabled
    }
}
#[doc = "Field `GPIOAEN` writer - IO port A clock enable"]
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOAEN>;
impl<'a, REG> GPIOAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IO port x clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Disabled)
    }
    #[doc = "IO port x clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Enabled)
    }
}
#[doc = "Field `GPIOBEN` reader - IO port B clock enable"]
pub use GPIOAEN_R as GPIOBEN_R;
#[doc = "Field `GPIOCEN` reader - IO port C clock enable"]
pub use GPIOAEN_R as GPIOCEN_R;
#[doc = "Field `GPIODEN` reader - IO port D clock enable"]
pub use GPIOAEN_R as GPIODEN_R;
#[doc = "Field `GPIOEEN` reader - IO port E clock enable"]
pub use GPIOAEN_R as GPIOEEN_R;
#[doc = "Field `GPIOFEN` reader - IO port F clock enable"]
pub use GPIOAEN_R as GPIOFEN_R;
#[doc = "Field `GPIOGEN` reader - IO port G clock enable"]
pub use GPIOAEN_R as GPIOGEN_R;
#[doc = "Field `GPIOHEN` reader - IO port H clock enable"]
pub use GPIOAEN_R as GPIOHEN_R;
#[doc = "Field `GPIOIEN` reader - IO port I clock enable"]
pub use GPIOAEN_R as GPIOIEN_R;
#[doc = "Field `GPIOBEN` writer - IO port B clock enable"]
pub use GPIOAEN_W as GPIOBEN_W;
#[doc = "Field `GPIOCEN` writer - IO port C clock enable"]
pub use GPIOAEN_W as GPIOCEN_W;
#[doc = "Field `GPIODEN` writer - IO port D clock enable"]
pub use GPIOAEN_W as GPIODEN_W;
#[doc = "Field `GPIOEEN` writer - IO port E clock enable"]
pub use GPIOAEN_W as GPIOEEN_W;
#[doc = "Field `GPIOFEN` writer - IO port F clock enable"]
pub use GPIOAEN_W as GPIOFEN_W;
#[doc = "Field `GPIOGEN` writer - IO port G clock enable"]
pub use GPIOAEN_W as GPIOGEN_W;
#[doc = "Field `GPIOHEN` writer - IO port H clock enable"]
pub use GPIOAEN_W as GPIOHEN_W;
#[doc = "Field `GPIOIEN` writer - IO port I clock enable"]
pub use GPIOAEN_W as GPIOIEN_W;
#[doc = "OTG full speed clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTGFSEN {
    #[doc = "0: USB OTG full speed clock disabled"]
    Disabled = 0,
    #[doc = "1: USB OTG full speed clock enabled"]
    Enabled = 1,
}
impl From<OTGFSEN> for bool {
    #[inline(always)]
    fn from(variant: OTGFSEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTGFSEN` reader - OTG full speed clock enable"]
pub type OTGFSEN_R = crate::BitReader<OTGFSEN>;
impl OTGFSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OTGFSEN {
        match self.bits {
            false => OTGFSEN::Disabled,
            true => OTGFSEN::Enabled,
        }
    }
    #[doc = "USB OTG full speed clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OTGFSEN::Disabled
    }
    #[doc = "USB OTG full speed clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OTGFSEN::Enabled
    }
}
#[doc = "Field `OTGFSEN` writer - OTG full speed clock enable"]
pub type OTGFSEN_W<'a, REG> = crate::BitWriter<'a, REG, OTGFSEN>;
impl<'a, REG> OTGFSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB OTG full speed clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OTGFSEN::Disabled)
    }
    #[doc = "USB OTG full speed clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OTGFSEN::Enabled)
    }
}
#[doc = "ADC clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCEN {
    #[doc = "0: ADC clock disabled"]
    Disabled = 0,
    #[doc = "1: ADC clock enabled"]
    Enabled = 1,
}
impl From<ADCEN> for bool {
    #[inline(always)]
    fn from(variant: ADCEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCEN` reader - ADC clock enable"]
pub type ADCEN_R = crate::BitReader<ADCEN>;
impl ADCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADCEN {
        match self.bits {
            false => ADCEN::Disabled,
            true => ADCEN::Enabled,
        }
    }
    #[doc = "ADC clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCEN::Disabled
    }
    #[doc = "ADC clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADCEN::Enabled
    }
}
#[doc = "Field `ADCEN` writer - ADC clock enable"]
pub type ADCEN_W<'a, REG> = crate::BitWriter<'a, REG, ADCEN>;
impl<'a, REG> ADCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADCEN::Disabled)
    }
    #[doc = "ADC clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADCEN::Enabled)
    }
}
#[doc = "DCMI clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMIEN {
    #[doc = "0: DCMI/PSSI clock disabled"]
    Disabled = 0,
    #[doc = "1: DCMI/PSSI clock enabled"]
    Enabled = 1,
}
impl From<DCMIEN> for bool {
    #[inline(always)]
    fn from(variant: DCMIEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMIEN` reader - DCMI clock enable"]
pub type DCMIEN_R = crate::BitReader<DCMIEN>;
impl DCMIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCMIEN {
        match self.bits {
            false => DCMIEN::Disabled,
            true => DCMIEN::Enabled,
        }
    }
    #[doc = "DCMI/PSSI clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCMIEN::Disabled
    }
    #[doc = "DCMI/PSSI clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCMIEN::Enabled
    }
}
#[doc = "Field `DCMIEN` writer - DCMI clock enable"]
pub type DCMIEN_W<'a, REG> = crate::BitWriter<'a, REG, DCMIEN>;
impl<'a, REG> DCMIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DCMI/PSSI clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMIEN::Disabled)
    }
    #[doc = "DCMI/PSSI clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMIEN::Enabled)
    }
}
#[doc = "PKA clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PKAEN {
    #[doc = "0: PKA clock disabled"]
    Disabled = 0,
    #[doc = "1: PKA clock enabled"]
    Enabled = 1,
}
impl From<PKAEN> for bool {
    #[inline(always)]
    fn from(variant: PKAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PKAEN` reader - PKA clock enable"]
pub type PKAEN_R = crate::BitReader<PKAEN>;
impl PKAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PKAEN {
        match self.bits {
            false => PKAEN::Disabled,
            true => PKAEN::Enabled,
        }
    }
    #[doc = "PKA clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PKAEN::Disabled
    }
    #[doc = "PKA clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PKAEN::Enabled
    }
}
#[doc = "Field `PKAEN` writer - PKA clock enable"]
pub type PKAEN_W<'a, REG> = crate::BitWriter<'a, REG, PKAEN>;
impl<'a, REG> PKAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PKA clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PKAEN::Disabled)
    }
    #[doc = "PKA clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PKAEN::Enabled)
    }
}
#[doc = "AES accelerator clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESEN {
    #[doc = "0: AES clock disabled"]
    Disabled = 0,
    #[doc = "1: AES clock enabled"]
    Enabled = 1,
}
impl From<AESEN> for bool {
    #[inline(always)]
    fn from(variant: AESEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESEN` reader - AES accelerator clock enable"]
pub type AESEN_R = crate::BitReader<AESEN>;
impl AESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AESEN {
        match self.bits {
            false => AESEN::Disabled,
            true => AESEN::Enabled,
        }
    }
    #[doc = "AES clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AESEN::Disabled
    }
    #[doc = "AES clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AESEN::Enabled
    }
}
#[doc = "Field `AESEN` writer - AES accelerator clock enable"]
pub type AESEN_W<'a, REG> = crate::BitWriter<'a, REG, AESEN>;
impl<'a, REG> AESEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AES clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AESEN::Disabled)
    }
    #[doc = "AES clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AESEN::Enabled)
    }
}
#[doc = "HASH clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HASHEN {
    #[doc = "0: HASH clock disabled"]
    Disabled = 0,
    #[doc = "1: HASH clock enabled"]
    Enabled = 1,
}
impl From<HASHEN> for bool {
    #[inline(always)]
    fn from(variant: HASHEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASHEN` reader - HASH clock enable"]
pub type HASHEN_R = crate::BitReader<HASHEN>;
impl HASHEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HASHEN {
        match self.bits {
            false => HASHEN::Disabled,
            true => HASHEN::Enabled,
        }
    }
    #[doc = "HASH clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HASHEN::Disabled
    }
    #[doc = "HASH clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HASHEN::Enabled
    }
}
#[doc = "Field `HASHEN` writer - HASH clock enable"]
pub type HASHEN_W<'a, REG> = crate::BitWriter<'a, REG, HASHEN>;
impl<'a, REG> HASHEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HASH clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HASHEN::Disabled)
    }
    #[doc = "HASH clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HASHEN::Enabled)
    }
}
#[doc = "Random Number Generator clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGEN {
    #[doc = "0: Random Number Generator clock disabled"]
    Disabled = 0,
    #[doc = "1: Random Number Generator clock enabled"]
    Enabled = 1,
}
impl From<RNGEN> for bool {
    #[inline(always)]
    fn from(variant: RNGEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNGEN` reader - Random Number Generator clock enable"]
pub type RNGEN_R = crate::BitReader<RNGEN>;
impl RNGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RNGEN {
        match self.bits {
            false => RNGEN::Disabled,
            true => RNGEN::Enabled,
        }
    }
    #[doc = "Random Number Generator clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RNGEN::Disabled
    }
    #[doc = "Random Number Generator clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RNGEN::Enabled
    }
}
#[doc = "Field `RNGEN` writer - Random Number Generator clock enable"]
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG, RNGEN>;
impl<'a, REG> RNGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Random Number Generator clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RNGEN::Disabled)
    }
    #[doc = "Random Number Generator clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RNGEN::Enabled)
    }
}
#[doc = "OctoSPI IO manager clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSPIMEN {
    #[doc = "0: OctoSPI IO manager clock disabled"]
    Disabled = 0,
    #[doc = "1: OctoSPI IO manager clock enabled"]
    Enabled = 1,
}
impl From<OSPIMEN> for bool {
    #[inline(always)]
    fn from(variant: OSPIMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSPIMEN` reader - OctoSPI IO manager clock enable"]
pub type OSPIMEN_R = crate::BitReader<OSPIMEN>;
impl OSPIMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSPIMEN {
        match self.bits {
            false => OSPIMEN::Disabled,
            true => OSPIMEN::Enabled,
        }
    }
    #[doc = "OctoSPI IO manager clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSPIMEN::Disabled
    }
    #[doc = "OctoSPI IO manager clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSPIMEN::Enabled
    }
}
#[doc = "Field `OSPIMEN` writer - OctoSPI IO manager clock enable"]
pub type OSPIMEN_W<'a, REG> = crate::BitWriter<'a, REG, OSPIMEN>;
impl<'a, REG> OSPIMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OctoSPI IO manager clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSPIMEN::Disabled)
    }
    #[doc = "OctoSPI IO manager clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSPIMEN::Enabled)
    }
}
#[doc = "SDMMC1 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMC1EN {
    #[doc = "0: SDMMCx clock disabled"]
    Disabled = 0,
    #[doc = "1: SDMMCx clock enabled"]
    Enabled = 1,
}
impl From<SDMMC1EN> for bool {
    #[inline(always)]
    fn from(variant: SDMMC1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMMC1EN` reader - SDMMC1 clock enable"]
pub type SDMMC1EN_R = crate::BitReader<SDMMC1EN>;
impl SDMMC1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDMMC1EN {
        match self.bits {
            false => SDMMC1EN::Disabled,
            true => SDMMC1EN::Enabled,
        }
    }
    #[doc = "SDMMCx clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDMMC1EN::Disabled
    }
    #[doc = "SDMMCx clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDMMC1EN::Enabled
    }
}
#[doc = "Field `SDMMC1EN` writer - SDMMC1 clock enable"]
pub type SDMMC1EN_W<'a, REG> = crate::BitWriter<'a, REG, SDMMC1EN>;
impl<'a, REG> SDMMC1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDMMCx clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1EN::Disabled)
    }
    #[doc = "SDMMCx clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1EN::Enabled)
    }
}
#[doc = "Field `SDMMC2EN` reader - SDMMC2 clock enable"]
pub use SDMMC1EN_R as SDMMC2EN_R;
#[doc = "Field `SDMMC2EN` writer - SDMMC2 clock enable"]
pub use SDMMC1EN_W as SDMMC2EN_W;
impl R {
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F clock enable"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G clock enable"]
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H clock enable"]
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IO port I clock enable"]
    #[inline(always)]
    pub fn gpioien(&self) -> GPIOIEN_R {
        GPIOIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - OTG full speed clock enable"]
    #[inline(always)]
    pub fn otgfsen(&self) -> OTGFSEN_R {
        OTGFSEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC clock enable"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DCMI clock enable"]
    #[inline(always)]
    pub fn dcmien(&self) -> DCMIEN_R {
        DCMIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PKA clock enable"]
    #[inline(always)]
    pub fn pkaen(&self) -> PKAEN_R {
        PKAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - AES accelerator clock enable"]
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HASH clock enable"]
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Random Number Generator clock enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - OctoSPI IO manager clock enable"]
    #[inline(always)]
    pub fn ospimen(&self) -> OSPIMEN_R {
        OSPIMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - SDMMC1 clock enable"]
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SDMMC2 clock enable"]
    #[inline(always)]
    pub fn sdmmc2en(&self) -> SDMMC2EN_R {
        SDMMC2EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<AHB2ENRrs> {
        GPIOAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<AHB2ENRrs> {
        GPIOBEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<AHB2ENRrs> {
        GPIOCEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GPIODEN_W<AHB2ENRrs> {
        GPIODEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<AHB2ENRrs> {
        GPIOEEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - IO port F clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<AHB2ENRrs> {
        GPIOFEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - IO port G clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<AHB2ENRrs> {
        GPIOGEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - IO port H clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<AHB2ENRrs> {
        GPIOHEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - IO port I clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioien(&mut self) -> GPIOIEN_W<AHB2ENRrs> {
        GPIOIEN_W::new(self, 8)
    }
    #[doc = "Bit 12 - OTG full speed clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn otgfsen(&mut self) -> OTGFSEN_W<AHB2ENRrs> {
        OTGFSEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - ADC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> ADCEN_W<AHB2ENRrs> {
        ADCEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - DCMI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcmien(&mut self) -> DCMIEN_W<AHB2ENRrs> {
        DCMIEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - PKA clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pkaen(&mut self) -> PKAEN_W<AHB2ENRrs> {
        PKAEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - AES accelerator clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn aesen(&mut self) -> AESEN_W<AHB2ENRrs> {
        AESEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - HASH clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hashen(&mut self) -> HASHEN_W<AHB2ENRrs> {
        HASHEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Random Number Generator clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<AHB2ENRrs> {
        RNGEN_W::new(self, 18)
    }
    #[doc = "Bit 20 - OctoSPI IO manager clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ospimen(&mut self) -> OSPIMEN_W<AHB2ENRrs> {
        OSPIMEN_W::new(self, 20)
    }
    #[doc = "Bit 22 - SDMMC1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<AHB2ENRrs> {
        SDMMC1EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - SDMMC2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2en(&mut self) -> SDMMC2EN_W<AHB2ENRrs> {
        SDMMC2EN_W::new(self, 23)
    }
}
#[doc = "AHB2 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2ENRrs;
impl crate::RegisterSpec for AHB2ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2enr::R`](R) reader structure"]
impl crate::Readable for AHB2ENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb2enr::W`](W) writer structure"]
impl crate::Writable for AHB2ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB2ENR to value 0"]
impl crate::Resettable for AHB2ENRrs {
    const RESET_VALUE: u32 = 0;
}
