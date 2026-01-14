///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
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
        f.debug_struct("SR")
            .field("sedcf", &self.sedcf())
            .field("dedf", &self.dedf())
            .field("debwdf", &self.debwdf())
            .finish()
    }
}
impl W {
    ///Bit 0 - ECC single error detected and corrected flag
    #[inline(always)]
    pub fn sedcf(&mut self) -> SEDCF_W<'_, SRrs> {
        SEDCF_W::new(self, 0)
    }
    ///Bit 1 - ECC double error detected flag
    #[inline(always)]
    pub fn dedf(&mut self) -> DEDF_W<'_, SRrs> {
        DEDF_W::new(self, 1)
    }
    ///Bit 2 - ECC double error on byte write (BW) detected flag
    #[inline(always)]
    pub fn debwdf(&mut self) -> DEBWDF_W<'_, SRrs> {
        DEBWDF_W::new(self, 2)
    }
}
/**RAMECC monitor x status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
