///Register `DR` reader
pub type R = crate::R<DRrs>;
///Register `DR` writer
pub type W = crate::W<DRrs>;
///Field `DR` reader - Data value
pub type DR_R = crate::FieldReader<u16>;
///Field `DR` writer - Data value
pub type DR_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16, crate::Safe>;
impl R {
    ///Bits 0:8 - Data value
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(self.bits & 0x01ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR").field("dr", &self.dr()).finish()
    }
}
impl W {
    ///Bits 0:8 - Data value
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W<'_, DRrs> {
        DR_W::new(self, 0)
    }
}
/**Data register

You can [`read`](crate::Reg::read) this register and get [`dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#USART1:DR)*/
pub struct DRrs;
impl crate::RegisterSpec for DRrs {
    type Ux = u16;
}
///`read()` method returns [`dr::R`](R) reader structure
impl crate::Readable for DRrs {}
///`write(|w| ..)` method takes [`dr::W`](W) writer structure
impl crate::Writable for DRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DR to value 0
impl crate::Resettable for DRrs {}
