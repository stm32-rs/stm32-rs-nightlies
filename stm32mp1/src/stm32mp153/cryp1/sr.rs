///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `IFEM` reader - IFEM
pub type IFEM_R = crate::BitReader;
///Field `IFNF` reader - IFNF
pub type IFNF_R = crate::BitReader;
///Field `OFNE` reader - OFNE
pub type OFNE_R = crate::BitReader;
///Field `OFFU` reader - OFFU
pub type OFFU_R = crate::BitReader;
///Field `BUSY` reader - BUSY
pub type BUSY_R = crate::BitReader;
impl R {
    ///Bit 0 - IFEM
    #[inline(always)]
    pub fn ifem(&self) -> IFEM_R {
        IFEM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IFNF
    #[inline(always)]
    pub fn ifnf(&self) -> IFNF_R {
        IFNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - OFNE
    #[inline(always)]
    pub fn ofne(&self) -> OFNE_R {
        OFNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - OFFU
    #[inline(always)]
    pub fn offu(&self) -> OFFU_R {
        OFFU_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - BUSY
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 4) & 1) != 0)
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
            .finish()
    }
}
/**CRYP status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#CRYP1:SR)*/
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
