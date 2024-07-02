///Register `DFSDM_FLT1CHGR` reader
pub type R = crate::R<DFSDM_FLT1CHGRrs>;
///Register `DFSDM_FLT1CHGR` writer
pub type W = crate::W<DFSDM_FLT1CHGRrs>;
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
        f.debug_struct("DFSDM_FLT1CHGR")
            .field("jchg", &self.jchg())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Injected channel group selection
    #[inline(always)]
    #[must_use]
    pub fn jchg(&mut self) -> JCHG_W<DFSDM_FLT1CHGRrs> {
        JCHG_W::new(self, 0)
    }
}
/**injected channel group selection register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1chgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1chgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#DFSDM1:DFSDM_FLT1CHGR)*/
pub struct DFSDM_FLT1CHGRrs;
impl crate::RegisterSpec for DFSDM_FLT1CHGRrs {
    type Ux = u32;
}
///`read()` method returns [`dfsdm_flt1chgr::R`](R) reader structure
impl crate::Readable for DFSDM_FLT1CHGRrs {}
///`write(|w| ..)` method takes [`dfsdm_flt1chgr::W`](W) writer structure
impl crate::Writable for DFSDM_FLT1CHGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DFSDM_FLT1CHGR to value 0x01
impl crate::Resettable for DFSDM_FLT1CHGRrs {
    const RESET_VALUE: u32 = 0x01;
}
