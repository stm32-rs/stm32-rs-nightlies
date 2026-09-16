///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `TAMP1F` reader - TAMP1F
pub type TAMP1F_R = crate::BitReader;
///Field `TAMP2F` reader - TAMP2F
pub type TAMP2F_R = crate::BitReader;
///Field `ITAMP1F` reader - ITAMP1F
pub type ITAMP1F_R = crate::BitReader;
///Field `ITAMP3F` reader - ITAMP3F
pub type ITAMP3F_R = crate::BitReader;
///Field `ITAMP4F` reader - ITAMP4F
pub type ITAMP4F_R = crate::BitReader;
///Field `ITAMP5F` reader - ITAMP5F
pub type ITAMP5F_R = crate::BitReader;
///Field `ITAMP6F` reader - ITAMP6F
pub type ITAMP6F_R = crate::BitReader;
///Field `ITAMP7F` reader - ITAMP7F
pub type ITAMP7F_R = crate::BitReader;
impl R {
    ///Bit 0 - TAMP1F
    #[inline(always)]
    pub fn tamp1f(&self) -> TAMP1F_R {
        TAMP1F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TAMP2F
    #[inline(always)]
    pub fn tamp2f(&self) -> TAMP2F_R {
        TAMP2F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 16 - ITAMP1F
    #[inline(always)]
    pub fn itamp1f(&self) -> ITAMP1F_R {
        ITAMP1F_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - ITAMP3F
    #[inline(always)]
    pub fn itamp3f(&self) -> ITAMP3F_R {
        ITAMP3F_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - ITAMP4F
    #[inline(always)]
    pub fn itamp4f(&self) -> ITAMP4F_R {
        ITAMP4F_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - ITAMP5F
    #[inline(always)]
    pub fn itamp5f(&self) -> ITAMP5F_R {
        ITAMP5F_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ITAMP6F
    #[inline(always)]
    pub fn itamp6f(&self) -> ITAMP6F_R {
        ITAMP6F_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ITAMP7F
    #[inline(always)]
    pub fn itamp7f(&self) -> ITAMP7F_R {
        ITAMP7F_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("tamp1f", &self.tamp1f())
            .field("tamp2f", &self.tamp2f())
            .field("itamp1f", &self.itamp1f())
            .field("itamp3f", &self.itamp3f())
            .field("itamp4f", &self.itamp4f())
            .field("itamp5f", &self.itamp5f())
            .field("itamp6f", &self.itamp6f())
            .field("itamp7f", &self.itamp7f())
            .finish()
    }
}
/**TAMP status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#TAMP:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
