#[doc = "Register `DVR` reader"]
pub type R = crate::R<DVRrs>;
#[doc = "Register `DVR` writer"]
pub type W = crate::W<DVRrs>;
#[doc = "Field `DV` reader - Default value"]
pub type DV_R = crate::FieldReader<u32>;
#[doc = "Field `DV` writer - Default value"]
pub type DV_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Default value"]
    #[inline(always)]
    pub fn dv(&self) -> DV_R {
        DV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Default value"]
    #[inline(always)]
    #[must_use]
    pub fn dv(&mut self) -> DV_W<DVRrs> {
        DV_W::new(self, 0)
    }
}
#[doc = "Graphic MMU default value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DVRrs;
impl crate::RegisterSpec for DVRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvr::R`](R) reader structure"]
impl crate::Readable for DVRrs {}
#[doc = "`write(|w| ..)` method takes [`dvr::W`](W) writer structure"]
impl crate::Writable for DVRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DVR to value 0"]
impl crate::Resettable for DVRrs {
    const RESET_VALUE: u32 = 0;
}
