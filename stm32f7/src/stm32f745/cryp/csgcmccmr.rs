///Register `CSGCMCCM%sR` reader
pub type R = crate::R<CSGCMCCMRrs>;
///Register `CSGCMCCM%sR` writer
pub type W = crate::W<CSGCMCCMRrs>;
///Field `CSGCMCCM0R` reader - CSGCMCCM0R
pub type CSGCMCCM0R_R = crate::FieldReader<u32>;
///Field `CSGCMCCM0R` writer - CSGCMCCM0R
pub type CSGCMCCM0R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSGCMCCM0R
    #[inline(always)]
    pub fn csgcmccm0r(&self) -> CSGCMCCM0R_R {
        CSGCMCCM0R_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSGCMCCMR")
            .field("csgcmccm0r", &self.csgcmccm0r())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSGCMCCM0R
    #[inline(always)]
    #[must_use]
    pub fn csgcmccm0r(&mut self) -> CSGCMCCM0R_W<CSGCMCCMRrs> {
        CSGCMCCM0R_W::new(self, 0)
    }
}
/**context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcmccmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#CRYP:CSGCMCCM[0]R)*/
pub struct CSGCMCCMRrs;
impl crate::RegisterSpec for CSGCMCCMRrs {
    type Ux = u32;
}
///`read()` method returns [`csgcmccmr::R`](R) reader structure
impl crate::Readable for CSGCMCCMRrs {}
///`write(|w| ..)` method takes [`csgcmccmr::W`](W) writer structure
impl crate::Writable for CSGCMCCMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CSGCMCCM%sR to value 0
impl crate::Resettable for CSGCMCCMRrs {
    const RESET_VALUE: u32 = 0;
}
