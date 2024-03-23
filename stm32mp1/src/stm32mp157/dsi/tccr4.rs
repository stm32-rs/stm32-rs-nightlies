#[doc = "Register `TCCR4` reader"]
pub type R = crate::R<TCCR4rs>;
#[doc = "Register `TCCR4` writer"]
pub type W = crate::W<TCCR4rs>;
#[doc = "Field `LPWR_TOCNT` reader - LPWR_TOCNT"]
pub type LPWR_TOCNT_R = crate::FieldReader<u16>;
#[doc = "Field `LPWR_TOCNT` writer - LPWR_TOCNT"]
pub type LPWR_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - LPWR_TOCNT"]
    #[inline(always)]
    pub fn lpwr_tocnt(&self) -> LPWR_TOCNT_R {
        LPWR_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - LPWR_TOCNT"]
    #[inline(always)]
    #[must_use]
    pub fn lpwr_tocnt(&mut self) -> LPWR_TOCNT_W<TCCR4rs> {
        LPWR_TOCNT_W::new(self, 0)
    }
}
#[doc = "DSI Host timeout counter configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR4rs;
impl crate::RegisterSpec for TCCR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tccr4::R`](R) reader structure"]
impl crate::Readable for TCCR4rs {}
#[doc = "`write(|w| ..)` method takes [`tccr4::W`](W) writer structure"]
impl crate::Writable for TCCR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCCR4 to value 0"]
impl crate::Resettable for TCCR4rs {
    const RESET_VALUE: u32 = 0;
}
