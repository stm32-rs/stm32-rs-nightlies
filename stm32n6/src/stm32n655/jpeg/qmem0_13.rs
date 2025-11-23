///Register `QMEM0_13` reader
pub type R = crate::R<QMEM0_13rs>;
///Register `QMEM0_13` writer
pub type W = crate::W<QMEM0_13rs>;
///Field `QCOEF52` reader - Quantization coefficient 52
pub type QCOEF52_R = crate::FieldReader;
///Field `QCOEF52` writer - Quantization coefficient 52
pub type QCOEF52_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF53` reader - Quantization coefficient 53
pub type QCOEF53_R = crate::FieldReader;
///Field `QCOEF53` writer - Quantization coefficient 53
pub type QCOEF53_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF54` reader - Quantization coefficient 54
pub type QCOEF54_R = crate::FieldReader;
///Field `QCOEF54` writer - Quantization coefficient 54
pub type QCOEF54_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `QCOEF55` reader - Quantization coefficient 55
pub type QCOEF55_R = crate::FieldReader;
///Field `QCOEF55` writer - Quantization coefficient 55
pub type QCOEF55_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Quantization coefficient 52
    #[inline(always)]
    pub fn qcoef52(&self) -> QCOEF52_R {
        QCOEF52_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Quantization coefficient 53
    #[inline(always)]
    pub fn qcoef53(&self) -> QCOEF53_R {
        QCOEF53_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Quantization coefficient 54
    #[inline(always)]
    pub fn qcoef54(&self) -> QCOEF54_R {
        QCOEF54_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Quantization coefficient 55
    #[inline(always)]
    pub fn qcoef55(&self) -> QCOEF55_R {
        QCOEF55_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QMEM0_13")
            .field("qcoef52", &self.qcoef52())
            .field("qcoef53", &self.qcoef53())
            .field("qcoef54", &self.qcoef54())
            .field("qcoef55", &self.qcoef55())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Quantization coefficient 52
    #[inline(always)]
    pub fn qcoef52(&mut self) -> QCOEF52_W<'_, QMEM0_13rs> {
        QCOEF52_W::new(self, 0)
    }
    ///Bits 8:15 - Quantization coefficient 53
    #[inline(always)]
    pub fn qcoef53(&mut self) -> QCOEF53_W<'_, QMEM0_13rs> {
        QCOEF53_W::new(self, 8)
    }
    ///Bits 16:23 - Quantization coefficient 54
    #[inline(always)]
    pub fn qcoef54(&mut self) -> QCOEF54_W<'_, QMEM0_13rs> {
        QCOEF54_W::new(self, 16)
    }
    ///Bits 24:31 - Quantization coefficient 55
    #[inline(always)]
    pub fn qcoef55(&mut self) -> QCOEF55_W<'_, QMEM0_13rs> {
        QCOEF55_W::new(self, 24)
    }
}
/**JPEG quantization memory 0

You can [`read`](crate::Reg::read) this register and get [`qmem0_13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem0_13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:QMEM0_13)*/
pub struct QMEM0_13rs;
impl crate::RegisterSpec for QMEM0_13rs {
    type Ux = u32;
}
///`read()` method returns [`qmem0_13::R`](R) reader structure
impl crate::Readable for QMEM0_13rs {}
///`write(|w| ..)` method takes [`qmem0_13::W`](W) writer structure
impl crate::Writable for QMEM0_13rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets QMEM0_13 to value 0
impl crate::Resettable for QMEM0_13rs {}
