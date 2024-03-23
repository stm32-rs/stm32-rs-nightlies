#[doc = "Register `OTG_GAHBCFG` reader"]
pub type R = crate::R<OTG_GAHBCFGrs>;
#[doc = "Register `OTG_GAHBCFG` writer"]
pub type W = crate::W<OTG_GAHBCFGrs>;
#[doc = "Field `GINTMSK` reader - GINTMSK"]
pub type GINTMSK_R = crate::BitReader;
#[doc = "Field `GINTMSK` writer - GINTMSK"]
pub type GINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBSTLEN` reader - HBSTLEN"]
pub type HBSTLEN_R = crate::FieldReader;
#[doc = "Field `HBSTLEN` writer - HBSTLEN"]
pub type HBSTLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMAEN` reader - DMAEN"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMAEN"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFELVL` reader - TXFELVL"]
pub type TXFELVL_R = crate::BitReader;
#[doc = "Field `TXFELVL` writer - TXFELVL"]
pub type TXFELVL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTXFELVL` reader - PTXFELVL"]
pub type PTXFELVL_R = crate::BitReader;
#[doc = "Field `PTXFELVL` writer - PTXFELVL"]
pub type PTXFELVL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GINTMSK"]
    #[inline(always)]
    pub fn gintmsk(&self) -> GINTMSK_R {
        GINTMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - HBSTLEN"]
    #[inline(always)]
    pub fn hbstlen(&self) -> HBSTLEN_R {
        HBSTLEN_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - TXFELVL"]
    #[inline(always)]
    pub fn txfelvl(&self) -> TXFELVL_R {
        TXFELVL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PTXFELVL"]
    #[inline(always)]
    pub fn ptxfelvl(&self) -> PTXFELVL_R {
        PTXFELVL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GINTMSK"]
    #[inline(always)]
    #[must_use]
    pub fn gintmsk(&mut self) -> GINTMSK_W<OTG_GAHBCFGrs> {
        GINTMSK_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - HBSTLEN"]
    #[inline(always)]
    #[must_use]
    pub fn hbstlen(&mut self) -> HBSTLEN_W<OTG_GAHBCFGrs> {
        HBSTLEN_W::new(self, 1)
    }
    #[doc = "Bit 5 - DMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<OTG_GAHBCFGrs> {
        DMAEN_W::new(self, 5)
    }
    #[doc = "Bit 7 - TXFELVL"]
    #[inline(always)]
    #[must_use]
    pub fn txfelvl(&mut self) -> TXFELVL_W<OTG_GAHBCFGrs> {
        TXFELVL_W::new(self, 7)
    }
    #[doc = "Bit 8 - PTXFELVL"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfelvl(&mut self) -> PTXFELVL_W<OTG_GAHBCFGrs> {
        PTXFELVL_W::new(self, 8)
    }
}
#[doc = "This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_gahbcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_gahbcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_GAHBCFGrs;
impl crate::RegisterSpec for OTG_GAHBCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_gahbcfg::R`](R) reader structure"]
impl crate::Readable for OTG_GAHBCFGrs {}
#[doc = "`write(|w| ..)` method takes [`otg_gahbcfg::W`](W) writer structure"]
impl crate::Writable for OTG_GAHBCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_GAHBCFG to value 0"]
impl crate::Resettable for OTG_GAHBCFGrs {
    const RESET_VALUE: u32 = 0;
}
