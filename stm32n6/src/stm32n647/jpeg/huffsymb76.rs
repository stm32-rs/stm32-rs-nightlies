///Register `HUFFSYMB76` reader
pub type R = crate::R<HUFFSYMB76rs>;
///Register `HUFFSYMB76` writer
pub type W = crate::W<HUFFSYMB76rs>;
///Field `DATA304` reader - Data 304
pub type DATA304_R = crate::FieldReader;
///Field `DATA304` writer - Data 304
pub type DATA304_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA305` reader - Data 305
pub type DATA305_R = crate::FieldReader;
///Field `DATA305` writer - Data 305
pub type DATA305_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA306` reader - Data 306
pub type DATA306_R = crate::FieldReader;
///Field `DATA306` writer - Data 306
pub type DATA306_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA307` reader - Data 307
pub type DATA307_R = crate::FieldReader;
///Field `DATA307` writer - Data 307
pub type DATA307_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 304
    #[inline(always)]
    pub fn data304(&self) -> DATA304_R {
        DATA304_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 305
    #[inline(always)]
    pub fn data305(&self) -> DATA305_R {
        DATA305_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 306
    #[inline(always)]
    pub fn data306(&self) -> DATA306_R {
        DATA306_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 307
    #[inline(always)]
    pub fn data307(&self) -> DATA307_R {
        DATA307_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB76")
            .field("data304", &self.data304())
            .field("data305", &self.data305())
            .field("data306", &self.data306())
            .field("data307", &self.data307())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 304
    #[inline(always)]
    pub fn data304(&mut self) -> DATA304_W<'_, HUFFSYMB76rs> {
        DATA304_W::new(self, 0)
    }
    ///Bits 8:15 - Data 305
    #[inline(always)]
    pub fn data305(&mut self) -> DATA305_W<'_, HUFFSYMB76rs> {
        DATA305_W::new(self, 8)
    }
    ///Bits 16:23 - Data 306
    #[inline(always)]
    pub fn data306(&mut self) -> DATA306_W<'_, HUFFSYMB76rs> {
        DATA306_W::new(self, 16)
    }
    ///Bits 24:31 - Data 307
    #[inline(always)]
    pub fn data307(&mut self) -> DATA307_W<'_, HUFFSYMB76rs> {
        DATA307_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb76::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb76::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFSYMB76)*/
pub struct HUFFSYMB76rs;
impl crate::RegisterSpec for HUFFSYMB76rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb76::R`](R) reader structure
impl crate::Readable for HUFFSYMB76rs {}
///`write(|w| ..)` method takes [`huffsymb76::W`](W) writer structure
impl crate::Writable for HUFFSYMB76rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB76 to value 0
impl crate::Resettable for HUFFSYMB76rs {}
