#[doc = "Register `DINR` writer"]
pub type W = crate::W<DINRrs>;
#[doc = "Field `DIN` writer - Input data word"]
pub type DIN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Input data word"]
    #[inline(always)]
    #[must_use]
    pub fn din(&mut self) -> DIN_W<DINRrs> {
        DIN_W::new(self, 0)
    }
}
#[doc = "data input register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dinr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINRrs;
impl crate::RegisterSpec for DINRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dinr::W`](W) writer structure"]
impl crate::Writable for DINRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DINR to value 0"]
impl crate::Resettable for DINRrs {
    const RESET_VALUE: u32 = 0;
}
