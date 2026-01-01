///Register `QMEM0_4` reader
pub type R = crate::R<QMEM0_4rs>;
///Register `QMEM0_4` writer
pub type W = crate::W<QMEM0_4rs>;
///Field `QCOEF16` reader - Quantization coefficient 16
pub type QCOEF16_R = crate::FieldReader;
///Field `QCOEF16` writer - Quantization coefficient 16
pub type QCOEF16_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF17` reader - Quantization coefficient 17
pub type QCOEF17_R = crate::FieldReader;
///Field `QCOEF17` writer - Quantization coefficient 17
pub type QCOEF17_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF18` reader - Quantization coefficient 18
pub type QCOEF18_R = crate::FieldReader;
///Field `QCOEF18` writer - Quantization coefficient 18
pub type QCOEF18_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF19` reader - Quantization coefficient 19
pub type QCOEF19_R = crate::FieldReader;
///Field `QCOEF19` writer - Quantization coefficient 19
pub type QCOEF19_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Quantization coefficient 16
    #[inline(always)]
    pub fn qcoef16(&self) -> QCOEF16_R {
        QCOEF16_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Quantization coefficient 17
    #[inline(always)]
    pub fn qcoef17(&self) -> QCOEF17_R {
        QCOEF17_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Quantization coefficient 18
    #[inline(always)]
    pub fn qcoef18(&self) -> QCOEF18_R {
        QCOEF18_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Quantization coefficient 19
    #[inline(always)]
    pub fn qcoef19(&self) -> QCOEF19_R {
        QCOEF19_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QMEM0_4")
            .field("qcoef16", &self.qcoef16())
            .field("qcoef17", &self.qcoef17())
            .field("qcoef18", &self.qcoef18())
            .field("qcoef19", &self.qcoef19())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Quantization coefficient 16
    #[inline(always)]
    pub fn qcoef16(&mut self) -> QCOEF16_W<'_, QMEM0_4rs> {
        QCOEF16_W::new(self, 0)
    }
    ///Bits 8:15 - Quantization coefficient 17
    #[inline(always)]
    pub fn qcoef17(&mut self) -> QCOEF17_W<'_, QMEM0_4rs> {
        QCOEF17_W::new(self, 8)
    }
    ///Bits 16:23 - Quantization coefficient 18
    #[inline(always)]
    pub fn qcoef18(&mut self) -> QCOEF18_W<'_, QMEM0_4rs> {
        QCOEF18_W::new(self, 16)
    }
    ///Bits 24:31 - Quantization coefficient 19
    #[inline(always)]
    pub fn qcoef19(&mut self) -> QCOEF19_W<'_, QMEM0_4rs> {
        QCOEF19_W::new(self, 24)
    }
}
/**JPEG quantization memory 0

You can [`read`](crate::Reg::read) this register and get [`qmem0_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem0_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:QMEM0_4)*/
pub struct QMEM0_4rs;
impl crate::RegisterSpec for QMEM0_4rs {
    type Ux = u32;
}
///`read()` method returns [`qmem0_4::R`](R) reader structure
impl crate::Readable for QMEM0_4rs {}
///`write(|w| ..)` method takes [`qmem0_4::W`](W) writer structure
impl crate::Writable for QMEM0_4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets QMEM0_4 to value 0
impl crate::Resettable for QMEM0_4rs {}
