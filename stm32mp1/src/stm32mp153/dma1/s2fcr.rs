///Register `S2FCR` reader
pub type R = crate::R<S2FCRrs>;
///Register `S2FCR` writer
pub type W = crate::W<S2FCRrs>;
///Field `FTH` reader - FTH
pub type FTH_R = crate::FieldReader;
///Field `FTH` writer - FTH
pub type FTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DMDIS` reader - DMDIS
pub type DMDIS_R = crate::BitReader;
///Field `DMDIS` writer - DMDIS
pub type DMDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FS` reader - FS
pub type FS_R = crate::FieldReader;
///Field `FEIE` reader - FEIE
pub type FEIE_R = crate::BitReader;
///Field `FEIE` writer - FEIE
pub type FEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - FTH
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - DMDIS
    #[inline(always)]
    pub fn dmdis(&self) -> DMDIS_R {
        DMDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:5 - FS
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bit 7 - FEIE
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S2FCR")
            .field("fth", &self.fth())
            .field("dmdis", &self.dmdis())
            .field("fs", &self.fs())
            .field("feie", &self.feie())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - FTH
    #[inline(always)]
    pub fn fth(&mut self) -> FTH_W<S2FCRrs> {
        FTH_W::new(self, 0)
    }
    ///Bit 2 - DMDIS
    #[inline(always)]
    pub fn dmdis(&mut self) -> DMDIS_W<S2FCRrs> {
        DMDIS_W::new(self, 2)
    }
    ///Bit 7 - FEIE
    #[inline(always)]
    pub fn feie(&mut self) -> FEIE_W<S2FCRrs> {
        FEIE_W::new(self, 7)
    }
}
/**DMA stream 2 FIFO control register

You can [`read`](crate::Reg::read) this register and get [`s2fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMA1:S2FCR)*/
pub struct S2FCRrs;
impl crate::RegisterSpec for S2FCRrs {
    type Ux = u32;
}
///`read()` method returns [`s2fcr::R`](R) reader structure
impl crate::Readable for S2FCRrs {}
///`write(|w| ..)` method takes [`s2fcr::W`](W) writer structure
impl crate::Writable for S2FCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets S2FCR to value 0x21
impl crate::Resettable for S2FCRrs {
    const RESET_VALUE: u32 = 0x21;
}
