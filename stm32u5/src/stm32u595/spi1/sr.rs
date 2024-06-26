///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `RXP` reader - Rx-Packet available
pub type RXP_R = crate::BitReader;
///Field `TXP` reader - Tx-Packet space available
pub type TXP_R = crate::BitReader;
///Field `DXP` reader - Duplex Packet
pub type DXP_R = crate::BitReader;
///Field `EOT` reader - End Of Transfer
pub type EOT_R = crate::BitReader;
///Field `TXTF` reader - Transmission Transfer Filled
pub type TXTF_R = crate::BitReader;
///Field `UDR` reader - Underrun at slave transmission mode
pub type UDR_R = crate::BitReader;
///Field `OVR` reader - Overrun
pub type OVR_R = crate::BitReader;
///Field `CRCE` reader - CRC Error
pub type CRCE_R = crate::BitReader;
///Field `TIFRE` reader - TI frame format error
pub type TIFRE_R = crate::BitReader;
///Field `MODF` reader - Mode Fault
pub type MODF_R = crate::BitReader;
///Field `SUSP` reader - SUSPend
pub type SUSP_R = crate::BitReader;
///Field `TXC` reader - TxFIFO transmission complete
pub type TXC_R = crate::BitReader;
///Field `RXPLVL` reader - RxFIFO Packing LeVeL
pub type RXPLVL_R = crate::FieldReader;
///Field `RXWNE` reader - RxFIFO Word Not Empty
pub type RXWNE_R = crate::BitReader;
///Field `CTSIZE` reader - Number of data frames remaining in current TSIZE session
pub type CTSIZE_R = crate::FieldReader<u16>;
impl R {
    ///Bit 0 - Rx-Packet available
    #[inline(always)]
    pub fn rxp(&self) -> RXP_R {
        RXP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Tx-Packet space available
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Duplex Packet
    #[inline(always)]
    pub fn dxp(&self) -> DXP_R {
        DXP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - End Of Transfer
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Transmission Transfer Filled
    #[inline(always)]
    pub fn txtf(&self) -> TXTF_R {
        TXTF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Underrun at slave transmission mode
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Overrun
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CRC Error
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TI frame format error
    #[inline(always)]
    pub fn tifre(&self) -> TIFRE_R {
        TIFRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Mode Fault
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - SUSPend
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TxFIFO transmission complete
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - RxFIFO Packing LeVeL
    #[inline(always)]
    pub fn rxplvl(&self) -> RXPLVL_R {
        RXPLVL_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - RxFIFO Word Not Empty
    #[inline(always)]
    pub fn rxwne(&self) -> RXWNE_R {
        RXWNE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:31 - Number of data frames remaining in current TSIZE session
    #[inline(always)]
    pub fn ctsize(&self) -> CTSIZE_R {
        CTSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("ctsize", &self.ctsize())
            .field("rxwne", &self.rxwne())
            .field("rxplvl", &self.rxplvl())
            .field("txc", &self.txc())
            .field("susp", &self.susp())
            .field("modf", &self.modf())
            .field("tifre", &self.tifre())
            .field("crce", &self.crce())
            .field("ovr", &self.ovr())
            .field("udr", &self.udr())
            .field("txtf", &self.txtf())
            .field("eot", &self.eot())
            .field("dxp", &self.dxp())
            .field("txp", &self.txp())
            .field("rxp", &self.rxp())
            .finish()
    }
}
/**Status Register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#SPI1:SR)*/
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
