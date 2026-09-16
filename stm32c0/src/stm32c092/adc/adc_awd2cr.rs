///Register `ADC_AWD2CR` reader
pub type R = crate::R<ADC_AWD2CRrs>;
///Register `ADC_AWD2CR` writer
pub type W = crate::W<ADC_AWD2CRrs>;
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH0 {
    ///0: ADC analog channel-x is not monitored by AWD2
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD2
    B0x1 = 1,
}
impl From<AWD2CH0> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH0) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH0` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH0_R = crate::BitReader<AWD2CH0>;
impl AWD2CH0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH0 {
        match self.bits {
            false => AWD2CH0::B0x0,
            true => AWD2CH0::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2CH0::B0x0
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2CH0::B0x1
    }
}
///Field `AWD2CH0` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH0_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH0>;
impl<'a, REG> AWD2CH0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH0::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH0::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH1 {
    ///0: ADC analog channel-x is not monitored by AWD2
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD2
    B0x1 = 1,
}
impl From<AWD2CH1> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH1) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH1` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH1_R = crate::BitReader<AWD2CH1>;
impl AWD2CH1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH1 {
        match self.bits {
            false => AWD2CH1::B0x0,
            true => AWD2CH1::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2CH1::B0x0
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2CH1::B0x1
    }
}
///Field `AWD2CH1` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH1_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH1>;
impl<'a, REG> AWD2CH1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH1::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH1::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH2 {
    ///0: ADC analog channel-x is not monitored by AWD2
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD2
    B0x1 = 1,
}
impl From<AWD2CH2> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH2) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH2` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH2_R = crate::BitReader<AWD2CH2>;
impl AWD2CH2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH2 {
        match self.bits {
            false => AWD2CH2::B0x0,
            true => AWD2CH2::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2CH2::B0x0
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2CH2::B0x1
    }
}
///Field `AWD2CH2` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH2_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH2>;
impl<'a, REG> AWD2CH2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH2::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH2::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH3 {
    ///0: ADC analog channel-x is not monitored by AWD2
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD2
    B0x1 = 1,
}
impl From<AWD2CH3> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH3) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH3` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH3_R = crate::BitReader<AWD2CH3>;
impl AWD2CH3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH3 {
        match self.bits {
            false => AWD2CH3::B0x0,
            true => AWD2CH3::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2CH3::B0x0
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2CH3::B0x1
    }
}
///Field `AWD2CH3` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH3_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH3>;
impl<'a, REG> AWD2CH3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH3::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH3::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH4 {
    ///0: ADC analog channel-x is not monitored by AWD2
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD2
    B0x1 = 1,
}
impl From<AWD2CH4> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH4) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH4` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH4_R = crate::BitReader<AWD2CH4>;
impl AWD2CH4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH4 {
        match self.bits {
            false => AWD2CH4::B0x0,
            true => AWD2CH4::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2CH4::B0x0
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2CH4::B0x1
    }
}
///Field `AWD2CH4` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH4_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH4>;
impl<'a, REG> AWD2CH4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH4::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH4::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH5 {
    ///0: ADC analog channel-x is not monitored by AWD2
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD2
    B0x1 = 1,
}
impl From<AWD2CH5> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH5) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH5` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH5_R = crate::BitReader<AWD2CH5>;
impl AWD2CH5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH5 {
        match self.bits {
            false => AWD2CH5::B0x0,
            true => AWD2CH5::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2CH5::B0x0
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2CH5::B0x1
    }
}
///Field `AWD2CH5` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH5_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH5>;
impl<'a, REG> AWD2CH5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH5::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH5::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH6 {
    ///0: ADC analog channel-x is not monitored by AWD2
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD2
    B0x1 = 1,
}
impl From<AWD2CH6> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH6) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH6` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH6_R = crate::BitReader<AWD2CH6>;
impl AWD2CH6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH6 {
        match self.bits {
            false => AWD2CH6::B0x0,
            true => AWD2CH6::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2CH6::B0x0
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2CH6::B0x1
    }
}
///Field `AWD2CH6` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH6_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH6>;
impl<'a, REG> AWD2CH6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH6::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH6::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH7 {
    ///0: ADC analog channel-x is not monitored by AWD2
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD2
    B0x1 = 1,
}
impl From<AWD2CH7> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH7) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH7` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH7_R = crate::BitReader<AWD2CH7>;
impl AWD2CH7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH7 {
        match self.bits {
            false => AWD2CH7::B0x0,
            true => AWD2CH7::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2CH7::B0x0
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2CH7::B0x1
    }
}
///Field `AWD2CH7` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH7_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH7>;
impl<'a, REG> AWD2CH7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH7::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH7::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH8 {
    ///0: ADC analog channel-x is not monitored by AWD2
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD2
    B0x1 = 1,
}
impl From<AWD2CH8> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH8) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH8` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH8_R = crate::BitReader<AWD2CH8>;
impl AWD2CH8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH8 {
        match self.bits {
            false => AWD2CH8::B0x0,
            true => AWD2CH8::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2CH8::B0x0
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2CH8::B0x1
    }
}
///Field `AWD2CH8` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH8_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH8>;
impl<'a, REG> AWD2CH8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH8::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH8::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH9 {
    ///0: ADC analog channel-x is not monitored by AWD2
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD2
    B0x1 = 1,
}
impl From<AWD2CH9> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH9) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH9` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH9_R = crate::BitReader<AWD2CH9>;
impl AWD2CH9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH9 {
        match self.bits {
            false => AWD2CH9::B0x0,
            true => AWD2CH9::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2CH9::B0x0
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2CH9::B0x1
    }
}
///Field `AWD2CH9` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH9_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH9>;
impl<'a, REG> AWD2CH9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH9::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH9::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH10 {
    ///0: ADC analog channel-x is not monitored by AWD2
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD2
    B0x1 = 1,
}
impl From<AWD2CH10> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH10) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH10` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH10_R = crate::BitReader<AWD2CH10>;
impl AWD2CH10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH10 {
        match self.bits {
            false => AWD2CH10::B0x0,
            true => AWD2CH10::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2CH10::B0x0
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2CH10::B0x1
    }
}
///Field `AWD2CH10` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH10_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH10>;
impl<'a, REG> AWD2CH10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH10::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH10::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH11 {
    ///0: ADC analog channel-x is not monitored by AWD2
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD2
    B0x1 = 1,
}
impl From<AWD2CH11> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH11) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH11` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH11_R = crate::BitReader<AWD2CH11>;
impl AWD2CH11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH11 {
        match self.bits {
            false => AWD2CH11::B0x0,
            true => AWD2CH11::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2CH11::B0x0
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2CH11::B0x1
    }
}
///Field `AWD2CH11` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH11_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH11>;
impl<'a, REG> AWD2CH11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH11::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH11::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH12 {
    ///0: ADC analog channel-x is not monitored by AWD2
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD2
    B0x1 = 1,
}
impl From<AWD2CH12> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH12) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH12` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH12_R = crate::BitReader<AWD2CH12>;
impl AWD2CH12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH12 {
        match self.bits {
            false => AWD2CH12::B0x0,
            true => AWD2CH12::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2CH12::B0x0
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2CH12::B0x1
    }
}
///Field `AWD2CH12` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH12_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH12>;
impl<'a, REG> AWD2CH12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH12::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH12::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH13 {
    ///0: ADC analog channel-x is not monitored by AWD2
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD2
    B0x1 = 1,
}
impl From<AWD2CH13> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH13) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH13` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH13_R = crate::BitReader<AWD2CH13>;
impl AWD2CH13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH13 {
        match self.bits {
            false => AWD2CH13::B0x0,
            true => AWD2CH13::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2CH13::B0x0
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2CH13::B0x1
    }
}
///Field `AWD2CH13` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH13_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH13>;
impl<'a, REG> AWD2CH13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH13::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH13::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH14 {
    ///0: ADC analog channel-x is not monitored by AWD2
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD2
    B0x1 = 1,
}
impl From<AWD2CH14> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH14) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH14` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH14_R = crate::BitReader<AWD2CH14>;
impl AWD2CH14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH14 {
        match self.bits {
            false => AWD2CH14::B0x0,
            true => AWD2CH14::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2CH14::B0x0
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2CH14::B0x1
    }
}
///Field `AWD2CH14` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH14_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH14>;
impl<'a, REG> AWD2CH14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH14::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH14::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH15 {
    ///0: ADC analog channel-x is not monitored by AWD2
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD2
    B0x1 = 1,
}
impl From<AWD2CH15> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH15) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH15` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH15_R = crate::BitReader<AWD2CH15>;
impl AWD2CH15_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH15 {
        match self.bits {
            false => AWD2CH15::B0x0,
            true => AWD2CH15::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2CH15::B0x0
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2CH15::B0x1
    }
}
///Field `AWD2CH15` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH15_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH15>;
impl<'a, REG> AWD2CH15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH15::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH15::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH16 {
    ///0: ADC analog channel-x is not monitored by AWD2
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD2
    B0x1 = 1,
}
impl From<AWD2CH16> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH16) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH16` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH16_R = crate::BitReader<AWD2CH16>;
impl AWD2CH16_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH16 {
        match self.bits {
            false => AWD2CH16::B0x0,
            true => AWD2CH16::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2CH16::B0x0
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2CH16::B0x1
    }
}
///Field `AWD2CH16` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH16_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH16>;
impl<'a, REG> AWD2CH16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH16::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH16::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH17 {
    ///0: ADC analog channel-x is not monitored by AWD2
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD2
    B0x1 = 1,
}
impl From<AWD2CH17> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH17) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH17` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH17_R = crate::BitReader<AWD2CH17>;
impl AWD2CH17_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH17 {
        match self.bits {
            false => AWD2CH17::B0x0,
            true => AWD2CH17::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2CH17::B0x0
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2CH17::B0x1
    }
}
///Field `AWD2CH17` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH17_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH17>;
impl<'a, REG> AWD2CH17_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH17::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH17::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH18 {
    ///0: ADC analog channel-x is not monitored by AWD2
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD2
    B0x1 = 1,
}
impl From<AWD2CH18> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH18) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH18` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH18_R = crate::BitReader<AWD2CH18>;
impl AWD2CH18_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH18 {
        match self.bits {
            false => AWD2CH18::B0x0,
            true => AWD2CH18::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2CH18::B0x0
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2CH18::B0x1
    }
}
///Field `AWD2CH18` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH18_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH18>;
impl<'a, REG> AWD2CH18_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH18::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH18::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH19 {
    ///0: ADC analog channel-x is not monitored by AWD2
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD2
    B0x1 = 1,
}
impl From<AWD2CH19> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH19) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH19` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH19_R = crate::BitReader<AWD2CH19>;
impl AWD2CH19_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH19 {
        match self.bits {
            false => AWD2CH19::B0x0,
            true => AWD2CH19::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2CH19::B0x0
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2CH19::B0x1
    }
}
///Field `AWD2CH19` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH19_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH19>;
impl<'a, REG> AWD2CH19_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH19::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH19::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH20 {
    ///0: ADC analog channel-x is not monitored by AWD2
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD2
    B0x1 = 1,
}
impl From<AWD2CH20> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH20) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH20` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH20_R = crate::BitReader<AWD2CH20>;
impl AWD2CH20_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH20 {
        match self.bits {
            false => AWD2CH20::B0x0,
            true => AWD2CH20::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2CH20::B0x0
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2CH20::B0x1
    }
}
///Field `AWD2CH20` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH20_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH20>;
impl<'a, REG> AWD2CH20_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH20::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH20::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH21 {
    ///0: ADC analog channel-x is not monitored by AWD2
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD2
    B0x1 = 1,
}
impl From<AWD2CH21> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH21) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH21` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH21_R = crate::BitReader<AWD2CH21>;
impl AWD2CH21_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH21 {
        match self.bits {
            false => AWD2CH21::B0x0,
            true => AWD2CH21::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2CH21::B0x0
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2CH21::B0x1
    }
}
///Field `AWD2CH21` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH21_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH21>;
impl<'a, REG> AWD2CH21_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH21::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH21::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH22 {
    ///0: ADC analog channel-x is not monitored by AWD2
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD2
    B0x1 = 1,
}
impl From<AWD2CH22> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH22) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH22` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH22_R = crate::BitReader<AWD2CH22>;
impl AWD2CH22_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH22 {
        match self.bits {
            false => AWD2CH22::B0x0,
            true => AWD2CH22::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2CH22::B0x0
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2CH22::B0x1
    }
}
///Field `AWD2CH22` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWD2CH22_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH22>;
impl<'a, REG> AWD2CH22_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH22::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH22::B0x1)
    }
}
impl R {
    ///Bit 0 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch0(&self) -> AWD2CH0_R {
        AWD2CH0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch1(&self) -> AWD2CH1_R {
        AWD2CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch2(&self) -> AWD2CH2_R {
        AWD2CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch3(&self) -> AWD2CH3_R {
        AWD2CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch4(&self) -> AWD2CH4_R {
        AWD2CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch5(&self) -> AWD2CH5_R {
        AWD2CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch6(&self) -> AWD2CH6_R {
        AWD2CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch7(&self) -> AWD2CH7_R {
        AWD2CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch8(&self) -> AWD2CH8_R {
        AWD2CH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch9(&self) -> AWD2CH9_R {
        AWD2CH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch10(&self) -> AWD2CH10_R {
        AWD2CH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch11(&self) -> AWD2CH11_R {
        AWD2CH11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch12(&self) -> AWD2CH12_R {
        AWD2CH12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch13(&self) -> AWD2CH13_R {
        AWD2CH13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch14(&self) -> AWD2CH14_R {
        AWD2CH14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch15(&self) -> AWD2CH15_R {
        AWD2CH15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch16(&self) -> AWD2CH16_R {
        AWD2CH16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch17(&self) -> AWD2CH17_R {
        AWD2CH17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch18(&self) -> AWD2CH18_R {
        AWD2CH18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch19(&self) -> AWD2CH19_R {
        AWD2CH19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch20(&self) -> AWD2CH20_R {
        AWD2CH20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch21(&self) -> AWD2CH21_R {
        AWD2CH21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch22(&self) -> AWD2CH22_R {
        AWD2CH22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_AWD2CR")
            .field("awd2ch0", &self.awd2ch0())
            .field("awd2ch1", &self.awd2ch1())
            .field("awd2ch2", &self.awd2ch2())
            .field("awd2ch3", &self.awd2ch3())
            .field("awd2ch4", &self.awd2ch4())
            .field("awd2ch5", &self.awd2ch5())
            .field("awd2ch6", &self.awd2ch6())
            .field("awd2ch7", &self.awd2ch7())
            .field("awd2ch8", &self.awd2ch8())
            .field("awd2ch9", &self.awd2ch9())
            .field("awd2ch10", &self.awd2ch10())
            .field("awd2ch11", &self.awd2ch11())
            .field("awd2ch12", &self.awd2ch12())
            .field("awd2ch13", &self.awd2ch13())
            .field("awd2ch14", &self.awd2ch14())
            .field("awd2ch15", &self.awd2ch15())
            .field("awd2ch16", &self.awd2ch16())
            .field("awd2ch17", &self.awd2ch17())
            .field("awd2ch18", &self.awd2ch18())
            .field("awd2ch19", &self.awd2ch19())
            .field("awd2ch20", &self.awd2ch20())
            .field("awd2ch21", &self.awd2ch21())
            .field("awd2ch22", &self.awd2ch22())
            .finish()
    }
}
impl W {
    ///Bit 0 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch0(&mut self) -> AWD2CH0_W<'_, ADC_AWD2CRrs> {
        AWD2CH0_W::new(self, 0)
    }
    ///Bit 1 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch1(&mut self) -> AWD2CH1_W<'_, ADC_AWD2CRrs> {
        AWD2CH1_W::new(self, 1)
    }
    ///Bit 2 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch2(&mut self) -> AWD2CH2_W<'_, ADC_AWD2CRrs> {
        AWD2CH2_W::new(self, 2)
    }
    ///Bit 3 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch3(&mut self) -> AWD2CH3_W<'_, ADC_AWD2CRrs> {
        AWD2CH3_W::new(self, 3)
    }
    ///Bit 4 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch4(&mut self) -> AWD2CH4_W<'_, ADC_AWD2CRrs> {
        AWD2CH4_W::new(self, 4)
    }
    ///Bit 5 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch5(&mut self) -> AWD2CH5_W<'_, ADC_AWD2CRrs> {
        AWD2CH5_W::new(self, 5)
    }
    ///Bit 6 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch6(&mut self) -> AWD2CH6_W<'_, ADC_AWD2CRrs> {
        AWD2CH6_W::new(self, 6)
    }
    ///Bit 7 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch7(&mut self) -> AWD2CH7_W<'_, ADC_AWD2CRrs> {
        AWD2CH7_W::new(self, 7)
    }
    ///Bit 8 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch8(&mut self) -> AWD2CH8_W<'_, ADC_AWD2CRrs> {
        AWD2CH8_W::new(self, 8)
    }
    ///Bit 9 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch9(&mut self) -> AWD2CH9_W<'_, ADC_AWD2CRrs> {
        AWD2CH9_W::new(self, 9)
    }
    ///Bit 10 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch10(&mut self) -> AWD2CH10_W<'_, ADC_AWD2CRrs> {
        AWD2CH10_W::new(self, 10)
    }
    ///Bit 11 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch11(&mut self) -> AWD2CH11_W<'_, ADC_AWD2CRrs> {
        AWD2CH11_W::new(self, 11)
    }
    ///Bit 12 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch12(&mut self) -> AWD2CH12_W<'_, ADC_AWD2CRrs> {
        AWD2CH12_W::new(self, 12)
    }
    ///Bit 13 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch13(&mut self) -> AWD2CH13_W<'_, ADC_AWD2CRrs> {
        AWD2CH13_W::new(self, 13)
    }
    ///Bit 14 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch14(&mut self) -> AWD2CH14_W<'_, ADC_AWD2CRrs> {
        AWD2CH14_W::new(self, 14)
    }
    ///Bit 15 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch15(&mut self) -> AWD2CH15_W<'_, ADC_AWD2CRrs> {
        AWD2CH15_W::new(self, 15)
    }
    ///Bit 16 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch16(&mut self) -> AWD2CH16_W<'_, ADC_AWD2CRrs> {
        AWD2CH16_W::new(self, 16)
    }
    ///Bit 17 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch17(&mut self) -> AWD2CH17_W<'_, ADC_AWD2CRrs> {
        AWD2CH17_W::new(self, 17)
    }
    ///Bit 18 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch18(&mut self) -> AWD2CH18_W<'_, ADC_AWD2CRrs> {
        AWD2CH18_W::new(self, 18)
    }
    ///Bit 19 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch19(&mut self) -> AWD2CH19_W<'_, ADC_AWD2CRrs> {
        AWD2CH19_W::new(self, 19)
    }
    ///Bit 20 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch20(&mut self) -> AWD2CH20_W<'_, ADC_AWD2CRrs> {
        AWD2CH20_W::new(self, 20)
    }
    ///Bit 21 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch21(&mut self) -> AWD2CH21_W<'_, ADC_AWD2CRrs> {
        AWD2CH21_W::new(self, 21)
    }
    ///Bit 22 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 2 (AWD2). Note: The channels selected through ADC_AWD2CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd2ch22(&mut self) -> AWD2CH22_W<'_, ADC_AWD2CRrs> {
        AWD2CH22_W::new(self, 22)
    }
}
/**ADC analog watchdog 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`adc_awd2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_awd2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#ADC:ADC_AWD2CR)*/
pub struct ADC_AWD2CRrs;
impl crate::RegisterSpec for ADC_AWD2CRrs {
    type Ux = u32;
}
///`read()` method returns [`adc_awd2cr::R`](R) reader structure
impl crate::Readable for ADC_AWD2CRrs {}
///`write(|w| ..)` method takes [`adc_awd2cr::W`](W) writer structure
impl crate::Writable for ADC_AWD2CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADC_AWD2CR to value 0
impl crate::Resettable for ADC_AWD2CRrs {}
