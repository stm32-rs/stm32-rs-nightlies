///Register `HUFFENC_DC0_1` reader
pub type R = crate::R<HUFFENC_DC0_1rs>;
///Register `HUFFENC_DC0_1` writer
pub type W = crate::W<HUFFENC_DC0_1rs>;
///Field `HCODE2` reader - Huffman code 2
pub type HCODE2_R = crate::FieldReader;
///Field `HCODE2` writer - Huffman code 2
pub type HCODE2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN2` reader - Huffman length 2
pub type HLEN2_R = crate::FieldReader;
///Field `HLEN2` writer - Huffman length 2
pub type HLEN2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE3` reader - Huffman code 3
pub type HCODE3_R = crate::FieldReader;
///Field `HCODE3` writer - Huffman code 3
pub type HCODE3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN3` reader - Huffman length 3
pub type HLEN3_R = crate::FieldReader;
///Field `HLEN3` writer - Huffman length 3
pub type HLEN3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 2
    #[inline(always)]
    pub fn hcode2(&self) -> HCODE2_R {
        HCODE2_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 2
    #[inline(always)]
    pub fn hlen2(&self) -> HLEN2_R {
        HLEN2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 3
    #[inline(always)]
    pub fn hcode3(&self) -> HCODE3_R {
        HCODE3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 3
    #[inline(always)]
    pub fn hlen3(&self) -> HLEN3_R {
        HLEN3_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_DC0_1")
            .field("hcode2", &self.hcode2())
            .field("hlen2", &self.hlen2())
            .field("hcode3", &self.hcode3())
            .field("hlen3", &self.hlen3())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 2
    #[inline(always)]
    pub fn hcode2(&mut self) -> HCODE2_W<'_, HUFFENC_DC0_1rs> {
        HCODE2_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 2
    #[inline(always)]
    pub fn hlen2(&mut self) -> HLEN2_W<'_, HUFFENC_DC0_1rs> {
        HLEN2_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 3
    #[inline(always)]
    pub fn hcode3(&mut self) -> HCODE3_W<'_, HUFFENC_DC0_1rs> {
        HCODE3_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 3
    #[inline(always)]
    pub fn hlen3(&mut self) -> HLEN3_W<'_, HUFFENC_DC0_1rs> {
        HLEN3_W::new(self, 24)
    }
}
/**JPEG Huffman encoder DC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc0_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc0_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_DC0_1)*/
pub struct HUFFENC_DC0_1rs;
impl crate::RegisterSpec for HUFFENC_DC0_1rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_dc0_1::R`](R) reader structure
impl crate::Readable for HUFFENC_DC0_1rs {}
///`write(|w| ..)` method takes [`huffenc_dc0_1::W`](W) writer structure
impl crate::Writable for HUFFENC_DC0_1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_DC0_1 to value 0
impl crate::Resettable for HUFFENC_DC0_1rs {}
