///Register `DINR` reader
pub type R = crate::R<DINRrs>;
///Register `DINR` writer
pub type W = crate::W<DINRrs>;
///Field `DIN` reader - Data input
pub type DIN_R = crate::FieldReader<u32>;
///Field `DIN` writer - Data input
pub type DIN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Data input
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR").field("din", &self.din()).finish()
    }
}
impl W {
    ///Bits 0:31 - Data input
    #[inline(always)]
    pub fn din(&mut self) -> DIN_W<'_, DINRrs> {
        DIN_W::new(self, 0)
    }
}
/**CRYP data input register

You can [`read`](crate::Reg::read) this register and get [`dinr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#CRYP:DINR)*/
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
