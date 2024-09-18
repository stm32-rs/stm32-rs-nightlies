///Register `EP0R` reader
pub type R = crate::R<EP0Rrs>;
///Register `EP0R` writer
pub type W = crate::W<EP0Rrs>;
///Field `EA` reader - Endpoint address
pub type EA_R = crate::FieldReader;
///Field `EA` writer - Endpoint address
pub type EA_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `STAT_TX` reader - Status bits, for transmission transfers
pub type STAT_TX_R = crate::FieldReader;
///Field `STAT_TX` writer - Status bits, for transmission transfers
pub type STAT_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DTOG_TX` reader - Data Toggle, for transmission transfers
pub type DTOG_TX_R = crate::BitReader;
///Field `DTOG_TX` writer - Data Toggle, for transmission transfers
pub type DTOG_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTR_TX` reader - Correct Transfer for transmission
pub type CTR_TX_R = crate::BitReader;
///Field `CTR_TX` writer - Correct Transfer for transmission
pub type CTR_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EP_KIND` reader - Endpoint kind
pub type EP_KIND_R = crate::BitReader;
///Field `EP_KIND` writer - Endpoint kind
pub type EP_KIND_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EP_TYPE` reader - Endpoint type
pub type EP_TYPE_R = crate::FieldReader;
///Field `EP_TYPE` writer - Endpoint type
pub type EP_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SETUP` reader - Setup transaction completed
pub type SETUP_R = crate::BitReader;
///Field `SETUP` writer - Setup transaction completed
pub type SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STAT_RX` reader - Status bits, for reception transfers
pub type STAT_RX_R = crate::FieldReader;
///Field `STAT_RX` writer - Status bits, for reception transfers
pub type STAT_RX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DTOG_RX` reader - Data Toggle, for reception transfers
pub type DTOG_RX_R = crate::BitReader;
///Field `DTOG_RX` writer - Data Toggle, for reception transfers
pub type DTOG_RX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTR_RX` reader - Correct transfer for reception
pub type CTR_RX_R = crate::BitReader;
///Field `CTR_RX` writer - Correct transfer for reception
pub type CTR_RX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Endpoint address
    #[inline(always)]
    pub fn ea(&self) -> EA_R {
        EA_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - Status bits, for transmission transfers
    #[inline(always)]
    pub fn stat_tx(&self) -> STAT_TX_R {
        STAT_TX_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Data Toggle, for transmission transfers
    #[inline(always)]
    pub fn dtog_tx(&self) -> DTOG_TX_R {
        DTOG_TX_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Correct Transfer for transmission
    #[inline(always)]
    pub fn ctr_tx(&self) -> CTR_TX_R {
        CTR_TX_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Endpoint kind
    #[inline(always)]
    pub fn ep_kind(&self) -> EP_KIND_R {
        EP_KIND_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - Endpoint type
    #[inline(always)]
    pub fn ep_type(&self) -> EP_TYPE_R {
        EP_TYPE_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bit 11 - Setup transaction completed
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Status bits, for reception transfers
    #[inline(always)]
    pub fn stat_rx(&self) -> STAT_RX_R {
        STAT_RX_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - Data Toggle, for reception transfers
    #[inline(always)]
    pub fn dtog_rx(&self) -> DTOG_RX_R {
        DTOG_RX_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Correct transfer for reception
    #[inline(always)]
    pub fn ctr_rx(&self) -> CTR_RX_R {
        CTR_RX_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EP0R")
            .field("ea", &self.ea())
            .field("stat_tx", &self.stat_tx())
            .field("dtog_tx", &self.dtog_tx())
            .field("ctr_tx", &self.ctr_tx())
            .field("ep_kind", &self.ep_kind())
            .field("ep_type", &self.ep_type())
            .field("setup", &self.setup())
            .field("stat_rx", &self.stat_rx())
            .field("dtog_rx", &self.dtog_rx())
            .field("ctr_rx", &self.ctr_rx())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Endpoint address
    #[inline(always)]
    #[must_use]
    pub fn ea(&mut self) -> EA_W<EP0Rrs> {
        EA_W::new(self, 0)
    }
    ///Bits 4:5 - Status bits, for transmission transfers
    #[inline(always)]
    #[must_use]
    pub fn stat_tx(&mut self) -> STAT_TX_W<EP0Rrs> {
        STAT_TX_W::new(self, 4)
    }
    ///Bit 6 - Data Toggle, for transmission transfers
    #[inline(always)]
    #[must_use]
    pub fn dtog_tx(&mut self) -> DTOG_TX_W<EP0Rrs> {
        DTOG_TX_W::new(self, 6)
    }
    ///Bit 7 - Correct Transfer for transmission
    #[inline(always)]
    #[must_use]
    pub fn ctr_tx(&mut self) -> CTR_TX_W<EP0Rrs> {
        CTR_TX_W::new(self, 7)
    }
    ///Bit 8 - Endpoint kind
    #[inline(always)]
    #[must_use]
    pub fn ep_kind(&mut self) -> EP_KIND_W<EP0Rrs> {
        EP_KIND_W::new(self, 8)
    }
    ///Bits 9:10 - Endpoint type
    #[inline(always)]
    #[must_use]
    pub fn ep_type(&mut self) -> EP_TYPE_W<EP0Rrs> {
        EP_TYPE_W::new(self, 9)
    }
    ///Bit 11 - Setup transaction completed
    #[inline(always)]
    #[must_use]
    pub fn setup(&mut self) -> SETUP_W<EP0Rrs> {
        SETUP_W::new(self, 11)
    }
    ///Bits 12:13 - Status bits, for reception transfers
    #[inline(always)]
    #[must_use]
    pub fn stat_rx(&mut self) -> STAT_RX_W<EP0Rrs> {
        STAT_RX_W::new(self, 12)
    }
    ///Bit 14 - Data Toggle, for reception transfers
    #[inline(always)]
    #[must_use]
    pub fn dtog_rx(&mut self) -> DTOG_RX_W<EP0Rrs> {
        DTOG_RX_W::new(self, 14)
    }
    ///Bit 15 - Correct transfer for reception
    #[inline(always)]
    #[must_use]
    pub fn ctr_rx(&mut self) -> CTR_RX_W<EP0Rrs> {
        CTR_RX_W::new(self, 15)
    }
}
/**endpoint 0 register

You can [`read`](crate::Reg::read) this register and get [`ep0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x5.html#USB:EP0R)*/
pub struct EP0Rrs;
impl crate::RegisterSpec for EP0Rrs {
    type Ux = u32;
}
///`read()` method returns [`ep0r::R`](R) reader structure
impl crate::Readable for EP0Rrs {}
///`write(|w| ..)` method takes [`ep0r::W`](W) writer structure
impl crate::Writable for EP0Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EP0R to value 0
impl crate::Resettable for EP0Rrs {
    const RESET_VALUE: u32 = 0;
}
