///Register `HUFFENC_AC1_39` reader
pub type R = crate::R<HUFFENC_AC1_39rs>;
///Register `HUFFENC_AC1_39` writer
pub type W = crate::W<HUFFENC_AC1_39rs>;
///Field `HCODE78` reader - Huffman code 78
pub type HCODE78_R = crate::FieldReader;
///Field `HCODE78` writer - Huffman code 78
pub type HCODE78_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN78` reader - Huffman length 78
pub type HLEN78_R = crate::FieldReader;
///Field `HLEN78` writer - Huffman length 78
pub type HLEN78_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE79` reader - Huffman code 79
pub type HCODE79_R = crate::FieldReader;
///Field `HCODE79` writer - Huffman code 79
pub type HCODE79_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN79` reader - Huffman length 79
pub type HLEN79_R = crate::FieldReader;
///Field `HLEN79` writer - Huffman length 79
pub type HLEN79_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 78
    #[inline(always)]
    pub fn hcode78(&self) -> HCODE78_R {
        HCODE78_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 78
    #[inline(always)]
    pub fn hlen78(&self) -> HLEN78_R {
        HLEN78_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 79
    #[inline(always)]
    pub fn hcode79(&self) -> HCODE79_R {
        HCODE79_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 79
    #[inline(always)]
    pub fn hlen79(&self) -> HLEN79_R {
        HLEN79_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_39")
            .field("hcode78", &self.hcode78())
            .field("hlen78", &self.hlen78())
            .field("hcode79", &self.hcode79())
            .field("hlen79", &self.hlen79())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 78
    #[inline(always)]
    pub fn hcode78(&mut self) -> HCODE78_W<'_, HUFFENC_AC1_39rs> {
        HCODE78_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 78
    #[inline(always)]
    pub fn hlen78(&mut self) -> HLEN78_W<'_, HUFFENC_AC1_39rs> {
        HLEN78_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 79
    #[inline(always)]
    pub fn hcode79(&mut self) -> HCODE79_W<'_, HUFFENC_AC1_39rs> {
        HCODE79_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 79
    #[inline(always)]
    pub fn hlen79(&mut self) -> HLEN79_W<'_, HUFFENC_AC1_39rs> {
        HLEN79_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_39::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_39::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_39)*/
pub struct HUFFENC_AC1_39rs;
impl crate::RegisterSpec for HUFFENC_AC1_39rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_39::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_39rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_39::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_39rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_39 to value 0
impl crate::Resettable for HUFFENC_AC1_39rs {}
