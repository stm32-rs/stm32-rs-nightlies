///Register `ITLINE11` reader
pub type R = crate::R<ITLINE11rs>;
///Field `DMAMUX` reader - DMAMUX interrupt request pending
pub type DMAMUX_R = crate::BitReader;
impl R {
    ///Bit 0 - DMAMUX interrupt request pending
    #[inline(always)]
    pub fn dmamux(&self) -> DMAMUX_R {
        DMAMUX_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE11")
            .field("dmamux", &self.dmamux())
            .finish()
    }
}
/**SYSCFG interrupt line 11 status register

You can [`read`](crate::Reg::read) this register and get [`itline11::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#SYSCFG:ITLINE11)*/
pub struct ITLINE11rs;
impl crate::RegisterSpec for ITLINE11rs {
    type Ux = u32;
}
///`read()` method returns [`itline11::R`](R) reader structure
impl crate::Readable for ITLINE11rs {}
///`reset()` method sets ITLINE11 to value 0
impl crate::Resettable for ITLINE11rs {
    const RESET_VALUE: u32 = 0;
}
