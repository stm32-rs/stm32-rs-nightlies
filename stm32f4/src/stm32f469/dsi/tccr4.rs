#[doc = "Register `TCCR4` reader"]
pub type R = crate::R<TCCR4rs>;
#[doc = "Register `TCCR4` writer"]
pub type W = crate::W<TCCR4rs>;
#[doc = "Field `HSWR_TOCNT` reader - High-Speed Write Timeout Counter"]
pub type HSWR_TOCNT_R = crate::FieldReader<u16>;
#[doc = "Field `HSWR_TOCNT` writer - High-Speed Write Timeout Counter"]
pub type HSWR_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PM` reader - Presp Mode"]
pub type PM_R = crate::BitReader;
#[doc = "Field `PM` writer - Presp Mode"]
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - High-Speed Write Timeout Counter"]
    #[inline(always)]
    pub fn hswr_tocnt(&self) -> HSWR_TOCNT_R {
        HSWR_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 24 - Presp Mode"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - High-Speed Write Timeout Counter"]
    #[inline(always)]
    #[must_use]
    pub fn hswr_tocnt(&mut self) -> HSWR_TOCNT_W<TCCR4rs> {
        HSWR_TOCNT_W::new(self, 0)
    }
    #[doc = "Bit 24 - Presp Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<TCCR4rs> {
        PM_W::new(self, 24)
    }
}
#[doc = "DSI Host Timeout Counter Configuration Register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
