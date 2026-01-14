///Register `DHR8R2` reader
pub type R = crate::R<DHR8R2rs>;
///Register `DHR8R2` writer
pub type W = crate::W<DHR8R2rs>;
///Field `DACC2DHR` reader - DACC2DHR
pub type DACC2DHR_R = crate::FieldReader;
///Field `DACC2DHR` writer - DACC2DHR
pub type DACC2DHR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - DACC2DHR
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHR8R2")
            .field("dacc2dhr", &self.dacc2dhr())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - DACC2DHR
    #[inline(always)]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W<'_, DHR8R2rs> {
        DACC2DHR_W::new(self, 0)
    }
}
/**This register is available only on dual-channel DACs. Refer to Section29.3: DAC implementation.

You can [`read`](crate::Reg::read) this register and get [`dhr8r2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8r2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DAC1:DHR8R2)*/
pub struct DHR8R2rs;
impl crate::RegisterSpec for DHR8R2rs {
    type Ux = u32;
}
///`read()` method returns [`dhr8r2::R`](R) reader structure
impl crate::Readable for DHR8R2rs {}
///`write(|w| ..)` method takes [`dhr8r2::W`](W) writer structure
impl crate::Writable for DHR8R2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHR8R2 to value 0
impl crate::Resettable for DHR8R2rs {}
