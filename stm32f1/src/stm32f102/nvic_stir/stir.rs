///Register `STIR` reader
pub type R = crate::R<STIRrs>;
///Register `STIR` writer
pub type W = crate::W<STIRrs>;
///Field `INTID` reader - Software generated interrupt ID
pub type INTID_R = crate::FieldReader<u16>;
///Field `INTID` writer - Software generated interrupt ID
pub type INTID_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Software generated interrupt ID
    #[inline(always)]
    pub fn intid(&self) -> INTID_R {
        INTID_R::new((self.bits & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIR")
            .field("intid", &self.intid())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Software generated interrupt ID
    #[inline(always)]
    pub fn intid(&mut self) -> INTID_W<'_, STIRrs> {
        INTID_W::new(self, 0)
    }
}
/**Software trigger interrupt register

You can [`read`](crate::Reg::read) this register and get [`stir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F102.html#NVIC_STIR:STIR)*/
pub struct STIRrs;
impl crate::RegisterSpec for STIRrs {
    type Ux = u32;
}
///`read()` method returns [`stir::R`](R) reader structure
impl crate::Readable for STIRrs {}
///`write(|w| ..)` method takes [`stir::W`](W) writer structure
impl crate::Writable for STIRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STIR to value 0
impl crate::Resettable for STIRrs {}
