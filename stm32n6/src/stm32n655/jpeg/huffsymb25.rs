///Register `HUFFSYMB25` reader
pub type R = crate::R<HUFFSYMB25rs>;
///Register `HUFFSYMB25` writer
pub type W = crate::W<HUFFSYMB25rs>;
///Field `DATA100` reader - Data 100
pub type DATA100_R = crate::FieldReader;
///Field `DATA100` writer - Data 100
pub type DATA100_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA101` reader - Data 101
pub type DATA101_R = crate::FieldReader;
///Field `DATA101` writer - Data 101
pub type DATA101_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA102` reader - Data 102
pub type DATA102_R = crate::FieldReader;
///Field `DATA102` writer - Data 102
pub type DATA102_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA103` reader - Data 103
pub type DATA103_R = crate::FieldReader;
///Field `DATA103` writer - Data 103
pub type DATA103_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 100
    #[inline(always)]
    pub fn data100(&self) -> DATA100_R {
        DATA100_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 101
    #[inline(always)]
    pub fn data101(&self) -> DATA101_R {
        DATA101_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 102
    #[inline(always)]
    pub fn data102(&self) -> DATA102_R {
        DATA102_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 103
    #[inline(always)]
    pub fn data103(&self) -> DATA103_R {
        DATA103_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB25")
            .field("data100", &self.data100())
            .field("data101", &self.data101())
            .field("data102", &self.data102())
            .field("data103", &self.data103())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 100
    #[inline(always)]
    pub fn data100(&mut self) -> DATA100_W<'_, HUFFSYMB25rs> {
        DATA100_W::new(self, 0)
    }
    ///Bits 8:15 - Data 101
    #[inline(always)]
    pub fn data101(&mut self) -> DATA101_W<'_, HUFFSYMB25rs> {
        DATA101_W::new(self, 8)
    }
    ///Bits 16:23 - Data 102
    #[inline(always)]
    pub fn data102(&mut self) -> DATA102_W<'_, HUFFSYMB25rs> {
        DATA102_W::new(self, 16)
    }
    ///Bits 24:31 - Data 103
    #[inline(always)]
    pub fn data103(&mut self) -> DATA103_W<'_, HUFFSYMB25rs> {
        DATA103_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB25)*/
pub struct HUFFSYMB25rs;
impl crate::RegisterSpec for HUFFSYMB25rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb25::R`](R) reader structure
impl crate::Readable for HUFFSYMB25rs {}
///`write(|w| ..)` method takes [`huffsymb25::W`](W) writer structure
impl crate::Writable for HUFFSYMB25rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB25 to value 0
impl crate::Resettable for HUFFSYMB25rs {}
