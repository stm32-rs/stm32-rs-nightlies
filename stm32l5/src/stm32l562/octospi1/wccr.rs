#[doc = "Register `WCCR` reader"]
pub type R = crate::R<WCCRrs>;
#[doc = "Register `WCCR` writer"]
pub type W = crate::W<WCCRrs>;
#[doc = "Field `REFRESH` reader - REFRESH"]
pub type REFRESH_R = crate::FieldReader<u16>;
#[doc = "Field `REFRESH` writer - REFRESH"]
pub type REFRESH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REFRESH"]
    #[inline(always)]
    pub fn refresh(&self) -> REFRESH_R {
        REFRESH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REFRESH"]
    #[inline(always)]
    #[must_use]
    pub fn refresh(&mut self) -> REFRESH_W<WCCRrs> {
        REFRESH_W::new(self, 0)
    }
}
#[doc = "WCCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WCCRrs;
impl crate::RegisterSpec for WCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wccr::R`](R) reader structure"]
impl crate::Readable for WCCRrs {}
#[doc = "`write(|w| ..)` method takes [`wccr::W`](W) writer structure"]
impl crate::Writable for WCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WCCR to value 0"]
impl crate::Resettable for WCCRrs {
    const RESET_VALUE: u32 = 0;
}
