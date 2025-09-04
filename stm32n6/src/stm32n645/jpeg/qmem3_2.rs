///Register `QMEM3_2` reader
pub type R = crate::R<QMEM3_2rs>;
///Register `QMEM3_2` writer
pub type W = crate::W<QMEM3_2rs>;
///Field `QCOEF8` reader - Quantization coefficient 8
pub type QCOEF8_R = crate::FieldReader;
///Field `QCOEF8` writer - Quantization coefficient 8
pub type QCOEF8_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF9` reader - Quantization coefficient 9
pub type QCOEF9_R = crate::FieldReader;
///Field `QCOEF9` writer - Quantization coefficient 9
pub type QCOEF9_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF10` reader - Quantization coefficient 10
pub type QCOEF10_R = crate::FieldReader;
///Field `QCOEF10` writer - Quantization coefficient 10
pub type QCOEF10_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF11` reader - Quantization coefficient 11
pub type QCOEF11_R = crate::FieldReader;
///Field `QCOEF11` writer - Quantization coefficient 11
pub type QCOEF11_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Quantization coefficient 8
    #[inline(always)]
    pub fn qcoef8(&self) -> QCOEF8_R {
        QCOEF8_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Quantization coefficient 9
    #[inline(always)]
    pub fn qcoef9(&self) -> QCOEF9_R {
        QCOEF9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Quantization coefficient 10
    #[inline(always)]
    pub fn qcoef10(&self) -> QCOEF10_R {
        QCOEF10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Quantization coefficient 11
    #[inline(always)]
    pub fn qcoef11(&self) -> QCOEF11_R {
        QCOEF11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QMEM3_2")
            .field("qcoef8", &self.qcoef8())
            .field("qcoef9", &self.qcoef9())
            .field("qcoef10", &self.qcoef10())
            .field("qcoef11", &self.qcoef11())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Quantization coefficient 8
    #[inline(always)]
    pub fn qcoef8(&mut self) -> QCOEF8_W<QMEM3_2rs> {
        QCOEF8_W::new(self, 0)
    }
    ///Bits 8:15 - Quantization coefficient 9
    #[inline(always)]
    pub fn qcoef9(&mut self) -> QCOEF9_W<QMEM3_2rs> {
        QCOEF9_W::new(self, 8)
    }
    ///Bits 16:23 - Quantization coefficient 10
    #[inline(always)]
    pub fn qcoef10(&mut self) -> QCOEF10_W<QMEM3_2rs> {
        QCOEF10_W::new(self, 16)
    }
    ///Bits 24:31 - Quantization coefficient 11
    #[inline(always)]
    pub fn qcoef11(&mut self) -> QCOEF11_W<QMEM3_2rs> {
        QCOEF11_W::new(self, 24)
    }
}
/**JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:QMEM3_2)*/
pub struct QMEM3_2rs;
impl crate::RegisterSpec for QMEM3_2rs {
    type Ux = u32;
}
///`read()` method returns [`qmem3_2::R`](R) reader structure
impl crate::Readable for QMEM3_2rs {}
///`write(|w| ..)` method takes [`qmem3_2::W`](W) writer structure
impl crate::Writable for QMEM3_2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets QMEM3_2 to value 0
impl crate::Resettable for QMEM3_2rs {}
