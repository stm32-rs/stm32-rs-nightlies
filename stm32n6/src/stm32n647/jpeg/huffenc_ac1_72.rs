///Register `HUFFENC_AC1_72` reader
pub type R = crate::R<HUFFENC_AC1_72rs>;
///Register `HUFFENC_AC1_72` writer
pub type W = crate::W<HUFFENC_AC1_72rs>;
///Field `HCODE144` reader - Huffman code 144
pub type HCODE144_R = crate::FieldReader;
///Field `HCODE144` writer - Huffman code 144
pub type HCODE144_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN144` reader - Huffman length 144
pub type HLEN144_R = crate::FieldReader;
///Field `HLEN144` writer - Huffman length 144
pub type HLEN144_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE145` reader - Huffman code 145
pub type HCODE145_R = crate::FieldReader;
///Field `HCODE145` writer - Huffman code 145
pub type HCODE145_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN145` reader - Huffman length 145
pub type HLEN145_R = crate::FieldReader;
///Field `HLEN145` writer - Huffman length 145
pub type HLEN145_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 144
    #[inline(always)]
    pub fn hcode144(&self) -> HCODE144_R {
        HCODE144_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 144
    #[inline(always)]
    pub fn hlen144(&self) -> HLEN144_R {
        HLEN144_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 145
    #[inline(always)]
    pub fn hcode145(&self) -> HCODE145_R {
        HCODE145_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 145
    #[inline(always)]
    pub fn hlen145(&self) -> HLEN145_R {
        HLEN145_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_72")
            .field("hcode144", &self.hcode144())
            .field("hlen144", &self.hlen144())
            .field("hcode145", &self.hcode145())
            .field("hlen145", &self.hlen145())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 144
    #[inline(always)]
    pub fn hcode144(&mut self) -> HCODE144_W<'_, HUFFENC_AC1_72rs> {
        HCODE144_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 144
    #[inline(always)]
    pub fn hlen144(&mut self) -> HLEN144_W<'_, HUFFENC_AC1_72rs> {
        HLEN144_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 145
    #[inline(always)]
    pub fn hcode145(&mut self) -> HCODE145_W<'_, HUFFENC_AC1_72rs> {
        HCODE145_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 145
    #[inline(always)]
    pub fn hlen145(&mut self) -> HLEN145_W<'_, HUFFENC_AC1_72rs> {
        HLEN145_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_72::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_72::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFENC_AC1_72)*/
pub struct HUFFENC_AC1_72rs;
impl crate::RegisterSpec for HUFFENC_AC1_72rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_72::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_72rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_72::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_72rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_72 to value 0
impl crate::Resettable for HUFFENC_AC1_72rs {}
