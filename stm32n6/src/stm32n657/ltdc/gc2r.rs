///Register `GC2R` reader
pub type R = crate::R<GC2Rrs>;
///Field `BLA` reader - background layer ability (pixels of background layer are read from memory)
pub type BLA_R = crate::BitReader;
///Field `STSA` reader - slave timings synchronization ability
pub type STSA_R = crate::BitReader;
///Field `DVA` reader - dual-view ability
pub type DVA_R = crate::BitReader;
///Field `DPA` reader - secondary RGB output port ability
pub type DPA_R = crate::BitReader;
///Field `BW` reader - bus width (log2 of number of bytes)
pub type BW_R = crate::FieldReader;
///Field `EDCA` reader - external display control ability
pub type EDCA_R = crate::BitReader;
///Field `OCA` reader - output conversion ability (RGB to YCbCr)
pub type OCA_R = crate::BitReader;
///Field `AXIIDA` reader - AXIID ability
pub type AXIIDA_R = crate::BitReader;
///Field `ROTA` reader - rotation support ability
pub type ROTA_R = crate::BitReader;
///Field `SISA` reader - second interrupt set ability
pub type SISA_R = crate::BitReader;
///Field `SFA` reader - single frame mode ability
pub type SFA_R = crate::BitReader;
///Field `CRCA` reader - CRC ability
pub type CRCA_R = crate::BitReader;
///Field `BOA` reader - blending order ability
pub type BOA_R = crate::BitReader;
impl R {
    ///Bit 0 - background layer ability (pixels of background layer are read from memory)
    #[inline(always)]
    pub fn bla(&self) -> BLA_R {
        BLA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - slave timings synchronization ability
    #[inline(always)]
    pub fn stsa(&self) -> STSA_R {
        STSA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - dual-view ability
    #[inline(always)]
    pub fn dva(&self) -> DVA_R {
        DVA_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - secondary RGB output port ability
    #[inline(always)]
    pub fn dpa(&self) -> DPA_R {
        DPA_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - bus width (log2 of number of bytes)
    #[inline(always)]
    pub fn bw(&self) -> BW_R {
        BW_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - external display control ability
    #[inline(always)]
    pub fn edca(&self) -> EDCA_R {
        EDCA_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - output conversion ability (RGB to YCbCr)
    #[inline(always)]
    pub fn oca(&self) -> OCA_R {
        OCA_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - AXIID ability
    #[inline(always)]
    pub fn axiida(&self) -> AXIIDA_R {
        AXIIDA_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - rotation support ability
    #[inline(always)]
    pub fn rota(&self) -> ROTA_R {
        ROTA_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - second interrupt set ability
    #[inline(always)]
    pub fn sisa(&self) -> SISA_R {
        SISA_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - single frame mode ability
    #[inline(always)]
    pub fn sfa(&self) -> SFA_R {
        SFA_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CRC ability
    #[inline(always)]
    pub fn crca(&self) -> CRCA_R {
        CRCA_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - blending order ability
    #[inline(always)]
    pub fn boa(&self) -> BOA_R {
        BOA_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GC2R")
            .field("bla", &self.bla())
            .field("stsa", &self.stsa())
            .field("dva", &self.dva())
            .field("dpa", &self.dpa())
            .field("bw", &self.bw())
            .field("edca", &self.edca())
            .field("oca", &self.oca())
            .field("axiida", &self.axiida())
            .field("rota", &self.rota())
            .field("sisa", &self.sisa())
            .field("sfa", &self.sfa())
            .field("crca", &self.crca())
            .field("boa", &self.boa())
            .finish()
    }
}
/**LTDC global configuration 2 register

You can [`read`](crate::Reg::read) this register and get [`gc2r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#LTDC:GC2R)*/
pub struct GC2Rrs;
impl crate::RegisterSpec for GC2Rrs {
    type Ux = u32;
}
///`read()` method returns [`gc2r::R`](R) reader structure
impl crate::Readable for GC2Rrs {}
///`reset()` method sets GC2R to value 0xbb30
impl crate::Resettable for GC2Rrs {
    const RESET_VALUE: u32 = 0xbb30;
}
