///Register `GPIOB_OTYPER` reader
pub type R = crate::R<GPIOB_OTYPERrs>;
///Register `GPIOB_OTYPER` writer
pub type W = crate::W<GPIOB_OTYPERrs>;
/**Port x configuration for I/O y These bits are written by software to configure the I/O output type.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT0 {
    ///0: Output push-pull (reset state)
    B0x0 = 0,
    ///1: Output open-drain
    B0x1 = 1,
}
impl From<OT0> for bool {
    #[inline(always)]
    fn from(variant: OT0) -> Self {
        variant as u8 != 0
    }
}
///Field `OT0` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT0_R = crate::BitReader<OT0>;
impl OT0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OT0 {
        match self.bits {
            false => OT0::B0x0,
            true => OT0::B0x1,
        }
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OT0::B0x0
    }
    ///Output open-drain
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OT0::B0x1
    }
}
///Field `OT0` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT0_W<'a, REG> = crate::BitWriter<'a, REG, OT0>;
impl<'a, REG> OT0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT0::B0x0)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT0::B0x1)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output type.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT1 {
    ///0: Output push-pull (reset state)
    B0x0 = 0,
    ///1: Output open-drain
    B0x1 = 1,
}
impl From<OT1> for bool {
    #[inline(always)]
    fn from(variant: OT1) -> Self {
        variant as u8 != 0
    }
}
///Field `OT1` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT1_R = crate::BitReader<OT1>;
impl OT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OT1 {
        match self.bits {
            false => OT1::B0x0,
            true => OT1::B0x1,
        }
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OT1::B0x0
    }
    ///Output open-drain
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OT1::B0x1
    }
}
///Field `OT1` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT1_W<'a, REG> = crate::BitWriter<'a, REG, OT1>;
impl<'a, REG> OT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT1::B0x0)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT1::B0x1)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output type.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT2 {
    ///0: Output push-pull (reset state)
    B0x0 = 0,
    ///1: Output open-drain
    B0x1 = 1,
}
impl From<OT2> for bool {
    #[inline(always)]
    fn from(variant: OT2) -> Self {
        variant as u8 != 0
    }
}
///Field `OT2` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT2_R = crate::BitReader<OT2>;
impl OT2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OT2 {
        match self.bits {
            false => OT2::B0x0,
            true => OT2::B0x1,
        }
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OT2::B0x0
    }
    ///Output open-drain
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OT2::B0x1
    }
}
///Field `OT2` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT2_W<'a, REG> = crate::BitWriter<'a, REG, OT2>;
impl<'a, REG> OT2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT2::B0x0)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT2::B0x1)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output type.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT3 {
    ///0: Output push-pull (reset state)
    B0x0 = 0,
    ///1: Output open-drain
    B0x1 = 1,
}
impl From<OT3> for bool {
    #[inline(always)]
    fn from(variant: OT3) -> Self {
        variant as u8 != 0
    }
}
///Field `OT3` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT3_R = crate::BitReader<OT3>;
impl OT3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OT3 {
        match self.bits {
            false => OT3::B0x0,
            true => OT3::B0x1,
        }
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OT3::B0x0
    }
    ///Output open-drain
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OT3::B0x1
    }
}
///Field `OT3` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT3_W<'a, REG> = crate::BitWriter<'a, REG, OT3>;
impl<'a, REG> OT3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT3::B0x0)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT3::B0x1)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output type.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT4 {
    ///0: Output push-pull (reset state)
    B0x0 = 0,
    ///1: Output open-drain
    B0x1 = 1,
}
impl From<OT4> for bool {
    #[inline(always)]
    fn from(variant: OT4) -> Self {
        variant as u8 != 0
    }
}
///Field `OT4` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT4_R = crate::BitReader<OT4>;
impl OT4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OT4 {
        match self.bits {
            false => OT4::B0x0,
            true => OT4::B0x1,
        }
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OT4::B0x0
    }
    ///Output open-drain
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OT4::B0x1
    }
}
///Field `OT4` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT4_W<'a, REG> = crate::BitWriter<'a, REG, OT4>;
impl<'a, REG> OT4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT4::B0x0)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT4::B0x1)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output type.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT5 {
    ///0: Output push-pull (reset state)
    B0x0 = 0,
    ///1: Output open-drain
    B0x1 = 1,
}
impl From<OT5> for bool {
    #[inline(always)]
    fn from(variant: OT5) -> Self {
        variant as u8 != 0
    }
}
///Field `OT5` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT5_R = crate::BitReader<OT5>;
impl OT5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OT5 {
        match self.bits {
            false => OT5::B0x0,
            true => OT5::B0x1,
        }
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OT5::B0x0
    }
    ///Output open-drain
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OT5::B0x1
    }
}
///Field `OT5` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT5_W<'a, REG> = crate::BitWriter<'a, REG, OT5>;
impl<'a, REG> OT5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT5::B0x0)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT5::B0x1)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output type.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT6 {
    ///0: Output push-pull (reset state)
    B0x0 = 0,
    ///1: Output open-drain
    B0x1 = 1,
}
impl From<OT6> for bool {
    #[inline(always)]
    fn from(variant: OT6) -> Self {
        variant as u8 != 0
    }
}
///Field `OT6` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT6_R = crate::BitReader<OT6>;
impl OT6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OT6 {
        match self.bits {
            false => OT6::B0x0,
            true => OT6::B0x1,
        }
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OT6::B0x0
    }
    ///Output open-drain
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OT6::B0x1
    }
}
///Field `OT6` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT6_W<'a, REG> = crate::BitWriter<'a, REG, OT6>;
impl<'a, REG> OT6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT6::B0x0)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT6::B0x1)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output type.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT7 {
    ///0: Output push-pull (reset state)
    B0x0 = 0,
    ///1: Output open-drain
    B0x1 = 1,
}
impl From<OT7> for bool {
    #[inline(always)]
    fn from(variant: OT7) -> Self {
        variant as u8 != 0
    }
}
///Field `OT7` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT7_R = crate::BitReader<OT7>;
impl OT7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OT7 {
        match self.bits {
            false => OT7::B0x0,
            true => OT7::B0x1,
        }
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OT7::B0x0
    }
    ///Output open-drain
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OT7::B0x1
    }
}
///Field `OT7` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT7_W<'a, REG> = crate::BitWriter<'a, REG, OT7>;
impl<'a, REG> OT7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT7::B0x0)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT7::B0x1)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output type.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT8 {
    ///0: Output push-pull (reset state)
    B0x0 = 0,
    ///1: Output open-drain
    B0x1 = 1,
}
impl From<OT8> for bool {
    #[inline(always)]
    fn from(variant: OT8) -> Self {
        variant as u8 != 0
    }
}
///Field `OT8` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT8_R = crate::BitReader<OT8>;
impl OT8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OT8 {
        match self.bits {
            false => OT8::B0x0,
            true => OT8::B0x1,
        }
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OT8::B0x0
    }
    ///Output open-drain
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OT8::B0x1
    }
}
///Field `OT8` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT8_W<'a, REG> = crate::BitWriter<'a, REG, OT8>;
impl<'a, REG> OT8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT8::B0x0)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT8::B0x1)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output type.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT9 {
    ///0: Output push-pull (reset state)
    B0x0 = 0,
    ///1: Output open-drain
    B0x1 = 1,
}
impl From<OT9> for bool {
    #[inline(always)]
    fn from(variant: OT9) -> Self {
        variant as u8 != 0
    }
}
///Field `OT9` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT9_R = crate::BitReader<OT9>;
impl OT9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OT9 {
        match self.bits {
            false => OT9::B0x0,
            true => OT9::B0x1,
        }
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OT9::B0x0
    }
    ///Output open-drain
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OT9::B0x1
    }
}
///Field `OT9` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT9_W<'a, REG> = crate::BitWriter<'a, REG, OT9>;
impl<'a, REG> OT9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT9::B0x0)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT9::B0x1)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output type.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT10 {
    ///0: Output push-pull (reset state)
    B0x0 = 0,
    ///1: Output open-drain
    B0x1 = 1,
}
impl From<OT10> for bool {
    #[inline(always)]
    fn from(variant: OT10) -> Self {
        variant as u8 != 0
    }
}
///Field `OT10` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT10_R = crate::BitReader<OT10>;
impl OT10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OT10 {
        match self.bits {
            false => OT10::B0x0,
            true => OT10::B0x1,
        }
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OT10::B0x0
    }
    ///Output open-drain
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OT10::B0x1
    }
}
///Field `OT10` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT10_W<'a, REG> = crate::BitWriter<'a, REG, OT10>;
impl<'a, REG> OT10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT10::B0x0)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT10::B0x1)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output type.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT11 {
    ///0: Output push-pull (reset state)
    B0x0 = 0,
    ///1: Output open-drain
    B0x1 = 1,
}
impl From<OT11> for bool {
    #[inline(always)]
    fn from(variant: OT11) -> Self {
        variant as u8 != 0
    }
}
///Field `OT11` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT11_R = crate::BitReader<OT11>;
impl OT11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OT11 {
        match self.bits {
            false => OT11::B0x0,
            true => OT11::B0x1,
        }
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OT11::B0x0
    }
    ///Output open-drain
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OT11::B0x1
    }
}
///Field `OT11` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT11_W<'a, REG> = crate::BitWriter<'a, REG, OT11>;
impl<'a, REG> OT11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT11::B0x0)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT11::B0x1)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output type.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT12 {
    ///0: Output push-pull (reset state)
    B0x0 = 0,
    ///1: Output open-drain
    B0x1 = 1,
}
impl From<OT12> for bool {
    #[inline(always)]
    fn from(variant: OT12) -> Self {
        variant as u8 != 0
    }
}
///Field `OT12` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT12_R = crate::BitReader<OT12>;
impl OT12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OT12 {
        match self.bits {
            false => OT12::B0x0,
            true => OT12::B0x1,
        }
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OT12::B0x0
    }
    ///Output open-drain
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OT12::B0x1
    }
}
///Field `OT12` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT12_W<'a, REG> = crate::BitWriter<'a, REG, OT12>;
impl<'a, REG> OT12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT12::B0x0)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT12::B0x1)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output type.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT13 {
    ///0: Output push-pull (reset state)
    B0x0 = 0,
    ///1: Output open-drain
    B0x1 = 1,
}
impl From<OT13> for bool {
    #[inline(always)]
    fn from(variant: OT13) -> Self {
        variant as u8 != 0
    }
}
///Field `OT13` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT13_R = crate::BitReader<OT13>;
impl OT13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OT13 {
        match self.bits {
            false => OT13::B0x0,
            true => OT13::B0x1,
        }
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OT13::B0x0
    }
    ///Output open-drain
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OT13::B0x1
    }
}
///Field `OT13` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT13_W<'a, REG> = crate::BitWriter<'a, REG, OT13>;
impl<'a, REG> OT13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT13::B0x0)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT13::B0x1)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output type.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT14 {
    ///0: Output push-pull (reset state)
    B0x0 = 0,
    ///1: Output open-drain
    B0x1 = 1,
}
impl From<OT14> for bool {
    #[inline(always)]
    fn from(variant: OT14) -> Self {
        variant as u8 != 0
    }
}
///Field `OT14` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT14_R = crate::BitReader<OT14>;
impl OT14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OT14 {
        match self.bits {
            false => OT14::B0x0,
            true => OT14::B0x1,
        }
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OT14::B0x0
    }
    ///Output open-drain
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OT14::B0x1
    }
}
///Field `OT14` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT14_W<'a, REG> = crate::BitWriter<'a, REG, OT14>;
impl<'a, REG> OT14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT14::B0x0)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT14::B0x1)
    }
}
/**Port x configuration for I/O y These bits are written by software to configure the I/O output type.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT15 {
    ///0: Output push-pull (reset state)
    B0x0 = 0,
    ///1: Output open-drain
    B0x1 = 1,
}
impl From<OT15> for bool {
    #[inline(always)]
    fn from(variant: OT15) -> Self {
        variant as u8 != 0
    }
}
///Field `OT15` reader - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT15_R = crate::BitReader<OT15>;
impl OT15_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OT15 {
        match self.bits {
            false => OT15::B0x0,
            true => OT15::B0x1,
        }
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OT15::B0x0
    }
    ///Output open-drain
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OT15::B0x1
    }
}
///Field `OT15` writer - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
pub type OT15_W<'a, REG> = crate::BitWriter<'a, REG, OT15>;
impl<'a, REG> OT15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT15::B0x0)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT15::B0x1)
    }
}
impl R {
    ///Bit 0 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot0(&self) -> OT0_R {
        OT0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot1(&self) -> OT1_R {
        OT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot2(&self) -> OT2_R {
        OT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot3(&self) -> OT3_R {
        OT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot4(&self) -> OT4_R {
        OT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot5(&self) -> OT5_R {
        OT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot6(&self) -> OT6_R {
        OT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot7(&self) -> OT7_R {
        OT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot8(&self) -> OT8_R {
        OT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot9(&self) -> OT9_R {
        OT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot10(&self) -> OT10_R {
        OT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot11(&self) -> OT11_R {
        OT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot12(&self) -> OT12_R {
        OT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot13(&self) -> OT13_R {
        OT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot14(&self) -> OT14_R {
        OT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot15(&self) -> OT15_R {
        OT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOB_OTYPER")
            .field("ot0", &self.ot0())
            .field("ot1", &self.ot1())
            .field("ot2", &self.ot2())
            .field("ot3", &self.ot3())
            .field("ot4", &self.ot4())
            .field("ot5", &self.ot5())
            .field("ot6", &self.ot6())
            .field("ot7", &self.ot7())
            .field("ot8", &self.ot8())
            .field("ot9", &self.ot9())
            .field("ot10", &self.ot10())
            .field("ot11", &self.ot11())
            .field("ot12", &self.ot12())
            .field("ot13", &self.ot13())
            .field("ot14", &self.ot14())
            .field("ot15", &self.ot15())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot0(&mut self) -> OT0_W<'_, GPIOB_OTYPERrs> {
        OT0_W::new(self, 0)
    }
    ///Bit 1 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot1(&mut self) -> OT1_W<'_, GPIOB_OTYPERrs> {
        OT1_W::new(self, 1)
    }
    ///Bit 2 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot2(&mut self) -> OT2_W<'_, GPIOB_OTYPERrs> {
        OT2_W::new(self, 2)
    }
    ///Bit 3 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot3(&mut self) -> OT3_W<'_, GPIOB_OTYPERrs> {
        OT3_W::new(self, 3)
    }
    ///Bit 4 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot4(&mut self) -> OT4_W<'_, GPIOB_OTYPERrs> {
        OT4_W::new(self, 4)
    }
    ///Bit 5 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot5(&mut self) -> OT5_W<'_, GPIOB_OTYPERrs> {
        OT5_W::new(self, 5)
    }
    ///Bit 6 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot6(&mut self) -> OT6_W<'_, GPIOB_OTYPERrs> {
        OT6_W::new(self, 6)
    }
    ///Bit 7 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot7(&mut self) -> OT7_W<'_, GPIOB_OTYPERrs> {
        OT7_W::new(self, 7)
    }
    ///Bit 8 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot8(&mut self) -> OT8_W<'_, GPIOB_OTYPERrs> {
        OT8_W::new(self, 8)
    }
    ///Bit 9 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot9(&mut self) -> OT9_W<'_, GPIOB_OTYPERrs> {
        OT9_W::new(self, 9)
    }
    ///Bit 10 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot10(&mut self) -> OT10_W<'_, GPIOB_OTYPERrs> {
        OT10_W::new(self, 10)
    }
    ///Bit 11 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot11(&mut self) -> OT11_W<'_, GPIOB_OTYPERrs> {
        OT11_W::new(self, 11)
    }
    ///Bit 12 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot12(&mut self) -> OT12_W<'_, GPIOB_OTYPERrs> {
        OT12_W::new(self, 12)
    }
    ///Bit 13 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot13(&mut self) -> OT13_W<'_, GPIOB_OTYPERrs> {
        OT13_W::new(self, 13)
    }
    ///Bit 14 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot14(&mut self) -> OT14_W<'_, GPIOB_OTYPERrs> {
        OT14_W::new(self, 14)
    }
    ///Bit 15 - Port x configuration for I/O y These bits are written by software to configure the I/O output type.
    #[inline(always)]
    pub fn ot15(&mut self) -> OT15_W<'_, GPIOB_OTYPERrs> {
        OT15_W::new(self, 15)
    }
}
/**GPIO port output type register

You can [`read`](crate::Reg::read) this register and get [`gpiob_otyper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob_otyper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#GPIOB:GPIOB_OTYPER)*/
pub struct GPIOB_OTYPERrs;
impl crate::RegisterSpec for GPIOB_OTYPERrs {
    type Ux = u32;
}
///`read()` method returns [`gpiob_otyper::R`](R) reader structure
impl crate::Readable for GPIOB_OTYPERrs {}
///`write(|w| ..)` method takes [`gpiob_otyper::W`](W) writer structure
impl crate::Writable for GPIOB_OTYPERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPIOB_OTYPER to value 0
impl crate::Resettable for GPIOB_OTYPERrs {}
