///Register `QMEM1_6` reader
pub type R = crate::R<QMEM1_6rs>;
///Register `QMEM1_6` writer
pub type W = crate::W<QMEM1_6rs>;
///Field `QCOEF24` reader - Quantization coefficient 24
pub type QCOEF24_R = crate::FieldReader;
///Field `QCOEF24` writer - Quantization coefficient 24
pub type QCOEF24_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF25` reader - Quantization coefficient 25
pub type QCOEF25_R = crate::FieldReader;
///Field `QCOEF25` writer - Quantization coefficient 25
pub type QCOEF25_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF26` reader - Quantization coefficient 26
pub type QCOEF26_R = crate::FieldReader;
///Field `QCOEF26` writer - Quantization coefficient 26
pub type QCOEF26_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF27` reader - Quantization coefficient 27
pub type QCOEF27_R = crate::FieldReader;
///Field `QCOEF27` writer - Quantization coefficient 27
pub type QCOEF27_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Quantization coefficient 24
    #[inline(always)]
    pub fn qcoef24(&self) -> QCOEF24_R {
        QCOEF24_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Quantization coefficient 25
    #[inline(always)]
    pub fn qcoef25(&self) -> QCOEF25_R {
        QCOEF25_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Quantization coefficient 26
    #[inline(always)]
    pub fn qcoef26(&self) -> QCOEF26_R {
        QCOEF26_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Quantization coefficient 27
    #[inline(always)]
    pub fn qcoef27(&self) -> QCOEF27_R {
        QCOEF27_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QMEM1_6")
            .field("qcoef24", &self.qcoef24())
            .field("qcoef25", &self.qcoef25())
            .field("qcoef26", &self.qcoef26())
            .field("qcoef27", &self.qcoef27())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Quantization coefficient 24
    #[inline(always)]
    pub fn qcoef24(&mut self) -> QCOEF24_W<'_, QMEM1_6rs> {
        QCOEF24_W::new(self, 0)
    }
    ///Bits 8:15 - Quantization coefficient 25
    #[inline(always)]
    pub fn qcoef25(&mut self) -> QCOEF25_W<'_, QMEM1_6rs> {
        QCOEF25_W::new(self, 8)
    }
    ///Bits 16:23 - Quantization coefficient 26
    #[inline(always)]
    pub fn qcoef26(&mut self) -> QCOEF26_W<'_, QMEM1_6rs> {
        QCOEF26_W::new(self, 16)
    }
    ///Bits 24:31 - Quantization coefficient 27
    #[inline(always)]
    pub fn qcoef27(&mut self) -> QCOEF27_W<'_, QMEM1_6rs> {
        QCOEF27_W::new(self, 24)
    }
}
/**JPEG quantization memory 1

You can [`read`](crate::Reg::read) this register and get [`qmem1_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem1_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM1_6)*/
pub struct QMEM1_6rs;
impl crate::RegisterSpec for QMEM1_6rs {
    type Ux = u32;
}
///`read()` method returns [`qmem1_6::R`](R) reader structure
impl crate::Readable for QMEM1_6rs {}
///`write(|w| ..)` method takes [`qmem1_6::W`](W) writer structure
impl crate::Writable for QMEM1_6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets QMEM1_6 to value 0
impl crate::Resettable for QMEM1_6rs {}
