///Register `OTG_HS_DIEPEMPMSK` reader
pub type R = crate::R<OTG_HS_DIEPEMPMSKrs>;
///Register `OTG_HS_DIEPEMPMSK` writer
pub type W = crate::W<OTG_HS_DIEPEMPMSKrs>;
///Field `INEPTXFEM` reader - IN EP Tx FIFO empty interrupt mask bits
pub type INEPTXFEM_R = crate::FieldReader<u16>;
///Field `INEPTXFEM` writer - IN EP Tx FIFO empty interrupt mask bits
pub type INEPTXFEM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - IN EP Tx FIFO empty interrupt mask bits
    #[inline(always)]
    pub fn ineptxfem(&self) -> INEPTXFEM_R {
        INEPTXFEM_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_HS_DIEPEMPMSK")
            .field("ineptxfem", &self.ineptxfem())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - IN EP Tx FIFO empty interrupt mask bits
    #[inline(always)]
    pub fn ineptxfem(&mut self) -> INEPTXFEM_W<'_, OTG_HS_DIEPEMPMSKrs> {
        INEPTXFEM_W::new(self, 0)
    }
}
/**OTG_HS device IN endpoint FIFO empty interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepempmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepempmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_HS_DEVICE:OTG_HS_DIEPEMPMSK)*/
pub struct OTG_HS_DIEPEMPMSKrs;
impl crate::RegisterSpec for OTG_HS_DIEPEMPMSKrs {
    type Ux = u32;
}
///`read()` method returns [`otg_hs_diepempmsk::R`](R) reader structure
impl crate::Readable for OTG_HS_DIEPEMPMSKrs {}
///`write(|w| ..)` method takes [`otg_hs_diepempmsk::W`](W) writer structure
impl crate::Writable for OTG_HS_DIEPEMPMSKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTG_HS_DIEPEMPMSK to value 0
impl crate::Resettable for OTG_HS_DIEPEMPMSKrs {}
