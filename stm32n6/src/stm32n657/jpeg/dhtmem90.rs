///Register `DHTMEM90` reader
pub type R = crate::R<DHTMEM90rs>;
///Register `DHTMEM90` writer
pub type W = crate::W<DHTMEM90rs>;
///Field `DATA360` reader - Huffman table data 360
pub type DATA360_R = crate::FieldReader;
///Field `DATA360` writer - Huffman table data 360
pub type DATA360_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA361` reader - Huffman table data 361
pub type DATA361_R = crate::FieldReader;
///Field `DATA361` writer - Huffman table data 361
pub type DATA361_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA362` reader - Huffman table data 362
pub type DATA362_R = crate::FieldReader;
///Field `DATA362` writer - Huffman table data 362
pub type DATA362_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA363` reader - Huffman table data 363
pub type DATA363_R = crate::FieldReader;
///Field `DATA363` writer - Huffman table data 363
pub type DATA363_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 360
    #[inline(always)]
    pub fn data360(&self) -> DATA360_R {
        DATA360_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 361
    #[inline(always)]
    pub fn data361(&self) -> DATA361_R {
        DATA361_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 362
    #[inline(always)]
    pub fn data362(&self) -> DATA362_R {
        DATA362_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 363
    #[inline(always)]
    pub fn data363(&self) -> DATA363_R {
        DATA363_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM90")
            .field("data360", &self.data360())
            .field("data361", &self.data361())
            .field("data362", &self.data362())
            .field("data363", &self.data363())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 360
    #[inline(always)]
    pub fn data360(&mut self) -> DATA360_W<'_, DHTMEM90rs> {
        DATA360_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 361
    #[inline(always)]
    pub fn data361(&mut self) -> DATA361_W<'_, DHTMEM90rs> {
        DATA361_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 362
    #[inline(always)]
    pub fn data362(&mut self) -> DATA362_W<'_, DHTMEM90rs> {
        DATA362_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 363
    #[inline(always)]
    pub fn data363(&mut self) -> DATA363_W<'_, DHTMEM90rs> {
        DATA363_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem90::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem90::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:DHTMEM90)*/
pub struct DHTMEM90rs;
impl crate::RegisterSpec for DHTMEM90rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem90::R`](R) reader structure
impl crate::Readable for DHTMEM90rs {}
///`write(|w| ..)` method takes [`dhtmem90::W`](W) writer structure
impl crate::Writable for DHTMEM90rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM90 to value 0
impl crate::Resettable for DHTMEM90rs {}
