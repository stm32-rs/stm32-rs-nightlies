///Register `DINR` reader
pub type R = crate::R<DINRrs>;
///Register `DINR` writer
pub type W = crate::W<DINRrs>;
///Field `DINR` reader - Data input
pub type DINR_R = crate::FieldReader<u32>;
///Field `DINR` writer - Data input
pub type DINR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Data input
    #[inline(always)]
    pub fn dinr(&self) -> DINR_R {
        DINR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR").field("dinr", &self.dinr()).finish()
    }
}
impl W {
    ///Bits 0:31 - Data input
    #[inline(always)]
    pub fn dinr(&mut self) -> DINR_W<'_, DINRrs> {
        DINR_W::new(self, 0)
    }
}
/**data input register

You can [`read`](crate::Reg::read) this register and get [`dinr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F723.html#CRYP:DINR)*/
pub struct DINRrs;
impl crate::RegisterSpec for DINRrs {
    type Ux = u32;
}
///`read()` method returns [`dinr::R`](R) reader structure
impl crate::Readable for DINRrs {}
///`write(|w| ..)` method takes [`dinr::W`](W) writer structure
impl crate::Writable for DINRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DINR to value 0
impl crate::Resettable for DINRrs {}
