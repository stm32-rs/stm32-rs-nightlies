///Register `QMEM3%s` reader
pub type R = crate::R<QMEM3rs>;
///Register `QMEM3%s` writer
pub type W = crate::W<QMEM3rs>;
///Field `QCOEF0` reader - Quantization coefficient 0 8-bit quantization coefficient.
pub type QCOEF0_R = crate::FieldReader;
///Field `QCOEF0` writer - Quantization coefficient 0 8-bit quantization coefficient.
pub type QCOEF0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF1` reader - Quantization coefficient 1 8-bit quantization coefficient.
pub type QCOEF1_R = crate::FieldReader;
///Field `QCOEF1` writer - Quantization coefficient 1 8-bit quantization coefficient.
pub type QCOEF1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF2` reader - Quantization coefficient 2 8-bit quantization coefficient.
pub type QCOEF2_R = crate::FieldReader;
///Field `QCOEF2` writer - Quantization coefficient 2 8-bit quantization coefficient.
pub type QCOEF2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF3` reader - Quantization coefficient 3 8-bit quantization coefficient.
pub type QCOEF3_R = crate::FieldReader;
///Field `QCOEF3` writer - Quantization coefficient 3 8-bit quantization coefficient.
pub type QCOEF3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Quantization coefficient 0 8-bit quantization coefficient.
    #[inline(always)]
    pub fn qcoef0(&self) -> QCOEF0_R {
        QCOEF0_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Quantization coefficient 1 8-bit quantization coefficient.
    #[inline(always)]
    pub fn qcoef1(&self) -> QCOEF1_R {
        QCOEF1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Quantization coefficient 2 8-bit quantization coefficient.
    #[inline(always)]
    pub fn qcoef2(&self) -> QCOEF2_R {
        QCOEF2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Quantization coefficient 3 8-bit quantization coefficient.
    #[inline(always)]
    pub fn qcoef3(&self) -> QCOEF3_R {
        QCOEF3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QMEM3")
            .field("qcoef0", &self.qcoef0())
            .field("qcoef1", &self.qcoef1())
            .field("qcoef2", &self.qcoef2())
            .field("qcoef3", &self.qcoef3())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Quantization coefficient 0 8-bit quantization coefficient.
    #[inline(always)]
    pub fn qcoef0(&mut self) -> QCOEF0_W<'_, QMEM3rs> {
        QCOEF0_W::new(self, 0)
    }
    ///Bits 8:15 - Quantization coefficient 1 8-bit quantization coefficient.
    #[inline(always)]
    pub fn qcoef1(&mut self) -> QCOEF1_W<'_, QMEM3rs> {
        QCOEF1_W::new(self, 8)
    }
    ///Bits 16:23 - Quantization coefficient 2 8-bit quantization coefficient.
    #[inline(always)]
    pub fn qcoef2(&mut self) -> QCOEF2_W<'_, QMEM3rs> {
        QCOEF2_W::new(self, 16)
    }
    ///Bits 24:31 - Quantization coefficient 3 8-bit quantization coefficient.
    #[inline(always)]
    pub fn qcoef3(&mut self) -> QCOEF3_W<'_, QMEM3rs> {
        QCOEF3_W::new(self, 24)
    }
}
/**JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#JPEG:QMEM3[0])*/
pub struct QMEM3rs;
impl crate::RegisterSpec for QMEM3rs {
    type Ux = u32;
}
///`read()` method returns [`qmem3::R`](R) reader structure
impl crate::Readable for QMEM3rs {}
///`write(|w| ..)` method takes [`qmem3::W`](W) writer structure
impl crate::Writable for QMEM3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets QMEM3%s to value 0
impl crate::Resettable for QMEM3rs {}
