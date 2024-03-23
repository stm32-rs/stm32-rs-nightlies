#[doc = "Register `VVSACR` reader"]
pub type R = crate::R<VVSACRrs>;
#[doc = "Register `VVSACR` writer"]
pub type W = crate::W<VVSACRrs>;
#[doc = "Field `VSA` reader - VSA"]
pub type VSA_R = crate::FieldReader<u16>;
#[doc = "Field `VSA` writer - VSA"]
pub type VSA_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - VSA"]
    #[inline(always)]
    pub fn vsa(&self) -> VSA_R {
        VSA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - VSA"]
    #[inline(always)]
    #[must_use]
    pub fn vsa(&mut self) -> VSA_W<VVSACRrs> {
        VSA_W::new(self, 0)
    }
}
#[doc = "DSI Host video VSA configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvsacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vvsacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VVSACRrs;
impl crate::RegisterSpec for VVSACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vvsacr::R`](R) reader structure"]
impl crate::Readable for VVSACRrs {}
#[doc = "`write(|w| ..)` method takes [`vvsacr::W`](W) writer structure"]
impl crate::Writable for VVSACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VVSACR to value 0"]
impl crate::Resettable for VVSACRrs {
    const RESET_VALUE: u32 = 0;
}
