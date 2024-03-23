#[doc = "Register `RTOR` reader"]
pub type R = crate::R<RTORrs>;
#[doc = "Register `RTOR` writer"]
pub type W = crate::W<RTORrs>;
#[doc = "Field `RTO` reader - Receiver timeout value"]
pub type RTO_R = crate::FieldReader<u32>;
#[doc = "Field `RTO` writer - Receiver timeout value"]
pub type RTO_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `BLEN` reader - Block Length"]
pub type BLEN_R = crate::FieldReader;
#[doc = "Field `BLEN` writer - Block Length"]
pub type BLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - Receiver timeout value"]
    #[inline(always)]
    pub fn rto(&self) -> RTO_R {
        RTO_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - Block Length"]
    #[inline(always)]
    pub fn blen(&self) -> BLEN_R {
        BLEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Receiver timeout value"]
    #[inline(always)]
    #[must_use]
    pub fn rto(&mut self) -> RTO_W<RTORrs> {
        RTO_W::new(self, 0)
    }
    #[doc = "Bits 24:31 - Block Length"]
    #[inline(always)]
    #[must_use]
    pub fn blen(&mut self) -> BLEN_W<RTORrs> {
        BLEN_W::new(self, 24)
    }
}
#[doc = "Receiver timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTORrs;
impl crate::RegisterSpec for RTORrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtor::R`](R) reader structure"]
impl crate::Readable for RTORrs {}
#[doc = "`write(|w| ..)` method takes [`rtor::W`](W) writer structure"]
impl crate::Writable for RTORrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTOR to value 0"]
impl crate::Resettable for RTORrs {
    const RESET_VALUE: u32 = 0;
}
