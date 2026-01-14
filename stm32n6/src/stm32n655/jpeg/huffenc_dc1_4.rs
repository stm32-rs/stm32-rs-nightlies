///Register `HUFFENC_DC1_4` reader
pub type R = crate::R<HUFFENC_DC1_4rs>;
///Register `HUFFENC_DC1_4` writer
pub type W = crate::W<HUFFENC_DC1_4rs>;
///Field `HCODE8` reader - Huffman code 8
pub type HCODE8_R = crate::FieldReader;
///Field `HCODE8` writer - Huffman code 8
pub type HCODE8_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN8` reader - Huffman length 8
pub type HLEN8_R = crate::FieldReader;
///Field `HLEN8` writer - Huffman length 8
pub type HLEN8_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE9` reader - Huffman code 9
pub type HCODE9_R = crate::FieldReader;
///Field `HCODE9` writer - Huffman code 9
pub type HCODE9_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN9` reader - Huffman length 9
pub type HLEN9_R = crate::FieldReader;
///Field `HLEN9` writer - Huffman length 9
pub type HLEN9_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 8
    #[inline(always)]
    pub fn hcode8(&self) -> HCODE8_R {
        HCODE8_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 8
    #[inline(always)]
    pub fn hlen8(&self) -> HLEN8_R {
        HLEN8_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 9
    #[inline(always)]
    pub fn hcode9(&self) -> HCODE9_R {
        HCODE9_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 9
    #[inline(always)]
    pub fn hlen9(&self) -> HLEN9_R {
        HLEN9_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_DC1_4")
            .field("hcode8", &self.hcode8())
            .field("hlen8", &self.hlen8())
            .field("hcode9", &self.hcode9())
            .field("hlen9", &self.hlen9())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 8
    #[inline(always)]
    pub fn hcode8(&mut self) -> HCODE8_W<'_, HUFFENC_DC1_4rs> {
        HCODE8_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 8
    #[inline(always)]
    pub fn hlen8(&mut self) -> HLEN8_W<'_, HUFFENC_DC1_4rs> {
        HLEN8_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 9
    #[inline(always)]
    pub fn hcode9(&mut self) -> HCODE9_W<'_, HUFFENC_DC1_4rs> {
        HCODE9_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 9
    #[inline(always)]
    pub fn hlen9(&mut self) -> HLEN9_W<'_, HUFFENC_DC1_4rs> {
        HLEN9_W::new(self, 24)
    }
}
/**JPEG Huffman encoder DC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc1_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc1_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_DC1_4)*/
pub struct HUFFENC_DC1_4rs;
impl crate::RegisterSpec for HUFFENC_DC1_4rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_dc1_4::R`](R) reader structure
impl crate::Readable for HUFFENC_DC1_4rs {}
///`write(|w| ..)` method takes [`huffenc_dc1_4::W`](W) writer structure
impl crate::Writable for HUFFENC_DC1_4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_DC1_4 to value 0
impl crate::Resettable for HUFFENC_DC1_4rs {}
