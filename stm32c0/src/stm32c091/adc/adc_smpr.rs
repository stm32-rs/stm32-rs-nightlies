///Register `ADC_SMPR` reader
pub type R = crate::R<ADC_SMPRrs>;
///Register `ADC_SMPR` writer
pub type W = crate::W<ADC_SMPRrs>;
/**Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP1 {
    ///0: 1.5 ADC clock cycles
    B0x0 = 0,
    ///1: 3.5 ADC clock cycles
    B0x1 = 1,
    ///2: 7.5 ADC clock cycles
    B0x2 = 2,
    ///3: 12.5 ADC clock cycles
    B0x3 = 3,
    ///4: 19.5 ADC clock cycles
    B0x4 = 4,
    ///5: 39.5 ADC clock cycles
    B0x5 = 5,
    ///6: 79.5 ADC clock cycles
    B0x6 = 6,
    ///7: 160.5 ADC clock cycles
    B0x7 = 7,
}
impl From<SMP1> for u8 {
    #[inline(always)]
    fn from(variant: SMP1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP1 {
    type Ux = u8;
}
impl crate::IsEnum for SMP1 {}
///Field `SMP1` reader - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SMP1_R = crate::FieldReader<SMP1>;
impl SMP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMP1 {
        match self.bits {
            0 => SMP1::B0x0,
            1 => SMP1::B0x1,
            2 => SMP1::B0x2,
            3 => SMP1::B0x3,
            4 => SMP1::B0x4,
            5 => SMP1::B0x5,
            6 => SMP1::B0x6,
            7 => SMP1::B0x7,
            _ => unreachable!(),
        }
    }
    ///1.5 ADC clock cycles
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMP1::B0x0
    }
    ///3.5 ADC clock cycles
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMP1::B0x1
    }
    ///7.5 ADC clock cycles
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SMP1::B0x2
    }
    ///12.5 ADC clock cycles
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SMP1::B0x3
    }
    ///19.5 ADC clock cycles
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == SMP1::B0x4
    }
    ///39.5 ADC clock cycles
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == SMP1::B0x5
    }
    ///79.5 ADC clock cycles
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == SMP1::B0x6
    }
    ///160.5 ADC clock cycles
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == SMP1::B0x7
    }
}
///Field `SMP1` writer - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SMP1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP1, crate::Safe>;
impl<'a, REG> SMP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1.5 ADC clock cycles
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::B0x0)
    }
    ///3.5 ADC clock cycles
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::B0x1)
    }
    ///7.5 ADC clock cycles
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::B0x2)
    }
    ///12.5 ADC clock cycles
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::B0x3)
    }
    ///19.5 ADC clock cycles
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::B0x4)
    }
    ///39.5 ADC clock cycles
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::B0x5)
    }
    ///79.5 ADC clock cycles
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::B0x6)
    }
    ///160.5 ADC clock cycles
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::B0x7)
    }
}
/**Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP2 {
    ///0: 1.5 ADC clock cycles
    B0x0 = 0,
    ///1: 3.5 ADC clock cycles
    B0x1 = 1,
    ///2: 7.5 ADC clock cycles
    B0x2 = 2,
    ///3: 12.5 ADC clock cycles
    B0x3 = 3,
    ///4: 19.5 ADC clock cycles
    B0x4 = 4,
    ///5: 39.5 ADC clock cycles
    B0x5 = 5,
    ///6: 79.5 ADC clock cycles
    B0x6 = 6,
    ///7: 160.5 ADC clock cycles
    B0x7 = 7,
}
impl From<SMP2> for u8 {
    #[inline(always)]
    fn from(variant: SMP2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP2 {
    type Ux = u8;
}
impl crate::IsEnum for SMP2 {}
///Field `SMP2` reader - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SMP2_R = crate::FieldReader<SMP2>;
impl SMP2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMP2 {
        match self.bits {
            0 => SMP2::B0x0,
            1 => SMP2::B0x1,
            2 => SMP2::B0x2,
            3 => SMP2::B0x3,
            4 => SMP2::B0x4,
            5 => SMP2::B0x5,
            6 => SMP2::B0x6,
            7 => SMP2::B0x7,
            _ => unreachable!(),
        }
    }
    ///1.5 ADC clock cycles
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMP2::B0x0
    }
    ///3.5 ADC clock cycles
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMP2::B0x1
    }
    ///7.5 ADC clock cycles
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SMP2::B0x2
    }
    ///12.5 ADC clock cycles
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SMP2::B0x3
    }
    ///19.5 ADC clock cycles
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == SMP2::B0x4
    }
    ///39.5 ADC clock cycles
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == SMP2::B0x5
    }
    ///79.5 ADC clock cycles
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == SMP2::B0x6
    }
    ///160.5 ADC clock cycles
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == SMP2::B0x7
    }
}
///Field `SMP2` writer - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SMP2_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP2, crate::Safe>;
impl<'a, REG> SMP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1.5 ADC clock cycles
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMP2::B0x0)
    }
    ///3.5 ADC clock cycles
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMP2::B0x1)
    }
    ///7.5 ADC clock cycles
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMP2::B0x2)
    }
    ///12.5 ADC clock cycles
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP2::B0x3)
    }
    ///19.5 ADC clock cycles
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMP2::B0x4)
    }
    ///39.5 ADC clock cycles
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP2::B0x5)
    }
    ///79.5 ADC clock cycles
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP2::B0x6)
    }
    ///160.5 ADC clock cycles
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMP2::B0x7)
    }
}
/**Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL0 {
    ///0: Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    B0x0 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    B0x1 = 1,
}
impl From<SMPSEL0> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL0) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL0` reader - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL0_R = crate::BitReader<SMPSEL0>;
impl SMPSEL0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL0 {
        match self.bits {
            false => SMPSEL0::B0x0,
            true => SMPSEL0::B0x1,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL0::B0x0
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL0::B0x1
    }
}
///Field `SMPSEL0` writer - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL0_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL0>;
impl<'a, REG> SMPSEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL0::B0x0)
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL0::B0x1)
    }
}
/**Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL1 {
    ///0: Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    B0x0 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    B0x1 = 1,
}
impl From<SMPSEL1> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL1) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL1` reader - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL1_R = crate::BitReader<SMPSEL1>;
impl SMPSEL1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL1 {
        match self.bits {
            false => SMPSEL1::B0x0,
            true => SMPSEL1::B0x1,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL1::B0x0
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL1::B0x1
    }
}
///Field `SMPSEL1` writer - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL1_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL1>;
impl<'a, REG> SMPSEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL1::B0x0)
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL1::B0x1)
    }
}
/**Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL2 {
    ///0: Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    B0x0 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    B0x1 = 1,
}
impl From<SMPSEL2> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL2) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL2` reader - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL2_R = crate::BitReader<SMPSEL2>;
impl SMPSEL2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL2 {
        match self.bits {
            false => SMPSEL2::B0x0,
            true => SMPSEL2::B0x1,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL2::B0x0
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL2::B0x1
    }
}
///Field `SMPSEL2` writer - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL2_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL2>;
impl<'a, REG> SMPSEL2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL2::B0x0)
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL2::B0x1)
    }
}
/**Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL3 {
    ///0: Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    B0x0 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    B0x1 = 1,
}
impl From<SMPSEL3> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL3) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL3` reader - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL3_R = crate::BitReader<SMPSEL3>;
impl SMPSEL3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL3 {
        match self.bits {
            false => SMPSEL3::B0x0,
            true => SMPSEL3::B0x1,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL3::B0x0
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL3::B0x1
    }
}
///Field `SMPSEL3` writer - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL3_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL3>;
impl<'a, REG> SMPSEL3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL3::B0x0)
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL3::B0x1)
    }
}
/**Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL4 {
    ///0: Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    B0x0 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    B0x1 = 1,
}
impl From<SMPSEL4> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL4) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL4` reader - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL4_R = crate::BitReader<SMPSEL4>;
impl SMPSEL4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL4 {
        match self.bits {
            false => SMPSEL4::B0x0,
            true => SMPSEL4::B0x1,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL4::B0x0
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL4::B0x1
    }
}
///Field `SMPSEL4` writer - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL4_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL4>;
impl<'a, REG> SMPSEL4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL4::B0x0)
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL4::B0x1)
    }
}
/**Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL5 {
    ///0: Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    B0x0 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    B0x1 = 1,
}
impl From<SMPSEL5> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL5) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL5` reader - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL5_R = crate::BitReader<SMPSEL5>;
impl SMPSEL5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL5 {
        match self.bits {
            false => SMPSEL5::B0x0,
            true => SMPSEL5::B0x1,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL5::B0x0
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL5::B0x1
    }
}
///Field `SMPSEL5` writer - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL5_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL5>;
impl<'a, REG> SMPSEL5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL5::B0x0)
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL5::B0x1)
    }
}
/**Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL6 {
    ///0: Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    B0x0 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    B0x1 = 1,
}
impl From<SMPSEL6> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL6) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL6` reader - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL6_R = crate::BitReader<SMPSEL6>;
impl SMPSEL6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL6 {
        match self.bits {
            false => SMPSEL6::B0x0,
            true => SMPSEL6::B0x1,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL6::B0x0
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL6::B0x1
    }
}
///Field `SMPSEL6` writer - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL6_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL6>;
impl<'a, REG> SMPSEL6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL6::B0x0)
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL6::B0x1)
    }
}
/**Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL7 {
    ///0: Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    B0x0 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    B0x1 = 1,
}
impl From<SMPSEL7> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL7) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL7` reader - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL7_R = crate::BitReader<SMPSEL7>;
impl SMPSEL7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL7 {
        match self.bits {
            false => SMPSEL7::B0x0,
            true => SMPSEL7::B0x1,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL7::B0x0
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL7::B0x1
    }
}
///Field `SMPSEL7` writer - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL7_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL7>;
impl<'a, REG> SMPSEL7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL7::B0x0)
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL7::B0x1)
    }
}
/**Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL8 {
    ///0: Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    B0x0 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    B0x1 = 1,
}
impl From<SMPSEL8> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL8) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL8` reader - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL8_R = crate::BitReader<SMPSEL8>;
impl SMPSEL8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL8 {
        match self.bits {
            false => SMPSEL8::B0x0,
            true => SMPSEL8::B0x1,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL8::B0x0
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL8::B0x1
    }
}
///Field `SMPSEL8` writer - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL8_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL8>;
impl<'a, REG> SMPSEL8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL8::B0x0)
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL8::B0x1)
    }
}
/**Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL9 {
    ///0: Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    B0x0 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    B0x1 = 1,
}
impl From<SMPSEL9> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL9) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL9` reader - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL9_R = crate::BitReader<SMPSEL9>;
impl SMPSEL9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL9 {
        match self.bits {
            false => SMPSEL9::B0x0,
            true => SMPSEL9::B0x1,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL9::B0x0
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL9::B0x1
    }
}
///Field `SMPSEL9` writer - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL9_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL9>;
impl<'a, REG> SMPSEL9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL9::B0x0)
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL9::B0x1)
    }
}
/**Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL10 {
    ///0: Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    B0x0 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    B0x1 = 1,
}
impl From<SMPSEL10> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL10) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL10` reader - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL10_R = crate::BitReader<SMPSEL10>;
impl SMPSEL10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL10 {
        match self.bits {
            false => SMPSEL10::B0x0,
            true => SMPSEL10::B0x1,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL10::B0x0
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL10::B0x1
    }
}
///Field `SMPSEL10` writer - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL10_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL10>;
impl<'a, REG> SMPSEL10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL10::B0x0)
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL10::B0x1)
    }
}
/**Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL11 {
    ///0: Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    B0x0 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    B0x1 = 1,
}
impl From<SMPSEL11> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL11) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL11` reader - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL11_R = crate::BitReader<SMPSEL11>;
impl SMPSEL11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL11 {
        match self.bits {
            false => SMPSEL11::B0x0,
            true => SMPSEL11::B0x1,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL11::B0x0
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL11::B0x1
    }
}
///Field `SMPSEL11` writer - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL11_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL11>;
impl<'a, REG> SMPSEL11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL11::B0x0)
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL11::B0x1)
    }
}
/**Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL12 {
    ///0: Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    B0x0 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    B0x1 = 1,
}
impl From<SMPSEL12> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL12) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL12` reader - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL12_R = crate::BitReader<SMPSEL12>;
impl SMPSEL12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL12 {
        match self.bits {
            false => SMPSEL12::B0x0,
            true => SMPSEL12::B0x1,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL12::B0x0
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL12::B0x1
    }
}
///Field `SMPSEL12` writer - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL12_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL12>;
impl<'a, REG> SMPSEL12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL12::B0x0)
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL12::B0x1)
    }
}
/**Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL13 {
    ///0: Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    B0x0 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    B0x1 = 1,
}
impl From<SMPSEL13> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL13) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL13` reader - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL13_R = crate::BitReader<SMPSEL13>;
impl SMPSEL13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL13 {
        match self.bits {
            false => SMPSEL13::B0x0,
            true => SMPSEL13::B0x1,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL13::B0x0
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL13::B0x1
    }
}
///Field `SMPSEL13` writer - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL13_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL13>;
impl<'a, REG> SMPSEL13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL13::B0x0)
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL13::B0x1)
    }
}
/**Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL14 {
    ///0: Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    B0x0 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    B0x1 = 1,
}
impl From<SMPSEL14> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL14) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL14` reader - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL14_R = crate::BitReader<SMPSEL14>;
impl SMPSEL14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL14 {
        match self.bits {
            false => SMPSEL14::B0x0,
            true => SMPSEL14::B0x1,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL14::B0x0
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL14::B0x1
    }
}
///Field `SMPSEL14` writer - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL14_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL14>;
impl<'a, REG> SMPSEL14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL14::B0x0)
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL14::B0x1)
    }
}
/**Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL15 {
    ///0: Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    B0x0 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    B0x1 = 1,
}
impl From<SMPSEL15> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL15) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL15` reader - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL15_R = crate::BitReader<SMPSEL15>;
impl SMPSEL15_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL15 {
        match self.bits {
            false => SMPSEL15::B0x0,
            true => SMPSEL15::B0x1,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL15::B0x0
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL15::B0x1
    }
}
///Field `SMPSEL15` writer - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL15_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL15>;
impl<'a, REG> SMPSEL15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL15::B0x0)
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL15::B0x1)
    }
}
/**Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL16 {
    ///0: Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    B0x0 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    B0x1 = 1,
}
impl From<SMPSEL16> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL16) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL16` reader - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL16_R = crate::BitReader<SMPSEL16>;
impl SMPSEL16_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL16 {
        match self.bits {
            false => SMPSEL16::B0x0,
            true => SMPSEL16::B0x1,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL16::B0x0
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL16::B0x1
    }
}
///Field `SMPSEL16` writer - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL16_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL16>;
impl<'a, REG> SMPSEL16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL16::B0x0)
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL16::B0x1)
    }
}
/**Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL17 {
    ///0: Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    B0x0 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    B0x1 = 1,
}
impl From<SMPSEL17> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL17) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL17` reader - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL17_R = crate::BitReader<SMPSEL17>;
impl SMPSEL17_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL17 {
        match self.bits {
            false => SMPSEL17::B0x0,
            true => SMPSEL17::B0x1,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL17::B0x0
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL17::B0x1
    }
}
///Field `SMPSEL17` writer - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL17_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL17>;
impl<'a, REG> SMPSEL17_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL17::B0x0)
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL17::B0x1)
    }
}
/**Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL18 {
    ///0: Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    B0x0 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    B0x1 = 1,
}
impl From<SMPSEL18> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL18) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL18` reader - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL18_R = crate::BitReader<SMPSEL18>;
impl SMPSEL18_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL18 {
        match self.bits {
            false => SMPSEL18::B0x0,
            true => SMPSEL18::B0x1,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL18::B0x0
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL18::B0x1
    }
}
///Field `SMPSEL18` writer - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL18_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL18>;
impl<'a, REG> SMPSEL18_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL18::B0x0)
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL18::B0x1)
    }
}
/**Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL19 {
    ///0: Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    B0x0 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    B0x1 = 1,
}
impl From<SMPSEL19> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL19) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL19` reader - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL19_R = crate::BitReader<SMPSEL19>;
impl SMPSEL19_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL19 {
        match self.bits {
            false => SMPSEL19::B0x0,
            true => SMPSEL19::B0x1,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL19::B0x0
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL19::B0x1
    }
}
///Field `SMPSEL19` writer - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL19_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL19>;
impl<'a, REG> SMPSEL19_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL19::B0x0)
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL19::B0x1)
    }
}
/**Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL20 {
    ///0: Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    B0x0 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    B0x1 = 1,
}
impl From<SMPSEL20> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL20) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL20` reader - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL20_R = crate::BitReader<SMPSEL20>;
impl SMPSEL20_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL20 {
        match self.bits {
            false => SMPSEL20::B0x0,
            true => SMPSEL20::B0x1,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL20::B0x0
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL20::B0x1
    }
}
///Field `SMPSEL20` writer - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL20_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL20>;
impl<'a, REG> SMPSEL20_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL20::B0x0)
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL20::B0x1)
    }
}
/**Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL21 {
    ///0: Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    B0x0 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    B0x1 = 1,
}
impl From<SMPSEL21> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL21) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL21` reader - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL21_R = crate::BitReader<SMPSEL21>;
impl SMPSEL21_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL21 {
        match self.bits {
            false => SMPSEL21::B0x0,
            true => SMPSEL21::B0x1,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL21::B0x0
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL21::B0x1
    }
}
///Field `SMPSEL21` writer - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL21_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL21>;
impl<'a, REG> SMPSEL21_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL21::B0x0)
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL21::B0x1)
    }
}
/**Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL22 {
    ///0: Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    B0x0 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    B0x1 = 1,
}
impl From<SMPSEL22> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL22) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL22` reader - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL22_R = crate::BitReader<SMPSEL22>;
impl SMPSEL22_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL22 {
        match self.bits {
            false => SMPSEL22::B0x0,
            true => SMPSEL22::B0x1,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL22::B0x0
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL22::B0x1
    }
}
///Field `SMPSEL22` writer - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
pub type SMPSEL22_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL22>;
impl<'a, REG> SMPSEL22_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of CHANNELx use the setting of SMP1\[2:0\] register.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL22::B0x0)
    }
    ///Sampling time of CHANNELx use the setting of SMP2\[2:0\] register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL22::B0x1)
    }
}
impl R {
    ///Bits 0:2 - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 8 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel0(&self) -> SMPSEL0_R {
        SMPSEL0_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel1(&self) -> SMPSEL1_R {
        SMPSEL1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel2(&self) -> SMPSEL2_R {
        SMPSEL2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel3(&self) -> SMPSEL3_R {
        SMPSEL3_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel4(&self) -> SMPSEL4_R {
        SMPSEL4_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel5(&self) -> SMPSEL5_R {
        SMPSEL5_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel6(&self) -> SMPSEL6_R {
        SMPSEL6_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel7(&self) -> SMPSEL7_R {
        SMPSEL7_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel8(&self) -> SMPSEL8_R {
        SMPSEL8_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel9(&self) -> SMPSEL9_R {
        SMPSEL9_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel10(&self) -> SMPSEL10_R {
        SMPSEL10_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel11(&self) -> SMPSEL11_R {
        SMPSEL11_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel12(&self) -> SMPSEL12_R {
        SMPSEL12_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel13(&self) -> SMPSEL13_R {
        SMPSEL13_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel14(&self) -> SMPSEL14_R {
        SMPSEL14_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel15(&self) -> SMPSEL15_R {
        SMPSEL15_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel16(&self) -> SMPSEL16_R {
        SMPSEL16_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel17(&self) -> SMPSEL17_R {
        SMPSEL17_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel18(&self) -> SMPSEL18_R {
        SMPSEL18_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel19(&self) -> SMPSEL19_R {
        SMPSEL19_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel20(&self) -> SMPSEL20_R {
        SMPSEL20_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel21(&self) -> SMPSEL21_R {
        SMPSEL21_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel22(&self) -> SMPSEL22_R {
        SMPSEL22_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_SMPR")
            .field("smp1", &self.smp1())
            .field("smp2", &self.smp2())
            .field("smpsel0", &self.smpsel0())
            .field("smpsel1", &self.smpsel1())
            .field("smpsel2", &self.smpsel2())
            .field("smpsel3", &self.smpsel3())
            .field("smpsel4", &self.smpsel4())
            .field("smpsel5", &self.smpsel5())
            .field("smpsel6", &self.smpsel6())
            .field("smpsel7", &self.smpsel7())
            .field("smpsel8", &self.smpsel8())
            .field("smpsel9", &self.smpsel9())
            .field("smpsel10", &self.smpsel10())
            .field("smpsel11", &self.smpsel11())
            .field("smpsel12", &self.smpsel12())
            .field("smpsel13", &self.smpsel13())
            .field("smpsel14", &self.smpsel14())
            .field("smpsel15", &self.smpsel15())
            .field("smpsel16", &self.smpsel16())
            .field("smpsel17", &self.smpsel17())
            .field("smpsel18", &self.smpsel18())
            .field("smpsel19", &self.smpsel19())
            .field("smpsel20", &self.smpsel20())
            .field("smpsel21", &self.smpsel21())
            .field("smpsel22", &self.smpsel22())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smp1(&mut self) -> SMP1_W<'_, ADC_SMPRrs> {
        SMP1_W::new(self, 0)
    }
    ///Bits 4:6 - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smp2(&mut self) -> SMP2_W<'_, ADC_SMPRrs> {
        SMP2_W::new(self, 4)
    }
    ///Bit 8 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel0(&mut self) -> SMPSEL0_W<'_, ADC_SMPRrs> {
        SMPSEL0_W::new(self, 8)
    }
    ///Bit 9 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel1(&mut self) -> SMPSEL1_W<'_, ADC_SMPRrs> {
        SMPSEL1_W::new(self, 9)
    }
    ///Bit 10 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel2(&mut self) -> SMPSEL2_W<'_, ADC_SMPRrs> {
        SMPSEL2_W::new(self, 10)
    }
    ///Bit 11 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel3(&mut self) -> SMPSEL3_W<'_, ADC_SMPRrs> {
        SMPSEL3_W::new(self, 11)
    }
    ///Bit 12 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel4(&mut self) -> SMPSEL4_W<'_, ADC_SMPRrs> {
        SMPSEL4_W::new(self, 12)
    }
    ///Bit 13 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel5(&mut self) -> SMPSEL5_W<'_, ADC_SMPRrs> {
        SMPSEL5_W::new(self, 13)
    }
    ///Bit 14 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel6(&mut self) -> SMPSEL6_W<'_, ADC_SMPRrs> {
        SMPSEL6_W::new(self, 14)
    }
    ///Bit 15 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel7(&mut self) -> SMPSEL7_W<'_, ADC_SMPRrs> {
        SMPSEL7_W::new(self, 15)
    }
    ///Bit 16 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel8(&mut self) -> SMPSEL8_W<'_, ADC_SMPRrs> {
        SMPSEL8_W::new(self, 16)
    }
    ///Bit 17 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel9(&mut self) -> SMPSEL9_W<'_, ADC_SMPRrs> {
        SMPSEL9_W::new(self, 17)
    }
    ///Bit 18 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel10(&mut self) -> SMPSEL10_W<'_, ADC_SMPRrs> {
        SMPSEL10_W::new(self, 18)
    }
    ///Bit 19 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel11(&mut self) -> SMPSEL11_W<'_, ADC_SMPRrs> {
        SMPSEL11_W::new(self, 19)
    }
    ///Bit 20 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel12(&mut self) -> SMPSEL12_W<'_, ADC_SMPRrs> {
        SMPSEL12_W::new(self, 20)
    }
    ///Bit 21 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel13(&mut self) -> SMPSEL13_W<'_, ADC_SMPRrs> {
        SMPSEL13_W::new(self, 21)
    }
    ///Bit 22 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel14(&mut self) -> SMPSEL14_W<'_, ADC_SMPRrs> {
        SMPSEL14_W::new(self, 22)
    }
    ///Bit 23 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel15(&mut self) -> SMPSEL15_W<'_, ADC_SMPRrs> {
        SMPSEL15_W::new(self, 23)
    }
    ///Bit 24 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel16(&mut self) -> SMPSEL16_W<'_, ADC_SMPRrs> {
        SMPSEL16_W::new(self, 24)
    }
    ///Bit 25 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel17(&mut self) -> SMPSEL17_W<'_, ADC_SMPRrs> {
        SMPSEL17_W::new(self, 25)
    }
    ///Bit 26 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel18(&mut self) -> SMPSEL18_W<'_, ADC_SMPRrs> {
        SMPSEL18_W::new(self, 26)
    }
    ///Bit 27 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel19(&mut self) -> SMPSEL19_W<'_, ADC_SMPRrs> {
        SMPSEL19_W::new(self, 27)
    }
    ///Bit 28 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel20(&mut self) -> SMPSEL20_W<'_, ADC_SMPRrs> {
        SMPSEL20_W::new(self, 28)
    }
    ///Bit 29 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel21(&mut self) -> SMPSEL21_W<'_, ADC_SMPRrs> {
        SMPSEL21_W::new(self, 29)
    }
    ///Bit 30 - Channel-x sampling time selection (x = 22 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: Refer to Section 16.3: ADC implementation for the maximum number of channels.
    #[inline(always)]
    pub fn smpsel22(&mut self) -> SMPSEL22_W<'_, ADC_SMPRrs> {
        SMPSEL22_W::new(self, 30)
    }
}
/**ADC sampling time register

You can [`read`](crate::Reg::read) this register and get [`adc_smpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_smpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#ADC:ADC_SMPR)*/
pub struct ADC_SMPRrs;
impl crate::RegisterSpec for ADC_SMPRrs {
    type Ux = u32;
}
///`read()` method returns [`adc_smpr::R`](R) reader structure
impl crate::Readable for ADC_SMPRrs {}
///`write(|w| ..)` method takes [`adc_smpr::W`](W) writer structure
impl crate::Writable for ADC_SMPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADC_SMPR to value 0
impl crate::Resettable for ADC_SMPRrs {}
