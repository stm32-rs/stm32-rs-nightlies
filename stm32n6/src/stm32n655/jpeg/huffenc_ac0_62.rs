///Register `HUFFENC_AC0_62` reader
pub type R = crate::R<HUFFENC_AC0_62rs>;
///Register `HUFFENC_AC0_62` writer
pub type W = crate::W<HUFFENC_AC0_62rs>;
///Field `HCODE124` reader - Huffman code 124
pub type HCODE124_R = crate::FieldReader;
///Field `HCODE124` writer - Huffman code 124
pub type HCODE124_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN124` reader - Huffman length 124
pub type HLEN124_R = crate::FieldReader;
///Field `HLEN124` writer - Huffman length 124
pub type HLEN124_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE125` reader - Huffman code 125
pub type HCODE125_R = crate::FieldReader;
///Field `HCODE125` writer - Huffman code 125
pub type HCODE125_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN125` reader - Huffman length 125
pub type HLEN125_R = crate::FieldReader;
///Field `HLEN125` writer - Huffman length 125
pub type HLEN125_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 124
    #[inline(always)]
    pub fn hcode124(&self) -> HCODE124_R {
        HCODE124_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 124
    #[inline(always)]
    pub fn hlen124(&self) -> HLEN124_R {
        HLEN124_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 125
    #[inline(always)]
    pub fn hcode125(&self) -> HCODE125_R {
        HCODE125_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 125
    #[inline(always)]
    pub fn hlen125(&self) -> HLEN125_R {
        HLEN125_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_62")
            .field("hcode124", &self.hcode124())
            .field("hlen124", &self.hlen124())
            .field("hcode125", &self.hcode125())
            .field("hlen125", &self.hlen125())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 124
    #[inline(always)]
    pub fn hcode124(&mut self) -> HCODE124_W<'_, HUFFENC_AC0_62rs> {
        HCODE124_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 124
    #[inline(always)]
    pub fn hlen124(&mut self) -> HLEN124_W<'_, HUFFENC_AC0_62rs> {
        HLEN124_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 125
    #[inline(always)]
    pub fn hcode125(&mut self) -> HCODE125_W<'_, HUFFENC_AC0_62rs> {
        HCODE125_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 125
    #[inline(always)]
    pub fn hlen125(&mut self) -> HLEN125_W<'_, HUFFENC_AC0_62rs> {
        HLEN125_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_62::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_62::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_62)*/
pub struct HUFFENC_AC0_62rs;
impl crate::RegisterSpec for HUFFENC_AC0_62rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_62::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_62rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_62::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_62rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_62 to value 0
impl crate::Resettable for HUFFENC_AC0_62rs {}
