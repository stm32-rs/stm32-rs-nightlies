///Register `DHTMEM39` reader
pub type R = crate::R<DHTMEM39rs>;
///Register `DHTMEM39` writer
pub type W = crate::W<DHTMEM39rs>;
///Field `DATA156` reader - Huffman table data 156
pub type DATA156_R = crate::FieldReader;
///Field `DATA156` writer - Huffman table data 156
pub type DATA156_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA157` reader - Huffman table data 157
pub type DATA157_R = crate::FieldReader;
///Field `DATA157` writer - Huffman table data 157
pub type DATA157_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA158` reader - Huffman table data 158
pub type DATA158_R = crate::FieldReader;
///Field `DATA158` writer - Huffman table data 158
pub type DATA158_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA159` reader - Huffman table data 159
pub type DATA159_R = crate::FieldReader;
///Field `DATA159` writer - Huffman table data 159
pub type DATA159_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 156
    #[inline(always)]
    pub fn data156(&self) -> DATA156_R {
        DATA156_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 157
    #[inline(always)]
    pub fn data157(&self) -> DATA157_R {
        DATA157_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 158
    #[inline(always)]
    pub fn data158(&self) -> DATA158_R {
        DATA158_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 159
    #[inline(always)]
    pub fn data159(&self) -> DATA159_R {
        DATA159_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM39")
            .field("data156", &self.data156())
            .field("data157", &self.data157())
            .field("data158", &self.data158())
            .field("data159", &self.data159())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 156
    #[inline(always)]
    pub fn data156(&mut self) -> DATA156_W<'_, DHTMEM39rs> {
        DATA156_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 157
    #[inline(always)]
    pub fn data157(&mut self) -> DATA157_W<'_, DHTMEM39rs> {
        DATA157_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 158
    #[inline(always)]
    pub fn data158(&mut self) -> DATA158_W<'_, DHTMEM39rs> {
        DATA158_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 159
    #[inline(always)]
    pub fn data159(&mut self) -> DATA159_W<'_, DHTMEM39rs> {
        DATA159_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem39::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem39::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM39)*/
pub struct DHTMEM39rs;
impl crate::RegisterSpec for DHTMEM39rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem39::R`](R) reader structure
impl crate::Readable for DHTMEM39rs {}
///`write(|w| ..)` method takes [`dhtmem39::W`](W) writer structure
impl crate::Writable for DHTMEM39rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM39 to value 0
impl crate::Resettable for DHTMEM39rs {}
