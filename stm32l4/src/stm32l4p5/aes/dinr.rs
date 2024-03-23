#[doc = "Register `DINR` reader"]
pub type R = crate::R<DINRrs>;
#[doc = "Register `DINR` writer"]
pub type W = crate::W<DINRrs>;
#[doc = "Field `DIN` reader - Data Input Register"]
pub type DIN_R = crate::FieldReader<u32>;
#[doc = "Field `DIN` writer - Data Input Register"]
pub type DIN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Input Register"]
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Input Register"]
    #[inline(always)]
    #[must_use]
    pub fn din(&mut self) -> DIN_W<DINRrs> {
        DIN_W::new(self, 0)
    }
}
#[doc = "data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dinr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINRrs;
impl crate::RegisterSpec for DINRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr::R`](R) reader structure"]
impl crate::Readable for DINRrs {}
#[doc = "`write(|w| ..)` method takes [`dinr::W`](W) writer structure"]
impl crate::Writable for DINRrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DINR to value 0"]
impl crate::Resettable for DINRrs {
    const RESET_VALUE: u32 = 0;
}
