///Register `DOUTR30` reader
pub type R = crate::R<DOUTR30rs>;
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
        f.debug_struct("DOUTR30")
            .field("dout", &self.dout())
            .finish()
    }
}
/**MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`doutr30::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDIOS:DOUTR30)*/
pub struct DOUTR30rs;
impl crate::RegisterSpec for DOUTR30rs {
    type Ux = u32;
}
///`read()` method returns [`doutr30::R`](R) reader structure
impl crate::Readable for DOUTR30rs {}
///`reset()` method sets DOUTR30 to value 0
impl crate::Resettable for DOUTR30rs {}
