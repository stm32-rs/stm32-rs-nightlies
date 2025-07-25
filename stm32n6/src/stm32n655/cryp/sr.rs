///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `IFEM` reader - Input FIFO empty flag
pub type IFEM_R = crate::BitReader;
///Field `IFNF` reader - Input FIFO not full flag
pub type IFNF_R = crate::BitReader;
///Field `OFNE` reader - Output FIFO not empty flag
pub type OFNE_R = crate::BitReader;
///Field `OFFU` reader - Output FIFO full flag
pub type OFFU_R = crate::BitReader;
///Field `BUSY` reader - Busy bit
pub type BUSY_R = crate::BitReader;
///Field `KERF` reader - Key error flag
pub type KERF_R = crate::BitReader;
///Field `KEYVALID` reader - Key valid flag
pub type KEYVALID_R = crate::BitReader;
impl R {
    ///Bit 0 - Input FIFO empty flag
    #[inline(always)]
    pub fn ifem(&self) -> IFEM_R {
        IFEM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Input FIFO not full flag
    #[inline(always)]
    pub fn ifnf(&self) -> IFNF_R {
        IFNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Output FIFO not empty flag
    #[inline(always)]
    pub fn ofne(&self) -> OFNE_R {
        OFNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Output FIFO full flag
    #[inline(always)]
    pub fn offu(&self) -> OFFU_R {
        OFFU_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Busy bit
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Key error flag
    #[inline(always)]
    pub fn kerf(&self) -> KERF_R {
        KERF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Key valid flag
    #[inline(always)]
    pub fn keyvalid(&self) -> KEYVALID_R {
        KEYVALID_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("ifem", &self.ifem())
            .field("ifnf", &self.ifnf())
            .field("ofne", &self.ofne())
            .field("offu", &self.offu())
            .field("busy", &self.busy())
            .field("kerf", &self.kerf())
            .field("keyvalid", &self.keyvalid())
            .finish()
    }
}
/**CRYP status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0x03
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x03;
}
