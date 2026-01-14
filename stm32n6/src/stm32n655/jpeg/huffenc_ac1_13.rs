///Register `HUFFENC_AC1_13` reader
pub type R = crate::R<HUFFENC_AC1_13rs>;
///Register `HUFFENC_AC1_13` writer
pub type W = crate::W<HUFFENC_AC1_13rs>;
///Field `HCODE26` reader - Huffman code 26
pub type HCODE26_R = crate::FieldReader;
///Field `HCODE26` writer - Huffman code 26
pub type HCODE26_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN26` reader - Huffman length 26
pub type HLEN26_R = crate::FieldReader;
///Field `HLEN26` writer - Huffman length 26
pub type HLEN26_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE27` reader - Huffman code 27
pub type HCODE27_R = crate::FieldReader;
///Field `HCODE27` writer - Huffman code 27
pub type HCODE27_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN27` reader - Huffman length 27
pub type HLEN27_R = crate::FieldReader;
///Field `HLEN27` writer - Huffman length 27
pub type HLEN27_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 26
    #[inline(always)]
    pub fn hcode26(&self) -> HCODE26_R {
        HCODE26_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 26
    #[inline(always)]
    pub fn hlen26(&self) -> HLEN26_R {
        HLEN26_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 27
    #[inline(always)]
    pub fn hcode27(&self) -> HCODE27_R {
        HCODE27_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 27
    #[inline(always)]
    pub fn hlen27(&self) -> HLEN27_R {
        HLEN27_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_13")
            .field("hcode26", &self.hcode26())
            .field("hlen26", &self.hlen26())
            .field("hcode27", &self.hcode27())
            .field("hlen27", &self.hlen27())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 26
    #[inline(always)]
    pub fn hcode26(&mut self) -> HCODE26_W<'_, HUFFENC_AC1_13rs> {
        HCODE26_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 26
    #[inline(always)]
    pub fn hlen26(&mut self) -> HLEN26_W<'_, HUFFENC_AC1_13rs> {
        HLEN26_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 27
    #[inline(always)]
    pub fn hcode27(&mut self) -> HCODE27_W<'_, HUFFENC_AC1_13rs> {
        HCODE27_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 27
    #[inline(always)]
    pub fn hlen27(&mut self) -> HLEN27_W<'_, HUFFENC_AC1_13rs> {
        HLEN27_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC1_13)*/
pub struct HUFFENC_AC1_13rs;
impl crate::RegisterSpec for HUFFENC_AC1_13rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_13::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_13rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_13::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_13rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_13 to value 0
impl crate::Resettable for HUFFENC_AC1_13rs {}
