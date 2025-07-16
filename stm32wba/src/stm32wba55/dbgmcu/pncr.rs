///Register `PNCR` reader
pub type R = crate::R<PNCRrs>;
///Field `CODIFICATION` reader - Part number codification
pub type CODIFICATION_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Part number codification
    #[inline(always)]
    pub fn codification(&self) -> CODIFICATION_R {
        CODIFICATION_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PNCR")
            .field("codification", &self.codification())
            .finish()
    }
}
/**DBGMCU part number codification register

You can [`read`](crate::Reg::read) this register and get [`pncr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#DBGMCU:PNCR)*/
pub struct PNCRrs;
impl crate::RegisterSpec for PNCRrs {
    type Ux = u32;
}
///`read()` method returns [`pncr::R`](R) reader structure
impl crate::Readable for PNCRrs {}
///`reset()` method sets PNCR to value 0
impl crate::Resettable for PNCRrs {}
