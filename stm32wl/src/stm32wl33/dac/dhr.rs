///Register `DHR` reader
pub type R = crate::R<DHRrs>;
///Register `DHR` writer
pub type W = crate::W<DHRrs>;
///Field `DACDHR` reader - DACDHR\[5:0\]: DAC channel 6-bit data These bits are written by software which specifies 6-bit data for DAC channel.
pub type DACDHR_R = crate::FieldReader;
///Field `DACDHR` writer - DACDHR\[5:0\]: DAC channel 6-bit data These bits are written by software which specifies 6-bit data for DAC channel.
pub type DACDHR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - DACDHR\[5:0\]: DAC channel 6-bit data These bits are written by software which specifies 6-bit data for DAC channel.
    #[inline(always)]
    pub fn dacdhr(&self) -> DACDHR_R {
        DACDHR_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHR")
            .field("dacdhr", &self.dacdhr())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - DACDHR\[5:0\]: DAC channel 6-bit data These bits are written by software which specifies 6-bit data for DAC channel.
    #[inline(always)]
    pub fn dacdhr(&mut self) -> DACDHR_W<'_, DHRrs> {
        DACDHR_W::new(self, 0)
    }
}
/**DHR register

You can [`read`](crate::Reg::read) this register and get [`dhr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DAC:DHR)*/
pub struct DHRrs;
impl crate::RegisterSpec for DHRrs {
    type Ux = u32;
}
///`read()` method returns [`dhr::R`](R) reader structure
impl crate::Readable for DHRrs {}
///`write(|w| ..)` method takes [`dhr::W`](W) writer structure
impl crate::Writable for DHRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHR to value 0
impl crate::Resettable for DHRrs {}
