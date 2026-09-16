///Register `QMEM1_11` reader
pub type R = crate::R<QMEM1_11rs>;
///Register `QMEM1_11` writer
pub type W = crate::W<QMEM1_11rs>;
///Field `QCOEF44` reader - Quantization coefficient 44
pub type QCOEF44_R = crate::FieldReader;
///Field `QCOEF44` writer - Quantization coefficient 44
pub type QCOEF44_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF45` reader - Quantization coefficient 45
pub type QCOEF45_R = crate::FieldReader;
///Field `QCOEF45` writer - Quantization coefficient 45
pub type QCOEF45_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF46` reader - Quantization coefficient 46
pub type QCOEF46_R = crate::FieldReader;
///Field `QCOEF46` writer - Quantization coefficient 46
pub type QCOEF46_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF47` reader - Quantization coefficient 47
pub type QCOEF47_R = crate::FieldReader;
///Field `QCOEF47` writer - Quantization coefficient 47
pub type QCOEF47_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Quantization coefficient 44
    #[inline(always)]
    pub fn qcoef44(&self) -> QCOEF44_R {
        QCOEF44_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Quantization coefficient 45
    #[inline(always)]
    pub fn qcoef45(&self) -> QCOEF45_R {
        QCOEF45_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Quantization coefficient 46
    #[inline(always)]
    pub fn qcoef46(&self) -> QCOEF46_R {
        QCOEF46_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Quantization coefficient 47
    #[inline(always)]
    pub fn qcoef47(&self) -> QCOEF47_R {
        QCOEF47_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QMEM1_11")
            .field("qcoef44", &self.qcoef44())
            .field("qcoef45", &self.qcoef45())
            .field("qcoef46", &self.qcoef46())
            .field("qcoef47", &self.qcoef47())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Quantization coefficient 44
    #[inline(always)]
    pub fn qcoef44(&mut self) -> QCOEF44_W<'_, QMEM1_11rs> {
        QCOEF44_W::new(self, 0)
    }
    ///Bits 8:15 - Quantization coefficient 45
    #[inline(always)]
    pub fn qcoef45(&mut self) -> QCOEF45_W<'_, QMEM1_11rs> {
        QCOEF45_W::new(self, 8)
    }
    ///Bits 16:23 - Quantization coefficient 46
    #[inline(always)]
    pub fn qcoef46(&mut self) -> QCOEF46_W<'_, QMEM1_11rs> {
        QCOEF46_W::new(self, 16)
    }
    ///Bits 24:31 - Quantization coefficient 47
    #[inline(always)]
    pub fn qcoef47(&mut self) -> QCOEF47_W<'_, QMEM1_11rs> {
        QCOEF47_W::new(self, 24)
    }
}
/**JPEG quantization memory 1

You can [`read`](crate::Reg::read) this register and get [`qmem1_11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem1_11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:QMEM1_11)*/
pub struct QMEM1_11rs;
impl crate::RegisterSpec for QMEM1_11rs {
    type Ux = u32;
}
///`read()` method returns [`qmem1_11::R`](R) reader structure
impl crate::Readable for QMEM1_11rs {}
///`write(|w| ..)` method takes [`qmem1_11::W`](W) writer structure
impl crate::Writable for QMEM1_11rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets QMEM1_11 to value 0
impl crate::Resettable for QMEM1_11rs {}
