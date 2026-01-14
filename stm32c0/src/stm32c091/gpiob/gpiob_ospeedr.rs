///Register `GPIOB_OSPEEDR` reader
pub type R = crate::R<GPIOB_OSPEEDRrs>;
///Register `GPIOB_OSPEEDR` writer
pub type W = crate::W<GPIOB_OSPEEDRrs>;
/**Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED0 {
    ///0: Very low speed
    B0x0 = 0,
    ///1: Low speed
    B0x1 = 1,
    ///2: High speed
    B0x2 = 2,
    ///3: Very high speed
    B0x3 = 3,
}
impl From<OSPEED0> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED0 {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED0 {}
///Field `OSPEED0` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED0_R = crate::FieldReader<OSPEED0>;
impl OSPEED0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED0 {
        match self.bits {
            0 => OSPEED0::B0x0,
            1 => OSPEED0::B0x1,
            2 => OSPEED0::B0x2,
            3 => OSPEED0::B0x3,
            _ => unreachable!(),
        }
    }
    ///Very low speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OSPEED0::B0x0
    }
    ///Low speed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OSPEED0::B0x1
    }
    ///High speed
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OSPEED0::B0x2
    }
    ///Very high speed
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OSPEED0::B0x3
    }
}
///Field `OSPEED0` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED0, crate::Safe>;
impl<'a, REG> OSPEED0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Very low speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED0::B0x0)
    }
    ///Low speed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED0::B0x1)
    }
    ///High speed
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED0::B0x2)
    }
    ///Very high speed
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED0::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED1 {
    ///0: Very low speed
    B0x0 = 0,
    ///1: Low speed
    B0x1 = 1,
    ///2: High speed
    B0x2 = 2,
    ///3: Very high speed
    B0x3 = 3,
}
impl From<OSPEED1> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED1 {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED1 {}
///Field `OSPEED1` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED1_R = crate::FieldReader<OSPEED1>;
impl OSPEED1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED1 {
        match self.bits {
            0 => OSPEED1::B0x0,
            1 => OSPEED1::B0x1,
            2 => OSPEED1::B0x2,
            3 => OSPEED1::B0x3,
            _ => unreachable!(),
        }
    }
    ///Very low speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OSPEED1::B0x0
    }
    ///Low speed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OSPEED1::B0x1
    }
    ///High speed
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OSPEED1::B0x2
    }
    ///Very high speed
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OSPEED1::B0x3
    }
}
///Field `OSPEED1` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED1, crate::Safe>;
impl<'a, REG> OSPEED1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Very low speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED1::B0x0)
    }
    ///Low speed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED1::B0x1)
    }
    ///High speed
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED1::B0x2)
    }
    ///Very high speed
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED1::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED2 {
    ///0: Very low speed
    B0x0 = 0,
    ///1: Low speed
    B0x1 = 1,
    ///2: High speed
    B0x2 = 2,
    ///3: Very high speed
    B0x3 = 3,
}
impl From<OSPEED2> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED2 {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED2 {}
///Field `OSPEED2` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED2_R = crate::FieldReader<OSPEED2>;
impl OSPEED2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED2 {
        match self.bits {
            0 => OSPEED2::B0x0,
            1 => OSPEED2::B0x1,
            2 => OSPEED2::B0x2,
            3 => OSPEED2::B0x3,
            _ => unreachable!(),
        }
    }
    ///Very low speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OSPEED2::B0x0
    }
    ///Low speed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OSPEED2::B0x1
    }
    ///High speed
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OSPEED2::B0x2
    }
    ///Very high speed
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OSPEED2::B0x3
    }
}
///Field `OSPEED2` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED2, crate::Safe>;
impl<'a, REG> OSPEED2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Very low speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED2::B0x0)
    }
    ///Low speed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED2::B0x1)
    }
    ///High speed
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED2::B0x2)
    }
    ///Very high speed
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED2::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED3 {
    ///0: Very low speed
    B0x0 = 0,
    ///1: Low speed
    B0x1 = 1,
    ///2: High speed
    B0x2 = 2,
    ///3: Very high speed
    B0x3 = 3,
}
impl From<OSPEED3> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED3 {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED3 {}
///Field `OSPEED3` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED3_R = crate::FieldReader<OSPEED3>;
impl OSPEED3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED3 {
        match self.bits {
            0 => OSPEED3::B0x0,
            1 => OSPEED3::B0x1,
            2 => OSPEED3::B0x2,
            3 => OSPEED3::B0x3,
            _ => unreachable!(),
        }
    }
    ///Very low speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OSPEED3::B0x0
    }
    ///Low speed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OSPEED3::B0x1
    }
    ///High speed
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OSPEED3::B0x2
    }
    ///Very high speed
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OSPEED3::B0x3
    }
}
///Field `OSPEED3` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED3_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED3, crate::Safe>;
impl<'a, REG> OSPEED3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Very low speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED3::B0x0)
    }
    ///Low speed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED3::B0x1)
    }
    ///High speed
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED3::B0x2)
    }
    ///Very high speed
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED3::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED4 {
    ///0: Very low speed
    B0x0 = 0,
    ///1: Low speed
    B0x1 = 1,
    ///2: High speed
    B0x2 = 2,
    ///3: Very high speed
    B0x3 = 3,
}
impl From<OSPEED4> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED4 {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED4 {}
///Field `OSPEED4` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED4_R = crate::FieldReader<OSPEED4>;
impl OSPEED4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED4 {
        match self.bits {
            0 => OSPEED4::B0x0,
            1 => OSPEED4::B0x1,
            2 => OSPEED4::B0x2,
            3 => OSPEED4::B0x3,
            _ => unreachable!(),
        }
    }
    ///Very low speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OSPEED4::B0x0
    }
    ///Low speed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OSPEED4::B0x1
    }
    ///High speed
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OSPEED4::B0x2
    }
    ///Very high speed
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OSPEED4::B0x3
    }
}
///Field `OSPEED4` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED4_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED4, crate::Safe>;
impl<'a, REG> OSPEED4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Very low speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED4::B0x0)
    }
    ///Low speed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED4::B0x1)
    }
    ///High speed
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED4::B0x2)
    }
    ///Very high speed
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED4::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED5 {
    ///0: Very low speed
    B0x0 = 0,
    ///1: Low speed
    B0x1 = 1,
    ///2: High speed
    B0x2 = 2,
    ///3: Very high speed
    B0x3 = 3,
}
impl From<OSPEED5> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED5 {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED5 {}
///Field `OSPEED5` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED5_R = crate::FieldReader<OSPEED5>;
impl OSPEED5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED5 {
        match self.bits {
            0 => OSPEED5::B0x0,
            1 => OSPEED5::B0x1,
            2 => OSPEED5::B0x2,
            3 => OSPEED5::B0x3,
            _ => unreachable!(),
        }
    }
    ///Very low speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OSPEED5::B0x0
    }
    ///Low speed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OSPEED5::B0x1
    }
    ///High speed
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OSPEED5::B0x2
    }
    ///Very high speed
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OSPEED5::B0x3
    }
}
///Field `OSPEED5` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED5_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED5, crate::Safe>;
impl<'a, REG> OSPEED5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Very low speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED5::B0x0)
    }
    ///Low speed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED5::B0x1)
    }
    ///High speed
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED5::B0x2)
    }
    ///Very high speed
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED5::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED6 {
    ///0: Very low speed
    B0x0 = 0,
    ///1: Low speed
    B0x1 = 1,
    ///2: High speed
    B0x2 = 2,
    ///3: Very high speed
    B0x3 = 3,
}
impl From<OSPEED6> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED6 {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED6 {}
///Field `OSPEED6` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED6_R = crate::FieldReader<OSPEED6>;
impl OSPEED6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED6 {
        match self.bits {
            0 => OSPEED6::B0x0,
            1 => OSPEED6::B0x1,
            2 => OSPEED6::B0x2,
            3 => OSPEED6::B0x3,
            _ => unreachable!(),
        }
    }
    ///Very low speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OSPEED6::B0x0
    }
    ///Low speed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OSPEED6::B0x1
    }
    ///High speed
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OSPEED6::B0x2
    }
    ///Very high speed
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OSPEED6::B0x3
    }
}
///Field `OSPEED6` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED6_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED6, crate::Safe>;
impl<'a, REG> OSPEED6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Very low speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED6::B0x0)
    }
    ///Low speed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED6::B0x1)
    }
    ///High speed
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED6::B0x2)
    }
    ///Very high speed
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED6::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED7 {
    ///0: Very low speed
    B0x0 = 0,
    ///1: Low speed
    B0x1 = 1,
    ///2: High speed
    B0x2 = 2,
    ///3: Very high speed
    B0x3 = 3,
}
impl From<OSPEED7> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED7 {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED7 {}
///Field `OSPEED7` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED7_R = crate::FieldReader<OSPEED7>;
impl OSPEED7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED7 {
        match self.bits {
            0 => OSPEED7::B0x0,
            1 => OSPEED7::B0x1,
            2 => OSPEED7::B0x2,
            3 => OSPEED7::B0x3,
            _ => unreachable!(),
        }
    }
    ///Very low speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OSPEED7::B0x0
    }
    ///Low speed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OSPEED7::B0x1
    }
    ///High speed
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OSPEED7::B0x2
    }
    ///Very high speed
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OSPEED7::B0x3
    }
}
///Field `OSPEED7` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED7_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED7, crate::Safe>;
impl<'a, REG> OSPEED7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Very low speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED7::B0x0)
    }
    ///Low speed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED7::B0x1)
    }
    ///High speed
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED7::B0x2)
    }
    ///Very high speed
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED7::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED8 {
    ///0: Very low speed
    B0x0 = 0,
    ///1: Low speed
    B0x1 = 1,
    ///2: High speed
    B0x2 = 2,
    ///3: Very high speed
    B0x3 = 3,
}
impl From<OSPEED8> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED8 {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED8 {}
///Field `OSPEED8` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED8_R = crate::FieldReader<OSPEED8>;
impl OSPEED8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED8 {
        match self.bits {
            0 => OSPEED8::B0x0,
            1 => OSPEED8::B0x1,
            2 => OSPEED8::B0x2,
            3 => OSPEED8::B0x3,
            _ => unreachable!(),
        }
    }
    ///Very low speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OSPEED8::B0x0
    }
    ///Low speed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OSPEED8::B0x1
    }
    ///High speed
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OSPEED8::B0x2
    }
    ///Very high speed
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OSPEED8::B0x3
    }
}
///Field `OSPEED8` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED8_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED8, crate::Safe>;
impl<'a, REG> OSPEED8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Very low speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED8::B0x0)
    }
    ///Low speed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED8::B0x1)
    }
    ///High speed
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED8::B0x2)
    }
    ///Very high speed
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED8::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED9 {
    ///0: Very low speed
    B0x0 = 0,
    ///1: Low speed
    B0x1 = 1,
    ///2: High speed
    B0x2 = 2,
    ///3: Very high speed
    B0x3 = 3,
}
impl From<OSPEED9> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED9 {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED9 {}
///Field `OSPEED9` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED9_R = crate::FieldReader<OSPEED9>;
impl OSPEED9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED9 {
        match self.bits {
            0 => OSPEED9::B0x0,
            1 => OSPEED9::B0x1,
            2 => OSPEED9::B0x2,
            3 => OSPEED9::B0x3,
            _ => unreachable!(),
        }
    }
    ///Very low speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OSPEED9::B0x0
    }
    ///Low speed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OSPEED9::B0x1
    }
    ///High speed
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OSPEED9::B0x2
    }
    ///Very high speed
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OSPEED9::B0x3
    }
}
///Field `OSPEED9` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED9_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED9, crate::Safe>;
impl<'a, REG> OSPEED9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Very low speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED9::B0x0)
    }
    ///Low speed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED9::B0x1)
    }
    ///High speed
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED9::B0x2)
    }
    ///Very high speed
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED9::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED10 {
    ///0: Very low speed
    B0x0 = 0,
    ///1: Low speed
    B0x1 = 1,
    ///2: High speed
    B0x2 = 2,
    ///3: Very high speed
    B0x3 = 3,
}
impl From<OSPEED10> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED10 {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED10 {}
///Field `OSPEED10` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED10_R = crate::FieldReader<OSPEED10>;
impl OSPEED10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED10 {
        match self.bits {
            0 => OSPEED10::B0x0,
            1 => OSPEED10::B0x1,
            2 => OSPEED10::B0x2,
            3 => OSPEED10::B0x3,
            _ => unreachable!(),
        }
    }
    ///Very low speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OSPEED10::B0x0
    }
    ///Low speed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OSPEED10::B0x1
    }
    ///High speed
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OSPEED10::B0x2
    }
    ///Very high speed
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OSPEED10::B0x3
    }
}
///Field `OSPEED10` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED10_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED10, crate::Safe>;
impl<'a, REG> OSPEED10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Very low speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED10::B0x0)
    }
    ///Low speed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED10::B0x1)
    }
    ///High speed
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED10::B0x2)
    }
    ///Very high speed
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED10::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED11 {
    ///0: Very low speed
    B0x0 = 0,
    ///1: Low speed
    B0x1 = 1,
    ///2: High speed
    B0x2 = 2,
    ///3: Very high speed
    B0x3 = 3,
}
impl From<OSPEED11> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED11 {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED11 {}
///Field `OSPEED11` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED11_R = crate::FieldReader<OSPEED11>;
impl OSPEED11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED11 {
        match self.bits {
            0 => OSPEED11::B0x0,
            1 => OSPEED11::B0x1,
            2 => OSPEED11::B0x2,
            3 => OSPEED11::B0x3,
            _ => unreachable!(),
        }
    }
    ///Very low speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OSPEED11::B0x0
    }
    ///Low speed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OSPEED11::B0x1
    }
    ///High speed
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OSPEED11::B0x2
    }
    ///Very high speed
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OSPEED11::B0x3
    }
}
///Field `OSPEED11` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED11_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED11, crate::Safe>;
impl<'a, REG> OSPEED11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Very low speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED11::B0x0)
    }
    ///Low speed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED11::B0x1)
    }
    ///High speed
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED11::B0x2)
    }
    ///Very high speed
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED11::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED12 {
    ///0: Very low speed
    B0x0 = 0,
    ///1: Low speed
    B0x1 = 1,
    ///2: High speed
    B0x2 = 2,
    ///3: Very high speed
    B0x3 = 3,
}
impl From<OSPEED12> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED12 {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED12 {}
///Field `OSPEED12` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED12_R = crate::FieldReader<OSPEED12>;
impl OSPEED12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED12 {
        match self.bits {
            0 => OSPEED12::B0x0,
            1 => OSPEED12::B0x1,
            2 => OSPEED12::B0x2,
            3 => OSPEED12::B0x3,
            _ => unreachable!(),
        }
    }
    ///Very low speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OSPEED12::B0x0
    }
    ///Low speed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OSPEED12::B0x1
    }
    ///High speed
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OSPEED12::B0x2
    }
    ///Very high speed
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OSPEED12::B0x3
    }
}
///Field `OSPEED12` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED12_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED12, crate::Safe>;
impl<'a, REG> OSPEED12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Very low speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED12::B0x0)
    }
    ///Low speed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED12::B0x1)
    }
    ///High speed
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED12::B0x2)
    }
    ///Very high speed
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED12::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED13 {
    ///0: Very low speed
    B0x0 = 0,
    ///1: Low speed
    B0x1 = 1,
    ///2: High speed
    B0x2 = 2,
    ///3: Very high speed
    B0x3 = 3,
}
impl From<OSPEED13> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED13 {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED13 {}
///Field `OSPEED13` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED13_R = crate::FieldReader<OSPEED13>;
impl OSPEED13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED13 {
        match self.bits {
            0 => OSPEED13::B0x0,
            1 => OSPEED13::B0x1,
            2 => OSPEED13::B0x2,
            3 => OSPEED13::B0x3,
            _ => unreachable!(),
        }
    }
    ///Very low speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OSPEED13::B0x0
    }
    ///Low speed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OSPEED13::B0x1
    }
    ///High speed
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OSPEED13::B0x2
    }
    ///Very high speed
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OSPEED13::B0x3
    }
}
///Field `OSPEED13` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED13_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED13, crate::Safe>;
impl<'a, REG> OSPEED13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Very low speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED13::B0x0)
    }
    ///Low speed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED13::B0x1)
    }
    ///High speed
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED13::B0x2)
    }
    ///Very high speed
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED13::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED14 {
    ///0: Very low speed
    B0x0 = 0,
    ///1: Low speed
    B0x1 = 1,
    ///2: High speed
    B0x2 = 2,
    ///3: Very high speed
    B0x3 = 3,
}
impl From<OSPEED14> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED14 {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED14 {}
///Field `OSPEED14` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED14_R = crate::FieldReader<OSPEED14>;
impl OSPEED14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED14 {
        match self.bits {
            0 => OSPEED14::B0x0,
            1 => OSPEED14::B0x1,
            2 => OSPEED14::B0x2,
            3 => OSPEED14::B0x3,
            _ => unreachable!(),
        }
    }
    ///Very low speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OSPEED14::B0x0
    }
    ///Low speed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OSPEED14::B0x1
    }
    ///High speed
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OSPEED14::B0x2
    }
    ///Very high speed
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OSPEED14::B0x3
    }
}
///Field `OSPEED14` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED14_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED14, crate::Safe>;
impl<'a, REG> OSPEED14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Very low speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED14::B0x0)
    }
    ///Low speed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED14::B0x1)
    }
    ///High speed
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED14::B0x2)
    }
    ///Very high speed
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED14::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED15 {
    ///0: Very low speed
    B0x0 = 0,
    ///1: Low speed
    B0x1 = 1,
    ///2: High speed
    B0x2 = 2,
    ///3: Very high speed
    B0x3 = 3,
}
impl From<OSPEED15> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED15 {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED15 {}
///Field `OSPEED15` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED15_R = crate::FieldReader<OSPEED15>;
impl OSPEED15_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED15 {
        match self.bits {
            0 => OSPEED15::B0x0,
            1 => OSPEED15::B0x1,
            2 => OSPEED15::B0x2,
            3 => OSPEED15::B0x3,
            _ => unreachable!(),
        }
    }
    ///Very low speed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OSPEED15::B0x0
    }
    ///Low speed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OSPEED15::B0x1
    }
    ///High speed
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OSPEED15::B0x2
    }
    ///Very high speed
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OSPEED15::B0x3
    }
}
///Field `OSPEED15` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
pub type OSPEED15_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED15, crate::Safe>;
impl<'a, REG> OSPEED15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Very low speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED15::B0x0)
    }
    ///Low speed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED15::B0x1)
    }
    ///High speed
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED15::B0x2)
    }
    ///Very high speed
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED15::B0x3)
    }
}
impl R {
    ///Bits 0:1 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed0(&self) -> OSPEED0_R {
        OSPEED0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed1(&self) -> OSPEED1_R {
        OSPEED1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed2(&self) -> OSPEED2_R {
        OSPEED2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed3(&self) -> OSPEED3_R {
        OSPEED3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed4(&self) -> OSPEED4_R {
        OSPEED4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed5(&self) -> OSPEED5_R {
        OSPEED5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed6(&self) -> OSPEED6_R {
        OSPEED6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed7(&self) -> OSPEED7_R {
        OSPEED7_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed8(&self) -> OSPEED8_R {
        OSPEED8_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed9(&self) -> OSPEED9_R {
        OSPEED9_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed10(&self) -> OSPEED10_R {
        OSPEED10_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed11(&self) -> OSPEED11_R {
        OSPEED11_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed12(&self) -> OSPEED12_R {
        OSPEED12_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed13(&self) -> OSPEED13_R {
        OSPEED13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed14(&self) -> OSPEED14_R {
        OSPEED14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed15(&self) -> OSPEED15_R {
        OSPEED15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOB_OSPEEDR")
            .field("ospeed0", &self.ospeed0())
            .field("ospeed1", &self.ospeed1())
            .field("ospeed2", &self.ospeed2())
            .field("ospeed3", &self.ospeed3())
            .field("ospeed4", &self.ospeed4())
            .field("ospeed5", &self.ospeed5())
            .field("ospeed6", &self.ospeed6())
            .field("ospeed7", &self.ospeed7())
            .field("ospeed8", &self.ospeed8())
            .field("ospeed9", &self.ospeed9())
            .field("ospeed10", &self.ospeed10())
            .field("ospeed11", &self.ospeed11())
            .field("ospeed12", &self.ospeed12())
            .field("ospeed13", &self.ospeed13())
            .field("ospeed14", &self.ospeed14())
            .field("ospeed15", &self.ospeed15())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed0(&mut self) -> OSPEED0_W<'_, GPIOB_OSPEEDRrs> {
        OSPEED0_W::new(self, 0)
    }
    ///Bits 2:3 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed1(&mut self) -> OSPEED1_W<'_, GPIOB_OSPEEDRrs> {
        OSPEED1_W::new(self, 2)
    }
    ///Bits 4:5 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed2(&mut self) -> OSPEED2_W<'_, GPIOB_OSPEEDRrs> {
        OSPEED2_W::new(self, 4)
    }
    ///Bits 6:7 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed3(&mut self) -> OSPEED3_W<'_, GPIOB_OSPEEDRrs> {
        OSPEED3_W::new(self, 6)
    }
    ///Bits 8:9 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed4(&mut self) -> OSPEED4_W<'_, GPIOB_OSPEEDRrs> {
        OSPEED4_W::new(self, 8)
    }
    ///Bits 10:11 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed5(&mut self) -> OSPEED5_W<'_, GPIOB_OSPEEDRrs> {
        OSPEED5_W::new(self, 10)
    }
    ///Bits 12:13 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed6(&mut self) -> OSPEED6_W<'_, GPIOB_OSPEEDRrs> {
        OSPEED6_W::new(self, 12)
    }
    ///Bits 14:15 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed7(&mut self) -> OSPEED7_W<'_, GPIOB_OSPEEDRrs> {
        OSPEED7_W::new(self, 14)
    }
    ///Bits 16:17 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed8(&mut self) -> OSPEED8_W<'_, GPIOB_OSPEEDRrs> {
        OSPEED8_W::new(self, 16)
    }
    ///Bits 18:19 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed9(&mut self) -> OSPEED9_W<'_, GPIOB_OSPEEDRrs> {
        OSPEED9_W::new(self, 18)
    }
    ///Bits 20:21 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed10(&mut self) -> OSPEED10_W<'_, GPIOB_OSPEEDRrs> {
        OSPEED10_W::new(self, 20)
    }
    ///Bits 22:23 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed11(&mut self) -> OSPEED11_W<'_, GPIOB_OSPEEDRrs> {
        OSPEED11_W::new(self, 22)
    }
    ///Bits 24:25 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed12(&mut self) -> OSPEED12_W<'_, GPIOB_OSPEEDRrs> {
        OSPEED12_W::new(self, 24)
    }
    ///Bits 26:27 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed13(&mut self) -> OSPEED13_W<'_, GPIOB_OSPEEDRrs> {
        OSPEED13_W::new(self, 26)
    }
    ///Bits 28:29 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed14(&mut self) -> OSPEED14_W<'_, GPIOB_OSPEEDRrs> {
        OSPEED14_W::new(self, 28)
    }
    ///Bits 30:31 - Port x configuration for I/O y These bits are written by software to configure the I/O output speed. Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.. Note: The FT_c GPIOs cannot be set to high speed.
    #[inline(always)]
    pub fn ospeed15(&mut self) -> OSPEED15_W<'_, GPIOB_OSPEEDRrs> {
        OSPEED15_W::new(self, 30)
    }
}
/**GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`gpiob_ospeedr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_ospeedr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOB:GPIOB_OSPEEDR)*/
pub struct GPIOB_OSPEEDRrs;
impl crate::RegisterSpec for GPIOB_OSPEEDRrs {
    type Ux = u32;
}
///`read()` method returns [`gpiob_ospeedr::R`](R) reader structure
impl crate::Readable for GPIOB_OSPEEDRrs {}
///`write(|w| ..)` method takes [`gpiob_ospeedr::W`](W) writer structure
impl crate::Writable for GPIOB_OSPEEDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPIOB_OSPEEDR to value 0
impl crate::Resettable for GPIOB_OSPEEDRrs {}
