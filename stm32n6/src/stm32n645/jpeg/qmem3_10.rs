///Register `QMEM3_10` reader
pub type R = crate::R<QMEM3_10rs>;
///Register `QMEM3_10` writer
pub type W = crate::W<QMEM3_10rs>;
///Field `QCOEF40` reader - Quantization coefficient 40
pub type QCOEF40_R = crate::FieldReader;
///Field `QCOEF40` writer - Quantization coefficient 40
pub type QCOEF40_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF41` reader - Quantization coefficient 41
pub type QCOEF41_R = crate::FieldReader;
///Field `QCOEF41` writer - Quantization coefficient 41
pub type QCOEF41_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF42` reader - Quantization coefficient 42
pub type QCOEF42_R = crate::FieldReader;
///Field `QCOEF42` writer - Quantization coefficient 42
pub type QCOEF42_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF43` reader - Quantization coefficient 43
pub type QCOEF43_R = crate::FieldReader;
///Field `QCOEF43` writer - Quantization coefficient 43
pub type QCOEF43_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Quantization coefficient 40
    #[inline(always)]
    pub fn qcoef40(&self) -> QCOEF40_R {
        QCOEF40_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Quantization coefficient 41
    #[inline(always)]
    pub fn qcoef41(&self) -> QCOEF41_R {
        QCOEF41_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Quantization coefficient 42
    #[inline(always)]
    pub fn qcoef42(&self) -> QCOEF42_R {
        QCOEF42_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Quantization coefficient 43
    #[inline(always)]
    pub fn qcoef43(&self) -> QCOEF43_R {
        QCOEF43_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QMEM3_10")
            .field("qcoef40", &self.qcoef40())
            .field("qcoef41", &self.qcoef41())
            .field("qcoef42", &self.qcoef42())
            .field("qcoef43", &self.qcoef43())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Quantization coefficient 40
    #[inline(always)]
    pub fn qcoef40(&mut self) -> QCOEF40_W<'_, QMEM3_10rs> {
        QCOEF40_W::new(self, 0)
    }
    ///Bits 8:15 - Quantization coefficient 41
    #[inline(always)]
    pub fn qcoef41(&mut self) -> QCOEF41_W<'_, QMEM3_10rs> {
        QCOEF41_W::new(self, 8)
    }
    ///Bits 16:23 - Quantization coefficient 42
    #[inline(always)]
    pub fn qcoef42(&mut self) -> QCOEF42_W<'_, QMEM3_10rs> {
        QCOEF42_W::new(self, 16)
    }
    ///Bits 24:31 - Quantization coefficient 43
    #[inline(always)]
    pub fn qcoef43(&mut self) -> QCOEF43_W<'_, QMEM3_10rs> {
        QCOEF43_W::new(self, 24)
    }
}
/**JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3_10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3_10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:QMEM3_10)*/
pub struct QMEM3_10rs;
impl crate::RegisterSpec for QMEM3_10rs {
    type Ux = u32;
}
///`read()` method returns [`qmem3_10::R`](R) reader structure
impl crate::Readable for QMEM3_10rs {}
///`write(|w| ..)` method takes [`qmem3_10::W`](W) writer structure
impl crate::Writable for QMEM3_10rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets QMEM3_10 to value 0
impl crate::Resettable for QMEM3_10rs {}
