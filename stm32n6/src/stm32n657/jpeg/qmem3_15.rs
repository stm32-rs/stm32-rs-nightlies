///Register `QMEM3_15` reader
pub type R = crate::R<QMEM3_15rs>;
///Register `QMEM3_15` writer
pub type W = crate::W<QMEM3_15rs>;
///Field `QCOEF60` reader - Quantization coefficient 60
pub type QCOEF60_R = crate::FieldReader;
///Field `QCOEF60` writer - Quantization coefficient 60
pub type QCOEF60_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF61` reader - Quantization coefficient 61
pub type QCOEF61_R = crate::FieldReader;
///Field `QCOEF61` writer - Quantization coefficient 61
pub type QCOEF61_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF62` reader - Quantization coefficient 62
pub type QCOEF62_R = crate::FieldReader;
///Field `QCOEF62` writer - Quantization coefficient 62
pub type QCOEF62_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF63` reader - Quantization coefficient 63
pub type QCOEF63_R = crate::FieldReader;
///Field `QCOEF63` writer - Quantization coefficient 63
pub type QCOEF63_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Quantization coefficient 60
    #[inline(always)]
    pub fn qcoef60(&self) -> QCOEF60_R {
        QCOEF60_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Quantization coefficient 61
    #[inline(always)]
    pub fn qcoef61(&self) -> QCOEF61_R {
        QCOEF61_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Quantization coefficient 62
    #[inline(always)]
    pub fn qcoef62(&self) -> QCOEF62_R {
        QCOEF62_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Quantization coefficient 63
    #[inline(always)]
    pub fn qcoef63(&self) -> QCOEF63_R {
        QCOEF63_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QMEM3_15")
            .field("qcoef60", &self.qcoef60())
            .field("qcoef61", &self.qcoef61())
            .field("qcoef62", &self.qcoef62())
            .field("qcoef63", &self.qcoef63())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Quantization coefficient 60
    #[inline(always)]
    pub fn qcoef60(&mut self) -> QCOEF60_W<'_, QMEM3_15rs> {
        QCOEF60_W::new(self, 0)
    }
    ///Bits 8:15 - Quantization coefficient 61
    #[inline(always)]
    pub fn qcoef61(&mut self) -> QCOEF61_W<'_, QMEM3_15rs> {
        QCOEF61_W::new(self, 8)
    }
    ///Bits 16:23 - Quantization coefficient 62
    #[inline(always)]
    pub fn qcoef62(&mut self) -> QCOEF62_W<'_, QMEM3_15rs> {
        QCOEF62_W::new(self, 16)
    }
    ///Bits 24:31 - Quantization coefficient 63
    #[inline(always)]
    pub fn qcoef63(&mut self) -> QCOEF63_W<'_, QMEM3_15rs> {
        QCOEF63_W::new(self, 24)
    }
}
/**JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3_15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3_15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:QMEM3_15)*/
pub struct QMEM3_15rs;
impl crate::RegisterSpec for QMEM3_15rs {
    type Ux = u32;
}
///`read()` method returns [`qmem3_15::R`](R) reader structure
impl crate::Readable for QMEM3_15rs {}
///`write(|w| ..)` method takes [`qmem3_15::W`](W) writer structure
impl crate::Writable for QMEM3_15rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets QMEM3_15 to value 0
impl crate::Resettable for QMEM3_15rs {}
