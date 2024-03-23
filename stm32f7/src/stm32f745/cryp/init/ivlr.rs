#[doc = "Register `IVLR` reader"]
pub type R = crate::R<IVLRrs>;
#[doc = "Register `IVLR` writer"]
pub type W = crate::W<IVLRrs>;
#[doc = "Field `IV` reader - IV31"]
pub type IV_R = crate::FieldReader<u32>;
#[doc = "Field `IV` writer - IV31"]
pub type IV_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IV31"]
    #[inline(always)]
    pub fn iv(&self) -> IV_R {
        IV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IV31"]
    #[inline(always)]
    #[must_use]
    pub fn iv(&mut self) -> IV_W<IVLRrs> {
        IV_W::new(self, 0)
    }
}
#[doc = "initialization vector registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IVLRrs;
impl crate::RegisterSpec for IVLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ivlr::R`](R) reader structure"]
impl crate::Readable for IVLRrs {}
#[doc = "`write(|w| ..)` method takes [`ivlr::W`](W) writer structure"]
impl crate::Writable for IVLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IVLR to value 0"]
impl crate::Resettable for IVLRrs {
    const RESET_VALUE: u32 = 0;
}
