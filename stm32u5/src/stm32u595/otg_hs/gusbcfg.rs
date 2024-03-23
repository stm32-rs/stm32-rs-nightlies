#[doc = "Register `GUSBCFG` reader"]
pub type R = crate::R<GUSBCFGrs>;
#[doc = "Register `GUSBCFG` writer"]
pub type W = crate::W<GUSBCFGrs>;
#[doc = "Field `TOCAL` reader - TOCAL"]
pub type TOCAL_R = crate::FieldReader;
#[doc = "Field `TOCAL` writer - TOCAL"]
pub type TOCAL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHYSEL` reader - PHYSEL"]
pub type PHYSEL_R = crate::BitReader;
#[doc = "Field `SRPCAP` reader - SRPCAP"]
pub type SRPCAP_R = crate::BitReader;
#[doc = "Field `SRPCAP` writer - SRPCAP"]
pub type SRPCAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNPCAP` reader - HNPCAP"]
pub type HNPCAP_R = crate::BitReader;
#[doc = "Field `HNPCAP` writer - HNPCAP"]
pub type HNPCAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRDT` reader - TRDT"]
pub type TRDT_R = crate::FieldReader;
#[doc = "Field `TRDT` writer - TRDT"]
pub type TRDT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHYLPC` reader - PHYLPC"]
pub type PHYLPC_R = crate::BitReader;
#[doc = "Field `PHYLPC` writer - PHYLPC"]
pub type PHYLPC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSDPS` reader - TSDPS"]
pub type TSDPS_R = crate::BitReader;
#[doc = "Field `TSDPS` writer - TSDPS"]
pub type TSDPS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FHMOD` reader - FHMOD"]
pub type FHMOD_R = crate::BitReader;
#[doc = "Field `FHMOD` writer - FHMOD"]
pub type FHMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDMOD` reader - FDMOD"]
pub type FDMOD_R = crate::BitReader;
#[doc = "Field `FDMOD` writer - FDMOD"]
pub type FDMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - TOCAL"]
    #[inline(always)]
    pub fn tocal(&self) -> TOCAL_R {
        TOCAL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - PHYSEL"]
    #[inline(always)]
    pub fn physel(&self) -> PHYSEL_R {
        PHYSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - SRPCAP"]
    #[inline(always)]
    pub fn srpcap(&self) -> SRPCAP_R {
        SRPCAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNPCAP"]
    #[inline(always)]
    pub fn hnpcap(&self) -> HNPCAP_R {
        HNPCAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - TRDT"]
    #[inline(always)]
    pub fn trdt(&self) -> TRDT_R {
        TRDT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - PHYLPC"]
    #[inline(always)]
    pub fn phylpc(&self) -> PHYLPC_R {
        PHYLPC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 22 - TSDPS"]
    #[inline(always)]
    pub fn tsdps(&self) -> TSDPS_R {
        TSDPS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 29 - FHMOD"]
    #[inline(always)]
    pub fn fhmod(&self) -> FHMOD_R {
        FHMOD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - FDMOD"]
    #[inline(always)]
    pub fn fdmod(&self) -> FDMOD_R {
        FDMOD_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - TOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn tocal(&mut self) -> TOCAL_W<GUSBCFGrs> {
        TOCAL_W::new(self, 0)
    }
    #[doc = "Bit 8 - SRPCAP"]
    #[inline(always)]
    #[must_use]
    pub fn srpcap(&mut self) -> SRPCAP_W<GUSBCFGrs> {
        SRPCAP_W::new(self, 8)
    }
    #[doc = "Bit 9 - HNPCAP"]
    #[inline(always)]
    #[must_use]
    pub fn hnpcap(&mut self) -> HNPCAP_W<GUSBCFGrs> {
        HNPCAP_W::new(self, 9)
    }
    #[doc = "Bits 10:13 - TRDT"]
    #[inline(always)]
    #[must_use]
    pub fn trdt(&mut self) -> TRDT_W<GUSBCFGrs> {
        TRDT_W::new(self, 10)
    }
    #[doc = "Bit 15 - PHYLPC"]
    #[inline(always)]
    #[must_use]
    pub fn phylpc(&mut self) -> PHYLPC_W<GUSBCFGrs> {
        PHYLPC_W::new(self, 15)
    }
    #[doc = "Bit 22 - TSDPS"]
    #[inline(always)]
    #[must_use]
    pub fn tsdps(&mut self) -> TSDPS_W<GUSBCFGrs> {
        TSDPS_W::new(self, 22)
    }
    #[doc = "Bit 29 - FHMOD"]
    #[inline(always)]
    #[must_use]
    pub fn fhmod(&mut self) -> FHMOD_W<GUSBCFGrs> {
        FHMOD_W::new(self, 29)
    }
    #[doc = "Bit 30 - FDMOD"]
    #[inline(always)]
    #[must_use]
    pub fn fdmod(&mut self) -> FDMOD_W<GUSBCFGrs> {
        FDMOD_W::new(self, 30)
    }
}
#[doc = "This register can be used to configure the core after power-on or a changing to host mode or device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. Do not make changes to this register after the initial programming.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusbcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GUSBCFGrs;
impl crate::RegisterSpec for GUSBCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gusbcfg::R`](R) reader structure"]
impl crate::Readable for GUSBCFGrs {}
#[doc = "`write(|w| ..)` method takes [`gusbcfg::W`](W) writer structure"]
impl crate::Writable for GUSBCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GUSBCFG to value 0x1400"]
impl crate::Resettable for GUSBCFGrs {
    const RESET_VALUE: u32 = 0x1400;
}
