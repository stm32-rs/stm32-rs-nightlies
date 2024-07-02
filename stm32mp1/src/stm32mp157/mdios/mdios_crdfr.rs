///Register `MDIOS_CRDFR` reader
pub type R = crate::R<MDIOS_CRDFRrs>;
///Register `MDIOS_CRDFR` writer
pub type W = crate::W<MDIOS_CRDFRrs>;
///Field `CRDF` reader - CRDF
pub type CRDF_R = crate::FieldReader<u32>;
///Field `CRDF` writer - CRDF
pub type CRDF_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CRDF
    #[inline(always)]
    pub fn crdf(&self) -> CRDF_R {
        CRDF_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDIOS_CRDFR")
            .field("crdf", &self.crdf())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CRDF
    #[inline(always)]
    #[must_use]
    pub fn crdf(&mut self) -> CRDF_W<MDIOS_CRDFRrs> {
        CRDF_W::new(self, 0)
    }
}
/**MDIOS clear read flag register

You can [`read`](crate::Reg::read) this register and get [`mdios_crdfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_crdfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_CRDFR)*/
pub struct MDIOS_CRDFRrs;
impl crate::RegisterSpec for MDIOS_CRDFRrs {
    type Ux = u32;
}
///`read()` method returns [`mdios_crdfr::R`](R) reader structure
impl crate::Readable for MDIOS_CRDFRrs {}
///`write(|w| ..)` method takes [`mdios_crdfr::W`](W) writer structure
impl crate::Writable for MDIOS_CRDFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MDIOS_CRDFR to value 0
impl crate::Resettable for MDIOS_CRDFRrs {
    const RESET_VALUE: u32 = 0;
}
