///Register `DHTMEM54` reader
pub type R = crate::R<DHTMEM54rs>;
///Register `DHTMEM54` writer
pub type W = crate::W<DHTMEM54rs>;
///Field `DATA216` reader - Huffman table data 216
pub type DATA216_R = crate::FieldReader;
///Field `DATA216` writer - Huffman table data 216
pub type DATA216_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA217` reader - Huffman table data 217
pub type DATA217_R = crate::FieldReader;
///Field `DATA217` writer - Huffman table data 217
pub type DATA217_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA218` reader - Huffman table data 218
pub type DATA218_R = crate::FieldReader;
///Field `DATA218` writer - Huffman table data 218
pub type DATA218_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA219` reader - Huffman table data 219
pub type DATA219_R = crate::FieldReader;
///Field `DATA219` writer - Huffman table data 219
pub type DATA219_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 216
    #[inline(always)]
    pub fn data216(&self) -> DATA216_R {
        DATA216_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 217
    #[inline(always)]
    pub fn data217(&self) -> DATA217_R {
        DATA217_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 218
    #[inline(always)]
    pub fn data218(&self) -> DATA218_R {
        DATA218_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 219
    #[inline(always)]
    pub fn data219(&self) -> DATA219_R {
        DATA219_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM54")
            .field("data216", &self.data216())
            .field("data217", &self.data217())
            .field("data218", &self.data218())
            .field("data219", &self.data219())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 216
    #[inline(always)]
    pub fn data216(&mut self) -> DATA216_W<'_, DHTMEM54rs> {
        DATA216_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 217
    #[inline(always)]
    pub fn data217(&mut self) -> DATA217_W<'_, DHTMEM54rs> {
        DATA217_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 218
    #[inline(always)]
    pub fn data218(&mut self) -> DATA218_W<'_, DHTMEM54rs> {
        DATA218_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 219
    #[inline(always)]
    pub fn data219(&mut self) -> DATA219_W<'_, DHTMEM54rs> {
        DATA219_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem54::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem54::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:DHTMEM54)*/
pub struct DHTMEM54rs;
impl crate::RegisterSpec for DHTMEM54rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem54::R`](R) reader structure
impl crate::Readable for DHTMEM54rs {}
///`write(|w| ..)` method takes [`dhtmem54::W`](W) writer structure
impl crate::Writable for DHTMEM54rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM54 to value 0
impl crate::Resettable for DHTMEM54rs {}
