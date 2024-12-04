///Register `FLT2EXMIN` reader
pub type R = crate::R<FLT2EXMINrs>;
///Field `EXMINCH` reader - Extremes detector minimum data channel
pub type EXMINCH_R = crate::FieldReader;
///Field `EXMIN` reader - EXMIN
pub type EXMIN_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:2 - Extremes detector minimum data channel
    #[inline(always)]
    pub fn exminch(&self) -> EXMINCH_R {
        EXMINCH_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:31 - EXMIN
    #[inline(always)]
    pub fn exmin(&self) -> EXMIN_R {
        EXMIN_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLT2EXMIN")
            .field("exmin", &self.exmin())
            .field("exminch", &self.exminch())
            .finish()
    }
}
/**Extremes detector minimum register

You can [`read`](crate::Reg::read) this register and get [`flt2exmin::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#DFSDM1:FLT2EXMIN)*/
pub struct FLT2EXMINrs;
impl crate::RegisterSpec for FLT2EXMINrs {
    type Ux = u32;
}
///`read()` method returns [`flt2exmin::R`](R) reader structure
impl crate::Readable for FLT2EXMINrs {}
///`reset()` method sets FLT2EXMIN to value 0x7fff_ff00
impl crate::Resettable for FLT2EXMINrs {
    const RESET_VALUE: u32 = 0x7fff_ff00;
}
