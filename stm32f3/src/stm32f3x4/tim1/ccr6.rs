#[doc = "Register `CCR6` reader"]
pub type R = crate::R<CCR6rs>;
#[doc = "Register `CCR6` writer"]
pub type W = crate::W<CCR6rs>;
#[doc = "Field `CCR` reader - Capture/Compare value"]
pub type CCR_R = crate::FieldReader<u16>;
#[doc = "Field `CCR` writer - Capture/Compare value"]
pub type CCR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare value"]
    #[inline(always)]
    pub fn ccr(&self) -> CCR_R {
        CCR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare value"]
    #[inline(always)]
    #[must_use]
    pub fn ccr(&mut self) -> CCR_W<CCR6rs> {
        CCR_W::new(self, 0)
    }
}
#[doc = "capture/compare register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR6rs;
impl crate::RegisterSpec for CCR6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr6::R`](R) reader structure"]
impl crate::Readable for CCR6rs {}
#[doc = "`write(|w| ..)` method takes [`ccr6::W`](W) writer structure"]
impl crate::Writable for CCR6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR6 to value 0"]
impl crate::Resettable for CCR6rs {
    const RESET_VALUE: u32 = 0;
}
