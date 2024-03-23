#[doc = "Register `LAR` reader"]
pub type R = crate::R<LARrs>;
#[doc = "Register `LAR` writer"]
pub type W = crate::W<LARrs>;
#[doc = "Field `LAR` reader - Link address register"]
pub type LAR_R = crate::FieldReader<u32>;
#[doc = "Field `LAR` writer - Link address register"]
pub type LAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Link address register"]
    #[inline(always)]
    pub fn lar(&self) -> LAR_R {
        LAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Link address register"]
    #[inline(always)]
    #[must_use]
    pub fn lar(&mut self) -> LAR_W<LARrs> {
        LAR_W::new(self, 0)
    }
}
#[doc = "MDMA channel x Link Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LARrs;
impl crate::RegisterSpec for LARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lar::R`](R) reader structure"]
impl crate::Readable for LARrs {}
#[doc = "`write(|w| ..)` method takes [`lar::W`](W) writer structure"]
impl crate::Writable for LARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAR to value 0"]
impl crate::Resettable for LARrs {
    const RESET_VALUE: u32 = 0;
}
