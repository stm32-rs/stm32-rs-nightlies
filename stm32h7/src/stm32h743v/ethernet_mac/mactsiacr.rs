#[doc = "Register `MACTSIACR` reader"]
pub type R = crate::R<MACTSIACRrs>;
#[doc = "Register `MACTSIACR` writer"]
pub type W = crate::W<MACTSIACRrs>;
#[doc = "Field `OSTIAC` reader - One-Step Timestamp Ingress Asymmetry Correction"]
pub type OSTIAC_R = crate::FieldReader<u32>;
#[doc = "Field `OSTIAC` writer - One-Step Timestamp Ingress Asymmetry Correction"]
pub type OSTIAC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - One-Step Timestamp Ingress Asymmetry Correction"]
    #[inline(always)]
    pub fn ostiac(&self) -> OSTIAC_R {
        OSTIAC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - One-Step Timestamp Ingress Asymmetry Correction"]
    #[inline(always)]
    #[must_use]
    pub fn ostiac(&mut self) -> OSTIAC_W<MACTSIACRrs> {
        OSTIAC_W::new(self, 0)
    }
}
#[doc = "Timestamp Ingress asymmetric correction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactsiacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mactsiacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACTSIACRrs;
impl crate::RegisterSpec for MACTSIACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactsiacr::R`](R) reader structure"]
impl crate::Readable for MACTSIACRrs {}
#[doc = "`write(|w| ..)` method takes [`mactsiacr::W`](W) writer structure"]
impl crate::Writable for MACTSIACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACTSIACR to value 0"]
impl crate::Resettable for MACTSIACRrs {
    const RESET_VALUE: u32 = 0;
}
