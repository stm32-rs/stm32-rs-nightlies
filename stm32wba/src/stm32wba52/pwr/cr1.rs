///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `LPMS` reader - Low-power mode selection These bits select the low-power mode entered when the CPU enters the SleepDeep mode. 10x: Standby mode others reserved
pub type LPMS_R = crate::FieldReader;
///Field `LPMS` writer - Low-power mode selection These bits select the low-power mode entered when the CPU enters the SleepDeep mode. 10x: Standby mode others reserved
pub type LPMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `R2RSB1` reader - SRAM2 retention in Standby mode This bit is used to keep the SRAM2 content in Standby retention mode.
pub type R2RSB1_R = crate::BitReader;
///Field `R2RSB1` writer - SRAM2 retention in Standby mode This bit is used to keep the SRAM2 content in Standby retention mode.
pub type R2RSB1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ULPMEN` reader - BOR0 ultra-low-power mode. This bit is used to reduce the consumption by configuring the BOR0 in discontinuous mode for Stop 1 and Standby modes. Discontinuous mode is only available when BOR levels 1 to 4 and PVD are disabled. Note: This bit must be set to reach the lowest power consumption in the low-power modes. Note: This bit must not be set together with autonomous peripherals using HSI16 as kernel clock. Note: When BOR level 1 to 4 or PVD is enabled continuous mode applies independent from ULPMEN.
pub type ULPMEN_R = crate::BitReader;
///Field `ULPMEN` writer - BOR0 ultra-low-power mode. This bit is used to reduce the consumption by configuring the BOR0 in discontinuous mode for Stop 1 and Standby modes. Discontinuous mode is only available when BOR levels 1 to 4 and PVD are disabled. Note: This bit must be set to reach the lowest power consumption in the low-power modes. Note: This bit must not be set together with autonomous peripherals using HSI16 as kernel clock. Note: When BOR level 1 to 4 or PVD is enabled continuous mode applies independent from ULPMEN.
pub type ULPMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RADIORSB` reader - 2.4 GHz RADIO SRAMs (RXTXRAM and Sequence RAM) and Sleep clock retention in Standby mode. This bit is used to keep the 2.4 GHz RADIO SRAMs content in Standby retention mode and the 2.4 GHz RADIO sleep timer counter operational.
pub type RADIORSB_R = crate::BitReader;
///Field `RADIORSB` writer - 2.4 GHz RADIO SRAMs (RXTXRAM and Sequence RAM) and Sleep clock retention in Standby mode. This bit is used to keep the 2.4 GHz RADIO SRAMs content in Standby retention mode and the 2.4 GHz RADIO sleep timer counter operational.
pub type RADIORSB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `R1RSB1` reader - SRAM1 retention in Standby mode This bit is used to keep the SRAM1 content in Standby retention mode.
pub type R1RSB1_R = crate::BitReader;
///Field `R1RSB1` writer - SRAM1 retention in Standby mode This bit is used to keep the SRAM1 content in Standby retention mode.
pub type R1RSB1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Low-power mode selection These bits select the low-power mode entered when the CPU enters the SleepDeep mode. 10x: Standby mode others reserved
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 7) as u8)
    }
    ///Bit 5 - SRAM2 retention in Standby mode This bit is used to keep the SRAM2 content in Standby retention mode.
    #[inline(always)]
    pub fn r2rsb1(&self) -> R2RSB1_R {
        R2RSB1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - BOR0 ultra-low-power mode. This bit is used to reduce the consumption by configuring the BOR0 in discontinuous mode for Stop 1 and Standby modes. Discontinuous mode is only available when BOR levels 1 to 4 and PVD are disabled. Note: This bit must be set to reach the lowest power consumption in the low-power modes. Note: This bit must not be set together with autonomous peripherals using HSI16 as kernel clock. Note: When BOR level 1 to 4 or PVD is enabled continuous mode applies independent from ULPMEN.
    #[inline(always)]
    pub fn ulpmen(&self) -> ULPMEN_R {
        ULPMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - 2.4 GHz RADIO SRAMs (RXTXRAM and Sequence RAM) and Sleep clock retention in Standby mode. This bit is used to keep the 2.4 GHz RADIO SRAMs content in Standby retention mode and the 2.4 GHz RADIO sleep timer counter operational.
    #[inline(always)]
    pub fn radiorsb(&self) -> RADIORSB_R {
        RADIORSB_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - SRAM1 retention in Standby mode This bit is used to keep the SRAM1 content in Standby retention mode.
    #[inline(always)]
    pub fn r1rsb1(&self) -> R1RSB1_R {
        R1RSB1_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("lpms", &self.lpms())
            .field("r2rsb1", &self.r2rsb1())
            .field("ulpmen", &self.ulpmen())
            .field("radiorsb", &self.radiorsb())
            .field("r1rsb1", &self.r1rsb1())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Low-power mode selection These bits select the low-power mode entered when the CPU enters the SleepDeep mode. 10x: Standby mode others reserved
    #[inline(always)]
    pub fn lpms(&mut self) -> LPMS_W<'_, CR1rs> {
        LPMS_W::new(self, 0)
    }
    ///Bit 5 - SRAM2 retention in Standby mode This bit is used to keep the SRAM2 content in Standby retention mode.
    #[inline(always)]
    pub fn r2rsb1(&mut self) -> R2RSB1_W<'_, CR1rs> {
        R2RSB1_W::new(self, 5)
    }
    ///Bit 7 - BOR0 ultra-low-power mode. This bit is used to reduce the consumption by configuring the BOR0 in discontinuous mode for Stop 1 and Standby modes. Discontinuous mode is only available when BOR levels 1 to 4 and PVD are disabled. Note: This bit must be set to reach the lowest power consumption in the low-power modes. Note: This bit must not be set together with autonomous peripherals using HSI16 as kernel clock. Note: When BOR level 1 to 4 or PVD is enabled continuous mode applies independent from ULPMEN.
    #[inline(always)]
    pub fn ulpmen(&mut self) -> ULPMEN_W<'_, CR1rs> {
        ULPMEN_W::new(self, 7)
    }
    ///Bit 9 - 2.4 GHz RADIO SRAMs (RXTXRAM and Sequence RAM) and Sleep clock retention in Standby mode. This bit is used to keep the 2.4 GHz RADIO SRAMs content in Standby retention mode and the 2.4 GHz RADIO sleep timer counter operational.
    #[inline(always)]
    pub fn radiorsb(&mut self) -> RADIORSB_W<'_, CR1rs> {
        RADIORSB_W::new(self, 9)
    }
    ///Bit 12 - SRAM1 retention in Standby mode This bit is used to keep the SRAM1 content in Standby retention mode.
    #[inline(always)]
    pub fn r1rsb1(&mut self) -> R1RSB1_W<'_, CR1rs> {
        R1RSB1_W::new(self, 12)
    }
}
/**PWR control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#PWR:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {}
