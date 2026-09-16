///Register `DOUT` reader
pub type R = crate::R<DOUTrs>;
///Field `DATAOUT` reader - Data output
pub type DATAOUT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Data output
    #[inline(always)]
    pub fn dataout(&self) -> DATAOUT_R {
        DATAOUT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUT")
            .field("dataout", &self.dataout())
            .finish()
    }
}
/**data output register

You can [`read`](crate::Reg::read) this register and get [`dout::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F779.html#CRYP:DOUT)*/
pub struct DOUTrs;
impl crate::RegisterSpec for DOUTrs {
    type Ux = u32;
}
///`read()` method returns [`dout::R`](R) reader structure
impl crate::Readable for DOUTrs {}
///`reset()` method sets DOUT to value 0
impl crate::Resettable for DOUTrs {}
