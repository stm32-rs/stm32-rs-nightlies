///Register `DHTMEM78` reader
pub type R = crate::R<DHTMEM78rs>;
///Register `DHTMEM78` writer
pub type W = crate::W<DHTMEM78rs>;
///Field `DATA312` reader - Huffman table data 312
pub type DATA312_R = crate::FieldReader;
///Field `DATA312` writer - Huffman table data 312
pub type DATA312_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA313` reader - Huffman table data 313
pub type DATA313_R = crate::FieldReader;
///Field `DATA313` writer - Huffman table data 313
pub type DATA313_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA314` reader - Huffman table data 314
pub type DATA314_R = crate::FieldReader;
///Field `DATA314` writer - Huffman table data 314
pub type DATA314_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA315` reader - Huffman table data 315
pub type DATA315_R = crate::FieldReader;
///Field `DATA315` writer - Huffman table data 315
pub type DATA315_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 312
    #[inline(always)]
    pub fn data312(&self) -> DATA312_R {
        DATA312_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 313
    #[inline(always)]
    pub fn data313(&self) -> DATA313_R {
        DATA313_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 314
    #[inline(always)]
    pub fn data314(&self) -> DATA314_R {
        DATA314_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 315
    #[inline(always)]
    pub fn data315(&self) -> DATA315_R {
        DATA315_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM78")
            .field("data312", &self.data312())
            .field("data313", &self.data313())
            .field("data314", &self.data314())
            .field("data315", &self.data315())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 312
    #[inline(always)]
    pub fn data312(&mut self) -> DATA312_W<'_, DHTMEM78rs> {
        DATA312_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 313
    #[inline(always)]
    pub fn data313(&mut self) -> DATA313_W<'_, DHTMEM78rs> {
        DATA313_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 314
    #[inline(always)]
    pub fn data314(&mut self) -> DATA314_W<'_, DHTMEM78rs> {
        DATA314_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 315
    #[inline(always)]
    pub fn data315(&mut self) -> DATA315_W<'_, DHTMEM78rs> {
        DATA315_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem78::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem78::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:DHTMEM78)*/
pub struct DHTMEM78rs;
impl crate::RegisterSpec for DHTMEM78rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem78::R`](R) reader structure
impl crate::Readable for DHTMEM78rs {}
///`write(|w| ..)` method takes [`dhtmem78::W`](W) writer structure
impl crate::Writable for DHTMEM78rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM78 to value 0
impl crate::Resettable for DHTMEM78rs {}
