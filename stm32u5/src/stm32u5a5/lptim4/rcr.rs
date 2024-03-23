#[doc = "Register `RCR` reader"]
pub type R = crate::R<RCRrs>;
#[doc = "Register `RCR` writer"]
pub type W = crate::W<RCRrs>;
#[doc = "Field `REP` reader - Repetition register value"]
pub type REP_R = crate::FieldReader;
#[doc = "Field `REP` writer - Repetition register value"]
pub type REP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Repetition register value"]
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Repetition register value"]
    #[inline(always)]
    #[must_use]
    pub fn rep(&mut self) -> REP_W<RCRrs> {
        REP_W::new(self, 0)
    }
}
#[doc = "LPTIM repetition register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCRrs;
impl crate::RegisterSpec for RCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcr::R`](R) reader structure"]
impl crate::Readable for RCRrs {}
#[doc = "`write(|w| ..)` method takes [`rcr::W`](W) writer structure"]
impl crate::Writable for RCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCR to value 0"]
impl crate::Resettable for RCRrs {
    const RESET_VALUE: u32 = 0;
}
