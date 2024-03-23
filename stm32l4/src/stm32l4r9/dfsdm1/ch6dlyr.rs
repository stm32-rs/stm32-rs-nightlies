#[doc = "Register `CH6DLYR` reader"]
pub type R = crate::R<CH6DLYRrs>;
#[doc = "Register `CH6DLYR` writer"]
pub type W = crate::W<CH6DLYRrs>;
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
    pub fn plsskp(&mut self) -> PLSSKP_W<CH6DLYRrs> {
        PLSSKP_W::new(self, 0)
    }
}
#[doc = "channel y delay register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6dlyr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6dlyr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH6DLYRrs;
impl crate::RegisterSpec for CH6DLYRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch6dlyr::R`](R) reader structure"]
impl crate::Readable for CH6DLYRrs {}
#[doc = "`write(|w| ..)` method takes [`ch6dlyr::W`](W) writer structure"]
impl crate::Writable for CH6DLYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH6DLYR to value 0"]
impl crate::Resettable for CH6DLYRrs {
    const RESET_VALUE: u32 = 0;
}
