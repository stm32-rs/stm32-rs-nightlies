#[doc = "Register `CMP` reader"]
pub type R = crate::R<CMPrs>;
#[doc = "Register `CMP` writer"]
pub type W = crate::W<CMPrs>;
#[doc = "Field `CMP` reader - CMP"]
pub type CMP_R = crate::FieldReader<u16>;
#[doc = "Field `CMP` writer - CMP"]
pub type CMP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CMP"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CMP"]
    #[inline(always)]
    #[must_use]
    pub fn cmp(&mut self) -> CMP_W<CMPrs> {
        CMP_W::new(self, 0)
    }
}
#[doc = "compare register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPrs;
impl crate::RegisterSpec for CMPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp::R`](R) reader structure"]
impl crate::Readable for CMPrs {}
#[doc = "`write(|w| ..)` method takes [`cmp::W`](W) writer structure"]
impl crate::Writable for CMPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP to value 0"]
impl crate::Resettable for CMPrs {
    const RESET_VALUE: u32 = 0;
}
