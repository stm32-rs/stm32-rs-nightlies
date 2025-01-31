///Register `M1CR` reader
pub type R = crate::R<M1CRrs>;
///Register `M1CR` writer
pub type W = crate::W<M1CRrs>;
///Field `ECCSEIE` reader - ECC single error interrupt enable
pub type ECCSEIE_R = crate::BitReader;
///Field `ECCSEIE` writer - ECC single error interrupt enable
pub type ECCSEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCDEIE` reader - ECC double error interrupt enable
pub type ECCDEIE_R = crate::BitReader;
///Field `ECCDEIE` writer - ECC double error interrupt enable
pub type ECCDEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCDEBWIE` reader - ECC double error on byte write interrupt enable
pub type ECCDEBWIE_R = crate::BitReader;
///Field `ECCDEBWIE` writer - ECC double error on byte write interrupt enable
pub type ECCDEBWIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCELEN` reader - ECC error context latching enable
pub type ECCELEN_R = crate::BitReader;
///Field `ECCELEN` writer - ECC error context latching enable
pub type ECCELEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
        f.debug_struct("M1CR")
            .field("eccelen", &self.eccelen())
            .field("eccdebwie", &self.eccdebwie())
            .field("eccdeie", &self.eccdeie())
            .field("eccseie", &self.eccseie())
            .finish()
    }
}
impl W {
    ///Bit 2 - ECC single error interrupt enable
    #[inline(always)]
    pub fn eccseie(&mut self) -> ECCSEIE_W<M1CRrs> {
        ECCSEIE_W::new(self, 2)
    }
    ///Bit 3 - ECC double error interrupt enable
    #[inline(always)]
    pub fn eccdeie(&mut self) -> ECCDEIE_W<M1CRrs> {
        ECCDEIE_W::new(self, 3)
    }
    ///Bit 4 - ECC double error on byte write interrupt enable
    #[inline(always)]
    pub fn eccdebwie(&mut self) -> ECCDEBWIE_W<M1CRrs> {
        ECCDEBWIE_W::new(self, 4)
    }
    ///Bit 5 - ECC error context latching enable
    #[inline(always)]
    pub fn eccelen(&mut self) -> ECCELEN_W<M1CRrs> {
        ECCELEN_W::new(self, 5)
    }
}
/**RAMECC monitor x configuration register

You can [`read`](crate::Reg::read) this register and get [`m1cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#RAMECC2:M1CR)*/
pub struct M1CRrs;
impl crate::RegisterSpec for M1CRrs {
    type Ux = u32;
}
///`read()` method returns [`m1cr::R`](R) reader structure
impl crate::Readable for M1CRrs {}
///`write(|w| ..)` method takes [`m1cr::W`](W) writer structure
impl crate::Writable for M1CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets M1CR to value 0
impl crate::Resettable for M1CRrs {
    const RESET_VALUE: u32 = 0;
}
