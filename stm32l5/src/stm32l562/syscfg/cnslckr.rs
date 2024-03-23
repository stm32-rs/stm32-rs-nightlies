#[doc = "Register `CNSLCKR` reader"]
pub type R = crate::R<CNSLCKRrs>;
#[doc = "Register `CNSLCKR` writer"]
pub type W = crate::W<CNSLCKRrs>;
#[doc = "Field `LOCKNSVTOR` reader - VTOR_NS register lock"]
pub type LOCKNSVTOR_R = crate::BitReader;
#[doc = "Field `LOCKNSVTOR` writer - VTOR_NS register lock"]
pub type LOCKNSVTOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKNSMPU` reader - Non-secure MPU registers lock"]
pub type LOCKNSMPU_R = crate::BitReader;
#[doc = "Field `LOCKNSMPU` writer - Non-secure MPU registers lock"]
pub type LOCKNSMPU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VTOR_NS register lock"]
    #[inline(always)]
    pub fn locknsvtor(&self) -> LOCKNSVTOR_R {
        LOCKNSVTOR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Non-secure MPU registers lock"]
    #[inline(always)]
    pub fn locknsmpu(&self) -> LOCKNSMPU_R {
        LOCKNSMPU_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VTOR_NS register lock"]
    #[inline(always)]
    #[must_use]
    pub fn locknsvtor(&mut self) -> LOCKNSVTOR_W<CNSLCKRrs> {
        LOCKNSVTOR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Non-secure MPU registers lock"]
    #[inline(always)]
    #[must_use]
    pub fn locknsmpu(&mut self) -> LOCKNSMPU_W<CNSLCKRrs> {
        LOCKNSMPU_W::new(self, 1)
    }
}
#[doc = "SYSCFG CPU non-secure lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnslckr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnslckr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNSLCKRrs;
impl crate::RegisterSpec for CNSLCKRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnslckr::R`](R) reader structure"]
impl crate::Readable for CNSLCKRrs {}
#[doc = "`write(|w| ..)` method takes [`cnslckr::W`](W) writer structure"]
impl crate::Writable for CNSLCKRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNSLCKR to value 0"]
impl crate::Resettable for CNSLCKRrs {
    const RESET_VALUE: u32 = 0;
}
