///Register `RCC_IOPENR` reader
pub type R = crate::R<RCC_IOPENRrs>;
///Register `RCC_IOPENR` writer
pub type W = crate::W<RCC_IOPENRrs>;
/**I/O port A clock enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOAEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<GPIOAEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOAEN` reader - I/O port A clock enable This bit is set and cleared by software.
pub type GPIOAEN_R = crate::BitReader<GPIOAEN>;
impl GPIOAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOAEN {
        match self.bits {
            false => GPIOAEN::B0x0,
            true => GPIOAEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOAEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOAEN::B0x1
    }
}
///Field `GPIOAEN` writer - I/O port A clock enable This bit is set and cleared by software.
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOAEN>;
impl<'a, REG> GPIOAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::B0x1)
    }
}
/**I/O port B clock enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOBEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<GPIOBEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOBEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOBEN` reader - I/O port B clock enable This bit is set and cleared by software.
pub type GPIOBEN_R = crate::BitReader<GPIOBEN>;
impl GPIOBEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOBEN {
        match self.bits {
            false => GPIOBEN::B0x0,
            true => GPIOBEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOBEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOBEN::B0x1
    }
}
///Field `GPIOBEN` writer - I/O port B clock enable This bit is set and cleared by software.
pub type GPIOBEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOBEN>;
impl<'a, REG> GPIOBEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOBEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOBEN::B0x1)
    }
}
/**I/O port C clock enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOCEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<GPIOCEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOCEN` reader - I/O port C clock enable This bit is set and cleared by software.
pub type GPIOCEN_R = crate::BitReader<GPIOCEN>;
impl GPIOCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOCEN {
        match self.bits {
            false => GPIOCEN::B0x0,
            true => GPIOCEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOCEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOCEN::B0x1
    }
}
///Field `GPIOCEN` writer - I/O port C clock enable This bit is set and cleared by software.
pub type GPIOCEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOCEN>;
impl<'a, REG> GPIOCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOCEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOCEN::B0x1)
    }
}
/**I/O port D clock enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIODEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<GPIODEN> for bool {
    #[inline(always)]
    fn from(variant: GPIODEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIODEN` reader - I/O port D clock enable This bit is set and cleared by software.
pub type GPIODEN_R = crate::BitReader<GPIODEN>;
impl GPIODEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIODEN {
        match self.bits {
            false => GPIODEN::B0x0,
            true => GPIODEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIODEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIODEN::B0x1
    }
}
///Field `GPIODEN` writer - I/O port D clock enable This bit is set and cleared by software.
pub type GPIODEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIODEN>;
impl<'a, REG> GPIODEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIODEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIODEN::B0x1)
    }
}
/**I/O port F clock enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOFEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<GPIOFEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOFEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOFEN` reader - I/O port F clock enable This bit is set and cleared by software.
pub type GPIOFEN_R = crate::BitReader<GPIOFEN>;
impl GPIOFEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOFEN {
        match self.bits {
            false => GPIOFEN::B0x0,
            true => GPIOFEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOFEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOFEN::B0x1
    }
}
///Field `GPIOFEN` writer - I/O port F clock enable This bit is set and cleared by software.
pub type GPIOFEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOFEN>;
impl<'a, REG> GPIOFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOFEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOFEN::B0x1)
    }
}
impl R {
    ///Bit 0 - I/O port A clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I/O port B clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I/O port C clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I/O port D clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - I/O port F clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_IOPENR")
            .field("gpioaen", &self.gpioaen())
            .field("gpioben", &self.gpioben())
            .field("gpiocen", &self.gpiocen())
            .field("gpioden", &self.gpioden())
            .field("gpiofen", &self.gpiofen())
            .finish()
    }
}
impl W {
    ///Bit 0 - I/O port A clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<'_, RCC_IOPENRrs> {
        GPIOAEN_W::new(self, 0)
    }
    ///Bit 1 - I/O port B clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W<'_, RCC_IOPENRrs> {
        GPIOBEN_W::new(self, 1)
    }
    ///Bit 2 - I/O port C clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<'_, RCC_IOPENRrs> {
        GPIOCEN_W::new(self, 2)
    }
    ///Bit 3 - I/O port D clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpioden(&mut self) -> GPIODEN_W<'_, RCC_IOPENRrs> {
        GPIODEN_W::new(self, 3)
    }
    ///Bit 5 - I/O port F clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<'_, RCC_IOPENRrs> {
        GPIOFEN_W::new(self, 5)
    }
}
/**RCC I/O port clock enable register

You can [`read`](crate::Reg::read) this register and get [`rcc_iopenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_iopenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#RCC:RCC_IOPENR)*/
pub struct RCC_IOPENRrs;
impl crate::RegisterSpec for RCC_IOPENRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_iopenr::R`](R) reader structure
impl crate::Readable for RCC_IOPENRrs {}
///`write(|w| ..)` method takes [`rcc_iopenr::W`](W) writer structure
impl crate::Writable for RCC_IOPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCC_IOPENR to value 0
impl crate::Resettable for RCC_IOPENRrs {}
