///Register `HUFFENC_AC0_66` reader
pub type R = crate::R<HUFFENC_AC0_66rs>;
///Register `HUFFENC_AC0_66` writer
pub type W = crate::W<HUFFENC_AC0_66rs>;
///Field `HCODE132` reader - Huffman code 132
pub type HCODE132_R = crate::FieldReader;
///Field `HCODE132` writer - Huffman code 132
pub type HCODE132_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN132` reader - Huffman length 132
pub type HLEN132_R = crate::FieldReader;
///Field `HLEN132` writer - Huffman length 132
pub type HLEN132_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE133` reader - Huffman code 133
pub type HCODE133_R = crate::FieldReader;
///Field `HCODE133` writer - Huffman code 133
pub type HCODE133_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN133` reader - Huffman length 133
pub type HLEN133_R = crate::FieldReader;
///Field `HLEN133` writer - Huffman length 133
pub type HLEN133_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 132
    #[inline(always)]
    pub fn hcode132(&self) -> HCODE132_R {
        HCODE132_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 132
    #[inline(always)]
    pub fn hlen132(&self) -> HLEN132_R {
        HLEN132_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 133
    #[inline(always)]
    pub fn hcode133(&self) -> HCODE133_R {
        HCODE133_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 133
    #[inline(always)]
    pub fn hlen133(&self) -> HLEN133_R {
        HLEN133_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_66")
            .field("hcode132", &self.hcode132())
            .field("hlen132", &self.hlen132())
            .field("hcode133", &self.hcode133())
            .field("hlen133", &self.hlen133())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 132
    #[inline(always)]
    pub fn hcode132(&mut self) -> HCODE132_W<'_, HUFFENC_AC0_66rs> {
        HCODE132_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 132
    #[inline(always)]
    pub fn hlen132(&mut self) -> HLEN132_W<'_, HUFFENC_AC0_66rs> {
        HLEN132_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 133
    #[inline(always)]
    pub fn hcode133(&mut self) -> HCODE133_W<'_, HUFFENC_AC0_66rs> {
        HCODE133_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 133
    #[inline(always)]
    pub fn hlen133(&mut self) -> HLEN133_W<'_, HUFFENC_AC0_66rs> {
        HLEN133_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_66::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_66::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFENC_AC0_66)*/
pub struct HUFFENC_AC0_66rs;
impl crate::RegisterSpec for HUFFENC_AC0_66rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_66::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_66rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_66::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_66rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_66 to value 0
impl crate::Resettable for HUFFENC_AC0_66rs {}
