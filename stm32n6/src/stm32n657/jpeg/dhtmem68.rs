///Register `DHTMEM68` reader
pub type R = crate::R<DHTMEM68rs>;
///Register `DHTMEM68` writer
pub type W = crate::W<DHTMEM68rs>;
///Field `DATA272` reader - Huffman table data 272
pub type DATA272_R = crate::FieldReader;
///Field `DATA272` writer - Huffman table data 272
pub type DATA272_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA273` reader - Huffman table data 273
pub type DATA273_R = crate::FieldReader;
///Field `DATA273` writer - Huffman table data 273
pub type DATA273_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA274` reader - Huffman table data 274
pub type DATA274_R = crate::FieldReader;
///Field `DATA274` writer - Huffman table data 274
pub type DATA274_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA275` reader - Huffman table data 275
pub type DATA275_R = crate::FieldReader;
///Field `DATA275` writer - Huffman table data 275
pub type DATA275_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 272
    #[inline(always)]
    pub fn data272(&self) -> DATA272_R {
        DATA272_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 273
    #[inline(always)]
    pub fn data273(&self) -> DATA273_R {
        DATA273_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 274
    #[inline(always)]
    pub fn data274(&self) -> DATA274_R {
        DATA274_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 275
    #[inline(always)]
    pub fn data275(&self) -> DATA275_R {
        DATA275_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM68")
            .field("data272", &self.data272())
            .field("data273", &self.data273())
            .field("data274", &self.data274())
            .field("data275", &self.data275())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 272
    #[inline(always)]
    pub fn data272(&mut self) -> DATA272_W<'_, DHTMEM68rs> {
        DATA272_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 273
    #[inline(always)]
    pub fn data273(&mut self) -> DATA273_W<'_, DHTMEM68rs> {
        DATA273_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 274
    #[inline(always)]
    pub fn data274(&mut self) -> DATA274_W<'_, DHTMEM68rs> {
        DATA274_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 275
    #[inline(always)]
    pub fn data275(&mut self) -> DATA275_W<'_, DHTMEM68rs> {
        DATA275_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem68::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem68::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:DHTMEM68)*/
pub struct DHTMEM68rs;
impl crate::RegisterSpec for DHTMEM68rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem68::R`](R) reader structure
impl crate::Readable for DHTMEM68rs {}
///`write(|w| ..)` method takes [`dhtmem68::W`](W) writer structure
impl crate::Writable for DHTMEM68rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM68 to value 0
impl crate::Resettable for DHTMEM68rs {}
