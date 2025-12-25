///Register `HUFFENC_AC0_41` reader
pub type R = crate::R<HUFFENC_AC0_41rs>;
///Register `HUFFENC_AC0_41` writer
pub type W = crate::W<HUFFENC_AC0_41rs>;
///Field `HCODE82` reader - Huffman code 82
pub type HCODE82_R = crate::FieldReader;
///Field `HCODE82` writer - Huffman code 82
pub type HCODE82_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN82` reader - Huffman length 82
pub type HLEN82_R = crate::FieldReader;
///Field `HLEN82` writer - Huffman length 82
pub type HLEN82_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE83` reader - Huffman code 83
pub type HCODE83_R = crate::FieldReader;
///Field `HCODE83` writer - Huffman code 83
pub type HCODE83_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN83` reader - Huffman length 83
pub type HLEN83_R = crate::FieldReader;
///Field `HLEN83` writer - Huffman length 83
pub type HLEN83_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 82
    #[inline(always)]
    pub fn hcode82(&self) -> HCODE82_R {
        HCODE82_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 82
    #[inline(always)]
    pub fn hlen82(&self) -> HLEN82_R {
        HLEN82_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 83
    #[inline(always)]
    pub fn hcode83(&self) -> HCODE83_R {
        HCODE83_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 83
    #[inline(always)]
    pub fn hlen83(&self) -> HLEN83_R {
        HLEN83_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_41")
            .field("hcode82", &self.hcode82())
            .field("hlen82", &self.hlen82())
            .field("hcode83", &self.hcode83())
            .field("hlen83", &self.hlen83())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 82
    #[inline(always)]
    pub fn hcode82(&mut self) -> HCODE82_W<'_, HUFFENC_AC0_41rs> {
        HCODE82_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 82
    #[inline(always)]
    pub fn hlen82(&mut self) -> HLEN82_W<'_, HUFFENC_AC0_41rs> {
        HLEN82_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 83
    #[inline(always)]
    pub fn hcode83(&mut self) -> HCODE83_W<'_, HUFFENC_AC0_41rs> {
        HCODE83_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 83
    #[inline(always)]
    pub fn hlen83(&mut self) -> HLEN83_W<'_, HUFFENC_AC0_41rs> {
        HLEN83_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_41::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_41::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_41)*/
pub struct HUFFENC_AC0_41rs;
impl crate::RegisterSpec for HUFFENC_AC0_41rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_41::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_41rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_41::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_41rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_41 to value 0
impl crate::Resettable for HUFFENC_AC0_41rs {}
