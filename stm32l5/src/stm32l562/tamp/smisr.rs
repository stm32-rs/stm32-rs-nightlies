///Register `SMISR` reader
pub type R = crate::R<SMISRrs>;
///Field `TAMP1MF` reader - TAMP1MF:
pub type TAMP1MF_R = crate::BitReader;
///Field `TAMP2MF` reader - TAMP2MF
pub type TAMP2MF_R = crate::BitReader;
///Field `TAMP3MF` reader - TAMP3MF
pub type TAMP3MF_R = crate::BitReader;
///Field `TAMP4MF` reader - TAMP4MF
pub type TAMP4MF_R = crate::BitReader;
///Field `TAMP5MF` reader - TAMP5MF
pub type TAMP5MF_R = crate::BitReader;
///Field `TAMP6MF` reader - TAMP6MF
pub type TAMP6MF_R = crate::BitReader;
///Field `TAMP7MF` reader - TAMP7MF:
pub type TAMP7MF_R = crate::BitReader;
///Field `TAMP8MF` reader - TAMP8MF
pub type TAMP8MF_R = crate::BitReader;
///Field `ITAMP1MF` reader - ITAMP1MF
pub type ITAMP1MF_R = crate::BitReader;
///Field `ITAMP2MF` reader - ITAMP2MF
pub type ITAMP2MF_R = crate::BitReader;
///Field `ITAMP3MF` reader - ITAMP3MF
pub type ITAMP3MF_R = crate::BitReader;
///Field `ITAMP5MF` reader - ITAMP5MF
pub type ITAMP5MF_R = crate::BitReader;
///Field `ITAMP8MF` reader - ITAMP8MF
pub type ITAMP8MF_R = crate::BitReader;
impl R {
    ///Bit 0 - TAMP1MF:
    #[inline(always)]
    pub fn tamp1mf(&self) -> TAMP1MF_R {
        TAMP1MF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TAMP2MF
    #[inline(always)]
    pub fn tamp2mf(&self) -> TAMP2MF_R {
        TAMP2MF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TAMP3MF
    #[inline(always)]
    pub fn tamp3mf(&self) -> TAMP3MF_R {
        TAMP3MF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TAMP4MF
    #[inline(always)]
    pub fn tamp4mf(&self) -> TAMP4MF_R {
        TAMP4MF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TAMP5MF
    #[inline(always)]
    pub fn tamp5mf(&self) -> TAMP5MF_R {
        TAMP5MF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TAMP6MF
    #[inline(always)]
    pub fn tamp6mf(&self) -> TAMP6MF_R {
        TAMP6MF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TAMP7MF:
    #[inline(always)]
    pub fn tamp7mf(&self) -> TAMP7MF_R {
        TAMP7MF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TAMP8MF
    #[inline(always)]
    pub fn tamp8mf(&self) -> TAMP8MF_R {
        TAMP8MF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - ITAMP1MF
    #[inline(always)]
    pub fn itamp1mf(&self) -> ITAMP1MF_R {
        ITAMP1MF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ITAMP2MF
    #[inline(always)]
    pub fn itamp2mf(&self) -> ITAMP2MF_R {
        ITAMP2MF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ITAMP3MF
    #[inline(always)]
    pub fn itamp3mf(&self) -> ITAMP3MF_R {
        ITAMP3MF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - ITAMP5MF
    #[inline(always)]
    pub fn itamp5mf(&self) -> ITAMP5MF_R {
        ITAMP5MF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 23 - ITAMP8MF
    #[inline(always)]
    pub fn itamp8mf(&self) -> ITAMP8MF_R {
        ITAMP8MF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMISR")
            .field("tamp1mf", &self.tamp1mf())
            .field("tamp2mf", &self.tamp2mf())
            .field("tamp3mf", &self.tamp3mf())
            .field("tamp4mf", &self.tamp4mf())
            .field("tamp5mf", &self.tamp5mf())
            .field("tamp6mf", &self.tamp6mf())
            .field("tamp7mf", &self.tamp7mf())
            .field("tamp8mf", &self.tamp8mf())
            .field("itamp1mf", &self.itamp1mf())
            .field("itamp2mf", &self.itamp2mf())
            .field("itamp3mf", &self.itamp3mf())
            .field("itamp5mf", &self.itamp5mf())
            .field("itamp8mf", &self.itamp8mf())
            .finish()
    }
}
/**TAMP secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`smisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#TAMP:SMISR)*/
pub struct SMISRrs;
impl crate::RegisterSpec for SMISRrs {
    type Ux = u32;
}
///`read()` method returns [`smisr::R`](R) reader structure
impl crate::Readable for SMISRrs {}
///`reset()` method sets SMISR to value 0
impl crate::Resettable for SMISRrs {}
