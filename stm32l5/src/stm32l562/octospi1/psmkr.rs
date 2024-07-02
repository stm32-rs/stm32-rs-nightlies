///Register `PSMKR` reader
pub type R = crate::R<PSMKRrs>;
///Register `PSMKR` writer
pub type W = crate::W<PSMKRrs>;
///Field `MATCH` reader - Status match
pub type MATCH_R = crate::FieldReader<u32>;
///Field `MATCH` writer - Status match
pub type MATCH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Status match
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSMKR")
            .field("match_", &self.match_())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Status match
    #[inline(always)]
    #[must_use]
    pub fn match_(&mut self) -> MATCH_W<PSMKRrs> {
        MATCH_W::new(self, 0)
    }
}
/**polling status mask register

You can [`read`](crate::Reg::read) this register and get [`psmkr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmkr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#OCTOSPI1:PSMKR)*/
pub struct PSMKRrs;
impl crate::RegisterSpec for PSMKRrs {
    type Ux = u32;
}
///`read()` method returns [`psmkr::R`](R) reader structure
impl crate::Readable for PSMKRrs {}
///`write(|w| ..)` method takes [`psmkr::W`](W) writer structure
impl crate::Writable for PSMKRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PSMKR to value 0
impl crate::Resettable for PSMKRrs {
    const RESET_VALUE: u32 = 0;
}
