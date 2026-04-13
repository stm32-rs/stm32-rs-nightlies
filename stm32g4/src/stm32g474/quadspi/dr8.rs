///Register `DR8` reader
pub type R = crate::R<DR8rs>;
///Register `DR8` writer
pub type W = crate::W<DR8rs>;
///Field `DATA` reader - Data
pub type DATA_R = crate::FieldReader;
///Field `DATA` writer - Data
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bits 0:7 - Data
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR8").field("data", &self.data()).finish()
    }
}
impl W {
    ///Bits 0:7 - Data
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<'_, DR8rs> {
        DATA_W::new(self, 0)
    }
}
/**Data register: one byte (8 bit) access

You can [`read`](crate::Reg::read) this register and get [`dr8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#QUADSPI:DR8)*/
pub struct DR8rs;
impl crate::RegisterSpec for DR8rs {
    type Ux = u8;
}
///`read()` method returns [`dr8::R`](R) reader structure
impl crate::Readable for DR8rs {}
///`write(|w| ..)` method takes [`dr8::W`](W) writer structure
impl crate::Writable for DR8rs {
    type Safety = crate::Safe;
}
///`reset()` method sets DR8 to value 0
impl crate::Resettable for DR8rs {}
