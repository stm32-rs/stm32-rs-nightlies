///Register `HUFFENC_DC1_0` reader
pub type R = crate::R<HUFFENC_DC1_0rs>;
///Register `HUFFENC_DC1_0` writer
pub type W = crate::W<HUFFENC_DC1_0rs>;
///Field `HCODE0` reader - Huffman code 0
pub type HCODE0_R = crate::FieldReader;
///Field `HCODE0` writer - Huffman code 0
pub type HCODE0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN0` reader - Huffman length 0
pub type HLEN0_R = crate::FieldReader;
///Field `HLEN0` writer - Huffman length 0
pub type HLEN0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE1` reader - Huffman code 1
pub type HCODE1_R = crate::FieldReader;
///Field `HCODE1` writer - Huffman code 1
pub type HCODE1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN1` reader - Huffman length 1
pub type HLEN1_R = crate::FieldReader;
///Field `HLEN1` writer - Huffman length 1
pub type HLEN1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 0
    #[inline(always)]
    pub fn hcode0(&self) -> HCODE0_R {
        HCODE0_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 0
    #[inline(always)]
    pub fn hlen0(&self) -> HLEN0_R {
        HLEN0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 1
    #[inline(always)]
    pub fn hcode1(&self) -> HCODE1_R {
        HCODE1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 1
    #[inline(always)]
    pub fn hlen1(&self) -> HLEN1_R {
        HLEN1_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_DC1_0")
            .field("hcode0", &self.hcode0())
            .field("hlen0", &self.hlen0())
            .field("hcode1", &self.hcode1())
            .field("hlen1", &self.hlen1())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 0
    #[inline(always)]
    pub fn hcode0(&mut self) -> HCODE0_W<HUFFENC_DC1_0rs> {
        HCODE0_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 0
    #[inline(always)]
    pub fn hlen0(&mut self) -> HLEN0_W<HUFFENC_DC1_0rs> {
        HLEN0_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 1
    #[inline(always)]
    pub fn hcode1(&mut self) -> HCODE1_W<HUFFENC_DC1_0rs> {
        HCODE1_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 1
    #[inline(always)]
    pub fn hlen1(&mut self) -> HLEN1_W<HUFFENC_DC1_0rs> {
        HLEN1_W::new(self, 24)
    }
}
/**JPEG Huffman encoder DC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc1_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc1_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_DC1_0)*/
pub struct HUFFENC_DC1_0rs;
impl crate::RegisterSpec for HUFFENC_DC1_0rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_dc1_0::R`](R) reader structure
impl crate::Readable for HUFFENC_DC1_0rs {}
///`write(|w| ..)` method takes [`huffenc_dc1_0::W`](W) writer structure
impl crate::Writable for HUFFENC_DC1_0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_DC1_0 to value 0
impl crate::Resettable for HUFFENC_DC1_0rs {}
