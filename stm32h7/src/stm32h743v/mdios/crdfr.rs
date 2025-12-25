///Register `CRDFR` reader
pub type R = crate::R<CRDFRrs>;
///Register `CRDFR` writer
pub type W = crate::W<CRDFRrs>;
///Field `CRDF` reader - Clear the read flag
pub type CRDF_R = crate::FieldReader<u32>;
///Field `CRDF` writer - Clear the read flag
pub type CRDF_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Clear the read flag
    #[inline(always)]
    pub fn crdf(&self) -> CRDF_R {
        CRDF_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRDFR").field("crdf", &self.crdf()).finish()
    }
}
impl W {
    ///Bits 0:31 - Clear the read flag
    #[inline(always)]
    pub fn crdf(&mut self) -> CRDF_W<'_, CRDFRrs> {
        CRDF_W::new(self, 0)
    }
}
/**MDIOS clear read flag register

You can [`read`](crate::Reg::read) this register and get [`crdfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crdfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#MDIOS:CRDFR)*/
pub struct CRDFRrs;
impl crate::RegisterSpec for CRDFRrs {
    type Ux = u32;
}
///`read()` method returns [`crdfr::R`](R) reader structure
impl crate::Readable for CRDFRrs {}
///`write(|w| ..)` method takes [`crdfr::W`](W) writer structure
impl crate::Writable for CRDFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRDFR to value 0
impl crate::Resettable for CRDFRrs {}
