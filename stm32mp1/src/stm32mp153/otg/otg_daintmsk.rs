#[doc = "Register `OTG_DAINTMSK` reader"]
pub type R = crate::R<OTG_DAINTMSKrs>;
#[doc = "Register `OTG_DAINTMSK` writer"]
pub type W = crate::W<OTG_DAINTMSKrs>;
#[doc = "Field `IEPM` reader - IEPM"]
pub type IEPM_R = crate::FieldReader<u16>;
#[doc = "Field `IEPM` writer - IEPM"]
pub type IEPM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `OEPM` reader - OEPM"]
pub type OEPM_R = crate::FieldReader<u16>;
#[doc = "Field `OEPM` writer - OEPM"]
pub type OEPM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IEPM"]
    #[inline(always)]
    pub fn iepm(&self) -> IEPM_R {
        IEPM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OEPM"]
    #[inline(always)]
    pub fn oepm(&self) -> OEPM_R {
        OEPM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IEPM"]
    #[inline(always)]
    #[must_use]
    pub fn iepm(&mut self) -> IEPM_W<OTG_DAINTMSKrs> {
        IEPM_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - OEPM"]
    #[inline(always)]
    #[must_use]
    pub fn oepm(&mut self) -> OEPM_W<OTG_DAINTMSKrs> {
        OEPM_W::new(self, 16)
    }
}
#[doc = "The OTG_DAINTMSK register works with the device endpoint interrupt register to interrupt the application when an event occurs on a device endpoint. However, the OTG_DAINT register bit corresponding to that interrupt is still set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_daintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_daintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_DAINTMSKrs;
impl crate::RegisterSpec for OTG_DAINTMSKrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_daintmsk::R`](R) reader structure"]
impl crate::Readable for OTG_DAINTMSKrs {}
#[doc = "`write(|w| ..)` method takes [`otg_daintmsk::W`](W) writer structure"]
impl crate::Writable for OTG_DAINTMSKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_DAINTMSK to value 0"]
impl crate::Resettable for OTG_DAINTMSKrs {
    const RESET_VALUE: u32 = 0;
}
