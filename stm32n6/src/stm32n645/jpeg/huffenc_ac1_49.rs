///Register `HUFFENC_AC1_49` reader
pub type R = crate::R<HUFFENC_AC1_49rs>;
///Register `HUFFENC_AC1_49` writer
pub type W = crate::W<HUFFENC_AC1_49rs>;
///Field `HCODE98` reader - Huffman code 98
pub type HCODE98_R = crate::FieldReader;
///Field `HCODE98` writer - Huffman code 98
pub type HCODE98_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN98` reader - Huffman length 98
pub type HLEN98_R = crate::FieldReader;
///Field `HLEN98` writer - Huffman length 98
pub type HLEN98_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE99` reader - Huffman code 99
pub type HCODE99_R = crate::FieldReader;
///Field `HCODE99` writer - Huffman code 99
pub type HCODE99_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN99` reader - Huffman length 99
pub type HLEN99_R = crate::FieldReader;
///Field `HLEN99` writer - Huffman length 99
pub type HLEN99_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 98
    #[inline(always)]
    pub fn hcode98(&self) -> HCODE98_R {
        HCODE98_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 98
    #[inline(always)]
    pub fn hlen98(&self) -> HLEN98_R {
        HLEN98_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 99
    #[inline(always)]
    pub fn hcode99(&self) -> HCODE99_R {
        HCODE99_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 99
    #[inline(always)]
    pub fn hlen99(&self) -> HLEN99_R {
        HLEN99_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_49")
            .field("hcode98", &self.hcode98())
            .field("hlen98", &self.hlen98())
            .field("hcode99", &self.hcode99())
            .field("hlen99", &self.hlen99())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 98
    #[inline(always)]
    pub fn hcode98(&mut self) -> HCODE98_W<'_, HUFFENC_AC1_49rs> {
        HCODE98_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 98
    #[inline(always)]
    pub fn hlen98(&mut self) -> HLEN98_W<'_, HUFFENC_AC1_49rs> {
        HLEN98_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 99
    #[inline(always)]
    pub fn hcode99(&mut self) -> HCODE99_W<'_, HUFFENC_AC1_49rs> {
        HCODE99_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 99
    #[inline(always)]
    pub fn hlen99(&mut self) -> HLEN99_W<'_, HUFFENC_AC1_49rs> {
        HLEN99_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_49::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_49::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFENC_AC1_49)*/
pub struct HUFFENC_AC1_49rs;
impl crate::RegisterSpec for HUFFENC_AC1_49rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_49::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_49rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_49::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_49rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_49 to value 0
impl crate::Resettable for HUFFENC_AC1_49rs {}
