///Register `EXMAX` reader
pub type R = crate::R<EXMAXrs>;
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
        f.debug_struct("EXMAX")
            .field("exmaxch", &self.exmaxch())
            .field("exmax", &self.exmax())
            .finish()
    }
}
/**DFSDM Extremes detector maximum register

You can [`read`](crate::Reg::read) this register and get [`exmax::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EXMAXrs;
impl crate::RegisterSpec for EXMAXrs {
    type Ux = u32;
}
///`read()` method returns [`exmax::R`](R) reader structure
impl crate::Readable for EXMAXrs {}
///`reset()` method sets EXMAX to value 0
impl crate::Resettable for EXMAXrs {}
