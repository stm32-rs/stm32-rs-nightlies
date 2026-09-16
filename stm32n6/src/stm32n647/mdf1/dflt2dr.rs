///Register `DFLT2DR` reader
pub type R = crate::R<DFLT2DRrs>;
///Field `DR` reader - Data processed by digital filter
pub type DR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 8:31 - Data processed by digital filter
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFLT2DR").field("dr", &self.dr()).finish()
    }
}
/**MDF digital filter data register 2

You can [`read`](crate::Reg::read) this register and get [`dflt2dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#MDF1:DFLT2DR)*/
pub struct DFLT2DRrs;
impl crate::RegisterSpec for DFLT2DRrs {
    type Ux = u32;
}
///`read()` method returns [`dflt2dr::R`](R) reader structure
impl crate::Readable for DFLT2DRrs {}
///`reset()` method sets DFLT2DR to value 0
impl crate::Resettable for DFLT2DRrs {}
