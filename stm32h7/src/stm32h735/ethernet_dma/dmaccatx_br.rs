#[doc = "Register `DMACCATxBR` reader"]
pub type R = crate::R<DMACCATX_BRrs>;
#[doc = "Register `DMACCATxBR` writer"]
pub type W = crate::W<DMACCATX_BRrs>;
#[doc = "Field `CURTBUFAPTR` reader - Application Transmit Buffer Address Pointer"]
pub type CURTBUFAPTR_R = crate::FieldReader<u32>;
#[doc = "Field `CURTBUFAPTR` writer - Application Transmit Buffer Address Pointer"]
pub type CURTBUFAPTR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Application Transmit Buffer Address Pointer"]
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CURTBUFAPTR_R {
        CURTBUFAPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Application Transmit Buffer Address Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn curtbufaptr(&mut self) -> CURTBUFAPTR_W<DMACCATX_BRrs> {
        CURTBUFAPTR_W::new(self, 0)
    }
}
#[doc = "Channel current application transmit buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaccatx_br::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaccatx_br::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACCATX_BRrs;
impl crate::RegisterSpec for DMACCATX_BRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaccatx_br::R`](R) reader structure"]
impl crate::Readable for DMACCATX_BRrs {}
#[doc = "`write(|w| ..)` method takes [`dmaccatx_br::W`](W) writer structure"]
impl crate::Writable for DMACCATX_BRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACCATxBR to value 0"]
impl crate::Resettable for DMACCATX_BRrs {
    const RESET_VALUE: u32 = 0;
}
