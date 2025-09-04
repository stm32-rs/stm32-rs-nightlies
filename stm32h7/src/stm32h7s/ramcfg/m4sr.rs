///Register `M4SR` reader
pub type R = crate::R<M4SRrs>;
///Register `M4SR` writer
pub type W = crate::W<M4SRrs>;
///Field `SEDCF` reader - ECC single error detected and corrected flag This bit is set by hardware. It is cleared by software by writing a 0
pub type SEDCF_R = crate::BitReader;
///Field `SEDCF` writer - ECC single error detected and corrected flag This bit is set by hardware. It is cleared by software by writing a 0
pub type SEDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEDF` reader - ECC double error detected flag This bit is set by hardware. It is cleared by software by writing a 0
pub type DEDF_R = crate::BitReader;
///Field `DEDF` writer - ECC double error detected flag This bit is set by hardware. It is cleared by software by writing a 0
pub type DEDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEBWDF` reader - ECC double error on byte write (BW) detected flag This bit is set by hardware. It is cleared by software by writing a 0
pub type DEBWDF_R = crate::BitReader;
///Field `DEBWDF` writer - ECC double error on byte write (BW) detected flag This bit is set by hardware. It is cleared by software by writing a 0
pub type DEBWDF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ECC single error detected and corrected flag This bit is set by hardware. It is cleared by software by writing a 0
    #[inline(always)]
    pub fn sedcf(&self) -> SEDCF_R {
        SEDCF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ECC double error detected flag This bit is set by hardware. It is cleared by software by writing a 0
    #[inline(always)]
    pub fn dedf(&self) -> DEDF_R {
        DEDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ECC double error on byte write (BW) detected flag This bit is set by hardware. It is cleared by software by writing a 0
    #[inline(always)]
    pub fn debwdf(&self) -> DEBWDF_R {
        DEBWDF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M4SR")
            .field("sedcf", &self.sedcf())
            .field("dedf", &self.dedf())
            .field("debwdf", &self.debwdf())
            .finish()
    }
}
impl W {
    ///Bit 0 - ECC single error detected and corrected flag This bit is set by hardware. It is cleared by software by writing a 0
    #[inline(always)]
    pub fn sedcf(&mut self) -> SEDCF_W<M4SRrs> {
        SEDCF_W::new(self, 0)
    }
    ///Bit 1 - ECC double error detected flag This bit is set by hardware. It is cleared by software by writing a 0
    #[inline(always)]
    pub fn dedf(&mut self) -> DEDF_W<M4SRrs> {
        DEDF_W::new(self, 1)
    }
    ///Bit 2 - ECC double error on byte write (BW) detected flag This bit is set by hardware. It is cleared by software by writing a 0
    #[inline(always)]
    pub fn debwdf(&mut self) -> DEBWDF_W<M4SRrs> {
        DEBWDF_W::new(self, 2)
    }
}
/**RAMECC monitor 4 status register

You can [`read`](crate::Reg::read) this register and get [`m4sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m4sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RAMCFG:M4SR)*/
pub struct M4SRrs;
impl crate::RegisterSpec for M4SRrs {
    type Ux = u32;
}
///`read()` method returns [`m4sr::R`](R) reader structure
impl crate::Readable for M4SRrs {}
///`write(|w| ..)` method takes [`m4sr::W`](W) writer structure
impl crate::Writable for M4SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M4SR to value 0
impl crate::Resettable for M4SRrs {}
