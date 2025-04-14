///Register `EP%sR` reader
pub type R = crate::R<EPRrs>;
///Register `EP%sR` writer
pub type W = crate::W<EPRrs>;
///Field `EA` reader - EA
pub type EA_R = crate::FieldReader;
///Field `EA` writer - EA
pub type EA_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `STAT_TX` reader - STAT_TX
pub type STAT_TX_R = crate::FieldReader;
///Field `STAT_TX` writer - STAT_TX
pub type STAT_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DTOG_TX` reader - DTOG_TX
pub type DTOG_TX_R = crate::BitReader;
///Field `DTOG_TX` writer - DTOG_TX
pub type DTOG_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTR_TX` reader - CTR_TX
pub type CTR_TX_R = crate::BitReader;
///Field `CTR_TX` writer - CTR_TX
pub type CTR_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EP_KIND` reader - EP_KIND
pub type EP_KIND_R = crate::BitReader;
///Field `EP_KIND` writer - EP_KIND
pub type EP_KIND_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EP_TYPE` reader - EP_TYPE
pub type EP_TYPE_R = crate::FieldReader;
///Field `EP_TYPE` writer - EP_TYPE
pub type EP_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SETUP` reader - SETUP
pub type SETUP_R = crate::BitReader;
///Field `SETUP` writer - SETUP
pub type SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STAT_RX` reader - STAT_RX
pub type STAT_RX_R = crate::FieldReader;
///Field `STAT_RX` writer - STAT_RX
pub type STAT_RX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DTOG_RX` reader - DTOG_RX
pub type DTOG_RX_R = crate::BitReader;
///Field `DTOG_RX` writer - DTOG_RX
pub type DTOG_RX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTR_RX` reader - CTR_RX
pub type CTR_RX_R = crate::BitReader;
///Field `CTR_RX` writer - CTR_RX
pub type CTR_RX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - EA
    #[inline(always)]
    pub fn ea(&self) -> EA_R {
        EA_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - STAT_TX
    #[inline(always)]
    pub fn stat_tx(&self) -> STAT_TX_R {
        STAT_TX_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - DTOG_TX
    #[inline(always)]
    pub fn dtog_tx(&self) -> DTOG_TX_R {
        DTOG_TX_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CTR_TX
    #[inline(always)]
    pub fn ctr_tx(&self) -> CTR_TX_R {
        CTR_TX_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - EP_KIND
    #[inline(always)]
    pub fn ep_kind(&self) -> EP_KIND_R {
        EP_KIND_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - EP_TYPE
    #[inline(always)]
    pub fn ep_type(&self) -> EP_TYPE_R {
        EP_TYPE_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bit 11 - SETUP
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - STAT_RX
    #[inline(always)]
    pub fn stat_rx(&self) -> STAT_RX_R {
        STAT_RX_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - DTOG_RX
    #[inline(always)]
    pub fn dtog_rx(&self) -> DTOG_RX_R {
        DTOG_RX_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CTR_RX
    #[inline(always)]
    pub fn ctr_rx(&self) -> CTR_RX_R {
        CTR_RX_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPR")
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
    ///Bits 0:3 - EA
    #[inline(always)]
    pub fn ea(&mut self) -> EA_W<EPRrs> {
        EA_W::new(self, 0)
    }
    ///Bits 4:5 - STAT_TX
    #[inline(always)]
    pub fn stat_tx(&mut self) -> STAT_TX_W<EPRrs> {
        STAT_TX_W::new(self, 4)
    }
    ///Bit 6 - DTOG_TX
    #[inline(always)]
    pub fn dtog_tx(&mut self) -> DTOG_TX_W<EPRrs> {
        DTOG_TX_W::new(self, 6)
    }
    ///Bit 7 - CTR_TX
    #[inline(always)]
    pub fn ctr_tx(&mut self) -> CTR_TX_W<EPRrs> {
        CTR_TX_W::new(self, 7)
    }
    ///Bit 8 - EP_KIND
    #[inline(always)]
    pub fn ep_kind(&mut self) -> EP_KIND_W<EPRrs> {
        EP_KIND_W::new(self, 8)
    }
    ///Bits 9:10 - EP_TYPE
    #[inline(always)]
    pub fn ep_type(&mut self) -> EP_TYPE_W<EPRrs> {
        EP_TYPE_W::new(self, 9)
    }
    ///Bit 11 - SETUP
    #[inline(always)]
    pub fn setup(&mut self) -> SETUP_W<EPRrs> {
        SETUP_W::new(self, 11)
    }
    ///Bits 12:13 - STAT_RX
    #[inline(always)]
    pub fn stat_rx(&mut self) -> STAT_RX_W<EPRrs> {
        STAT_RX_W::new(self, 12)
    }
    ///Bit 14 - DTOG_RX
    #[inline(always)]
    pub fn dtog_rx(&mut self) -> DTOG_RX_W<EPRrs> {
        DTOG_RX_W::new(self, 14)
    }
    ///Bit 15 - CTR_RX
    #[inline(always)]
    pub fn ctr_rx(&mut self) -> CTR_RX_W<EPRrs> {
        CTR_RX_W::new(self, 15)
    }
}
/**USB endpoint n register

You can [`read`](crate::Reg::read) this register and get [`epr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G473.html#USB:EP[0]R)*/
pub struct EPRrs;
impl crate::RegisterSpec for EPRrs {
    type Ux = u32;
}
///`read()` method returns [`epr::R`](R) reader structure
impl crate::Readable for EPRrs {}
///`write(|w| ..)` method takes [`epr::W`](W) writer structure
impl crate::Writable for EPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EP%sR to value 0
impl crate::Resettable for EPRrs {}
