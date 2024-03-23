#[doc = "Register `CCR4` reader"]
pub type R = crate::R<CCR4rs>;
#[doc = "Register `CCR4` writer"]
pub type W = crate::W<CCR4rs>;
#[doc = "Field `CCR4` reader - Capture/Compare 4 value"]
pub type CCR4_R = crate::FieldReader<u32>;
#[doc = "Field `CCR4` writer - Capture/Compare 4 value"]
pub type CCR4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Capture/Compare 4 value"]
    #[inline(always)]
    pub fn ccr4(&self) -> CCR4_R {
        CCR4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Capture/Compare 4 value"]
    #[inline(always)]
    #[must_use]
    pub fn ccr4(&mut self) -> CCR4_W<CCR4rs> {
        CCR4_W::new(self, 0)
    }
}
#[doc = "capture/compare register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR4rs;
impl crate::RegisterSpec for CCR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr4::R`](R) reader structure"]
impl crate::Readable for CCR4rs {}
#[doc = "`write(|w| ..)` method takes [`ccr4::W`](W) writer structure"]
impl crate::Writable for CCR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR4 to value 0"]
impl crate::Resettable for CCR4rs {
    const RESET_VALUE: u32 = 0;
}
