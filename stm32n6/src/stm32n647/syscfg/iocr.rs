///Register `IOCR` reader
pub type R = crate::R<IOCRrs>;
///Register `IOCR` writer
pub type W = crate::W<IOCRrs>;
///Field `IOCR` reader - Digital or analog pins
pub type IOCR_R = crate::FieldReader<u32>;
///Field `IOCR` writer - Digital or analog pins
pub type IOCR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Digital or analog pins
    #[inline(always)]
    pub fn iocr(&self) -> IOCR_R {
        IOCR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCR").field("iocr", &self.iocr()).finish()
    }
}
impl W {
    ///Bits 0:31 - Digital or analog pins
    #[inline(always)]
    pub fn iocr(&mut self) -> IOCR_W<'_, IOCRrs> {
        IOCR_W::new(self, 0)
    }
}
/**SYSCFG /O control register

You can [`read`](crate::Reg::read) this register and get [`iocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SYSCFG:IOCR)*/
pub struct IOCRrs;
impl crate::RegisterSpec for IOCRrs {
    type Ux = u32;
}
///`read()` method returns [`iocr::R`](R) reader structure
impl crate::Readable for IOCRrs {}
///`write(|w| ..)` method takes [`iocr::W`](W) writer structure
impl crate::Writable for IOCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IOCR to value 0
impl crate::Resettable for IOCRrs {}
