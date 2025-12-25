///Register `CWRFR` reader
pub type R = crate::R<CWRFRrs>;
///Register `CWRFR` writer
pub type W = crate::W<CWRFRrs>;
///Field `CWRF` reader - Clear the write flag
pub type CWRF_R = crate::FieldReader<u32>;
///Field `CWRF` writer - Clear the write flag
pub type CWRF_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Clear the write flag
    #[inline(always)]
    pub fn cwrf(&self) -> CWRF_R {
        CWRF_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CWRFR").field("cwrf", &self.cwrf()).finish()
    }
}
impl W {
    ///Bits 0:31 - Clear the write flag
    #[inline(always)]
    pub fn cwrf(&mut self) -> CWRF_W<'_, CWRFRrs> {
        CWRF_W::new(self, 0)
    }
}
/**MDIOS clear write flag register

You can [`read`](crate::Reg::read) this register and get [`cwrfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwrfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#MDIOS:CWRFR)*/
pub struct CWRFRrs;
impl crate::RegisterSpec for CWRFRrs {
    type Ux = u32;
}
///`read()` method returns [`cwrfr::R`](R) reader structure
impl crate::Readable for CWRFRrs {}
///`write(|w| ..)` method takes [`cwrfr::W`](W) writer structure
impl crate::Writable for CWRFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CWRFR to value 0
impl crate::Resettable for CWRFRrs {}
