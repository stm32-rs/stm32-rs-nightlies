///Register `S1FCR` reader
pub type R = crate::R<S1FCRrs>;
///Register `S1FCR` writer
pub type W = crate::W<S1FCRrs>;
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
        f.debug_struct("S1FCR")
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
    pub fn fth(&mut self) -> FTH_W<'_, S1FCRrs> {
        FTH_W::new(self, 0)
    }
    ///Bit 2 - DMDIS
    #[inline(always)]
    pub fn dmdis(&mut self) -> DMDIS_W<'_, S1FCRrs> {
        DMDIS_W::new(self, 2)
    }
    ///Bit 7 - FEIE
    #[inline(always)]
    pub fn feie(&mut self) -> FEIE_W<'_, S1FCRrs> {
        FEIE_W::new(self, 7)
    }
}
/**DMA stream 1 FIFO control register

You can [`read`](crate::Reg::read) this register and get [`s1fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s1fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DMA1:S1FCR)*/
pub struct S1FCRrs;
impl crate::RegisterSpec for S1FCRrs {
    type Ux = u32;
}
///`read()` method returns [`s1fcr::R`](R) reader structure
impl crate::Readable for S1FCRrs {}
///`write(|w| ..)` method takes [`s1fcr::W`](W) writer structure
impl crate::Writable for S1FCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets S1FCR to value 0x21
impl crate::Resettable for S1FCRrs {
    const RESET_VALUE: u32 = 0x21;
}
