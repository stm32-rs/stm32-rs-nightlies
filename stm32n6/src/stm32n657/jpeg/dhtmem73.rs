///Register `DHTMEM73` reader
pub type R = crate::R<DHTMEM73rs>;
///Register `DHTMEM73` writer
pub type W = crate::W<DHTMEM73rs>;
///Field `DATA292` reader - Huffman table data 292
pub type DATA292_R = crate::FieldReader;
///Field `DATA292` writer - Huffman table data 292
pub type DATA292_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA293` reader - Huffman table data 293
pub type DATA293_R = crate::FieldReader;
///Field `DATA293` writer - Huffman table data 293
pub type DATA293_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA294` reader - Huffman table data 294
pub type DATA294_R = crate::FieldReader;
///Field `DATA294` writer - Huffman table data 294
pub type DATA294_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA295` reader - Huffman table data 295
pub type DATA295_R = crate::FieldReader;
///Field `DATA295` writer - Huffman table data 295
pub type DATA295_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 292
    #[inline(always)]
    pub fn data292(&self) -> DATA292_R {
        DATA292_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 293
    #[inline(always)]
    pub fn data293(&self) -> DATA293_R {
        DATA293_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 294
    #[inline(always)]
    pub fn data294(&self) -> DATA294_R {
        DATA294_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 295
    #[inline(always)]
    pub fn data295(&self) -> DATA295_R {
        DATA295_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM73")
            .field("data292", &self.data292())
            .field("data293", &self.data293())
            .field("data294", &self.data294())
            .field("data295", &self.data295())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 292
    #[inline(always)]
    pub fn data292(&mut self) -> DATA292_W<'_, DHTMEM73rs> {
        DATA292_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 293
    #[inline(always)]
    pub fn data293(&mut self) -> DATA293_W<'_, DHTMEM73rs> {
        DATA293_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 294
    #[inline(always)]
    pub fn data294(&mut self) -> DATA294_W<'_, DHTMEM73rs> {
        DATA294_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 295
    #[inline(always)]
    pub fn data295(&mut self) -> DATA295_W<'_, DHTMEM73rs> {
        DATA295_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem73::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem73::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:DHTMEM73)*/
pub struct DHTMEM73rs;
impl crate::RegisterSpec for DHTMEM73rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem73::R`](R) reader structure
impl crate::Readable for DHTMEM73rs {}
///`write(|w| ..)` method takes [`dhtmem73::W`](W) writer structure
impl crate::Writable for DHTMEM73rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM73 to value 0
impl crate::Resettable for DHTMEM73rs {}
