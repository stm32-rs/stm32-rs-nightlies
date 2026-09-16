///Register `SYSCFG_ITLINE26` reader
pub type R = crate::R<SYSCFG_ITLINE26rs>;
///Field `SPI2` reader - SPI2 interrupt request pending
pub type SPI2_R = crate::BitReader;
impl R {
    ///Bit 0 - SPI2 interrupt request pending
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_ITLINE26")
            .field("spi2", &self.spi2())
            .finish()
    }
}
/**SYSCFG interrupt line 26 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline26::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE26)*/
pub struct SYSCFG_ITLINE26rs;
impl crate::RegisterSpec for SYSCFG_ITLINE26rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_itline26::R`](R) reader structure
impl crate::Readable for SYSCFG_ITLINE26rs {}
///`reset()` method sets SYSCFG_ITLINE26 to value 0
impl crate::Resettable for SYSCFG_ITLINE26rs {}
