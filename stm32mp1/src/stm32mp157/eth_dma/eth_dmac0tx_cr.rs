///Register `ETH_DMAC0TxCR` reader
pub type R = crate::R<ETH_DMAC0TX_CRrs>;
///Register `ETH_DMAC0TxCR` writer
pub type W = crate::W<ETH_DMAC0TX_CRrs>;
///Field `ST` reader - ST
pub type ST_R = crate::BitReader;
///Field `ST` writer - ST
pub type ST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCW` reader - TCW
pub type TCW_R = crate::FieldReader;
///Field `TCW` writer - TCW
pub type TCW_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OSF` reader - OSF
pub type OSF_R = crate::BitReader;
///Field `OSF` writer - OSF
pub type OSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSE` reader - TSE
pub type TSE_R = crate::BitReader;
///Field `TSE` writer - TSE
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXPBL` reader - TXPBL
pub type TXPBL_R = crate::FieldReader;
///Field `TXPBL` writer - TXPBL
pub type TXPBL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `TQOS` reader - TQOS
pub type TQOS_R = crate::FieldReader;
///Field `TQOS` writer - TQOS
pub type TQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - ST
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - TCW
    #[inline(always)]
    pub fn tcw(&self) -> TCW_R {
        TCW_R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bit 4 - OSF
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 12 - TSE
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 16:21 - TXPBL
    #[inline(always)]
    pub fn txpbl(&self) -> TXPBL_R {
        TXPBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:27 - TQOS
    #[inline(always)]
    pub fn tqos(&self) -> TQOS_R {
        TQOS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_DMAC0TxCR")
            .field("st", &self.st())
            .field("tcw", &self.tcw())
            .field("osf", &self.osf())
            .field("tse", &self.tse())
            .field("txpbl", &self.txpbl())
            .field("tqos", &self.tqos())
            .finish()
    }
}
impl W {
    ///Bit 0 - ST
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<ETH_DMAC0TX_CRrs> {
        ST_W::new(self, 0)
    }
    ///Bits 1:3 - TCW
    #[inline(always)]
    #[must_use]
    pub fn tcw(&mut self) -> TCW_W<ETH_DMAC0TX_CRrs> {
        TCW_W::new(self, 1)
    }
    ///Bit 4 - OSF
    #[inline(always)]
    #[must_use]
    pub fn osf(&mut self) -> OSF_W<ETH_DMAC0TX_CRrs> {
        OSF_W::new(self, 4)
    }
    ///Bit 12 - TSE
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TSE_W<ETH_DMAC0TX_CRrs> {
        TSE_W::new(self, 12)
    }
    ///Bits 16:21 - TXPBL
    #[inline(always)]
    #[must_use]
    pub fn txpbl(&mut self) -> TXPBL_W<ETH_DMAC0TX_CRrs> {
        TXPBL_W::new(self, 16)
    }
    ///Bits 24:27 - TQOS
    #[inline(always)]
    #[must_use]
    pub fn tqos(&mut self) -> TQOS_W<ETH_DMAC0TX_CRrs> {
        TQOS_W::new(self, 24)
    }
}
/**Channel 0 transmit control register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0tx_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac0tx_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_DMA:ETH_DMAC0TxCR)*/
pub struct ETH_DMAC0TX_CRrs;
impl crate::RegisterSpec for ETH_DMAC0TX_CRrs {
    type Ux = u32;
}
///`read()` method returns [`eth_dmac0tx_cr::R`](R) reader structure
impl crate::Readable for ETH_DMAC0TX_CRrs {}
///`write(|w| ..)` method takes [`eth_dmac0tx_cr::W`](W) writer structure
impl crate::Writable for ETH_DMAC0TX_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETH_DMAC0TxCR to value 0
impl crate::Resettable for ETH_DMAC0TX_CRrs {
    const RESET_VALUE: u32 = 0;
}
