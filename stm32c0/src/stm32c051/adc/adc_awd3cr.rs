///Register `ADC_AWD3CR` reader
pub type R = crate::R<ADC_AWD3CRrs>;
///Register `ADC_AWD3CR` writer
pub type W = crate::W<ADC_AWD3CRrs>;
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH0 {
    ///0: ADC analog channel-x is not monitored by AWD3
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    B0x1 = 1,
}
impl From<AWD3CH0> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH0) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH0` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH0_R = crate::BitReader<AWD3CH0>;
impl AWD3CH0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH0 {
        match self.bits {
            false => AWD3CH0::B0x0,
            true => AWD3CH0::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3CH0::B0x0
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3CH0::B0x1
    }
}
///Field `AWD3CH0` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH0_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH0>;
impl<'a, REG> AWD3CH0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH0::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH0::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH1 {
    ///0: ADC analog channel-x is not monitored by AWD3
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    B0x1 = 1,
}
impl From<AWD3CH1> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH1) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH1` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH1_R = crate::BitReader<AWD3CH1>;
impl AWD3CH1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH1 {
        match self.bits {
            false => AWD3CH1::B0x0,
            true => AWD3CH1::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3CH1::B0x0
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3CH1::B0x1
    }
}
///Field `AWD3CH1` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH1_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH1>;
impl<'a, REG> AWD3CH1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH1::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH1::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH2 {
    ///0: ADC analog channel-x is not monitored by AWD3
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    B0x1 = 1,
}
impl From<AWD3CH2> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH2) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH2` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH2_R = crate::BitReader<AWD3CH2>;
impl AWD3CH2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH2 {
        match self.bits {
            false => AWD3CH2::B0x0,
            true => AWD3CH2::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3CH2::B0x0
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3CH2::B0x1
    }
}
///Field `AWD3CH2` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH2_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH2>;
impl<'a, REG> AWD3CH2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH2::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH2::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH3 {
    ///0: ADC analog channel-x is not monitored by AWD3
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    B0x1 = 1,
}
impl From<AWD3CH3> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH3) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH3` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH3_R = crate::BitReader<AWD3CH3>;
impl AWD3CH3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH3 {
        match self.bits {
            false => AWD3CH3::B0x0,
            true => AWD3CH3::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3CH3::B0x0
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3CH3::B0x1
    }
}
///Field `AWD3CH3` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH3_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH3>;
impl<'a, REG> AWD3CH3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH3::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH3::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH4 {
    ///0: ADC analog channel-x is not monitored by AWD3
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    B0x1 = 1,
}
impl From<AWD3CH4> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH4) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH4` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH4_R = crate::BitReader<AWD3CH4>;
impl AWD3CH4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH4 {
        match self.bits {
            false => AWD3CH4::B0x0,
            true => AWD3CH4::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3CH4::B0x0
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3CH4::B0x1
    }
}
///Field `AWD3CH4` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH4_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH4>;
impl<'a, REG> AWD3CH4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH4::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH4::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH5 {
    ///0: ADC analog channel-x is not monitored by AWD3
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    B0x1 = 1,
}
impl From<AWD3CH5> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH5) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH5` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH5_R = crate::BitReader<AWD3CH5>;
impl AWD3CH5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH5 {
        match self.bits {
            false => AWD3CH5::B0x0,
            true => AWD3CH5::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3CH5::B0x0
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3CH5::B0x1
    }
}
///Field `AWD3CH5` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH5_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH5>;
impl<'a, REG> AWD3CH5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH5::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH5::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH6 {
    ///0: ADC analog channel-x is not monitored by AWD3
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    B0x1 = 1,
}
impl From<AWD3CH6> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH6) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH6` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH6_R = crate::BitReader<AWD3CH6>;
impl AWD3CH6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH6 {
        match self.bits {
            false => AWD3CH6::B0x0,
            true => AWD3CH6::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3CH6::B0x0
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3CH6::B0x1
    }
}
///Field `AWD3CH6` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH6_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH6>;
impl<'a, REG> AWD3CH6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH6::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH6::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH7 {
    ///0: ADC analog channel-x is not monitored by AWD3
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    B0x1 = 1,
}
impl From<AWD3CH7> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH7) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH7` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH7_R = crate::BitReader<AWD3CH7>;
impl AWD3CH7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH7 {
        match self.bits {
            false => AWD3CH7::B0x0,
            true => AWD3CH7::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3CH7::B0x0
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3CH7::B0x1
    }
}
///Field `AWD3CH7` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH7_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH7>;
impl<'a, REG> AWD3CH7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH7::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH7::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH8 {
    ///0: ADC analog channel-x is not monitored by AWD3
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    B0x1 = 1,
}
impl From<AWD3CH8> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH8) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH8` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH8_R = crate::BitReader<AWD3CH8>;
impl AWD3CH8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH8 {
        match self.bits {
            false => AWD3CH8::B0x0,
            true => AWD3CH8::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3CH8::B0x0
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3CH8::B0x1
    }
}
///Field `AWD3CH8` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH8_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH8>;
impl<'a, REG> AWD3CH8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH8::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH8::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH9 {
    ///0: ADC analog channel-x is not monitored by AWD3
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    B0x1 = 1,
}
impl From<AWD3CH9> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH9) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH9` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH9_R = crate::BitReader<AWD3CH9>;
impl AWD3CH9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH9 {
        match self.bits {
            false => AWD3CH9::B0x0,
            true => AWD3CH9::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3CH9::B0x0
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3CH9::B0x1
    }
}
///Field `AWD3CH9` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH9_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH9>;
impl<'a, REG> AWD3CH9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH9::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH9::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH10 {
    ///0: ADC analog channel-x is not monitored by AWD3
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    B0x1 = 1,
}
impl From<AWD3CH10> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH10) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH10` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH10_R = crate::BitReader<AWD3CH10>;
impl AWD3CH10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH10 {
        match self.bits {
            false => AWD3CH10::B0x0,
            true => AWD3CH10::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3CH10::B0x0
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3CH10::B0x1
    }
}
///Field `AWD3CH10` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH10_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH10>;
impl<'a, REG> AWD3CH10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH10::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH10::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH11 {
    ///0: ADC analog channel-x is not monitored by AWD3
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    B0x1 = 1,
}
impl From<AWD3CH11> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH11) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH11` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH11_R = crate::BitReader<AWD3CH11>;
impl AWD3CH11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH11 {
        match self.bits {
            false => AWD3CH11::B0x0,
            true => AWD3CH11::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3CH11::B0x0
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3CH11::B0x1
    }
}
///Field `AWD3CH11` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH11_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH11>;
impl<'a, REG> AWD3CH11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH11::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH11::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH12 {
    ///0: ADC analog channel-x is not monitored by AWD3
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    B0x1 = 1,
}
impl From<AWD3CH12> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH12) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH12` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH12_R = crate::BitReader<AWD3CH12>;
impl AWD3CH12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH12 {
        match self.bits {
            false => AWD3CH12::B0x0,
            true => AWD3CH12::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3CH12::B0x0
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3CH12::B0x1
    }
}
///Field `AWD3CH12` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH12_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH12>;
impl<'a, REG> AWD3CH12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH12::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH12::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH13 {
    ///0: ADC analog channel-x is not monitored by AWD3
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    B0x1 = 1,
}
impl From<AWD3CH13> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH13) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH13` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH13_R = crate::BitReader<AWD3CH13>;
impl AWD3CH13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH13 {
        match self.bits {
            false => AWD3CH13::B0x0,
            true => AWD3CH13::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3CH13::B0x0
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3CH13::B0x1
    }
}
///Field `AWD3CH13` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH13_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH13>;
impl<'a, REG> AWD3CH13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH13::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH13::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH14 {
    ///0: ADC analog channel-x is not monitored by AWD3
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    B0x1 = 1,
}
impl From<AWD3CH14> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH14) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH14` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH14_R = crate::BitReader<AWD3CH14>;
impl AWD3CH14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH14 {
        match self.bits {
            false => AWD3CH14::B0x0,
            true => AWD3CH14::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3CH14::B0x0
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3CH14::B0x1
    }
}
///Field `AWD3CH14` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH14_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH14>;
impl<'a, REG> AWD3CH14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH14::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH14::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH15 {
    ///0: ADC analog channel-x is not monitored by AWD3
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    B0x1 = 1,
}
impl From<AWD3CH15> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH15) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH15` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH15_R = crate::BitReader<AWD3CH15>;
impl AWD3CH15_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH15 {
        match self.bits {
            false => AWD3CH15::B0x0,
            true => AWD3CH15::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3CH15::B0x0
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3CH15::B0x1
    }
}
///Field `AWD3CH15` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH15_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH15>;
impl<'a, REG> AWD3CH15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH15::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH15::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH16 {
    ///0: ADC analog channel-x is not monitored by AWD3
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    B0x1 = 1,
}
impl From<AWD3CH16> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH16) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH16` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH16_R = crate::BitReader<AWD3CH16>;
impl AWD3CH16_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH16 {
        match self.bits {
            false => AWD3CH16::B0x0,
            true => AWD3CH16::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3CH16::B0x0
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3CH16::B0x1
    }
}
///Field `AWD3CH16` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH16_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH16>;
impl<'a, REG> AWD3CH16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH16::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH16::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH17 {
    ///0: ADC analog channel-x is not monitored by AWD3
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    B0x1 = 1,
}
impl From<AWD3CH17> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH17) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH17` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH17_R = crate::BitReader<AWD3CH17>;
impl AWD3CH17_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH17 {
        match self.bits {
            false => AWD3CH17::B0x0,
            true => AWD3CH17::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3CH17::B0x0
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3CH17::B0x1
    }
}
///Field `AWD3CH17` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH17_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH17>;
impl<'a, REG> AWD3CH17_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH17::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH17::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH18 {
    ///0: ADC analog channel-x is not monitored by AWD3
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    B0x1 = 1,
}
impl From<AWD3CH18> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH18) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH18` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH18_R = crate::BitReader<AWD3CH18>;
impl AWD3CH18_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH18 {
        match self.bits {
            false => AWD3CH18::B0x0,
            true => AWD3CH18::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3CH18::B0x0
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3CH18::B0x1
    }
}
///Field `AWD3CH18` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH18_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH18>;
impl<'a, REG> AWD3CH18_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH18::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH18::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH19 {
    ///0: ADC analog channel-x is not monitored by AWD3
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    B0x1 = 1,
}
impl From<AWD3CH19> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH19) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH19` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH19_R = crate::BitReader<AWD3CH19>;
impl AWD3CH19_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH19 {
        match self.bits {
            false => AWD3CH19::B0x0,
            true => AWD3CH19::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3CH19::B0x0
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3CH19::B0x1
    }
}
///Field `AWD3CH19` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH19_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH19>;
impl<'a, REG> AWD3CH19_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH19::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH19::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH20 {
    ///0: ADC analog channel-x is not monitored by AWD3
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    B0x1 = 1,
}
impl From<AWD3CH20> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH20) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH20` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH20_R = crate::BitReader<AWD3CH20>;
impl AWD3CH20_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH20 {
        match self.bits {
            false => AWD3CH20::B0x0,
            true => AWD3CH20::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3CH20::B0x0
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3CH20::B0x1
    }
}
///Field `AWD3CH20` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH20_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH20>;
impl<'a, REG> AWD3CH20_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH20::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH20::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH21 {
    ///0: ADC analog channel-x is not monitored by AWD3
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    B0x1 = 1,
}
impl From<AWD3CH21> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH21) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH21` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH21_R = crate::BitReader<AWD3CH21>;
impl AWD3CH21_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH21 {
        match self.bits {
            false => AWD3CH21::B0x0,
            true => AWD3CH21::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3CH21::B0x0
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3CH21::B0x1
    }
}
///Field `AWD3CH21` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH21_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH21>;
impl<'a, REG> AWD3CH21_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH21::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH21::B0x1)
    }
}
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH22 {
    ///0: ADC analog channel-x is not monitored by AWD3
    B0x0 = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    B0x1 = 1,
}
impl From<AWD3CH22> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH22) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH22` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH22_R = crate::BitReader<AWD3CH22>;
impl AWD3CH22_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH22 {
        match self.bits {
            false => AWD3CH22::B0x0,
            true => AWD3CH22::B0x1,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3CH22::B0x0
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3CH22::B0x1
    }
}
///Field `AWD3CH22` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type AWD3CH22_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH22>;
impl<'a, REG> AWD3CH22_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH22::B0x0)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH22::B0x1)
    }
}
impl R {
    ///Bit 0 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch0(&self) -> AWD3CH0_R {
        AWD3CH0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch1(&self) -> AWD3CH1_R {
        AWD3CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch2(&self) -> AWD3CH2_R {
        AWD3CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch3(&self) -> AWD3CH3_R {
        AWD3CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch4(&self) -> AWD3CH4_R {
        AWD3CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch5(&self) -> AWD3CH5_R {
        AWD3CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch6(&self) -> AWD3CH6_R {
        AWD3CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch7(&self) -> AWD3CH7_R {
        AWD3CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch8(&self) -> AWD3CH8_R {
        AWD3CH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch9(&self) -> AWD3CH9_R {
        AWD3CH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch10(&self) -> AWD3CH10_R {
        AWD3CH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch11(&self) -> AWD3CH11_R {
        AWD3CH11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch12(&self) -> AWD3CH12_R {
        AWD3CH12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch13(&self) -> AWD3CH13_R {
        AWD3CH13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch14(&self) -> AWD3CH14_R {
        AWD3CH14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch15(&self) -> AWD3CH15_R {
        AWD3CH15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch16(&self) -> AWD3CH16_R {
        AWD3CH16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch17(&self) -> AWD3CH17_R {
        AWD3CH17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch18(&self) -> AWD3CH18_R {
        AWD3CH18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch19(&self) -> AWD3CH19_R {
        AWD3CH19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch20(&self) -> AWD3CH20_R {
        AWD3CH20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch21(&self) -> AWD3CH21_R {
        AWD3CH21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch22(&self) -> AWD3CH22_R {
        AWD3CH22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_AWD3CR")
            .field("awd3ch0", &self.awd3ch0())
            .field("awd3ch1", &self.awd3ch1())
            .field("awd3ch2", &self.awd3ch2())
            .field("awd3ch3", &self.awd3ch3())
            .field("awd3ch4", &self.awd3ch4())
            .field("awd3ch5", &self.awd3ch5())
            .field("awd3ch6", &self.awd3ch6())
            .field("awd3ch7", &self.awd3ch7())
            .field("awd3ch8", &self.awd3ch8())
            .field("awd3ch9", &self.awd3ch9())
            .field("awd3ch10", &self.awd3ch10())
            .field("awd3ch11", &self.awd3ch11())
            .field("awd3ch12", &self.awd3ch12())
            .field("awd3ch13", &self.awd3ch13())
            .field("awd3ch14", &self.awd3ch14())
            .field("awd3ch15", &self.awd3ch15())
            .field("awd3ch16", &self.awd3ch16())
            .field("awd3ch17", &self.awd3ch17())
            .field("awd3ch18", &self.awd3ch18())
            .field("awd3ch19", &self.awd3ch19())
            .field("awd3ch20", &self.awd3ch20())
            .field("awd3ch21", &self.awd3ch21())
            .field("awd3ch22", &self.awd3ch22())
            .finish()
    }
}
impl W {
    ///Bit 0 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch0(&mut self) -> AWD3CH0_W<'_, ADC_AWD3CRrs> {
        AWD3CH0_W::new(self, 0)
    }
    ///Bit 1 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch1(&mut self) -> AWD3CH1_W<'_, ADC_AWD3CRrs> {
        AWD3CH1_W::new(self, 1)
    }
    ///Bit 2 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch2(&mut self) -> AWD3CH2_W<'_, ADC_AWD3CRrs> {
        AWD3CH2_W::new(self, 2)
    }
    ///Bit 3 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch3(&mut self) -> AWD3CH3_W<'_, ADC_AWD3CRrs> {
        AWD3CH3_W::new(self, 3)
    }
    ///Bit 4 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch4(&mut self) -> AWD3CH4_W<'_, ADC_AWD3CRrs> {
        AWD3CH4_W::new(self, 4)
    }
    ///Bit 5 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch5(&mut self) -> AWD3CH5_W<'_, ADC_AWD3CRrs> {
        AWD3CH5_W::new(self, 5)
    }
    ///Bit 6 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch6(&mut self) -> AWD3CH6_W<'_, ADC_AWD3CRrs> {
        AWD3CH6_W::new(self, 6)
    }
    ///Bit 7 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch7(&mut self) -> AWD3CH7_W<'_, ADC_AWD3CRrs> {
        AWD3CH7_W::new(self, 7)
    }
    ///Bit 8 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch8(&mut self) -> AWD3CH8_W<'_, ADC_AWD3CRrs> {
        AWD3CH8_W::new(self, 8)
    }
    ///Bit 9 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch9(&mut self) -> AWD3CH9_W<'_, ADC_AWD3CRrs> {
        AWD3CH9_W::new(self, 9)
    }
    ///Bit 10 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch10(&mut self) -> AWD3CH10_W<'_, ADC_AWD3CRrs> {
        AWD3CH10_W::new(self, 10)
    }
    ///Bit 11 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch11(&mut self) -> AWD3CH11_W<'_, ADC_AWD3CRrs> {
        AWD3CH11_W::new(self, 11)
    }
    ///Bit 12 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch12(&mut self) -> AWD3CH12_W<'_, ADC_AWD3CRrs> {
        AWD3CH12_W::new(self, 12)
    }
    ///Bit 13 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch13(&mut self) -> AWD3CH13_W<'_, ADC_AWD3CRrs> {
        AWD3CH13_W::new(self, 13)
    }
    ///Bit 14 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch14(&mut self) -> AWD3CH14_W<'_, ADC_AWD3CRrs> {
        AWD3CH14_W::new(self, 14)
    }
    ///Bit 15 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch15(&mut self) -> AWD3CH15_W<'_, ADC_AWD3CRrs> {
        AWD3CH15_W::new(self, 15)
    }
    ///Bit 16 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch16(&mut self) -> AWD3CH16_W<'_, ADC_AWD3CRrs> {
        AWD3CH16_W::new(self, 16)
    }
    ///Bit 17 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch17(&mut self) -> AWD3CH17_W<'_, ADC_AWD3CRrs> {
        AWD3CH17_W::new(self, 17)
    }
    ///Bit 18 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch18(&mut self) -> AWD3CH18_W<'_, ADC_AWD3CRrs> {
        AWD3CH18_W::new(self, 18)
    }
    ///Bit 19 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch19(&mut self) -> AWD3CH19_W<'_, ADC_AWD3CRrs> {
        AWD3CH19_W::new(self, 19)
    }
    ///Bit 20 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch20(&mut self) -> AWD3CH20_W<'_, ADC_AWD3CRrs> {
        AWD3CH20_W::new(self, 20)
    }
    ///Bit 21 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch21(&mut self) -> AWD3CH21_W<'_, ADC_AWD3CRrs> {
        AWD3CH21_W::new(self, 21)
    }
    ///Bit 22 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awd3ch22(&mut self) -> AWD3CH22_W<'_, ADC_AWD3CRrs> {
        AWD3CH22_W::new(self, 22)
    }
}
/**ADC Analog Watchdog 3 Configuration register

You can [`read`](crate::Reg::read) this register and get [`adc_awd3cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_awd3cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#ADC:ADC_AWD3CR)*/
pub struct ADC_AWD3CRrs;
impl crate::RegisterSpec for ADC_AWD3CRrs {
    type Ux = u32;
}
///`read()` method returns [`adc_awd3cr::R`](R) reader structure
impl crate::Readable for ADC_AWD3CRrs {}
///`write(|w| ..)` method takes [`adc_awd3cr::W`](W) writer structure
impl crate::Writable for ADC_AWD3CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADC_AWD3CR to value 0
impl crate::Resettable for ADC_AWD3CRrs {}
