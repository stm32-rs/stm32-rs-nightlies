///Register `HUFFENC_AC0_40` reader
pub type R = crate::R<HUFFENC_AC0_40rs>;
///Register `HUFFENC_AC0_40` writer
pub type W = crate::W<HUFFENC_AC0_40rs>;
///Field `HCODE80` reader - Huffman code 80
pub type HCODE80_R = crate::FieldReader;
///Field `HCODE80` writer - Huffman code 80
pub type HCODE80_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN80` reader - Huffman length 80
pub type HLEN80_R = crate::FieldReader;
///Field `HLEN80` writer - Huffman length 80
pub type HLEN80_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE81` reader - Huffman code 81
pub type HCODE81_R = crate::FieldReader;
///Field `HCODE81` writer - Huffman code 81
pub type HCODE81_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN81` reader - Huffman length 81
pub type HLEN81_R = crate::FieldReader;
///Field `HLEN81` writer - Huffman length 81
pub type HLEN81_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 80
    #[inline(always)]
    pub fn hcode80(&self) -> HCODE80_R {
        HCODE80_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 80
    #[inline(always)]
    pub fn hlen80(&self) -> HLEN80_R {
        HLEN80_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 81
    #[inline(always)]
    pub fn hcode81(&self) -> HCODE81_R {
        HCODE81_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 81
    #[inline(always)]
    pub fn hlen81(&self) -> HLEN81_R {
        HLEN81_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_40")
            .field("hcode80", &self.hcode80())
            .field("hlen80", &self.hlen80())
            .field("hcode81", &self.hcode81())
            .field("hlen81", &self.hlen81())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 80
    #[inline(always)]
    pub fn hcode80(&mut self) -> HCODE80_W<'_, HUFFENC_AC0_40rs> {
        HCODE80_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 80
    #[inline(always)]
    pub fn hlen80(&mut self) -> HLEN80_W<'_, HUFFENC_AC0_40rs> {
        HLEN80_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 81
    #[inline(always)]
    pub fn hcode81(&mut self) -> HCODE81_W<'_, HUFFENC_AC0_40rs> {
        HCODE81_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 81
    #[inline(always)]
    pub fn hlen81(&mut self) -> HLEN81_W<'_, HUFFENC_AC0_40rs> {
        HLEN81_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFENC_AC0_40)*/
pub struct HUFFENC_AC0_40rs;
impl crate::RegisterSpec for HUFFENC_AC0_40rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_40::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_40rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_40::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_40rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_40 to value 0
impl crate::Resettable for HUFFENC_AC0_40rs {}
