#[doc = "Register `OTG_HPRT` reader"]
pub type R = crate::R<OTG_HPRTrs>;
#[doc = "Register `OTG_HPRT` writer"]
pub type W = crate::W<OTG_HPRTrs>;
#[doc = "Field `PCSTS` reader - PCSTS"]
pub type PCSTS_R = crate::BitReader;
#[doc = "Field `PCDET` reader - PCDET"]
pub type PCDET_R = crate::BitReader;
#[doc = "Field `PCDET` writer - PCDET"]
pub type PCDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENA` reader - PENA"]
pub type PENA_R = crate::BitReader;
#[doc = "Field `PENA` writer - PENA"]
pub type PENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENCHNG` reader - PENCHNG"]
pub type PENCHNG_R = crate::BitReader;
#[doc = "Field `PENCHNG` writer - PENCHNG"]
pub type PENCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POCA` reader - POCA"]
pub type POCA_R = crate::BitReader;
#[doc = "Field `POCCHNG` reader - POCCHNG"]
pub type POCCHNG_R = crate::BitReader;
#[doc = "Field `POCCHNG` writer - POCCHNG"]
pub type POCCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRES` reader - PRES"]
pub type PRES_R = crate::BitReader;
#[doc = "Field `PRES` writer - PRES"]
pub type PRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSUSP` reader - PSUSP"]
pub type PSUSP_R = crate::BitReader;
#[doc = "Field `PSUSP` writer - PSUSP"]
pub type PSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRST` reader - PRST"]
pub type PRST_R = crate::BitReader;
#[doc = "Field `PRST` writer - PRST"]
pub type PRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLSTS` reader - PLSTS"]
pub type PLSTS_R = crate::FieldReader;
#[doc = "Field `PPWR` reader - PPWR"]
pub type PPWR_R = crate::BitReader;
#[doc = "Field `PPWR` writer - PPWR"]
pub type PPWR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTCTL` reader - PTCTL"]
pub type PTCTL_R = crate::FieldReader;
#[doc = "Field `PTCTL` writer - PTCTL"]
pub type PTCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PSPD` reader - PSPD"]
pub type PSPD_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - PCSTS"]
    #[inline(always)]
    pub fn pcsts(&self) -> PCSTS_R {
        PCSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCDET"]
    #[inline(always)]
    pub fn pcdet(&self) -> PCDET_R {
        PCDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PENA"]
    #[inline(always)]
    pub fn pena(&self) -> PENA_R {
        PENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PENCHNG"]
    #[inline(always)]
    pub fn penchng(&self) -> PENCHNG_R {
        PENCHNG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - POCA"]
    #[inline(always)]
    pub fn poca(&self) -> POCA_R {
        POCA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - POCCHNG"]
    #[inline(always)]
    pub fn pocchng(&self) -> POCCHNG_R {
        POCCHNG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PRES"]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PSUSP"]
    #[inline(always)]
    pub fn psusp(&self) -> PSUSP_R {
        PSUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PRST"]
    #[inline(always)]
    pub fn prst(&self) -> PRST_R {
        PRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - PLSTS"]
    #[inline(always)]
    pub fn plsts(&self) -> PLSTS_R {
        PLSTS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - PPWR"]
    #[inline(always)]
    pub fn ppwr(&self) -> PPWR_R {
        PPWR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - PTCTL"]
    #[inline(always)]
    pub fn ptctl(&self) -> PTCTL_R {
        PTCTL_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:18 - PSPD"]
    #[inline(always)]
    pub fn pspd(&self) -> PSPD_R {
        PSPD_R::new(((self.bits >> 17) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - PCDET"]
    #[inline(always)]
    #[must_use]
    pub fn pcdet(&mut self) -> PCDET_W<OTG_HPRTrs> {
        PCDET_W::new(self, 1)
    }
    #[doc = "Bit 2 - PENA"]
    #[inline(always)]
    #[must_use]
    pub fn pena(&mut self) -> PENA_W<OTG_HPRTrs> {
        PENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - PENCHNG"]
    #[inline(always)]
    #[must_use]
    pub fn penchng(&mut self) -> PENCHNG_W<OTG_HPRTrs> {
        PENCHNG_W::new(self, 3)
    }
    #[doc = "Bit 5 - POCCHNG"]
    #[inline(always)]
    #[must_use]
    pub fn pocchng(&mut self) -> POCCHNG_W<OTG_HPRTrs> {
        POCCHNG_W::new(self, 5)
    }
    #[doc = "Bit 6 - PRES"]
    #[inline(always)]
    #[must_use]
    pub fn pres(&mut self) -> PRES_W<OTG_HPRTrs> {
        PRES_W::new(self, 6)
    }
    #[doc = "Bit 7 - PSUSP"]
    #[inline(always)]
    #[must_use]
    pub fn psusp(&mut self) -> PSUSP_W<OTG_HPRTrs> {
        PSUSP_W::new(self, 7)
    }
    #[doc = "Bit 8 - PRST"]
    #[inline(always)]
    #[must_use]
    pub fn prst(&mut self) -> PRST_W<OTG_HPRTrs> {
        PRST_W::new(self, 8)
    }
    #[doc = "Bit 12 - PPWR"]
    #[inline(always)]
    #[must_use]
    pub fn ppwr(&mut self) -> PPWR_W<OTG_HPRTrs> {
        PPWR_W::new(self, 12)
    }
    #[doc = "Bits 13:16 - PTCTL"]
    #[inline(always)]
    #[must_use]
    pub fn ptctl(&mut self) -> PTCTL_W<OTG_HPRTrs> {
        PTCTL_W::new(self, 13)
    }
}
#[doc = "This register is available only in host mode. Currently, the OTG host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in Figure724. The rc_w1 bits in this register can trigger an interrupt to the application through the host port interrupt bit of the core interrupt register (HPRTINT bit in OTG_GINTSTS). On a port interrupt, the application must read this register and clear the bit that caused the interrupt. For the rc_w1 bits, the application must write a 1 to the bit to clear the interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hprt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hprt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HPRTrs;
impl crate::RegisterSpec for OTG_HPRTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hprt::R`](R) reader structure"]
impl crate::Readable for OTG_HPRTrs {}
#[doc = "`write(|w| ..)` method takes [`otg_hprt::W`](W) writer structure"]
impl crate::Writable for OTG_HPRTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_HPRT to value 0"]
impl crate::Resettable for OTG_HPRTrs {
    const RESET_VALUE: u32 = 0;
}
