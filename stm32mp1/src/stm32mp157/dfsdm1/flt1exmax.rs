///Register `FLT1EXMAX` reader
pub type R = crate::R<FLT1EXMAXrs>;
///Field `EXMAXCH` reader - EXMAXCH
pub type EXMAXCH_R = crate::FieldReader;
///Field `EXMAX` reader - EXMAX
pub type EXMAX_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:2 - EXMAXCH
    #[inline(always)]
    pub fn exmaxch(&self) -> EXMAXCH_R {
        EXMAXCH_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:31 - EXMAX
    #[inline(always)]
    pub fn exmax(&self) -> EXMAX_R {
        EXMAX_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLT1EXMAX")
            .field("exmaxch", &self.exmaxch())
            .field("exmax", &self.exmax())
            .finish()
    }
}
/**DFSDM filter 1 extremes detector maximum register

You can [`read`](crate::Reg::read) this register and get [`flt1exmax::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT1EXMAX)*/
pub struct FLT1EXMAXrs;
impl crate::RegisterSpec for FLT1EXMAXrs {
    type Ux = u32;
}
///`read()` method returns [`flt1exmax::R`](R) reader structure
impl crate::Readable for FLT1EXMAXrs {}
///`reset()` method sets FLT1EXMAX to value 0x8000_0000
impl crate::Resettable for FLT1EXMAXrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
