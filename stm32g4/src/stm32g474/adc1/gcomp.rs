#[doc = "Register `GCOMP` reader"]
pub type R = crate::R<GCOMPrs>;
#[doc = "Register `GCOMP` writer"]
pub type W = crate::W<GCOMPrs>;
#[doc = "Field `GCOMPCOEFF` reader - Gain compensation coefficient"]
pub type GCOMPCOEFF_R = crate::FieldReader<u16>;
#[doc = "Field `GCOMPCOEFF` writer - Gain compensation coefficient"]
pub type GCOMPCOEFF_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Gain compensation coefficient"]
    #[inline(always)]
    pub fn gcompcoeff(&self) -> GCOMPCOEFF_R {
        GCOMPCOEFF_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Gain compensation coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn gcompcoeff(&mut self) -> GCOMPCOEFF_W<GCOMPrs> {
        GCOMPCOEFF_W::new(self, 0)
    }
}
#[doc = "Gain compensation Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcomp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcomp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCOMPrs;
impl crate::RegisterSpec for GCOMPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcomp::R`](R) reader structure"]
impl crate::Readable for GCOMPrs {}
#[doc = "`write(|w| ..)` method takes [`gcomp::W`](W) writer structure"]
impl crate::Writable for GCOMPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCOMP to value 0"]
impl crate::Resettable for GCOMPrs {
    const RESET_VALUE: u32 = 0;
}
