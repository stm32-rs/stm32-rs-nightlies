#[doc = "Register `DMACCARxBR` reader"]
pub type R = crate::R<DMACCARX_BRrs>;
#[doc = "Register `DMACCARxBR` writer"]
pub type W = crate::W<DMACCARX_BRrs>;
#[doc = "Field `CURRBUFAPTR` reader - Application Receive Buffer Address Pointer"]
pub type CURRBUFAPTR_R = crate::FieldReader<u32>;
#[doc = "Field `CURRBUFAPTR` writer - Application Receive Buffer Address Pointer"]
pub type CURRBUFAPTR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Application Receive Buffer Address Pointer"]
    #[inline(always)]
    pub fn currbufaptr(&self) -> CURRBUFAPTR_R {
        CURRBUFAPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Application Receive Buffer Address Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn currbufaptr(&mut self) -> CURRBUFAPTR_W<DMACCARX_BRrs> {
        CURRBUFAPTR_W::new(self, 0)
    }
}
#[doc = "Channel current application receive buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaccarx_br::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaccarx_br::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACCARX_BRrs;
impl crate::RegisterSpec for DMACCARX_BRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaccarx_br::R`](R) reader structure"]
impl crate::Readable for DMACCARX_BRrs {}
#[doc = "`write(|w| ..)` method takes [`dmaccarx_br::W`](W) writer structure"]
impl crate::Writable for DMACCARX_BRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACCARxBR to value 0"]
impl crate::Resettable for DMACCARX_BRrs {
    const RESET_VALUE: u32 = 0;
}
