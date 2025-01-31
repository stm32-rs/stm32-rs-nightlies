///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `SYSDIV` reader - Clock division factor for system clock Set and cleared by software. SYSCLK is result of the division by: Note: This bitfield is only available on STM32C071xx.
pub type SYSDIV_R = crate::FieldReader;
///Field `SYSDIV` writer - Clock division factor for system clock Set and cleared by software. SYSCLK is result of the division by: Note: This bitfield is only available on STM32C071xx.
pub type SYSDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `HSIKERDIV` reader - HSI48 kernel clock division factor This bitfield controlled by software sets the division factor of the kernel clock divider to produce HSIKER clock:
pub type HSIKERDIV_R = crate::FieldReader;
///Field `HSIKERDIV` writer - HSI48 kernel clock division factor This bitfield controlled by software sets the division factor of the kernel clock divider to produce HSIKER clock:
pub type HSIKERDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `HSION` reader - HSI48 clock enable Set and cleared by software and hardware, with hardware taking priority. Kept low by hardware as long as the device is in a low-power mode. Kept high by hardware as long as the system is clocked with a clock derived from HSI48. This includes the exit from low-power modes and the system clock fall-back to HSI48 upon failing HSE oscillator clock selected as system clock source.
pub type HSION_R = crate::BitReader;
///Field `HSION` writer - HSI48 clock enable Set and cleared by software and hardware, with hardware taking priority. Kept low by hardware as long as the device is in a low-power mode. Kept high by hardware as long as the system is clocked with a clock derived from HSI48. This includes the exit from low-power modes and the system clock fall-back to HSI48 upon failing HSE oscillator clock selected as system clock source.
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIKERON` reader - HSI48 always-enable for peripheral kernels. Set and cleared by software. Setting the bit activates the HSI48 oscillator in Run and Stop modes, regardless of the HSION bit state. The HSI48 clock can only feed USART1, USART2, and I2C1 peripherals configured with HSI48 as kernel clock. Note: Keeping the HSI48 active in Stop mode allows speeding up the serial interface communication as the HSI48 clock is ready immediately upon exiting Stop mode.
pub type HSIKERON_R = crate::BitReader;
///Field `HSIKERON` writer - HSI48 always-enable for peripheral kernels. Set and cleared by software. Setting the bit activates the HSI48 oscillator in Run and Stop modes, regardless of the HSION bit state. The HSI48 clock can only feed USART1, USART2, and I2C1 peripherals configured with HSI48 as kernel clock. Note: Keeping the HSI48 active in Stop mode allows speeding up the serial interface communication as the HSI48 clock is ready immediately upon exiting Stop mode.
pub type HSIKERON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIRDY` reader - HSI48 clock ready flag Set by hardware when the HSI48 oscillator is enabled through HSION and ready to use (stable). Note: Upon clearing HSION, HSIRDY goes low after six HSI48 clock cycles.
pub type HSIRDY_R = crate::BitReader;
///Field `HSIDIV` reader - HSI48 clock division factor This bitfield controlled by software sets the division factor of the HSI48 clock divider to produce HSISYS clock:
pub type HSIDIV_R = crate::FieldReader;
///Field `HSIDIV` writer - HSI48 clock division factor This bitfield controlled by software sets the division factor of the HSI48 clock divider to produce HSISYS clock:
pub type HSIDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `HSEON` reader - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, or Standby, or Shutdown mode. This bit cannot be cleared if the HSE oscillator is used directly or indirectly as the system clock.
pub type HSEON_R = crate::BitReader;
///Field `HSEON` writer - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, or Standby, or Shutdown mode. This bit cannot be cleared if the HSE oscillator is used directly or indirectly as the system clock.
pub type HSEON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSERDY` reader - HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable and ready for use. Note: Upon clearing HSEON, HSERDY goes low after six HSE clock cycles.
pub type HSERDY_R = crate::BitReader;
///Field `HSEBYP` reader - HSE crystal oscillator bypass Set and cleared by software. When the bit is set, the internal HSE oscillator is bypassed for use of an external clock. The external clock must then be enabled with the HSEON bit set. Write access to the bit is only effective when the HSE oscillator is disabled.
pub type HSEBYP_R = crate::BitReader;
///Field `HSEBYP` writer - HSE crystal oscillator bypass Set and cleared by software. When the bit is set, the internal HSE oscillator is bypassed for use of an external clock. The external clock must then be enabled with the HSEON bit set. Write access to the bit is only effective when the HSE oscillator is disabled.
pub type HSEBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSSON` reader - Clock security system enable Set by software to enable the clock security system. When the bit is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. The bit is cleared by hardware upon reset.
pub type CSSON_R = crate::BitReader;
///Field `CSSON` writer - Clock security system enable Set by software to enable the clock security system. When the bit is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. The bit is cleared by hardware upon reset.
pub type CSSON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIUSB48ON` reader - HSIUSB48 clock enable Set and cleared by software and hardware, with hardware taking priority. Kept low by hardware as long as the device is in a low-power mode. Kept high by hardware as long as the system is clocked from HSIUSB48. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type HSIUSB48ON_R = crate::BitReader;
///Field `HSIUSB48ON` writer - HSIUSB48 clock enable Set and cleared by software and hardware, with hardware taking priority. Kept low by hardware as long as the device is in a low-power mode. Kept high by hardware as long as the system is clocked from HSIUSB48. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type HSIUSB48ON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIUSB48RDY` reader - HSIUSB48 clock ready flag Set by hardware when the HSIUSB48 oscillator is enabled through HSIUSB48ON and ready to use (stable). Note: Only applicable on STM32C071xx, reserved on other devices.
pub type HSIUSB48RDY_R = crate::BitReader;
///Field `HSIUSB48RDY` writer - HSIUSB48 clock ready flag Set by hardware when the HSIUSB48 oscillator is enabled through HSIUSB48ON and ready to use (stable). Note: Only applicable on STM32C071xx, reserved on other devices.
pub type HSIUSB48RDY_W<'a, REG> = crate::BitWriter<'a, REG>;
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
        f.debug_struct("CR")
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
    pub fn sysdiv(&mut self) -> SYSDIV_W<CRrs> {
        SYSDIV_W::new(self, 2)
    }
    ///Bits 5:7 - HSI48 kernel clock division factor This bitfield controlled by software sets the division factor of the kernel clock divider to produce HSIKER clock:
    #[inline(always)]
    pub fn hsikerdiv(&mut self) -> HSIKERDIV_W<CRrs> {
        HSIKERDIV_W::new(self, 5)
    }
    ///Bit 8 - HSI48 clock enable Set and cleared by software and hardware, with hardware taking priority. Kept low by hardware as long as the device is in a low-power mode. Kept high by hardware as long as the system is clocked with a clock derived from HSI48. This includes the exit from low-power modes and the system clock fall-back to HSI48 upon failing HSE oscillator clock selected as system clock source.
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W<CRrs> {
        HSION_W::new(self, 8)
    }
    ///Bit 9 - HSI48 always-enable for peripheral kernels. Set and cleared by software. Setting the bit activates the HSI48 oscillator in Run and Stop modes, regardless of the HSION bit state. The HSI48 clock can only feed USART1, USART2, and I2C1 peripherals configured with HSI48 as kernel clock. Note: Keeping the HSI48 active in Stop mode allows speeding up the serial interface communication as the HSI48 clock is ready immediately upon exiting Stop mode.
    #[inline(always)]
    pub fn hsikeron(&mut self) -> HSIKERON_W<CRrs> {
        HSIKERON_W::new(self, 9)
    }
    ///Bits 11:13 - HSI48 clock division factor This bitfield controlled by software sets the division factor of the HSI48 clock divider to produce HSISYS clock:
    #[inline(always)]
    pub fn hsidiv(&mut self) -> HSIDIV_W<CRrs> {
        HSIDIV_W::new(self, 11)
    }
    ///Bit 16 - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, or Standby, or Shutdown mode. This bit cannot be cleared if the HSE oscillator is used directly or indirectly as the system clock.
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W<CRrs> {
        HSEON_W::new(self, 16)
    }
    ///Bit 18 - HSE crystal oscillator bypass Set and cleared by software. When the bit is set, the internal HSE oscillator is bypassed for use of an external clock. The external clock must then be enabled with the HSEON bit set. Write access to the bit is only effective when the HSE oscillator is disabled.
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HSEBYP_W<CRrs> {
        HSEBYP_W::new(self, 18)
    }
    ///Bit 19 - Clock security system enable Set by software to enable the clock security system. When the bit is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. The bit is cleared by hardware upon reset.
    #[inline(always)]
    pub fn csson(&mut self) -> CSSON_W<CRrs> {
        CSSON_W::new(self, 19)
    }
    ///Bit 22 - HSIUSB48 clock enable Set and cleared by software and hardware, with hardware taking priority. Kept low by hardware as long as the device is in a low-power mode. Kept high by hardware as long as the system is clocked from HSIUSB48. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn hsiusb48on(&mut self) -> HSIUSB48ON_W<CRrs> {
        HSIUSB48ON_W::new(self, 22)
    }
    ///Bit 23 - HSIUSB48 clock ready flag Set by hardware when the HSIUSB48 oscillator is enabled through HSIUSB48ON and ready to use (stable). Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn hsiusb48rdy(&mut self) -> HSIUSB48RDY_W<CRrs> {
        HSIUSB48RDY_W::new(self, 23)
    }
}
/**RCC clock control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#RCC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR to value 0x1540
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x1540;
}
