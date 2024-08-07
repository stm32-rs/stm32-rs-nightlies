///Register `GICD_PIDR5` reader
pub type R = crate::R<GICD_PIDR5rs>;
///Field `PIDR5` reader - PIDR5
pub type PIDR5_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - PIDR5
    #[inline(always)]
    pub fn pidr5(&self) -> PIDR5_R {
        PIDR5_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_PIDR5")
            .field("pidr5", &self.pidr5())
            .finish()
    }
}
/**GICD peripheral ID5 to ID7 register 5

You can [`read`](crate::Reg::read) this register and get [`gicd_pidr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:GICD_PIDR5)*/
pub struct GICD_PIDR5rs;
impl crate::RegisterSpec for GICD_PIDR5rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_pidr5::R`](R) reader structure
impl crate::Readable for GICD_PIDR5rs {}
///`reset()` method sets GICD_PIDR5 to value 0
impl crate::Resettable for GICD_PIDR5rs {
    const RESET_VALUE: u32 = 0;
}
