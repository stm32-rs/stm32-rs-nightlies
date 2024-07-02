///Register `DAC_DOR2` reader
pub type R = crate::R<DAC_DOR2rs>;
///Field `DACC2DOR` reader - DAC channel2 data output
pub type DACC2DOR_R = crate::FieldReader<u16>;
///Field `DACC2DORB` reader - DAC channel2 data output
pub type DACC2DORB_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:11 - DAC channel2 data output
    #[inline(always)]
    pub fn dacc2dor(&self) -> DACC2DOR_R {
        DACC2DOR_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - DAC channel2 data output
    #[inline(always)]
    pub fn dacc2dorb(&self) -> DACC2DORB_R {
        DACC2DORB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_DOR2")
            .field("dacc2dor", &self.dacc2dor())
            .field("dacc2dorb", &self.dacc2dorb())
            .finish()
    }
}
/**DAC channel2 data output register

You can [`read`](crate::Reg::read) this register and get [`dac_dor2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#DAC1:DAC_DOR2)*/
pub struct DAC_DOR2rs;
impl crate::RegisterSpec for DAC_DOR2rs {
    type Ux = u32;
}
///`read()` method returns [`dac_dor2::R`](R) reader structure
impl crate::Readable for DAC_DOR2rs {}
///`reset()` method sets DAC_DOR2 to value 0
impl crate::Resettable for DAC_DOR2rs {
    const RESET_VALUE: u32 = 0;
}
