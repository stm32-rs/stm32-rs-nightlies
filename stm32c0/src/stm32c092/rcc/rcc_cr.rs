///Register `RCC_CR` reader
pub type R = crate::R<RCC_CRrs>;
///Register `RCC_CR` writer
pub type W = crate::W<RCC_CRrs>;
/**Clock division factor for system clock Set and cleared by software. SYSCLK is result of the division by: Note: This bitfield is only available on STM32C071xx.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSDIV {
    ///0: 1 (no division, reset value)
    B0x0 = 0,
    ///1: 2
    B0x1 = 1,
    ///2: 3
    B0x2 = 2,
    ///3: 4
    B0x3 = 3,
    ///4: 5
    B0x4 = 4,
    ///5: 6
    B0x5 = 5,
    ///6: 7
    B0x6 = 6,
    ///7: 8
    B0x7 = 7,
}
impl From<SYSDIV> for u8 {
    #[inline(always)]
    fn from(variant: SYSDIV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSDIV {
    type Ux = u8;
}
impl crate::IsEnum for SYSDIV {}
///Field `SYSDIV` reader - Clock division factor for system clock Set and cleared by software. SYSCLK is result of the division by: Note: This bitfield is only available on STM32C071xx.
pub type SYSDIV_R = crate::FieldReader<SYSDIV>;
impl SYSDIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYSDIV {
        match self.bits {
            0 => SYSDIV::B0x0,
            1 => SYSDIV::B0x1,
            2 => SYSDIV::B0x2,
            3 => SYSDIV::B0x3,
            4 => SYSDIV::B0x4,
            5 => SYSDIV::B0x5,
            6 => SYSDIV::B0x6,
            7 => SYSDIV::B0x7,
            _ => unreachable!(),
        }
    }
    ///1 (no division, reset value)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SYSDIV::B0x0
    }
    ///2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SYSDIV::B0x1
    }
    ///3
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SYSDIV::B0x2
    }
    ///4
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SYSDIV::B0x3
    }
    ///5
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == SYSDIV::B0x4
    }
    ///6
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == SYSDIV::B0x5
    }
    ///7
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == SYSDIV::B0x6
    }
    ///8
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == SYSDIV::B0x7
    }
}
///Field `SYSDIV` writer - Clock division factor for system clock Set and cleared by software. SYSCLK is result of the division by: Note: This bitfield is only available on STM32C071xx.
pub type SYSDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SYSDIV, crate::Safe>;
impl<'a, REG> SYSDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 (no division, reset value)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SYSDIV::B0x0)
    }
    ///2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SYSDIV::B0x1)
    }
    ///3
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SYSDIV::B0x2)
    }
    ///4
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SYSDIV::B0x3)
    }
    ///5
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SYSDIV::B0x4)
    }
    ///6
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SYSDIV::B0x5)
    }
    ///7
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SYSDIV::B0x6)
    }
    ///8
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SYSDIV::B0x7)
    }
}
/**HSI48 kernel clock division factor This bitfield controlled by software sets the division factor of the kernel clock divider to produce HSIKER clock:

Value on reset: 2*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HSIKERDIV {
    ///0: 1
    B0x0 = 0,
    ///1: 2
    B0x1 = 1,
    ///2: 3 (reset value)
    B0x2 = 2,
    ///3: 4
    B0x3 = 3,
    ///4: 5
    B0x4 = 4,
    ///5: 6
    B0x5 = 5,
    ///6: 7
    B0x6 = 6,
    ///7: 8
    B0x7 = 7,
}
impl From<HSIKERDIV> for u8 {
    #[inline(always)]
    fn from(variant: HSIKERDIV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HSIKERDIV {
    type Ux = u8;
}
impl crate::IsEnum for HSIKERDIV {}
///Field `HSIKERDIV` reader - HSI48 kernel clock division factor This bitfield controlled by software sets the division factor of the kernel clock divider to produce HSIKER clock:
pub type HSIKERDIV_R = crate::FieldReader<HSIKERDIV>;
impl HSIKERDIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIKERDIV {
        match self.bits {
            0 => HSIKERDIV::B0x0,
            1 => HSIKERDIV::B0x1,
            2 => HSIKERDIV::B0x2,
            3 => HSIKERDIV::B0x3,
            4 => HSIKERDIV::B0x4,
            5 => HSIKERDIV::B0x5,
            6 => HSIKERDIV::B0x6,
            7 => HSIKERDIV::B0x7,
            _ => unreachable!(),
        }
    }
    ///1
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSIKERDIV::B0x0
    }
    ///2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSIKERDIV::B0x1
    }
    ///3 (reset value)
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == HSIKERDIV::B0x2
    }
    ///4
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == HSIKERDIV::B0x3
    }
    ///5
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == HSIKERDIV::B0x4
    }
    ///6
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == HSIKERDIV::B0x5
    }
    ///7
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == HSIKERDIV::B0x6
    }
    ///8
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == HSIKERDIV::B0x7
    }
}
///Field `HSIKERDIV` writer - HSI48 kernel clock division factor This bitfield controlled by software sets the division factor of the kernel clock divider to produce HSIKER clock:
pub type HSIKERDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3, HSIKERDIV, crate::Safe>;
impl<'a, REG> HSIKERDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSIKERDIV::B0x0)
    }
    ///2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSIKERDIV::B0x1)
    }
    ///3 (reset value)
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(HSIKERDIV::B0x2)
    }
    ///4
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(HSIKERDIV::B0x3)
    }
    ///5
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(HSIKERDIV::B0x4)
    }
    ///6
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(HSIKERDIV::B0x5)
    }
    ///7
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(HSIKERDIV::B0x6)
    }
    ///8
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(HSIKERDIV::B0x7)
    }
}
/**HSI48 clock enable Set and cleared by software and hardware, with hardware taking priority. Kept low by hardware as long as the device is in a low-power mode. Kept high by hardware as long as the system is clocked with a clock derived from HSI48. This includes the exit from low-power modes and the system clock fall-back to HSI48 upon failing HSE oscillator clock selected as system clock source.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSION {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<HSION> for bool {
    #[inline(always)]
    fn from(variant: HSION) -> Self {
        variant as u8 != 0
    }
}
///Field `HSION` reader - HSI48 clock enable Set and cleared by software and hardware, with hardware taking priority. Kept low by hardware as long as the device is in a low-power mode. Kept high by hardware as long as the system is clocked with a clock derived from HSI48. This includes the exit from low-power modes and the system clock fall-back to HSI48 upon failing HSE oscillator clock selected as system clock source.
pub type HSION_R = crate::BitReader<HSION>;
impl HSION_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSION {
        match self.bits {
            false => HSION::B0x0,
            true => HSION::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSION::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSION::B0x1
    }
}
///Field `HSION` writer - HSI48 clock enable Set and cleared by software and hardware, with hardware taking priority. Kept low by hardware as long as the device is in a low-power mode. Kept high by hardware as long as the system is clocked with a clock derived from HSI48. This includes the exit from low-power modes and the system clock fall-back to HSI48 upon failing HSE oscillator clock selected as system clock source.
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG, HSION>;
impl<'a, REG> HSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::B0x1)
    }
}
/**HSI48 always-enable for peripheral kernels. Set and cleared by software. Setting the bit activates the HSI48 oscillator in Run and Stop modes, regardless of the HSION bit state. The HSI48 clock can only feed USART1, USART2, and I2C1 peripherals configured with HSI48 as kernel clock. Note: Keeping the HSI48 active in Stop mode allows speeding up the serial interface communication as the HSI48 clock is ready immediately upon exiting Stop mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIKERON {
    ///0: HSI48 oscillator enable depends on the HSION bit
    B0x0 = 0,
    ///1: HSI48 oscillator is active in Run and Stop modes
    B0x1 = 1,
}
impl From<HSIKERON> for bool {
    #[inline(always)]
    fn from(variant: HSIKERON) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIKERON` reader - HSI48 always-enable for peripheral kernels. Set and cleared by software. Setting the bit activates the HSI48 oscillator in Run and Stop modes, regardless of the HSION bit state. The HSI48 clock can only feed USART1, USART2, and I2C1 peripherals configured with HSI48 as kernel clock. Note: Keeping the HSI48 active in Stop mode allows speeding up the serial interface communication as the HSI48 clock is ready immediately upon exiting Stop mode.
pub type HSIKERON_R = crate::BitReader<HSIKERON>;
impl HSIKERON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIKERON {
        match self.bits {
            false => HSIKERON::B0x0,
            true => HSIKERON::B0x1,
        }
    }
    ///HSI48 oscillator enable depends on the HSION bit
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSIKERON::B0x0
    }
    ///HSI48 oscillator is active in Run and Stop modes
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSIKERON::B0x1
    }
}
///Field `HSIKERON` writer - HSI48 always-enable for peripheral kernels. Set and cleared by software. Setting the bit activates the HSI48 oscillator in Run and Stop modes, regardless of the HSION bit state. The HSI48 clock can only feed USART1, USART2, and I2C1 peripherals configured with HSI48 as kernel clock. Note: Keeping the HSI48 active in Stop mode allows speeding up the serial interface communication as the HSI48 clock is ready immediately upon exiting Stop mode.
pub type HSIKERON_W<'a, REG> = crate::BitWriter<'a, REG, HSIKERON>;
impl<'a, REG> HSIKERON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSI48 oscillator enable depends on the HSION bit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSIKERON::B0x0)
    }
    ///HSI48 oscillator is active in Run and Stop modes
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSIKERON::B0x1)
    }
}
/**HSI48 clock ready flag Set by hardware when the HSI48 oscillator is enabled through HSION and ready to use (stable). Note: Upon clearing HSION, HSIRDY goes low after six HSI48 clock cycles.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDY {
    ///0: Not ready
    B0x0 = 0,
    ///1: Ready
    B0x1 = 1,
}
impl From<HSIRDY> for bool {
    #[inline(always)]
    fn from(variant: HSIRDY) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIRDY` reader - HSI48 clock ready flag Set by hardware when the HSI48 oscillator is enabled through HSION and ready to use (stable). Note: Upon clearing HSION, HSIRDY goes low after six HSI48 clock cycles.
pub type HSIRDY_R = crate::BitReader<HSIRDY>;
impl HSIRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIRDY {
        match self.bits {
            false => HSIRDY::B0x0,
            true => HSIRDY::B0x1,
        }
    }
    ///Not ready
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSIRDY::B0x0
    }
    ///Ready
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSIRDY::B0x1
    }
}
/**HSI48 clock division factor This bitfield controlled by software sets the division factor of the HSI48 clock divider to produce HSISYS clock:

Value on reset: 2*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HSIDIV {
    ///0: 1
    B0x0 = 0,
    ///1: 2
    B0x1 = 1,
    ///2: 4 (reset value)
    B0x2 = 2,
    ///3: 8
    B0x3 = 3,
    ///4: 16
    B0x4 = 4,
    ///5: 32
    B0x5 = 5,
    ///6: 64
    B0x6 = 6,
    ///7: 128
    B0x7 = 7,
}
impl From<HSIDIV> for u8 {
    #[inline(always)]
    fn from(variant: HSIDIV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HSIDIV {
    type Ux = u8;
}
impl crate::IsEnum for HSIDIV {}
///Field `HSIDIV` reader - HSI48 clock division factor This bitfield controlled by software sets the division factor of the HSI48 clock divider to produce HSISYS clock:
pub type HSIDIV_R = crate::FieldReader<HSIDIV>;
impl HSIDIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIDIV {
        match self.bits {
            0 => HSIDIV::B0x0,
            1 => HSIDIV::B0x1,
            2 => HSIDIV::B0x2,
            3 => HSIDIV::B0x3,
            4 => HSIDIV::B0x4,
            5 => HSIDIV::B0x5,
            6 => HSIDIV::B0x6,
            7 => HSIDIV::B0x7,
            _ => unreachable!(),
        }
    }
    ///1
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSIDIV::B0x0
    }
    ///2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSIDIV::B0x1
    }
    ///4 (reset value)
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == HSIDIV::B0x2
    }
    ///8
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == HSIDIV::B0x3
    }
    ///16
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == HSIDIV::B0x4
    }
    ///32
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == HSIDIV::B0x5
    }
    ///64
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == HSIDIV::B0x6
    }
    ///128
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == HSIDIV::B0x7
    }
}
///Field `HSIDIV` writer - HSI48 clock division factor This bitfield controlled by software sets the division factor of the HSI48 clock divider to produce HSISYS clock:
pub type HSIDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3, HSIDIV, crate::Safe>;
impl<'a, REG> HSIDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::B0x0)
    }
    ///2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::B0x1)
    }
    ///4 (reset value)
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::B0x2)
    }
    ///8
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::B0x3)
    }
    ///16
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::B0x4)
    }
    ///32
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::B0x5)
    }
    ///64
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::B0x6)
    }
    ///128
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDIV::B0x7)
    }
}
/**HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, or Standby, or Shutdown mode. This bit cannot be cleared if the HSE oscillator is used directly or indirectly as the system clock.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEON {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<HSEON> for bool {
    #[inline(always)]
    fn from(variant: HSEON) -> Self {
        variant as u8 != 0
    }
}
///Field `HSEON` reader - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, or Standby, or Shutdown mode. This bit cannot be cleared if the HSE oscillator is used directly or indirectly as the system clock.
pub type HSEON_R = crate::BitReader<HSEON>;
impl HSEON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSEON {
        match self.bits {
            false => HSEON::B0x0,
            true => HSEON::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSEON::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSEON::B0x1
    }
}
///Field `HSEON` writer - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, or Standby, or Shutdown mode. This bit cannot be cleared if the HSE oscillator is used directly or indirectly as the system clock.
pub type HSEON_W<'a, REG> = crate::BitWriter<'a, REG, HSEON>;
impl<'a, REG> HSEON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSEON::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSEON::B0x1)
    }
}
/**HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable and ready for use. Note: Upon clearing HSEON, HSERDY goes low after six HSE clock cycles.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDY {
    ///0: Not ready
    B0x0 = 0,
    ///1: Ready
    B0x1 = 1,
}
impl From<HSERDY> for bool {
    #[inline(always)]
    fn from(variant: HSERDY) -> Self {
        variant as u8 != 0
    }
}
///Field `HSERDY` reader - HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable and ready for use. Note: Upon clearing HSEON, HSERDY goes low after six HSE clock cycles.
pub type HSERDY_R = crate::BitReader<HSERDY>;
impl HSERDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSERDY {
        match self.bits {
            false => HSERDY::B0x0,
            true => HSERDY::B0x1,
        }
    }
    ///Not ready
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSERDY::B0x0
    }
    ///Ready
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSERDY::B0x1
    }
}
/**HSE crystal oscillator bypass Set and cleared by software. When the bit is set, the internal HSE oscillator is bypassed for use of an external clock. The external clock must then be enabled with the HSEON bit set. Write access to the bit is only effective when the HSE oscillator is disabled.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEBYP {
    ///0: No bypass
    B0x0 = 0,
    ///1: Bypass
    B0x1 = 1,
}
impl From<HSEBYP> for bool {
    #[inline(always)]
    fn from(variant: HSEBYP) -> Self {
        variant as u8 != 0
    }
}
///Field `HSEBYP` reader - HSE crystal oscillator bypass Set and cleared by software. When the bit is set, the internal HSE oscillator is bypassed for use of an external clock. The external clock must then be enabled with the HSEON bit set. Write access to the bit is only effective when the HSE oscillator is disabled.
pub type HSEBYP_R = crate::BitReader<HSEBYP>;
impl HSEBYP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSEBYP {
        match self.bits {
            false => HSEBYP::B0x0,
            true => HSEBYP::B0x1,
        }
    }
    ///No bypass
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSEBYP::B0x0
    }
    ///Bypass
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSEBYP::B0x1
    }
}
///Field `HSEBYP` writer - HSE crystal oscillator bypass Set and cleared by software. When the bit is set, the internal HSE oscillator is bypassed for use of an external clock. The external clock must then be enabled with the HSEON bit set. Write access to the bit is only effective when the HSE oscillator is disabled.
pub type HSEBYP_W<'a, REG> = crate::BitWriter<'a, REG, HSEBYP>;
impl<'a, REG> HSEBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No bypass
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYP::B0x0)
    }
    ///Bypass
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYP::B0x1)
    }
}
/**Clock security system enable Set by software to enable the clock security system. When the bit is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. The bit is cleared by hardware upon reset.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSON {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<CSSON> for bool {
    #[inline(always)]
    fn from(variant: CSSON) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSON` reader - Clock security system enable Set by software to enable the clock security system. When the bit is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. The bit is cleared by hardware upon reset.
pub type CSSON_R = crate::BitReader<CSSON>;
impl CSSON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSSON {
        match self.bits {
            false => CSSON::B0x0,
            true => CSSON::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CSSON::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CSSON::B0x1
    }
}
///Field `CSSON` writer - Clock security system enable Set by software to enable the clock security system. When the bit is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. The bit is cleared by hardware upon reset.
pub type CSSON_W<'a, REG> = crate::BitWriter<'a, REG, CSSON>;
impl<'a, REG> CSSON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CSSON::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CSSON::B0x1)
    }
}
/**HSIUSB48 clock enable Set and cleared by software and hardware, with hardware taking priority. Kept low by hardware as long as the device is in a low-power mode. Kept high by hardware as long as the system is clocked from HSIUSB48. Note: Only applicable on STM32C071xx, reserved on other devices.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIUSB48ON {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<HSIUSB48ON> for bool {
    #[inline(always)]
    fn from(variant: HSIUSB48ON) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIUSB48ON` reader - HSIUSB48 clock enable Set and cleared by software and hardware, with hardware taking priority. Kept low by hardware as long as the device is in a low-power mode. Kept high by hardware as long as the system is clocked from HSIUSB48. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type HSIUSB48ON_R = crate::BitReader<HSIUSB48ON>;
impl HSIUSB48ON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIUSB48ON {
        match self.bits {
            false => HSIUSB48ON::B0x0,
            true => HSIUSB48ON::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSIUSB48ON::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSIUSB48ON::B0x1
    }
}
///Field `HSIUSB48ON` writer - HSIUSB48 clock enable Set and cleared by software and hardware, with hardware taking priority. Kept low by hardware as long as the device is in a low-power mode. Kept high by hardware as long as the system is clocked from HSIUSB48. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type HSIUSB48ON_W<'a, REG> = crate::BitWriter<'a, REG, HSIUSB48ON>;
impl<'a, REG> HSIUSB48ON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSIUSB48ON::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSIUSB48ON::B0x1)
    }
}
/**HSIUSB48 clock ready flag Set by hardware when the HSIUSB48 oscillator is enabled through HSIUSB48ON and ready to use (stable). Note: Only applicable on STM32C071xx, reserved on other devices.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIUSB48RDY {
    ///0: Not ready
    B0x0 = 0,
    ///1: Ready
    B0x1 = 1,
}
impl From<HSIUSB48RDY> for bool {
    #[inline(always)]
    fn from(variant: HSIUSB48RDY) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIUSB48RDY` reader - HSIUSB48 clock ready flag Set by hardware when the HSIUSB48 oscillator is enabled through HSIUSB48ON and ready to use (stable). Note: Only applicable on STM32C071xx, reserved on other devices.
pub type HSIUSB48RDY_R = crate::BitReader<HSIUSB48RDY>;
impl HSIUSB48RDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIUSB48RDY {
        match self.bits {
            false => HSIUSB48RDY::B0x0,
            true => HSIUSB48RDY::B0x1,
        }
    }
    ///Not ready
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSIUSB48RDY::B0x0
    }
    ///Ready
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSIUSB48RDY::B0x1
    }
}
///Field `HSIUSB48RDY` writer - HSIUSB48 clock ready flag Set by hardware when the HSIUSB48 oscillator is enabled through HSIUSB48ON and ready to use (stable). Note: Only applicable on STM32C071xx, reserved on other devices.
pub type HSIUSB48RDY_W<'a, REG> = crate::BitWriter<'a, REG, HSIUSB48RDY>;
impl<'a, REG> HSIUSB48RDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not ready
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSIUSB48RDY::B0x0)
    }
    ///Ready
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSIUSB48RDY::B0x1)
    }
}
impl R {
    ///Bits 2:4 - Clock division factor for system clock Set and cleared by software. SYSCLK is result of the division by: Note: This bitfield is only available on STM32C071xx.
    #[inline(always)]
    pub fn sysdiv(&self) -> SYSDIV_R {
        SYSDIV_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bits 5:7 - HSI48 kernel clock division factor This bitfield controlled by software sets the division factor of the kernel clock divider to produce HSIKER clock:
    #[inline(always)]
    pub fn hsikerdiv(&self) -> HSIKERDIV_R {
        HSIKERDIV_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bit 8 - HSI48 clock enable Set and cleared by software and hardware, with hardware taking priority. Kept low by hardware as long as the device is in a low-power mode. Kept high by hardware as long as the system is clocked with a clock derived from HSI48. This includes the exit from low-power modes and the system clock fall-back to HSI48 upon failing HSE oscillator clock selected as system clock source.
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HSI48 always-enable for peripheral kernels. Set and cleared by software. Setting the bit activates the HSI48 oscillator in Run and Stop modes, regardless of the HSION bit state. The HSI48 clock can only feed USART1, USART2, and I2C1 peripherals configured with HSI48 as kernel clock. Note: Keeping the HSI48 active in Stop mode allows speeding up the serial interface communication as the HSI48 clock is ready immediately upon exiting Stop mode.
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSI48 clock ready flag Set by hardware when the HSI48 oscillator is enabled through HSION and ready to use (stable). Note: Upon clearing HSION, HSIRDY goes low after six HSI48 clock cycles.
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:13 - HSI48 clock division factor This bitfield controlled by software sets the division factor of the HSI48 clock divider to produce HSISYS clock:
    #[inline(always)]
    pub fn hsidiv(&self) -> HSIDIV_R {
        HSIDIV_R::new(((self.bits >> 11) & 7) as u8)
    }
    ///Bit 16 - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, or Standby, or Shutdown mode. This bit cannot be cleared if the HSE oscillator is used directly or indirectly as the system clock.
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable and ready for use. Note: Upon clearing HSEON, HSERDY goes low after six HSE clock cycles.
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - HSE crystal oscillator bypass Set and cleared by software. When the bit is set, the internal HSE oscillator is bypassed for use of an external clock. The external clock must then be enabled with the HSEON bit set. Write access to the bit is only effective when the HSE oscillator is disabled.
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Clock security system enable Set by software to enable the clock security system. When the bit is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. The bit is cleared by hardware upon reset.
    #[inline(always)]
    pub fn csson(&self) -> CSSON_R {
        CSSON_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 22 - HSIUSB48 clock enable Set and cleared by software and hardware, with hardware taking priority. Kept low by hardware as long as the device is in a low-power mode. Kept high by hardware as long as the system is clocked from HSIUSB48. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn hsiusb48on(&self) -> HSIUSB48ON_R {
        HSIUSB48ON_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - HSIUSB48 clock ready flag Set by hardware when the HSIUSB48 oscillator is enabled through HSIUSB48ON and ready to use (stable). Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn hsiusb48rdy(&self) -> HSIUSB48RDY_R {
        HSIUSB48RDY_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CR")
            .field("sysdiv", &self.sysdiv())
            .field("hsikerdiv", &self.hsikerdiv())
            .field("hsion", &self.hsion())
            .field("hsikeron", &self.hsikeron())
            .field("hsirdy", &self.hsirdy())
            .field("hsidiv", &self.hsidiv())
            .field("hseon", &self.hseon())
            .field("hserdy", &self.hserdy())
            .field("hsebyp", &self.hsebyp())
            .field("csson", &self.csson())
            .field("hsiusb48on", &self.hsiusb48on())
            .field("hsiusb48rdy", &self.hsiusb48rdy())
            .finish()
    }
}
impl W {
    ///Bits 2:4 - Clock division factor for system clock Set and cleared by software. SYSCLK is result of the division by: Note: This bitfield is only available on STM32C071xx.
    #[inline(always)]
    pub fn sysdiv(&mut self) -> SYSDIV_W<'_, RCC_CRrs> {
        SYSDIV_W::new(self, 2)
    }
    ///Bits 5:7 - HSI48 kernel clock division factor This bitfield controlled by software sets the division factor of the kernel clock divider to produce HSIKER clock:
    #[inline(always)]
    pub fn hsikerdiv(&mut self) -> HSIKERDIV_W<'_, RCC_CRrs> {
        HSIKERDIV_W::new(self, 5)
    }
    ///Bit 8 - HSI48 clock enable Set and cleared by software and hardware, with hardware taking priority. Kept low by hardware as long as the device is in a low-power mode. Kept high by hardware as long as the system is clocked with a clock derived from HSI48. This includes the exit from low-power modes and the system clock fall-back to HSI48 upon failing HSE oscillator clock selected as system clock source.
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W<'_, RCC_CRrs> {
        HSION_W::new(self, 8)
    }
    ///Bit 9 - HSI48 always-enable for peripheral kernels. Set and cleared by software. Setting the bit activates the HSI48 oscillator in Run and Stop modes, regardless of the HSION bit state. The HSI48 clock can only feed USART1, USART2, and I2C1 peripherals configured with HSI48 as kernel clock. Note: Keeping the HSI48 active in Stop mode allows speeding up the serial interface communication as the HSI48 clock is ready immediately upon exiting Stop mode.
    #[inline(always)]
    pub fn hsikeron(&mut self) -> HSIKERON_W<'_, RCC_CRrs> {
        HSIKERON_W::new(self, 9)
    }
    ///Bits 11:13 - HSI48 clock division factor This bitfield controlled by software sets the division factor of the HSI48 clock divider to produce HSISYS clock:
    #[inline(always)]
    pub fn hsidiv(&mut self) -> HSIDIV_W<'_, RCC_CRrs> {
        HSIDIV_W::new(self, 11)
    }
    ///Bit 16 - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, or Standby, or Shutdown mode. This bit cannot be cleared if the HSE oscillator is used directly or indirectly as the system clock.
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W<'_, RCC_CRrs> {
        HSEON_W::new(self, 16)
    }
    ///Bit 18 - HSE crystal oscillator bypass Set and cleared by software. When the bit is set, the internal HSE oscillator is bypassed for use of an external clock. The external clock must then be enabled with the HSEON bit set. Write access to the bit is only effective when the HSE oscillator is disabled.
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HSEBYP_W<'_, RCC_CRrs> {
        HSEBYP_W::new(self, 18)
    }
    ///Bit 19 - Clock security system enable Set by software to enable the clock security system. When the bit is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. The bit is cleared by hardware upon reset.
    #[inline(always)]
    pub fn csson(&mut self) -> CSSON_W<'_, RCC_CRrs> {
        CSSON_W::new(self, 19)
    }
    ///Bit 22 - HSIUSB48 clock enable Set and cleared by software and hardware, with hardware taking priority. Kept low by hardware as long as the device is in a low-power mode. Kept high by hardware as long as the system is clocked from HSIUSB48. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn hsiusb48on(&mut self) -> HSIUSB48ON_W<'_, RCC_CRrs> {
        HSIUSB48ON_W::new(self, 22)
    }
    ///Bit 23 - HSIUSB48 clock ready flag Set by hardware when the HSIUSB48 oscillator is enabled through HSIUSB48ON and ready to use (stable). Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn hsiusb48rdy(&mut self) -> HSIUSB48RDY_W<'_, RCC_CRrs> {
        HSIUSB48RDY_W::new(self, 23)
    }
}
/**RCC clock control register

You can [`read`](crate::Reg::read) this register and get [`rcc_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#RCC:RCC_CR)*/
pub struct RCC_CRrs;
impl crate::RegisterSpec for RCC_CRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_cr::R`](R) reader structure
impl crate::Readable for RCC_CRrs {}
///`write(|w| ..)` method takes [`rcc_cr::W`](W) writer structure
impl crate::Writable for RCC_CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCC_CR to value 0x1540
impl crate::Resettable for RCC_CRrs {
    const RESET_VALUE: u32 = 0x1540;
}
