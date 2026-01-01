///Register `DHTMEM0` reader
pub type R = crate::R<DHTMEM0rs>;
///Register `DHTMEM0` writer
pub type W = crate::W<DHTMEM0rs>;
///Field `DATA0` reader - Huffman table data 0
pub type DATA0_R = crate::FieldReader;
///Field `DATA0` writer - Huffman table data 0
pub type DATA0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA1` reader - Huffman table data 1
pub type DATA1_R = crate::FieldReader;
///Field `DATA1` writer - Huffman table data 1
pub type DATA1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA2` reader - Huffman table data 2
pub type DATA2_R = crate::FieldReader;
///Field `DATA2` writer - Huffman table data 2
pub type DATA2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA3` reader - Huffman table data 3
pub type DATA3_R = crate::FieldReader;
///Field `DATA3` writer - Huffman table data 3
pub type DATA3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Huffman table data 0
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Huffman table data 1
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Huffman table data 2
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Huffman table data 3
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHTMEM0")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .field("data2", &self.data2())
            .field("data3", &self.data3())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman table data 0
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W<'_, DHTMEM0rs> {
        DATA0_W::new(self, 0)
    }
    ///Bits 8:15 - Huffman table data 1
    #[inline(always)]
    pub fn data1(&mut self) -> DATA1_W<'_, DHTMEM0rs> {
        DATA1_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman table data 2
    #[inline(always)]
    pub fn data2(&mut self) -> DATA2_W<'_, DHTMEM0rs> {
        DATA2_W::new(self, 16)
    }
    ///Bits 24:31 - Huffman table data 3
    #[inline(always)]
    pub fn data3(&mut self) -> DATA3_W<'_, DHTMEM0rs> {
        DATA3_W::new(self, 24)
    }
}
/**JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:DHTMEM0)*/
pub struct DHTMEM0rs;
impl crate::RegisterSpec for DHTMEM0rs {
    type Ux = u32;
}
///`read()` method returns [`dhtmem0::R`](R) reader structure
impl crate::Readable for DHTMEM0rs {}
///`write(|w| ..)` method takes [`dhtmem0::W`](W) writer structure
impl crate::Writable for DHTMEM0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHTMEM0 to value 0
impl crate::Resettable for DHTMEM0rs {}
