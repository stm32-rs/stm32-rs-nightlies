///Register `RCC_IOPSMENR` reader
pub type R = crate::R<RCC_IOPSMENRrs>;
///Register `RCC_IOPSMENR` writer
pub type W = crate::W<RCC_IOPSMENRrs>;
/**I/O port A clock enable during Sleep mode Set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOASMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<GPIOASMEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOASMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOASMEN` reader - I/O port A clock enable during Sleep mode Set and cleared by software.
pub type GPIOASMEN_R = crate::BitReader<GPIOASMEN>;
impl GPIOASMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOASMEN {
        match self.bits {
            false => GPIOASMEN::B0x0,
            true => GPIOASMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOASMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOASMEN::B0x1
    }
}
///Field `GPIOASMEN` writer - I/O port A clock enable during Sleep mode Set and cleared by software.
pub type GPIOASMEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOASMEN>;
impl<'a, REG> GPIOASMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOASMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOASMEN::B0x1)
    }
}
/**I/O port B clock enable during Sleep mode Set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOBSMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<GPIOBSMEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOBSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOBSMEN` reader - I/O port B clock enable during Sleep mode Set and cleared by software.
pub type GPIOBSMEN_R = crate::BitReader<GPIOBSMEN>;
impl GPIOBSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOBSMEN {
        match self.bits {
            false => GPIOBSMEN::B0x0,
            true => GPIOBSMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOBSMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOBSMEN::B0x1
    }
}
///Field `GPIOBSMEN` writer - I/O port B clock enable during Sleep mode Set and cleared by software.
pub type GPIOBSMEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOBSMEN>;
impl<'a, REG> GPIOBSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOBSMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOBSMEN::B0x1)
    }
}
/**I/O port C clock enable during Sleep mode Set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOCSMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<GPIOCSMEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOCSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOCSMEN` reader - I/O port C clock enable during Sleep mode Set and cleared by software.
pub type GPIOCSMEN_R = crate::BitReader<GPIOCSMEN>;
impl GPIOCSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOCSMEN {
        match self.bits {
            false => GPIOCSMEN::B0x0,
            true => GPIOCSMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOCSMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOCSMEN::B0x1
    }
}
///Field `GPIOCSMEN` writer - I/O port C clock enable during Sleep mode Set and cleared by software.
pub type GPIOCSMEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOCSMEN>;
impl<'a, REG> GPIOCSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOCSMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOCSMEN::B0x1)
    }
}
/**I/O port D clock enable during Sleep mode Set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIODSMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<GPIODSMEN> for bool {
    #[inline(always)]
    fn from(variant: GPIODSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIODSMEN` reader - I/O port D clock enable during Sleep mode Set and cleared by software.
pub type GPIODSMEN_R = crate::BitReader<GPIODSMEN>;
impl GPIODSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIODSMEN {
        match self.bits {
            false => GPIODSMEN::B0x0,
            true => GPIODSMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIODSMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIODSMEN::B0x1
    }
}
///Field `GPIODSMEN` writer - I/O port D clock enable during Sleep mode Set and cleared by software.
pub type GPIODSMEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIODSMEN>;
impl<'a, REG> GPIODSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIODSMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIODSMEN::B0x1)
    }
}
/**I/O port F clock enable during Sleep mode Set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOFSMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<GPIOFSMEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOFSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOFSMEN` reader - I/O port F clock enable during Sleep mode Set and cleared by software.
pub type GPIOFSMEN_R = crate::BitReader<GPIOFSMEN>;
impl GPIOFSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOFSMEN {
        match self.bits {
            false => GPIOFSMEN::B0x0,
            true => GPIOFSMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOFSMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOFSMEN::B0x1
    }
}
///Field `GPIOFSMEN` writer - I/O port F clock enable during Sleep mode Set and cleared by software.
pub type GPIOFSMEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOFSMEN>;
impl<'a, REG> GPIOFSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOFSMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOFSMEN::B0x1)
    }
}
impl R {
    ///Bit 0 - I/O port A clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I/O port B clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I/O port C clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I/O port D clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GPIODSMEN_R {
        GPIODSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - I/O port F clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn gpiofsmen(&self) -> GPIOFSMEN_R {
        GPIOFSMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_IOPSMENR")
            .field("gpioasmen", &self.gpioasmen())
            .field("gpiobsmen", &self.gpiobsmen())
            .field("gpiocsmen", &self.gpiocsmen())
            .field("gpiodsmen", &self.gpiodsmen())
            .field("gpiofsmen", &self.gpiofsmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - I/O port A clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<'_, RCC_IOPSMENRrs> {
        GPIOASMEN_W::new(self, 0)
    }
    ///Bit 1 - I/O port B clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<'_, RCC_IOPSMENRrs> {
        GPIOBSMEN_W::new(self, 1)
    }
    ///Bit 2 - I/O port C clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<'_, RCC_IOPSMENRrs> {
        GPIOCSMEN_W::new(self, 2)
    }
    ///Bit 3 - I/O port D clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn gpiodsmen(&mut self) -> GPIODSMEN_W<'_, RCC_IOPSMENRrs> {
        GPIODSMEN_W::new(self, 3)
    }
    ///Bit 5 - I/O port F clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn gpiofsmen(&mut self) -> GPIOFSMEN_W<'_, RCC_IOPSMENRrs> {
        GPIOFSMEN_W::new(self, 5)
    }
}
/**RCC I/O port in Sleep mode clock enable register

You can [`read`](crate::Reg::read) this register and get [`rcc_iopsmenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_iopsmenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#RCC:RCC_IOPSMENR)*/
pub struct RCC_IOPSMENRrs;
impl crate::RegisterSpec for RCC_IOPSMENRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_iopsmenr::R`](R) reader structure
impl crate::Readable for RCC_IOPSMENRrs {}
///`write(|w| ..)` method takes [`rcc_iopsmenr::W`](W) writer structure
impl crate::Writable for RCC_IOPSMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCC_IOPSMENR to value 0x2f
impl crate::Resettable for RCC_IOPSMENRrs {
    const RESET_VALUE: u32 = 0x2f;
}
