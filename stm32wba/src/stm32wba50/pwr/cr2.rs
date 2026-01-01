///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `SRAM1PDS1` reader - SRAM1 power-down in Stop modes (Stop 0, 1) Note: The SRAM1 retention in Standby mode is controlled by R1RSB1 bit in PWR_CR1.
pub type SRAM1PDS1_R = crate::BitReader;
///Field `SRAM1PDS1` writer - SRAM1 power-down in Stop modes (Stop 0, 1) Note: The SRAM1 retention in Standby mode is controlled by R1RSB1 bit in PWR_CR1.
pub type SRAM1PDS1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2PDS1` reader - SRAM2 power-down in Stop modes (Stop 0, 1) Note: The SRAM2 retention in Standby mode is controlled by R2RSB1 bit in PWR_CR1.
pub type SRAM2PDS1_R = crate::BitReader;
///Field `SRAM2PDS1` writer - SRAM2 power-down in Stop modes (Stop 0, 1) Note: The SRAM2 retention in Standby mode is controlled by R2RSB1 bit in PWR_CR1.
pub type SRAM2PDS1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICRAMPDS` reader - ICACHE SRAM power-down in Stop modes (Stop 0, 1)
pub type ICRAMPDS_R = crate::BitReader;
///Field `ICRAMPDS` writer - ICACHE SRAM power-down in Stop modes (Stop 0, 1)
pub type ICRAMPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHFWU` reader - Flash memory fast wakeup from Stop modes (Stop 0, 1) This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption.
pub type FLASHFWU_R = crate::BitReader;
///Field `FLASHFWU` writer - Flash memory fast wakeup from Stop modes (Stop 0, 1) This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption.
pub type FLASHFWU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SRAM1 power-down in Stop modes (Stop 0, 1) Note: The SRAM1 retention in Standby mode is controlled by R1RSB1 bit in PWR_CR1.
    #[inline(always)]
    pub fn sram1pds1(&self) -> SRAM1PDS1_R {
        SRAM1PDS1_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - SRAM2 power-down in Stop modes (Stop 0, 1) Note: The SRAM2 retention in Standby mode is controlled by R2RSB1 bit in PWR_CR1.
    #[inline(always)]
    pub fn sram2pds1(&self) -> SRAM2PDS1_R {
        SRAM2PDS1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - ICACHE SRAM power-down in Stop modes (Stop 0, 1)
    #[inline(always)]
    pub fn icrampds(&self) -> ICRAMPDS_R {
        ICRAMPDS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 14 - Flash memory fast wakeup from Stop modes (Stop 0, 1) This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption.
    #[inline(always)]
    pub fn flashfwu(&self) -> FLASHFWU_R {
        FLASHFWU_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("sram1pds1", &self.sram1pds1())
            .field("sram2pds1", &self.sram2pds1())
            .field("icrampds", &self.icrampds())
            .field("flashfwu", &self.flashfwu())
            .finish()
    }
}
impl W {
    ///Bit 0 - SRAM1 power-down in Stop modes (Stop 0, 1) Note: The SRAM1 retention in Standby mode is controlled by R1RSB1 bit in PWR_CR1.
    #[inline(always)]
    pub fn sram1pds1(&mut self) -> SRAM1PDS1_W<'_, CR2rs> {
        SRAM1PDS1_W::new(self, 0)
    }
    ///Bit 4 - SRAM2 power-down in Stop modes (Stop 0, 1) Note: The SRAM2 retention in Standby mode is controlled by R2RSB1 bit in PWR_CR1.
    #[inline(always)]
    pub fn sram2pds1(&mut self) -> SRAM2PDS1_W<'_, CR2rs> {
        SRAM2PDS1_W::new(self, 4)
    }
    ///Bit 8 - ICACHE SRAM power-down in Stop modes (Stop 0, 1)
    #[inline(always)]
    pub fn icrampds(&mut self) -> ICRAMPDS_W<'_, CR2rs> {
        ICRAMPDS_W::new(self, 8)
    }
    ///Bit 14 - Flash memory fast wakeup from Stop modes (Stop 0, 1) This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption.
    #[inline(always)]
    pub fn flashfwu(&mut self) -> FLASHFWU_W<'_, CR2rs> {
        FLASHFWU_W::new(self, 14)
    }
}
/**PWR control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#PWR:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
