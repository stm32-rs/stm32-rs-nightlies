#[doc = "Register `EVCR` reader"]
pub type R = crate::R<EVCRrs>;
#[doc = "Register `EVCR` writer"]
pub type W = crate::W<EVCRrs>;
#[doc = "Field `PIN` reader - Pin selection"]
pub type PIN_R = crate::FieldReader;
#[doc = "Field `PIN` writer - Pin selection"]
pub type PIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PORT` reader - Port selection"]
pub type PORT_R = crate::FieldReader;
#[doc = "Field `PORT` writer - Port selection"]
pub type PORT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EVOE` reader - Event Output Enable"]
pub type EVOE_R = crate::BitReader;
#[doc = "Field `EVOE` writer - Event Output Enable"]
pub type EVOE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Pin selection"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Port selection"]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Event Output Enable"]
    #[inline(always)]
    pub fn evoe(&self) -> EVOE_R {
        EVOE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pin selection"]
    #[inline(always)]
    #[must_use]
    pub fn pin(&mut self) -> PIN_W<EVCRrs> {
        PIN_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Port selection"]
    #[inline(always)]
    #[must_use]
    pub fn port(&mut self) -> PORT_W<EVCRrs> {
        PORT_W::new(self, 4)
    }
    #[doc = "Bit 7 - Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evoe(&mut self) -> EVOE_W<EVCRrs> {
        EVOE_W::new(self, 7)
    }
}
#[doc = "Event Control Register (AFIO_EVCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVCRrs;
impl crate::RegisterSpec for EVCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evcr::R`](R) reader structure"]
impl crate::Readable for EVCRrs {}
#[doc = "`write(|w| ..)` method takes [`evcr::W`](W) writer structure"]
impl crate::Writable for EVCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVCR to value 0"]
impl crate::Resettable for EVCRrs {
    const RESET_VALUE: u32 = 0;
}
