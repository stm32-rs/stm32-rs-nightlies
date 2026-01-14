///Register `DHTMEM48` reader
pub type R = crate::R<DHTMEM48rs>;
///Register `DHTMEM48` writer
pub type W = crate::W<DHTMEM48rs>;
///Field `DATA192` reader - Huffman table data 192
pub type DATA192_R = crate::FieldReader;
///Field `DATA192` writer - Huffman table data 192
pub type DATA192_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA193` reader - Huffman table data 193
pub type DATA193_R = crate::FieldReader;
///Field `DATA193` writer - Huffman table data 193
pub type DATA193_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA194` reader - Huffman table data 194
pub type DATA194_R = crate::FieldReader;
///Field `DATA194` writer - Huffman table data 194
pub type DATA194_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA195` reader - Huffman table data 195
pub type DATA195_R = crate::FieldReader;
///Field `DATA195` writer - Huffman table data 195
pub type DATA195_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 192
    #[inline(always)]
    pub fn data192(&self) -> DATA192_R {
        DATA192_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 193
    #[inline(always)]
    pub fn data193(&self) -> DATA193_R {
        DATA193_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 194
    #[inline(always)]
    pub fn data194(&self) -> DATA194_R {
        DATA194_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 195
    #[inline(always)]
    pub fn data195(&self) -> DATA195_R {
        DATA195_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM48")
            .field("data192", &self.data192())
            .field("data193", &self.data193())
            .field("data194", &self.data194())
            .field("data195", &self.data195())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 192
    #[inline(always)]
    pub fn data192(&mut self) -> DATA192_W<'_, DHTMEM48rs> {
        DATA192_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 193
    #[inline(always)]
    pub fn data193(&mut self) -> DATA193_W<'_, DHTMEM48rs> {
        DATA193_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 194
    #[inline(always)]
    pub fn data194(&mut self) -> DATA194_W<'_, DHTMEM48rs> {
        DATA194_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 195
    #[inline(always)]
    pub fn data195(&mut self) -> DATA195_W<'_, DHTMEM48rs> {
        DATA195_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem48::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem48::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:DHTMEM48)*/
pub struct DHTMEM48rs;
impl crate::RegisterSpec for DHTMEM48rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem48::R`](R) reader structure
impl crate::Readable for DHTMEM48rs {}
///`write(|w| ..)` method takes [`dhtmem48::W`](W) writer structure
impl crate::Writable for DHTMEM48rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM48 to value 0
impl crate::Resettable for DHTMEM48rs {}
