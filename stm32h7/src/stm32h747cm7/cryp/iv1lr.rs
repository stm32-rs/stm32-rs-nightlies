#[doc = "Register `IV1LR` reader"]
pub type R = crate::R<IV1LRrs>;
#[doc = "Register `IV1LR` writer"]
pub type W = crate::W<IV1LRrs>;
#[doc = "Field `IV` reader - IV95"]
pub type IV_R = crate::FieldReader<u32>;
#[doc = "Field `IV` writer - IV95"]
pub type IV_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IV95"]
    #[inline(always)]
    pub fn iv(&self) -> IV_R {
        IV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IV95"]
    #[inline(always)]
    #[must_use]
    pub fn iv(&mut self) -> IV_W<IV1LRrs> {
        IV_W::new(self, 0)
    }
}
#[doc = "initialization vector registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv1lr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv1lr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IV1LRrs;
impl crate::RegisterSpec for IV1LRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iv1lr::R`](R) reader structure"]
impl crate::Readable for IV1LRrs {}
#[doc = "`write(|w| ..)` method takes [`iv1lr::W`](W) writer structure"]
impl crate::Writable for IV1LRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IV1LR to value 0"]
impl crate::Resettable for IV1LRrs {
    const RESET_VALUE: u32 = 0;
}
