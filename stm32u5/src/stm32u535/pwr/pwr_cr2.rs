///Register `PWR_CR2` reader
pub type R = crate::R<PWR_CR2rs>;
///Register `PWR_CR2` writer
pub type W = crate::W<PWR_CR2rs>;
///Field `SRAM1PDS1` reader - SRAM1 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM1PDS1_R = crate::BitReader;
///Field `SRAM1PDS1` writer - SRAM1 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM1PDS1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM1PDS2` reader - SRAM1 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM1PDS2_R = crate::BitReader;
///Field `SRAM1PDS2` writer - SRAM1 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM1PDS2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM1PDS3` reader - SRAM1 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM1PDS3_R = crate::BitReader;
///Field `SRAM1PDS3` writer - SRAM1 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM1PDS3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2PDS1` reader - SRAM2 page 1 (8 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 1 retention in Stop 3 is controlled by RRSB1 bit in PWR_CR1.
pub type SRAM2PDS1_R = crate::BitReader;
///Field `SRAM2PDS1` writer - SRAM2 page 1 (8 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 1 retention in Stop 3 is controlled by RRSB1 bit in PWR_CR1.
pub type SRAM2PDS1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2PDS2` reader - SRAM2 page 2 (56 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 2 retention in Stop 3 is controlled by RRSB2 bit in PWR_CR1.
pub type SRAM2PDS2_R = crate::BitReader;
///Field `SRAM2PDS2` writer - SRAM2 page 2 (56 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 2 retention in Stop 3 is controlled by RRSB2 bit in PWR_CR1.
pub type SRAM2PDS2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM4PDS` reader - SRAM4 power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM4PDS_R = crate::BitReader;
///Field `SRAM4PDS` writer - SRAM4 power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM4PDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICRAMPDS` reader - ICACHE SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
pub type ICRAMPDS_R = crate::BitReader;
///Field `ICRAMPDS` writer - ICACHE SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
pub type ICRAMPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DC1RAMPDS` reader - DCACHE1 SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
pub type DC1RAMPDS_R = crate::BitReader;
///Field `DC1RAMPDS` writer - DCACHE1 SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
pub type DC1RAMPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRAMPDS` reader - FMAC, FDCAN and USB peripherals SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
pub type PRAMPDS_R = crate::BitReader;
///Field `PRAMPDS` writer - FMAC, FDCAN and USB peripherals SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
pub type PRAMPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM4FWU` reader - SRAM4 fast wakeup from Stop 0, Stop 1 and Stop 2 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time. SRAM4 wakeup time increases the wakeup time when exiting Stop 0, 1 and 2 modes, and also increases the LPDMA access time to SRAM4 during Stop modes.
pub type SRAM4FWU_R = crate::BitReader;
///Field `SRAM4FWU` writer - SRAM4 fast wakeup from Stop 0, Stop 1 and Stop 2 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time. SRAM4 wakeup time increases the wakeup time when exiting Stop 0, 1 and 2 modes, and also increases the LPDMA access time to SRAM4 during Stop modes.
pub type SRAM4FWU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHFWU` reader - Flash memory fast wakeup from Stop 0 and Stop 1 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption.
pub type FLASHFWU_R = crate::BitReader;
///Field `FLASHFWU` writer - Flash memory fast wakeup from Stop 0 and Stop 1 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption.
pub type FLASHFWU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRDRUN` reader - SmartRun domain in Run mode
pub type SRDRUN_R = crate::BitReader;
///Field `SRDRUN` writer - SmartRun domain in Run mode
pub type SRDRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SRAM1 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn sram1pds1(&self) -> SRAM1PDS1_R {
        SRAM1PDS1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SRAM1 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn sram1pds2(&self) -> SRAM1PDS2_R {
        SRAM1PDS2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SRAM1 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn sram1pds3(&self) -> SRAM1PDS3_R {
        SRAM1PDS3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - SRAM2 page 1 (8 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 1 retention in Stop 3 is controlled by RRSB1 bit in PWR_CR1.
    #[inline(always)]
    pub fn sram2pds1(&self) -> SRAM2PDS1_R {
        SRAM2PDS1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SRAM2 page 2 (56 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 2 retention in Stop 3 is controlled by RRSB2 bit in PWR_CR1.
    #[inline(always)]
    pub fn sram2pds2(&self) -> SRAM2PDS2_R {
        SRAM2PDS2_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SRAM4 power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn sram4pds(&self) -> SRAM4PDS_R {
        SRAM4PDS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - ICACHE SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn icrampds(&self) -> ICRAMPDS_R {
        ICRAMPDS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DCACHE1 SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn dc1rampds(&self) -> DC1RAMPDS_R {
        DC1RAMPDS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - FMAC, FDCAN and USB peripherals SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn prampds(&self) -> PRAMPDS_R {
        PRAMPDS_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - SRAM4 fast wakeup from Stop 0, Stop 1 and Stop 2 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time. SRAM4 wakeup time increases the wakeup time when exiting Stop 0, 1 and 2 modes, and also increases the LPDMA access time to SRAM4 during Stop modes.
    #[inline(always)]
    pub fn sram4fwu(&self) -> SRAM4FWU_R {
        SRAM4FWU_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Flash memory fast wakeup from Stop 0 and Stop 1 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption.
    #[inline(always)]
    pub fn flashfwu(&self) -> FLASHFWU_R {
        FLASHFWU_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 31 - SmartRun domain in Run mode
    #[inline(always)]
    pub fn srdrun(&self) -> SRDRUN_R {
        SRDRUN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_CR2")
            .field("sram1pds1", &self.sram1pds1())
            .field("sram1pds2", &self.sram1pds2())
            .field("sram1pds3", &self.sram1pds3())
            .field("sram2pds1", &self.sram2pds1())
            .field("sram2pds2", &self.sram2pds2())
            .field("sram4pds", &self.sram4pds())
            .field("icrampds", &self.icrampds())
            .field("dc1rampds", &self.dc1rampds())
            .field("prampds", &self.prampds())
            .field("sram4fwu", &self.sram4fwu())
            .field("flashfwu", &self.flashfwu())
            .field("srdrun", &self.srdrun())
            .finish()
    }
}
impl W {
    ///Bit 0 - SRAM1 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    #[must_use]
    pub fn sram1pds1(&mut self) -> SRAM1PDS1_W<PWR_CR2rs> {
        SRAM1PDS1_W::new(self, 0)
    }
    ///Bit 1 - SRAM1 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    #[must_use]
    pub fn sram1pds2(&mut self) -> SRAM1PDS2_W<PWR_CR2rs> {
        SRAM1PDS2_W::new(self, 1)
    }
    ///Bit 2 - SRAM1 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    #[must_use]
    pub fn sram1pds3(&mut self) -> SRAM1PDS3_W<PWR_CR2rs> {
        SRAM1PDS3_W::new(self, 2)
    }
    ///Bit 4 - SRAM2 page 1 (8 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 1 retention in Stop 3 is controlled by RRSB1 bit in PWR_CR1.
    #[inline(always)]
    #[must_use]
    pub fn sram2pds1(&mut self) -> SRAM2PDS1_W<PWR_CR2rs> {
        SRAM2PDS1_W::new(self, 4)
    }
    ///Bit 5 - SRAM2 page 2 (56 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 2 retention in Stop 3 is controlled by RRSB2 bit in PWR_CR1.
    #[inline(always)]
    #[must_use]
    pub fn sram2pds2(&mut self) -> SRAM2PDS2_W<PWR_CR2rs> {
        SRAM2PDS2_W::new(self, 5)
    }
    ///Bit 6 - SRAM4 power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    #[must_use]
    pub fn sram4pds(&mut self) -> SRAM4PDS_W<PWR_CR2rs> {
        SRAM4PDS_W::new(self, 6)
    }
    ///Bit 8 - ICACHE SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    #[must_use]
    pub fn icrampds(&mut self) -> ICRAMPDS_W<PWR_CR2rs> {
        ICRAMPDS_W::new(self, 8)
    }
    ///Bit 9 - DCACHE1 SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    #[must_use]
    pub fn dc1rampds(&mut self) -> DC1RAMPDS_W<PWR_CR2rs> {
        DC1RAMPDS_W::new(self, 9)
    }
    ///Bit 11 - FMAC, FDCAN and USB peripherals SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    #[must_use]
    pub fn prampds(&mut self) -> PRAMPDS_W<PWR_CR2rs> {
        PRAMPDS_W::new(self, 11)
    }
    ///Bit 13 - SRAM4 fast wakeup from Stop 0, Stop 1 and Stop 2 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time. SRAM4 wakeup time increases the wakeup time when exiting Stop 0, 1 and 2 modes, and also increases the LPDMA access time to SRAM4 during Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn sram4fwu(&mut self) -> SRAM4FWU_W<PWR_CR2rs> {
        SRAM4FWU_W::new(self, 13)
    }
    ///Bit 14 - Flash memory fast wakeup from Stop 0 and Stop 1 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption.
    #[inline(always)]
    #[must_use]
    pub fn flashfwu(&mut self) -> FLASHFWU_W<PWR_CR2rs> {
        FLASHFWU_W::new(self, 14)
    }
    ///Bit 31 - SmartRun domain in Run mode
    #[inline(always)]
    #[must_use]
    pub fn srdrun(&mut self) -> SRDRUN_W<PWR_CR2rs> {
        SRDRUN_W::new(self, 31)
    }
}
/**PWR control register 2

You can [`read`](crate::Reg::read) this register and get [`pwr_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:PWR_CR2)*/
pub struct PWR_CR2rs;
impl crate::RegisterSpec for PWR_CR2rs {
    type Ux = u32;
}
///`read()` method returns [`pwr_cr2::R`](R) reader structure
impl crate::Readable for PWR_CR2rs {}
///`write(|w| ..)` method takes [`pwr_cr2::W`](W) writer structure
impl crate::Writable for PWR_CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PWR_CR2 to value 0
impl crate::Resettable for PWR_CR2rs {
    const RESET_VALUE: u32 = 0;
}
