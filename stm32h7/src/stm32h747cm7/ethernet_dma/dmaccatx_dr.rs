#[doc = "Register `DMACCATxDR` reader"]
pub type R = crate::R<DMACCATX_DRrs>;
#[doc = "Register `DMACCATxDR` writer"]
pub type W = crate::W<DMACCATX_DRrs>;
#[doc = "Field `CURTDESAPTR` reader - Application Transmit Descriptor Address Pointer"]
pub type CURTDESAPTR_R = crate::FieldReader<u32>;
#[doc = "Field `CURTDESAPTR` writer - Application Transmit Descriptor Address Pointer"]
pub type CURTDESAPTR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Application Transmit Descriptor Address Pointer"]
    #[inline(always)]
    pub fn curtdesaptr(&self) -> CURTDESAPTR_R {
        CURTDESAPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Application Transmit Descriptor Address Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn curtdesaptr(&mut self) -> CURTDESAPTR_W<DMACCATX_DRrs> {
        CURTDESAPTR_W::new(self, 0)
    }
}
#[doc = "Channel current application transmit descriptor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaccatx_dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaccatx_dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACCATX_DRrs;
impl crate::RegisterSpec for DMACCATX_DRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaccatx_dr::R`](R) reader structure"]
impl crate::Readable for DMACCATX_DRrs {}
#[doc = "`write(|w| ..)` method takes [`dmaccatx_dr::W`](W) writer structure"]
impl crate::Writable for DMACCATX_DRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACCATxDR to value 0"]
impl crate::Resettable for DMACCATX_DRrs {
    const RESET_VALUE: u32 = 0;
}
