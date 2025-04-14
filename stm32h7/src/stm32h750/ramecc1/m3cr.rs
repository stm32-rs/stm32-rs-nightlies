///Register `M3CR` reader
pub type R = crate::R<M3CRrs>;
///Field `ECCSEIE` reader - ECC single error interrupt enable
pub type ECCSEIE_R = crate::BitReader;
///Field `ECCDEIE` reader - ECC double error interrupt enable
pub type ECCDEIE_R = crate::BitReader;
///Field `ECCDEBWIE` reader - ECC double error on byte write interrupt enable
pub type ECCDEBWIE_R = crate::BitReader;
///Field `ECCELEN` reader - ECC error context latching enable
pub type ECCELEN_R = crate::BitReader;
impl R {
    ///Bit 2 - ECC single error interrupt enable
    #[inline(always)]
    pub fn eccseie(&self) -> ECCSEIE_R {
        ECCSEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ECC double error interrupt enable
    #[inline(always)]
    pub fn eccdeie(&self) -> ECCDEIE_R {
        ECCDEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ECC double error on byte write interrupt enable
    #[inline(always)]
    pub fn eccdebwie(&self) -> ECCDEBWIE_R {
        ECCDEBWIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ECC error context latching enable
    #[inline(always)]
    pub fn eccelen(&self) -> ECCELEN_R {
        ECCELEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M3CR")
            .field("eccelen", &self.eccelen())
            .field("eccdebwie", &self.eccdebwie())
            .field("eccdeie", &self.eccdeie())
            .field("eccseie", &self.eccseie())
            .finish()
    }
}
/**RAMECC monitor x configuration register

You can [`read`](crate::Reg::read) this register and get [`m3cr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#RAMECC1:M3CR)*/
pub struct M3CRrs;
impl crate::RegisterSpec for M3CRrs {
    type Ux = u32;
}
///`read()` method returns [`m3cr::R`](R) reader structure
impl crate::Readable for M3CRrs {}
///`reset()` method sets M3CR to value 0
impl crate::Resettable for M3CRrs {}
