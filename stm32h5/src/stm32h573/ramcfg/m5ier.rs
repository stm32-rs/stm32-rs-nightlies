#[doc = "Register `M5IER` reader"]
pub type R = crate::R<M5IERrs>;
#[doc = "Register `M5IER` writer"]
pub type W = crate::W<M5IERrs>;
#[doc = "Field `SEIE` reader - ECC single error interrupt enable"]
pub type SEIE_R = crate::BitReader;
#[doc = "Field `SEIE` writer - ECC single error interrupt enable"]
pub type SEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEIE` reader - ECC double error interrupt enable"]
pub type DEIE_R = crate::BitReader;
#[doc = "Field `DEIE` writer - ECC double error interrupt enable"]
pub type DEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCNMI` reader - Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value."]
pub type ECCNMI_R = crate::BitReader;
#[doc = "Field `ECCNMI` writer - Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value."]
pub type ECCNMI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ECC single error interrupt enable"]
    #[inline(always)]
    pub fn seie(&self) -> SEIE_R {
        SEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECC double error interrupt enable"]
    #[inline(always)]
    pub fn deie(&self) -> DEIE_R {
        DEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value."]
    #[inline(always)]
    pub fn eccnmi(&self) -> ECCNMI_R {
        ECCNMI_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC single error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn seie(&mut self) -> SEIE_W<M5IERrs> {
        SEIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - ECC double error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn deie(&mut self) -> DEIE_W<M5IERrs> {
        DEIE_W::new(self, 1)
    }
    #[doc = "Bit 3 - Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value."]
    #[inline(always)]
    #[must_use]
    pub fn eccnmi(&mut self) -> ECCNMI_W<M5IERrs> {
        ECCNMI_W::new(self, 3)
    }
}
#[doc = "RAMCFG memory 5 interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m5ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M5IERrs;
impl crate::RegisterSpec for M5IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m5ier::R`](R) reader structure"]
impl crate::Readable for M5IERrs {}
#[doc = "`write(|w| ..)` method takes [`m5ier::W`](W) writer structure"]
impl crate::Writable for M5IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M5IER to value 0"]
impl crate::Resettable for M5IERrs {
    const RESET_VALUE: u32 = 0;
}
