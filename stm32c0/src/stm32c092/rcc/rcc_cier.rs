///Register `RCC_CIER` reader
pub type R = crate::R<RCC_CIERrs>;
///Register `RCC_CIER` writer
pub type W = crate::W<RCC_CIERrs>;
/**LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYIE {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<LSIRDYIE> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYIE` reader - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization:
pub type LSIRDYIE_R = crate::BitReader<LSIRDYIE>;
impl LSIRDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYIE {
        match self.bits {
            false => LSIRDYIE::B0x0,
            true => LSIRDYIE::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSIRDYIE::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSIRDYIE::B0x1
    }
}
///Field `LSIRDYIE` writer - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization:
pub type LSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDYIE>;
impl<'a, REG> LSIRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYIE::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYIE::B0x1)
    }
}
/**LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYIE {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<LSERDYIE> for bool {
    #[inline(always)]
    fn from(variant: LSERDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `LSERDYIE` reader - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization:
pub type LSERDYIE_R = crate::BitReader<LSERDYIE>;
impl LSERDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSERDYIE {
        match self.bits {
            false => LSERDYIE::B0x0,
            true => LSERDYIE::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSERDYIE::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSERDYIE::B0x1
    }
}
///Field `LSERDYIE` writer - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization:
pub type LSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG, LSERDYIE>;
impl<'a, REG> LSERDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSERDYIE::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSERDYIE::B0x1)
    }
}
/**HSIUSB48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSIUSB48 oscillator stabilization: Note: Only applicable on STM32C071xx, reserved on other devices.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIUSB48RDYIE {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<HSIUSB48RDYIE> for bool {
    #[inline(always)]
    fn from(variant: HSIUSB48RDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIUSB48RDYIE` reader - HSIUSB48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSIUSB48 oscillator stabilization: Note: Only applicable on STM32C071xx, reserved on other devices.
pub type HSIUSB48RDYIE_R = crate::BitReader<HSIUSB48RDYIE>;
impl HSIUSB48RDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIUSB48RDYIE {
        match self.bits {
            false => HSIUSB48RDYIE::B0x0,
            true => HSIUSB48RDYIE::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSIUSB48RDYIE::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSIUSB48RDYIE::B0x1
    }
}
///Field `HSIUSB48RDYIE` writer - HSIUSB48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSIUSB48 oscillator stabilization: Note: Only applicable on STM32C071xx, reserved on other devices.
pub type HSIUSB48RDYIE_W<'a, REG> = crate::BitWriter<'a, REG, HSIUSB48RDYIE>;
impl<'a, REG> HSIUSB48RDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSIUSB48RDYIE::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSIUSB48RDYIE::B0x1)
    }
}
/**HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYIE {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<HSIRDYIE> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIRDYIE` reader - HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization:
pub type HSIRDYIE_R = crate::BitReader<HSIRDYIE>;
impl HSIRDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIRDYIE {
        match self.bits {
            false => HSIRDYIE::B0x0,
            true => HSIRDYIE::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSIRDYIE::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSIRDYIE::B0x1
    }
}
///Field `HSIRDYIE` writer - HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization:
pub type HSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, HSIRDYIE>;
impl<'a, REG> HSIRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSIRDYIE::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSIRDYIE::B0x1)
    }
}
/**HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYIE {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<HSERDYIE> for bool {
    #[inline(always)]
    fn from(variant: HSERDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `HSERDYIE` reader - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization:
pub type HSERDYIE_R = crate::BitReader<HSERDYIE>;
impl HSERDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSERDYIE {
        match self.bits {
            false => HSERDYIE::B0x0,
            true => HSERDYIE::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSERDYIE::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSERDYIE::B0x1
    }
}
///Field `HSERDYIE` writer - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization:
pub type HSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG, HSERDYIE>;
impl<'a, REG> HSERDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSERDYIE::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSERDYIE::B0x1)
    }
}
impl R {
    ///Bit 0 - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization:
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization:
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSIUSB48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSIUSB48 oscillator stabilization: Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn hsiusb48rdyie(&self) -> HSIUSB48RDYIE_R {
        HSIUSB48RDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization:
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization:
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CIER")
            .field("lsirdyie", &self.lsirdyie())
            .field("lserdyie", &self.lserdyie())
            .field("hsiusb48rdyie", &self.hsiusb48rdyie())
            .field("hsirdyie", &self.hsirdyie())
            .field("hserdyie", &self.hserdyie())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization:
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<'_, RCC_CIERrs> {
        LSIRDYIE_W::new(self, 0)
    }
    ///Bit 1 - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization:
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<'_, RCC_CIERrs> {
        LSERDYIE_W::new(self, 1)
    }
    ///Bit 2 - HSIUSB48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSIUSB48 oscillator stabilization: Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn hsiusb48rdyie(&mut self) -> HSIUSB48RDYIE_W<'_, RCC_CIERrs> {
        HSIUSB48RDYIE_W::new(self, 2)
    }
    ///Bit 3 - HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization:
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<'_, RCC_CIERrs> {
        HSIRDYIE_W::new(self, 3)
    }
    ///Bit 4 - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization:
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<'_, RCC_CIERrs> {
        HSERDYIE_W::new(self, 4)
    }
}
/**RCC clock interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`rcc_cier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#RCC:RCC_CIER)*/
pub struct RCC_CIERrs;
impl crate::RegisterSpec for RCC_CIERrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_cier::R`](R) reader structure
impl crate::Readable for RCC_CIERrs {}
///`write(|w| ..)` method takes [`rcc_cier::W`](W) writer structure
impl crate::Writable for RCC_CIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCC_CIER to value 0
impl crate::Resettable for RCC_CIERrs {}
