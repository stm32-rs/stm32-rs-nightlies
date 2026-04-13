///Register `FLT2JCHGR` reader
pub type R = crate::R<FLT2JCHGRrs>;
///Register `FLT2JCHGR` writer
pub type W = crate::W<FLT2JCHGRrs>;
///Field `JCHG` reader - JCHG
pub type JCHG_R = crate::FieldReader;
///Field `JCHG` writer - JCHG
pub type JCHG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - JCHG
    #[inline(always)]
    pub fn jchg(&self) -> JCHG_R {
        JCHG_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLT2JCHGR")
            .field("jchg", &self.jchg())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - JCHG
    #[inline(always)]
    pub fn jchg(&mut self) -> JCHG_W<'_, FLT2JCHGRrs> {
        JCHG_W::new(self, 0)
    }
}
/**DFSDM filter 2 injected channel group selection register

You can [`read`](crate::Reg::read) this register and get [`flt2jchgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt2jchgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT2JCHGR)*/
pub struct FLT2JCHGRrs;
impl crate::RegisterSpec for FLT2JCHGRrs {
    type Ux = u32;
}
///`read()` method returns [`flt2jchgr::R`](R) reader structure
impl crate::Readable for FLT2JCHGRrs {}
///`write(|w| ..)` method takes [`flt2jchgr::W`](W) writer structure
impl crate::Writable for FLT2JCHGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLT2JCHGR to value 0x01
impl crate::Resettable for FLT2JCHGRrs {
    const RESET_VALUE: u32 = 0x01;
}
