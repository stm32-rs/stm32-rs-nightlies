///Register `PWR_CR1` reader
pub type R = crate::R<PWR_CR1rs>;
///Register `PWR_CR1` writer
pub type W = crate::W<PWR_CR1rs>;
///Field `LPMS` reader - Low-power mode selection These bits select the low-power mode entered when the CPU enters the Deepsleep mode. 10x: Standby mode (Standby mode also entered if LPMS = 11X in PWR_CR1 with BREN = 1 in PWR_BDCR1) 11x: Shutdown mode if BREN = 0 in PWR_BDCR1
pub type LPMS_R = crate::FieldReader;
///Field `LPMS` writer - Low-power mode selection These bits select the low-power mode entered when the CPU enters the Deepsleep mode. 10x: Standby mode (Standby mode also entered if LPMS = 11X in PWR_CR1 with BREN = 1 in PWR_BDCR1) 11x: Shutdown mode if BREN = 0 in PWR_BDCR1
pub type LPMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RRSB1` reader - SRAM2 page 1 retention in Stop 3 and Standby modes This bit is used to keep the SRAM2 page 1 content in Stop 3 and Standby modes. The SRAM2 page 1 corresponds to the first 8 Kbytes of the SRAM2 (from SRAM2 base address to SRAM2 base address + 0x1FFF). Note: This bit has no effect in Shutdown mode.
pub type RRSB1_R = crate::BitReader;
///Field `RRSB1` writer - SRAM2 page 1 retention in Stop 3 and Standby modes This bit is used to keep the SRAM2 page 1 content in Stop 3 and Standby modes. The SRAM2 page 1 corresponds to the first 8 Kbytes of the SRAM2 (from SRAM2 base address to SRAM2 base address + 0x1FFF). Note: This bit has no effect in Shutdown mode.
pub type RRSB1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RRSB2` reader - SRAM2 page 2 retention in Stop 3 and Standby modes This bit is used to keep the SRAM2 page 2 content in Stop 3 and Standby modes. The SRAM2 page 2 corresponds to the last 56 Kbytes of the SRAM2 (from SRAM2 base address + 0x2000 to SRAM2 base address + 0xFFFF). Note: This bit has no effect in Shutdown mode.
pub type RRSB2_R = crate::BitReader;
///Field `RRSB2` writer - SRAM2 page 2 retention in Stop 3 and Standby modes This bit is used to keep the SRAM2 page 2 content in Stop 3 and Standby modes. The SRAM2 page 2 corresponds to the last 56 Kbytes of the SRAM2 (from SRAM2 base address + 0x2000 to SRAM2 base address + 0xFFFF). Note: This bit has no effect in Shutdown mode.
pub type RRSB2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ULPMEN` reader - BOR ultra-low power mode This bit is used to reduce the consumption by configuring the BOR in discontinuous mode. This bit must be set to reach the lowest power consumption in the low-power modes.
pub type ULPMEN_R = crate::BitReader;
///Field `ULPMEN` writer - BOR ultra-low power mode This bit is used to reduce the consumption by configuring the BOR in discontinuous mode. This bit must be set to reach the lowest power consumption in the low-power modes.
pub type ULPMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM1PD` reader - SRAM1 power down This bit is used to reduce the consumption by powering off the SRAM1.
pub type SRAM1PD_R = crate::BitReader;
///Field `SRAM1PD` writer - SRAM1 power down This bit is used to reduce the consumption by powering off the SRAM1.
pub type SRAM1PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2PD` reader - SRAM2 power down This bit is used to reduce the consumption by powering off the SRAM2.
pub type SRAM2PD_R = crate::BitReader;
///Field `SRAM2PD` writer - SRAM2 power down This bit is used to reduce the consumption by powering off the SRAM2.
pub type SRAM2PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM3PD` reader - SRAM3 power down This bit is used to reduce the consumption by powering off the SRAM3.
pub type SRAM3PD_R = crate::BitReader;
///Field `SRAM3PD` writer - SRAM3 power down This bit is used to reduce the consumption by powering off the SRAM3.
pub type SRAM3PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM4PD` reader - SRAM4 power down This bit is used to reduce the consumption by powering off the SRAM4.
pub type SRAM4PD_R = crate::BitReader;
///Field `SRAM4PD` writer - SRAM4 power down This bit is used to reduce the consumption by powering off the SRAM4.
pub type SRAM4PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Low-power mode selection These bits select the low-power mode entered when the CPU enters the Deepsleep mode. 10x: Standby mode (Standby mode also entered if LPMS = 11X in PWR_CR1 with BREN = 1 in PWR_BDCR1) 11x: Shutdown mode if BREN = 0 in PWR_BDCR1
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 7) as u8)
    }
    ///Bit 5 - SRAM2 page 1 retention in Stop 3 and Standby modes This bit is used to keep the SRAM2 page 1 content in Stop 3 and Standby modes. The SRAM2 page 1 corresponds to the first 8 Kbytes of the SRAM2 (from SRAM2 base address to SRAM2 base address + 0x1FFF). Note: This bit has no effect in Shutdown mode.
    #[inline(always)]
    pub fn rrsb1(&self) -> RRSB1_R {
        RRSB1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SRAM2 page 2 retention in Stop 3 and Standby modes This bit is used to keep the SRAM2 page 2 content in Stop 3 and Standby modes. The SRAM2 page 2 corresponds to the last 56 Kbytes of the SRAM2 (from SRAM2 base address + 0x2000 to SRAM2 base address + 0xFFFF). Note: This bit has no effect in Shutdown mode.
    #[inline(always)]
    pub fn rrsb2(&self) -> RRSB2_R {
        RRSB2_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - BOR ultra-low power mode This bit is used to reduce the consumption by configuring the BOR in discontinuous mode. This bit must be set to reach the lowest power consumption in the low-power modes.
    #[inline(always)]
    pub fn ulpmen(&self) -> ULPMEN_R {
        ULPMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SRAM1 power down This bit is used to reduce the consumption by powering off the SRAM1.
    #[inline(always)]
    pub fn sram1pd(&self) -> SRAM1PD_R {
        SRAM1PD_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SRAM2 power down This bit is used to reduce the consumption by powering off the SRAM2.
    #[inline(always)]
    pub fn sram2pd(&self) -> SRAM2PD_R {
        SRAM2PD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SRAM3 power down This bit is used to reduce the consumption by powering off the SRAM3.
    #[inline(always)]
    pub fn sram3pd(&self) -> SRAM3PD_R {
        SRAM3PD_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SRAM4 power down This bit is used to reduce the consumption by powering off the SRAM4.
    #[inline(always)]
    pub fn sram4pd(&self) -> SRAM4PD_R {
        SRAM4PD_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_CR1")
            .field("lpms", &self.lpms())
            .field("rrsb1", &self.rrsb1())
            .field("rrsb2", &self.rrsb2())
            .field("ulpmen", &self.ulpmen())
            .field("sram1pd", &self.sram1pd())
            .field("sram2pd", &self.sram2pd())
            .field("sram3pd", &self.sram3pd())
            .field("sram4pd", &self.sram4pd())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Low-power mode selection These bits select the low-power mode entered when the CPU enters the Deepsleep mode. 10x: Standby mode (Standby mode also entered if LPMS = 11X in PWR_CR1 with BREN = 1 in PWR_BDCR1) 11x: Shutdown mode if BREN = 0 in PWR_BDCR1
    #[inline(always)]
    #[must_use]
    pub fn lpms(&mut self) -> LPMS_W<PWR_CR1rs> {
        LPMS_W::new(self, 0)
    }
    ///Bit 5 - SRAM2 page 1 retention in Stop 3 and Standby modes This bit is used to keep the SRAM2 page 1 content in Stop 3 and Standby modes. The SRAM2 page 1 corresponds to the first 8 Kbytes of the SRAM2 (from SRAM2 base address to SRAM2 base address + 0x1FFF). Note: This bit has no effect in Shutdown mode.
    #[inline(always)]
    #[must_use]
    pub fn rrsb1(&mut self) -> RRSB1_W<PWR_CR1rs> {
        RRSB1_W::new(self, 5)
    }
    ///Bit 6 - SRAM2 page 2 retention in Stop 3 and Standby modes This bit is used to keep the SRAM2 page 2 content in Stop 3 and Standby modes. The SRAM2 page 2 corresponds to the last 56 Kbytes of the SRAM2 (from SRAM2 base address + 0x2000 to SRAM2 base address + 0xFFFF). Note: This bit has no effect in Shutdown mode.
    #[inline(always)]
    #[must_use]
    pub fn rrsb2(&mut self) -> RRSB2_W<PWR_CR1rs> {
        RRSB2_W::new(self, 6)
    }
    ///Bit 7 - BOR ultra-low power mode This bit is used to reduce the consumption by configuring the BOR in discontinuous mode. This bit must be set to reach the lowest power consumption in the low-power modes.
    #[inline(always)]
    #[must_use]
    pub fn ulpmen(&mut self) -> ULPMEN_W<PWR_CR1rs> {
        ULPMEN_W::new(self, 7)
    }
    ///Bit 8 - SRAM1 power down This bit is used to reduce the consumption by powering off the SRAM1.
    #[inline(always)]
    #[must_use]
    pub fn sram1pd(&mut self) -> SRAM1PD_W<PWR_CR1rs> {
        SRAM1PD_W::new(self, 8)
    }
    ///Bit 9 - SRAM2 power down This bit is used to reduce the consumption by powering off the SRAM2.
    #[inline(always)]
    #[must_use]
    pub fn sram2pd(&mut self) -> SRAM2PD_W<PWR_CR1rs> {
        SRAM2PD_W::new(self, 9)
    }
    ///Bit 10 - SRAM3 power down This bit is used to reduce the consumption by powering off the SRAM3.
    #[inline(always)]
    #[must_use]
    pub fn sram3pd(&mut self) -> SRAM3PD_W<PWR_CR1rs> {
        SRAM3PD_W::new(self, 10)
    }
    ///Bit 11 - SRAM4 power down This bit is used to reduce the consumption by powering off the SRAM4.
    #[inline(always)]
    #[must_use]
    pub fn sram4pd(&mut self) -> SRAM4PD_W<PWR_CR1rs> {
        SRAM4PD_W::new(self, 11)
    }
}
/**PWR control register 1

You can [`read`](crate::Reg::read) this register and get [`pwr_cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#PWR:PWR_CR1)*/
pub struct PWR_CR1rs;
impl crate::RegisterSpec for PWR_CR1rs {
    type Ux = u32;
}
///`read()` method returns [`pwr_cr1::R`](R) reader structure
impl crate::Readable for PWR_CR1rs {}
///`write(|w| ..)` method takes [`pwr_cr1::W`](W) writer structure
impl crate::Writable for PWR_CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PWR_CR1 to value 0
impl crate::Resettable for PWR_CR1rs {
    const RESET_VALUE: u32 = 0;
}
