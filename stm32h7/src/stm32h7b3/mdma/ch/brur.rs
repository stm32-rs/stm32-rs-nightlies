#[doc = "Register `BRUR` reader"]
pub type R = crate::R<BRURrs>;
#[doc = "Register `BRUR` writer"]
pub type W = crate::W<BRURrs>;
#[doc = "Field `SUV` reader - source adresse update value"]
pub type SUV_R = crate::FieldReader<u16>;
#[doc = "Field `SUV` writer - source adresse update value"]
pub type SUV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DUV` reader - destination address update"]
pub type DUV_R = crate::FieldReader<u16>;
#[doc = "Field `DUV` writer - destination address update"]
pub type DUV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - source adresse update value"]
    #[inline(always)]
    pub fn suv(&self) -> SUV_R {
        SUV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - destination address update"]
    #[inline(always)]
    pub fn duv(&self) -> DUV_R {
        DUV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - source adresse update value"]
    #[inline(always)]
    #[must_use]
    pub fn suv(&mut self) -> SUV_W<BRURrs> {
        SUV_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - destination address update"]
    #[inline(always)]
    #[must_use]
    pub fn duv(&mut self) -> DUV_W<BRURrs> {
        DUV_W::new(self, 16)
    }
}
#[doc = "MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRURrs;
impl crate::RegisterSpec for BRURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brur::R`](R) reader structure"]
impl crate::Readable for BRURrs {}
#[doc = "`write(|w| ..)` method takes [`brur::W`](W) writer structure"]
impl crate::Writable for BRURrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BRUR to value 0"]
impl crate::Resettable for BRURrs {
    const RESET_VALUE: u32 = 0;
}
