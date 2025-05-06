///Register `DOUTR8` reader
pub type R = crate::R<DOUTR8rs>;
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
        f.debug_struct("DOUTR8")
            .field("dout", &self.dout())
            .finish()
    }
}
/**MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`doutr8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDIOS:DOUTR8)*/
pub struct DOUTR8rs;
impl crate::RegisterSpec for DOUTR8rs {
    type Ux = u32;
}
///`read()` method returns [`doutr8::R`](R) reader structure
impl crate::Readable for DOUTR8rs {}
///`reset()` method sets DOUTR8 to value 0
impl crate::Resettable for DOUTR8rs {}
