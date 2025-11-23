///Register `QMEM3_12` reader
pub type R = crate::R<QMEM3_12rs>;
///Register `QMEM3_12` writer
pub type W = crate::W<QMEM3_12rs>;
///Field `QCOEF48` reader - Quantization coefficient 48
pub type QCOEF48_R = crate::FieldReader;
///Field `QCOEF48` writer - Quantization coefficient 48
pub type QCOEF48_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF49` reader - Quantization coefficient 49
pub type QCOEF49_R = crate::FieldReader;
///Field `QCOEF49` writer - Quantization coefficient 49
pub type QCOEF49_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF50` reader - Quantization coefficient 50
pub type QCOEF50_R = crate::FieldReader;
///Field `QCOEF50` writer - Quantization coefficient 50
pub type QCOEF50_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF51` reader - Quantization coefficient 51
pub type QCOEF51_R = crate::FieldReader;
///Field `QCOEF51` writer - Quantization coefficient 51
pub type QCOEF51_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Quantization coefficient 48
    #[inline(always)]
    pub fn qcoef48(&self) -> QCOEF48_R {
        QCOEF48_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Quantization coefficient 49
    #[inline(always)]
    pub fn qcoef49(&self) -> QCOEF49_R {
        QCOEF49_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Quantization coefficient 50
    #[inline(always)]
    pub fn qcoef50(&self) -> QCOEF50_R {
        QCOEF50_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Quantization coefficient 51
    #[inline(always)]
    pub fn qcoef51(&self) -> QCOEF51_R {
        QCOEF51_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QMEM3_12")
            .field("qcoef48", &self.qcoef48())
            .field("qcoef49", &self.qcoef49())
            .field("qcoef50", &self.qcoef50())
            .field("qcoef51", &self.qcoef51())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Quantization coefficient 48
    #[inline(always)]
    pub fn qcoef48(&mut self) -> QCOEF48_W<'_, QMEM3_12rs> {
        QCOEF48_W::new(self, 0)
    }
    ///Bits 8:15 - Quantization coefficient 49
    #[inline(always)]
    pub fn qcoef49(&mut self) -> QCOEF49_W<'_, QMEM3_12rs> {
        QCOEF49_W::new(self, 8)
    }
    ///Bits 16:23 - Quantization coefficient 50
    #[inline(always)]
    pub fn qcoef50(&mut self) -> QCOEF50_W<'_, QMEM3_12rs> {
        QCOEF50_W::new(self, 16)
    }
    ///Bits 24:31 - Quantization coefficient 51
    #[inline(always)]
    pub fn qcoef51(&mut self) -> QCOEF51_W<'_, QMEM3_12rs> {
        QCOEF51_W::new(self, 24)
    }
}
/**JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3_12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3_12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:QMEM3_12)*/
pub struct QMEM3_12rs;
impl crate::RegisterSpec for QMEM3_12rs {
    type Ux = u32;
}
///`read()` method returns [`qmem3_12::R`](R) reader structure
impl crate::Readable for QMEM3_12rs {}
///`write(|w| ..)` method takes [`qmem3_12::W`](W) writer structure
impl crate::Writable for QMEM3_12rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets QMEM3_12 to value 0
impl crate::Resettable for QMEM3_12rs {}
