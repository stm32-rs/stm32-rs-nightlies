///Register `FLT3EXMIN` reader
pub type R = crate::R<FLT3EXMINrs>;
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
        f.debug_struct("FLT3EXMIN")
            .field("exmin", &self.exmin())
            .field("exminch", &self.exminch())
            .finish()
    }
}
/**Extremes detector minimum register

You can [`read`](crate::Reg::read) this register and get [`flt3exmin::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#DFSDM1:FLT3EXMIN)*/
pub struct FLT3EXMINrs;
impl crate::RegisterSpec for FLT3EXMINrs {
    type Ux = u32;
}
///`read()` method returns [`flt3exmin::R`](R) reader structure
impl crate::Readable for FLT3EXMINrs {}
///`reset()` method sets FLT3EXMIN to value 0x7fff_ff00
impl crate::Resettable for FLT3EXMINrs {
    const RESET_VALUE: u32 = 0x7fff_ff00;
}
