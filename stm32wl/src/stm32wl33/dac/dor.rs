///Register `DOR` reader
pub type R = crate::R<DORrs>;
///Field `DACDOR` reader - DACDOR\[5:0\]: DAC channel data output These bits are read-only, they contain data output for DAC channel.
pub type DACDOR_R = crate::FieldReader;
impl R {
    ///Bits 0:5 - DACDOR\[5:0\]: DAC channel data output These bits are read-only, they contain data output for DAC channel.
    #[inline(always)]
    pub fn dacdor(&self) -> DACDOR_R {
        DACDOR_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOR")
            .field("dacdor", &self.dacdor())
            .finish()
    }
}
/**DOR register

You can [`read`](crate::Reg::read) this register and get [`dor::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DAC:DOR)*/
pub struct DORrs;
impl crate::RegisterSpec for DORrs {
    type Ux = u32;
}
///`read()` method returns [`dor::R`](R) reader structure
impl crate::Readable for DORrs {}
///`reset()` method sets DOR to value 0
impl crate::Resettable for DORrs {}
