///Register `HUFFENC_AC1_35` reader
pub type R = crate::R<HUFFENC_AC1_35rs>;
///Register `HUFFENC_AC1_35` writer
pub type W = crate::W<HUFFENC_AC1_35rs>;
///Field `HCODE70` reader - Huffman code 70
pub type HCODE70_R = crate::FieldReader;
///Field `HCODE70` writer - Huffman code 70
pub type HCODE70_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN70` reader - Huffman length 70
pub type HLEN70_R = crate::FieldReader;
///Field `HLEN70` writer - Huffman length 70
pub type HLEN70_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE71` reader - Huffman code 71
pub type HCODE71_R = crate::FieldReader;
///Field `HCODE71` writer - Huffman code 71
pub type HCODE71_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN71` reader - Huffman length 71
pub type HLEN71_R = crate::FieldReader;
///Field `HLEN71` writer - Huffman length 71
pub type HLEN71_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 70
    #[inline(always)]
    pub fn hcode70(&self) -> HCODE70_R {
        HCODE70_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 70
    #[inline(always)]
    pub fn hlen70(&self) -> HLEN70_R {
        HLEN70_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 71
    #[inline(always)]
    pub fn hcode71(&self) -> HCODE71_R {
        HCODE71_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 71
    #[inline(always)]
    pub fn hlen71(&self) -> HLEN71_R {
        HLEN71_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_35")
            .field("hcode70", &self.hcode70())
            .field("hlen70", &self.hlen70())
            .field("hcode71", &self.hcode71())
            .field("hlen71", &self.hlen71())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 70
    #[inline(always)]
    pub fn hcode70(&mut self) -> HCODE70_W<'_, HUFFENC_AC1_35rs> {
        HCODE70_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 70
    #[inline(always)]
    pub fn hlen70(&mut self) -> HLEN70_W<'_, HUFFENC_AC1_35rs> {
        HLEN70_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 71
    #[inline(always)]
    pub fn hcode71(&mut self) -> HCODE71_W<'_, HUFFENC_AC1_35rs> {
        HCODE71_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 71
    #[inline(always)]
    pub fn hlen71(&mut self) -> HLEN71_W<'_, HUFFENC_AC1_35rs> {
        HLEN71_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_35::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_35::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFENC_AC1_35)*/
pub struct HUFFENC_AC1_35rs;
impl crate::RegisterSpec for HUFFENC_AC1_35rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_35::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_35rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_35::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_35rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_35 to value 0
impl crate::Resettable for HUFFENC_AC1_35rs {}
