///Register `HUFFENC_AC1_21` reader
pub type R = crate::R<HUFFENC_AC1_21rs>;
///Register `HUFFENC_AC1_21` writer
pub type W = crate::W<HUFFENC_AC1_21rs>;
///Field `HCODE42` reader - Huffman code 42
pub type HCODE42_R = crate::FieldReader;
///Field `HCODE42` writer - Huffman code 42
pub type HCODE42_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN42` reader - Huffman length 42
pub type HLEN42_R = crate::FieldReader;
///Field `HLEN42` writer - Huffman length 42
pub type HLEN42_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE43` reader - Huffman code 43
pub type HCODE43_R = crate::FieldReader;
///Field `HCODE43` writer - Huffman code 43
pub type HCODE43_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN43` reader - Huffman length 43
pub type HLEN43_R = crate::FieldReader;
///Field `HLEN43` writer - Huffman length 43
pub type HLEN43_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 42
    #[inline(always)]
    pub fn hcode42(&self) -> HCODE42_R {
        HCODE42_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 42
    #[inline(always)]
    pub fn hlen42(&self) -> HLEN42_R {
        HLEN42_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 43
    #[inline(always)]
    pub fn hcode43(&self) -> HCODE43_R {
        HCODE43_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 43
    #[inline(always)]
    pub fn hlen43(&self) -> HLEN43_R {
        HLEN43_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_21")
            .field("hcode42", &self.hcode42())
            .field("hlen42", &self.hlen42())
            .field("hcode43", &self.hcode43())
            .field("hlen43", &self.hlen43())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 42
    #[inline(always)]
    pub fn hcode42(&mut self) -> HCODE42_W<'_, HUFFENC_AC1_21rs> {
        HCODE42_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 42
    #[inline(always)]
    pub fn hlen42(&mut self) -> HLEN42_W<'_, HUFFENC_AC1_21rs> {
        HLEN42_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 43
    #[inline(always)]
    pub fn hcode43(&mut self) -> HCODE43_W<'_, HUFFENC_AC1_21rs> {
        HCODE43_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 43
    #[inline(always)]
    pub fn hlen43(&mut self) -> HLEN43_W<'_, HUFFENC_AC1_21rs> {
        HLEN43_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFENC_AC1_21)*/
pub struct HUFFENC_AC1_21rs;
impl crate::RegisterSpec for HUFFENC_AC1_21rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_21::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_21rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_21::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_21rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_21 to value 0
impl crate::Resettable for HUFFENC_AC1_21rs {}
