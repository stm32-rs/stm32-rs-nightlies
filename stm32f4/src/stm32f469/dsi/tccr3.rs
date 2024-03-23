#[doc = "Register `TCCR3` reader"]
pub type R = crate::R<TCCR3rs>;
#[doc = "Register `TCCR3` writer"]
pub type W = crate::W<TCCR3rs>;
#[doc = "Field `LPRD_TOCNT` reader - Low-Power Read Timeout Counter"]
pub type LPRD_TOCNT_R = crate::FieldReader<u16>;
#[doc = "Field `LPRD_TOCNT` writer - Low-Power Read Timeout Counter"]
pub type LPRD_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Low-Power Read Timeout Counter"]
    #[inline(always)]
    pub fn lprd_tocnt(&self) -> LPRD_TOCNT_R {
        LPRD_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low-Power Read Timeout Counter"]
    #[inline(always)]
    #[must_use]
    pub fn lprd_tocnt(&mut self) -> LPRD_TOCNT_W<TCCR3rs> {
        LPRD_TOCNT_W::new(self, 0)
    }
}
#[doc = "DSI Host Timeout Counter Configuration Register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR3rs;
impl crate::RegisterSpec for TCCR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tccr3::R`](R) reader structure"]
impl crate::Readable for TCCR3rs {}
#[doc = "`write(|w| ..)` method takes [`tccr3::W`](W) writer structure"]
impl crate::Writable for TCCR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCCR3 to value 0"]
impl crate::Resettable for TCCR3rs {
    const RESET_VALUE: u32 = 0;
}
