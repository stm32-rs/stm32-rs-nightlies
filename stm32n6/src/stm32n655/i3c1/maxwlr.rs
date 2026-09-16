///Register `MAXWLR` reader
pub type R = crate::R<MAXWLRrs>;
///Register `MAXWLR` writer
pub type W = crate::W<MAXWLRrs>;
///Field `MWL` reader - Maximum data write length (when I3C acts as target)
pub type MWL_R = crate::FieldReader<u16>;
///Field `MWL` writer - Maximum data write length (when I3C acts as target)
pub type MWL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Maximum data write length (when I3C acts as target)
    #[inline(always)]
    pub fn mwl(&self) -> MWL_R {
        MWL_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAXWLR").field("mwl", &self.mwl()).finish()
    }
}
impl W {
    ///Bits 0:15 - Maximum data write length (when I3C acts as target)
    #[inline(always)]
    pub fn mwl(&mut self) -> MWL_W<'_, MAXWLRrs> {
        MWL_W::new(self, 0)
    }
}
/**I3C maximum write length register

You can [`read`](crate::Reg::read) this register and get [`maxwlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxwlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#I3C1:MAXWLR)*/
pub struct MAXWLRrs;
impl crate::RegisterSpec for MAXWLRrs {
    type Ux = u32;
}
///`read()` method returns [`maxwlr::R`](R) reader structure
impl crate::Readable for MAXWLRrs {}
///`write(|w| ..)` method takes [`maxwlr::W`](W) writer structure
impl crate::Writable for MAXWLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MAXWLR to value 0
impl crate::Resettable for MAXWLRrs {}
