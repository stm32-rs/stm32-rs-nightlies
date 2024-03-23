#[doc = "Register `BKP22R` reader"]
pub type R = crate::R<BKP22Rrs>;
#[doc = "Register `BKP22R` writer"]
pub type W = crate::W<BKP22Rrs>;
#[doc = "Field `BKP` reader - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set."]
pub type BKP_R = crate::FieldReader<u32>;
#[doc = "Field `BKP` writer - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set."]
pub type BKP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set."]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set."]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<BKP22Rrs> {
        BKP_W::new(self, 0)
    }
}
#[doc = "TAMP backup 22 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp22r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp22r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BKP22Rrs;
impl crate::RegisterSpec for BKP22Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp22r::R`](R) reader structure"]
impl crate::Readable for BKP22Rrs {}
#[doc = "`write(|w| ..)` method takes [`bkp22r::W`](W) writer structure"]
impl crate::Writable for BKP22Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BKP22R to value 0"]
impl crate::Resettable for BKP22Rrs {
    const RESET_VALUE: u32 = 0;
}
