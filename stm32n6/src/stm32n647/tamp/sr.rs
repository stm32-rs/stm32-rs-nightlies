///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `TAMP1F` reader - TAMP1 detection flag
pub type TAMP1F_R = crate::BitReader;
///Field `TAMP2F` reader - TAMP2 detection flag
pub type TAMP2F_R = crate::BitReader;
///Field `TAMP3F` reader - TAMP3 detection flag
pub type TAMP3F_R = crate::BitReader;
///Field `TAMP4F` reader - TAMP4 detection flag
pub type TAMP4F_R = crate::BitReader;
///Field `TAMP5F` reader - TAMP5 detection flag
pub type TAMP5F_R = crate::BitReader;
///Field `TAMP6F` reader - TAMP6 detection flag
pub type TAMP6F_R = crate::BitReader;
///Field `TAMP7F` reader - TAMP7 detection flag
pub type TAMP7F_R = crate::BitReader;
///Field `ITAMP1F` reader - Internal tamper 1 flag
pub type ITAMP1F_R = crate::BitReader;
///Field `ITAMP2F` reader - Internal tamper 2 flag
pub type ITAMP2F_R = crate::BitReader;
///Field `ITAMP3F` reader - Internal tamper 3 flag
pub type ITAMP3F_R = crate::BitReader;
///Field `ITAMP4F` reader - Internal tamper 4 flag
pub type ITAMP4F_R = crate::BitReader;
///Field `ITAMP5F` reader - Internal tamper 5 flag
pub type ITAMP5F_R = crate::BitReader;
///Field `ITAMP6F` reader - Internal tamper 6 flag
pub type ITAMP6F_R = crate::BitReader;
///Field `ITAMP7F` reader - Internal tamper 7 flag
pub type ITAMP7F_R = crate::BitReader;
///Field `ITAMP8F` reader - Internal tamper 8 flag
pub type ITAMP8F_R = crate::BitReader;
///Field `ITAMP9F` reader - Internal tamper 9 flag
pub type ITAMP9F_R = crate::BitReader;
///Field `ITAMP11F` reader - Internal tamper 11 flag
pub type ITAMP11F_R = crate::BitReader;
impl R {
    ///Bit 0 - TAMP1 detection flag
    #[inline(always)]
    pub fn tamp1f(&self) -> TAMP1F_R {
        TAMP1F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TAMP2 detection flag
    #[inline(always)]
    pub fn tamp2f(&self) -> TAMP2F_R {
        TAMP2F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TAMP3 detection flag
    #[inline(always)]
    pub fn tamp3f(&self) -> TAMP3F_R {
        TAMP3F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TAMP4 detection flag
    #[inline(always)]
    pub fn tamp4f(&self) -> TAMP4F_R {
        TAMP4F_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TAMP5 detection flag
    #[inline(always)]
    pub fn tamp5f(&self) -> TAMP5F_R {
        TAMP5F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TAMP6 detection flag
    #[inline(always)]
    pub fn tamp6f(&self) -> TAMP6F_R {
        TAMP6F_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TAMP7 detection flag
    #[inline(always)]
    pub fn tamp7f(&self) -> TAMP7F_R {
        TAMP7F_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 16 - Internal tamper 1 flag
    #[inline(always)]
    pub fn itamp1f(&self) -> ITAMP1F_R {
        ITAMP1F_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Internal tamper 2 flag
    #[inline(always)]
    pub fn itamp2f(&self) -> ITAMP2F_R {
        ITAMP2F_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Internal tamper 3 flag
    #[inline(always)]
    pub fn itamp3f(&self) -> ITAMP3F_R {
        ITAMP3F_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Internal tamper 4 flag
    #[inline(always)]
    pub fn itamp4f(&self) -> ITAMP4F_R {
        ITAMP4F_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Internal tamper 5 flag
    #[inline(always)]
    pub fn itamp5f(&self) -> ITAMP5F_R {
        ITAMP5F_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Internal tamper 6 flag
    #[inline(always)]
    pub fn itamp6f(&self) -> ITAMP6F_R {
        ITAMP6F_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Internal tamper 7 flag
    #[inline(always)]
    pub fn itamp7f(&self) -> ITAMP7F_R {
        ITAMP7F_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Internal tamper 8 flag
    #[inline(always)]
    pub fn itamp8f(&self) -> ITAMP8F_R {
        ITAMP8F_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Internal tamper 9 flag
    #[inline(always)]
    pub fn itamp9f(&self) -> ITAMP9F_R {
        ITAMP9F_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - Internal tamper 11 flag
    #[inline(always)]
    pub fn itamp11f(&self) -> ITAMP11F_R {
        ITAMP11F_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("tamp1f", &self.tamp1f())
            .field("tamp2f", &self.tamp2f())
            .field("tamp3f", &self.tamp3f())
            .field("tamp4f", &self.tamp4f())
            .field("tamp5f", &self.tamp5f())
            .field("tamp6f", &self.tamp6f())
            .field("tamp7f", &self.tamp7f())
            .field("itamp1f", &self.itamp1f())
            .field("itamp2f", &self.itamp2f())
            .field("itamp3f", &self.itamp3f())
            .field("itamp4f", &self.itamp4f())
            .field("itamp5f", &self.itamp5f())
            .field("itamp6f", &self.itamp6f())
            .field("itamp7f", &self.itamp7f())
            .field("itamp8f", &self.itamp8f())
            .field("itamp9f", &self.itamp9f())
            .field("itamp11f", &self.itamp11f())
            .finish()
    }
}
/**TAMP status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TAMP:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
