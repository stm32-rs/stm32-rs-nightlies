///Register `DHTMEM70` reader
pub type R = crate::R<DHTMEM70rs>;
///Register `DHTMEM70` writer
pub type W = crate::W<DHTMEM70rs>;
///Field `DATA280` reader - Huffman table data 280
pub type DATA280_R = crate::FieldReader;
///Field `DATA280` writer - Huffman table data 280
pub type DATA280_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA281` reader - Huffman table data 281
pub type DATA281_R = crate::FieldReader;
///Field `DATA281` writer - Huffman table data 281
pub type DATA281_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA282` reader - Huffman table data 282
pub type DATA282_R = crate::FieldReader;
///Field `DATA282` writer - Huffman table data 282
pub type DATA282_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA283` reader - Huffman table data 283
pub type DATA283_R = crate::FieldReader;
///Field `DATA283` writer - Huffman table data 283
pub type DATA283_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 280
    #[inline(always)]
    pub fn data280(&self) -> DATA280_R {
        DATA280_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 281
    #[inline(always)]
    pub fn data281(&self) -> DATA281_R {
        DATA281_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 282
    #[inline(always)]
    pub fn data282(&self) -> DATA282_R {
        DATA282_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 283
    #[inline(always)]
    pub fn data283(&self) -> DATA283_R {
        DATA283_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM70")
            .field("data280", &self.data280())
            .field("data281", &self.data281())
            .field("data282", &self.data282())
            .field("data283", &self.data283())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 280
    #[inline(always)]
    pub fn data280(&mut self) -> DATA280_W<'_, DHTMEM70rs> {
        DATA280_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 281
    #[inline(always)]
    pub fn data281(&mut self) -> DATA281_W<'_, DHTMEM70rs> {
        DATA281_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 282
    #[inline(always)]
    pub fn data282(&mut self) -> DATA282_W<'_, DHTMEM70rs> {
        DATA282_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 283
    #[inline(always)]
    pub fn data283(&mut self) -> DATA283_W<'_, DHTMEM70rs> {
        DATA283_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem70::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem70::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DHTMEM70)*/
pub struct DHTMEM70rs;
impl crate::RegisterSpec for DHTMEM70rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem70::R`](R) reader structure
impl crate::Readable for DHTMEM70rs {}
///`write(|w| ..)` method takes [`dhtmem70::W`](W) writer structure
impl crate::Writable for DHTMEM70rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM70 to value 0
impl crate::Resettable for DHTMEM70rs {}
