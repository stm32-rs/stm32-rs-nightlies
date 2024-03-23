#[doc = "Register `MTLQICSR` reader"]
pub type R = crate::R<MTLQICSRrs>;
#[doc = "Register `MTLQICSR` writer"]
pub type W = crate::W<MTLQICSRrs>;
#[doc = "Field `TXUNFIS` reader - Transmit Queue Underflow Interrupt Status"]
pub type TXUNFIS_R = crate::BitReader;
#[doc = "Field `TXUNFIS` writer - Transmit Queue Underflow Interrupt Status"]
pub type TXUNFIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUIE` reader - Transmit Queue Underflow Interrupt Enable"]
pub type TXUIE_R = crate::BitReader;
#[doc = "Field `TXUIE` writer - Transmit Queue Underflow Interrupt Enable"]
pub type TXUIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVFIS` reader - Receive Queue Overflow Interrupt Status"]
pub type RXOVFIS_R = crate::BitReader;
#[doc = "Field `RXOVFIS` writer - Receive Queue Overflow Interrupt Status"]
pub type RXOVFIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOIE` reader - Receive Queue Overflow Interrupt Enable"]
pub type RXOIE_R = crate::BitReader;
#[doc = "Field `RXOIE` writer - Receive Queue Overflow Interrupt Enable"]
pub type RXOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit Queue Underflow Interrupt Status"]
    #[inline(always)]
    pub fn txunfis(&self) -> TXUNFIS_R {
        TXUNFIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Queue Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn txuie(&self) -> TXUIE_R {
        TXUIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Receive Queue Overflow Interrupt Status"]
    #[inline(always)]
    pub fn rxovfis(&self) -> RXOVFIS_R {
        RXOVFIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Receive Queue Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Queue Underflow Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn txunfis(&mut self) -> TXUNFIS_W<MTLQICSRrs> {
        TXUNFIS_W::new(self, 0)
    }
    #[doc = "Bit 8 - Transmit Queue Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txuie(&mut self) -> TXUIE_W<MTLQICSRrs> {
        TXUIE_W::new(self, 8)
    }
    #[doc = "Bit 16 - Receive Queue Overflow Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn rxovfis(&mut self) -> RXOVFIS_W<MTLQICSRrs> {
        RXOVFIS_W::new(self, 16)
    }
    #[doc = "Bit 24 - Receive Queue Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxoie(&mut self) -> RXOIE_W<MTLQICSRrs> {
        RXOIE_W::new(self, 24)
    }
}
#[doc = "Queue interrupt control status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtlqicsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtlqicsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTLQICSRrs;
impl crate::RegisterSpec for MTLQICSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtlqicsr::R`](R) reader structure"]
impl crate::Readable for MTLQICSRrs {}
#[doc = "`write(|w| ..)` method takes [`mtlqicsr::W`](W) writer structure"]
impl crate::Writable for MTLQICSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTLQICSR to value 0"]
impl crate::Resettable for MTLQICSRrs {
    const RESET_VALUE: u32 = 0;
}
