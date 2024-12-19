///Register `DR` reader
pub type R = crate::R<DRrs>;
///Register `DR` writer
pub type W = crate::W<DRrs>;
///Field `RDATA_0_6` reader - Regular Data converted 0_6
pub type RDATA_0_6_R = crate::FieldReader;
///Field `RDATA_0_6` writer - Regular Data converted 0_6
pub type RDATA_0_6_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RDATA_7_15` reader - 15
pub type RDATA_7_15_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:5 - Regular Data converted 0_6
    #[inline(always)]
    pub fn rdata_0_6(&self) -> RDATA_0_6_R {
        RDATA_0_6_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 7:15 - 15
    #[inline(always)]
    pub fn rdata_7_15(&self) -> RDATA_7_15_R {
        RDATA_7_15_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR")
            .field("rdata_0_6", &self.rdata_0_6())
            .field("rdata_7_15", &self.rdata_7_15())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Regular Data converted 0_6
    #[inline(always)]
    pub fn rdata_0_6(&mut self) -> RDATA_0_6_W<DRrs> {
        RDATA_0_6_W::new(self, 0)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DR to value 0
impl crate::Resettable for DRrs {
    const RESET_VALUE: u32 = 0;
}
