#[doc = "Register `CNDTR4` reader"]
pub type R = crate::R<CNDTR4rs>;
#[doc = "Register `CNDTR4` writer"]
pub type W = crate::W<CNDTR4rs>;
#[doc = "Field `NDT` reader - number of data to transfer (0 to 218 - 1)"]
pub type NDT_R = crate::FieldReader<u32>;
#[doc = "Field `NDT` writer - number of data to transfer (0 to 218 - 1)"]
pub type NDT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - number of data to transfer (0 to 218 - 1)"]
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - number of data to transfer (0 to 218 - 1)"]
    #[inline(always)]
    #[must_use]
    pub fn ndt(&mut self) -> NDT_W<CNDTR4rs> {
        NDT_W::new(self, 0)
    }
}
#[doc = "channel x number of data to transfer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNDTR4rs;
impl crate::RegisterSpec for CNDTR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cndtr4::R`](R) reader structure"]
impl crate::Readable for CNDTR4rs {}
#[doc = "`write(|w| ..)` method takes [`cndtr4::W`](W) writer structure"]
impl crate::Writable for CNDTR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNDTR4 to value 0"]
impl crate::Resettable for CNDTR4rs {
    const RESET_VALUE: u32 = 0;
}
