///Register `CSGCMCCM%sR` reader
pub type R = crate::R<CSGCMCCMRrs>;
///Register `CSGCMCCM%sR` writer
pub type W = crate::W<CSGCMCCMRrs>;
///Field `CSGCMCCMR` reader - CSGCMCCM0R
pub type CSGCMCCMR_R = crate::FieldReader<u32>;
///Field `CSGCMCCMR` writer - CSGCMCCM0R
pub type CSGCMCCMR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSGCMCCM0R
    #[inline(always)]
    pub fn csgcmccmr(&self) -> CSGCMCCMR_R {
        CSGCMCCMR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSGCMCCMR")
            .field("csgcmccmr", &self.csgcmccmr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSGCMCCM0R
    #[inline(always)]
    pub fn csgcmccmr(&mut self) -> CSGCMCCMR_W<'_, CSGCMCCMRrs> {
        CSGCMCCMR_W::new(self, 0)
    }
}
/**context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcmccmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#CRYP:CSGCMCCM[0]R)*/
pub struct CSGCMCCMRrs;
impl crate::RegisterSpec for CSGCMCCMRrs {
    type Ux = u32;
}
///`read()` method returns [`csgcmccmr::R`](R) reader structure
impl crate::Readable for CSGCMCCMRrs {}
///`write(|w| ..)` method takes [`csgcmccmr::W`](W) writer structure
impl crate::Writable for CSGCMCCMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSGCMCCM%sR to value 0
impl crate::Resettable for CSGCMCCMRrs {}
