///Register `DOR2` reader
pub type R = crate::R<DOR2rs>;
///Field `DACC2DOR` reader - DAC channel2 data output These bits are read-only, they contain data output for DAC channel2.
pub type DACC2DOR_R = crate::FieldReader<u16>;
///Field `DACC2DORB` reader - DAC channel2 data output
pub type DACC2DORB_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:11 - DAC channel2 data output These bits are read-only, they contain data output for DAC channel2.
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
        f.debug_struct("DOR2")
            .field("dacc2dor", &self.dacc2dor())
            .field("dacc2dorb", &self.dacc2dorb())
            .finish()
    }
}
/**DAC channel2 data output register

You can [`read`](crate::Reg::read) this register and get [`dor2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G431.html#DAC1:DOR2)*/
pub struct DOR2rs;
impl crate::RegisterSpec for DOR2rs {
    type Ux = u32;
}
///`read()` method returns [`dor2::R`](R) reader structure
impl crate::Readable for DOR2rs {}
///`reset()` method sets DOR2 to value 0
impl crate::Resettable for DOR2rs {
    const RESET_VALUE: u32 = 0;
}
