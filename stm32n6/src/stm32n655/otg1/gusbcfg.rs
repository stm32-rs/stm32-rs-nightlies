///Register `GUSBCFG` reader
pub type R = crate::R<GUSBCFGrs>;
///Register `GUSBCFG` writer
pub type W = crate::W<GUSBCFGrs>;
///Field `TOCAL` reader - FS timeout calibration
pub type TOCAL_R = crate::FieldReader;
///Field `TOCAL` writer - FS timeout calibration
pub type TOCAL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TRDT` reader - USB turnaround time
pub type TRDT_R = crate::FieldReader;
///Field `TRDT` writer - USB turnaround time
pub type TRDT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PHYLPC` reader - PHY Low-power clock select
pub type PHYLPC_R = crate::BitReader;
///Field `PHYLPC` writer - PHY Low-power clock select
pub type PHYLPC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSDPS` reader - TermSel DLine pulsing selection
pub type TSDPS_R = crate::BitReader;
///Field `TSDPS` writer - TermSel DLine pulsing selection
pub type TSDPS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FHMOD` reader - Force host mode
pub type FHMOD_R = crate::BitReader;
///Field `FHMOD` writer - Force host mode
pub type FHMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDMOD` reader - Force device mode
pub type FDMOD_R = crate::BitReader;
///Field `FDMOD` writer - Force device mode
pub type FDMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - FS timeout calibration
    #[inline(always)]
    pub fn tocal(&self) -> TOCAL_R {
        TOCAL_R::new((self.bits & 7) as u8)
    }
    ///Bits 10:13 - USB turnaround time
    #[inline(always)]
    pub fn trdt(&self) -> TRDT_R {
        TRDT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    ///Bit 15 - PHY Low-power clock select
    #[inline(always)]
    pub fn phylpc(&self) -> PHYLPC_R {
        PHYLPC_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 22 - TermSel DLine pulsing selection
    #[inline(always)]
    pub fn tsdps(&self) -> TSDPS_R {
        TSDPS_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 29 - Force host mode
    #[inline(always)]
    pub fn fhmod(&self) -> FHMOD_R {
        FHMOD_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Force device mode
    #[inline(always)]
    pub fn fdmod(&self) -> FDMOD_R {
        FDMOD_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GUSBCFG")
            .field("tocal", &self.tocal())
            .field("trdt", &self.trdt())
            .field("phylpc", &self.phylpc())
            .field("tsdps", &self.tsdps())
            .field("fhmod", &self.fhmod())
            .field("fdmod", &self.fdmod())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - FS timeout calibration
    #[inline(always)]
    pub fn tocal(&mut self) -> TOCAL_W<'_, GUSBCFGrs> {
        TOCAL_W::new(self, 0)
    }
    ///Bits 10:13 - USB turnaround time
    #[inline(always)]
    pub fn trdt(&mut self) -> TRDT_W<'_, GUSBCFGrs> {
        TRDT_W::new(self, 10)
    }
    ///Bit 15 - PHY Low-power clock select
    #[inline(always)]
    pub fn phylpc(&mut self) -> PHYLPC_W<'_, GUSBCFGrs> {
        PHYLPC_W::new(self, 15)
    }
    ///Bit 22 - TermSel DLine pulsing selection
    #[inline(always)]
    pub fn tsdps(&mut self) -> TSDPS_W<'_, GUSBCFGrs> {
        TSDPS_W::new(self, 22)
    }
    ///Bit 29 - Force host mode
    #[inline(always)]
    pub fn fhmod(&mut self) -> FHMOD_W<'_, GUSBCFGrs> {
        FHMOD_W::new(self, 29)
    }
    ///Bit 30 - Force device mode
    #[inline(always)]
    pub fn fdmod(&mut self) -> FDMOD_W<'_, GUSBCFGrs> {
        FDMOD_W::new(self, 30)
    }
}
/**OTG USB configuration register

You can [`read`](crate::Reg::read) this register and get [`gusbcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gusbcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#OTG1:GUSBCFG)*/
pub struct GUSBCFGrs;
impl crate::RegisterSpec for GUSBCFGrs {
    type Ux = u32;
}
///`read()` method returns [`gusbcfg::R`](R) reader structure
impl crate::Readable for GUSBCFGrs {}
///`write(|w| ..)` method takes [`gusbcfg::W`](W) writer structure
impl crate::Writable for GUSBCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GUSBCFG to value 0x1400
impl crate::Resettable for GUSBCFGrs {
    const RESET_VALUE: u32 = 0x1400;
}
