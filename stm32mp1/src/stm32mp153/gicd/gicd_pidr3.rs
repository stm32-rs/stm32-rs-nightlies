///Register `GICD_PIDR3` reader
pub type R = crate::R<GICD_PIDR3rs>;
///Field `PIDR3` reader - PIDR3
pub type PIDR3_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - PIDR3
    #[inline(always)]
    pub fn pidr3(&self) -> PIDR3_R {
        PIDR3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_PIDR3")
            .field("pidr3", &self.pidr3())
            .finish()
    }
}
/**GICD peripheral ID3 register

You can [`read`](crate::Reg::read) this register and get [`gicd_pidr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:GICD_PIDR3)*/
pub struct GICD_PIDR3rs;
impl crate::RegisterSpec for GICD_PIDR3rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_pidr3::R`](R) reader structure
impl crate::Readable for GICD_PIDR3rs {}
///`reset()` method sets GICD_PIDR3 to value 0
impl crate::Resettable for GICD_PIDR3rs {
    const RESET_VALUE: u32 = 0;
}
