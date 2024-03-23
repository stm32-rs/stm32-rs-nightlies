#[doc = "Register `S4NDTR` reader"]
pub type R = crate::R<S4NDTRrs>;
#[doc = "Register `S4NDTR` writer"]
pub type W = crate::W<S4NDTRrs>;
#[doc = "Field `NDT` reader - Number of data items to transfer"]
pub type NDT_R = crate::FieldReader<u16>;
#[doc = "Field `NDT` writer - Number of data items to transfer"]
pub type NDT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data items to transfer"]
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data items to transfer"]
    #[inline(always)]
    #[must_use]
    pub fn ndt(&mut self) -> NDT_W<S4NDTRrs> {
        NDT_W::new(self, 0)
    }
}
#[doc = "stream x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s4ndtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s4ndtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S4NDTRrs;
impl crate::RegisterSpec for S4NDTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s4ndtr::R`](R) reader structure"]
impl crate::Readable for S4NDTRrs {}
#[doc = "`write(|w| ..)` method takes [`s4ndtr::W`](W) writer structure"]
impl crate::Writable for S4NDTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S4NDTR to value 0"]
impl crate::Resettable for S4NDTRrs {
    const RESET_VALUE: u32 = 0;
}
