///Register `QMEM3_9` reader
pub type R = crate::R<QMEM3_9rs>;
///Register `QMEM3_9` writer
pub type W = crate::W<QMEM3_9rs>;
///Field `QCOEF36` reader - Quantization coefficient 36
pub type QCOEF36_R = crate::FieldReader;
///Field `QCOEF36` writer - Quantization coefficient 36
pub type QCOEF36_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF37` reader - Quantization coefficient 37
pub type QCOEF37_R = crate::FieldReader;
///Field `QCOEF37` writer - Quantization coefficient 37
pub type QCOEF37_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF38` reader - Quantization coefficient 38
pub type QCOEF38_R = crate::FieldReader;
///Field `QCOEF38` writer - Quantization coefficient 38
pub type QCOEF38_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF39` reader - Quantization coefficient 39
pub type QCOEF39_R = crate::FieldReader;
///Field `QCOEF39` writer - Quantization coefficient 39
pub type QCOEF39_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Quantization coefficient 36
    #[inline(always)]
    pub fn qcoef36(&self) -> QCOEF36_R {
        QCOEF36_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Quantization coefficient 37
    #[inline(always)]
    pub fn qcoef37(&self) -> QCOEF37_R {
        QCOEF37_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Quantization coefficient 38
    #[inline(always)]
    pub fn qcoef38(&self) -> QCOEF38_R {
        QCOEF38_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Quantization coefficient 39
    #[inline(always)]
    pub fn qcoef39(&self) -> QCOEF39_R {
        QCOEF39_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QMEM3_9")
            .field("qcoef36", &self.qcoef36())
            .field("qcoef37", &self.qcoef37())
            .field("qcoef38", &self.qcoef38())
            .field("qcoef39", &self.qcoef39())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Quantization coefficient 36
    #[inline(always)]
    pub fn qcoef36(&mut self) -> QCOEF36_W<'_, QMEM3_9rs> {
        QCOEF36_W::new(self, 0)
    }
    ///Bits 8:15 - Quantization coefficient 37
    #[inline(always)]
    pub fn qcoef37(&mut self) -> QCOEF37_W<'_, QMEM3_9rs> {
        QCOEF37_W::new(self, 8)
    }
    ///Bits 16:23 - Quantization coefficient 38
    #[inline(always)]
    pub fn qcoef38(&mut self) -> QCOEF38_W<'_, QMEM3_9rs> {
        QCOEF38_W::new(self, 16)
    }
    ///Bits 24:31 - Quantization coefficient 39
    #[inline(always)]
    pub fn qcoef39(&mut self) -> QCOEF39_W<'_, QMEM3_9rs> {
        QCOEF39_W::new(self, 24)
    }
}
/**JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3_9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3_9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM3_9)*/
pub struct QMEM3_9rs;
impl crate::RegisterSpec for QMEM3_9rs {
    type Ux = u32;
}
///`read()` method returns [`qmem3_9::R`](R) reader structure
impl crate::Readable for QMEM3_9rs {}
///`write(|w| ..)` method takes [`qmem3_9::W`](W) writer structure
impl crate::Writable for QMEM3_9rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets QMEM3_9 to value 0
impl crate::Resettable for QMEM3_9rs {}
