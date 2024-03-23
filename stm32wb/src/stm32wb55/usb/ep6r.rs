#[doc = "Register `EP6R` reader"]
pub type R = crate::R<EP6Rrs>;
#[doc = "Register `EP6R` writer"]
pub type W = crate::W<EP6Rrs>;
#[doc = "Field `EA` reader - Endpoint address"]
pub type EA_R = crate::FieldReader;
#[doc = "Field `EA` writer - Endpoint address"]
pub type EA_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `STAT_TX` reader - Status bits, for transmission transfers"]
pub type STAT_TX_R = crate::FieldReader;
#[doc = "Field `STAT_TX` writer - Status bits, for transmission transfers"]
pub type STAT_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DTOG_TX` reader - Data Toggle, for transmission transfers"]
pub type DTOG_TX_R = crate::BitReader;
#[doc = "Field `DTOG_TX` writer - Data Toggle, for transmission transfers"]
pub type DTOG_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTR_TX` reader - Correct Transfer for transmission"]
pub type CTR_TX_R = crate::BitReader;
#[doc = "Field `CTR_TX` writer - Correct Transfer for transmission"]
pub type CTR_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP_KIND` reader - Endpoint kind"]
pub type EP_KIND_R = crate::BitReader;
#[doc = "Field `EP_KIND` writer - Endpoint kind"]
pub type EP_KIND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP_TYPE` reader - Endpoint type"]
pub type EP_TYPE_R = crate::FieldReader;
#[doc = "Field `EP_TYPE` writer - Endpoint type"]
pub type EP_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SETUP` reader - Setup transaction completed"]
pub type SETUP_R = crate::BitReader;
#[doc = "Field `SETUP` writer - Setup transaction completed"]
pub type SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STAT_RX` reader - Status bits, for reception transfers"]
pub type STAT_RX_R = crate::FieldReader;
#[doc = "Field `STAT_RX` writer - Status bits, for reception transfers"]
pub type STAT_RX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DTOG_RX` reader - Data Toggle, for reception transfers"]
pub type DTOG_RX_R = crate::BitReader;
#[doc = "Field `DTOG_RX` writer - Data Toggle, for reception transfers"]
pub type DTOG_RX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTR_RX` reader - Correct transfer for reception"]
pub type CTR_RX_R = crate::BitReader;
#[doc = "Field `CTR_RX` writer - Correct transfer for reception"]
pub type CTR_RX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline(always)]
    pub fn ea(&self) -> EA_R {
        EA_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Status bits, for transmission transfers"]
    #[inline(always)]
    pub fn stat_tx(&self) -> STAT_TX_R {
        STAT_TX_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Data Toggle, for transmission transfers"]
    #[inline(always)]
    pub fn dtog_tx(&self) -> DTOG_TX_R {
        DTOG_TX_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Correct Transfer for transmission"]
    #[inline(always)]
    pub fn ctr_tx(&self) -> CTR_TX_R {
        CTR_TX_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoint kind"]
    #[inline(always)]
    pub fn ep_kind(&self) -> EP_KIND_R {
        EP_KIND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Endpoint type"]
    #[inline(always)]
    pub fn ep_type(&self) -> EP_TYPE_R {
        EP_TYPE_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Status bits, for reception transfers"]
    #[inline(always)]
    pub fn stat_rx(&self) -> STAT_RX_R {
        STAT_RX_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Data Toggle, for reception transfers"]
    #[inline(always)]
    pub fn dtog_rx(&self) -> DTOG_RX_R {
        DTOG_RX_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Correct transfer for reception"]
    #[inline(always)]
    pub fn ctr_rx(&self) -> CTR_RX_R {
        CTR_RX_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline(always)]
    #[must_use]
    pub fn ea(&mut self) -> EA_W<EP6Rrs> {
        EA_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Status bits, for transmission transfers"]
    #[inline(always)]
    #[must_use]
    pub fn stat_tx(&mut self) -> STAT_TX_W<EP6Rrs> {
        STAT_TX_W::new(self, 4)
    }
    #[doc = "Bit 6 - Data Toggle, for transmission transfers"]
    #[inline(always)]
    #[must_use]
    pub fn dtog_tx(&mut self) -> DTOG_TX_W<EP6Rrs> {
        DTOG_TX_W::new(self, 6)
    }
    #[doc = "Bit 7 - Correct Transfer for transmission"]
    #[inline(always)]
    #[must_use]
    pub fn ctr_tx(&mut self) -> CTR_TX_W<EP6Rrs> {
        CTR_TX_W::new(self, 7)
    }
    #[doc = "Bit 8 - Endpoint kind"]
    #[inline(always)]
    #[must_use]
    pub fn ep_kind(&mut self) -> EP_KIND_W<EP6Rrs> {
        EP_KIND_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - Endpoint type"]
    #[inline(always)]
    #[must_use]
    pub fn ep_type(&mut self) -> EP_TYPE_W<EP6Rrs> {
        EP_TYPE_W::new(self, 9)
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    #[must_use]
    pub fn setup(&mut self) -> SETUP_W<EP6Rrs> {
        SETUP_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - Status bits, for reception transfers"]
    #[inline(always)]
    #[must_use]
    pub fn stat_rx(&mut self) -> STAT_RX_W<EP6Rrs> {
        STAT_RX_W::new(self, 12)
    }
    #[doc = "Bit 14 - Data Toggle, for reception transfers"]
    #[inline(always)]
    #[must_use]
    pub fn dtog_rx(&mut self) -> DTOG_RX_W<EP6Rrs> {
        DTOG_RX_W::new(self, 14)
    }
    #[doc = "Bit 15 - Correct transfer for reception"]
    #[inline(always)]
    #[must_use]
    pub fn ctr_rx(&mut self) -> CTR_RX_W<EP6Rrs> {
        CTR_RX_W::new(self, 15)
    }
}
#[doc = "endpoint 6 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep6r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep6r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EP6Rrs;
impl crate::RegisterSpec for EP6Rrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep6r::R`](R) reader structure"]
impl crate::Readable for EP6Rrs {}
#[doc = "`write(|w| ..)` method takes [`ep6r::W`](W) writer structure"]
impl crate::Writable for EP6Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EP6R to value 0"]
impl crate::Resettable for EP6Rrs {
    const RESET_VALUE: u16 = 0;
}
