///Register `DHTMEM79` reader
pub type R = crate::R<DHTMEM79rs>;
///Register `DHTMEM79` writer
pub type W = crate::W<DHTMEM79rs>;
///Field `DATA316` reader - Huffman table data 316
pub type DATA316_R = crate::FieldReader;
///Field `DATA316` writer - Huffman table data 316
pub type DATA316_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA317` reader - Huffman table data 317
pub type DATA317_R = crate::FieldReader;
///Field `DATA317` writer - Huffman table data 317
pub type DATA317_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA318` reader - Huffman table data 318
pub type DATA318_R = crate::FieldReader;
///Field `DATA318` writer - Huffman table data 318
pub type DATA318_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA319` reader - Huffman table data 319
pub type DATA319_R = crate::FieldReader;
///Field `DATA319` writer - Huffman table data 319
pub type DATA319_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 316
    #[inline(always)]
    pub fn data316(&self) -> DATA316_R {
        DATA316_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 317
    #[inline(always)]
    pub fn data317(&self) -> DATA317_R {
        DATA317_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 318
    #[inline(always)]
    pub fn data318(&self) -> DATA318_R {
        DATA318_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 319
    #[inline(always)]
    pub fn data319(&self) -> DATA319_R {
        DATA319_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM79")
            .field("data316", &self.data316())
            .field("data317", &self.data317())
            .field("data318", &self.data318())
            .field("data319", &self.data319())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 316
    #[inline(always)]
    pub fn data316(&mut self) -> DATA316_W<'_, DHTMEM79rs> {
        DATA316_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 317
    #[inline(always)]
    pub fn data317(&mut self) -> DATA317_W<'_, DHTMEM79rs> {
        DATA317_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 318
    #[inline(always)]
    pub fn data318(&mut self) -> DATA318_W<'_, DHTMEM79rs> {
        DATA318_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 319
    #[inline(always)]
    pub fn data319(&mut self) -> DATA319_W<'_, DHTMEM79rs> {
        DATA319_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem79::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem79::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:DHTMEM79)*/
pub struct DHTMEM79rs;
impl crate::RegisterSpec for DHTMEM79rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem79::R`](R) reader structure
impl crate::Readable for DHTMEM79rs {}
///`write(|w| ..)` method takes [`dhtmem79::W`](W) writer structure
impl crate::Writable for DHTMEM79rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM79 to value 0
impl crate::Resettable for DHTMEM79rs {}
