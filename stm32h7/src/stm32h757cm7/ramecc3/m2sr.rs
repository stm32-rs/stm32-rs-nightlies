///Register `M2SR` reader
pub type R = crate::R<M2SRrs>;
///Register `M2SR` writer
pub type W = crate::W<M2SRrs>;
///Field `SEDCF` reader - ECC single error detected flag
pub type SEDCF_R = crate::BitReader;
///Field `SEDCF` writer - ECC single error detected flag
pub type SEDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEDF` reader - ECC double error detected flag
pub type DEDF_R = crate::BitReader;
///Field `DEDF` writer - ECC double error detected flag
pub type DEDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEBWDF` reader - ECC double error on byte write flag
pub type DEBWDF_R = crate::BitReader;
///Field `DEBWDF` writer - ECC double error on byte write flag
pub type DEBWDF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ECC single error detected flag
    #[inline(always)]
    pub fn sedcf(&self) -> SEDCF_R {
        SEDCF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ECC double error detected flag
    #[inline(always)]
    pub fn dedf(&self) -> DEDF_R {
        DEDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ECC double error on byte write flag
    #[inline(always)]
    pub fn debwdf(&self) -> DEBWDF_R {
        DEBWDF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M2SR")
            .field("debwdf", &self.debwdf())
            .field("dedf", &self.dedf())
            .field("sedcf", &self.sedcf())
            .finish()
    }
}
impl W {
    ///Bit 0 - ECC single error detected flag
    #[inline(always)]
    pub fn sedcf(&mut self) -> SEDCF_W<M2SRrs> {
        SEDCF_W::new(self, 0)
    }
    ///Bit 1 - ECC double error detected flag
    #[inline(always)]
    pub fn dedf(&mut self) -> DEDF_W<M2SRrs> {
        DEDF_W::new(self, 1)
    }
    ///Bit 2 - ECC double error on byte write flag
    #[inline(always)]
    pub fn debwdf(&mut self) -> DEBWDF_W<M2SRrs> {
        DEBWDF_W::new(self, 2)
    }
}
/**RAMECC monitor 2 status register

You can [`read`](crate::Reg::read) this register and get [`m2sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#RAMECC3:M2SR)*/
pub struct M2SRrs;
impl crate::RegisterSpec for M2SRrs {
    type Ux = u32;
}
///`read()` method returns [`m2sr::R`](R) reader structure
impl crate::Readable for M2SRrs {}
///`write(|w| ..)` method takes [`m2sr::W`](W) writer structure
impl crate::Writable for M2SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M2SR to value 0
impl crate::Resettable for M2SRrs {}
