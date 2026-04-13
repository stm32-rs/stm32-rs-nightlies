///Register `HUFFENC_AC0_31` reader
pub type R = crate::R<HUFFENC_AC0_31rs>;
///Register `HUFFENC_AC0_31` writer
pub type W = crate::W<HUFFENC_AC0_31rs>;
///Field `HCODE62` reader - Huffman code 62
pub type HCODE62_R = crate::FieldReader;
///Field `HCODE62` writer - Huffman code 62
pub type HCODE62_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN62` reader - Huffman length 62
pub type HLEN62_R = crate::FieldReader;
///Field `HLEN62` writer - Huffman length 62
pub type HLEN62_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE63` reader - Huffman code 63
pub type HCODE63_R = crate::FieldReader;
///Field `HCODE63` writer - Huffman code 63
pub type HCODE63_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN63` reader - Huffman length 63
pub type HLEN63_R = crate::FieldReader;
///Field `HLEN63` writer - Huffman length 63
pub type HLEN63_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 62
    #[inline(always)]
    pub fn hcode62(&self) -> HCODE62_R {
        HCODE62_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 62
    #[inline(always)]
    pub fn hlen62(&self) -> HLEN62_R {
        HLEN62_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 63
    #[inline(always)]
    pub fn hcode63(&self) -> HCODE63_R {
        HCODE63_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 63
    #[inline(always)]
    pub fn hlen63(&self) -> HLEN63_R {
        HLEN63_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_31")
            .field("hcode62", &self.hcode62())
            .field("hlen62", &self.hlen62())
            .field("hcode63", &self.hcode63())
            .field("hlen63", &self.hlen63())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 62
    #[inline(always)]
    pub fn hcode62(&mut self) -> HCODE62_W<'_, HUFFENC_AC0_31rs> {
        HCODE62_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 62
    #[inline(always)]
    pub fn hlen62(&mut self) -> HLEN62_W<'_, HUFFENC_AC0_31rs> {
        HLEN62_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 63
    #[inline(always)]
    pub fn hcode63(&mut self) -> HCODE63_W<'_, HUFFENC_AC0_31rs> {
        HCODE63_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 63
    #[inline(always)]
    pub fn hlen63(&mut self) -> HLEN63_W<'_, HUFFENC_AC0_31rs> {
        HLEN63_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFENC_AC0_31)*/
pub struct HUFFENC_AC0_31rs;
impl crate::RegisterSpec for HUFFENC_AC0_31rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_31::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_31rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_31::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_31rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_31 to value 0
impl crate::Resettable for HUFFENC_AC0_31rs {}
