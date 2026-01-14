///Register `QMEM3_14` reader
pub type R = crate::R<QMEM3_14rs>;
///Register `QMEM3_14` writer
pub type W = crate::W<QMEM3_14rs>;
///Field `QCOEF56` reader - Quantization coefficient 56
pub type QCOEF56_R = crate::FieldReader;
///Field `QCOEF56` writer - Quantization coefficient 56
pub type QCOEF56_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF57` reader - Quantization coefficient 57
pub type QCOEF57_R = crate::FieldReader;
///Field `QCOEF57` writer - Quantization coefficient 57
pub type QCOEF57_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF58` reader - Quantization coefficient 58
pub type QCOEF58_R = crate::FieldReader;
///Field `QCOEF58` writer - Quantization coefficient 58
pub type QCOEF58_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF59` reader - Quantization coefficient 59
pub type QCOEF59_R = crate::FieldReader;
///Field `QCOEF59` writer - Quantization coefficient 59
pub type QCOEF59_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Quantization coefficient 56
    #[inline(always)]
    pub fn qcoef56(&self) -> QCOEF56_R {
        QCOEF56_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Quantization coefficient 57
    #[inline(always)]
    pub fn qcoef57(&self) -> QCOEF57_R {
        QCOEF57_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Quantization coefficient 58
    #[inline(always)]
    pub fn qcoef58(&self) -> QCOEF58_R {
        QCOEF58_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Quantization coefficient 59
    #[inline(always)]
    pub fn qcoef59(&self) -> QCOEF59_R {
        QCOEF59_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QMEM3_14")
            .field("qcoef56", &self.qcoef56())
            .field("qcoef57", &self.qcoef57())
            .field("qcoef58", &self.qcoef58())
            .field("qcoef59", &self.qcoef59())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Quantization coefficient 56
    #[inline(always)]
    pub fn qcoef56(&mut self) -> QCOEF56_W<'_, QMEM3_14rs> {
        QCOEF56_W::new(self, 0)
    }
    ///Bits 8:15 - Quantization coefficient 57
    #[inline(always)]
    pub fn qcoef57(&mut self) -> QCOEF57_W<'_, QMEM3_14rs> {
        QCOEF57_W::new(self, 8)
    }
    ///Bits 16:23 - Quantization coefficient 58
    #[inline(always)]
    pub fn qcoef58(&mut self) -> QCOEF58_W<'_, QMEM3_14rs> {
        QCOEF58_W::new(self, 16)
    }
    ///Bits 24:31 - Quantization coefficient 59
    #[inline(always)]
    pub fn qcoef59(&mut self) -> QCOEF59_W<'_, QMEM3_14rs> {
        QCOEF59_W::new(self, 24)
    }
}
/**JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3_14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3_14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:QMEM3_14)*/
pub struct QMEM3_14rs;
impl crate::RegisterSpec for QMEM3_14rs {
    type Ux = u32;
}
///`read()` method returns [`qmem3_14::R`](R) reader structure
impl crate::Readable for QMEM3_14rs {}
///`write(|w| ..)` method takes [`qmem3_14::W`](W) writer structure
impl crate::Writable for QMEM3_14rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets QMEM3_14 to value 0
impl crate::Resettable for QMEM3_14rs {}
