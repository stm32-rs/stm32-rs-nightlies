///Register `DHR8RD` reader
pub type R = crate::R<DHR8RDrs>;
///Register `DHR8RD` writer
pub type W = crate::W<DHR8RDrs>;
///Field `DACC1DHR` reader - DAC channel1 8-bit right-aligned data
pub type DACC1DHR_R = crate::FieldReader;
///Field `DACC1DHR` writer - DAC channel1 8-bit right-aligned data
pub type DACC1DHR_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `DACC2DHR` reader - DAC channel2 8-bit right-aligned data
pub type DACC2DHR_R = crate::FieldReader;
///Field `DACC2DHR` writer - DAC channel2 8-bit right-aligned data
pub type DACC2DHR_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bits 0:7 - DAC channel1 8-bit right-aligned data
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - DAC channel2 8-bit right-aligned data
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHR8RD")
            .field("dacc1dhr", &self.dacc1dhr())
            .field("dacc2dhr", &self.dacc2dhr())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - DAC channel1 8-bit right-aligned data
    #[inline(always)]
    #[must_use]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W<DHR8RDrs> {
        DACC1DHR_W::new(self, 0)
    }
    ///Bits 8:15 - DAC channel2 8-bit right-aligned data
    #[inline(always)]
    #[must_use]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W<DHR8RDrs> {
        DACC2DHR_W::new(self, 8)
    }
}
/**Dual DAC 8-bit right-aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr8rd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8rd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x2.html#DAC:DHR8RD)*/
pub struct DHR8RDrs;
impl crate::RegisterSpec for DHR8RDrs {
    type Ux = u32;
}
///`read()` method returns [`dhr8rd::R`](R) reader structure
impl crate::Readable for DHR8RDrs {}
///`write(|w| ..)` method takes [`dhr8rd::W`](W) writer structure
impl crate::Writable for DHR8RDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DHR8RD to value 0
impl crate::Resettable for DHR8RDrs {
    const RESET_VALUE: u32 = 0;
}
