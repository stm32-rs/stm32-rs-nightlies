///Register `HUFFENC_AC1_32` reader
pub type R = crate::R<HUFFENC_AC1_32rs>;
///Register `HUFFENC_AC1_32` writer
pub type W = crate::W<HUFFENC_AC1_32rs>;
///Field `HCODE64` reader - Huffman code 64
pub type HCODE64_R = crate::FieldReader;
///Field `HCODE64` writer - Huffman code 64
pub type HCODE64_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN64` reader - Huffman length 64
pub type HLEN64_R = crate::FieldReader;
///Field `HLEN64` writer - Huffman length 64
pub type HLEN64_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE65` reader - Huffman code 65
pub type HCODE65_R = crate::FieldReader;
///Field `HCODE65` writer - Huffman code 65
pub type HCODE65_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN65` reader - Huffman length 65
pub type HLEN65_R = crate::FieldReader;
///Field `HLEN65` writer - Huffman length 65
pub type HLEN65_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 64
    #[inline(always)]
    pub fn hcode64(&self) -> HCODE64_R {
        HCODE64_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 64
    #[inline(always)]
    pub fn hlen64(&self) -> HLEN64_R {
        HLEN64_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 65
    #[inline(always)]
    pub fn hcode65(&self) -> HCODE65_R {
        HCODE65_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 65
    #[inline(always)]
    pub fn hlen65(&self) -> HLEN65_R {
        HLEN65_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_32")
            .field("hcode64", &self.hcode64())
            .field("hlen64", &self.hlen64())
            .field("hcode65", &self.hcode65())
            .field("hlen65", &self.hlen65())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 64
    #[inline(always)]
    pub fn hcode64(&mut self) -> HCODE64_W<'_, HUFFENC_AC1_32rs> {
        HCODE64_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 64
    #[inline(always)]
    pub fn hlen64(&mut self) -> HLEN64_W<'_, HUFFENC_AC1_32rs> {
        HLEN64_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 65
    #[inline(always)]
    pub fn hcode65(&mut self) -> HCODE65_W<'_, HUFFENC_AC1_32rs> {
        HCODE65_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 65
    #[inline(always)]
    pub fn hlen65(&mut self) -> HLEN65_W<'_, HUFFENC_AC1_32rs> {
        HLEN65_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFENC_AC1_32)*/
pub struct HUFFENC_AC1_32rs;
impl crate::RegisterSpec for HUFFENC_AC1_32rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_32::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_32rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_32::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_32rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_32 to value 0
impl crate::Resettable for HUFFENC_AC1_32rs {}
