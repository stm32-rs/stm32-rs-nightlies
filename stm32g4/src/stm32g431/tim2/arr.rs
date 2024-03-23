#[doc = "Register `ARR` reader"]
pub type R = crate::R<ARRrs>;
#[doc = "Register `ARR` writer"]
pub type W = crate::W<ARRrs>;
#[doc = "Field `ARR` reader - Auto-reload value"]
pub type ARR_R = crate::FieldReader<u32>;
#[doc = "Field `ARR` writer - Auto-reload value"]
pub type ARR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Auto-reload value"]
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Auto-reload value"]
    #[inline(always)]
    #[must_use]
    pub fn arr(&mut self) -> ARR_W<ARRrs> {
        ARR_W::new(self, 0)
    }
}
#[doc = "auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARRrs;
impl crate::RegisterSpec for ARRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arr::R`](R) reader structure"]
impl crate::Readable for ARRrs {}
#[doc = "`write(|w| ..)` method takes [`arr::W`](W) writer structure"]
impl crate::Writable for ARRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARR to value 0xffff_ffff"]
impl crate::Resettable for ARRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
