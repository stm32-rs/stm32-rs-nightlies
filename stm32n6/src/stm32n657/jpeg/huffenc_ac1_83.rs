///Register `HUFFENC_AC1_83` reader
pub type R = crate::R<HUFFENC_AC1_83rs>;
///Register `HUFFENC_AC1_83` writer
pub type W = crate::W<HUFFENC_AC1_83rs>;
///Field `HCODE166` reader - Huffman code 166
pub type HCODE166_R = crate::FieldReader;
///Field `HCODE166` writer - Huffman code 166
pub type HCODE166_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN166` reader - Huffman length 166
pub type HLEN166_R = crate::FieldReader;
///Field `HLEN166` writer - Huffman length 166
pub type HLEN166_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE167` reader - Huffman code 167
pub type HCODE167_R = crate::FieldReader;
///Field `HCODE167` writer - Huffman code 167
pub type HCODE167_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN167` reader - Huffman length 167
pub type HLEN167_R = crate::FieldReader;
///Field `HLEN167` writer - Huffman length 167
pub type HLEN167_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 166
    #[inline(always)]
    pub fn hcode166(&self) -> HCODE166_R {
        HCODE166_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 166
    #[inline(always)]
    pub fn hlen166(&self) -> HLEN166_R {
        HLEN166_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 167
    #[inline(always)]
    pub fn hcode167(&self) -> HCODE167_R {
        HCODE167_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 167
    #[inline(always)]
    pub fn hlen167(&self) -> HLEN167_R {
        HLEN167_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_83")
            .field("hcode166", &self.hcode166())
            .field("hlen166", &self.hlen166())
            .field("hcode167", &self.hcode167())
            .field("hlen167", &self.hlen167())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 166
    #[inline(always)]
    pub fn hcode166(&mut self) -> HCODE166_W<'_, HUFFENC_AC1_83rs> {
        HCODE166_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 166
    #[inline(always)]
    pub fn hlen166(&mut self) -> HLEN166_W<'_, HUFFENC_AC1_83rs> {
        HLEN166_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 167
    #[inline(always)]
    pub fn hcode167(&mut self) -> HCODE167_W<'_, HUFFENC_AC1_83rs> {
        HCODE167_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 167
    #[inline(always)]
    pub fn hlen167(&mut self) -> HLEN167_W<'_, HUFFENC_AC1_83rs> {
        HLEN167_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_83::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_83::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFENC_AC1_83)*/
pub struct HUFFENC_AC1_83rs;
impl crate::RegisterSpec for HUFFENC_AC1_83rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_83::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_83rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_83::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_83rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_83 to value 0
impl crate::Resettable for HUFFENC_AC1_83rs {}
