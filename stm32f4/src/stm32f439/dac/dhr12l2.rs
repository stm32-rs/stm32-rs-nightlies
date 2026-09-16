///Register `DHR12L2` reader
pub type R = crate::R<DHR12L2rs>;
///Register `DHR12L2` writer
pub type W = crate::W<DHR12L2rs>;
///Field `DACC2DHR` reader - DAC channel2 12-bit left-aligned data
pub type DACC2DHR_R = crate::FieldReader<u16>;
///Field `DACC2DHR` writer - DAC channel2 12-bit left-aligned data
pub type DACC2DHR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 4:15 - DAC channel2 12-bit left-aligned data
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHR12L2")
            .field("dacc2dhr", &self.dacc2dhr())
            .finish()
    }
}
impl W {
    ///Bits 4:15 - DAC channel2 12-bit left-aligned data
    #[inline(always)]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W<'_, DHR12L2rs> {
        DACC2DHR_W::new(self, 4)
    }
}
/**channel2 12-bit left aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12l2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12l2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#DAC:DHR12L2)*/
pub struct DHR12L2rs;
impl crate::RegisterSpec for DHR12L2rs {
    type Ux = u32;
}
///`read()` method returns [`dhr12l2::R`](R) reader structure
impl crate::Readable for DHR12L2rs {}
///`write(|w| ..)` method takes [`dhr12l2::W`](W) writer structure
impl crate::Writable for DHR12L2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHR12L2 to value 0
impl crate::Resettable for DHR12L2rs {}
