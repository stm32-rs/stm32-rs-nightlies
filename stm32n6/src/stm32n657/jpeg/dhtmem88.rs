///Register `DHTMEM88` reader
pub type R = crate::R<DHTMEM88rs>;
///Register `DHTMEM88` writer
pub type W = crate::W<DHTMEM88rs>;
///Field `DATA352` reader - Huffman table data 352
pub type DATA352_R = crate::FieldReader;
///Field `DATA352` writer - Huffman table data 352
pub type DATA352_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA353` reader - Huffman table data 353
pub type DATA353_R = crate::FieldReader;
///Field `DATA353` writer - Huffman table data 353
pub type DATA353_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA354` reader - Huffman table data 354
pub type DATA354_R = crate::FieldReader;
///Field `DATA354` writer - Huffman table data 354
pub type DATA354_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA355` reader - Huffman table data 355
pub type DATA355_R = crate::FieldReader;
///Field `DATA355` writer - Huffman table data 355
pub type DATA355_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 352
    #[inline(always)]
    pub fn data352(&self) -> DATA352_R {
        DATA352_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 353
    #[inline(always)]
    pub fn data353(&self) -> DATA353_R {
        DATA353_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 354
    #[inline(always)]
    pub fn data354(&self) -> DATA354_R {
        DATA354_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 355
    #[inline(always)]
    pub fn data355(&self) -> DATA355_R {
        DATA355_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM88")
            .field("data352", &self.data352())
            .field("data353", &self.data353())
            .field("data354", &self.data354())
            .field("data355", &self.data355())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 352
    #[inline(always)]
    pub fn data352(&mut self) -> DATA352_W<'_, DHTMEM88rs> {
        DATA352_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 353
    #[inline(always)]
    pub fn data353(&mut self) -> DATA353_W<'_, DHTMEM88rs> {
        DATA353_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 354
    #[inline(always)]
    pub fn data354(&mut self) -> DATA354_W<'_, DHTMEM88rs> {
        DATA354_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 355
    #[inline(always)]
    pub fn data355(&mut self) -> DATA355_W<'_, DHTMEM88rs> {
        DATA355_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem88::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem88::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:DHTMEM88)*/
pub struct DHTMEM88rs;
impl crate::RegisterSpec for DHTMEM88rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem88::R`](R) reader structure
impl crate::Readable for DHTMEM88rs {}
///`write(|w| ..)` method takes [`dhtmem88::W`](W) writer structure
impl crate::Writable for DHTMEM88rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM88 to value 0
impl crate::Resettable for DHTMEM88rs {}
