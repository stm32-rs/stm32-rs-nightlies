///Register `DOUTR` reader
pub type R = crate::R<DOUTRrs>;
///Field `DOUTR` reader - DOUTR\[x+31:x\]: One of four 32-bit words of a 128-bit output data block being read from the
pub type DOUTR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - DOUTR\[x+31:x\]: One of four 32-bit words of a 128-bit output data block being read from the
    #[inline(always)]
    pub fn doutr(&self) -> DOUTR_R {
        DOUTR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR")
            .field("doutr", &self.doutr())
            .finish()
    }
}
/**AES_DOUTR register

You can [`read`](crate::Reg::read) this register and get [`doutr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#AES:DOUTR)*/
pub struct DOUTRrs;
impl crate::RegisterSpec for DOUTRrs {
    type Ux = u32;
}
///`read()` method returns [`doutr::R`](R) reader structure
impl crate::Readable for DOUTRrs {}
///`reset()` method sets DOUTR to value 0
impl crate::Resettable for DOUTRrs {}
