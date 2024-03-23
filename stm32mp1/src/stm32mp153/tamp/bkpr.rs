#[doc = "Register `BKP%sR` reader"]
pub type R = crate::R<BKPRrs>;
#[doc = "Register `BKP%sR` writer"]
pub type W = crate::W<BKPRrs>;
#[doc = "Field `BKP` reader - BKP"]
pub type BKP_R = crate::FieldReader<u32>;
#[doc = "Field `BKP` writer - BKP"]
pub type BKP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<BKPRrs> {
        BKP_W::new(self, 0)
    }
}
#[doc = "TAMP backup %s register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BKPRrs;
impl crate::RegisterSpec for BKPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkpr::R`](R) reader structure"]
impl crate::Readable for BKPRrs {}
#[doc = "`write(|w| ..)` method takes [`bkpr::W`](W) writer structure"]
impl crate::Writable for BKPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BKP%sR to value 0"]
impl crate::Resettable for BKPRrs {
    const RESET_VALUE: u32 = 0;
}
