///Register `S5FCR` reader
pub type R = crate::R<S5FCRrs>;
///Register `S5FCR` writer
pub type W = crate::W<S5FCRrs>;
///Field `FTH` reader - FIFO threshold selection
pub type FTH_R = crate::FieldReader;
///Field `FTH` writer - FIFO threshold selection
pub type FTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DMDIS` reader - Direct mode disable
pub type DMDIS_R = crate::BitReader;
///Field `DMDIS` writer - Direct mode disable
pub type DMDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FS` reader - FIFO status
pub type FS_R = crate::FieldReader;
///Field `FEIE` reader - FIFO error interrupt enable
pub type FEIE_R = crate::BitReader;
///Field `FEIE` writer - FIFO error interrupt enable
pub type FEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - FIFO threshold selection
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Direct mode disable
    #[inline(always)]
    pub fn dmdis(&self) -> DMDIS_R {
        DMDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:5 - FIFO status
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bit 7 - FIFO error interrupt enable
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S5FCR")
            .field("feie", &self.feie())
            .field("fs", &self.fs())
            .field("dmdis", &self.dmdis())
            .field("fth", &self.fth())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - FIFO threshold selection
    #[inline(always)]
    pub fn fth(&mut self) -> FTH_W<'_, S5FCRrs> {
        FTH_W::new(self, 0)
    }
    ///Bit 2 - Direct mode disable
    #[inline(always)]
    pub fn dmdis(&mut self) -> DMDIS_W<'_, S5FCRrs> {
        DMDIS_W::new(self, 2)
    }
    ///Bit 7 - FIFO error interrupt enable
    #[inline(always)]
    pub fn feie(&mut self) -> FEIE_W<'_, S5FCRrs> {
        FEIE_W::new(self, 7)
    }
}
/**stream x FIFO control register

You can [`read`](crate::Reg::read) this register and get [`s5fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s5fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#DMA2:S5FCR)*/
pub struct S5FCRrs;
impl crate::RegisterSpec for S5FCRrs {
    type Ux = u32;
}
///`read()` method returns [`s5fcr::R`](R) reader structure
impl crate::Readable for S5FCRrs {}
///`write(|w| ..)` method takes [`s5fcr::W`](W) writer structure
impl crate::Writable for S5FCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets S5FCR to value 0x21
impl crate::Resettable for S5FCRrs {
    const RESET_VALUE: u32 = 0x21;
}
