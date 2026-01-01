///Register `QMEM3_5` reader
pub type R = crate::R<QMEM3_5rs>;
///Register `QMEM3_5` writer
pub type W = crate::W<QMEM3_5rs>;
///Field `QCOEF20` reader - Quantization coefficient 20
pub type QCOEF20_R = crate::FieldReader;
///Field `QCOEF20` writer - Quantization coefficient 20
pub type QCOEF20_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF21` reader - Quantization coefficient 21
pub type QCOEF21_R = crate::FieldReader;
///Field `QCOEF21` writer - Quantization coefficient 21
pub type QCOEF21_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF22` reader - Quantization coefficient 22
pub type QCOEF22_R = crate::FieldReader;
///Field `QCOEF22` writer - Quantization coefficient 22
pub type QCOEF22_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF23` reader - Quantization coefficient 23
pub type QCOEF23_R = crate::FieldReader;
///Field `QCOEF23` writer - Quantization coefficient 23
pub type QCOEF23_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Quantization coefficient 20
    #[inline(always)]
    pub fn qcoef20(&self) -> QCOEF20_R {
        QCOEF20_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Quantization coefficient 21
    #[inline(always)]
    pub fn qcoef21(&self) -> QCOEF21_R {
        QCOEF21_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Quantization coefficient 22
    #[inline(always)]
    pub fn qcoef22(&self) -> QCOEF22_R {
        QCOEF22_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Quantization coefficient 23
    #[inline(always)]
    pub fn qcoef23(&self) -> QCOEF23_R {
        QCOEF23_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QMEM3_5")
            .field("qcoef20", &self.qcoef20())
            .field("qcoef21", &self.qcoef21())
            .field("qcoef22", &self.qcoef22())
            .field("qcoef23", &self.qcoef23())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Quantization coefficient 20
    #[inline(always)]
    pub fn qcoef20(&mut self) -> QCOEF20_W<'_, QMEM3_5rs> {
        QCOEF20_W::new(self, 0)
    }
    ///Bits 8:15 - Quantization coefficient 21
    #[inline(always)]
    pub fn qcoef21(&mut self) -> QCOEF21_W<'_, QMEM3_5rs> {
        QCOEF21_W::new(self, 8)
    }
    ///Bits 16:23 - Quantization coefficient 22
    #[inline(always)]
    pub fn qcoef22(&mut self) -> QCOEF22_W<'_, QMEM3_5rs> {
        QCOEF22_W::new(self, 16)
    }
    ///Bits 24:31 - Quantization coefficient 23
    #[inline(always)]
    pub fn qcoef23(&mut self) -> QCOEF23_W<'_, QMEM3_5rs> {
        QCOEF23_W::new(self, 24)
    }
}
/**JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:QMEM3_5)*/
pub struct QMEM3_5rs;
impl crate::RegisterSpec for QMEM3_5rs {
    type Ux = u32;
}
///`read()` method returns [`qmem3_5::R`](R) reader structure
impl crate::Readable for QMEM3_5rs {}
///`write(|w| ..)` method takes [`qmem3_5::W`](W) writer structure
impl crate::Writable for QMEM3_5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets QMEM3_5 to value 0
impl crate::Resettable for QMEM3_5rs {}
