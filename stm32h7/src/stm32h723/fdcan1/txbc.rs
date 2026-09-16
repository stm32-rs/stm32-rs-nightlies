///Register `TXBC` reader
pub type R = crate::R<TXBCrs>;
///Register `TXBC` writer
pub type W = crate::W<TXBCrs>;
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
        f.debug_struct("TXBC")
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
    pub fn tbsa(&mut self) -> TBSA_W<'_, TXBCrs> {
        TBSA_W::new(self, 2)
    }
    ///Bits 16:21 - Number of Dedicated Transmit Buffers
    #[inline(always)]
    pub fn ndtb(&mut self) -> NDTB_W<'_, TXBCrs> {
        NDTB_W::new(self, 16)
    }
    ///Bits 24:29 - Transmit FIFO/Queue Size
    #[inline(always)]
    pub fn tfqs(&mut self) -> TFQS_W<'_, TXBCrs> {
        TFQS_W::new(self, 24)
    }
    ///Bit 30 - Tx FIFO/Queue Mode
    #[inline(always)]
    pub fn tfqm(&mut self) -> TFQM_W<'_, TXBCrs> {
        TFQM_W::new(self, 30)
    }
}
/**FDCAN Tx Buffer Configuration Register

You can [`read`](crate::Reg::read) this register and get [`txbc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H723.html#FDCAN1:TXBC)*/
pub struct TXBCrs;
impl crate::RegisterSpec for TXBCrs {
    type Ux = u32;
}
///`read()` method returns [`txbc::R`](R) reader structure
impl crate::Readable for TXBCrs {}
///`write(|w| ..)` method takes [`txbc::W`](W) writer structure
impl crate::Writable for TXBCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TXBC to value 0
impl crate::Resettable for TXBCrs {}
