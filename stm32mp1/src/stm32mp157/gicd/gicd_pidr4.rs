///Register `GICD_PIDR4` reader
pub type R = crate::R<GICD_PIDR4rs>;
///Field `PIDR4` reader - PIDR4
pub type PIDR4_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - PIDR4
    #[inline(always)]
    pub fn pidr4(&self) -> PIDR4_R {
        PIDR4_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_PIDR4")
            .field("pidr4", &self.pidr4())
            .finish()
    }
}
/**GICD peripheral ID4 register

You can [`read`](crate::Reg::read) this register and get [`gicd_pidr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:GICD_PIDR4)*/
pub struct GICD_PIDR4rs;
impl crate::RegisterSpec for GICD_PIDR4rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_pidr4::R`](R) reader structure
impl crate::Readable for GICD_PIDR4rs {}
///`reset()` method sets GICD_PIDR4 to value 0x04
impl crate::Resettable for GICD_PIDR4rs {
    const RESET_VALUE: u32 = 0x04;
}
