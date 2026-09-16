///Register `DMAC0TXCR` reader
pub type R = crate::R<DMAC0TXCRrs>;
///Register `DMAC0TXCR` writer
pub type W = crate::W<DMAC0TXCRrs>;
///Field `ST` reader - Start or Stop Transmission Command
pub type ST_R = crate::BitReader;
///Field `ST` writer - Start or Stop Transmission Command
pub type ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCW` reader - Transmit Channel Weight
pub type TCW_R = crate::FieldReader;
///Field `OSF` reader - Operate on Second Packet
pub type OSF_R = crate::BitReader;
///Field `OSF` writer - Operate on Second Packet
pub type OSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSE` reader - TCP Segmentation Enabled
pub type TSE_R = crate::BitReader;
///Field `TSE` writer - TCP Segmentation Enabled
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IPBL` reader - Ignore PBL Requirement
pub type IPBL_R = crate::BitReader;
///Field `IPBL` writer - Ignore PBL Requirement
pub type IPBL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXPBL` reader - Transmit Programmable Burst Length
pub type TXPBL_R = crate::FieldReader;
///Field `TXPBL` writer - Transmit Programmable Burst Length
pub type TXPBL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `TQOS` reader - Transmit QOS
pub type TQOS_R = crate::FieldReader;
///Field `TQOS` writer - Transmit QOS
pub type TQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `EDSE` reader - Enhanced Descriptor Enable
pub type EDSE_R = crate::BitReader;
///Field `EDSE` writer - Enhanced Descriptor Enable
pub type EDSE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Start or Stop Transmission Command
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - Transmit Channel Weight
    #[inline(always)]
    pub fn tcw(&self) -> TCW_R {
        TCW_R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bit 4 - Operate on Second Packet
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 12 - TCP Segmentation Enabled
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - Ignore PBL Requirement
    #[inline(always)]
    pub fn ipbl(&self) -> IPBL_R {
        IPBL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:21 - Transmit Programmable Burst Length
    #[inline(always)]
    pub fn txpbl(&self) -> TXPBL_R {
        TXPBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:27 - Transmit QOS
    #[inline(always)]
    pub fn tqos(&self) -> TQOS_R {
        TQOS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bit 28 - Enhanced Descriptor Enable
    #[inline(always)]
    pub fn edse(&self) -> EDSE_R {
        EDSE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC0TXCR")
            .field("st", &self.st())
            .field("tcw", &self.tcw())
            .field("osf", &self.osf())
            .field("tse", &self.tse())
            .field("ipbl", &self.ipbl())
            .field("txpbl", &self.txpbl())
            .field("tqos", &self.tqos())
            .field("edse", &self.edse())
            .finish()
    }
}
impl W {
    ///Bit 0 - Start or Stop Transmission Command
    #[inline(always)]
    pub fn st(&mut self) -> ST_W<'_, DMAC0TXCRrs> {
        ST_W::new(self, 0)
    }
    ///Bit 4 - Operate on Second Packet
    #[inline(always)]
    pub fn osf(&mut self) -> OSF_W<'_, DMAC0TXCRrs> {
        OSF_W::new(self, 4)
    }
    ///Bit 12 - TCP Segmentation Enabled
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W<'_, DMAC0TXCRrs> {
        TSE_W::new(self, 12)
    }
    ///Bit 15 - Ignore PBL Requirement
    #[inline(always)]
    pub fn ipbl(&mut self) -> IPBL_W<'_, DMAC0TXCRrs> {
        IPBL_W::new(self, 15)
    }
    ///Bits 16:21 - Transmit Programmable Burst Length
    #[inline(always)]
    pub fn txpbl(&mut self) -> TXPBL_W<'_, DMAC0TXCRrs> {
        TXPBL_W::new(self, 16)
    }
    ///Bits 24:27 - Transmit QOS
    #[inline(always)]
    pub fn tqos(&mut self) -> TQOS_W<'_, DMAC0TXCRrs> {
        TQOS_W::new(self, 24)
    }
    ///Bit 28 - Enhanced Descriptor Enable
    #[inline(always)]
    pub fn edse(&mut self) -> EDSE_W<'_, DMAC0TXCRrs> {
        EDSE_W::new(self, 28)
    }
}
/**Channel 0 transmit control register

You can [`read`](crate::Reg::read) this register and get [`dmac0txcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0txcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:DMAC0TXCR)*/
pub struct DMAC0TXCRrs;
impl crate::RegisterSpec for DMAC0TXCRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac0txcr::R`](R) reader structure
impl crate::Readable for DMAC0TXCRrs {}
///`write(|w| ..)` method takes [`dmac0txcr::W`](W) writer structure
impl crate::Writable for DMAC0TXCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAC0TXCR to value 0
impl crate::Resettable for DMAC0TXCRrs {}
