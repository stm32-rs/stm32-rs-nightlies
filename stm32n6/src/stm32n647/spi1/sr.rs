///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `RXP` reader - Rx-packet available
pub type RXP_R = crate::BitReader;
///Field `TXP` reader - Tx-packet space available
pub type TXP_R = crate::BitReader;
///Field `DXP` reader - duplex packet
pub type DXP_R = crate::BitReader;
///Field `EOT` reader - end of transfer
pub type EOT_R = crate::BitReader;
///Field `TXTF` reader - transmission transfer filled
pub type TXTF_R = crate::BitReader;
///Field `UDR` reader - underrun
pub type UDR_R = crate::BitReader;
///Field `OVR` reader - overrun
pub type OVR_R = crate::BitReader;
///Field `CRCE` reader - CRC error
pub type CRCE_R = crate::BitReader;
///Field `TIFRE` reader - TI frame format error
pub type TIFRE_R = crate::BitReader;
///Field `MODF` reader - mode fault
pub type MODF_R = crate::BitReader;
///Field `SUSP` reader - suspension status
pub type SUSP_R = crate::BitReader;
///Field `TXC` reader - TxFIFO transmission complete
pub type TXC_R = crate::BitReader;
///Field `RXPLVL` reader - RxFIFO packing level
pub type RXPLVL_R = crate::FieldReader;
///Field `RXWNE` reader - RxFIFO word not empty
pub type RXWNE_R = crate::BitReader;
///Field `CTSIZE` reader - number of data frames remaining in current TSIZE session
pub type CTSIZE_R = crate::FieldReader<u16>;
impl R {
    ///Bit 0 - Rx-packet available
    #[inline(always)]
    pub fn rxp(&self) -> RXP_R {
        RXP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Tx-packet space available
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - duplex packet
    #[inline(always)]
    pub fn dxp(&self) -> DXP_R {
        DXP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - end of transfer
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - transmission transfer filled
    #[inline(always)]
    pub fn txtf(&self) -> TXTF_R {
        TXTF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - underrun
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - overrun
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CRC error
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TI frame format error
    #[inline(always)]
    pub fn tifre(&self) -> TIFRE_R {
        TIFRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - mode fault
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - suspension status
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TxFIFO transmission complete
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - RxFIFO packing level
    #[inline(always)]
    pub fn rxplvl(&self) -> RXPLVL_R {
        RXPLVL_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - RxFIFO word not empty
    #[inline(always)]
    pub fn rxwne(&self) -> RXWNE_R {
        RXWNE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:31 - number of data frames remaining in current TSIZE session
    #[inline(always)]
    pub fn ctsize(&self) -> CTSIZE_R {
        CTSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("rxp", &self.rxp())
            .field("txp", &self.txp())
            .field("dxp", &self.dxp())
            .field("eot", &self.eot())
            .field("txtf", &self.txtf())
            .field("udr", &self.udr())
            .field("ovr", &self.ovr())
            .field("crce", &self.crce())
            .field("tifre", &self.tifre())
            .field("modf", &self.modf())
            .field("susp", &self.susp())
            .field("txc", &self.txc())
            .field("rxplvl", &self.rxplvl())
            .field("rxwne", &self.rxwne())
            .field("ctsize", &self.ctsize())
            .finish()
    }
}
/**SPI/I2S status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SPI1:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0x1002
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x1002;
}
