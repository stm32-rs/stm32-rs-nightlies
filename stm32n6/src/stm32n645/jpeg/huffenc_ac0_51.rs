///Register `HUFFENC_AC0_51` reader
pub type R = crate::R<HUFFENC_AC0_51rs>;
///Register `HUFFENC_AC0_51` writer
pub type W = crate::W<HUFFENC_AC0_51rs>;
///Field `HCODE102` reader - Huffman code 102
pub type HCODE102_R = crate::FieldReader;
///Field `HCODE102` writer - Huffman code 102
pub type HCODE102_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN102` reader - Huffman length 102
pub type HLEN102_R = crate::FieldReader;
///Field `HLEN102` writer - Huffman length 102
pub type HLEN102_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE103` reader - Huffman code 103
pub type HCODE103_R = crate::FieldReader;
///Field `HCODE103` writer - Huffman code 103
pub type HCODE103_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN103` reader - Huffman length 103
pub type HLEN103_R = crate::FieldReader;
///Field `HLEN103` writer - Huffman length 103
pub type HLEN103_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 102
    #[inline(always)]
    pub fn hcode102(&self) -> HCODE102_R {
        HCODE102_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 102
    #[inline(always)]
    pub fn hlen102(&self) -> HLEN102_R {
        HLEN102_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 103
    #[inline(always)]
    pub fn hcode103(&self) -> HCODE103_R {
        HCODE103_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 103
    #[inline(always)]
    pub fn hlen103(&self) -> HLEN103_R {
        HLEN103_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_51")
            .field("hcode102", &self.hcode102())
            .field("hlen102", &self.hlen102())
            .field("hcode103", &self.hcode103())
            .field("hlen103", &self.hlen103())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 102
    #[inline(always)]
    pub fn hcode102(&mut self) -> HCODE102_W<'_, HUFFENC_AC0_51rs> {
        HCODE102_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 102
    #[inline(always)]
    pub fn hlen102(&mut self) -> HLEN102_W<'_, HUFFENC_AC0_51rs> {
        HLEN102_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 103
    #[inline(always)]
    pub fn hcode103(&mut self) -> HCODE103_W<'_, HUFFENC_AC0_51rs> {
        HCODE103_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 103
    #[inline(always)]
    pub fn hlen103(&mut self) -> HLEN103_W<'_, HUFFENC_AC0_51rs> {
        HLEN103_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_51::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_51::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFENC_AC0_51)*/
pub struct HUFFENC_AC0_51rs;
impl crate::RegisterSpec for HUFFENC_AC0_51rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_51::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_51rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_51::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_51rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_51 to value 0
impl crate::Resettable for HUFFENC_AC0_51rs {}
