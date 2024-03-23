#[doc = "Register `CNDTR6` reader"]
pub type R = crate::R<CNDTR6rs>;
#[doc = "Register `CNDTR6` writer"]
pub type W = crate::W<CNDTR6rs>;
#[doc = "Field `NDT` reader - Number of data to transfer"]
pub type NDT_R = crate::FieldReader<u16>;
#[doc = "Field `NDT` writer - Number of data to transfer"]
pub type NDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    #[must_use]
    pub fn ndt(&mut self) -> NDT_W<CNDTR6rs> {
        NDT_W::new(self, 0)
    }
}
#[doc = "channel x number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNDTR6rs;
impl crate::RegisterSpec for CNDTR6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cndtr6::R`](R) reader structure"]
impl crate::Readable for CNDTR6rs {}
#[doc = "`write(|w| ..)` method takes [`cndtr6::W`](W) writer structure"]
impl crate::Writable for CNDTR6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNDTR6 to value 0"]
impl crate::Resettable for CNDTR6rs {
    const RESET_VALUE: u32 = 0;
}
