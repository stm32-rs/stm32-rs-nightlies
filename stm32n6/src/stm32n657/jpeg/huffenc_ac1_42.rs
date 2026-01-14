///Register `HUFFENC_AC1_42` reader
pub type R = crate::R<HUFFENC_AC1_42rs>;
///Register `HUFFENC_AC1_42` writer
pub type W = crate::W<HUFFENC_AC1_42rs>;
///Field `HCODE84` reader - Huffman code 84
pub type HCODE84_R = crate::FieldReader;
///Field `HCODE84` writer - Huffman code 84
pub type HCODE84_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN84` reader - Huffman length 84
pub type HLEN84_R = crate::FieldReader;
///Field `HLEN84` writer - Huffman length 84
pub type HLEN84_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE85` reader - Huffman code 85
pub type HCODE85_R = crate::FieldReader;
///Field `HCODE85` writer - Huffman code 85
pub type HCODE85_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN85` reader - Huffman length 85
pub type HLEN85_R = crate::FieldReader;
///Field `HLEN85` writer - Huffman length 85
pub type HLEN85_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 84
    #[inline(always)]
    pub fn hcode84(&self) -> HCODE84_R {
        HCODE84_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 84
    #[inline(always)]
    pub fn hlen84(&self) -> HLEN84_R {
        HLEN84_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 85
    #[inline(always)]
    pub fn hcode85(&self) -> HCODE85_R {
        HCODE85_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 85
    #[inline(always)]
    pub fn hlen85(&self) -> HLEN85_R {
        HLEN85_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_42")
            .field("hcode84", &self.hcode84())
            .field("hlen84", &self.hlen84())
            .field("hcode85", &self.hcode85())
            .field("hlen85", &self.hlen85())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 84
    #[inline(always)]
    pub fn hcode84(&mut self) -> HCODE84_W<'_, HUFFENC_AC1_42rs> {
        HCODE84_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 84
    #[inline(always)]
    pub fn hlen84(&mut self) -> HLEN84_W<'_, HUFFENC_AC1_42rs> {
        HLEN84_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 85
    #[inline(always)]
    pub fn hcode85(&mut self) -> HCODE85_W<'_, HUFFENC_AC1_42rs> {
        HCODE85_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 85
    #[inline(always)]
    pub fn hlen85(&mut self) -> HLEN85_W<'_, HUFFENC_AC1_42rs> {
        HLEN85_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_42::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_42::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFENC_AC1_42)*/
pub struct HUFFENC_AC1_42rs;
impl crate::RegisterSpec for HUFFENC_AC1_42rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_42::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_42rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_42::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_42rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_42 to value 0
impl crate::Resettable for HUFFENC_AC1_42rs {}
