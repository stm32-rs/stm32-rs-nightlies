#[doc = "Register `AMTCR` reader"]
pub type R = crate::R<AMTCRrs>;
#[doc = "Register `AMTCR` writer"]
pub type W = crate::W<AMTCRrs>;
#[doc = "Field `EN` reader - Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DT` reader - Dead Time"]
pub type DT_R = crate::FieldReader;
#[doc = "Field `DT` writer - Dead Time"]
pub type DT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - Dead Time"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<AMTCRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Dead Time"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<AMTCRrs> {
        DT_W::new(self, 8)
    }
}
#[doc = "AHB master timer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`amtcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`amtcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AMTCRrs;
impl crate::RegisterSpec for AMTCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`amtcr::R`](R) reader structure"]
impl crate::Readable for AMTCRrs {}
#[doc = "`write(|w| ..)` method takes [`amtcr::W`](W) writer structure"]
impl crate::Writable for AMTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AMTCR to value 0"]
impl crate::Resettable for AMTCRrs {
    const RESET_VALUE: u32 = 0;
}
