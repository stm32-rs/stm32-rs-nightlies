///Register `QMEM1_3` reader
pub type R = crate::R<QMEM1_3rs>;
///Register `QMEM1_3` writer
pub type W = crate::W<QMEM1_3rs>;
///Field `QCOEF12` reader - Quantization coefficient 12
pub type QCOEF12_R = crate::FieldReader;
///Field `QCOEF12` writer - Quantization coefficient 12
pub type QCOEF12_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF13` reader - Quantization coefficient 13
pub type QCOEF13_R = crate::FieldReader;
///Field `QCOEF13` writer - Quantization coefficient 13
pub type QCOEF13_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF14` reader - Quantization coefficient 14
pub type QCOEF14_R = crate::FieldReader;
///Field `QCOEF14` writer - Quantization coefficient 14
pub type QCOEF14_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF15` reader - Quantization coefficient 15
pub type QCOEF15_R = crate::FieldReader;
///Field `QCOEF15` writer - Quantization coefficient 15
pub type QCOEF15_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Quantization coefficient 12
    #[inline(always)]
    pub fn qcoef12(&self) -> QCOEF12_R {
        QCOEF12_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Quantization coefficient 13
    #[inline(always)]
    pub fn qcoef13(&self) -> QCOEF13_R {
        QCOEF13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Quantization coefficient 14
    #[inline(always)]
    pub fn qcoef14(&self) -> QCOEF14_R {
        QCOEF14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Quantization coefficient 15
    #[inline(always)]
    pub fn qcoef15(&self) -> QCOEF15_R {
        QCOEF15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QMEM1_3")
            .field("qcoef12", &self.qcoef12())
            .field("qcoef13", &self.qcoef13())
            .field("qcoef14", &self.qcoef14())
            .field("qcoef15", &self.qcoef15())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Quantization coefficient 12
    #[inline(always)]
    pub fn qcoef12(&mut self) -> QCOEF12_W<'_, QMEM1_3rs> {
        QCOEF12_W::new(self, 0)
    }
    ///Bits 8:15 - Quantization coefficient 13
    #[inline(always)]
    pub fn qcoef13(&mut self) -> QCOEF13_W<'_, QMEM1_3rs> {
        QCOEF13_W::new(self, 8)
    }
    ///Bits 16:23 - Quantization coefficient 14
    #[inline(always)]
    pub fn qcoef14(&mut self) -> QCOEF14_W<'_, QMEM1_3rs> {
        QCOEF14_W::new(self, 16)
    }
    ///Bits 24:31 - Quantization coefficient 15
    #[inline(always)]
    pub fn qcoef15(&mut self) -> QCOEF15_W<'_, QMEM1_3rs> {
        QCOEF15_W::new(self, 24)
    }
}
/**JPEG quantization memory 1

You can [`read`](crate::Reg::read) this register and get [`qmem1_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem1_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:QMEM1_3)*/
pub struct QMEM1_3rs;
impl crate::RegisterSpec for QMEM1_3rs {
    type Ux = u32;
}
///`read()` method returns [`qmem1_3::R`](R) reader structure
impl crate::Readable for QMEM1_3rs {}
///`write(|w| ..)` method takes [`qmem1_3::W`](W) writer structure
impl crate::Writable for QMEM1_3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets QMEM1_3 to value 0
impl crate::Resettable for QMEM1_3rs {}
