///Register `EXTI_RTSR1` reader
pub type R = crate::R<EXTI_RTSR1rs>;
///Register `EXTI_RTSR1` writer
pub type W = crate::W<EXTI_RTSR1rs>;
/**Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT0 {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<RT0> for bool {
    #[inline(always)]
    fn from(variant: RT0) -> Self {
        variant as u8 != 0
    }
}
///Field `RT0` reader - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT0_R = crate::BitReader<RT0>;
impl RT0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RT0 {
        match self.bits {
            false => RT0::B0x0,
            true => RT0::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RT0::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RT0::B0x1
    }
}
///Field `RT0` writer - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT0_W<'a, REG> = crate::BitWriter<'a, REG, RT0>;
impl<'a, REG> RT0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT0::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT0::B0x1)
    }
}
/**Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT1 {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<RT1> for bool {
    #[inline(always)]
    fn from(variant: RT1) -> Self {
        variant as u8 != 0
    }
}
///Field `RT1` reader - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT1_R = crate::BitReader<RT1>;
impl RT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RT1 {
        match self.bits {
            false => RT1::B0x0,
            true => RT1::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RT1::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RT1::B0x1
    }
}
///Field `RT1` writer - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT1_W<'a, REG> = crate::BitWriter<'a, REG, RT1>;
impl<'a, REG> RT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT1::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT1::B0x1)
    }
}
/**Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT2 {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<RT2> for bool {
    #[inline(always)]
    fn from(variant: RT2) -> Self {
        variant as u8 != 0
    }
}
///Field `RT2` reader - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT2_R = crate::BitReader<RT2>;
impl RT2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RT2 {
        match self.bits {
            false => RT2::B0x0,
            true => RT2::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RT2::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RT2::B0x1
    }
}
///Field `RT2` writer - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT2_W<'a, REG> = crate::BitWriter<'a, REG, RT2>;
impl<'a, REG> RT2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT2::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT2::B0x1)
    }
}
/**Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT3 {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<RT3> for bool {
    #[inline(always)]
    fn from(variant: RT3) -> Self {
        variant as u8 != 0
    }
}
///Field `RT3` reader - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT3_R = crate::BitReader<RT3>;
impl RT3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RT3 {
        match self.bits {
            false => RT3::B0x0,
            true => RT3::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RT3::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RT3::B0x1
    }
}
///Field `RT3` writer - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT3_W<'a, REG> = crate::BitWriter<'a, REG, RT3>;
impl<'a, REG> RT3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT3::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT3::B0x1)
    }
}
/**Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT4 {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<RT4> for bool {
    #[inline(always)]
    fn from(variant: RT4) -> Self {
        variant as u8 != 0
    }
}
///Field `RT4` reader - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT4_R = crate::BitReader<RT4>;
impl RT4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RT4 {
        match self.bits {
            false => RT4::B0x0,
            true => RT4::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RT4::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RT4::B0x1
    }
}
///Field `RT4` writer - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT4_W<'a, REG> = crate::BitWriter<'a, REG, RT4>;
impl<'a, REG> RT4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT4::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT4::B0x1)
    }
}
/**Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT5 {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<RT5> for bool {
    #[inline(always)]
    fn from(variant: RT5) -> Self {
        variant as u8 != 0
    }
}
///Field `RT5` reader - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT5_R = crate::BitReader<RT5>;
impl RT5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RT5 {
        match self.bits {
            false => RT5::B0x0,
            true => RT5::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RT5::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RT5::B0x1
    }
}
///Field `RT5` writer - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT5_W<'a, REG> = crate::BitWriter<'a, REG, RT5>;
impl<'a, REG> RT5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT5::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT5::B0x1)
    }
}
/**Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT6 {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<RT6> for bool {
    #[inline(always)]
    fn from(variant: RT6) -> Self {
        variant as u8 != 0
    }
}
///Field `RT6` reader - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT6_R = crate::BitReader<RT6>;
impl RT6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RT6 {
        match self.bits {
            false => RT6::B0x0,
            true => RT6::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RT6::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RT6::B0x1
    }
}
///Field `RT6` writer - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT6_W<'a, REG> = crate::BitWriter<'a, REG, RT6>;
impl<'a, REG> RT6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT6::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT6::B0x1)
    }
}
/**Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT7 {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<RT7> for bool {
    #[inline(always)]
    fn from(variant: RT7) -> Self {
        variant as u8 != 0
    }
}
///Field `RT7` reader - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT7_R = crate::BitReader<RT7>;
impl RT7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RT7 {
        match self.bits {
            false => RT7::B0x0,
            true => RT7::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RT7::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RT7::B0x1
    }
}
///Field `RT7` writer - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT7_W<'a, REG> = crate::BitWriter<'a, REG, RT7>;
impl<'a, REG> RT7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT7::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT7::B0x1)
    }
}
/**Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT8 {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<RT8> for bool {
    #[inline(always)]
    fn from(variant: RT8) -> Self {
        variant as u8 != 0
    }
}
///Field `RT8` reader - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT8_R = crate::BitReader<RT8>;
impl RT8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RT8 {
        match self.bits {
            false => RT8::B0x0,
            true => RT8::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RT8::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RT8::B0x1
    }
}
///Field `RT8` writer - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT8_W<'a, REG> = crate::BitWriter<'a, REG, RT8>;
impl<'a, REG> RT8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT8::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT8::B0x1)
    }
}
/**Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT9 {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<RT9> for bool {
    #[inline(always)]
    fn from(variant: RT9) -> Self {
        variant as u8 != 0
    }
}
///Field `RT9` reader - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT9_R = crate::BitReader<RT9>;
impl RT9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RT9 {
        match self.bits {
            false => RT9::B0x0,
            true => RT9::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RT9::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RT9::B0x1
    }
}
///Field `RT9` writer - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT9_W<'a, REG> = crate::BitWriter<'a, REG, RT9>;
impl<'a, REG> RT9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT9::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT9::B0x1)
    }
}
/**Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT10 {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<RT10> for bool {
    #[inline(always)]
    fn from(variant: RT10) -> Self {
        variant as u8 != 0
    }
}
///Field `RT10` reader - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT10_R = crate::BitReader<RT10>;
impl RT10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RT10 {
        match self.bits {
            false => RT10::B0x0,
            true => RT10::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RT10::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RT10::B0x1
    }
}
///Field `RT10` writer - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT10_W<'a, REG> = crate::BitWriter<'a, REG, RT10>;
impl<'a, REG> RT10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT10::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT10::B0x1)
    }
}
/**Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT11 {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<RT11> for bool {
    #[inline(always)]
    fn from(variant: RT11) -> Self {
        variant as u8 != 0
    }
}
///Field `RT11` reader - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT11_R = crate::BitReader<RT11>;
impl RT11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RT11 {
        match self.bits {
            false => RT11::B0x0,
            true => RT11::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RT11::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RT11::B0x1
    }
}
///Field `RT11` writer - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT11_W<'a, REG> = crate::BitWriter<'a, REG, RT11>;
impl<'a, REG> RT11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT11::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT11::B0x1)
    }
}
/**Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT12 {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<RT12> for bool {
    #[inline(always)]
    fn from(variant: RT12) -> Self {
        variant as u8 != 0
    }
}
///Field `RT12` reader - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT12_R = crate::BitReader<RT12>;
impl RT12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RT12 {
        match self.bits {
            false => RT12::B0x0,
            true => RT12::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RT12::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RT12::B0x1
    }
}
///Field `RT12` writer - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT12_W<'a, REG> = crate::BitWriter<'a, REG, RT12>;
impl<'a, REG> RT12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT12::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT12::B0x1)
    }
}
/**Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT13 {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<RT13> for bool {
    #[inline(always)]
    fn from(variant: RT13) -> Self {
        variant as u8 != 0
    }
}
///Field `RT13` reader - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT13_R = crate::BitReader<RT13>;
impl RT13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RT13 {
        match self.bits {
            false => RT13::B0x0,
            true => RT13::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RT13::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RT13::B0x1
    }
}
///Field `RT13` writer - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT13_W<'a, REG> = crate::BitWriter<'a, REG, RT13>;
impl<'a, REG> RT13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT13::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT13::B0x1)
    }
}
/**Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT14 {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<RT14> for bool {
    #[inline(always)]
    fn from(variant: RT14) -> Self {
        variant as u8 != 0
    }
}
///Field `RT14` reader - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT14_R = crate::BitReader<RT14>;
impl RT14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RT14 {
        match self.bits {
            false => RT14::B0x0,
            true => RT14::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RT14::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RT14::B0x1
    }
}
///Field `RT14` writer - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT14_W<'a, REG> = crate::BitWriter<'a, REG, RT14>;
impl<'a, REG> RT14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT14::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT14::B0x1)
    }
}
/**Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT15 {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<RT15> for bool {
    #[inline(always)]
    fn from(variant: RT15) -> Self {
        variant as u8 != 0
    }
}
///Field `RT15` reader - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT15_R = crate::BitReader<RT15>;
impl RT15_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RT15 {
        match self.bits {
            false => RT15::B0x0,
            true => RT15::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RT15::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RT15::B0x1
    }
}
///Field `RT15` writer - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT15_W<'a, REG> = crate::BitWriter<'a, REG, RT15>;
impl<'a, REG> RT15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT15::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT15::B0x1)
    }
}
impl R {
    ///Bit 0 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt0(&self) -> RT0_R {
        RT0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt1(&self) -> RT1_R {
        RT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt2(&self) -> RT2_R {
        RT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt3(&self) -> RT3_R {
        RT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt4(&self) -> RT4_R {
        RT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt5(&self) -> RT5_R {
        RT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt6(&self) -> RT6_R {
        RT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt7(&self) -> RT7_R {
        RT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt8(&self) -> RT8_R {
        RT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt9(&self) -> RT9_R {
        RT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt10(&self) -> RT10_R {
        RT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt11(&self) -> RT11_R {
        RT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt12(&self) -> RT12_R {
        RT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt13(&self) -> RT13_R {
        RT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt14(&self) -> RT14_R {
        RT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt15(&self) -> RT15_R {
        RT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTI_RTSR1")
            .field("rt0", &self.rt0())
            .field("rt1", &self.rt1())
            .field("rt2", &self.rt2())
            .field("rt3", &self.rt3())
            .field("rt4", &self.rt4())
            .field("rt5", &self.rt5())
            .field("rt6", &self.rt6())
            .field("rt7", &self.rt7())
            .field("rt8", &self.rt8())
            .field("rt9", &self.rt9())
            .field("rt10", &self.rt10())
            .field("rt11", &self.rt11())
            .field("rt12", &self.rt12())
            .field("rt13", &self.rt13())
            .field("rt14", &self.rt14())
            .field("rt15", &self.rt15())
            .finish()
    }
}
impl W {
    ///Bit 0 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt0(&mut self) -> RT0_W<'_, EXTI_RTSR1rs> {
        RT0_W::new(self, 0)
    }
    ///Bit 1 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt1(&mut self) -> RT1_W<'_, EXTI_RTSR1rs> {
        RT1_W::new(self, 1)
    }
    ///Bit 2 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt2(&mut self) -> RT2_W<'_, EXTI_RTSR1rs> {
        RT2_W::new(self, 2)
    }
    ///Bit 3 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt3(&mut self) -> RT3_W<'_, EXTI_RTSR1rs> {
        RT3_W::new(self, 3)
    }
    ///Bit 4 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt4(&mut self) -> RT4_W<'_, EXTI_RTSR1rs> {
        RT4_W::new(self, 4)
    }
    ///Bit 5 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt5(&mut self) -> RT5_W<'_, EXTI_RTSR1rs> {
        RT5_W::new(self, 5)
    }
    ///Bit 6 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt6(&mut self) -> RT6_W<'_, EXTI_RTSR1rs> {
        RT6_W::new(self, 6)
    }
    ///Bit 7 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt7(&mut self) -> RT7_W<'_, EXTI_RTSR1rs> {
        RT7_W::new(self, 7)
    }
    ///Bit 8 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt8(&mut self) -> RT8_W<'_, EXTI_RTSR1rs> {
        RT8_W::new(self, 8)
    }
    ///Bit 9 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt9(&mut self) -> RT9_W<'_, EXTI_RTSR1rs> {
        RT9_W::new(self, 9)
    }
    ///Bit 10 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt10(&mut self) -> RT10_W<'_, EXTI_RTSR1rs> {
        RT10_W::new(self, 10)
    }
    ///Bit 11 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt11(&mut self) -> RT11_W<'_, EXTI_RTSR1rs> {
        RT11_W::new(self, 11)
    }
    ///Bit 12 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt12(&mut self) -> RT12_W<'_, EXTI_RTSR1rs> {
        RT12_W::new(self, 12)
    }
    ///Bit 13 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt13(&mut self) -> RT13_W<'_, EXTI_RTSR1rs> {
        RT13_W::new(self, 13)
    }
    ///Bit 14 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt14(&mut self) -> RT14_W<'_, EXTI_RTSR1rs> {
        RT14_W::new(self, 14)
    }
    ///Bit 15 - Rising trigger event configuration bit of configurable line x (x = 15 to 0) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Note: The configurable lines are edge triggered; no glitch must be generated on these inputs. If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt15(&mut self) -> RT15_W<'_, EXTI_RTSR1rs> {
        RT15_W::new(self, 15)
    }
}
/**EXTI rising trigger selection register 1

You can [`read`](crate::Reg::read) this register and get [`exti_rtsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_rtsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#EXTI:EXTI_RTSR1)*/
pub struct EXTI_RTSR1rs;
impl crate::RegisterSpec for EXTI_RTSR1rs {
    type Ux = u32;
}
///`read()` method returns [`exti_rtsr1::R`](R) reader structure
impl crate::Readable for EXTI_RTSR1rs {}
///`write(|w| ..)` method takes [`exti_rtsr1::W`](W) writer structure
impl crate::Writable for EXTI_RTSR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTI_RTSR1 to value 0
impl crate::Resettable for EXTI_RTSR1rs {}
