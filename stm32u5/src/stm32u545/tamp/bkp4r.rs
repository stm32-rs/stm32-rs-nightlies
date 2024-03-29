#[doc = "Register `BKP4R` reader"]
pub type R = crate::R<BKP4Rrs>;
#[doc = "Register `BKP4R` writer"]
pub type W = crate::W<BKP4Rrs>;
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
    pub fn bkp(&mut self) -> BKP_W<BKP4Rrs> {
        BKP_W::new(self, 0)
    }
}
#[doc = "TAMP backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp4r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp4r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BKP4Rrs;
impl crate::RegisterSpec for BKP4Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp4r::R`](R) reader structure"]
impl crate::Readable for BKP4Rrs {}
#[doc = "`write(|w| ..)` method takes [`bkp4r::W`](W) writer structure"]
impl crate::Writable for BKP4Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BKP4R to value 0"]
impl crate::Resettable for BKP4Rrs {
    const RESET_VALUE: u32 = 0;
}
