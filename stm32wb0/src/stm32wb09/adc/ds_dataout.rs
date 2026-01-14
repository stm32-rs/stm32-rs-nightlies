///Register `DS_DATAOUT` reader
pub type R = crate::R<DS_DATAOUTrs>;
///Field `DS_DATA` reader - DS_DATA\[15:0\]: contain the converted data at the output of the Down Sampler.
pub type DS_DATA_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - DS_DATA\[15:0\]: contain the converted data at the output of the Down Sampler.
    #[inline(always)]
    pub fn ds_data(&self) -> DS_DATA_R {
        DS_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DS_DATAOUT")
            .field("ds_data", &self.ds_data())
            .finish()
    }
}
/**DS_DATAOUT register

You can [`read`](crate::Reg::read) this register and get [`ds_dataout::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#ADC:DS_DATAOUT)*/
pub struct DS_DATAOUTrs;
impl crate::RegisterSpec for DS_DATAOUTrs {
    type Ux = u32;
}
///`read()` method returns [`ds_dataout::R`](R) reader structure
impl crate::Readable for DS_DATAOUTrs {}
///`reset()` method sets DS_DATAOUT to value 0
impl crate::Resettable for DS_DATAOUTrs {}
