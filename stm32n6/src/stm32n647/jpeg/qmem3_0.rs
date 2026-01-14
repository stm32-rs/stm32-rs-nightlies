///Register `QMEM3_0` reader
pub type R = crate::R<QMEM3_0rs>;
///Register `QMEM3_0` writer
pub type W = crate::W<QMEM3_0rs>;
///Field `QCOEF0` reader - Quantization coefficient 0
pub type QCOEF0_R = crate::FieldReader;
///Field `QCOEF0` writer - Quantization coefficient 0
pub type QCOEF0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF1` reader - Quantization coefficient 1
pub type QCOEF1_R = crate::FieldReader;
///Field `QCOEF1` writer - Quantization coefficient 1
pub type QCOEF1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF2` reader - Quantization coefficient 2
pub type QCOEF2_R = crate::FieldReader;
///Field `QCOEF2` writer - Quantization coefficient 2
pub type QCOEF2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF3` reader - Quantization coefficient 3
pub type QCOEF3_R = crate::FieldReader;
///Field `QCOEF3` writer - Quantization coefficient 3
pub type QCOEF3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Quantization coefficient 0
    #[inline(always)]
    pub fn qcoef0(&self) -> QCOEF0_R {
        QCOEF0_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Quantization coefficient 1
    #[inline(always)]
    pub fn qcoef1(&self) -> QCOEF1_R {
        QCOEF1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Quantization coefficient 2
    #[inline(always)]
    pub fn qcoef2(&self) -> QCOEF2_R {
        QCOEF2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Quantization coefficient 3
    #[inline(always)]
    pub fn qcoef3(&self) -> QCOEF3_R {
        QCOEF3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QMEM3_0")
            .field("qcoef0", &self.qcoef0())
            .field("qcoef1", &self.qcoef1())
            .field("qcoef2", &self.qcoef2())
            .field("qcoef3", &self.qcoef3())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Quantization coefficient 0
    #[inline(always)]
    pub fn qcoef0(&mut self) -> QCOEF0_W<'_, QMEM3_0rs> {
        QCOEF0_W::new(self, 0)
    }
    ///Bits 8:15 - Quantization coefficient 1
    #[inline(always)]
    pub fn qcoef1(&mut self) -> QCOEF1_W<'_, QMEM3_0rs> {
        QCOEF1_W::new(self, 8)
    }
    ///Bits 16:23 - Quantization coefficient 2
    #[inline(always)]
    pub fn qcoef2(&mut self) -> QCOEF2_W<'_, QMEM3_0rs> {
        QCOEF2_W::new(self, 16)
    }
    ///Bits 24:31 - Quantization coefficient 3
    #[inline(always)]
    pub fn qcoef3(&mut self) -> QCOEF3_W<'_, QMEM3_0rs> {
        QCOEF3_W::new(self, 24)
    }
}
/**JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:QMEM3_0)*/
pub struct QMEM3_0rs;
impl crate::RegisterSpec for QMEM3_0rs {
    type Ux = u32;
}
///`read()` method returns [`qmem3_0::R`](R) reader structure
impl crate::Readable for QMEM3_0rs {}
///`write(|w| ..)` method takes [`qmem3_0::W`](W) writer structure
impl crate::Writable for QMEM3_0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets QMEM3_0 to value 0
impl crate::Resettable for QMEM3_0rs {}
