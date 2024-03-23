#[doc = "Register `IVRR` reader"]
pub type R = crate::R<IVRRrs>;
#[doc = "Register `IVRR` writer"]
pub type W = crate::W<IVRRrs>;
#[doc = "Field `IV` reader - IV63"]
pub type IV_R = crate::FieldReader<u32>;
#[doc = "Field `IV` writer - IV63"]
pub type IV_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IV63"]
    #[inline(always)]
    pub fn iv(&self) -> IV_R {
        IV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IV63"]
    #[inline(always)]
    #[must_use]
    pub fn iv(&mut self) -> IV_W<IVRRrs> {
        IV_W::new(self, 0)
    }
}
#[doc = "initialization vector registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IVRRrs;
impl crate::RegisterSpec for IVRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ivrr::R`](R) reader structure"]
impl crate::Readable for IVRRrs {}
#[doc = "`write(|w| ..)` method takes [`ivrr::W`](W) writer structure"]
impl crate::Writable for IVRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IVRR to value 0"]
impl crate::Resettable for IVRRrs {
    const RESET_VALUE: u32 = 0;
}
