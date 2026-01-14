///Register `GPIOF_MODER` reader
pub type R = crate::R<GPIOF_MODERrs>;
///Register `GPIOF_MODER` writer
pub type W = crate::W<GPIOF_MODERrs>;
/**Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE0 {
    ///0: Input
    B0x0 = 0,
    ///1: Output
    B0x1 = 1,
    ///2: Alternate function
    B0x2 = 2,
    ///3: Analog
    B0x3 = 3,
}
impl From<MODE0> for u8 {
    #[inline(always)]
    fn from(variant: MODE0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE0 {
    type Ux = u8;
}
impl crate::IsEnum for MODE0 {}
///Field `MODE0` reader - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE0_R = crate::FieldReader<MODE0>;
impl MODE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODE0 {
        match self.bits {
            0 => MODE0::B0x0,
            1 => MODE0::B0x1,
            2 => MODE0::B0x2,
            3 => MODE0::B0x3,
            _ => unreachable!(),
        }
    }
    ///Input
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MODE0::B0x0
    }
    ///Output
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MODE0::B0x1
    }
    ///Alternate function
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MODE0::B0x2
    }
    ///Analog
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MODE0::B0x3
    }
}
///Field `MODE0` writer - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE0, crate::Safe>;
impl<'a, REG> MODE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::B0x0)
    }
    ///Output
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::B0x1)
    }
    ///Alternate function
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::B0x2)
    }
    ///Analog
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE1 {
    ///0: Input
    B0x0 = 0,
    ///1: Output
    B0x1 = 1,
    ///2: Alternate function
    B0x2 = 2,
    ///3: Analog
    B0x3 = 3,
}
impl From<MODE1> for u8 {
    #[inline(always)]
    fn from(variant: MODE1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE1 {
    type Ux = u8;
}
impl crate::IsEnum for MODE1 {}
///Field `MODE1` reader - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE1_R = crate::FieldReader<MODE1>;
impl MODE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODE1 {
        match self.bits {
            0 => MODE1::B0x0,
            1 => MODE1::B0x1,
            2 => MODE1::B0x2,
            3 => MODE1::B0x3,
            _ => unreachable!(),
        }
    }
    ///Input
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MODE1::B0x0
    }
    ///Output
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MODE1::B0x1
    }
    ///Alternate function
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MODE1::B0x2
    }
    ///Analog
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MODE1::B0x3
    }
}
///Field `MODE1` writer - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE1, crate::Safe>;
impl<'a, REG> MODE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::B0x0)
    }
    ///Output
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::B0x1)
    }
    ///Alternate function
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::B0x2)
    }
    ///Analog
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE2 {
    ///0: Input
    B0x0 = 0,
    ///1: Output
    B0x1 = 1,
    ///2: Alternate function
    B0x2 = 2,
    ///3: Analog
    B0x3 = 3,
}
impl From<MODE2> for u8 {
    #[inline(always)]
    fn from(variant: MODE2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE2 {
    type Ux = u8;
}
impl crate::IsEnum for MODE2 {}
///Field `MODE2` reader - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE2_R = crate::FieldReader<MODE2>;
impl MODE2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODE2 {
        match self.bits {
            0 => MODE2::B0x0,
            1 => MODE2::B0x1,
            2 => MODE2::B0x2,
            3 => MODE2::B0x3,
            _ => unreachable!(),
        }
    }
    ///Input
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MODE2::B0x0
    }
    ///Output
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MODE2::B0x1
    }
    ///Alternate function
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MODE2::B0x2
    }
    ///Analog
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MODE2::B0x3
    }
}
///Field `MODE2` writer - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE2, crate::Safe>;
impl<'a, REG> MODE2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2::B0x0)
    }
    ///Output
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2::B0x1)
    }
    ///Alternate function
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2::B0x2)
    }
    ///Analog
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE3 {
    ///0: Input
    B0x0 = 0,
    ///1: Output
    B0x1 = 1,
    ///2: Alternate function
    B0x2 = 2,
    ///3: Analog
    B0x3 = 3,
}
impl From<MODE3> for u8 {
    #[inline(always)]
    fn from(variant: MODE3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE3 {
    type Ux = u8;
}
impl crate::IsEnum for MODE3 {}
///Field `MODE3` reader - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE3_R = crate::FieldReader<MODE3>;
impl MODE3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODE3 {
        match self.bits {
            0 => MODE3::B0x0,
            1 => MODE3::B0x1,
            2 => MODE3::B0x2,
            3 => MODE3::B0x3,
            _ => unreachable!(),
        }
    }
    ///Input
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MODE3::B0x0
    }
    ///Output
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MODE3::B0x1
    }
    ///Alternate function
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MODE3::B0x2
    }
    ///Analog
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MODE3::B0x3
    }
}
///Field `MODE3` writer - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE3_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE3, crate::Safe>;
impl<'a, REG> MODE3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3::B0x0)
    }
    ///Output
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3::B0x1)
    }
    ///Alternate function
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3::B0x2)
    }
    ///Analog
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE4 {
    ///0: Input
    B0x0 = 0,
    ///1: Output
    B0x1 = 1,
    ///2: Alternate function
    B0x2 = 2,
    ///3: Analog
    B0x3 = 3,
}
impl From<MODE4> for u8 {
    #[inline(always)]
    fn from(variant: MODE4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE4 {
    type Ux = u8;
}
impl crate::IsEnum for MODE4 {}
///Field `MODE4` reader - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE4_R = crate::FieldReader<MODE4>;
impl MODE4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODE4 {
        match self.bits {
            0 => MODE4::B0x0,
            1 => MODE4::B0x1,
            2 => MODE4::B0x2,
            3 => MODE4::B0x3,
            _ => unreachable!(),
        }
    }
    ///Input
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MODE4::B0x0
    }
    ///Output
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MODE4::B0x1
    }
    ///Alternate function
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MODE4::B0x2
    }
    ///Analog
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MODE4::B0x3
    }
}
///Field `MODE4` writer - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE4_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE4, crate::Safe>;
impl<'a, REG> MODE4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4::B0x0)
    }
    ///Output
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4::B0x1)
    }
    ///Alternate function
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4::B0x2)
    }
    ///Analog
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE5 {
    ///0: Input
    B0x0 = 0,
    ///1: Output
    B0x1 = 1,
    ///2: Alternate function
    B0x2 = 2,
    ///3: Analog
    B0x3 = 3,
}
impl From<MODE5> for u8 {
    #[inline(always)]
    fn from(variant: MODE5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE5 {
    type Ux = u8;
}
impl crate::IsEnum for MODE5 {}
///Field `MODE5` reader - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE5_R = crate::FieldReader<MODE5>;
impl MODE5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODE5 {
        match self.bits {
            0 => MODE5::B0x0,
            1 => MODE5::B0x1,
            2 => MODE5::B0x2,
            3 => MODE5::B0x3,
            _ => unreachable!(),
        }
    }
    ///Input
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MODE5::B0x0
    }
    ///Output
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MODE5::B0x1
    }
    ///Alternate function
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MODE5::B0x2
    }
    ///Analog
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MODE5::B0x3
    }
}
///Field `MODE5` writer - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE5_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE5, crate::Safe>;
impl<'a, REG> MODE5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5::B0x0)
    }
    ///Output
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5::B0x1)
    }
    ///Alternate function
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5::B0x2)
    }
    ///Analog
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE6 {
    ///0: Input
    B0x0 = 0,
    ///1: Output
    B0x1 = 1,
    ///2: Alternate function
    B0x2 = 2,
    ///3: Analog
    B0x3 = 3,
}
impl From<MODE6> for u8 {
    #[inline(always)]
    fn from(variant: MODE6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE6 {
    type Ux = u8;
}
impl crate::IsEnum for MODE6 {}
///Field `MODE6` reader - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE6_R = crate::FieldReader<MODE6>;
impl MODE6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODE6 {
        match self.bits {
            0 => MODE6::B0x0,
            1 => MODE6::B0x1,
            2 => MODE6::B0x2,
            3 => MODE6::B0x3,
            _ => unreachable!(),
        }
    }
    ///Input
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MODE6::B0x0
    }
    ///Output
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MODE6::B0x1
    }
    ///Alternate function
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MODE6::B0x2
    }
    ///Analog
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MODE6::B0x3
    }
}
///Field `MODE6` writer - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE6_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE6, crate::Safe>;
impl<'a, REG> MODE6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6::B0x0)
    }
    ///Output
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6::B0x1)
    }
    ///Alternate function
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6::B0x2)
    }
    ///Analog
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE7 {
    ///0: Input
    B0x0 = 0,
    ///1: Output
    B0x1 = 1,
    ///2: Alternate function
    B0x2 = 2,
    ///3: Analog
    B0x3 = 3,
}
impl From<MODE7> for u8 {
    #[inline(always)]
    fn from(variant: MODE7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE7 {
    type Ux = u8;
}
impl crate::IsEnum for MODE7 {}
///Field `MODE7` reader - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE7_R = crate::FieldReader<MODE7>;
impl MODE7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODE7 {
        match self.bits {
            0 => MODE7::B0x0,
            1 => MODE7::B0x1,
            2 => MODE7::B0x2,
            3 => MODE7::B0x3,
            _ => unreachable!(),
        }
    }
    ///Input
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MODE7::B0x0
    }
    ///Output
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MODE7::B0x1
    }
    ///Alternate function
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MODE7::B0x2
    }
    ///Analog
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MODE7::B0x3
    }
}
///Field `MODE7` writer - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE7_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE7, crate::Safe>;
impl<'a, REG> MODE7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7::B0x0)
    }
    ///Output
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7::B0x1)
    }
    ///Alternate function
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7::B0x2)
    }
    ///Analog
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE8 {
    ///0: Input
    B0x0 = 0,
    ///1: Output
    B0x1 = 1,
    ///2: Alternate function
    B0x2 = 2,
    ///3: Analog
    B0x3 = 3,
}
impl From<MODE8> for u8 {
    #[inline(always)]
    fn from(variant: MODE8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE8 {
    type Ux = u8;
}
impl crate::IsEnum for MODE8 {}
///Field `MODE8` reader - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE8_R = crate::FieldReader<MODE8>;
impl MODE8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODE8 {
        match self.bits {
            0 => MODE8::B0x0,
            1 => MODE8::B0x1,
            2 => MODE8::B0x2,
            3 => MODE8::B0x3,
            _ => unreachable!(),
        }
    }
    ///Input
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MODE8::B0x0
    }
    ///Output
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MODE8::B0x1
    }
    ///Alternate function
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MODE8::B0x2
    }
    ///Analog
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MODE8::B0x3
    }
}
///Field `MODE8` writer - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE8_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE8, crate::Safe>;
impl<'a, REG> MODE8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8::B0x0)
    }
    ///Output
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8::B0x1)
    }
    ///Alternate function
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8::B0x2)
    }
    ///Analog
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE9 {
    ///0: Input
    B0x0 = 0,
    ///1: Output
    B0x1 = 1,
    ///2: Alternate function
    B0x2 = 2,
    ///3: Analog
    B0x3 = 3,
}
impl From<MODE9> for u8 {
    #[inline(always)]
    fn from(variant: MODE9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE9 {
    type Ux = u8;
}
impl crate::IsEnum for MODE9 {}
///Field `MODE9` reader - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE9_R = crate::FieldReader<MODE9>;
impl MODE9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODE9 {
        match self.bits {
            0 => MODE9::B0x0,
            1 => MODE9::B0x1,
            2 => MODE9::B0x2,
            3 => MODE9::B0x3,
            _ => unreachable!(),
        }
    }
    ///Input
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MODE9::B0x0
    }
    ///Output
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MODE9::B0x1
    }
    ///Alternate function
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MODE9::B0x2
    }
    ///Analog
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MODE9::B0x3
    }
}
///Field `MODE9` writer - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE9_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE9, crate::Safe>;
impl<'a, REG> MODE9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE9::B0x0)
    }
    ///Output
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE9::B0x1)
    }
    ///Alternate function
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE9::B0x2)
    }
    ///Analog
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE9::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE10 {
    ///0: Input
    B0x0 = 0,
    ///1: Output
    B0x1 = 1,
    ///2: Alternate function
    B0x2 = 2,
    ///3: Analog
    B0x3 = 3,
}
impl From<MODE10> for u8 {
    #[inline(always)]
    fn from(variant: MODE10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE10 {
    type Ux = u8;
}
impl crate::IsEnum for MODE10 {}
///Field `MODE10` reader - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE10_R = crate::FieldReader<MODE10>;
impl MODE10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODE10 {
        match self.bits {
            0 => MODE10::B0x0,
            1 => MODE10::B0x1,
            2 => MODE10::B0x2,
            3 => MODE10::B0x3,
            _ => unreachable!(),
        }
    }
    ///Input
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MODE10::B0x0
    }
    ///Output
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MODE10::B0x1
    }
    ///Alternate function
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MODE10::B0x2
    }
    ///Analog
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MODE10::B0x3
    }
}
///Field `MODE10` writer - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE10_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE10, crate::Safe>;
impl<'a, REG> MODE10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE10::B0x0)
    }
    ///Output
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE10::B0x1)
    }
    ///Alternate function
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE10::B0x2)
    }
    ///Analog
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE10::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE11 {
    ///0: Input
    B0x0 = 0,
    ///1: Output
    B0x1 = 1,
    ///2: Alternate function
    B0x2 = 2,
    ///3: Analog
    B0x3 = 3,
}
impl From<MODE11> for u8 {
    #[inline(always)]
    fn from(variant: MODE11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE11 {
    type Ux = u8;
}
impl crate::IsEnum for MODE11 {}
///Field `MODE11` reader - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE11_R = crate::FieldReader<MODE11>;
impl MODE11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODE11 {
        match self.bits {
            0 => MODE11::B0x0,
            1 => MODE11::B0x1,
            2 => MODE11::B0x2,
            3 => MODE11::B0x3,
            _ => unreachable!(),
        }
    }
    ///Input
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MODE11::B0x0
    }
    ///Output
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MODE11::B0x1
    }
    ///Alternate function
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MODE11::B0x2
    }
    ///Analog
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MODE11::B0x3
    }
}
///Field `MODE11` writer - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE11_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE11, crate::Safe>;
impl<'a, REG> MODE11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE11::B0x0)
    }
    ///Output
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE11::B0x1)
    }
    ///Alternate function
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE11::B0x2)
    }
    ///Analog
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE11::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE12 {
    ///0: Input
    B0x0 = 0,
    ///1: Output
    B0x1 = 1,
    ///2: Alternate function
    B0x2 = 2,
    ///3: Analog
    B0x3 = 3,
}
impl From<MODE12> for u8 {
    #[inline(always)]
    fn from(variant: MODE12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE12 {
    type Ux = u8;
}
impl crate::IsEnum for MODE12 {}
///Field `MODE12` reader - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE12_R = crate::FieldReader<MODE12>;
impl MODE12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODE12 {
        match self.bits {
            0 => MODE12::B0x0,
            1 => MODE12::B0x1,
            2 => MODE12::B0x2,
            3 => MODE12::B0x3,
            _ => unreachable!(),
        }
    }
    ///Input
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MODE12::B0x0
    }
    ///Output
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MODE12::B0x1
    }
    ///Alternate function
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MODE12::B0x2
    }
    ///Analog
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MODE12::B0x3
    }
}
///Field `MODE12` writer - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE12_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE12, crate::Safe>;
impl<'a, REG> MODE12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE12::B0x0)
    }
    ///Output
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE12::B0x1)
    }
    ///Alternate function
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE12::B0x2)
    }
    ///Analog
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE12::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE13 {
    ///0: Input
    B0x0 = 0,
    ///1: Output
    B0x1 = 1,
    ///2: Alternate function
    B0x2 = 2,
    ///3: Analog
    B0x3 = 3,
}
impl From<MODE13> for u8 {
    #[inline(always)]
    fn from(variant: MODE13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE13 {
    type Ux = u8;
}
impl crate::IsEnum for MODE13 {}
///Field `MODE13` reader - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE13_R = crate::FieldReader<MODE13>;
impl MODE13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODE13 {
        match self.bits {
            0 => MODE13::B0x0,
            1 => MODE13::B0x1,
            2 => MODE13::B0x2,
            3 => MODE13::B0x3,
            _ => unreachable!(),
        }
    }
    ///Input
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MODE13::B0x0
    }
    ///Output
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MODE13::B0x1
    }
    ///Alternate function
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MODE13::B0x2
    }
    ///Analog
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MODE13::B0x3
    }
}
///Field `MODE13` writer - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE13_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE13, crate::Safe>;
impl<'a, REG> MODE13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE13::B0x0)
    }
    ///Output
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE13::B0x1)
    }
    ///Alternate function
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE13::B0x2)
    }
    ///Analog
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE13::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE14 {
    ///0: Input
    B0x0 = 0,
    ///1: Output
    B0x1 = 1,
    ///2: Alternate function
    B0x2 = 2,
    ///3: Analog
    B0x3 = 3,
}
impl From<MODE14> for u8 {
    #[inline(always)]
    fn from(variant: MODE14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE14 {
    type Ux = u8;
}
impl crate::IsEnum for MODE14 {}
///Field `MODE14` reader - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE14_R = crate::FieldReader<MODE14>;
impl MODE14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODE14 {
        match self.bits {
            0 => MODE14::B0x0,
            1 => MODE14::B0x1,
            2 => MODE14::B0x2,
            3 => MODE14::B0x3,
            _ => unreachable!(),
        }
    }
    ///Input
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MODE14::B0x0
    }
    ///Output
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MODE14::B0x1
    }
    ///Alternate function
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MODE14::B0x2
    }
    ///Analog
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MODE14::B0x3
    }
}
///Field `MODE14` writer - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE14_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE14, crate::Safe>;
impl<'a, REG> MODE14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE14::B0x0)
    }
    ///Output
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE14::B0x1)
    }
    ///Alternate function
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE14::B0x2)
    }
    ///Analog
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE14::B0x3)
    }
}
/**Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE15 {
    ///0: Input
    B0x0 = 0,
    ///1: Output
    B0x1 = 1,
    ///2: Alternate function
    B0x2 = 2,
    ///3: Analog
    B0x3 = 3,
}
impl From<MODE15> for u8 {
    #[inline(always)]
    fn from(variant: MODE15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE15 {
    type Ux = u8;
}
impl crate::IsEnum for MODE15 {}
///Field `MODE15` reader - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE15_R = crate::FieldReader<MODE15>;
impl MODE15_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODE15 {
        match self.bits {
            0 => MODE15::B0x0,
            1 => MODE15::B0x1,
            2 => MODE15::B0x2,
            3 => MODE15::B0x3,
            _ => unreachable!(),
        }
    }
    ///Input
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MODE15::B0x0
    }
    ///Output
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MODE15::B0x1
    }
    ///Alternate function
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MODE15::B0x2
    }
    ///Analog
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MODE15::B0x3
    }
}
///Field `MODE15` writer - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
pub type MODE15_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE15, crate::Safe>;
impl<'a, REG> MODE15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE15::B0x0)
    }
    ///Output
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE15::B0x1)
    }
    ///Alternate function
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE15::B0x2)
    }
    ///Analog
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE15::B0x3)
    }
}
impl R {
    ///Bits 0:1 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode4(&self) -> MODE4_R {
        MODE4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode5(&self) -> MODE5_R {
        MODE5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode6(&self) -> MODE6_R {
        MODE6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode7(&self) -> MODE7_R {
        MODE7_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode8(&self) -> MODE8_R {
        MODE8_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode9(&self) -> MODE9_R {
        MODE9_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode10(&self) -> MODE10_R {
        MODE10_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode11(&self) -> MODE11_R {
        MODE11_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode12(&self) -> MODE12_R {
        MODE12_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode13(&self) -> MODE13_R {
        MODE13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode14(&self) -> MODE14_R {
        MODE14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode15(&self) -> MODE15_R {
        MODE15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOF_MODER")
            .field("mode0", &self.mode0())
            .field("mode1", &self.mode1())
            .field("mode2", &self.mode2())
            .field("mode3", &self.mode3())
            .field("mode4", &self.mode4())
            .field("mode5", &self.mode5())
            .field("mode6", &self.mode6())
            .field("mode7", &self.mode7())
            .field("mode8", &self.mode8())
            .field("mode9", &self.mode9())
            .field("mode10", &self.mode10())
            .field("mode11", &self.mode11())
            .field("mode12", &self.mode12())
            .field("mode13", &self.mode13())
            .field("mode14", &self.mode14())
            .field("mode15", &self.mode15())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode0(&mut self) -> MODE0_W<'_, GPIOF_MODERrs> {
        MODE0_W::new(self, 0)
    }
    ///Bits 2:3 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W<'_, GPIOF_MODERrs> {
        MODE1_W::new(self, 2)
    }
    ///Bits 4:5 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE2_W<'_, GPIOF_MODERrs> {
        MODE2_W::new(self, 4)
    }
    ///Bits 6:7 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode3(&mut self) -> MODE3_W<'_, GPIOF_MODERrs> {
        MODE3_W::new(self, 6)
    }
    ///Bits 8:9 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode4(&mut self) -> MODE4_W<'_, GPIOF_MODERrs> {
        MODE4_W::new(self, 8)
    }
    ///Bits 10:11 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode5(&mut self) -> MODE5_W<'_, GPIOF_MODERrs> {
        MODE5_W::new(self, 10)
    }
    ///Bits 12:13 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode6(&mut self) -> MODE6_W<'_, GPIOF_MODERrs> {
        MODE6_W::new(self, 12)
    }
    ///Bits 14:15 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode7(&mut self) -> MODE7_W<'_, GPIOF_MODERrs> {
        MODE7_W::new(self, 14)
    }
    ///Bits 16:17 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode8(&mut self) -> MODE8_W<'_, GPIOF_MODERrs> {
        MODE8_W::new(self, 16)
    }
    ///Bits 18:19 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode9(&mut self) -> MODE9_W<'_, GPIOF_MODERrs> {
        MODE9_W::new(self, 18)
    }
    ///Bits 20:21 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode10(&mut self) -> MODE10_W<'_, GPIOF_MODERrs> {
        MODE10_W::new(self, 20)
    }
    ///Bits 22:23 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode11(&mut self) -> MODE11_W<'_, GPIOF_MODERrs> {
        MODE11_W::new(self, 22)
    }
    ///Bits 24:25 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode12(&mut self) -> MODE12_W<'_, GPIOF_MODERrs> {
        MODE12_W::new(self, 24)
    }
    ///Bits 26:27 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode13(&mut self) -> MODE13_W<'_, GPIOF_MODERrs> {
        MODE13_W::new(self, 26)
    }
    ///Bits 28:29 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode14(&mut self) -> MODE14_W<'_, GPIOF_MODERrs> {
        MODE14_W::new(self, 28)
    }
    ///Bits 30:31 - Port x configuration for I/O y These bits are written by software to set the I/O to one of four operating modes.
    #[inline(always)]
    pub fn mode15(&mut self) -> MODE15_W<'_, GPIOF_MODERrs> {
        MODE15_W::new(self, 30)
    }
}
/**GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`gpiof_moder::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiof_moder::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#GPIOF:GPIOF_MODER)*/
pub struct GPIOF_MODERrs;
impl crate::RegisterSpec for GPIOF_MODERrs {
    type Ux = u32;
}
///`read()` method returns [`gpiof_moder::R`](R) reader structure
impl crate::Readable for GPIOF_MODERrs {}
///`write(|w| ..)` method takes [`gpiof_moder::W`](W) writer structure
impl crate::Writable for GPIOF_MODERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPIOF_MODER to value 0xffff_ffff
impl crate::Resettable for GPIOF_MODERrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
