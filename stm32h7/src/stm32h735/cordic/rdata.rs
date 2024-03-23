#[doc = "Register `RDATA` reader"]
pub type R = crate::R<RDATArs>;
#[doc = "Register `RDATA` writer"]
pub type W = crate::W<RDATArs>;
#[doc = "Field `RES` reader - Function result"]
pub type RES_R = crate::FieldReader<u32>;
#[doc = "Field `RES` writer - Function result"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Function result"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Function result"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<RDATArs> {
        RES_W::new(self, 0)
    }
}
#[doc = "Result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDATArs;
impl crate::RegisterSpec for RDATArs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdata::R`](R) reader structure"]
impl crate::Readable for RDATArs {}
#[doc = "`write(|w| ..)` method takes [`rdata::W`](W) writer structure"]
impl crate::Writable for RDATArs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RDATA to value 0"]
impl crate::Resettable for RDATArs {
    const RESET_VALUE: u32 = 0;
}
