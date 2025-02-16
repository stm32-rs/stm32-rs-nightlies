///Register `M2CR` reader
pub type R = crate::R<M2CRrs>;
///Register `M2CR` writer
pub type W = crate::W<M2CRrs>;
///Field `SEDCF` reader - ECC single error detected and corrected flag
pub type SEDCF_R = crate::BitReader;
///Field `SEDCF` writer - ECC single error detected and corrected flag
pub type SEDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEDF` reader - ECC double error detected flag
pub type DEDF_R = crate::BitReader;
///Field `DEDF` writer - ECC double error detected flag
pub type DEDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEBWDF` reader - ECC double error on byte write (BW) detected flag
pub type DEBWDF_R = crate::BitReader;
///Field `DEBWDF` writer - ECC double error on byte write (BW) detected flag
pub type DEBWDF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ECC single error detected and corrected flag
    #[inline(always)]
    pub fn sedcf(&self) -> SEDCF_R {
        SEDCF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ECC double error detected flag
    #[inline(always)]
    pub fn dedf(&self) -> DEDF_R {
        DEDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ECC double error on byte write (BW) detected flag
    #[inline(always)]
    pub fn debwdf(&self) -> DEBWDF_R {
        DEBWDF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M2CR")
            .field("sedcf", &self.sedcf())
            .field("dedf", &self.dedf())
            .field("debwdf", &self.debwdf())
            .finish()
    }
}
impl W {
    ///Bit 0 - ECC single error detected and corrected flag
    #[inline(always)]
    pub fn sedcf(&mut self) -> SEDCF_W<M2CRrs> {
        SEDCF_W::new(self, 0)
    }
    ///Bit 1 - ECC double error detected flag
    #[inline(always)]
    pub fn dedf(&mut self) -> DEDF_W<M2CRrs> {
        DEDF_W::new(self, 1)
    }
    ///Bit 2 - ECC double error on byte write (BW) detected flag
    #[inline(always)]
    pub fn debwdf(&mut self) -> DEBWDF_W<M2CRrs> {
        DEBWDF_W::new(self, 2)
    }
}
/**RAMECC monitor x configuration register

You can [`read`](crate::Reg::read) this register and get [`m2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#RAMECC:M2CR)*/
pub struct M2CRrs;
impl crate::RegisterSpec for M2CRrs {
    type Ux = u32;
}
///`read()` method returns [`m2cr::R`](R) reader structure
impl crate::Readable for M2CRrs {}
///`write(|w| ..)` method takes [`m2cr::W`](W) writer structure
impl crate::Writable for M2CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets M2CR to value 0
impl crate::Resettable for M2CRrs {
    const RESET_VALUE: u32 = 0;
}
