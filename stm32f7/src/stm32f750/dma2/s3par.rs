#[doc = "Register `S3PAR` reader"]
pub type R = crate::R<S3PARrs>;
#[doc = "Register `S3PAR` writer"]
pub type W = crate::W<S3PARrs>;
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
    pub fn pa(&mut self) -> PA_W<S3PARrs> {
        PA_W::new(self, 0)
    }
}
#[doc = "stream x peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s3par::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s3par::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S3PARrs;
impl crate::RegisterSpec for S3PARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s3par::R`](R) reader structure"]
impl crate::Readable for S3PARrs {}
#[doc = "`write(|w| ..)` method takes [`s3par::W`](W) writer structure"]
impl crate::Writable for S3PARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S3PAR to value 0"]
impl crate::Resettable for S3PARrs {
    const RESET_VALUE: u32 = 0;
}
