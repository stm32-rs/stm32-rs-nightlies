///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `RXNE` reader - Receive buffer not empty
pub type RXNE_R = crate::BitReader;
///Field `TXE` reader - Transmit buffer empty
pub type TXE_R = crate::BitReader;
///Field `CHSIDE` reader - Channel side
pub type CHSIDE_R = crate::BitReader;
///Field `UDR` reader - Underrun flag
pub type UDR_R = crate::BitReader;
///Field `CRCERR` reader - CRC error flag
pub type CRCERR_R = crate::BitReader;
///Field `CRCERR` writer - CRC error flag
pub type CRCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODF` reader - Mode fault
pub type MODF_R = crate::BitReader;
///Field `OVR` reader - Overrun flag
pub type OVR_R = crate::BitReader;
///Field `BSY` reader - Busy flag
pub type BSY_R = crate::BitReader;
///Field `TIFRFE` reader - TI frame format error
pub type TIFRFE_R = crate::BitReader;
impl R {
    ///Bit 0 - Receive buffer not empty
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit buffer empty
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Channel side
    #[inline(always)]
    pub fn chside(&self) -> CHSIDE_R {
        CHSIDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Underrun flag
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CRC error flag
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Mode fault
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Overrun flag
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Busy flag
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TI frame format error
    #[inline(always)]
    pub fn tifrfe(&self) -> TIFRFE_R {
        TIFRFE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("tifrfe", &self.tifrfe())
            .field("bsy", &self.bsy())
            .field("ovr", &self.ovr())
            .field("modf", &self.modf())
            .field("crcerr", &self.crcerr())
            .field("udr", &self.udr())
            .field("chside", &self.chside())
            .field("txe", &self.txe())
            .field("rxne", &self.rxne())
            .finish()
    }
}
impl W {
    ///Bit 4 - CRC error flag
    #[inline(always)]
    pub fn crcerr(&mut self) -> CRCERR_W<'_, SRrs> {
        CRCERR_W::new(self, 4)
    }
}
/**status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#SPI1:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR to value 0x02
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x02;
}
