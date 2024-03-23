#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Field `CRXBFF` writer - Clear receive buffer full flag"]
pub type CRXBFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTXBEF` writer - Clear transmit buffer empty flag"]
pub type CTXBEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRXBERF` writer - Clear receive CRC error flag"]
pub type CRXBERF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRXOVRF` writer - Clear receive overrun error flag"]
pub type CRXOVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTXUNRF` writer - Clear transmit underrun error flag"]
pub type CTXUNRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCF` writer - Clear transfer complete flag"]
pub type CTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSRF` writer - Clear slave resume flag"]
pub type CSRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRDYF` writer - Clear transceiver ready flag"]
pub type CRDYF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear receive buffer full flag"]
    #[inline(always)]
    #[must_use]
    pub fn crxbff(&mut self) -> CRXBFF_W<ICRrs> {
        CRXBFF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear transmit buffer empty flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctxbef(&mut self) -> CTXBEF_W<ICRrs> {
        CTXBEF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear receive CRC error flag"]
    #[inline(always)]
    #[must_use]
    pub fn crxberf(&mut self) -> CRXBERF_W<ICRrs> {
        CRXBERF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear receive overrun error flag"]
    #[inline(always)]
    #[must_use]
    pub fn crxovrf(&mut self) -> CRXOVRF_W<ICRrs> {
        CRXOVRF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear transmit underrun error flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctxunrf(&mut self) -> CTXUNRF_W<ICRrs> {
        CTXUNRF_W::new(self, 4)
    }
    #[doc = "Bit 7 - Clear transfer complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctcf(&mut self) -> CTCF_W<ICRrs> {
        CTCF_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear slave resume flag"]
    #[inline(always)]
    #[must_use]
    pub fn csrf(&mut self) -> CSRF_W<ICRrs> {
        CSRF_W::new(self, 8)
    }
    #[doc = "Bit 11 - Clear transceiver ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn crdyf(&mut self) -> CRDYF_W<ICRrs> {
        CRDYF_W::new(self, 11)
    }
}
#[doc = "SWPMI Interrupt Flag Clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
