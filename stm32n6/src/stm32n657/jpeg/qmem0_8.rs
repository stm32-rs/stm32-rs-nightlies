///Register `QMEM0_8` reader
pub type R = crate::R<QMEM0_8rs>;
///Register `QMEM0_8` writer
pub type W = crate::W<QMEM0_8rs>;
///Field `QCOEF32` reader - Quantization coefficient 32
pub type QCOEF32_R = crate::FieldReader;
///Field `QCOEF32` writer - Quantization coefficient 32
pub type QCOEF32_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF33` reader - Quantization coefficient 33
pub type QCOEF33_R = crate::FieldReader;
///Field `QCOEF33` writer - Quantization coefficient 33
pub type QCOEF33_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF34` reader - Quantization coefficient 34
pub type QCOEF34_R = crate::FieldReader;
///Field `QCOEF34` writer - Quantization coefficient 34
pub type QCOEF34_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF35` reader - Quantization coefficient 35
pub type QCOEF35_R = crate::FieldReader;
///Field `QCOEF35` writer - Quantization coefficient 35
pub type QCOEF35_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Quantization coefficient 32
    #[inline(always)]
    pub fn qcoef32(&self) -> QCOEF32_R {
        QCOEF32_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Quantization coefficient 33
    #[inline(always)]
    pub fn qcoef33(&self) -> QCOEF33_R {
        QCOEF33_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Quantization coefficient 34
    #[inline(always)]
    pub fn qcoef34(&self) -> QCOEF34_R {
        QCOEF34_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Quantization coefficient 35
    #[inline(always)]
    pub fn qcoef35(&self) -> QCOEF35_R {
        QCOEF35_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QMEM0_8")
            .field("qcoef32", &self.qcoef32())
            .field("qcoef33", &self.qcoef33())
            .field("qcoef34", &self.qcoef34())
            .field("qcoef35", &self.qcoef35())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Quantization coefficient 32
    #[inline(always)]
    pub fn qcoef32(&mut self) -> QCOEF32_W<'_, QMEM0_8rs> {
        QCOEF32_W::new(self, 0)
    }
    ///Bits 8:15 - Quantization coefficient 33
    #[inline(always)]
    pub fn qcoef33(&mut self) -> QCOEF33_W<'_, QMEM0_8rs> {
        QCOEF33_W::new(self, 8)
    }
    ///Bits 16:23 - Quantization coefficient 34
    #[inline(always)]
    pub fn qcoef34(&mut self) -> QCOEF34_W<'_, QMEM0_8rs> {
        QCOEF34_W::new(self, 16)
    }
    ///Bits 24:31 - Quantization coefficient 35
    #[inline(always)]
    pub fn qcoef35(&mut self) -> QCOEF35_W<'_, QMEM0_8rs> {
        QCOEF35_W::new(self, 24)
    }
}
/**JPEG quantization memory 0

You can [`read`](crate::Reg::read) this register and get [`qmem0_8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem0_8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:QMEM0_8)*/
pub struct QMEM0_8rs;
impl crate::RegisterSpec for QMEM0_8rs {
    type Ux = u32;
}
///`read()` method returns [`qmem0_8::R`](R) reader structure
impl crate::Readable for QMEM0_8rs {}
///`write(|w| ..)` method takes [`qmem0_8::W`](W) writer structure
impl crate::Writable for QMEM0_8rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets QMEM0_8 to value 0
impl crate::Resettable for QMEM0_8rs {}
