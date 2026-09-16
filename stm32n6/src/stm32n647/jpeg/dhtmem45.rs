///Register `DHTMEM45` reader
pub type R = crate::R<DHTMEM45rs>;
///Register `DHTMEM45` writer
pub type W = crate::W<DHTMEM45rs>;
///Field `DATA180` reader - Huffman table data 180
pub type DATA180_R = crate::FieldReader;
///Field `DATA180` writer - Huffman table data 180
pub type DATA180_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA181` reader - Huffman table data 181
pub type DATA181_R = crate::FieldReader;
///Field `DATA181` writer - Huffman table data 181
pub type DATA181_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA182` reader - Huffman table data 182
pub type DATA182_R = crate::FieldReader;
///Field `DATA182` writer - Huffman table data 182
pub type DATA182_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA183` reader - Huffman table data 183
pub type DATA183_R = crate::FieldReader;
///Field `DATA183` writer - Huffman table data 183
pub type DATA183_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 180
    #[inline(always)]
    pub fn data180(&self) -> DATA180_R {
        DATA180_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 181
    #[inline(always)]
    pub fn data181(&self) -> DATA181_R {
        DATA181_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 182
    #[inline(always)]
    pub fn data182(&self) -> DATA182_R {
        DATA182_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 183
    #[inline(always)]
    pub fn data183(&self) -> DATA183_R {
        DATA183_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM45")
            .field("data180", &self.data180())
            .field("data181", &self.data181())
            .field("data182", &self.data182())
            .field("data183", &self.data183())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 180
    #[inline(always)]
    pub fn data180(&mut self) -> DATA180_W<'_, DHTMEM45rs> {
        DATA180_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 181
    #[inline(always)]
    pub fn data181(&mut self) -> DATA181_W<'_, DHTMEM45rs> {
        DATA181_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 182
    #[inline(always)]
    pub fn data182(&mut self) -> DATA182_W<'_, DHTMEM45rs> {
        DATA182_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 183
    #[inline(always)]
    pub fn data183(&mut self) -> DATA183_W<'_, DHTMEM45rs> {
        DATA183_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem45::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem45::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM45)*/
pub struct DHTMEM45rs;
impl crate::RegisterSpec for DHTMEM45rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem45::R`](R) reader structure
impl crate::Readable for DHTMEM45rs {}
///`write(|w| ..)` method takes [`dhtmem45::W`](W) writer structure
impl crate::Writable for DHTMEM45rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM45 to value 0
impl crate::Resettable for DHTMEM45rs {}
