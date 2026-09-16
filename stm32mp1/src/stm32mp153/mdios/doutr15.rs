///Register `DOUTR15` reader
pub type R = crate::R<DOUTR15rs>;
///Field `DOUT` reader - DOUT
pub type DOUT_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - DOUT
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTR15")
            .field("dout", &self.dout())
            .finish()
    }
}
/**MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`doutr15::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDIOS:DOUTR15)*/
pub struct DOUTR15rs;
impl crate::RegisterSpec for DOUTR15rs {
    type Ux = u32;
}
///`read()` method returns [`doutr15::R`](R) reader structure
impl crate::Readable for DOUTR15rs {}
///`reset()` method sets DOUTR15 to value 0
impl crate::Resettable for DOUTR15rs {}
