///Register `DF_DATAOUT` reader
pub type R = crate::R<DF_DATAOUTrs>;
///Field `DF_DATA` reader - contain the converted data at the output of the decimation filter.
pub type DF_DATA_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - contain the converted data at the output of the decimation filter.
    #[inline(always)]
    pub fn df_data(&self) -> DF_DATA_R {
        DF_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DF_DATAOUT")
            .field("df_data", &self.df_data())
            .finish()
    }
}
/**Decimation filter Data output register

You can [`read`](crate::Reg::read) this register and get [`df_dataout::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#ADC:DF_DATAOUT)*/
pub struct DF_DATAOUTrs;
impl crate::RegisterSpec for DF_DATAOUTrs {
    type Ux = u32;
}
///`read()` method returns [`df_dataout::R`](R) reader structure
impl crate::Readable for DF_DATAOUTrs {}
///`reset()` method sets DF_DATAOUT to value 0
impl crate::Resettable for DF_DATAOUTrs {}
