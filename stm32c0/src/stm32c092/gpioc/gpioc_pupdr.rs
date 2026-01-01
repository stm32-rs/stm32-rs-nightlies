///Register `GPIOC_PUPDR` reader
pub type R = crate::R<GPIOC_PUPDRrs>;
///Register `GPIOC_PUPDR` writer
pub type W = crate::W<GPIOC_PUPDRrs>;
/**Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD0 {
    ///0: No pull-up, pull-down
    B0x0 = 0,
    ///1: Pull-up
    B0x1 = 1,
    ///2: Pull-down
    B0x2 = 2,
}
impl From<PUPD0> for u8 {
    #[inline(always)]
    fn from(variant: PUPD0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD0 {
    type Ux = u8;
}
impl crate::IsEnum for PUPD0 {}
///Field `PUPD0` reader - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD0_R = crate::FieldReader<PUPD0>;
impl PUPD0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD0> {
        match self.bits {
            0 => Some(PUPD0::B0x0),
            1 => Some(PUPD0::B0x1),
            2 => Some(PUPD0::B0x2),
            _ => None,
        }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PUPD0::B0x0
    }
    ///Pull-up
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PUPD0::B0x1
    }
    ///Pull-down
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PUPD0::B0x2
    }
}
///Field `PUPD0` writer - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD0>;
impl<'a, REG> PUPD0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD0::B0x0)
    }
    ///Pull-up
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD0::B0x1)
    }
    ///Pull-down
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD0::B0x2)
    }
}
/**Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD1 {
    ///0: No pull-up, pull-down
    B0x0 = 0,
    ///1: Pull-up
    B0x1 = 1,
    ///2: Pull-down
    B0x2 = 2,
}
impl From<PUPD1> for u8 {
    #[inline(always)]
    fn from(variant: PUPD1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD1 {
    type Ux = u8;
}
impl crate::IsEnum for PUPD1 {}
///Field `PUPD1` reader - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD1_R = crate::FieldReader<PUPD1>;
impl PUPD1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD1> {
        match self.bits {
            0 => Some(PUPD1::B0x0),
            1 => Some(PUPD1::B0x1),
            2 => Some(PUPD1::B0x2),
            _ => None,
        }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PUPD1::B0x0
    }
    ///Pull-up
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PUPD1::B0x1
    }
    ///Pull-down
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PUPD1::B0x2
    }
}
///Field `PUPD1` writer - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD1>;
impl<'a, REG> PUPD1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD1::B0x0)
    }
    ///Pull-up
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD1::B0x1)
    }
    ///Pull-down
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD1::B0x2)
    }
}
/**Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD2 {
    ///0: No pull-up, pull-down
    B0x0 = 0,
    ///1: Pull-up
    B0x1 = 1,
    ///2: Pull-down
    B0x2 = 2,
}
impl From<PUPD2> for u8 {
    #[inline(always)]
    fn from(variant: PUPD2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD2 {
    type Ux = u8;
}
impl crate::IsEnum for PUPD2 {}
///Field `PUPD2` reader - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD2_R = crate::FieldReader<PUPD2>;
impl PUPD2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD2> {
        match self.bits {
            0 => Some(PUPD2::B0x0),
            1 => Some(PUPD2::B0x1),
            2 => Some(PUPD2::B0x2),
            _ => None,
        }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PUPD2::B0x0
    }
    ///Pull-up
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PUPD2::B0x1
    }
    ///Pull-down
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PUPD2::B0x2
    }
}
///Field `PUPD2` writer - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD2>;
impl<'a, REG> PUPD2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD2::B0x0)
    }
    ///Pull-up
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD2::B0x1)
    }
    ///Pull-down
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD2::B0x2)
    }
}
/**Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD3 {
    ///0: No pull-up, pull-down
    B0x0 = 0,
    ///1: Pull-up
    B0x1 = 1,
    ///2: Pull-down
    B0x2 = 2,
}
impl From<PUPD3> for u8 {
    #[inline(always)]
    fn from(variant: PUPD3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD3 {
    type Ux = u8;
}
impl crate::IsEnum for PUPD3 {}
///Field `PUPD3` reader - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD3_R = crate::FieldReader<PUPD3>;
impl PUPD3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD3> {
        match self.bits {
            0 => Some(PUPD3::B0x0),
            1 => Some(PUPD3::B0x1),
            2 => Some(PUPD3::B0x2),
            _ => None,
        }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PUPD3::B0x0
    }
    ///Pull-up
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PUPD3::B0x1
    }
    ///Pull-down
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PUPD3::B0x2
    }
}
///Field `PUPD3` writer - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD3_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD3>;
impl<'a, REG> PUPD3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD3::B0x0)
    }
    ///Pull-up
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD3::B0x1)
    }
    ///Pull-down
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD3::B0x2)
    }
}
/**Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD4 {
    ///0: No pull-up, pull-down
    B0x0 = 0,
    ///1: Pull-up
    B0x1 = 1,
    ///2: Pull-down
    B0x2 = 2,
}
impl From<PUPD4> for u8 {
    #[inline(always)]
    fn from(variant: PUPD4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD4 {
    type Ux = u8;
}
impl crate::IsEnum for PUPD4 {}
///Field `PUPD4` reader - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD4_R = crate::FieldReader<PUPD4>;
impl PUPD4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD4> {
        match self.bits {
            0 => Some(PUPD4::B0x0),
            1 => Some(PUPD4::B0x1),
            2 => Some(PUPD4::B0x2),
            _ => None,
        }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PUPD4::B0x0
    }
    ///Pull-up
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PUPD4::B0x1
    }
    ///Pull-down
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PUPD4::B0x2
    }
}
///Field `PUPD4` writer - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD4_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD4>;
impl<'a, REG> PUPD4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD4::B0x0)
    }
    ///Pull-up
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD4::B0x1)
    }
    ///Pull-down
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD4::B0x2)
    }
}
/**Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD5 {
    ///0: No pull-up, pull-down
    B0x0 = 0,
    ///1: Pull-up
    B0x1 = 1,
    ///2: Pull-down
    B0x2 = 2,
}
impl From<PUPD5> for u8 {
    #[inline(always)]
    fn from(variant: PUPD5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD5 {
    type Ux = u8;
}
impl crate::IsEnum for PUPD5 {}
///Field `PUPD5` reader - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD5_R = crate::FieldReader<PUPD5>;
impl PUPD5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD5> {
        match self.bits {
            0 => Some(PUPD5::B0x0),
            1 => Some(PUPD5::B0x1),
            2 => Some(PUPD5::B0x2),
            _ => None,
        }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PUPD5::B0x0
    }
    ///Pull-up
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PUPD5::B0x1
    }
    ///Pull-down
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PUPD5::B0x2
    }
}
///Field `PUPD5` writer - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD5_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD5>;
impl<'a, REG> PUPD5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD5::B0x0)
    }
    ///Pull-up
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD5::B0x1)
    }
    ///Pull-down
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD5::B0x2)
    }
}
/**Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD6 {
    ///0: No pull-up, pull-down
    B0x0 = 0,
    ///1: Pull-up
    B0x1 = 1,
    ///2: Pull-down
    B0x2 = 2,
}
impl From<PUPD6> for u8 {
    #[inline(always)]
    fn from(variant: PUPD6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD6 {
    type Ux = u8;
}
impl crate::IsEnum for PUPD6 {}
///Field `PUPD6` reader - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD6_R = crate::FieldReader<PUPD6>;
impl PUPD6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD6> {
        match self.bits {
            0 => Some(PUPD6::B0x0),
            1 => Some(PUPD6::B0x1),
            2 => Some(PUPD6::B0x2),
            _ => None,
        }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PUPD6::B0x0
    }
    ///Pull-up
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PUPD6::B0x1
    }
    ///Pull-down
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PUPD6::B0x2
    }
}
///Field `PUPD6` writer - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD6_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD6>;
impl<'a, REG> PUPD6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD6::B0x0)
    }
    ///Pull-up
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD6::B0x1)
    }
    ///Pull-down
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD6::B0x2)
    }
}
/**Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD7 {
    ///0: No pull-up, pull-down
    B0x0 = 0,
    ///1: Pull-up
    B0x1 = 1,
    ///2: Pull-down
    B0x2 = 2,
}
impl From<PUPD7> for u8 {
    #[inline(always)]
    fn from(variant: PUPD7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD7 {
    type Ux = u8;
}
impl crate::IsEnum for PUPD7 {}
///Field `PUPD7` reader - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD7_R = crate::FieldReader<PUPD7>;
impl PUPD7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD7> {
        match self.bits {
            0 => Some(PUPD7::B0x0),
            1 => Some(PUPD7::B0x1),
            2 => Some(PUPD7::B0x2),
            _ => None,
        }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PUPD7::B0x0
    }
    ///Pull-up
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PUPD7::B0x1
    }
    ///Pull-down
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PUPD7::B0x2
    }
}
///Field `PUPD7` writer - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD7_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD7>;
impl<'a, REG> PUPD7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD7::B0x0)
    }
    ///Pull-up
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD7::B0x1)
    }
    ///Pull-down
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD7::B0x2)
    }
}
/**Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD8 {
    ///0: No pull-up, pull-down
    B0x0 = 0,
    ///1: Pull-up
    B0x1 = 1,
    ///2: Pull-down
    B0x2 = 2,
}
impl From<PUPD8> for u8 {
    #[inline(always)]
    fn from(variant: PUPD8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD8 {
    type Ux = u8;
}
impl crate::IsEnum for PUPD8 {}
///Field `PUPD8` reader - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD8_R = crate::FieldReader<PUPD8>;
impl PUPD8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD8> {
        match self.bits {
            0 => Some(PUPD8::B0x0),
            1 => Some(PUPD8::B0x1),
            2 => Some(PUPD8::B0x2),
            _ => None,
        }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PUPD8::B0x0
    }
    ///Pull-up
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PUPD8::B0x1
    }
    ///Pull-down
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PUPD8::B0x2
    }
}
///Field `PUPD8` writer - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD8_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD8>;
impl<'a, REG> PUPD8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD8::B0x0)
    }
    ///Pull-up
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD8::B0x1)
    }
    ///Pull-down
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD8::B0x2)
    }
}
/**Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD9 {
    ///0: No pull-up, pull-down
    B0x0 = 0,
    ///1: Pull-up
    B0x1 = 1,
    ///2: Pull-down
    B0x2 = 2,
}
impl From<PUPD9> for u8 {
    #[inline(always)]
    fn from(variant: PUPD9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD9 {
    type Ux = u8;
}
impl crate::IsEnum for PUPD9 {}
///Field `PUPD9` reader - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD9_R = crate::FieldReader<PUPD9>;
impl PUPD9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD9> {
        match self.bits {
            0 => Some(PUPD9::B0x0),
            1 => Some(PUPD9::B0x1),
            2 => Some(PUPD9::B0x2),
            _ => None,
        }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PUPD9::B0x0
    }
    ///Pull-up
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PUPD9::B0x1
    }
    ///Pull-down
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PUPD9::B0x2
    }
}
///Field `PUPD9` writer - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD9_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD9>;
impl<'a, REG> PUPD9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD9::B0x0)
    }
    ///Pull-up
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD9::B0x1)
    }
    ///Pull-down
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD9::B0x2)
    }
}
/**Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD10 {
    ///0: No pull-up, pull-down
    B0x0 = 0,
    ///1: Pull-up
    B0x1 = 1,
    ///2: Pull-down
    B0x2 = 2,
}
impl From<PUPD10> for u8 {
    #[inline(always)]
    fn from(variant: PUPD10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD10 {
    type Ux = u8;
}
impl crate::IsEnum for PUPD10 {}
///Field `PUPD10` reader - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD10_R = crate::FieldReader<PUPD10>;
impl PUPD10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD10> {
        match self.bits {
            0 => Some(PUPD10::B0x0),
            1 => Some(PUPD10::B0x1),
            2 => Some(PUPD10::B0x2),
            _ => None,
        }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PUPD10::B0x0
    }
    ///Pull-up
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PUPD10::B0x1
    }
    ///Pull-down
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PUPD10::B0x2
    }
}
///Field `PUPD10` writer - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD10_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD10>;
impl<'a, REG> PUPD10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD10::B0x0)
    }
    ///Pull-up
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD10::B0x1)
    }
    ///Pull-down
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD10::B0x2)
    }
}
/**Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD11 {
    ///0: No pull-up, pull-down
    B0x0 = 0,
    ///1: Pull-up
    B0x1 = 1,
    ///2: Pull-down
    B0x2 = 2,
}
impl From<PUPD11> for u8 {
    #[inline(always)]
    fn from(variant: PUPD11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD11 {
    type Ux = u8;
}
impl crate::IsEnum for PUPD11 {}
///Field `PUPD11` reader - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD11_R = crate::FieldReader<PUPD11>;
impl PUPD11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD11> {
        match self.bits {
            0 => Some(PUPD11::B0x0),
            1 => Some(PUPD11::B0x1),
            2 => Some(PUPD11::B0x2),
            _ => None,
        }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PUPD11::B0x0
    }
    ///Pull-up
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PUPD11::B0x1
    }
    ///Pull-down
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PUPD11::B0x2
    }
}
///Field `PUPD11` writer - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD11_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD11>;
impl<'a, REG> PUPD11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD11::B0x0)
    }
    ///Pull-up
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD11::B0x1)
    }
    ///Pull-down
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD11::B0x2)
    }
}
/**Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD12 {
    ///0: No pull-up, pull-down
    B0x0 = 0,
    ///1: Pull-up
    B0x1 = 1,
    ///2: Pull-down
    B0x2 = 2,
}
impl From<PUPD12> for u8 {
    #[inline(always)]
    fn from(variant: PUPD12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD12 {
    type Ux = u8;
}
impl crate::IsEnum for PUPD12 {}
///Field `PUPD12` reader - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD12_R = crate::FieldReader<PUPD12>;
impl PUPD12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD12> {
        match self.bits {
            0 => Some(PUPD12::B0x0),
            1 => Some(PUPD12::B0x1),
            2 => Some(PUPD12::B0x2),
            _ => None,
        }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PUPD12::B0x0
    }
    ///Pull-up
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PUPD12::B0x1
    }
    ///Pull-down
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PUPD12::B0x2
    }
}
///Field `PUPD12` writer - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD12_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD12>;
impl<'a, REG> PUPD12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD12::B0x0)
    }
    ///Pull-up
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD12::B0x1)
    }
    ///Pull-down
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD12::B0x2)
    }
}
/**Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD13 {
    ///0: No pull-up, pull-down
    B0x0 = 0,
    ///1: Pull-up
    B0x1 = 1,
    ///2: Pull-down
    B0x2 = 2,
}
impl From<PUPD13> for u8 {
    #[inline(always)]
    fn from(variant: PUPD13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD13 {
    type Ux = u8;
}
impl crate::IsEnum for PUPD13 {}
///Field `PUPD13` reader - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD13_R = crate::FieldReader<PUPD13>;
impl PUPD13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD13> {
        match self.bits {
            0 => Some(PUPD13::B0x0),
            1 => Some(PUPD13::B0x1),
            2 => Some(PUPD13::B0x2),
            _ => None,
        }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PUPD13::B0x0
    }
    ///Pull-up
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PUPD13::B0x1
    }
    ///Pull-down
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PUPD13::B0x2
    }
}
///Field `PUPD13` writer - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD13_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD13>;
impl<'a, REG> PUPD13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD13::B0x0)
    }
    ///Pull-up
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD13::B0x1)
    }
    ///Pull-down
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD13::B0x2)
    }
}
/**Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD14 {
    ///0: No pull-up, pull-down
    B0x0 = 0,
    ///1: Pull-up
    B0x1 = 1,
    ///2: Pull-down
    B0x2 = 2,
}
impl From<PUPD14> for u8 {
    #[inline(always)]
    fn from(variant: PUPD14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD14 {
    type Ux = u8;
}
impl crate::IsEnum for PUPD14 {}
///Field `PUPD14` reader - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD14_R = crate::FieldReader<PUPD14>;
impl PUPD14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD14> {
        match self.bits {
            0 => Some(PUPD14::B0x0),
            1 => Some(PUPD14::B0x1),
            2 => Some(PUPD14::B0x2),
            _ => None,
        }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PUPD14::B0x0
    }
    ///Pull-up
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PUPD14::B0x1
    }
    ///Pull-down
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PUPD14::B0x2
    }
}
///Field `PUPD14` writer - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD14_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD14>;
impl<'a, REG> PUPD14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD14::B0x0)
    }
    ///Pull-up
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD14::B0x1)
    }
    ///Pull-down
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD14::B0x2)
    }
}
/**Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD15 {
    ///0: No pull-up, pull-down
    B0x0 = 0,
    ///1: Pull-up
    B0x1 = 1,
    ///2: Pull-down
    B0x2 = 2,
}
impl From<PUPD15> for u8 {
    #[inline(always)]
    fn from(variant: PUPD15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD15 {
    type Ux = u8;
}
impl crate::IsEnum for PUPD15 {}
///Field `PUPD15` reader - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD15_R = crate::FieldReader<PUPD15>;
impl PUPD15_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD15> {
        match self.bits {
            0 => Some(PUPD15::B0x0),
            1 => Some(PUPD15::B0x1),
            2 => Some(PUPD15::B0x2),
            _ => None,
        }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PUPD15::B0x0
    }
    ///Pull-up
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PUPD15::B0x1
    }
    ///Pull-down
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PUPD15::B0x2
    }
}
///Field `PUPD15` writer - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
pub type PUPD15_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD15>;
impl<'a, REG> PUPD15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD15::B0x0)
    }
    ///Pull-up
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD15::B0x1)
    }
    ///Pull-down
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD15::B0x2)
    }
}
impl R {
    ///Bits 0:1 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd0(&self) -> PUPD0_R {
        PUPD0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd1(&self) -> PUPD1_R {
        PUPD1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd2(&self) -> PUPD2_R {
        PUPD2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd3(&self) -> PUPD3_R {
        PUPD3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd4(&self) -> PUPD4_R {
        PUPD4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd5(&self) -> PUPD5_R {
        PUPD5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd6(&self) -> PUPD6_R {
        PUPD6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd7(&self) -> PUPD7_R {
        PUPD7_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd8(&self) -> PUPD8_R {
        PUPD8_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd9(&self) -> PUPD9_R {
        PUPD9_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd10(&self) -> PUPD10_R {
        PUPD10_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd11(&self) -> PUPD11_R {
        PUPD11_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd12(&self) -> PUPD12_R {
        PUPD12_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd13(&self) -> PUPD13_R {
        PUPD13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd14(&self) -> PUPD14_R {
        PUPD14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd15(&self) -> PUPD15_R {
        PUPD15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOC_PUPDR")
            .field("pupd0", &self.pupd0())
            .field("pupd1", &self.pupd1())
            .field("pupd2", &self.pupd2())
            .field("pupd3", &self.pupd3())
            .field("pupd4", &self.pupd4())
            .field("pupd5", &self.pupd5())
            .field("pupd6", &self.pupd6())
            .field("pupd7", &self.pupd7())
            .field("pupd8", &self.pupd8())
            .field("pupd9", &self.pupd9())
            .field("pupd10", &self.pupd10())
            .field("pupd11", &self.pupd11())
            .field("pupd12", &self.pupd12())
            .field("pupd13", &self.pupd13())
            .field("pupd14", &self.pupd14())
            .field("pupd15", &self.pupd15())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd0(&mut self) -> PUPD0_W<'_, GPIOC_PUPDRrs> {
        PUPD0_W::new(self, 0)
    }
    ///Bits 2:3 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd1(&mut self) -> PUPD1_W<'_, GPIOC_PUPDRrs> {
        PUPD1_W::new(self, 2)
    }
    ///Bits 4:5 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd2(&mut self) -> PUPD2_W<'_, GPIOC_PUPDRrs> {
        PUPD2_W::new(self, 4)
    }
    ///Bits 6:7 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd3(&mut self) -> PUPD3_W<'_, GPIOC_PUPDRrs> {
        PUPD3_W::new(self, 6)
    }
    ///Bits 8:9 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd4(&mut self) -> PUPD4_W<'_, GPIOC_PUPDRrs> {
        PUPD4_W::new(self, 8)
    }
    ///Bits 10:11 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd5(&mut self) -> PUPD5_W<'_, GPIOC_PUPDRrs> {
        PUPD5_W::new(self, 10)
    }
    ///Bits 12:13 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd6(&mut self) -> PUPD6_W<'_, GPIOC_PUPDRrs> {
        PUPD6_W::new(self, 12)
    }
    ///Bits 14:15 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd7(&mut self) -> PUPD7_W<'_, GPIOC_PUPDRrs> {
        PUPD7_W::new(self, 14)
    }
    ///Bits 16:17 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd8(&mut self) -> PUPD8_W<'_, GPIOC_PUPDRrs> {
        PUPD8_W::new(self, 16)
    }
    ///Bits 18:19 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd9(&mut self) -> PUPD9_W<'_, GPIOC_PUPDRrs> {
        PUPD9_W::new(self, 18)
    }
    ///Bits 20:21 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd10(&mut self) -> PUPD10_W<'_, GPIOC_PUPDRrs> {
        PUPD10_W::new(self, 20)
    }
    ///Bits 22:23 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd11(&mut self) -> PUPD11_W<'_, GPIOC_PUPDRrs> {
        PUPD11_W::new(self, 22)
    }
    ///Bits 24:25 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd12(&mut self) -> PUPD12_W<'_, GPIOC_PUPDRrs> {
        PUPD12_W::new(self, 24)
    }
    ///Bits 26:27 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd13(&mut self) -> PUPD13_W<'_, GPIOC_PUPDRrs> {
        PUPD13_W::new(self, 26)
    }
    ///Bits 28:29 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd14(&mut self) -> PUPD14_W<'_, GPIOC_PUPDRrs> {
        PUPD14_W::new(self, 28)
    }
    ///Bits 30:31 - Port x configuration I/O y These bits are written by software to configure the I/O pull-up or pull-down Note: On the same pin, this pull up/down must not be activated when a pull down/up is set through the PWR_PDCRx/PWR_PUCRx registers.
    #[inline(always)]
    pub fn pupd15(&mut self) -> PUPD15_W<'_, GPIOC_PUPDRrs> {
        PUPD15_W::new(self, 30)
    }
}
/**GPIO port pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`gpioc_pupdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc_pupdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#GPIOC:GPIOC_PUPDR)*/
pub struct GPIOC_PUPDRrs;
impl crate::RegisterSpec for GPIOC_PUPDRrs {
    type Ux = u32;
}
///`read()` method returns [`gpioc_pupdr::R`](R) reader structure
impl crate::Readable for GPIOC_PUPDRrs {}
///`write(|w| ..)` method takes [`gpioc_pupdr::W`](W) writer structure
impl crate::Writable for GPIOC_PUPDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPIOC_PUPDR to value 0
impl crate::Resettable for GPIOC_PUPDRrs {}
