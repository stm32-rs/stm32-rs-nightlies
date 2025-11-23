///Register `HUFFENC_AC0_87` reader
pub type R = crate::R<HUFFENC_AC0_87rs>;
///Register `HUFFENC_AC0_87` writer
pub type W = crate::W<HUFFENC_AC0_87rs>;
///Field `HCODE174` reader - Huffman code 174
pub type HCODE174_R = crate::FieldReader;
///Field `HCODE174` writer - Huffman code 174
pub type HCODE174_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN174` reader - Huffman length 174
pub type HLEN174_R = crate::FieldReader;
///Field `HLEN174` writer - Huffman length 174
pub type HLEN174_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE175` reader - Huffman code 175
pub type HCODE175_R = crate::FieldReader;
///Field `HCODE175` writer - Huffman code 175
pub type HCODE175_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN175` reader - Huffman length 175
pub type HLEN175_R = crate::FieldReader;
///Field `HLEN175` writer - Huffman length 175
pub type HLEN175_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 174
    #[inline(always)]
    pub fn hcode174(&self) -> HCODE174_R {
        HCODE174_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 174
    #[inline(always)]
    pub fn hlen174(&self) -> HLEN174_R {
        HLEN174_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 175
    #[inline(always)]
    pub fn hcode175(&self) -> HCODE175_R {
        HCODE175_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 175
    #[inline(always)]
    pub fn hlen175(&self) -> HLEN175_R {
        HLEN175_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_87")
            .field("hcode174", &self.hcode174())
            .field("hlen174", &self.hlen174())
            .field("hcode175", &self.hcode175())
            .field("hlen175", &self.hlen175())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 174
    #[inline(always)]
    pub fn hcode174(&mut self) -> HCODE174_W<'_, HUFFENC_AC0_87rs> {
        HCODE174_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 174
    #[inline(always)]
    pub fn hlen174(&mut self) -> HLEN174_W<'_, HUFFENC_AC0_87rs> {
        HLEN174_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 175
    #[inline(always)]
    pub fn hcode175(&mut self) -> HCODE175_W<'_, HUFFENC_AC0_87rs> {
        HCODE175_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 175
    #[inline(always)]
    pub fn hlen175(&mut self) -> HLEN175_W<'_, HUFFENC_AC0_87rs> {
        HLEN175_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_87::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_87::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFENC_AC0_87)*/
pub struct HUFFENC_AC0_87rs;
impl crate::RegisterSpec for HUFFENC_AC0_87rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_87::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_87rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_87::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_87rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_87 to value 0
impl crate::Resettable for HUFFENC_AC0_87rs {}
