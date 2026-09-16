///Register `HUFFENC_AC0_3` reader
pub type R = crate::R<HUFFENC_AC0_3rs>;
///Register `HUFFENC_AC0_3` writer
pub type W = crate::W<HUFFENC_AC0_3rs>;
///Field `HCODE6` reader - Huffman code 6
pub type HCODE6_R = crate::FieldReader;
///Field `HCODE6` writer - Huffman code 6
pub type HCODE6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN6` reader - Huffman length 6
pub type HLEN6_R = crate::FieldReader;
///Field `HLEN6` writer - Huffman length 6
pub type HLEN6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE7` reader - Huffman code 7
pub type HCODE7_R = crate::FieldReader;
///Field `HCODE7` writer - Huffman code 7
pub type HCODE7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN7` reader - Huffman length 7
pub type HLEN7_R = crate::FieldReader;
///Field `HLEN7` writer - Huffman length 7
pub type HLEN7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 6
    #[inline(always)]
    pub fn hcode6(&self) -> HCODE6_R {
        HCODE6_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 6
    #[inline(always)]
    pub fn hlen6(&self) -> HLEN6_R {
        HLEN6_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 7
    #[inline(always)]
    pub fn hcode7(&self) -> HCODE7_R {
        HCODE7_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 7
    #[inline(always)]
    pub fn hlen7(&self) -> HLEN7_R {
        HLEN7_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_3")
            .field("hcode6", &self.hcode6())
            .field("hlen6", &self.hlen6())
            .field("hcode7", &self.hcode7())
            .field("hlen7", &self.hlen7())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 6
    #[inline(always)]
    pub fn hcode6(&mut self) -> HCODE6_W<'_, HUFFENC_AC0_3rs> {
        HCODE6_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 6
    #[inline(always)]
    pub fn hlen6(&mut self) -> HLEN6_W<'_, HUFFENC_AC0_3rs> {
        HLEN6_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 7
    #[inline(always)]
    pub fn hcode7(&mut self) -> HCODE7_W<'_, HUFFENC_AC0_3rs> {
        HCODE7_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 7
    #[inline(always)]
    pub fn hlen7(&mut self) -> HLEN7_W<'_, HUFFENC_AC0_3rs> {
        HLEN7_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFENC_AC0_3)*/
pub struct HUFFENC_AC0_3rs;
impl crate::RegisterSpec for HUFFENC_AC0_3rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_3::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_3rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_3::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_3 to value 0
impl crate::Resettable for HUFFENC_AC0_3rs {}
