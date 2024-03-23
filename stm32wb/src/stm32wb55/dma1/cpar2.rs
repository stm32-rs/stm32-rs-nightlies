#[doc = "Register `CPAR2` reader"]
pub type R = crate::R<CPAR2rs>;
#[doc = "Register `CPAR2` writer"]
pub type W = crate::W<CPAR2rs>;
#[doc = "Field `PA` reader - Peripheral address"]
pub type PA_R = crate::FieldReader<u32>;
#[doc = "Field `PA` writer - Peripheral address"]
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<CPAR2rs> {
        PA_W::new(self, 0)
    }
}
#[doc = "channel x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPAR2rs;
impl crate::RegisterSpec for CPAR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpar2::R`](R) reader structure"]
impl crate::Readable for CPAR2rs {}
#[doc = "`write(|w| ..)` method takes [`cpar2::W`](W) writer structure"]
impl crate::Writable for CPAR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPAR2 to value 0"]
impl crate::Resettable for CPAR2rs {
    const RESET_VALUE: u32 = 0;
}
