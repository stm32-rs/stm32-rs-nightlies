///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `IFTF` reader - Input FIFO threshold flag
pub type IFTF_R = crate::BitReader;
///Field `IFNFF` reader - Input FIFO not full flag
pub type IFNFF_R = crate::BitReader;
///Field `OFTF` reader - Output FIFO threshold flag
pub type OFTF_R = crate::BitReader;
///Field `OFNEF` reader - Output FIFO not empty flag
pub type OFNEF_R = crate::BitReader;
///Field `EOCF` reader - End of conversion flag
pub type EOCF_R = crate::BitReader;
///Field `HPDF` reader - Header parsing done flag
pub type HPDF_R = crate::BitReader;
///Field `COF` reader - Codec operation flag
pub type COF_R = crate::BitReader;
impl R {
    ///Bit 1 - Input FIFO threshold flag
    #[inline(always)]
    pub fn iftf(&self) -> IFTF_R {
        IFTF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Input FIFO not full flag
    #[inline(always)]
    pub fn ifnff(&self) -> IFNFF_R {
        IFNFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Output FIFO threshold flag
    #[inline(always)]
    pub fn oftf(&self) -> OFTF_R {
        OFTF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Output FIFO not empty flag
    #[inline(always)]
    pub fn ofnef(&self) -> OFNEF_R {
        OFNEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - End of conversion flag
    #[inline(always)]
    pub fn eocf(&self) -> EOCF_R {
        EOCF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Header parsing done flag
    #[inline(always)]
    pub fn hpdf(&self) -> HPDF_R {
        HPDF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Codec operation flag
    #[inline(always)]
    pub fn cof(&self) -> COF_R {
        COF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("iftf", &self.iftf())
            .field("ifnff", &self.ifnff())
            .field("oftf", &self.oftf())
            .field("ofnef", &self.ofnef())
            .field("eocf", &self.eocf())
            .field("hpdf", &self.hpdf())
            .field("cof", &self.cof())
            .finish()
    }
}
/**JPEG status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0x06
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x06;
}
