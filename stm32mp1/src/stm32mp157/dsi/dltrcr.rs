#[doc = "Register `DLTRCR` reader"]
pub type R = crate::R<DLTRCRrs>;
#[doc = "Register `DLTRCR` writer"]
pub type W = crate::W<DLTRCRrs>;
#[doc = "Field `MRD_TIME` reader - MRD_TIME"]
pub type MRD_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `MRD_TIME` writer - MRD_TIME"]
pub type MRD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - MRD_TIME"]
    #[inline(always)]
    pub fn mrd_time(&self) -> MRD_TIME_R {
        MRD_TIME_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - MRD_TIME"]
    #[inline(always)]
    #[must_use]
    pub fn mrd_time(&mut self) -> MRD_TIME_W<DLTRCRrs> {
        MRD_TIME_W::new(self, 0)
    }
}
#[doc = "DSI Host data lane timer read configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dltrcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dltrcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLTRCRrs;
impl crate::RegisterSpec for DLTRCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dltrcr::R`](R) reader structure"]
impl crate::Readable for DLTRCRrs {}
#[doc = "`write(|w| ..)` method takes [`dltrcr::W`](W) writer structure"]
impl crate::Writable for DLTRCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLTRCR to value 0"]
impl crate::Resettable for DLTRCRrs {
    const RESET_VALUE: u32 = 0;
}
