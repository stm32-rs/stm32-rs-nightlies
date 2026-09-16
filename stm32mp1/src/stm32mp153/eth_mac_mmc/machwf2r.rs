///Register `MACHWF2R` reader
pub type R = crate::R<MACHWF2Rrs>;
///Field `RXQCNT` reader - RXQCNT
pub type RXQCNT_R = crate::FieldReader;
///Field `TXQCNT` reader - TXQCNT
pub type TXQCNT_R = crate::FieldReader;
///Field `RXCHCNT` reader - RXCHCNT
pub type RXCHCNT_R = crate::FieldReader;
///Field `TXCHCNT` reader - TXCHCNT
pub type TXCHCNT_R = crate::FieldReader;
///Field `PPSOUTNUM` reader - PPSOUTNUM
pub type PPSOUTNUM_R = crate::FieldReader;
///Field `AUXSNAPNUM` reader - AUXSNAPNUM
pub type AUXSNAPNUM_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - RXQCNT
    #[inline(always)]
    pub fn rxqcnt(&self) -> RXQCNT_R {
        RXQCNT_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 6:9 - TXQCNT
    #[inline(always)]
    pub fn txqcnt(&self) -> TXQCNT_R {
        TXQCNT_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    ///Bits 12:15 - RXCHCNT
    #[inline(always)]
    pub fn rxchcnt(&self) -> RXCHCNT_R {
        RXCHCNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 18:21 - TXCHCNT
    #[inline(always)]
    pub fn txchcnt(&self) -> TXCHCNT_R {
        TXCHCNT_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bits 24:26 - PPSOUTNUM
    #[inline(always)]
    pub fn ppsoutnum(&self) -> PPSOUTNUM_R {
        PPSOUTNUM_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 28:30 - AUXSNAPNUM
    #[inline(always)]
    pub fn auxsnapnum(&self) -> AUXSNAPNUM_R {
        AUXSNAPNUM_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACHWF2R")
            .field("rxqcnt", &self.rxqcnt())
            .field("txqcnt", &self.txqcnt())
            .field("rxchcnt", &self.rxchcnt())
            .field("txchcnt", &self.txchcnt())
            .field("ppsoutnum", &self.ppsoutnum())
            .field("auxsnapnum", &self.auxsnapnum())
            .finish()
    }
}
/**This register indicates the presence of third set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks.

You can [`read`](crate::Reg::read) this register and get [`machwf2r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACHWF2R)*/
pub struct MACHWF2Rrs;
impl crate::RegisterSpec for MACHWF2Rrs {
    type Ux = u32;
}
///`read()` method returns [`machwf2r::R`](R) reader structure
impl crate::Readable for MACHWF2Rrs {}
///`reset()` method sets MACHWF2R to value 0x4104_0041
impl crate::Resettable for MACHWF2Rrs {
    const RESET_VALUE: u32 = 0x4104_0041;
}
