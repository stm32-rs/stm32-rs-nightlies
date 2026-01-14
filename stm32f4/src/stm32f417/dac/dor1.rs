///Register `DOR1` reader
pub type R = crate::R<DOR1rs>;
///Field `DACC1DOR` reader - DAC channel1 data output
pub type DACC1DOR_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:11 - DAC channel1 data output
    #[inline(always)]
    pub fn dacc1dor(&self) -> DACC1DOR_R {
        DACC1DOR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOR1")
            .field("dacc1dor", &self.dacc1dor())
            .finish()
    }
}
/**channel1 data output register

You can [`read`](crate::Reg::read) this register and get [`dor1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#DAC:DOR1)*/
pub struct DOR1rs;
impl crate::RegisterSpec for DOR1rs {
    type Ux = u32;
}
///`read()` method returns [`dor1::R`](R) reader structure
impl crate::Readable for DOR1rs {}
///`reset()` method sets DOR1 to value 0
impl crate::Resettable for DOR1rs {}
