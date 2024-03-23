#[doc = "Register `LPTIM_ARR` reader"]
pub type R = crate::R<LPTIM_ARRrs>;
#[doc = "Register `LPTIM_ARR` writer"]
pub type W = crate::W<LPTIM_ARRrs>;
#[doc = "Field `ARR` reader - ARR"]
pub type ARR_R = crate::FieldReader<u16>;
#[doc = "Field `ARR` writer - ARR"]
pub type ARR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ARR"]
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ARR"]
    #[inline(always)]
    #[must_use]
    pub fn arr(&mut self) -> ARR_W<LPTIM_ARRrs> {
        ARR_W::new(self, 0)
    }
}
#[doc = "LPTIM autoreload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_arr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim_arr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPTIM_ARRrs;
impl crate::RegisterSpec for LPTIM_ARRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lptim_arr::R`](R) reader structure"]
impl crate::Readable for LPTIM_ARRrs {}
#[doc = "`write(|w| ..)` method takes [`lptim_arr::W`](W) writer structure"]
impl crate::Writable for LPTIM_ARRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPTIM_ARR to value 0x01"]
impl crate::Resettable for LPTIM_ARRrs {
    const RESET_VALUE: u32 = 0x01;
}
