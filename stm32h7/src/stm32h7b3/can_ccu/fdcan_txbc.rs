///Register `FDCAN_TXBC` reader
pub type R = crate::R<FDCAN_TXBCrs>;
///Register `FDCAN_TXBC` writer
pub type W = crate::W<FDCAN_TXBCrs>;
///Field `TBSA` reader - Tx Buffers Start Address
pub type TBSA_R = crate::FieldReader<u16>;
///Field `TBSA` writer - Tx Buffers Start Address
pub type TBSA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `NDTB` reader - Number of Dedicated Transmit Buffers
pub type NDTB_R = crate::FieldReader;
///Field `NDTB` writer - Number of Dedicated Transmit Buffers
pub type NDTB_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `TFQS` reader - Transmit FIFO/Queue Size
pub type TFQS_R = crate::FieldReader;
///Field `TFQS` writer - Transmit FIFO/Queue Size
pub type TFQS_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `TFQM` reader - Tx FIFO/Queue Mode
pub type TFQM_R = crate::BitReader;
///Field `TFQM` writer - Tx FIFO/Queue Mode
pub type TFQM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 2:15 - Tx Buffers Start Address
    #[inline(always)]
    pub fn tbsa(&self) -> TBSA_R {
        TBSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    ///Bits 16:21 - Number of Dedicated Transmit Buffers
    #[inline(always)]
    pub fn ndtb(&self) -> NDTB_R {
        NDTB_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:29 - Transmit FIFO/Queue Size
    #[inline(always)]
    pub fn tfqs(&self) -> TFQS_R {
        TFQS_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    ///Bit 30 - Tx FIFO/Queue Mode
    #[inline(always)]
    pub fn tfqm(&self) -> TFQM_R {
        TFQM_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBC")
            .field("tbsa", &self.tbsa())
            .field("ndtb", &self.ndtb())
            .field("tfqs", &self.tfqs())
            .field("tfqm", &self.tfqm())
            .finish()
    }
}
impl W {
    ///Bits 2:15 - Tx Buffers Start Address
    #[inline(always)]
    #[must_use]
    pub fn tbsa(&mut self) -> TBSA_W<FDCAN_TXBCrs> {
        TBSA_W::new(self, 2)
    }
    ///Bits 16:21 - Number of Dedicated Transmit Buffers
    #[inline(always)]
    #[must_use]
    pub fn ndtb(&mut self) -> NDTB_W<FDCAN_TXBCrs> {
        NDTB_W::new(self, 16)
    }
    ///Bits 24:29 - Transmit FIFO/Queue Size
    #[inline(always)]
    #[must_use]
    pub fn tfqs(&mut self) -> TFQS_W<FDCAN_TXBCrs> {
        TFQS_W::new(self, 24)
    }
    ///Bit 30 - Tx FIFO/Queue Mode
    #[inline(always)]
    #[must_use]
    pub fn tfqm(&mut self) -> TFQM_W<FDCAN_TXBCrs> {
        TFQM_W::new(self, 30)
    }
}
/**FDCAN Tx Buffer Configuration Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TXBC)*/
pub struct FDCAN_TXBCrs;
impl crate::RegisterSpec for FDCAN_TXBCrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_txbc::R`](R) reader structure
impl crate::Readable for FDCAN_TXBCrs {}
///`write(|w| ..)` method takes [`fdcan_txbc::W`](W) writer structure
impl crate::Writable for FDCAN_TXBCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FDCAN_TXBC to value 0
impl crate::Resettable for FDCAN_TXBCrs {
    const RESET_VALUE: u32 = 0;
}
