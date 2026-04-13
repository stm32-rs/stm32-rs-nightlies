///Register `ADC_IER` reader
pub type R = crate::R<ADC_IERrs>;
///Register `ADC_IER` writer
pub type W = crate::W<ADC_IERrs>;
/**ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYIE {
    ///0: ADRDY interrupt disabled.
    B0x0 = 0,
    ///1: ADRDY interrupt enabled. An interrupt is generated when the ADRDY bit is set.
    B0x1 = 1,
}
impl From<ADRDYIE> for bool {
    #[inline(always)]
    fn from(variant: ADRDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ADRDYIE` reader - ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
pub type ADRDYIE_R = crate::BitReader<ADRDYIE>;
impl ADRDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADRDYIE {
        match self.bits {
            false => ADRDYIE::B0x0,
            true => ADRDYIE::B0x1,
        }
    }
    ///ADRDY interrupt disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADRDYIE::B0x0
    }
    ///ADRDY interrupt enabled. An interrupt is generated when the ADRDY bit is set.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADRDYIE::B0x1
    }
}
///Field `ADRDYIE` writer - ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
pub type ADRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, ADRDYIE>;
impl<'a, REG> ADRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADRDY interrupt disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADRDYIE::B0x0)
    }
    ///ADRDY interrupt enabled. An interrupt is generated when the ADRDY bit is set.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADRDYIE::B0x1)
    }
}
/**End of sampling flag interrupt enable This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPIE {
    ///0: EOSMP interrupt disabled.
    B0x0 = 0,
    ///1: EOSMP interrupt enabled. An interrupt is generated when the EOSMP bit is set.
    B0x1 = 1,
}
impl From<EOSMPIE> for bool {
    #[inline(always)]
    fn from(variant: EOSMPIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSMPIE` reader - End of sampling flag interrupt enable This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
pub type EOSMPIE_R = crate::BitReader<EOSMPIE>;
impl EOSMPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOSMPIE {
        match self.bits {
            false => EOSMPIE::B0x0,
            true => EOSMPIE::B0x1,
        }
    }
    ///EOSMP interrupt disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EOSMPIE::B0x0
    }
    ///EOSMP interrupt enabled. An interrupt is generated when the EOSMP bit is set.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EOSMPIE::B0x1
    }
}
///Field `EOSMPIE` writer - End of sampling flag interrupt enable This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
pub type EOSMPIE_W<'a, REG> = crate::BitWriter<'a, REG, EOSMPIE>;
impl<'a, REG> EOSMPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///EOSMP interrupt disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMPIE::B0x0)
    }
    ///EOSMP interrupt enabled. An interrupt is generated when the EOSMP bit is set.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMPIE::B0x1)
    }
}
/**End of conversion interrupt enable This bit is set and cleared by software to enable/disable the end of conversion interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCIE {
    ///0: EOC interrupt disabled
    B0x0 = 0,
    ///1: EOC interrupt enabled. An interrupt is generated when the EOC bit is set.
    B0x1 = 1,
}
impl From<EOCIE> for bool {
    #[inline(always)]
    fn from(variant: EOCIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EOCIE` reader - End of conversion interrupt enable This bit is set and cleared by software to enable/disable the end of conversion interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
pub type EOCIE_R = crate::BitReader<EOCIE>;
impl EOCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOCIE {
        match self.bits {
            false => EOCIE::B0x0,
            true => EOCIE::B0x1,
        }
    }
    ///EOC interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EOCIE::B0x0
    }
    ///EOC interrupt enabled. An interrupt is generated when the EOC bit is set.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EOCIE::B0x1
    }
}
///Field `EOCIE` writer - End of conversion interrupt enable This bit is set and cleared by software to enable/disable the end of conversion interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
pub type EOCIE_W<'a, REG> = crate::BitWriter<'a, REG, EOCIE>;
impl<'a, REG> EOCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///EOC interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EOCIE::B0x0)
    }
    ///EOC interrupt enabled. An interrupt is generated when the EOC bit is set.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EOCIE::B0x1)
    }
}
/**End of conversion sequence interrupt enable This bit is set and cleared by software to enable/disable the end of sequence of conversions interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSIE {
    ///0: EOS interrupt disabled
    B0x0 = 0,
    ///1: EOS interrupt enabled. An interrupt is generated when the EOS bit is set.
    B0x1 = 1,
}
impl From<EOSIE> for bool {
    #[inline(always)]
    fn from(variant: EOSIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSIE` reader - End of conversion sequence interrupt enable This bit is set and cleared by software to enable/disable the end of sequence of conversions interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
pub type EOSIE_R = crate::BitReader<EOSIE>;
impl EOSIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOSIE {
        match self.bits {
            false => EOSIE::B0x0,
            true => EOSIE::B0x1,
        }
    }
    ///EOS interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EOSIE::B0x0
    }
    ///EOS interrupt enabled. An interrupt is generated when the EOS bit is set.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EOSIE::B0x1
    }
}
///Field `EOSIE` writer - End of conversion sequence interrupt enable This bit is set and cleared by software to enable/disable the end of sequence of conversions interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
pub type EOSIE_W<'a, REG> = crate::BitWriter<'a, REG, EOSIE>;
impl<'a, REG> EOSIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///EOS interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EOSIE::B0x0)
    }
    ///EOS interrupt enabled. An interrupt is generated when the EOS bit is set.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EOSIE::B0x1)
    }
}
/**Overrun interrupt enable This bit is set and cleared by software to enable/disable the overrun interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRIE {
    ///0: Overrun interrupt disabled
    B0x0 = 0,
    ///1: Overrun interrupt enabled. An interrupt is generated when the OVR bit is set.
    B0x1 = 1,
}
impl From<OVRIE> for bool {
    #[inline(always)]
    fn from(variant: OVRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRIE` reader - Overrun interrupt enable This bit is set and cleared by software to enable/disable the overrun interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
pub type OVRIE_R = crate::BitReader<OVRIE>;
impl OVRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVRIE {
        match self.bits {
            false => OVRIE::B0x0,
            true => OVRIE::B0x1,
        }
    }
    ///Overrun interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OVRIE::B0x0
    }
    ///Overrun interrupt enabled. An interrupt is generated when the OVR bit is set.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OVRIE::B0x1
    }
}
///Field `OVRIE` writer - Overrun interrupt enable This bit is set and cleared by software to enable/disable the overrun interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG, OVRIE>;
impl<'a, REG> OVRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Overrun interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OVRIE::B0x0)
    }
    ///Overrun interrupt enabled. An interrupt is generated when the OVR bit is set.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OVRIE::B0x1)
    }
}
/**Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1IE {
    ///0: Analog watchdog interrupt disabled
    B0x0 = 0,
    ///1: Analog watchdog interrupt enabled
    B0x1 = 1,
}
impl From<AWD1IE> for bool {
    #[inline(always)]
    fn from(variant: AWD1IE) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD1IE` reader - Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
pub type AWD1IE_R = crate::BitReader<AWD1IE>;
impl AWD1IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD1IE {
        match self.bits {
            false => AWD1IE::B0x0,
            true => AWD1IE::B0x1,
        }
    }
    ///Analog watchdog interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD1IE::B0x0
    }
    ///Analog watchdog interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD1IE::B0x1
    }
}
///Field `AWD1IE` writer - Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
pub type AWD1IE_W<'a, REG> = crate::BitWriter<'a, REG, AWD1IE>;
impl<'a, REG> AWD1IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1IE::B0x0)
    }
    ///Analog watchdog interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1IE::B0x1)
    }
}
/**Analog watchdog 2 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2IE {
    ///0: Analog watchdog interrupt disabled
    B0x0 = 0,
    ///1: Analog watchdog interrupt enabled
    B0x1 = 1,
}
impl From<AWD2IE> for bool {
    #[inline(always)]
    fn from(variant: AWD2IE) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2IE` reader - Analog watchdog 2 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
pub type AWD2IE_R = crate::BitReader<AWD2IE>;
impl AWD2IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2IE {
        match self.bits {
            false => AWD2IE::B0x0,
            true => AWD2IE::B0x1,
        }
    }
    ///Analog watchdog interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2IE::B0x0
    }
    ///Analog watchdog interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2IE::B0x1
    }
}
///Field `AWD2IE` writer - Analog watchdog 2 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
pub type AWD2IE_W<'a, REG> = crate::BitWriter<'a, REG, AWD2IE>;
impl<'a, REG> AWD2IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2IE::B0x0)
    }
    ///Analog watchdog interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2IE::B0x1)
    }
}
/**Analog watchdog 3 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3IE {
    ///0: Analog watchdog interrupt disabled
    B0x0 = 0,
    ///1: Analog watchdog interrupt enabled
    B0x1 = 1,
}
impl From<AWD3IE> for bool {
    #[inline(always)]
    fn from(variant: AWD3IE) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3IE` reader - Analog watchdog 3 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
pub type AWD3IE_R = crate::BitReader<AWD3IE>;
impl AWD3IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3IE {
        match self.bits {
            false => AWD3IE::B0x0,
            true => AWD3IE::B0x1,
        }
    }
    ///Analog watchdog interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3IE::B0x0
    }
    ///Analog watchdog interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3IE::B0x1
    }
}
///Field `AWD3IE` writer - Analog watchdog 3 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
pub type AWD3IE_W<'a, REG> = crate::BitWriter<'a, REG, AWD3IE>;
impl<'a, REG> AWD3IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3IE::B0x0)
    }
    ///Analog watchdog interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3IE::B0x1)
    }
}
/**End of calibration interrupt enable This bit is set and cleared by software to enable/disable the end of calibration interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCALIE {
    ///0: End of calibration interrupt disabled
    B0x0 = 0,
    ///1: End of calibration interrupt enabled
    B0x1 = 1,
}
impl From<EOCALIE> for bool {
    #[inline(always)]
    fn from(variant: EOCALIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EOCALIE` reader - End of calibration interrupt enable This bit is set and cleared by software to enable/disable the end of calibration interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
pub type EOCALIE_R = crate::BitReader<EOCALIE>;
impl EOCALIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOCALIE {
        match self.bits {
            false => EOCALIE::B0x0,
            true => EOCALIE::B0x1,
        }
    }
    ///End of calibration interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EOCALIE::B0x0
    }
    ///End of calibration interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EOCALIE::B0x1
    }
}
///Field `EOCALIE` writer - End of calibration interrupt enable This bit is set and cleared by software to enable/disable the end of calibration interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
pub type EOCALIE_W<'a, REG> = crate::BitWriter<'a, REG, EOCALIE>;
impl<'a, REG> EOCALIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///End of calibration interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EOCALIE::B0x0)
    }
    ///End of calibration interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EOCALIE::B0x1)
    }
}
/**Channel Configuration Ready Interrupt enable This bit is set and cleared by software to enable/disable the channel configuration ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCRDYIE {
    ///0: Channel configuration ready interrupt disabled
    B0x0 = 0,
    ///1: Channel configuration ready interrupt enabled
    B0x1 = 1,
}
impl From<CCRDYIE> for bool {
    #[inline(always)]
    fn from(variant: CCRDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `CCRDYIE` reader - Channel Configuration Ready Interrupt enable This bit is set and cleared by software to enable/disable the channel configuration ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
pub type CCRDYIE_R = crate::BitReader<CCRDYIE>;
impl CCRDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CCRDYIE {
        match self.bits {
            false => CCRDYIE::B0x0,
            true => CCRDYIE::B0x1,
        }
    }
    ///Channel configuration ready interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CCRDYIE::B0x0
    }
    ///Channel configuration ready interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CCRDYIE::B0x1
    }
}
///Field `CCRDYIE` writer - Channel Configuration Ready Interrupt enable This bit is set and cleared by software to enable/disable the channel configuration ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
pub type CCRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, CCRDYIE>;
impl<'a, REG> CCRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel configuration ready interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CCRDYIE::B0x0)
    }
    ///Channel configuration ready interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CCRDYIE::B0x1)
    }
}
impl R {
    ///Bit 0 - ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn adrdyie(&self) -> ADRDYIE_R {
        ADRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of sampling flag interrupt enable This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn eosmpie(&self) -> EOSMPIE_R {
        EOSMPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - End of conversion interrupt enable This bit is set and cleared by software to enable/disable the end of conversion interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - End of conversion sequence interrupt enable This bit is set and cleared by software to enable/disable the end of sequence of conversions interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn eosie(&self) -> EOSIE_R {
        EOSIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Overrun interrupt enable This bit is set and cleared by software to enable/disable the overrun interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd1ie(&self) -> AWD1IE_R {
        AWD1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Analog watchdog 2 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ie(&self) -> AWD2IE_R {
        AWD2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Analog watchdog 3 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ie(&self) -> AWD3IE_R {
        AWD3IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - End of calibration interrupt enable This bit is set and cleared by software to enable/disable the end of calibration interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn eocalie(&self) -> EOCALIE_R {
        EOCALIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - Channel Configuration Ready Interrupt enable This bit is set and cleared by software to enable/disable the channel configuration ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ccrdyie(&self) -> CCRDYIE_R {
        CCRDYIE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_IER")
            .field("adrdyie", &self.adrdyie())
            .field("eosmpie", &self.eosmpie())
            .field("eocie", &self.eocie())
            .field("eosie", &self.eosie())
            .field("ovrie", &self.ovrie())
            .field("awd1ie", &self.awd1ie())
            .field("awd2ie", &self.awd2ie())
            .field("awd3ie", &self.awd3ie())
            .field("eocalie", &self.eocalie())
            .field("ccrdyie", &self.ccrdyie())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn adrdyie(&mut self) -> ADRDYIE_W<'_, ADC_IERrs> {
        ADRDYIE_W::new(self, 0)
    }
    ///Bit 1 - End of sampling flag interrupt enable This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn eosmpie(&mut self) -> EOSMPIE_W<'_, ADC_IERrs> {
        EOSMPIE_W::new(self, 1)
    }
    ///Bit 2 - End of conversion interrupt enable This bit is set and cleared by software to enable/disable the end of conversion interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn eocie(&mut self) -> EOCIE_W<'_, ADC_IERrs> {
        EOCIE_W::new(self, 2)
    }
    ///Bit 3 - End of conversion sequence interrupt enable This bit is set and cleared by software to enable/disable the end of sequence of conversions interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn eosie(&mut self) -> EOSIE_W<'_, ADC_IERrs> {
        EOSIE_W::new(self, 3)
    }
    ///Bit 4 - Overrun interrupt enable This bit is set and cleared by software to enable/disable the overrun interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ovrie(&mut self) -> OVRIE_W<'_, ADC_IERrs> {
        OVRIE_W::new(self, 4)
    }
    ///Bit 7 - Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd1ie(&mut self) -> AWD1IE_W<'_, ADC_IERrs> {
        AWD1IE_W::new(self, 7)
    }
    ///Bit 8 - Analog watchdog 2 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ie(&mut self) -> AWD2IE_W<'_, ADC_IERrs> {
        AWD2IE_W::new(self, 8)
    }
    ///Bit 9 - Analog watchdog 3 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog interrupt. Note: The Software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ie(&mut self) -> AWD3IE_W<'_, ADC_IERrs> {
        AWD3IE_W::new(self, 9)
    }
    ///Bit 11 - End of calibration interrupt enable This bit is set and cleared by software to enable/disable the end of calibration interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn eocalie(&mut self) -> EOCALIE_W<'_, ADC_IERrs> {
        EOCALIE_W::new(self, 11)
    }
    ///Bit 13 - Channel Configuration Ready Interrupt enable This bit is set and cleared by software to enable/disable the channel configuration ready interrupt. Note: The software is allowed to write this bit only when ADSTART bit is cleared (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ccrdyie(&mut self) -> CCRDYIE_W<'_, ADC_IERrs> {
        CCRDYIE_W::new(self, 13)
    }
}
/**ADC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`adc_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#ADC:ADC_IER)*/
pub struct ADC_IERrs;
impl crate::RegisterSpec for ADC_IERrs {
    type Ux = u32;
}
///`read()` method returns [`adc_ier::R`](R) reader structure
impl crate::Readable for ADC_IERrs {}
///`write(|w| ..)` method takes [`adc_ier::W`](W) writer structure
impl crate::Writable for ADC_IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADC_IER to value 0
impl crate::Resettable for ADC_IERrs {}
