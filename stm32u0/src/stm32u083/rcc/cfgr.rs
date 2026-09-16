///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `SW` reader - System clock switch This bitfield is controlled by software and hardware. The bitfield selects the clock for SYSCLK as follows: Others: Reserved The setting is forced by hardware to 000 (HSISYS selected) when the MCU exits Stop, Standby, or Shutdown mode, or when the setting is 001 (HSE selected) and HSE oscillator failure is detected.
pub type SW_R = crate::FieldReader;
///Field `SW` writer - System clock switch This bitfield is controlled by software and hardware. The bitfield selects the clock for SYSCLK as follows: Others: Reserved The setting is forced by hardware to 000 (HSISYS selected) when the MCU exits Stop, Standby, or Shutdown mode, or when the setting is 001 (HSE selected) and HSE oscillator failure is detected.
pub type SW_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SWS` reader - System clock switch status This bitfield is controlled by hardware to indicate the clock source used as system clock: Others: Reserved
pub type SWS_R = crate::FieldReader;
///Field `HPRE` reader - AHB prescaler This bitfield is controlled by software. To produce HCLK clock, it sets the division factor of SYSCLK clock as follows: 0xxx: 1 Caution: Depending on the device voltage range, the software has to set correctly these bits to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to Section14.1.4: Dynamic voltage scaling management). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value has been taken into account.
pub type HPRE_R = crate::FieldReader;
///Field `HPRE` writer - AHB prescaler This bitfield is controlled by software. To produce HCLK clock, it sets the division factor of SYSCLK clock as follows: 0xxx: 1 Caution: Depending on the device voltage range, the software has to set correctly these bits to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to Section14.1.4: Dynamic voltage scaling management). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value has been taken into account.
pub type HPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PPRE` reader - APB prescaler This bitfield is controlled by software. To produce PCLK clock, it sets the division factor of HCLK clock as follows: 0xx: 1
pub type PPRE_R = crate::FieldReader;
///Field `PPRE` writer - APB prescaler This bitfield is controlled by software. To produce PCLK clock, it sets the division factor of HCLK clock as follows: 0xx: 1
pub type PPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `STOPWUCK` reader - Wake-up from Stop and CSS backup clock selection Set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the Clock Security System on HSE. Warning: STOPWUCK must not be modified when the Clock Security System is enabled by HSECSSON in RCC_CR register and the system clock is HSE (SWS=10) or a switch on HSE is requested (SW=10).
pub type STOPWUCK_R = crate::BitReader;
///Field `STOPWUCK` writer - Wake-up from Stop and CSS backup clock selection Set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the Clock Security System on HSE. Warning: STOPWUCK must not be modified when the Clock Security System is enabled by HSECSSON in RCC_CR register and the system clock is HSE (SWS=10) or a switch on HSE is requested (SW=10).
pub type STOPWUCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCO2SEL` reader - Microcontroller clock output 2 clock selector This bitfield is controlled by software. It sets the clock selector for MCO2 output as follows: Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO2 clock source switching.
pub type MCO2SEL_R = crate::FieldReader;
///Field `MCO2SEL` writer - Microcontroller clock output 2 clock selector This bitfield is controlled by software. It sets the clock selector for MCO2 output as follows: Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO2 clock source switching.
pub type MCO2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MCO2PRE` reader - Microcontroller clock output 2 prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO2 output as follows: ... Others: reserved It is highly recommended to set this field before the MCO2 output is enabled.
pub type MCO2PRE_R = crate::FieldReader;
///Field `MCO2PRE` writer - Microcontroller clock output 2 prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO2 output as follows: ... Others: reserved It is highly recommended to set this field before the MCO2 output is enabled.
pub type MCO2PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MCOSEL` reader - Microcontroller clock output clock selector This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.
pub type MCOSEL_R = crate::FieldReader;
///Field `MCOSEL` writer - Microcontroller clock output clock selector This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.
pub type MCOSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MCOPRE` reader - Microcontroller clock output prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO output as follows: ... Others: reserved It is highly recommended to set this field before the MCO output is enabled.
pub type MCOPRE_R = crate::FieldReader;
///Field `MCOPRE` writer - Microcontroller clock output prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO output as follows: ... Others: reserved It is highly recommended to set this field before the MCO output is enabled.
pub type MCOPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:2 - System clock switch This bitfield is controlled by software and hardware. The bitfield selects the clock for SYSCLK as follows: Others: Reserved The setting is forced by hardware to 000 (HSISYS selected) when the MCU exits Stop, Standby, or Shutdown mode, or when the setting is 001 (HSE selected) and HSE oscillator failure is detected.
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - System clock switch status This bitfield is controlled by hardware to indicate the clock source used as system clock: Others: Reserved
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 8:11 - AHB prescaler This bitfield is controlled by software. To produce HCLK clock, it sets the division factor of SYSCLK clock as follows: 0xxx: 1 Caution: Depending on the device voltage range, the software has to set correctly these bits to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to Section14.1.4: Dynamic voltage scaling management). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value has been taken into account.
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:14 - APB prescaler This bitfield is controlled by software. To produce PCLK clock, it sets the division factor of HCLK clock as follows: 0xx: 1
    #[inline(always)]
    pub fn ppre(&self) -> PPRE_R {
        PPRE_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - Wake-up from Stop and CSS backup clock selection Set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the Clock Security System on HSE. Warning: STOPWUCK must not be modified when the Clock Security System is enabled by HSECSSON in RCC_CR register and the system clock is HSE (SWS=10) or a switch on HSE is requested (SW=10).
    #[inline(always)]
    pub fn stopwuck(&self) -> STOPWUCK_R {
        STOPWUCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:19 - Microcontroller clock output 2 clock selector This bitfield is controlled by software. It sets the clock selector for MCO2 output as follows: Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO2 clock source switching.
    #[inline(always)]
    pub fn mco2sel(&self) -> MCO2SEL_R {
        MCO2SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Microcontroller clock output 2 prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO2 output as follows: ... Others: reserved It is highly recommended to set this field before the MCO2 output is enabled.
    #[inline(always)]
    pub fn mco2pre(&self) -> MCO2PRE_R {
        MCO2PRE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Microcontroller clock output clock selector This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.
    #[inline(always)]
    pub fn mcosel(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Microcontroller clock output prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO output as follows: ... Others: reserved It is highly recommended to set this field before the MCO output is enabled.
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("sw", &self.sw())
            .field("sws", &self.sws())
            .field("hpre", &self.hpre())
            .field("ppre", &self.ppre())
            .field("stopwuck", &self.stopwuck())
            .field("mco2sel", &self.mco2sel())
            .field("mco2pre", &self.mco2pre())
            .field("mcosel", &self.mcosel())
            .field("mcopre", &self.mcopre())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - System clock switch This bitfield is controlled by software and hardware. The bitfield selects the clock for SYSCLK as follows: Others: Reserved The setting is forced by hardware to 000 (HSISYS selected) when the MCU exits Stop, Standby, or Shutdown mode, or when the setting is 001 (HSE selected) and HSE oscillator failure is detected.
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W<'_, CFGRrs> {
        SW_W::new(self, 0)
    }
    ///Bits 8:11 - AHB prescaler This bitfield is controlled by software. To produce HCLK clock, it sets the division factor of SYSCLK clock as follows: 0xxx: 1 Caution: Depending on the device voltage range, the software has to set correctly these bits to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to Section14.1.4: Dynamic voltage scaling management). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value has been taken into account.
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W<'_, CFGRrs> {
        HPRE_W::new(self, 8)
    }
    ///Bits 12:14 - APB prescaler This bitfield is controlled by software. To produce PCLK clock, it sets the division factor of HCLK clock as follows: 0xx: 1
    #[inline(always)]
    pub fn ppre(&mut self) -> PPRE_W<'_, CFGRrs> {
        PPRE_W::new(self, 12)
    }
    ///Bit 15 - Wake-up from Stop and CSS backup clock selection Set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the Clock Security System on HSE. Warning: STOPWUCK must not be modified when the Clock Security System is enabled by HSECSSON in RCC_CR register and the system clock is HSE (SWS=10) or a switch on HSE is requested (SW=10).
    #[inline(always)]
    pub fn stopwuck(&mut self) -> STOPWUCK_W<'_, CFGRrs> {
        STOPWUCK_W::new(self, 15)
    }
    ///Bits 16:19 - Microcontroller clock output 2 clock selector This bitfield is controlled by software. It sets the clock selector for MCO2 output as follows: Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO2 clock source switching.
    #[inline(always)]
    pub fn mco2sel(&mut self) -> MCO2SEL_W<'_, CFGRrs> {
        MCO2SEL_W::new(self, 16)
    }
    ///Bits 20:23 - Microcontroller clock output 2 prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO2 output as follows: ... Others: reserved It is highly recommended to set this field before the MCO2 output is enabled.
    #[inline(always)]
    pub fn mco2pre(&mut self) -> MCO2PRE_W<'_, CFGRrs> {
        MCO2PRE_W::new(self, 20)
    }
    ///Bits 24:27 - Microcontroller clock output clock selector This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.
    #[inline(always)]
    pub fn mcosel(&mut self) -> MCOSEL_W<'_, CFGRrs> {
        MCOSEL_W::new(self, 24)
    }
    ///Bits 28:31 - Microcontroller clock output prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO output as follows: ... Others: reserved It is highly recommended to set this field before the MCO output is enabled.
    #[inline(always)]
    pub fn mcopre(&mut self) -> MCOPRE_W<'_, CFGRrs> {
        MCOPRE_W::new(self, 28)
    }
}
/**Clock configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {}
