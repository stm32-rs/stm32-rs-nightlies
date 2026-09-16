///Register `HUFFENC_AC1_45` reader
pub type R = crate::R<HUFFENC_AC1_45rs>;
///Register `HUFFENC_AC1_45` writer
pub type W = crate::W<HUFFENC_AC1_45rs>;
///Field `HCODE90` reader - Huffman code 90
pub type HCODE90_R = crate::FieldReader;
///Field `HCODE90` writer - Huffman code 90
pub type HCODE90_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN90` reader - Huffman length 90
pub type HLEN90_R = crate::FieldReader;
///Field `HLEN90` writer - Huffman length 90
pub type HLEN90_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE91` reader - Huffman code 91
pub type HCODE91_R = crate::FieldReader;
///Field `HCODE91` writer - Huffman code 91
pub type HCODE91_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN91` reader - Huffman length 91
pub type HLEN91_R = crate::FieldReader;
///Field `HLEN91` writer - Huffman length 91
pub type HLEN91_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 90
    #[inline(always)]
    pub fn hcode90(&self) -> HCODE90_R {
        HCODE90_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 90
    #[inline(always)]
    pub fn hlen90(&self) -> HLEN90_R {
        HLEN90_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 91
    #[inline(always)]
    pub fn hcode91(&self) -> HCODE91_R {
        HCODE91_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 91
    #[inline(always)]
    pub fn hlen91(&self) -> HLEN91_R {
        HLEN91_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_45")
            .field("hcode90", &self.hcode90())
            .field("hlen90", &self.hlen90())
            .field("hcode91", &self.hcode91())
            .field("hlen91", &self.hlen91())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 90
    #[inline(always)]
    pub fn hcode90(&mut self) -> HCODE90_W<'_, HUFFENC_AC1_45rs> {
        HCODE90_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 90
    #[inline(always)]
    pub fn hlen90(&mut self) -> HLEN90_W<'_, HUFFENC_AC1_45rs> {
        HLEN90_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 91
    #[inline(always)]
    pub fn hcode91(&mut self) -> HCODE91_W<'_, HUFFENC_AC1_45rs> {
        HCODE91_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 91
    #[inline(always)]
    pub fn hlen91(&mut self) -> HLEN91_W<'_, HUFFENC_AC1_45rs> {
        HLEN91_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_45::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_45::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFENC_AC1_45)*/
pub struct HUFFENC_AC1_45rs;
impl crate::RegisterSpec for HUFFENC_AC1_45rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_45::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_45rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_45::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_45rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_45 to value 0
impl crate::Resettable for HUFFENC_AC1_45rs {}
