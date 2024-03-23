#[doc = "Register `TCCR5` reader"]
pub type R = crate::R<TCCR5rs>;
#[doc = "Register `TCCR5` writer"]
pub type W = crate::W<TCCR5rs>;
#[doc = "Field `LSWR_TOCNT` reader - Low-Power Write Timeout Counter"]
pub type LSWR_TOCNT_R = crate::FieldReader<u16>;
#[doc = "Field `LSWR_TOCNT` writer - Low-Power Write Timeout Counter"]
pub type LSWR_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Low-Power Write Timeout Counter"]
    #[inline(always)]
    pub fn lswr_tocnt(&self) -> LSWR_TOCNT_R {
        LSWR_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low-Power Write Timeout Counter"]
    #[inline(always)]
    #[must_use]
    pub fn lswr_tocnt(&mut self) -> LSWR_TOCNT_W<TCCR5rs> {
        LSWR_TOCNT_W::new(self, 0)
    }
}
#[doc = "DSI Host Timeout Counter Configuration Register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR5rs;
impl crate::RegisterSpec for TCCR5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tccr5::R`](R) reader structure"]
impl crate::Readable for TCCR5rs {}
#[doc = "`write(|w| ..)` method takes [`tccr5::W`](W) writer structure"]
impl crate::Writable for TCCR5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCCR5 to value 0"]
impl crate::Resettable for TCCR5rs {
    const RESET_VALUE: u32 = 0;
}
