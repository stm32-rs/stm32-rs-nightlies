///Register `DHTMEM83` reader
pub type R = crate::R<DHTMEM83rs>;
///Register `DHTMEM83` writer
pub type W = crate::W<DHTMEM83rs>;
///Field `DATA332` reader - Huffman table data 332
pub type DATA332_R = crate::FieldReader;
///Field `DATA332` writer - Huffman table data 332
pub type DATA332_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA333` reader - Huffman table data 333
pub type DATA333_R = crate::FieldReader;
///Field `DATA333` writer - Huffman table data 333
pub type DATA333_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA334` reader - Huffman table data 334
pub type DATA334_R = crate::FieldReader;
///Field `DATA334` writer - Huffman table data 334
pub type DATA334_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA335` reader - Huffman table data 335
pub type DATA335_R = crate::FieldReader;
///Field `DATA335` writer - Huffman table data 335
pub type DATA335_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 332
    #[inline(always)]
    pub fn data332(&self) -> DATA332_R {
        DATA332_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 333
    #[inline(always)]
    pub fn data333(&self) -> DATA333_R {
        DATA333_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 334
    #[inline(always)]
    pub fn data334(&self) -> DATA334_R {
        DATA334_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 335
    #[inline(always)]
    pub fn data335(&self) -> DATA335_R {
        DATA335_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM83")
            .field("data332", &self.data332())
            .field("data333", &self.data333())
            .field("data334", &self.data334())
            .field("data335", &self.data335())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 332
    #[inline(always)]
    pub fn data332(&mut self) -> DATA332_W<'_, DHTMEM83rs> {
        DATA332_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 333
    #[inline(always)]
    pub fn data333(&mut self) -> DATA333_W<'_, DHTMEM83rs> {
        DATA333_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 334
    #[inline(always)]
    pub fn data334(&mut self) -> DATA334_W<'_, DHTMEM83rs> {
        DATA334_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 335
    #[inline(always)]
    pub fn data335(&mut self) -> DATA335_W<'_, DHTMEM83rs> {
        DATA335_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem83::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem83::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM83)*/
pub struct DHTMEM83rs;
impl crate::RegisterSpec for DHTMEM83rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem83::R`](R) reader structure
impl crate::Readable for DHTMEM83rs {}
///`write(|w| ..)` method takes [`dhtmem83::W`](W) writer structure
impl crate::Writable for DHTMEM83rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM83 to value 0
impl crate::Resettable for DHTMEM83rs {}
