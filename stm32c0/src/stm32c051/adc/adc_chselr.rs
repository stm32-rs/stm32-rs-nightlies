///Register `ADC_CHSELR` reader
pub type R = crate::R<ADC_CHSELRrs>;
///Register `ADC_CHSELR` writer
pub type W = crate::W<ADC_CHSELRrs>;
/**Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL0 {
    ///0: Input Channel-x is not selected for conversion
    B0x0 = 0,
    ///1: Input Channel-x is selected for conversion
    B0x1 = 1,
}
impl From<CHSEL0> for bool {
    #[inline(always)]
    fn from(variant: CHSEL0) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL0` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL0_R = crate::BitReader<CHSEL0>;
impl CHSEL0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL0 {
        match self.bits {
            false => CHSEL0::B0x0,
            true => CHSEL0::B0x1,
        }
    }
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL0::B0x0
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL0::B0x1
    }
}
///Field `CHSEL0` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL0_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL0>;
impl<'a, REG> CHSEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL0::B0x0)
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL0::B0x1)
    }
}
/**Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL1 {
    ///0: Input Channel-x is not selected for conversion
    B0x0 = 0,
    ///1: Input Channel-x is selected for conversion
    B0x1 = 1,
}
impl From<CHSEL1> for bool {
    #[inline(always)]
    fn from(variant: CHSEL1) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL1` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL1_R = crate::BitReader<CHSEL1>;
impl CHSEL1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL1 {
        match self.bits {
            false => CHSEL1::B0x0,
            true => CHSEL1::B0x1,
        }
    }
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL1::B0x0
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL1::B0x1
    }
}
///Field `CHSEL1` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL1_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL1>;
impl<'a, REG> CHSEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL1::B0x0)
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL1::B0x1)
    }
}
/**Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL2 {
    ///0: Input Channel-x is not selected for conversion
    B0x0 = 0,
    ///1: Input Channel-x is selected for conversion
    B0x1 = 1,
}
impl From<CHSEL2> for bool {
    #[inline(always)]
    fn from(variant: CHSEL2) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL2` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL2_R = crate::BitReader<CHSEL2>;
impl CHSEL2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL2 {
        match self.bits {
            false => CHSEL2::B0x0,
            true => CHSEL2::B0x1,
        }
    }
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL2::B0x0
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL2::B0x1
    }
}
///Field `CHSEL2` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL2_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL2>;
impl<'a, REG> CHSEL2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL2::B0x0)
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL2::B0x1)
    }
}
/**Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL3 {
    ///0: Input Channel-x is not selected for conversion
    B0x0 = 0,
    ///1: Input Channel-x is selected for conversion
    B0x1 = 1,
}
impl From<CHSEL3> for bool {
    #[inline(always)]
    fn from(variant: CHSEL3) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL3` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL3_R = crate::BitReader<CHSEL3>;
impl CHSEL3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL3 {
        match self.bits {
            false => CHSEL3::B0x0,
            true => CHSEL3::B0x1,
        }
    }
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL3::B0x0
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL3::B0x1
    }
}
///Field `CHSEL3` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL3_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL3>;
impl<'a, REG> CHSEL3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL3::B0x0)
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL3::B0x1)
    }
}
/**Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL4 {
    ///0: Input Channel-x is not selected for conversion
    B0x0 = 0,
    ///1: Input Channel-x is selected for conversion
    B0x1 = 1,
}
impl From<CHSEL4> for bool {
    #[inline(always)]
    fn from(variant: CHSEL4) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL4` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL4_R = crate::BitReader<CHSEL4>;
impl CHSEL4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL4 {
        match self.bits {
            false => CHSEL4::B0x0,
            true => CHSEL4::B0x1,
        }
    }
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL4::B0x0
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL4::B0x1
    }
}
///Field `CHSEL4` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL4_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL4>;
impl<'a, REG> CHSEL4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL4::B0x0)
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL4::B0x1)
    }
}
/**Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL5 {
    ///0: Input Channel-x is not selected for conversion
    B0x0 = 0,
    ///1: Input Channel-x is selected for conversion
    B0x1 = 1,
}
impl From<CHSEL5> for bool {
    #[inline(always)]
    fn from(variant: CHSEL5) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL5` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL5_R = crate::BitReader<CHSEL5>;
impl CHSEL5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL5 {
        match self.bits {
            false => CHSEL5::B0x0,
            true => CHSEL5::B0x1,
        }
    }
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL5::B0x0
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL5::B0x1
    }
}
///Field `CHSEL5` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL5_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL5>;
impl<'a, REG> CHSEL5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL5::B0x0)
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL5::B0x1)
    }
}
/**Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL6 {
    ///0: Input Channel-x is not selected for conversion
    B0x0 = 0,
    ///1: Input Channel-x is selected for conversion
    B0x1 = 1,
}
impl From<CHSEL6> for bool {
    #[inline(always)]
    fn from(variant: CHSEL6) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL6` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL6_R = crate::BitReader<CHSEL6>;
impl CHSEL6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL6 {
        match self.bits {
            false => CHSEL6::B0x0,
            true => CHSEL6::B0x1,
        }
    }
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL6::B0x0
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL6::B0x1
    }
}
///Field `CHSEL6` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL6_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL6>;
impl<'a, REG> CHSEL6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL6::B0x0)
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL6::B0x1)
    }
}
/**Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL7 {
    ///0: Input Channel-x is not selected for conversion
    B0x0 = 0,
    ///1: Input Channel-x is selected for conversion
    B0x1 = 1,
}
impl From<CHSEL7> for bool {
    #[inline(always)]
    fn from(variant: CHSEL7) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL7` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL7_R = crate::BitReader<CHSEL7>;
impl CHSEL7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL7 {
        match self.bits {
            false => CHSEL7::B0x0,
            true => CHSEL7::B0x1,
        }
    }
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL7::B0x0
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL7::B0x1
    }
}
///Field `CHSEL7` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL7_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL7>;
impl<'a, REG> CHSEL7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL7::B0x0)
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL7::B0x1)
    }
}
/**Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL8 {
    ///0: Input Channel-x is not selected for conversion
    B0x0 = 0,
    ///1: Input Channel-x is selected for conversion
    B0x1 = 1,
}
impl From<CHSEL8> for bool {
    #[inline(always)]
    fn from(variant: CHSEL8) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL8` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL8_R = crate::BitReader<CHSEL8>;
impl CHSEL8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL8 {
        match self.bits {
            false => CHSEL8::B0x0,
            true => CHSEL8::B0x1,
        }
    }
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL8::B0x0
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL8::B0x1
    }
}
///Field `CHSEL8` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL8_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL8>;
impl<'a, REG> CHSEL8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL8::B0x0)
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL8::B0x1)
    }
}
/**Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL9 {
    ///0: Input Channel-x is not selected for conversion
    B0x0 = 0,
    ///1: Input Channel-x is selected for conversion
    B0x1 = 1,
}
impl From<CHSEL9> for bool {
    #[inline(always)]
    fn from(variant: CHSEL9) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL9` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL9_R = crate::BitReader<CHSEL9>;
impl CHSEL9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL9 {
        match self.bits {
            false => CHSEL9::B0x0,
            true => CHSEL9::B0x1,
        }
    }
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL9::B0x0
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL9::B0x1
    }
}
///Field `CHSEL9` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL9_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL9>;
impl<'a, REG> CHSEL9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL9::B0x0)
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL9::B0x1)
    }
}
/**Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL10 {
    ///0: Input Channel-x is not selected for conversion
    B0x0 = 0,
    ///1: Input Channel-x is selected for conversion
    B0x1 = 1,
}
impl From<CHSEL10> for bool {
    #[inline(always)]
    fn from(variant: CHSEL10) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL10` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL10_R = crate::BitReader<CHSEL10>;
impl CHSEL10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL10 {
        match self.bits {
            false => CHSEL10::B0x0,
            true => CHSEL10::B0x1,
        }
    }
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL10::B0x0
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL10::B0x1
    }
}
///Field `CHSEL10` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL10_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL10>;
impl<'a, REG> CHSEL10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL10::B0x0)
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL10::B0x1)
    }
}
/**Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL11 {
    ///0: Input Channel-x is not selected for conversion
    B0x0 = 0,
    ///1: Input Channel-x is selected for conversion
    B0x1 = 1,
}
impl From<CHSEL11> for bool {
    #[inline(always)]
    fn from(variant: CHSEL11) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL11` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL11_R = crate::BitReader<CHSEL11>;
impl CHSEL11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL11 {
        match self.bits {
            false => CHSEL11::B0x0,
            true => CHSEL11::B0x1,
        }
    }
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL11::B0x0
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL11::B0x1
    }
}
///Field `CHSEL11` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL11_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL11>;
impl<'a, REG> CHSEL11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL11::B0x0)
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL11::B0x1)
    }
}
/**Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL12 {
    ///0: Input Channel-x is not selected for conversion
    B0x0 = 0,
    ///1: Input Channel-x is selected for conversion
    B0x1 = 1,
}
impl From<CHSEL12> for bool {
    #[inline(always)]
    fn from(variant: CHSEL12) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL12` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL12_R = crate::BitReader<CHSEL12>;
impl CHSEL12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL12 {
        match self.bits {
            false => CHSEL12::B0x0,
            true => CHSEL12::B0x1,
        }
    }
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL12::B0x0
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL12::B0x1
    }
}
///Field `CHSEL12` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL12_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL12>;
impl<'a, REG> CHSEL12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL12::B0x0)
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL12::B0x1)
    }
}
/**Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL13 {
    ///0: Input Channel-x is not selected for conversion
    B0x0 = 0,
    ///1: Input Channel-x is selected for conversion
    B0x1 = 1,
}
impl From<CHSEL13> for bool {
    #[inline(always)]
    fn from(variant: CHSEL13) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL13` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL13_R = crate::BitReader<CHSEL13>;
impl CHSEL13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL13 {
        match self.bits {
            false => CHSEL13::B0x0,
            true => CHSEL13::B0x1,
        }
    }
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL13::B0x0
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL13::B0x1
    }
}
///Field `CHSEL13` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL13_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL13>;
impl<'a, REG> CHSEL13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL13::B0x0)
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL13::B0x1)
    }
}
/**Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL14 {
    ///0: Input Channel-x is not selected for conversion
    B0x0 = 0,
    ///1: Input Channel-x is selected for conversion
    B0x1 = 1,
}
impl From<CHSEL14> for bool {
    #[inline(always)]
    fn from(variant: CHSEL14) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL14` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL14_R = crate::BitReader<CHSEL14>;
impl CHSEL14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL14 {
        match self.bits {
            false => CHSEL14::B0x0,
            true => CHSEL14::B0x1,
        }
    }
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL14::B0x0
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL14::B0x1
    }
}
///Field `CHSEL14` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL14_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL14>;
impl<'a, REG> CHSEL14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL14::B0x0)
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL14::B0x1)
    }
}
/**Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL15 {
    ///0: Input Channel-x is not selected for conversion
    B0x0 = 0,
    ///1: Input Channel-x is selected for conversion
    B0x1 = 1,
}
impl From<CHSEL15> for bool {
    #[inline(always)]
    fn from(variant: CHSEL15) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL15` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL15_R = crate::BitReader<CHSEL15>;
impl CHSEL15_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL15 {
        match self.bits {
            false => CHSEL15::B0x0,
            true => CHSEL15::B0x1,
        }
    }
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL15::B0x0
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL15::B0x1
    }
}
///Field `CHSEL15` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL15_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL15>;
impl<'a, REG> CHSEL15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL15::B0x0)
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL15::B0x1)
    }
}
/**Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL16 {
    ///0: Input Channel-x is not selected for conversion
    B0x0 = 0,
    ///1: Input Channel-x is selected for conversion
    B0x1 = 1,
}
impl From<CHSEL16> for bool {
    #[inline(always)]
    fn from(variant: CHSEL16) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL16` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL16_R = crate::BitReader<CHSEL16>;
impl CHSEL16_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL16 {
        match self.bits {
            false => CHSEL16::B0x0,
            true => CHSEL16::B0x1,
        }
    }
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL16::B0x0
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL16::B0x1
    }
}
///Field `CHSEL16` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL16_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL16>;
impl<'a, REG> CHSEL16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL16::B0x0)
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL16::B0x1)
    }
}
/**Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL17 {
    ///0: Input Channel-x is not selected for conversion
    B0x0 = 0,
    ///1: Input Channel-x is selected for conversion
    B0x1 = 1,
}
impl From<CHSEL17> for bool {
    #[inline(always)]
    fn from(variant: CHSEL17) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL17` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL17_R = crate::BitReader<CHSEL17>;
impl CHSEL17_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL17 {
        match self.bits {
            false => CHSEL17::B0x0,
            true => CHSEL17::B0x1,
        }
    }
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL17::B0x0
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL17::B0x1
    }
}
///Field `CHSEL17` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL17_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL17>;
impl<'a, REG> CHSEL17_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL17::B0x0)
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL17::B0x1)
    }
}
/**Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL18 {
    ///0: Input Channel-x is not selected for conversion
    B0x0 = 0,
    ///1: Input Channel-x is selected for conversion
    B0x1 = 1,
}
impl From<CHSEL18> for bool {
    #[inline(always)]
    fn from(variant: CHSEL18) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL18` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL18_R = crate::BitReader<CHSEL18>;
impl CHSEL18_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL18 {
        match self.bits {
            false => CHSEL18::B0x0,
            true => CHSEL18::B0x1,
        }
    }
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL18::B0x0
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL18::B0x1
    }
}
///Field `CHSEL18` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL18_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL18>;
impl<'a, REG> CHSEL18_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL18::B0x0)
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL18::B0x1)
    }
}
/**Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL19 {
    ///0: Input Channel-x is not selected for conversion
    B0x0 = 0,
    ///1: Input Channel-x is selected for conversion
    B0x1 = 1,
}
impl From<CHSEL19> for bool {
    #[inline(always)]
    fn from(variant: CHSEL19) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL19` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL19_R = crate::BitReader<CHSEL19>;
impl CHSEL19_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL19 {
        match self.bits {
            false => CHSEL19::B0x0,
            true => CHSEL19::B0x1,
        }
    }
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL19::B0x0
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL19::B0x1
    }
}
///Field `CHSEL19` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL19_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL19>;
impl<'a, REG> CHSEL19_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL19::B0x0)
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL19::B0x1)
    }
}
/**Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL20 {
    ///0: Input Channel-x is not selected for conversion
    B0x0 = 0,
    ///1: Input Channel-x is selected for conversion
    B0x1 = 1,
}
impl From<CHSEL20> for bool {
    #[inline(always)]
    fn from(variant: CHSEL20) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL20` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL20_R = crate::BitReader<CHSEL20>;
impl CHSEL20_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL20 {
        match self.bits {
            false => CHSEL20::B0x0,
            true => CHSEL20::B0x1,
        }
    }
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL20::B0x0
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL20::B0x1
    }
}
///Field `CHSEL20` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL20_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL20>;
impl<'a, REG> CHSEL20_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL20::B0x0)
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL20::B0x1)
    }
}
/**Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL21 {
    ///0: Input Channel-x is not selected for conversion
    B0x0 = 0,
    ///1: Input Channel-x is selected for conversion
    B0x1 = 1,
}
impl From<CHSEL21> for bool {
    #[inline(always)]
    fn from(variant: CHSEL21) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL21` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL21_R = crate::BitReader<CHSEL21>;
impl CHSEL21_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL21 {
        match self.bits {
            false => CHSEL21::B0x0,
            true => CHSEL21::B0x1,
        }
    }
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL21::B0x0
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL21::B0x1
    }
}
///Field `CHSEL21` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL21_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL21>;
impl<'a, REG> CHSEL21_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL21::B0x0)
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL21::B0x1)
    }
}
/**Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL22 {
    ///0: Input Channel-x is not selected for conversion
    B0x0 = 0,
    ///1: Input Channel-x is selected for conversion
    B0x1 = 1,
}
impl From<CHSEL22> for bool {
    #[inline(always)]
    fn from(variant: CHSEL22) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL22` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL22_R = crate::BitReader<CHSEL22>;
impl CHSEL22_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL22 {
        match self.bits {
            false => CHSEL22::B0x0,
            true => CHSEL22::B0x1,
        }
    }
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL22::B0x0
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL22::B0x1
    }
}
///Field `CHSEL22` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
pub type CHSEL22_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL22>;
impl<'a, REG> CHSEL22_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel-x is not selected for conversion
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL22::B0x0)
    }
    ///Input Channel-x is selected for conversion
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL22::B0x1)
    }
}
impl R {
    ///Bit 0 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel0(&self) -> CHSEL0_R {
        CHSEL0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel1(&self) -> CHSEL1_R {
        CHSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel2(&self) -> CHSEL2_R {
        CHSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel3(&self) -> CHSEL3_R {
        CHSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel4(&self) -> CHSEL4_R {
        CHSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel5(&self) -> CHSEL5_R {
        CHSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel6(&self) -> CHSEL6_R {
        CHSEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel7(&self) -> CHSEL7_R {
        CHSEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel8(&self) -> CHSEL8_R {
        CHSEL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel9(&self) -> CHSEL9_R {
        CHSEL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel10(&self) -> CHSEL10_R {
        CHSEL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel11(&self) -> CHSEL11_R {
        CHSEL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel12(&self) -> CHSEL12_R {
        CHSEL12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel13(&self) -> CHSEL13_R {
        CHSEL13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel14(&self) -> CHSEL14_R {
        CHSEL14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel15(&self) -> CHSEL15_R {
        CHSEL15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel16(&self) -> CHSEL16_R {
        CHSEL16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel17(&self) -> CHSEL17_R {
        CHSEL17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel18(&self) -> CHSEL18_R {
        CHSEL18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel19(&self) -> CHSEL19_R {
        CHSEL19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel20(&self) -> CHSEL20_R {
        CHSEL20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel21(&self) -> CHSEL21_R {
        CHSEL21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel22(&self) -> CHSEL22_R {
        CHSEL22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CHSELR")
            .field("chsel0", &self.chsel0())
            .field("chsel1", &self.chsel1())
            .field("chsel2", &self.chsel2())
            .field("chsel3", &self.chsel3())
            .field("chsel4", &self.chsel4())
            .field("chsel5", &self.chsel5())
            .field("chsel6", &self.chsel6())
            .field("chsel7", &self.chsel7())
            .field("chsel8", &self.chsel8())
            .field("chsel9", &self.chsel9())
            .field("chsel10", &self.chsel10())
            .field("chsel11", &self.chsel11())
            .field("chsel12", &self.chsel12())
            .field("chsel13", &self.chsel13())
            .field("chsel14", &self.chsel14())
            .field("chsel15", &self.chsel15())
            .field("chsel16", &self.chsel16())
            .field("chsel17", &self.chsel17())
            .field("chsel18", &self.chsel18())
            .field("chsel19", &self.chsel19())
            .field("chsel20", &self.chsel20())
            .field("chsel21", &self.chsel21())
            .field("chsel22", &self.chsel22())
            .finish()
    }
}
impl W {
    ///Bit 0 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel0(&mut self) -> CHSEL0_W<'_, ADC_CHSELRrs> {
        CHSEL0_W::new(self, 0)
    }
    ///Bit 1 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel1(&mut self) -> CHSEL1_W<'_, ADC_CHSELRrs> {
        CHSEL1_W::new(self, 1)
    }
    ///Bit 2 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel2(&mut self) -> CHSEL2_W<'_, ADC_CHSELRrs> {
        CHSEL2_W::new(self, 2)
    }
    ///Bit 3 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel3(&mut self) -> CHSEL3_W<'_, ADC_CHSELRrs> {
        CHSEL3_W::new(self, 3)
    }
    ///Bit 4 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel4(&mut self) -> CHSEL4_W<'_, ADC_CHSELRrs> {
        CHSEL4_W::new(self, 4)
    }
    ///Bit 5 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel5(&mut self) -> CHSEL5_W<'_, ADC_CHSELRrs> {
        CHSEL5_W::new(self, 5)
    }
    ///Bit 6 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel6(&mut self) -> CHSEL6_W<'_, ADC_CHSELRrs> {
        CHSEL6_W::new(self, 6)
    }
    ///Bit 7 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel7(&mut self) -> CHSEL7_W<'_, ADC_CHSELRrs> {
        CHSEL7_W::new(self, 7)
    }
    ///Bit 8 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel8(&mut self) -> CHSEL8_W<'_, ADC_CHSELRrs> {
        CHSEL8_W::new(self, 8)
    }
    ///Bit 9 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel9(&mut self) -> CHSEL9_W<'_, ADC_CHSELRrs> {
        CHSEL9_W::new(self, 9)
    }
    ///Bit 10 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel10(&mut self) -> CHSEL10_W<'_, ADC_CHSELRrs> {
        CHSEL10_W::new(self, 10)
    }
    ///Bit 11 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel11(&mut self) -> CHSEL11_W<'_, ADC_CHSELRrs> {
        CHSEL11_W::new(self, 11)
    }
    ///Bit 12 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel12(&mut self) -> CHSEL12_W<'_, ADC_CHSELRrs> {
        CHSEL12_W::new(self, 12)
    }
    ///Bit 13 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel13(&mut self) -> CHSEL13_W<'_, ADC_CHSELRrs> {
        CHSEL13_W::new(self, 13)
    }
    ///Bit 14 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel14(&mut self) -> CHSEL14_W<'_, ADC_CHSELRrs> {
        CHSEL14_W::new(self, 14)
    }
    ///Bit 15 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel15(&mut self) -> CHSEL15_W<'_, ADC_CHSELRrs> {
        CHSEL15_W::new(self, 15)
    }
    ///Bit 16 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel16(&mut self) -> CHSEL16_W<'_, ADC_CHSELRrs> {
        CHSEL16_W::new(self, 16)
    }
    ///Bit 17 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel17(&mut self) -> CHSEL17_W<'_, ADC_CHSELRrs> {
        CHSEL17_W::new(self, 17)
    }
    ///Bit 18 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel18(&mut self) -> CHSEL18_W<'_, ADC_CHSELRrs> {
        CHSEL18_W::new(self, 18)
    }
    ///Bit 19 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel19(&mut self) -> CHSEL19_W<'_, ADC_CHSELRrs> {
        CHSEL19_W::new(self, 19)
    }
    ///Bit 20 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel20(&mut self) -> CHSEL20_W<'_, ADC_CHSELRrs> {
        CHSEL20_W::new(self, 20)
    }
    ///Bit 21 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel21(&mut self) -> CHSEL21_W<'_, ADC_CHSELRrs> {
        CHSEL21_W::new(self, 21)
    }
    ///Bit 22 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to Figure 35: ADC connectivity for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing). Note: If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.
    #[inline(always)]
    pub fn chsel22(&mut self) -> CHSEL22_W<'_, ADC_CHSELRrs> {
        CHSEL22_W::new(self, 22)
    }
}
/**ADC channel selection register

You can [`read`](crate::Reg::read) this register and get [`adc_chselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_chselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#ADC:ADC_CHSELR)*/
pub struct ADC_CHSELRrs;
impl crate::RegisterSpec for ADC_CHSELRrs {
    type Ux = u32;
}
///`read()` method returns [`adc_chselr::R`](R) reader structure
impl crate::Readable for ADC_CHSELRrs {}
///`write(|w| ..)` method takes [`adc_chselr::W`](W) writer structure
impl crate::Writable for ADC_CHSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADC_CHSELR to value 0
impl crate::Resettable for ADC_CHSELRrs {}
