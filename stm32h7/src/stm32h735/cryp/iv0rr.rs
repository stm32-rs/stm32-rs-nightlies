#[doc = "Register `IV0RR` reader"]
pub type R = crate::R<IV0RRrs>;
#[doc = "Register `IV0RR` writer"]
pub type W = crate::W<IV0RRrs>;
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
    pub fn iv(&mut self) -> IV_W<IV0RRrs> {
        IV_W::new(self, 0)
    }
}
#[doc = "initialization vector register 0R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv0rr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv0rr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IV0RRrs;
impl crate::RegisterSpec for IV0RRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iv0rr::R`](R) reader structure"]
impl crate::Readable for IV0RRrs {}
#[doc = "`write(|w| ..)` method takes [`iv0rr::W`](W) writer structure"]
impl crate::Writable for IV0RRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IV0RR to value 0"]
impl crate::Resettable for IV0RRrs {
    const RESET_VALUE: u32 = 0;
}
