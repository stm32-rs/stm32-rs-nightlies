///Register `STGENR_PIDR5` reader
pub type R = crate::R<STGENR_PIDR5rs>;
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
        f.debug_struct("STGENR_PIDR5")
            .field("pidr5", &self.pidr5())
            .finish()
    }
}
/**STGENR peripheral ID5 register

You can [`read`](crate::Reg::read) this register and get [`stgenr_pidr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENR:STGENR_PIDR5)*/
pub struct STGENR_PIDR5rs;
impl crate::RegisterSpec for STGENR_PIDR5rs {
    type Ux = u32;
}
///`read()` method returns [`stgenr_pidr5::R`](R) reader structure
impl crate::Readable for STGENR_PIDR5rs {}
///`reset()` method sets STGENR_PIDR5 to value 0
impl crate::Resettable for STGENR_PIDR5rs {
    const RESET_VALUE: u32 = 0;
}
