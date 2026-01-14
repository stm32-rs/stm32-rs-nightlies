///Register `CSGCM%sR` reader
pub type R = crate::R<CSGCMRrs>;
///Register `CSGCM%sR` writer
pub type W = crate::W<CSGCMRrs>;
///Field `CSGCMR` reader - CSGCM0R
pub type CSGCMR_R = crate::FieldReader<u32>;
///Field `CSGCMR` writer - CSGCM0R
pub type CSGCMR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSGCM0R
    #[inline(always)]
    pub fn csgcmr(&self) -> CSGCMR_R {
        CSGCMR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSGCMR")
            .field("csgcmr", &self.csgcmr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSGCM0R
    #[inline(always)]
    pub fn csgcmr(&mut self) -> CSGCMR_W<'_, CSGCMRrs> {
        CSGCMR_W::new(self, 0)
    }
}
/**context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F746.html#CRYP:CSGCM[0]R)*/
pub struct CSGCMRrs;
impl crate::RegisterSpec for CSGCMRrs {
    type Ux = u32;
}
///`read()` method returns [`csgcmr::R`](R) reader structure
impl crate::Readable for CSGCMRrs {}
///`write(|w| ..)` method takes [`csgcmr::W`](W) writer structure
impl crate::Writable for CSGCMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSGCM%sR to value 0
impl crate::Resettable for CSGCMRrs {}
