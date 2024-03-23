#[doc = "Register `CH5DLYR` reader"]
pub type R = crate::R<CH5DLYRrs>;
#[doc = "Register `CH5DLYR` writer"]
pub type W = crate::W<CH5DLYRrs>;
#[doc = "Field `PLSSKP` reader - PLSSKP"]
pub type PLSSKP_R = crate::FieldReader;
#[doc = "Field `PLSSKP` writer - PLSSKP"]
pub type PLSSKP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - PLSSKP"]
    #[inline(always)]
    pub fn plsskp(&self) -> PLSSKP_R {
        PLSSKP_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PLSSKP"]
    #[inline(always)]
    #[must_use]
    pub fn plsskp(&mut self) -> PLSSKP_W<CH5DLYRrs> {
        PLSSKP_W::new(self, 0)
    }
}
#[doc = "channel y delay register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5dlyr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5dlyr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH5DLYRrs;
impl crate::RegisterSpec for CH5DLYRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch5dlyr::R`](R) reader structure"]
impl crate::Readable for CH5DLYRrs {}
#[doc = "`write(|w| ..)` method takes [`ch5dlyr::W`](W) writer structure"]
impl crate::Writable for CH5DLYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH5DLYR to value 0"]
impl crate::Resettable for CH5DLYRrs {
    const RESET_VALUE: u32 = 0;
}
