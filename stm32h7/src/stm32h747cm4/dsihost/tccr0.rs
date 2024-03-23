#[doc = "Register `TCCR0` reader"]
pub type R = crate::R<TCCR0rs>;
#[doc = "Register `TCCR0` writer"]
pub type W = crate::W<TCCR0rs>;
#[doc = "Field `LPRX_TOCNT` reader - Low-power reception timeout counter"]
pub type LPRX_TOCNT_R = crate::FieldReader<u16>;
#[doc = "Field `LPRX_TOCNT` writer - Low-power reception timeout counter"]
pub type LPRX_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `HSTX_TOCNT` reader - High-speed transmission timeout counter"]
pub type HSTX_TOCNT_R = crate::FieldReader<u16>;
#[doc = "Field `HSTX_TOCNT` writer - High-speed transmission timeout counter"]
pub type HSTX_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Low-power reception timeout counter"]
    #[inline(always)]
    pub fn lprx_tocnt(&self) -> LPRX_TOCNT_R {
        LPRX_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High-speed transmission timeout counter"]
    #[inline(always)]
    pub fn hstx_tocnt(&self) -> HSTX_TOCNT_R {
        HSTX_TOCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low-power reception timeout counter"]
    #[inline(always)]
    #[must_use]
    pub fn lprx_tocnt(&mut self) -> LPRX_TOCNT_W<TCCR0rs> {
        LPRX_TOCNT_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - High-speed transmission timeout counter"]
    #[inline(always)]
    #[must_use]
    pub fn hstx_tocnt(&mut self) -> HSTX_TOCNT_W<TCCR0rs> {
        HSTX_TOCNT_W::new(self, 16)
    }
}
#[doc = "DSI Host timeout counter configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR0rs;
impl crate::RegisterSpec for TCCR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tccr0::R`](R) reader structure"]
impl crate::Readable for TCCR0rs {}
#[doc = "`write(|w| ..)` method takes [`tccr0::W`](W) writer structure"]
impl crate::Writable for TCCR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCCR0 to value 0"]
impl crate::Resettable for TCCR0rs {
    const RESET_VALUE: u32 = 0;
}
