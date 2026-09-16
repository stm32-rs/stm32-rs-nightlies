///Register `HUFFSYMB51` reader
pub type R = crate::R<HUFFSYMB51rs>;
///Register `HUFFSYMB51` writer
pub type W = crate::W<HUFFSYMB51rs>;
///Field `DATA204` reader - Data 204
pub type DATA204_R = crate::FieldReader;
///Field `DATA204` writer - Data 204
pub type DATA204_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA205` reader - Data 205
pub type DATA205_R = crate::FieldReader;
///Field `DATA205` writer - Data 205
pub type DATA205_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA206` reader - Data 206
pub type DATA206_R = crate::FieldReader;
///Field `DATA206` writer - Data 206
pub type DATA206_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA207` reader - Data 207
pub type DATA207_R = crate::FieldReader;
///Field `DATA207` writer - Data 207
pub type DATA207_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 204
    #[inline(always)]
    pub fn data204(&self) -> DATA204_R {
        DATA204_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 205
    #[inline(always)]
    pub fn data205(&self) -> DATA205_R {
        DATA205_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 206
    #[inline(always)]
    pub fn data206(&self) -> DATA206_R {
        DATA206_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 207
    #[inline(always)]
    pub fn data207(&self) -> DATA207_R {
        DATA207_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB51")
            .field("data204", &self.data204())
            .field("data205", &self.data205())
            .field("data206", &self.data206())
            .field("data207", &self.data207())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 204
    #[inline(always)]
    pub fn data204(&mut self) -> DATA204_W<'_, HUFFSYMB51rs> {
        DATA204_W::new(self, 0)
    }
    ///Bits 8:15 - Data 205
    #[inline(always)]
    pub fn data205(&mut self) -> DATA205_W<'_, HUFFSYMB51rs> {
        DATA205_W::new(self, 8)
    }
    ///Bits 16:23 - Data 206
    #[inline(always)]
    pub fn data206(&mut self) -> DATA206_W<'_, HUFFSYMB51rs> {
        DATA206_W::new(self, 16)
    }
    ///Bits 24:31 - Data 207
    #[inline(always)]
    pub fn data207(&mut self) -> DATA207_W<'_, HUFFSYMB51rs> {
        DATA207_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb51::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb51::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB51)*/
pub struct HUFFSYMB51rs;
impl crate::RegisterSpec for HUFFSYMB51rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb51::R`](R) reader structure
impl crate::Readable for HUFFSYMB51rs {}
///`write(|w| ..)` method takes [`huffsymb51::W`](W) writer structure
impl crate::Writable for HUFFSYMB51rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB51 to value 0
impl crate::Resettable for HUFFSYMB51rs {}
