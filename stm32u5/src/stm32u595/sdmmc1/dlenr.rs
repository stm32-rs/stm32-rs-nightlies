#[doc = "Register `DLENR` reader"]
pub type R = crate::R<DLENRrs>;
#[doc = "Register `DLENR` writer"]
pub type W = crate::W<DLENRrs>;
#[doc = "Field `DATALENGTH` reader - Data length value"]
pub type DATALENGTH_R = crate::FieldReader<u32>;
#[doc = "Field `DATALENGTH` writer - Data length value"]
pub type DATALENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - Data length value"]
    #[inline(always)]
    pub fn datalength(&self) -> DATALENGTH_R {
        DATALENGTH_R::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - Data length value"]
    #[inline(always)]
    #[must_use]
    pub fn datalength(&mut self) -> DATALENGTH_W<DLENRrs> {
        DATALENGTH_W::new(self, 0)
    }
}
#[doc = "data length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLENRrs;
impl crate::RegisterSpec for DLENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dlenr::R`](R) reader structure"]
impl crate::Readable for DLENRrs {}
#[doc = "`write(|w| ..)` method takes [`dlenr::W`](W) writer structure"]
impl crate::Writable for DLENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLENR to value 0"]
impl crate::Resettable for DLENRrs {
    const RESET_VALUE: u32 = 0;
}
