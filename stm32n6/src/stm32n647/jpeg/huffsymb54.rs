///Register `HUFFSYMB54` reader
pub type R = crate::R<HUFFSYMB54rs>;
///Register `HUFFSYMB54` writer
pub type W = crate::W<HUFFSYMB54rs>;
///Field `DATA216` reader - Data 216
pub type DATA216_R = crate::FieldReader;
///Field `DATA216` writer - Data 216
pub type DATA216_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA217` reader - Data 217
pub type DATA217_R = crate::FieldReader;
///Field `DATA217` writer - Data 217
pub type DATA217_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA218` reader - Data 218
pub type DATA218_R = crate::FieldReader;
///Field `DATA218` writer - Data 218
pub type DATA218_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA219` reader - Data 219
pub type DATA219_R = crate::FieldReader;
///Field `DATA219` writer - Data 219
pub type DATA219_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 216
    #[inline(always)]
    pub fn data216(&self) -> DATA216_R {
        DATA216_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 217
    #[inline(always)]
    pub fn data217(&self) -> DATA217_R {
        DATA217_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 218
    #[inline(always)]
    pub fn data218(&self) -> DATA218_R {
        DATA218_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 219
    #[inline(always)]
    pub fn data219(&self) -> DATA219_R {
        DATA219_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB54")
            .field("data216", &self.data216())
            .field("data217", &self.data217())
            .field("data218", &self.data218())
            .field("data219", &self.data219())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 216
    #[inline(always)]
    pub fn data216(&mut self) -> DATA216_W<'_, HUFFSYMB54rs> {
        DATA216_W::new(self, 0)
    }
    ///Bits 8:15 - Data 217
    #[inline(always)]
    pub fn data217(&mut self) -> DATA217_W<'_, HUFFSYMB54rs> {
        DATA217_W::new(self, 8)
    }
    ///Bits 16:23 - Data 218
    #[inline(always)]
    pub fn data218(&mut self) -> DATA218_W<'_, HUFFSYMB54rs> {
        DATA218_W::new(self, 16)
    }
    ///Bits 24:31 - Data 219
    #[inline(always)]
    pub fn data219(&mut self) -> DATA219_W<'_, HUFFSYMB54rs> {
        DATA219_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb54::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb54::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFSYMB54)*/
pub struct HUFFSYMB54rs;
impl crate::RegisterSpec for HUFFSYMB54rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb54::R`](R) reader structure
impl crate::Readable for HUFFSYMB54rs {}
///`write(|w| ..)` method takes [`huffsymb54::W`](W) writer structure
impl crate::Writable for HUFFSYMB54rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB54 to value 0
impl crate::Resettable for HUFFSYMB54rs {}
