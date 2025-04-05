///Register `FLT2EXMAX` reader
pub type R = crate::R<FLT2EXMAXrs>;
///Field `EXMAXCH` reader - Extremes detector maximum data channel
pub type EXMAXCH_R = crate::FieldReader;
///Field `EXMAX` reader - Extremes detector maximum value
pub type EXMAX_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:2 - Extremes detector maximum data channel
    #[inline(always)]
    pub fn exmaxch(&self) -> EXMAXCH_R {
        EXMAXCH_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:31 - Extremes detector maximum value
    #[inline(always)]
    pub fn exmax(&self) -> EXMAX_R {
        EXMAX_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLT2EXMAX")
            .field("exmax", &self.exmax())
            .field("exmaxch", &self.exmaxch())
            .finish()
    }
}
/**Extremes detector maximum register

You can [`read`](crate::Reg::read) this register and get [`flt2exmax::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:FLT2EXMAX)*/
pub struct FLT2EXMAXrs;
impl crate::RegisterSpec for FLT2EXMAXrs {
    type Ux = u32;
}
///`read()` method returns [`flt2exmax::R`](R) reader structure
impl crate::Readable for FLT2EXMAXrs {}
///`reset()` method sets FLT2EXMAX to value 0x8000_0000
impl crate::Resettable for FLT2EXMAXrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
