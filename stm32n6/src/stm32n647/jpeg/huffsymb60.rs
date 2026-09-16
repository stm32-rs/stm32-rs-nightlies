///Register `HUFFSYMB60` reader
pub type R = crate::R<HUFFSYMB60rs>;
///Register `HUFFSYMB60` writer
pub type W = crate::W<HUFFSYMB60rs>;
///Field `DATA240` reader - Data 240
pub type DATA240_R = crate::FieldReader;
///Field `DATA240` writer - Data 240
pub type DATA240_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA241` reader - Data 241
pub type DATA241_R = crate::FieldReader;
///Field `DATA241` writer - Data 241
pub type DATA241_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA242` reader - Data 242
pub type DATA242_R = crate::FieldReader;
///Field `DATA242` writer - Data 242
pub type DATA242_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA243` reader - Data 243
pub type DATA243_R = crate::FieldReader;
///Field `DATA243` writer - Data 243
pub type DATA243_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 240
    #[inline(always)]
    pub fn data240(&self) -> DATA240_R {
        DATA240_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 241
    #[inline(always)]
    pub fn data241(&self) -> DATA241_R {
        DATA241_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 242
    #[inline(always)]
    pub fn data242(&self) -> DATA242_R {
        DATA242_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 243
    #[inline(always)]
    pub fn data243(&self) -> DATA243_R {
        DATA243_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB60")
            .field("data240", &self.data240())
            .field("data241", &self.data241())
            .field("data242", &self.data242())
            .field("data243", &self.data243())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 240
    #[inline(always)]
    pub fn data240(&mut self) -> DATA240_W<'_, HUFFSYMB60rs> {
        DATA240_W::new(self, 0)
    }
    ///Bits 8:15 - Data 241
    #[inline(always)]
    pub fn data241(&mut self) -> DATA241_W<'_, HUFFSYMB60rs> {
        DATA241_W::new(self, 8)
    }
    ///Bits 16:23 - Data 242
    #[inline(always)]
    pub fn data242(&mut self) -> DATA242_W<'_, HUFFSYMB60rs> {
        DATA242_W::new(self, 16)
    }
    ///Bits 24:31 - Data 243
    #[inline(always)]
    pub fn data243(&mut self) -> DATA243_W<'_, HUFFSYMB60rs> {
        DATA243_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb60::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb60::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFSYMB60)*/
pub struct HUFFSYMB60rs;
impl crate::RegisterSpec for HUFFSYMB60rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb60::R`](R) reader structure
impl crate::Readable for HUFFSYMB60rs {}
///`write(|w| ..)` method takes [`huffsymb60::W`](W) writer structure
impl crate::Writable for HUFFSYMB60rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB60 to value 0
impl crate::Resettable for HUFFSYMB60rs {}
