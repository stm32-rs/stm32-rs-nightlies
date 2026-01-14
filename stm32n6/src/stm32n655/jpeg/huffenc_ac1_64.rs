///Register `HUFFENC_AC1_64` reader
pub type R = crate::R<HUFFENC_AC1_64rs>;
///Register `HUFFENC_AC1_64` writer
pub type W = crate::W<HUFFENC_AC1_64rs>;
///Field `HCODE128` reader - Huffman code 128
pub type HCODE128_R = crate::FieldReader;
///Field `HCODE128` writer - Huffman code 128
pub type HCODE128_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN128` reader - Huffman length 128
pub type HLEN128_R = crate::FieldReader;
///Field `HLEN128` writer - Huffman length 128
pub type HLEN128_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE129` reader - Huffman code 129
pub type HCODE129_R = crate::FieldReader;
///Field `HCODE129` writer - Huffman code 129
pub type HCODE129_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN129` reader - Huffman length 129
pub type HLEN129_R = crate::FieldReader;
///Field `HLEN129` writer - Huffman length 129
pub type HLEN129_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 128
    #[inline(always)]
    pub fn hcode128(&self) -> HCODE128_R {
        HCODE128_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 128
    #[inline(always)]
    pub fn hlen128(&self) -> HLEN128_R {
        HLEN128_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 129
    #[inline(always)]
    pub fn hcode129(&self) -> HCODE129_R {
        HCODE129_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 129
    #[inline(always)]
    pub fn hlen129(&self) -> HLEN129_R {
        HLEN129_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_64")
            .field("hcode128", &self.hcode128())
            .field("hlen128", &self.hlen128())
            .field("hcode129", &self.hcode129())
            .field("hlen129", &self.hlen129())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 128
    #[inline(always)]
    pub fn hcode128(&mut self) -> HCODE128_W<'_, HUFFENC_AC1_64rs> {
        HCODE128_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 128
    #[inline(always)]
    pub fn hlen128(&mut self) -> HLEN128_W<'_, HUFFENC_AC1_64rs> {
        HLEN128_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 129
    #[inline(always)]
    pub fn hcode129(&mut self) -> HCODE129_W<'_, HUFFENC_AC1_64rs> {
        HCODE129_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 129
    #[inline(always)]
    pub fn hlen129(&mut self) -> HLEN129_W<'_, HUFFENC_AC1_64rs> {
        HLEN129_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_64::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_64::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_64)*/
pub struct HUFFENC_AC1_64rs;
impl crate::RegisterSpec for HUFFENC_AC1_64rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_64::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_64rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_64::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_64rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_64 to value 0
impl crate::Resettable for HUFFENC_AC1_64rs {}
