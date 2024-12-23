///Register `DFSDM3_JCHGR` reader
pub type R = crate::R<DFSDM3_JCHGRrs>;
///Register `DFSDM3_JCHGR` writer
pub type W = crate::W<DFSDM3_JCHGRrs>;
///Field `JCHG` reader - Injected channel group selection
pub type JCHG_R = crate::FieldReader;
///Field `JCHG` writer - Injected channel group selection
pub type JCHG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Injected channel group selection
    #[inline(always)]
    pub fn jchg(&self) -> JCHG_R {
        JCHG_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFSDM3_JCHGR")
            .field("jchg", &self.jchg())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Injected channel group selection
    #[inline(always)]
    pub fn jchg(&mut self) -> JCHG_W<DFSDM3_JCHGRrs> {
        JCHG_W::new(self, 0)
    }
}
/**DFSDM injected channel group selection register

You can [`read`](crate::Reg::read) this register and get [`dfsdm3_jchgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm3_jchgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F765.html#DFSDM1:DFSDM3_JCHGR)*/
pub struct DFSDM3_JCHGRrs;
impl crate::RegisterSpec for DFSDM3_JCHGRrs {
    type Ux = u32;
}
///`read()` method returns [`dfsdm3_jchgr::R`](R) reader structure
impl crate::Readable for DFSDM3_JCHGRrs {}
///`write(|w| ..)` method takes [`dfsdm3_jchgr::W`](W) writer structure
impl crate::Writable for DFSDM3_JCHGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DFSDM3_JCHGR to value 0x01
impl crate::Resettable for DFSDM3_JCHGRrs {
    const RESET_VALUE: u32 = 0x01;
}
