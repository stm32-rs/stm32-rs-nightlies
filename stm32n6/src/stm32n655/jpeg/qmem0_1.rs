///Register `QMEM0_1` reader
pub type R = crate::R<QMEM0_1rs>;
///Register `QMEM0_1` writer
pub type W = crate::W<QMEM0_1rs>;
///Field `QCOEF4` reader - Quantization coefficient 4
pub type QCOEF4_R = crate::FieldReader;
///Field `QCOEF4` writer - Quantization coefficient 4
pub type QCOEF4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF5` reader - Quantization coefficient 5
pub type QCOEF5_R = crate::FieldReader;
///Field `QCOEF5` writer - Quantization coefficient 5
pub type QCOEF5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF6` reader - Quantization coefficient 6
pub type QCOEF6_R = crate::FieldReader;
///Field `QCOEF6` writer - Quantization coefficient 6
pub type QCOEF6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF7` reader - Quantization coefficient 7
pub type QCOEF7_R = crate::FieldReader;
///Field `QCOEF7` writer - Quantization coefficient 7
pub type QCOEF7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Quantization coefficient 4
    #[inline(always)]
    pub fn qcoef4(&self) -> QCOEF4_R {
        QCOEF4_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Quantization coefficient 5
    #[inline(always)]
    pub fn qcoef5(&self) -> QCOEF5_R {
        QCOEF5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Quantization coefficient 6
    #[inline(always)]
    pub fn qcoef6(&self) -> QCOEF6_R {
        QCOEF6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Quantization coefficient 7
    #[inline(always)]
    pub fn qcoef7(&self) -> QCOEF7_R {
        QCOEF7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QMEM0_1")
            .field("qcoef4", &self.qcoef4())
            .field("qcoef5", &self.qcoef5())
            .field("qcoef6", &self.qcoef6())
            .field("qcoef7", &self.qcoef7())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Quantization coefficient 4
    #[inline(always)]
    pub fn qcoef4(&mut self) -> QCOEF4_W<'_, QMEM0_1rs> {
        QCOEF4_W::new(self, 0)
    }
    ///Bits 8:15 - Quantization coefficient 5
    #[inline(always)]
    pub fn qcoef5(&mut self) -> QCOEF5_W<'_, QMEM0_1rs> {
        QCOEF5_W::new(self, 8)
    }
    ///Bits 16:23 - Quantization coefficient 6
    #[inline(always)]
    pub fn qcoef6(&mut self) -> QCOEF6_W<'_, QMEM0_1rs> {
        QCOEF6_W::new(self, 16)
    }
    ///Bits 24:31 - Quantization coefficient 7
    #[inline(always)]
    pub fn qcoef7(&mut self) -> QCOEF7_W<'_, QMEM0_1rs> {
        QCOEF7_W::new(self, 24)
    }
}
/**JPEG quantization memory 0

You can [`read`](crate::Reg::read) this register and get [`qmem0_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem0_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM0_1)*/
pub struct QMEM0_1rs;
impl crate::RegisterSpec for QMEM0_1rs {
    type Ux = u32;
}
///`read()` method returns [`qmem0_1::R`](R) reader structure
impl crate::Readable for QMEM0_1rs {}
///`write(|w| ..)` method takes [`qmem0_1::W`](W) writer structure
impl crate::Writable for QMEM0_1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets QMEM0_1 to value 0
impl crate::Resettable for QMEM0_1rs {}
