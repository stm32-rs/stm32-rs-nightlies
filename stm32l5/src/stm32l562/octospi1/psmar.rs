#[doc = "Register `PSMAR` reader"]
pub type R = crate::R<PSMARrs>;
#[doc = "Register `PSMAR` writer"]
pub type W = crate::W<PSMARrs>;
#[doc = "Field `INTERVAL` reader - Polling interval"]
pub type INTERVAL_R = crate::FieldReader<u16>;
#[doc = "Field `INTERVAL` writer - Polling interval"]
pub type INTERVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Polling interval"]
    #[inline(always)]
    pub fn interval(&self) -> INTERVAL_R {
        INTERVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Polling interval"]
    #[inline(always)]
    #[must_use]
    pub fn interval(&mut self) -> INTERVAL_W<PSMARrs> {
        INTERVAL_W::new(self, 0)
    }
}
#[doc = "polling status match register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psmar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psmar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSMARrs;
impl crate::RegisterSpec for PSMARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psmar::R`](R) reader structure"]
impl crate::Readable for PSMARrs {}
#[doc = "`write(|w| ..)` method takes [`psmar::W`](W) writer structure"]
impl crate::Writable for PSMARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSMAR to value 0"]
impl crate::Resettable for PSMARrs {
    const RESET_VALUE: u32 = 0;
}
