#[doc = "Register `IV1RR` reader"]
pub type R = crate::R<IV1RRrs>;
#[doc = "Register `IV1RR` writer"]
pub type W = crate::W<IV1RRrs>;
#[doc = "Field `IV` reader - IV127"]
pub type IV_R = crate::FieldReader<u32>;
#[doc = "Field `IV` writer - IV127"]
pub type IV_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IV127"]
    #[inline(always)]
    pub fn iv(&self) -> IV_R {
        IV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IV127"]
    #[inline(always)]
    #[must_use]
    pub fn iv(&mut self) -> IV_W<IV1RRrs> {
        IV_W::new(self, 0)
    }
}
#[doc = "Initialization vector register 1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv1rr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv1rr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IV1RRrs;
impl crate::RegisterSpec for IV1RRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iv1rr::R`](R) reader structure"]
impl crate::Readable for IV1RRrs {}
#[doc = "`write(|w| ..)` method takes [`iv1rr::W`](W) writer structure"]
impl crate::Writable for IV1RRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IV1RR to value 0"]
impl crate::Resettable for IV1RRrs {
    const RESET_VALUE: u32 = 0;
}
