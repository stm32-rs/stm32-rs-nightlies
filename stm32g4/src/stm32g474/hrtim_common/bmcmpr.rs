#[doc = "Register `BMCMPR` reader"]
pub type R = crate::R<BMCMPRrs>;
#[doc = "Register `BMCMPR` writer"]
pub type W = crate::W<BMCMPRrs>;
#[doc = "Field `BMCMP` reader - BMCMP"]
pub type BMCMP_R = crate::FieldReader<u16>;
#[doc = "Field `BMCMP` writer - BMCMP"]
pub type BMCMP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BMCMP"]
    #[inline(always)]
    pub fn bmcmp(&self) -> BMCMP_R {
        BMCMP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BMCMP"]
    #[inline(always)]
    #[must_use]
    pub fn bmcmp(&mut self) -> BMCMP_W<BMCMPRrs> {
        BMCMP_W::new(self, 0)
    }
}
#[doc = "BMCMPR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmcmpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmcmpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BMCMPRrs;
impl crate::RegisterSpec for BMCMPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmcmpr::R`](R) reader structure"]
impl crate::Readable for BMCMPRrs {}
#[doc = "`write(|w| ..)` method takes [`bmcmpr::W`](W) writer structure"]
impl crate::Writable for BMCMPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BMCMPR to value 0"]
impl crate::Resettable for BMCMPRrs {
    const RESET_VALUE: u32 = 0;
}
