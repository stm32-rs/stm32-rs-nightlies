#[doc = "Register `OTG_DIEPEMPMSK` reader"]
pub type R = crate::R<OTG_DIEPEMPMSKrs>;
#[doc = "Register `OTG_DIEPEMPMSK` writer"]
pub type W = crate::W<OTG_DIEPEMPMSKrs>;
#[doc = "Field `INEPTXFEM` reader - INEPTXFEM"]
pub type INEPTXFEM_R = crate::FieldReader<u16>;
#[doc = "Field `INEPTXFEM` writer - INEPTXFEM"]
pub type INEPTXFEM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - INEPTXFEM"]
    #[inline(always)]
    pub fn ineptxfem(&self) -> INEPTXFEM_R {
        INEPTXFEM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - INEPTXFEM"]
    #[inline(always)]
    #[must_use]
    pub fn ineptxfem(&mut self) -> INEPTXFEM_W<OTG_DIEPEMPMSKrs> {
        INEPTXFEM_W::new(self, 0)
    }
}
#[doc = "This register is used to control the IN endpoint FIFO empty interrupt generation (TXFE_OTG_DIEPINTx).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepempmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepempmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_DIEPEMPMSKrs;
impl crate::RegisterSpec for OTG_DIEPEMPMSKrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_diepempmsk::R`](R) reader structure"]
impl crate::Readable for OTG_DIEPEMPMSKrs {}
#[doc = "`write(|w| ..)` method takes [`otg_diepempmsk::W`](W) writer structure"]
impl crate::Writable for OTG_DIEPEMPMSKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_DIEPEMPMSK to value 0"]
impl crate::Resettable for OTG_DIEPEMPMSKrs {
    const RESET_VALUE: u32 = 0;
}
