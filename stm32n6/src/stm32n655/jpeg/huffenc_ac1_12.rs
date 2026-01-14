///Register `HUFFENC_AC1_12` reader
pub type R = crate::R<HUFFENC_AC1_12rs>;
///Register `HUFFENC_AC1_12` writer
pub type W = crate::W<HUFFENC_AC1_12rs>;
///Field `HCODE24` reader - Huffman code 24
pub type HCODE24_R = crate::FieldReader;
///Field `HCODE24` writer - Huffman code 24
pub type HCODE24_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN24` reader - Huffman length 24
pub type HLEN24_R = crate::FieldReader;
///Field `HLEN24` writer - Huffman length 24
pub type HLEN24_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE25` reader - Huffman code 25
pub type HCODE25_R = crate::FieldReader;
///Field `HCODE25` writer - Huffman code 25
pub type HCODE25_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN25` reader - Huffman length 25
pub type HLEN25_R = crate::FieldReader;
///Field `HLEN25` writer - Huffman length 25
pub type HLEN25_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 24
    #[inline(always)]
    pub fn hcode24(&self) -> HCODE24_R {
        HCODE24_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 24
    #[inline(always)]
    pub fn hlen24(&self) -> HLEN24_R {
        HLEN24_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 25
    #[inline(always)]
    pub fn hcode25(&self) -> HCODE25_R {
        HCODE25_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 25
    #[inline(always)]
    pub fn hlen25(&self) -> HLEN25_R {
        HLEN25_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_12")
            .field("hcode24", &self.hcode24())
            .field("hlen24", &self.hlen24())
            .field("hcode25", &self.hcode25())
            .field("hlen25", &self.hlen25())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 24
    #[inline(always)]
    pub fn hcode24(&mut self) -> HCODE24_W<'_, HUFFENC_AC1_12rs> {
        HCODE24_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 24
    #[inline(always)]
    pub fn hlen24(&mut self) -> HLEN24_W<'_, HUFFENC_AC1_12rs> {
        HLEN24_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 25
    #[inline(always)]
    pub fn hcode25(&mut self) -> HCODE25_W<'_, HUFFENC_AC1_12rs> {
        HCODE25_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 25
    #[inline(always)]
    pub fn hlen25(&mut self) -> HLEN25_W<'_, HUFFENC_AC1_12rs> {
        HLEN25_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_12)*/
pub struct HUFFENC_AC1_12rs;
impl crate::RegisterSpec for HUFFENC_AC1_12rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_12::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_12rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_12::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_12rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_12 to value 0
impl crate::Resettable for HUFFENC_AC1_12rs {}
