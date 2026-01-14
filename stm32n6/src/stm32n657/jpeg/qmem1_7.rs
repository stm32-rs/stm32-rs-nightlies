///Register `QMEM1_7` reader
pub type R = crate::R<QMEM1_7rs>;
///Register `QMEM1_7` writer
pub type W = crate::W<QMEM1_7rs>;
///Field `QCOEF28` reader - Quantization coefficient 28
pub type QCOEF28_R = crate::FieldReader;
///Field `QCOEF28` writer - Quantization coefficient 28
pub type QCOEF28_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF29` reader - Quantization coefficient 29
pub type QCOEF29_R = crate::FieldReader;
///Field `QCOEF29` writer - Quantization coefficient 29
pub type QCOEF29_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF30` reader - Quantization coefficient 30
pub type QCOEF30_R = crate::FieldReader;
///Field `QCOEF30` writer - Quantization coefficient 30
pub type QCOEF30_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF31` reader - Quantization coefficient 31
pub type QCOEF31_R = crate::FieldReader;
///Field `QCOEF31` writer - Quantization coefficient 31
pub type QCOEF31_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Quantization coefficient 28
    #[inline(always)]
    pub fn qcoef28(&self) -> QCOEF28_R {
        QCOEF28_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Quantization coefficient 29
    #[inline(always)]
    pub fn qcoef29(&self) -> QCOEF29_R {
        QCOEF29_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Quantization coefficient 30
    #[inline(always)]
    pub fn qcoef30(&self) -> QCOEF30_R {
        QCOEF30_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Quantization coefficient 31
    #[inline(always)]
    pub fn qcoef31(&self) -> QCOEF31_R {
        QCOEF31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QMEM1_7")
            .field("qcoef28", &self.qcoef28())
            .field("qcoef29", &self.qcoef29())
            .field("qcoef30", &self.qcoef30())
            .field("qcoef31", &self.qcoef31())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Quantization coefficient 28
    #[inline(always)]
    pub fn qcoef28(&mut self) -> QCOEF28_W<'_, QMEM1_7rs> {
        QCOEF28_W::new(self, 0)
    }
    ///Bits 8:15 - Quantization coefficient 29
    #[inline(always)]
    pub fn qcoef29(&mut self) -> QCOEF29_W<'_, QMEM1_7rs> {
        QCOEF29_W::new(self, 8)
    }
    ///Bits 16:23 - Quantization coefficient 30
    #[inline(always)]
    pub fn qcoef30(&mut self) -> QCOEF30_W<'_, QMEM1_7rs> {
        QCOEF30_W::new(self, 16)
    }
    ///Bits 24:31 - Quantization coefficient 31
    #[inline(always)]
    pub fn qcoef31(&mut self) -> QCOEF31_W<'_, QMEM1_7rs> {
        QCOEF31_W::new(self, 24)
    }
}
/**JPEG quantization memory 1

You can [`read`](crate::Reg::read) this register and get [`qmem1_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem1_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:QMEM1_7)*/
pub struct QMEM1_7rs;
impl crate::RegisterSpec for QMEM1_7rs {
    type Ux = u32;
}
///`read()` method returns [`qmem1_7::R`](R) reader structure
impl crate::Readable for QMEM1_7rs {}
///`write(|w| ..)` method takes [`qmem1_7::W`](W) writer structure
impl crate::Writable for QMEM1_7rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets QMEM1_7 to value 0
impl crate::Resettable for QMEM1_7rs {}
