#[doc = "Register `FLT1JCHGR` reader"]
pub type R = crate::R<FLT1JCHGRrs>;
#[doc = "Register `FLT1JCHGR` writer"]
pub type W = crate::W<FLT1JCHGRrs>;
#[doc = "Field `JCHG` reader - Injected channel group selection"]
pub type JCHG_R = crate::FieldReader;
#[doc = "Field `JCHG` writer - Injected channel group selection"]
pub type JCHG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Injected channel group selection"]
    #[inline(always)]
    pub fn jchg(&self) -> JCHG_R {
        JCHG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Injected channel group selection"]
    #[inline(always)]
    #[must_use]
    pub fn jchg(&mut self) -> JCHG_W<FLT1JCHGRrs> {
        JCHG_W::new(self, 0)
    }
}
#[doc = "injected channel group selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flt1jchgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flt1jchgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLT1JCHGRrs;
impl crate::RegisterSpec for FLT1JCHGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flt1jchgr::R`](R) reader structure"]
impl crate::Readable for FLT1JCHGRrs {}
#[doc = "`write(|w| ..)` method takes [`flt1jchgr::W`](W) writer structure"]
impl crate::Writable for FLT1JCHGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLT1JCHGR to value 0x01"]
impl crate::Resettable for FLT1JCHGRrs {
    const RESET_VALUE: u32 = 0x01;
}
