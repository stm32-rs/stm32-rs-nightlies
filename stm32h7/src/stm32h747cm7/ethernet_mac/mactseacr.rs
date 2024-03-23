#[doc = "Register `MACTSEACR` reader"]
pub type R = crate::R<MACTSEACRrs>;
#[doc = "Register `MACTSEACR` writer"]
pub type W = crate::W<MACTSEACRrs>;
#[doc = "Field `OSTEAC` reader - One-Step Timestamp Egress Asymmetry Correction"]
pub type OSTEAC_R = crate::FieldReader<u32>;
#[doc = "Field `OSTEAC` writer - One-Step Timestamp Egress Asymmetry Correction"]
pub type OSTEAC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - One-Step Timestamp Egress Asymmetry Correction"]
    #[inline(always)]
    pub fn osteac(&self) -> OSTEAC_R {
        OSTEAC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - One-Step Timestamp Egress Asymmetry Correction"]
    #[inline(always)]
    #[must_use]
    pub fn osteac(&mut self) -> OSTEAC_W<MACTSEACRrs> {
        OSTEAC_W::new(self, 0)
    }
}
#[doc = "Timestamp Egress asymmetric correction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactseacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mactseacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACTSEACRrs;
impl crate::RegisterSpec for MACTSEACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactseacr::R`](R) reader structure"]
impl crate::Readable for MACTSEACRrs {}
#[doc = "`write(|w| ..)` method takes [`mactseacr::W`](W) writer structure"]
impl crate::Writable for MACTSEACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACTSEACR to value 0"]
impl crate::Resettable for MACTSEACRrs {
    const RESET_VALUE: u32 = 0;
}
