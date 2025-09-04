///Register `DR` reader
pub type R = crate::R<DRrs>;
///Register `DR` writer
pub type W = crate::W<DRrs>;
///Field `DR` reader - Data register
pub type DR_R = crate::FieldReader<u16>;
///Field `DR` writer - Data register
pub type DR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - Data register
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR").field("dr", &self.dr()).finish()
    }
}
impl W {
    ///Bits 0:15 - Data register
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W<DRrs> {
        DR_W::new(self, 0)
    }
}
/**data register

You can [`read`](crate::Reg::read) this register and get [`dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#SPI1:DR)*/
pub struct DRrs;
impl crate::RegisterSpec for DRrs {
    type Ux = u16;
}
///`read()` method returns [`dr::R`](R) reader structure
impl crate::Readable for DRrs {}
///`write(|w| ..)` method takes [`dr::W`](W) writer structure
impl crate::Writable for DRrs {
    type Safety = crate::Safe;
}
///`reset()` method sets DR to value 0
impl crate::Resettable for DRrs {}
