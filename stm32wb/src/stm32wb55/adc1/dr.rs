///Register `DR` reader
pub type R = crate::R<DRrs>;
///Register `DR` writer
pub type W = crate::W<DRrs>;
///Field `RDATA` reader - Regular Data converted 0_6
pub type RDATA_R = crate::FieldReader<u16>;
///Field `RDATA` writer - Regular Data converted 0_6
pub type RDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - Regular Data converted 0_6
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR").field("rdata", &self.rdata()).finish()
    }
}
impl W {
    ///Bits 0:15 - Regular Data converted 0_6
    #[inline(always)]
    pub fn rdata(&mut self) -> RDATA_W<'_, DRrs> {
        RDATA_W::new(self, 0)
    }
}
/**ADC group regular conversion data register

You can [`read`](crate::Reg::read) this register and get [`dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#ADC1:DR)*/
pub struct DRrs;
impl crate::RegisterSpec for DRrs {
    type Ux = u32;
}
///`read()` method returns [`dr::R`](R) reader structure
impl crate::Readable for DRrs {}
///`write(|w| ..)` method takes [`dr::W`](W) writer structure
impl crate::Writable for DRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DR to value 0
impl crate::Resettable for DRrs {}
