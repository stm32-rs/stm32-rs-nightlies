#[doc = "Register `VVSACCR` reader"]
pub type R = crate::R<VVSACCRrs>;
#[doc = "Register `VVSACCR` writer"]
pub type W = crate::W<VVSACCRrs>;
#[doc = "Field `VSA` reader - Vertical synchronism active duration"]
pub type VSA_R = crate::FieldReader<u16>;
#[doc = "Field `VSA` writer - Vertical synchronism active duration"]
pub type VSA_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Vertical synchronism active duration"]
    #[inline(always)]
    pub fn vsa(&self) -> VSA_R {
        VSA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Vertical synchronism active duration"]
    #[inline(always)]
    #[must_use]
    pub fn vsa(&mut self) -> VSA_W<VVSACCRrs> {
        VSA_W::new(self, 0)
    }
}
#[doc = "DSI Host video VSA current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvsaccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vvsaccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VVSACCRrs;
impl crate::RegisterSpec for VVSACCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vvsaccr::R`](R) reader structure"]
impl crate::Readable for VVSACCRrs {}
#[doc = "`write(|w| ..)` method takes [`vvsaccr::W`](W) writer structure"]
impl crate::Writable for VVSACCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VVSACCR to value 0"]
impl crate::Resettable for VVSACCRrs {
    const RESET_VALUE: u32 = 0;
}
