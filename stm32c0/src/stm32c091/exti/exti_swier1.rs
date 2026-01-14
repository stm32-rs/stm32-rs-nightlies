///Register `EXTI_SWIER1` reader
pub type R = crate::R<EXTI_SWIER1rs>;
///Register `EXTI_SWIER1` writer
pub type W = crate::W<EXTI_SWIER1rs>;
/**Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI0 {
    ///0: No effect
    B0x0 = 0,
    ///1: Rising edge event generated on the corresponding line, followed by an interrupt
    B0x1 = 1,
}
impl From<SWI0> for bool {
    #[inline(always)]
    fn from(variant: SWI0) -> Self {
        variant as u8 != 0
    }
}
///Field `SWI0` reader - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI0_R = crate::BitReader<SWI0>;
impl SWI0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWI0 {
        match self.bits {
            false => SWI0::B0x0,
            true => SWI0::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SWI0::B0x0
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SWI0::B0x1
    }
}
///Field `SWI0` writer - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI0_W<'a, REG> = crate::BitWriter<'a, REG, SWI0>;
impl<'a, REG> SWI0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI0::B0x0)
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI0::B0x1)
    }
}
/**Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI1 {
    ///0: No effect
    B0x0 = 0,
    ///1: Rising edge event generated on the corresponding line, followed by an interrupt
    B0x1 = 1,
}
impl From<SWI1> for bool {
    #[inline(always)]
    fn from(variant: SWI1) -> Self {
        variant as u8 != 0
    }
}
///Field `SWI1` reader - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI1_R = crate::BitReader<SWI1>;
impl SWI1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWI1 {
        match self.bits {
            false => SWI1::B0x0,
            true => SWI1::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SWI1::B0x0
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SWI1::B0x1
    }
}
///Field `SWI1` writer - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI1_W<'a, REG> = crate::BitWriter<'a, REG, SWI1>;
impl<'a, REG> SWI1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI1::B0x0)
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI1::B0x1)
    }
}
/**Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI2 {
    ///0: No effect
    B0x0 = 0,
    ///1: Rising edge event generated on the corresponding line, followed by an interrupt
    B0x1 = 1,
}
impl From<SWI2> for bool {
    #[inline(always)]
    fn from(variant: SWI2) -> Self {
        variant as u8 != 0
    }
}
///Field `SWI2` reader - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI2_R = crate::BitReader<SWI2>;
impl SWI2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWI2 {
        match self.bits {
            false => SWI2::B0x0,
            true => SWI2::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SWI2::B0x0
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SWI2::B0x1
    }
}
///Field `SWI2` writer - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI2_W<'a, REG> = crate::BitWriter<'a, REG, SWI2>;
impl<'a, REG> SWI2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI2::B0x0)
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI2::B0x1)
    }
}
/**Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI3 {
    ///0: No effect
    B0x0 = 0,
    ///1: Rising edge event generated on the corresponding line, followed by an interrupt
    B0x1 = 1,
}
impl From<SWI3> for bool {
    #[inline(always)]
    fn from(variant: SWI3) -> Self {
        variant as u8 != 0
    }
}
///Field `SWI3` reader - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI3_R = crate::BitReader<SWI3>;
impl SWI3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWI3 {
        match self.bits {
            false => SWI3::B0x0,
            true => SWI3::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SWI3::B0x0
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SWI3::B0x1
    }
}
///Field `SWI3` writer - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI3_W<'a, REG> = crate::BitWriter<'a, REG, SWI3>;
impl<'a, REG> SWI3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI3::B0x0)
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI3::B0x1)
    }
}
/**Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI4 {
    ///0: No effect
    B0x0 = 0,
    ///1: Rising edge event generated on the corresponding line, followed by an interrupt
    B0x1 = 1,
}
impl From<SWI4> for bool {
    #[inline(always)]
    fn from(variant: SWI4) -> Self {
        variant as u8 != 0
    }
}
///Field `SWI4` reader - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI4_R = crate::BitReader<SWI4>;
impl SWI4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWI4 {
        match self.bits {
            false => SWI4::B0x0,
            true => SWI4::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SWI4::B0x0
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SWI4::B0x1
    }
}
///Field `SWI4` writer - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI4_W<'a, REG> = crate::BitWriter<'a, REG, SWI4>;
impl<'a, REG> SWI4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI4::B0x0)
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI4::B0x1)
    }
}
/**Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI5 {
    ///0: No effect
    B0x0 = 0,
    ///1: Rising edge event generated on the corresponding line, followed by an interrupt
    B0x1 = 1,
}
impl From<SWI5> for bool {
    #[inline(always)]
    fn from(variant: SWI5) -> Self {
        variant as u8 != 0
    }
}
///Field `SWI5` reader - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI5_R = crate::BitReader<SWI5>;
impl SWI5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWI5 {
        match self.bits {
            false => SWI5::B0x0,
            true => SWI5::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SWI5::B0x0
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SWI5::B0x1
    }
}
///Field `SWI5` writer - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI5_W<'a, REG> = crate::BitWriter<'a, REG, SWI5>;
impl<'a, REG> SWI5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI5::B0x0)
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI5::B0x1)
    }
}
/**Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI6 {
    ///0: No effect
    B0x0 = 0,
    ///1: Rising edge event generated on the corresponding line, followed by an interrupt
    B0x1 = 1,
}
impl From<SWI6> for bool {
    #[inline(always)]
    fn from(variant: SWI6) -> Self {
        variant as u8 != 0
    }
}
///Field `SWI6` reader - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI6_R = crate::BitReader<SWI6>;
impl SWI6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWI6 {
        match self.bits {
            false => SWI6::B0x0,
            true => SWI6::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SWI6::B0x0
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SWI6::B0x1
    }
}
///Field `SWI6` writer - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI6_W<'a, REG> = crate::BitWriter<'a, REG, SWI6>;
impl<'a, REG> SWI6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI6::B0x0)
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI6::B0x1)
    }
}
/**Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI7 {
    ///0: No effect
    B0x0 = 0,
    ///1: Rising edge event generated on the corresponding line, followed by an interrupt
    B0x1 = 1,
}
impl From<SWI7> for bool {
    #[inline(always)]
    fn from(variant: SWI7) -> Self {
        variant as u8 != 0
    }
}
///Field `SWI7` reader - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI7_R = crate::BitReader<SWI7>;
impl SWI7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWI7 {
        match self.bits {
            false => SWI7::B0x0,
            true => SWI7::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SWI7::B0x0
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SWI7::B0x1
    }
}
///Field `SWI7` writer - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI7_W<'a, REG> = crate::BitWriter<'a, REG, SWI7>;
impl<'a, REG> SWI7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI7::B0x0)
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI7::B0x1)
    }
}
/**Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI8 {
    ///0: No effect
    B0x0 = 0,
    ///1: Rising edge event generated on the corresponding line, followed by an interrupt
    B0x1 = 1,
}
impl From<SWI8> for bool {
    #[inline(always)]
    fn from(variant: SWI8) -> Self {
        variant as u8 != 0
    }
}
///Field `SWI8` reader - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI8_R = crate::BitReader<SWI8>;
impl SWI8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWI8 {
        match self.bits {
            false => SWI8::B0x0,
            true => SWI8::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SWI8::B0x0
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SWI8::B0x1
    }
}
///Field `SWI8` writer - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI8_W<'a, REG> = crate::BitWriter<'a, REG, SWI8>;
impl<'a, REG> SWI8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI8::B0x0)
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI8::B0x1)
    }
}
/**Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI9 {
    ///0: No effect
    B0x0 = 0,
    ///1: Rising edge event generated on the corresponding line, followed by an interrupt
    B0x1 = 1,
}
impl From<SWI9> for bool {
    #[inline(always)]
    fn from(variant: SWI9) -> Self {
        variant as u8 != 0
    }
}
///Field `SWI9` reader - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI9_R = crate::BitReader<SWI9>;
impl SWI9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWI9 {
        match self.bits {
            false => SWI9::B0x0,
            true => SWI9::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SWI9::B0x0
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SWI9::B0x1
    }
}
///Field `SWI9` writer - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI9_W<'a, REG> = crate::BitWriter<'a, REG, SWI9>;
impl<'a, REG> SWI9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI9::B0x0)
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI9::B0x1)
    }
}
/**Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI10 {
    ///0: No effect
    B0x0 = 0,
    ///1: Rising edge event generated on the corresponding line, followed by an interrupt
    B0x1 = 1,
}
impl From<SWI10> for bool {
    #[inline(always)]
    fn from(variant: SWI10) -> Self {
        variant as u8 != 0
    }
}
///Field `SWI10` reader - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI10_R = crate::BitReader<SWI10>;
impl SWI10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWI10 {
        match self.bits {
            false => SWI10::B0x0,
            true => SWI10::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SWI10::B0x0
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SWI10::B0x1
    }
}
///Field `SWI10` writer - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI10_W<'a, REG> = crate::BitWriter<'a, REG, SWI10>;
impl<'a, REG> SWI10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI10::B0x0)
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI10::B0x1)
    }
}
/**Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI11 {
    ///0: No effect
    B0x0 = 0,
    ///1: Rising edge event generated on the corresponding line, followed by an interrupt
    B0x1 = 1,
}
impl From<SWI11> for bool {
    #[inline(always)]
    fn from(variant: SWI11) -> Self {
        variant as u8 != 0
    }
}
///Field `SWI11` reader - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI11_R = crate::BitReader<SWI11>;
impl SWI11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWI11 {
        match self.bits {
            false => SWI11::B0x0,
            true => SWI11::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SWI11::B0x0
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SWI11::B0x1
    }
}
///Field `SWI11` writer - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI11_W<'a, REG> = crate::BitWriter<'a, REG, SWI11>;
impl<'a, REG> SWI11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI11::B0x0)
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI11::B0x1)
    }
}
/**Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI12 {
    ///0: No effect
    B0x0 = 0,
    ///1: Rising edge event generated on the corresponding line, followed by an interrupt
    B0x1 = 1,
}
impl From<SWI12> for bool {
    #[inline(always)]
    fn from(variant: SWI12) -> Self {
        variant as u8 != 0
    }
}
///Field `SWI12` reader - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI12_R = crate::BitReader<SWI12>;
impl SWI12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWI12 {
        match self.bits {
            false => SWI12::B0x0,
            true => SWI12::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SWI12::B0x0
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SWI12::B0x1
    }
}
///Field `SWI12` writer - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI12_W<'a, REG> = crate::BitWriter<'a, REG, SWI12>;
impl<'a, REG> SWI12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI12::B0x0)
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI12::B0x1)
    }
}
/**Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI13 {
    ///0: No effect
    B0x0 = 0,
    ///1: Rising edge event generated on the corresponding line, followed by an interrupt
    B0x1 = 1,
}
impl From<SWI13> for bool {
    #[inline(always)]
    fn from(variant: SWI13) -> Self {
        variant as u8 != 0
    }
}
///Field `SWI13` reader - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI13_R = crate::BitReader<SWI13>;
impl SWI13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWI13 {
        match self.bits {
            false => SWI13::B0x0,
            true => SWI13::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SWI13::B0x0
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SWI13::B0x1
    }
}
///Field `SWI13` writer - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI13_W<'a, REG> = crate::BitWriter<'a, REG, SWI13>;
impl<'a, REG> SWI13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI13::B0x0)
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI13::B0x1)
    }
}
/**Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI14 {
    ///0: No effect
    B0x0 = 0,
    ///1: Rising edge event generated on the corresponding line, followed by an interrupt
    B0x1 = 1,
}
impl From<SWI14> for bool {
    #[inline(always)]
    fn from(variant: SWI14) -> Self {
        variant as u8 != 0
    }
}
///Field `SWI14` reader - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI14_R = crate::BitReader<SWI14>;
impl SWI14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWI14 {
        match self.bits {
            false => SWI14::B0x0,
            true => SWI14::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SWI14::B0x0
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SWI14::B0x1
    }
}
///Field `SWI14` writer - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI14_W<'a, REG> = crate::BitWriter<'a, REG, SWI14>;
impl<'a, REG> SWI14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI14::B0x0)
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI14::B0x1)
    }
}
/**Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI15 {
    ///0: No effect
    B0x0 = 0,
    ///1: Rising edge event generated on the corresponding line, followed by an interrupt
    B0x1 = 1,
}
impl From<SWI15> for bool {
    #[inline(always)]
    fn from(variant: SWI15) -> Self {
        variant as u8 != 0
    }
}
///Field `SWI15` reader - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI15_R = crate::BitReader<SWI15>;
impl SWI15_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWI15 {
        match self.bits {
            false => SWI15::B0x0,
            true => SWI15::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SWI15::B0x0
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SWI15::B0x1
    }
}
///Field `SWI15` writer - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI15_W<'a, REG> = crate::BitWriter<'a, REG, SWI15>;
impl<'a, REG> SWI15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI15::B0x0)
    }
    ///Rising edge event generated on the corresponding line, followed by an interrupt
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI15::B0x1)
    }
}
impl R {
    ///Bit 0 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi0(&self) -> SWI0_R {
        SWI0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi1(&self) -> SWI1_R {
        SWI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi3(&self) -> SWI3_R {
        SWI3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi4(&self) -> SWI4_R {
        SWI4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi5(&self) -> SWI5_R {
        SWI5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi6(&self) -> SWI6_R {
        SWI6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi7(&self) -> SWI7_R {
        SWI7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi8(&self) -> SWI8_R {
        SWI8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi9(&self) -> SWI9_R {
        SWI9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi10(&self) -> SWI10_R {
        SWI10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi11(&self) -> SWI11_R {
        SWI11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi12(&self) -> SWI12_R {
        SWI12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi13(&self) -> SWI13_R {
        SWI13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi14(&self) -> SWI14_R {
        SWI14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi15(&self) -> SWI15_R {
        SWI15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTI_SWIER1")
            .field("swi0", &self.swi0())
            .field("swi1", &self.swi1())
            .field("swi2", &self.swi2())
            .field("swi3", &self.swi3())
            .field("swi4", &self.swi4())
            .field("swi5", &self.swi5())
            .field("swi6", &self.swi6())
            .field("swi7", &self.swi7())
            .field("swi8", &self.swi8())
            .field("swi9", &self.swi9())
            .field("swi10", &self.swi10())
            .field("swi11", &self.swi11())
            .field("swi12", &self.swi12())
            .field("swi13", &self.swi13())
            .field("swi14", &self.swi14())
            .field("swi15", &self.swi15())
            .finish()
    }
}
impl W {
    ///Bit 0 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi0(&mut self) -> SWI0_W<'_, EXTI_SWIER1rs> {
        SWI0_W::new(self, 0)
    }
    ///Bit 1 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi1(&mut self) -> SWI1_W<'_, EXTI_SWIER1rs> {
        SWI1_W::new(self, 1)
    }
    ///Bit 2 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi2(&mut self) -> SWI2_W<'_, EXTI_SWIER1rs> {
        SWI2_W::new(self, 2)
    }
    ///Bit 3 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi3(&mut self) -> SWI3_W<'_, EXTI_SWIER1rs> {
        SWI3_W::new(self, 3)
    }
    ///Bit 4 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi4(&mut self) -> SWI4_W<'_, EXTI_SWIER1rs> {
        SWI4_W::new(self, 4)
    }
    ///Bit 5 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi5(&mut self) -> SWI5_W<'_, EXTI_SWIER1rs> {
        SWI5_W::new(self, 5)
    }
    ///Bit 6 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi6(&mut self) -> SWI6_W<'_, EXTI_SWIER1rs> {
        SWI6_W::new(self, 6)
    }
    ///Bit 7 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi7(&mut self) -> SWI7_W<'_, EXTI_SWIER1rs> {
        SWI7_W::new(self, 7)
    }
    ///Bit 8 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi8(&mut self) -> SWI8_W<'_, EXTI_SWIER1rs> {
        SWI8_W::new(self, 8)
    }
    ///Bit 9 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi9(&mut self) -> SWI9_W<'_, EXTI_SWIER1rs> {
        SWI9_W::new(self, 9)
    }
    ///Bit 10 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi10(&mut self) -> SWI10_W<'_, EXTI_SWIER1rs> {
        SWI10_W::new(self, 10)
    }
    ///Bit 11 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi11(&mut self) -> SWI11_W<'_, EXTI_SWIER1rs> {
        SWI11_W::new(self, 11)
    }
    ///Bit 12 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi12(&mut self) -> SWI12_W<'_, EXTI_SWIER1rs> {
        SWI12_W::new(self, 12)
    }
    ///Bit 13 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi13(&mut self) -> SWI13_W<'_, EXTI_SWIER1rs> {
        SWI13_W::new(self, 13)
    }
    ///Bit 14 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi14(&mut self) -> SWI14_W<'_, EXTI_SWIER1rs> {
        SWI14_W::new(self, 14)
    }
    ///Bit 15 - Software rising edge event trigger on line x (x = 15 to 0) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi15(&mut self) -> SWI15_W<'_, EXTI_SWIER1rs> {
        SWI15_W::new(self, 15)
    }
}
/**EXTI software interrupt event register 1

You can [`read`](crate::Reg::read) this register and get [`exti_swier1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_swier1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#EXTI:EXTI_SWIER1)*/
pub struct EXTI_SWIER1rs;
impl crate::RegisterSpec for EXTI_SWIER1rs {
    type Ux = u32;
}
///`read()` method returns [`exti_swier1::R`](R) reader structure
impl crate::Readable for EXTI_SWIER1rs {}
///`write(|w| ..)` method takes [`exti_swier1::W`](W) writer structure
impl crate::Writable for EXTI_SWIER1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTI_SWIER1 to value 0
impl crate::Resettable for EXTI_SWIER1rs {}
