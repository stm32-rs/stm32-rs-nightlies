///Register `ITLINE25` reader
pub type R = crate::R<ITLINE25rs>;
///Field `SPI1` reader - SPI1 interrupt request pending
pub type SPI1_R = crate::BitReader;
impl R {
    ///Bit 0 - SPI1 interrupt request pending
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE25")
            .field("spi1", &self.spi1())
            .finish()
    }
}
/**SYSCFG interrupt line 25 status register

You can [`read`](crate::Reg::read) this register and get [`itline25::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE25)*/
pub struct ITLINE25rs;
impl crate::RegisterSpec for ITLINE25rs {
    type Ux = u32;
}
///`read()` method returns [`itline25::R`](R) reader structure
impl crate::Readable for ITLINE25rs {}
///`reset()` method sets ITLINE25 to value 0
impl crate::Resettable for ITLINE25rs {
    const RESET_VALUE: u32 = 0;
}
