#[doc = "Register `EWCR` reader"]
pub type R = crate::R<EWCRrs>;
#[doc = "Register `EWCR` writer"]
pub type W = crate::W<EWCRrs>;
#[doc = "Field `EWIT` reader - Watchdog counter window value"]
pub type EWIT_R = crate::FieldReader<u16>;
#[doc = "Field `EWIT` writer - Watchdog counter window value"]
pub type EWIT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `EWIC` reader - Watchdog early interrupt acknowledge"]
pub type EWIC_R = crate::BitReader;
#[doc = "Field `EWIC` writer - Watchdog early interrupt acknowledge"]
pub type EWIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWIE` reader - Watchdog early interrupt enable"]
pub type EWIE_R = crate::BitReader;
#[doc = "Field `EWIE` writer - Watchdog early interrupt enable"]
pub type EWIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Watchdog counter window value"]
    #[inline(always)]
    pub fn ewit(&self) -> EWIT_R {
        EWIT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - Watchdog early interrupt acknowledge"]
    #[inline(always)]
    pub fn ewic(&self) -> EWIC_R {
        EWIC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Watchdog early interrupt enable"]
    #[inline(always)]
    pub fn ewie(&self) -> EWIE_R {
        EWIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog counter window value"]
    #[inline(always)]
    #[must_use]
    pub fn ewit(&mut self) -> EWIT_W<EWCRrs> {
        EWIT_W::new(self, 0)
    }
    #[doc = "Bit 14 - Watchdog early interrupt acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn ewic(&mut self) -> EWIC_W<EWCRrs> {
        EWIC_W::new(self, 14)
    }
    #[doc = "Bit 15 - Watchdog early interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ewie(&mut self) -> EWIE_W<EWCRrs> {
        EWIE_W::new(self, 15)
    }
}
#[doc = "IWDG early wakeup interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ewcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ewcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EWCRrs;
impl crate::RegisterSpec for EWCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ewcr::R`](R) reader structure"]
impl crate::Readable for EWCRrs {}
#[doc = "`write(|w| ..)` method takes [`ewcr::W`](W) writer structure"]
impl crate::Writable for EWCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EWCR to value 0"]
impl crate::Resettable for EWCRrs {
    const RESET_VALUE: u32 = 0;
}
