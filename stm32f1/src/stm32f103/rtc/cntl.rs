#[doc = "Register `CNTL` reader"]
pub type R = crate::R<CNTLrs>;
#[doc = "Register `CNTL` writer"]
pub type W = crate::W<CNTLrs>;
#[doc = "Field `CNTL` reader - RTC counter register Low"]
pub type CNTL_R = crate::FieldReader<u16>;
#[doc = "Field `CNTL` writer - RTC counter register Low"]
pub type CNTL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC counter register Low"]
    #[inline(always)]
    pub fn cntl(&self) -> CNTL_R {
        CNTL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC counter register Low"]
    #[inline(always)]
    #[must_use]
    pub fn cntl(&mut self) -> CNTL_W<CNTLrs> {
        CNTL_W::new(self, 0)
    }
}
#[doc = "RTC Counter Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTLrs;
impl crate::RegisterSpec for CNTLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntl::R`](R) reader structure"]
impl crate::Readable for CNTLrs {}
#[doc = "`write(|w| ..)` method takes [`cntl::W`](W) writer structure"]
impl crate::Writable for CNTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNTL to value 0"]
impl crate::Resettable for CNTLrs {
    const RESET_VALUE: u32 = 0;
}
