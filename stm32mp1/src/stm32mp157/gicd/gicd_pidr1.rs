///Register `GICD_PIDR1` reader
pub type R = crate::R<GICD_PIDR1rs>;
///Field `PIDR1` reader - PIDR1
pub type PIDR1_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - PIDR1
    #[inline(always)]
    pub fn pidr1(&self) -> PIDR1_R {
        PIDR1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_PIDR1")
            .field("pidr1", &self.pidr1())
            .finish()
    }
}
/**GICD peripheral ID1 register

You can [`read`](crate::Reg::read) this register and get [`gicd_pidr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:GICD_PIDR1)*/
pub struct GICD_PIDR1rs;
impl crate::RegisterSpec for GICD_PIDR1rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_pidr1::R`](R) reader structure
impl crate::Readable for GICD_PIDR1rs {}
///`reset()` method sets GICD_PIDR1 to value 0xb4
impl crate::Resettable for GICD_PIDR1rs {
    const RESET_VALUE: u32 = 0xb4;
}
