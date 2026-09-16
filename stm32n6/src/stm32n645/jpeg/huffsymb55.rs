///Register `HUFFSYMB55` reader
pub type R = crate::R<HUFFSYMB55rs>;
///Register `HUFFSYMB55` writer
pub type W = crate::W<HUFFSYMB55rs>;
///Field `DATA220` reader - Data 220
pub type DATA220_R = crate::FieldReader;
///Field `DATA220` writer - Data 220
pub type DATA220_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA221` reader - Data 221
pub type DATA221_R = crate::FieldReader;
///Field `DATA221` writer - Data 221
pub type DATA221_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA222` reader - Data 222
pub type DATA222_R = crate::FieldReader;
///Field `DATA222` writer - Data 222
pub type DATA222_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA223` reader - Data 223
pub type DATA223_R = crate::FieldReader;
///Field `DATA223` writer - Data 223
pub type DATA223_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 220
    #[inline(always)]
    pub fn data220(&self) -> DATA220_R {
        DATA220_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 221
    #[inline(always)]
    pub fn data221(&self) -> DATA221_R {
        DATA221_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 222
    #[inline(always)]
    pub fn data222(&self) -> DATA222_R {
        DATA222_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 223
    #[inline(always)]
    pub fn data223(&self) -> DATA223_R {
        DATA223_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB55")
            .field("data220", &self.data220())
            .field("data221", &self.data221())
            .field("data222", &self.data222())
            .field("data223", &self.data223())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 220
    #[inline(always)]
    pub fn data220(&mut self) -> DATA220_W<'_, HUFFSYMB55rs> {
        DATA220_W::new(self, 0)
    }
    ///Bits 8:15 - Data 221
    #[inline(always)]
    pub fn data221(&mut self) -> DATA221_W<'_, HUFFSYMB55rs> {
        DATA221_W::new(self, 8)
    }
    ///Bits 16:23 - Data 222
    #[inline(always)]
    pub fn data222(&mut self) -> DATA222_W<'_, HUFFSYMB55rs> {
        DATA222_W::new(self, 16)
    }
    ///Bits 24:31 - Data 223
    #[inline(always)]
    pub fn data223(&mut self) -> DATA223_W<'_, HUFFSYMB55rs> {
        DATA223_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb55::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb55::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFSYMB55)*/
pub struct HUFFSYMB55rs;
impl crate::RegisterSpec for HUFFSYMB55rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb55::R`](R) reader structure
impl crate::Readable for HUFFSYMB55rs {}
///`write(|w| ..)` method takes [`huffsymb55::W`](W) writer structure
impl crate::Writable for HUFFSYMB55rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB55 to value 0
impl crate::Resettable for HUFFSYMB55rs {}
