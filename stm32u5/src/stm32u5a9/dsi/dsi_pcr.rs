#[doc = "Register `DSI_PCR` reader"]
pub type R = crate::R<DSI_PCRrs>;
#[doc = "Register `DSI_PCR` writer"]
pub type W = crate::W<DSI_PCRrs>;
#[doc = "Field `ETTXE` reader - EoTp transmission enable This bit enables the EoTP transmission."]
pub type ETTXE_R = crate::BitReader;
#[doc = "Field `ETTXE` writer - EoTp transmission enable This bit enables the EoTP transmission."]
pub type ETTXE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETRXE` reader - EoTp reception enable This bit enables the EoTp reception."]
pub type ETRXE_R = crate::BitReader;
#[doc = "Field `ETRXE` writer - EoTp reception enable This bit enables the EoTp reception."]
pub type ETRXE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTAE` reader - Bus-turn-around enable This bit enables the bus-turn-around (BTA) request."]
pub type BTAE_R = crate::BitReader;
#[doc = "Field `BTAE` writer - Bus-turn-around enable This bit enables the bus-turn-around (BTA) request."]
pub type BTAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCRXE` reader - ECC reception enable This bit enables the ECC reception, error correction and reporting."]
pub type ECCRXE_R = crate::BitReader;
#[doc = "Field `ECCRXE` writer - ECC reception enable This bit enables the ECC reception, error correction and reporting."]
pub type ECCRXE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCRXE` reader - CRC reception enable This bit enables the CRC reception and error reporting."]
pub type CRCRXE_R = crate::BitReader;
#[doc = "Field `CRCRXE` writer - CRC reception enable This bit enables the CRC reception and error reporting."]
pub type CRCRXE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETTXLPE` reader - EoTp transmission in low-power enable This bit enables the EoTP transmission in low-power."]
pub type ETTXLPE_R = crate::BitReader;
#[doc = "Field `ETTXLPE` writer - EoTp transmission in low-power enable This bit enables the EoTP transmission in low-power."]
pub type ETTXLPE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EoTp transmission enable This bit enables the EoTP transmission."]
    #[inline(always)]
    pub fn ettxe(&self) -> ETTXE_R {
        ETTXE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EoTp reception enable This bit enables the EoTp reception."]
    #[inline(always)]
    pub fn etrxe(&self) -> ETRXE_R {
        ETRXE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bus-turn-around enable This bit enables the bus-turn-around (BTA) request."]
    #[inline(always)]
    pub fn btae(&self) -> BTAE_R {
        BTAE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECC reception enable This bit enables the ECC reception, error correction and reporting."]
    #[inline(always)]
    pub fn eccrxe(&self) -> ECCRXE_R {
        ECCRXE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC reception enable This bit enables the CRC reception and error reporting."]
    #[inline(always)]
    pub fn crcrxe(&self) -> CRCRXE_R {
        CRCRXE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EoTp transmission in low-power enable This bit enables the EoTP transmission in low-power."]
    #[inline(always)]
    pub fn ettxlpe(&self) -> ETTXLPE_R {
        ETTXLPE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EoTp transmission enable This bit enables the EoTP transmission."]
    #[inline(always)]
    #[must_use]
    pub fn ettxe(&mut self) -> ETTXE_W<DSI_PCRrs> {
        ETTXE_W::new(self, 0)
    }
    #[doc = "Bit 1 - EoTp reception enable This bit enables the EoTp reception."]
    #[inline(always)]
    #[must_use]
    pub fn etrxe(&mut self) -> ETRXE_W<DSI_PCRrs> {
        ETRXE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bus-turn-around enable This bit enables the bus-turn-around (BTA) request."]
    #[inline(always)]
    #[must_use]
    pub fn btae(&mut self) -> BTAE_W<DSI_PCRrs> {
        BTAE_W::new(self, 2)
    }
    #[doc = "Bit 3 - ECC reception enable This bit enables the ECC reception, error correction and reporting."]
    #[inline(always)]
    #[must_use]
    pub fn eccrxe(&mut self) -> ECCRXE_W<DSI_PCRrs> {
        ECCRXE_W::new(self, 3)
    }
    #[doc = "Bit 4 - CRC reception enable This bit enables the CRC reception and error reporting."]
    #[inline(always)]
    #[must_use]
    pub fn crcrxe(&mut self) -> CRCRXE_W<DSI_PCRrs> {
        CRCRXE_W::new(self, 4)
    }
    #[doc = "Bit 5 - EoTp transmission in low-power enable This bit enables the EoTP transmission in low-power."]
    #[inline(always)]
    #[must_use]
    pub fn ettxlpe(&mut self) -> ETTXLPE_W<DSI_PCRrs> {
        ETTXLPE_W::new(self, 5)
    }
}
#[doc = "DSI Host protocol configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_pcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_pcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_PCRrs;
impl crate::RegisterSpec for DSI_PCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_pcr::R`](R) reader structure"]
impl crate::Readable for DSI_PCRrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_pcr::W`](W) writer structure"]
impl crate::Writable for DSI_PCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_PCR to value 0"]
impl crate::Resettable for DSI_PCRrs {
    const RESET_VALUE: u32 = 0;
}
