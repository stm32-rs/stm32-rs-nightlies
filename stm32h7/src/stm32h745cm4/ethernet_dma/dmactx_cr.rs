///Register `DMACTxCR` reader
pub type R = crate::R<DMACTX_CRrs>;
///Register `DMACTxCR` writer
pub type W = crate::W<DMACTX_CRrs>;
///Field `ST` reader - Start or Stop Transmission Command
pub type ST_R = crate::BitReader;
///Field `ST` writer - Start or Stop Transmission Command
pub type ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OSF` reader - Operate on Second Packet
pub type OSF_R = crate::BitReader;
///Field `OSF` writer - Operate on Second Packet
pub type OSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSE` reader - TCP Segmentation Enabled
pub type TSE_R = crate::BitReader;
///Field `TSE` writer - TCP Segmentation Enabled
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXPBL` reader - Transmit Programmable Burst Length
pub type TXPBL_R = crate::FieldReader;
///Field `TXPBL` writer - Transmit Programmable Burst Length
pub type TXPBL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bit 0 - Start or Stop Transmission Command
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 1) != 0)
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
    ///Bits 16:21 - Transmit Programmable Burst Length
    #[inline(always)]
    pub fn txpbl(&self) -> TXPBL_R {
        TXPBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACTxCR")
            .field("txpbl", &self.txpbl())
            .field("tse", &self.tse())
            .field("osf", &self.osf())
            .field("st", &self.st())
            .finish()
    }
}
impl W {
    ///Bit 0 - Start or Stop Transmission Command
    #[inline(always)]
    pub fn st(&mut self) -> ST_W<'_, DMACTX_CRrs> {
        ST_W::new(self, 0)
    }
    ///Bit 4 - Operate on Second Packet
    #[inline(always)]
    pub fn osf(&mut self) -> OSF_W<'_, DMACTX_CRrs> {
        OSF_W::new(self, 4)
    }
    ///Bit 12 - TCP Segmentation Enabled
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W<'_, DMACTX_CRrs> {
        TSE_W::new(self, 12)
    }
    ///Bits 16:21 - Transmit Programmable Burst Length
    #[inline(always)]
    pub fn txpbl(&mut self) -> TXPBL_W<'_, DMACTX_CRrs> {
        TXPBL_W::new(self, 16)
    }
}
/**Channel transmit control register

You can [`read`](crate::Reg::read) this register and get [`dmactx_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactx_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#Ethernet_DMA:DMACTxCR)*/
pub struct DMACTX_CRrs;
impl crate::RegisterSpec for DMACTX_CRrs {
    type Ux = u32;
}
///`read()` method returns [`dmactx_cr::R`](R) reader structure
impl crate::Readable for DMACTX_CRrs {}
///`write(|w| ..)` method takes [`dmactx_cr::W`](W) writer structure
impl crate::Writable for DMACTX_CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACTxCR to value 0
impl crate::Resettable for DMACTX_CRrs {}
