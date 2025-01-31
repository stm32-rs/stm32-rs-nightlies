///Register `M1SR` reader
pub type R = crate::R<M1SRrs>;
///Register `M1SR` writer
pub type W = crate::W<M1SRrs>;
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
        f.debug_struct("M1SR")
            .field("debwdf", &self.debwdf())
            .field("dedf", &self.dedf())
            .field("sedcf", &self.sedcf())
            .finish()
    }
}
impl W {
    ///Bit 0 - ECC single error detected flag
    #[inline(always)]
    pub fn sedcf(&mut self) -> SEDCF_W<M1SRrs> {
        SEDCF_W::new(self, 0)
    }
    ///Bit 1 - ECC double error detected flag
    #[inline(always)]
    pub fn dedf(&mut self) -> DEDF_W<M1SRrs> {
        DEDF_W::new(self, 1)
    }
    ///Bit 2 - ECC double error on byte write flag
    #[inline(always)]
    pub fn debwdf(&mut self) -> DEBWDF_W<M1SRrs> {
        DEBWDF_W::new(self, 2)
    }
}
/**RAMECC monitor x status register

You can [`read`](crate::Reg::read) this register and get [`m1sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#RAMECC3:M1SR)*/
pub struct M1SRrs;
impl crate::RegisterSpec for M1SRrs {
    type Ux = u32;
}
///`read()` method returns [`m1sr::R`](R) reader structure
impl crate::Readable for M1SRrs {}
///`write(|w| ..)` method takes [`m1sr::W`](W) writer structure
impl crate::Writable for M1SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets M1SR to value 0
impl crate::Resettable for M1SRrs {
    const RESET_VALUE: u32 = 0;
}
