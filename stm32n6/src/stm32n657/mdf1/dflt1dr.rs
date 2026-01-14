///Register `DFLT1DR` reader
pub type R = crate::R<DFLT1DRrs>;
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
        f.debug_struct("DFLT1DR").field("dr", &self.dr()).finish()
    }
}
/**MDF digital filter data register 1

You can [`read`](crate::Reg::read) this register and get [`dflt1dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDF1:DFLT1DR)*/
pub struct DFLT1DRrs;
impl crate::RegisterSpec for DFLT1DRrs {
    type Ux = u32;
}
///`read()` method returns [`dflt1dr::R`](R) reader structure
impl crate::Readable for DFLT1DRrs {}
///`reset()` method sets DFLT1DR to value 0
impl crate::Resettable for DFLT1DRrs {}
