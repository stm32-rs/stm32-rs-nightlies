#[doc = "Register `S3M0AR` reader"]
pub type R = crate::R<S3M0ARrs>;
#[doc = "Register `S3M0AR` writer"]
pub type W = crate::W<S3M0ARrs>;
#[doc = "Field `M0A` reader - Memory 0 address"]
pub type M0A_R = crate::FieldReader<u32>;
#[doc = "Field `M0A` writer - Memory 0 address"]
pub type M0A_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory 0 address"]
    #[inline(always)]
    pub fn m0a(&self) -> M0A_R {
        M0A_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 0 address"]
    #[inline(always)]
    #[must_use]
    pub fn m0a(&mut self) -> M0A_W<S3M0ARrs> {
        M0A_W::new(self, 0)
    }
}
#[doc = "stream x memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s3m0ar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s3m0ar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S3M0ARrs;
impl crate::RegisterSpec for S3M0ARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s3m0ar::R`](R) reader structure"]
impl crate::Readable for S3M0ARrs {}
#[doc = "`write(|w| ..)` method takes [`s3m0ar::W`](W) writer structure"]
impl crate::Writable for S3M0ARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S3M0AR to value 0"]
impl crate::Resettable for S3M0ARrs {
    const RESET_VALUE: u32 = 0;
}
