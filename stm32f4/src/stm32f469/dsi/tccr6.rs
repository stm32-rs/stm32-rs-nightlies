#[doc = "Register `TCCR6` reader"]
pub type R = crate::R<TCCR6rs>;
#[doc = "Register `TCCR6` writer"]
pub type W = crate::W<TCCR6rs>;
#[doc = "Field `BTA_TOCNT` reader - Bus-Turn-Around Timeout Counter"]
pub type BTA_TOCNT_R = crate::FieldReader<u16>;
#[doc = "Field `BTA_TOCNT` writer - Bus-Turn-Around Timeout Counter"]
pub type BTA_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Bus-Turn-Around Timeout Counter"]
    #[inline(always)]
    pub fn bta_tocnt(&self) -> BTA_TOCNT_R {
        BTA_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bus-Turn-Around Timeout Counter"]
    #[inline(always)]
    #[must_use]
    pub fn bta_tocnt(&mut self) -> BTA_TOCNT_W<TCCR6rs> {
        BTA_TOCNT_W::new(self, 0)
    }
}
#[doc = "DSI Host Timeout Counter Configuration Register6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR6rs;
impl crate::RegisterSpec for TCCR6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tccr6::R`](R) reader structure"]
impl crate::Readable for TCCR6rs {}
#[doc = "`write(|w| ..)` method takes [`tccr6::W`](W) writer structure"]
impl crate::Writable for TCCR6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCCR6 to value 0"]
impl crate::Resettable for TCCR6rs {
    const RESET_VALUE: u32 = 0;
}
