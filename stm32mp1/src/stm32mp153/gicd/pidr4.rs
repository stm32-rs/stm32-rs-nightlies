///Register `PIDR4` reader
pub type R = crate::R<PIDR4rs>;
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
        f.debug_struct("PIDR4")
            .field("pidr4", &self.pidr4())
            .finish()
    }
}
/**GICD peripheral ID4 register

You can [`read`](crate::Reg::read) this register and get [`pidr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:PIDR4)*/
pub struct PIDR4rs;
impl crate::RegisterSpec for PIDR4rs {
    type Ux = u32;
}
///`read()` method returns [`pidr4::R`](R) reader structure
impl crate::Readable for PIDR4rs {}
///`reset()` method sets PIDR4 to value 0x04
impl crate::Resettable for PIDR4rs {
    const RESET_VALUE: u32 = 0x04;
}
