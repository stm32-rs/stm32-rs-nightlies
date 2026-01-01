///Register `HUFFENC_AC1_24` reader
pub type R = crate::R<HUFFENC_AC1_24rs>;
///Register `HUFFENC_AC1_24` writer
pub type W = crate::W<HUFFENC_AC1_24rs>;
///Field `HCODE48` reader - Huffman code 48
pub type HCODE48_R = crate::FieldReader;
///Field `HCODE48` writer - Huffman code 48
pub type HCODE48_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN48` reader - Huffman length 48
pub type HLEN48_R = crate::FieldReader;
///Field `HLEN48` writer - Huffman length 48
pub type HLEN48_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE49` reader - Huffman code 49
pub type HCODE49_R = crate::FieldReader;
///Field `HCODE49` writer - Huffman code 49
pub type HCODE49_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN49` reader - Huffman length 49
pub type HLEN49_R = crate::FieldReader;
///Field `HLEN49` writer - Huffman length 49
pub type HLEN49_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 48
    #[inline(always)]
    pub fn hcode48(&self) -> HCODE48_R {
        HCODE48_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 48
    #[inline(always)]
    pub fn hlen48(&self) -> HLEN48_R {
        HLEN48_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 49
    #[inline(always)]
    pub fn hcode49(&self) -> HCODE49_R {
        HCODE49_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 49
    #[inline(always)]
    pub fn hlen49(&self) -> HLEN49_R {
        HLEN49_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_24")
            .field("hcode48", &self.hcode48())
            .field("hlen48", &self.hlen48())
            .field("hcode49", &self.hcode49())
            .field("hlen49", &self.hlen49())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 48
    #[inline(always)]
    pub fn hcode48(&mut self) -> HCODE48_W<'_, HUFFENC_AC1_24rs> {
        HCODE48_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 48
    #[inline(always)]
    pub fn hlen48(&mut self) -> HLEN48_W<'_, HUFFENC_AC1_24rs> {
        HLEN48_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 49
    #[inline(always)]
    pub fn hcode49(&mut self) -> HCODE49_W<'_, HUFFENC_AC1_24rs> {
        HCODE49_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 49
    #[inline(always)]
    pub fn hlen49(&mut self) -> HLEN49_W<'_, HUFFENC_AC1_24rs> {
        HLEN49_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFENC_AC1_24)*/
pub struct HUFFENC_AC1_24rs;
impl crate::RegisterSpec for HUFFENC_AC1_24rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_24::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_24rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_24::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_24rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_24 to value 0
impl crate::Resettable for HUFFENC_AC1_24rs {}
