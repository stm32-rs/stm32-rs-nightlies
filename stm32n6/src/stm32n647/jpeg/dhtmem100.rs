///Register `DHTMEM100` reader
pub type R = crate::R<DHTMEM100rs>;
///Register `DHTMEM100` writer
pub type W = crate::W<DHTMEM100rs>;
///Field `DATA400` reader - Huffman table data 400
pub type DATA400_R = crate::FieldReader;
///Field `DATA400` writer - Huffman table data 400
pub type DATA400_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA401` reader - Huffman table data 401
pub type DATA401_R = crate::FieldReader;
///Field `DATA401` writer - Huffman table data 401
pub type DATA401_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA402` reader - Huffman table data 402
pub type DATA402_R = crate::FieldReader;
///Field `DATA402` writer - Huffman table data 402
pub type DATA402_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA403` reader - Huffman table data 403
pub type DATA403_R = crate::FieldReader;
///Field `DATA403` writer - Huffman table data 403
pub type DATA403_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 400
    #[inline(always)]
    pub fn data400(&self) -> DATA400_R {
        DATA400_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 401
    #[inline(always)]
    pub fn data401(&self) -> DATA401_R {
        DATA401_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 402
    #[inline(always)]
    pub fn data402(&self) -> DATA402_R {
        DATA402_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 403
    #[inline(always)]
    pub fn data403(&self) -> DATA403_R {
        DATA403_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM100")
            .field("data400", &self.data400())
            .field("data401", &self.data401())
            .field("data402", &self.data402())
            .field("data403", &self.data403())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 400
    #[inline(always)]
    pub fn data400(&mut self) -> DATA400_W<'_, DHTMEM100rs> {
        DATA400_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 401
    #[inline(always)]
    pub fn data401(&mut self) -> DATA401_W<'_, DHTMEM100rs> {
        DATA401_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 402
    #[inline(always)]
    pub fn data402(&mut self) -> DATA402_W<'_, DHTMEM100rs> {
        DATA402_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 403
    #[inline(always)]
    pub fn data403(&mut self) -> DATA403_W<'_, DHTMEM100rs> {
        DATA403_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem100::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem100::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM100)*/
pub struct DHTMEM100rs;
impl crate::RegisterSpec for DHTMEM100rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem100::R`](R) reader structure
impl crate::Readable for DHTMEM100rs {}
///`write(|w| ..)` method takes [`dhtmem100::W`](W) writer structure
impl crate::Writable for DHTMEM100rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM100 to value 0
impl crate::Resettable for DHTMEM100rs {}
