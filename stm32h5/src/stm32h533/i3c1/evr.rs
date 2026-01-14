///Register `EVR` reader
pub type R = crate::R<EVRrs>;
///Field `CFEF` reader - C-FIFO empty flag (whatever the I3C acts as controller)
pub type CFEF_R = crate::BitReader;
///Field `TXFEF` reader - TX-FIFO empty flag (whatever the I3C acts as controller/target)
pub type TXFEF_R = crate::BitReader;
///Field `CFNFF` reader - C-FIFO not full flag (when the I3C acts as controller)
pub type CFNFF_R = crate::BitReader;
///Field `SFNEF` reader - S-FIFO not empty flag (when the I3C acts as controller)
pub type SFNEF_R = crate::BitReader;
///Field `TXFNFF` reader - TX-FIFO not full flag (whatever the I3C acts as controller/target)
pub type TXFNFF_R = crate::BitReader;
///Field `RXFNEF` reader - RX-FIFO not empty flag (whatever the I3C acts as controller/target)
pub type RXFNEF_R = crate::BitReader;
///Field `TXLASTF` reader - Last written data byte/word flag (whatever the I3C acts as controller/target)
pub type TXLASTF_R = crate::BitReader;
///Field `RXLASTF` reader - Last read data byte/word flag (when the I3C acts as controller)
pub type RXLASTF_R = crate::BitReader;
///Field `FCF` reader - Frame complete flag (whatever the I3C acts as controller/target)
pub type FCF_R = crate::BitReader;
///Field `RXTGTENDF` reader - Target-initiated read end flag (when the I3C acts as controller)
pub type RXTGTENDF_R = crate::BitReader;
///Field `ERRF` reader - Flag (whatever the I3C acts as controller/target)
pub type ERRF_R = crate::BitReader;
///Field `IBIF` reader - IBI flag (when the I3C acts as controller)
pub type IBIF_R = crate::BitReader;
///Field `IBIENDF` reader - IBI end flag (when the I3C acts as target)
pub type IBIENDF_R = crate::BitReader;
///Field `CRF` reader - Controller-role request flag (when the I3C acts as controller)
pub type CRF_R = crate::BitReader;
///Field `CRUPDF` reader - Controller-role update flag (when the I3C acts as target)
pub type CRUPDF_R = crate::BitReader;
///Field `HJF` reader - Hot-join flag (when the I3C acts as controller)
pub type HJF_R = crate::BitReader;
///Field `WKPF` reader - Wake-up/missed start flag (when the I3C acts as target)
pub type WKPF_R = crate::BitReader;
///Field `GETF` reader - Get flag (when the I3C acts as target)
pub type GETF_R = crate::BitReader;
///Field `STAF` reader - Get status flag (when the I3C acts as target)
pub type STAF_R = crate::BitReader;
///Field `DAUPDF` reader - Dynamic address update flag (when the I3C acts as target)
pub type DAUPDF_R = crate::BitReader;
///Field `MWLUPDF` reader - Maximum write length update flag (when the I3C acts as target)
pub type MWLUPDF_R = crate::BitReader;
///Field `MRLUPDF` reader - Maximum read length update flag (when the I3C acts as target)
pub type MRLUPDF_R = crate::BitReader;
///Field `RSTF` reader - Reset pattern flag (when the I3C acts as target)
pub type RSTF_R = crate::BitReader;
///Field `ASUPDF` reader - Activity state update flag (when the I3C acts as target)
pub type ASUPDF_R = crate::BitReader;
///Field `INTUPDF` reader - Interrupt/controller-role/hot-join update flag (when the I3C acts as target)
pub type INTUPDF_R = crate::BitReader;
///Field `DEFF` reader - DEFTGTS flag (when the I3C acts as target)
pub type DEFF_R = crate::BitReader;
///Field `GRPF` reader - Group addressing flag (when the I3C acts as target)
pub type GRPF_R = crate::BitReader;
impl R {
    ///Bit 0 - C-FIFO empty flag (whatever the I3C acts as controller)
    #[inline(always)]
    pub fn cfef(&self) -> CFEF_R {
        CFEF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TX-FIFO empty flag (whatever the I3C acts as controller/target)
    #[inline(always)]
    pub fn txfef(&self) -> TXFEF_R {
        TXFEF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - C-FIFO not full flag (when the I3C acts as controller)
    #[inline(always)]
    pub fn cfnff(&self) -> CFNFF_R {
        CFNFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - S-FIFO not empty flag (when the I3C acts as controller)
    #[inline(always)]
    pub fn sfnef(&self) -> SFNEF_R {
        SFNEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TX-FIFO not full flag (whatever the I3C acts as controller/target)
    #[inline(always)]
    pub fn txfnff(&self) -> TXFNFF_R {
        TXFNFF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RX-FIFO not empty flag (whatever the I3C acts as controller/target)
    #[inline(always)]
    pub fn rxfnef(&self) -> RXFNEF_R {
        RXFNEF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Last written data byte/word flag (whatever the I3C acts as controller/target)
    #[inline(always)]
    pub fn txlastf(&self) -> TXLASTF_R {
        TXLASTF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Last read data byte/word flag (when the I3C acts as controller)
    #[inline(always)]
    pub fn rxlastf(&self) -> RXLASTF_R {
        RXLASTF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - Frame complete flag (whatever the I3C acts as controller/target)
    #[inline(always)]
    pub fn fcf(&self) -> FCF_R {
        FCF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Target-initiated read end flag (when the I3C acts as controller)
    #[inline(always)]
    pub fn rxtgtendf(&self) -> RXTGTENDF_R {
        RXTGTENDF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Flag (whatever the I3C acts as controller/target)
    #[inline(always)]
    pub fn errf(&self) -> ERRF_R {
        ERRF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - IBI flag (when the I3C acts as controller)
    #[inline(always)]
    pub fn ibif(&self) -> IBIF_R {
        IBIF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - IBI end flag (when the I3C acts as target)
    #[inline(always)]
    pub fn ibiendf(&self) -> IBIENDF_R {
        IBIENDF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Controller-role request flag (when the I3C acts as controller)
    #[inline(always)]
    pub fn crf(&self) -> CRF_R {
        CRF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Controller-role update flag (when the I3C acts as target)
    #[inline(always)]
    pub fn crupdf(&self) -> CRUPDF_R {
        CRUPDF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Hot-join flag (when the I3C acts as controller)
    #[inline(always)]
    pub fn hjf(&self) -> HJF_R {
        HJF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - Wake-up/missed start flag (when the I3C acts as target)
    #[inline(always)]
    pub fn wkpf(&self) -> WKPF_R {
        WKPF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Get flag (when the I3C acts as target)
    #[inline(always)]
    pub fn getf(&self) -> GETF_R {
        GETF_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Get status flag (when the I3C acts as target)
    #[inline(always)]
    pub fn staf(&self) -> STAF_R {
        STAF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Dynamic address update flag (when the I3C acts as target)
    #[inline(always)]
    pub fn daupdf(&self) -> DAUPDF_R {
        DAUPDF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Maximum write length update flag (when the I3C acts as target)
    #[inline(always)]
    pub fn mwlupdf(&self) -> MWLUPDF_R {
        MWLUPDF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Maximum read length update flag (when the I3C acts as target)
    #[inline(always)]
    pub fn mrlupdf(&self) -> MRLUPDF_R {
        MRLUPDF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Reset pattern flag (when the I3C acts as target)
    #[inline(always)]
    pub fn rstf(&self) -> RSTF_R {
        RSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Activity state update flag (when the I3C acts as target)
    #[inline(always)]
    pub fn asupdf(&self) -> ASUPDF_R {
        ASUPDF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Interrupt/controller-role/hot-join update flag (when the I3C acts as target)
    #[inline(always)]
    pub fn intupdf(&self) -> INTUPDF_R {
        INTUPDF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - DEFTGTS flag (when the I3C acts as target)
    #[inline(always)]
    pub fn deff(&self) -> DEFF_R {
        DEFF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Group addressing flag (when the I3C acts as target)
    #[inline(always)]
    pub fn grpf(&self) -> GRPF_R {
        GRPF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVR")
            .field("cfef", &self.cfef())
            .field("txfef", &self.txfef())
            .field("cfnff", &self.cfnff())
            .field("sfnef", &self.sfnef())
            .field("txfnff", &self.txfnff())
            .field("rxfnef", &self.rxfnef())
            .field("txlastf", &self.txlastf())
            .field("rxlastf", &self.rxlastf())
            .field("fcf", &self.fcf())
            .field("rxtgtendf", &self.rxtgtendf())
            .field("errf", &self.errf())
            .field("ibif", &self.ibif())
            .field("ibiendf", &self.ibiendf())
            .field("crf", &self.crf())
            .field("crupdf", &self.crupdf())
            .field("hjf", &self.hjf())
            .field("wkpf", &self.wkpf())
            .field("getf", &self.getf())
            .field("staf", &self.staf())
            .field("daupdf", &self.daupdf())
            .field("mwlupdf", &self.mwlupdf())
            .field("mrlupdf", &self.mrlupdf())
            .field("rstf", &self.rstf())
            .field("asupdf", &self.asupdf())
            .field("intupdf", &self.intupdf())
            .field("deff", &self.deff())
            .field("grpf", &self.grpf())
            .finish()
    }
}
/**I3C event register

You can [`read`](crate::Reg::read) this register and get [`evr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#I3C1:EVR)*/
pub struct EVRrs;
impl crate::RegisterSpec for EVRrs {
    type Ux = u32;
}
///`read()` method returns [`evr::R`](R) reader structure
impl crate::Readable for EVRrs {}
///`reset()` method sets EVR to value 0x03
impl crate::Resettable for EVRrs {
    const RESET_VALUE: u32 = 0x03;
}
