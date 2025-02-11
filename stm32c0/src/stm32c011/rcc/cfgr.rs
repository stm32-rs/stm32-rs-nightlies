///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `SW` reader - System clock switch This bitfield is controlled by software and hardware. The bitfield selects the clock for SYSCLK as follows: Others: Reserved The setting is forced by hardware to 000 (HSISYS selected) when the MCU exits Stop, or Standby, or Shutdown mode, or when the setting is 001 (HSE selected) and HSE oscillator failure is detected.
pub type SW_R = crate::FieldReader;
///Field `SW` writer - System clock switch This bitfield is controlled by software and hardware. The bitfield selects the clock for SYSCLK as follows: Others: Reserved The setting is forced by hardware to 000 (HSISYS selected) when the MCU exits Stop, or Standby, or Shutdown mode, or when the setting is 001 (HSE selected) and HSE oscillator failure is detected.
pub type SW_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SWS` reader - System clock switch status This bitfield is controlled by hardware to indicate the clock source used as system clock: Others: Reserved
pub type SWS_R = crate::FieldReader;
///Field `HPRE` reader - AHB prescaler This bitfield is controlled by software. To produce HCLK clock, it sets the division factor of SYSCLK clock as follows: 0xxx: 1
pub type HPRE_R = crate::FieldReader;
///Field `HPRE` writer - AHB prescaler This bitfield is controlled by software. To produce HCLK clock, it sets the division factor of SYSCLK clock as follows: 0xxx: 1
pub type HPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PPRE` reader - APB prescaler This bitfield is controlled by software. To produce PCLK clock, it sets the division factor of HCLK clock as follows: 0xx: 1
pub type PPRE_R = crate::FieldReader;
///Field `PPRE` writer - APB prescaler This bitfield is controlled by software. To produce PCLK clock, it sets the division factor of HCLK clock as follows: 0xx: 1
pub type PPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MCO2SEL` reader - Microcontroller clock output 2 clock selector This bitfield is controlled by software. It sets the clock selector for MCO2 output as follows: This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Note: This clock output may have some truncated cycles at startup or during MCO2 clock source switching.
pub type MCO2SEL_R = crate::FieldReader;
///Field `MCO2SEL` writer - Microcontroller clock output 2 clock selector This bitfield is controlled by software. It sets the clock selector for MCO2 output as follows: This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Note: This clock output may have some truncated cycles at startup or during MCO2 clock source switching.
pub type MCO2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MCO2PRE` reader - Microcontroller clock output 2 prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO2 output as follows: ... It is highly recommended to set this field before the MCO2 output is enabled.
pub type MCO2PRE_R = crate::FieldReader;
///Field `MCO2PRE` writer - Microcontroller clock output 2 prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO2 output as follows: ... It is highly recommended to set this field before the MCO2 output is enabled.
pub type MCO2PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MCOSEL` reader - Microcontroller clock output clock selector This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Note: This clock output may have some truncated cycles at startup or during MCO clock source switching. Any other value means no clock on MCO.
pub type MCOSEL_R = crate::FieldReader;
///Field `MCOSEL` writer - Microcontroller clock output clock selector This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Note: This clock output may have some truncated cycles at startup or during MCO clock source switching. Any other value means no clock on MCO.
pub type MCOSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MCOPRE` reader - Microcontroller clock output prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO output as follows: ... It is highly recommended to set this field before the MCO output is enabled.
pub type MCOPRE_R = crate::FieldReader;
///Field `MCOPRE` writer - Microcontroller clock output prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO output as follows: ... It is highly recommended to set this field before the MCO output is enabled.
pub type MCOPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:2 - System clock switch This bitfield is controlled by software and hardware. The bitfield selects the clock for SYSCLK as follows: Others: Reserved The setting is forced by hardware to 000 (HSISYS selected) when the MCU exits Stop, or Standby, or Shutdown mode, or when the setting is 001 (HSE selected) and HSE oscillator failure is detected.
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - System clock switch status This bitfield is controlled by hardware to indicate the clock source used as system clock: Others: Reserved
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 8:11 - AHB prescaler This bitfield is controlled by software. To produce HCLK clock, it sets the division factor of SYSCLK clock as follows: 0xxx: 1
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:14 - APB prescaler This bitfield is controlled by software. To produce PCLK clock, it sets the division factor of HCLK clock as follows: 0xx: 1
    #[inline(always)]
    pub fn ppre(&self) -> PPRE_R {
        PPRE_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:19 - Microcontroller clock output 2 clock selector This bitfield is controlled by software. It sets the clock selector for MCO2 output as follows: This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Note: This clock output may have some truncated cycles at startup or during MCO2 clock source switching.
    #[inline(always)]
    pub fn mco2sel(&self) -> MCO2SEL_R {
        MCO2SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Microcontroller clock output 2 prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO2 output as follows: ... It is highly recommended to set this field before the MCO2 output is enabled.
    #[inline(always)]
    pub fn mco2pre(&self) -> MCO2PRE_R {
        MCO2PRE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Microcontroller clock output clock selector This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Note: This clock output may have some truncated cycles at startup or during MCO clock source switching. Any other value means no clock on MCO.
    #[inline(always)]
    pub fn mcosel(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Microcontroller clock output prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO output as follows: ... It is highly recommended to set this field before the MCO output is enabled.
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
            .field("mco2sel", &self.mco2sel())
            .field("mco2pre", &self.mco2pre())
            .field("mcosel", &self.mcosel())
            .field("mcopre", &self.mcopre())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - System clock switch This bitfield is controlled by software and hardware. The bitfield selects the clock for SYSCLK as follows: Others: Reserved The setting is forced by hardware to 000 (HSISYS selected) when the MCU exits Stop, or Standby, or Shutdown mode, or when the setting is 001 (HSE selected) and HSE oscillator failure is detected.
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W<CFGRrs> {
        SW_W::new(self, 0)
    }
    ///Bits 8:11 - AHB prescaler This bitfield is controlled by software. To produce HCLK clock, it sets the division factor of SYSCLK clock as follows: 0xxx: 1
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W<CFGRrs> {
        HPRE_W::new(self, 8)
    }
    ///Bits 12:14 - APB prescaler This bitfield is controlled by software. To produce PCLK clock, it sets the division factor of HCLK clock as follows: 0xx: 1
    #[inline(always)]
    pub fn ppre(&mut self) -> PPRE_W<CFGRrs> {
        PPRE_W::new(self, 12)
    }
    ///Bits 16:19 - Microcontroller clock output 2 clock selector This bitfield is controlled by software. It sets the clock selector for MCO2 output as follows: This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Note: This clock output may have some truncated cycles at startup or during MCO2 clock source switching.
    #[inline(always)]
    pub fn mco2sel(&mut self) -> MCO2SEL_W<CFGRrs> {
        MCO2SEL_W::new(self, 16)
    }
    ///Bits 20:23 - Microcontroller clock output 2 prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO2 output as follows: ... It is highly recommended to set this field before the MCO2 output is enabled.
    #[inline(always)]
    pub fn mco2pre(&mut self) -> MCO2PRE_W<CFGRrs> {
        MCO2PRE_W::new(self, 20)
    }
    ///Bits 24:27 - Microcontroller clock output clock selector This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Note: This clock output may have some truncated cycles at startup or during MCO clock source switching. Any other value means no clock on MCO.
    #[inline(always)]
    pub fn mcosel(&mut self) -> MCOSEL_W<CFGRrs> {
        MCOSEL_W::new(self, 24)
    }
    ///Bits 28:31 - Microcontroller clock output prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO output as follows: ... It is highly recommended to set this field before the MCO output is enabled.
    #[inline(always)]
    pub fn mcopre(&mut self) -> MCOPRE_W<CFGRrs> {
        MCOPRE_W::new(self, 28)
    }
}
/**RCC clock configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0;
}
