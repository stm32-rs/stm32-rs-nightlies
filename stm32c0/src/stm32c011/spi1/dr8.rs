///Register `DR8` reader
pub type R = crate::R<DR8rs>;
///Register `DR8` writer
pub type W = crate::W<DR8rs>;
///Field `DR` reader - Data register
pub type DR_R = crate::FieldReader;
///Field `DR` writer - Data register
pub type DR_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bits 0:7 - Data register
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR8").field("dr", &self.dr()).finish()
    }
}
impl W {
    ///Bits 0:7 - Data register
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W<DR8rs> {
        DR_W::new(self, 0)
    }
}
/**Direct 8-bit access to data register

You can [`read`](crate::Reg::read) this register and get [`dr8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#SPI1:DR8)*/
pub struct DR8rs;
impl crate::RegisterSpec for DR8rs {
    type Ux = u8;
}
///`read()` method returns [`dr8::R`](R) reader structure
impl crate::Readable for DR8rs {}
///`write(|w| ..)` method takes [`dr8::W`](W) writer structure
impl crate::Writable for DR8rs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
///`reset()` method sets DR8 to value 0
impl crate::Resettable for DR8rs {
    const RESET_VALUE: u8 = 0;
}
