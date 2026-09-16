///Register `DHR12RD` reader
pub type R = crate::R<DHR12RDrs>;
///Register `DHR12RD` writer
pub type W = crate::W<DHR12RDrs>;
///Field `DACC1DHR` reader - DAC channel1 12-bit right-aligned data
pub type DACC1DHR_R = crate::FieldReader<u16>;
///Field `DACC1DHR` writer - DAC channel1 12-bit right-aligned data
pub type DACC1DHR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `DACC2DHR` reader - DAC channel2 12-bit right-aligned data
pub type DACC2DHR_R = crate::FieldReader<u16>;
///Field `DACC2DHR` writer - DAC channel2 12-bit right-aligned data
pub type DACC2DHR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - DAC channel1 12-bit right-aligned data
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - DAC channel2 12-bit right-aligned data
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHR12RD")
            .field("dacc2dhr", &self.dacc2dhr())
            .field("dacc1dhr", &self.dacc1dhr())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - DAC channel1 12-bit right-aligned data
    #[inline(always)]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W<'_, DHR12RDrs> {
        DACC1DHR_W::new(self, 0)
    }
    ///Bits 16:27 - DAC channel2 12-bit right-aligned data
    #[inline(always)]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W<'_, DHR12RDrs> {
        DACC2DHR_W::new(self, 16)
    }
}
/**Dual DAC 12-bit right-aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12rd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12rd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#DAC:DHR12RD)*/
pub struct DHR12RDrs;
impl crate::RegisterSpec for DHR12RDrs {
    type Ux = u32;
}
///`read()` method returns [`dhr12rd::R`](R) reader structure
impl crate::Readable for DHR12RDrs {}
///`write(|w| ..)` method takes [`dhr12rd::W`](W) writer structure
impl crate::Writable for DHR12RDrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHR12RD to value 0
impl crate::Resettable for DHR12RDrs {}
