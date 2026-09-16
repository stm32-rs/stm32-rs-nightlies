///Register `HUFFENC_AC0_68` reader
pub type R = crate::R<HUFFENC_AC0_68rs>;
///Register `HUFFENC_AC0_68` writer
pub type W = crate::W<HUFFENC_AC0_68rs>;
///Field `HCODE136` reader - Huffman code 136
pub type HCODE136_R = crate::FieldReader;
///Field `HCODE136` writer - Huffman code 136
pub type HCODE136_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN136` reader - Huffman length 136
pub type HLEN136_R = crate::FieldReader;
///Field `HLEN136` writer - Huffman length 136
pub type HLEN136_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE137` reader - Huffman code 137
pub type HCODE137_R = crate::FieldReader;
///Field `HCODE137` writer - Huffman code 137
pub type HCODE137_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN137` reader - Huffman length 137
pub type HLEN137_R = crate::FieldReader;
///Field `HLEN137` writer - Huffman length 137
pub type HLEN137_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 136
    #[inline(always)]
    pub fn hcode136(&self) -> HCODE136_R {
        HCODE136_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 136
    #[inline(always)]
    pub fn hlen136(&self) -> HLEN136_R {
        HLEN136_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 137
    #[inline(always)]
    pub fn hcode137(&self) -> HCODE137_R {
        HCODE137_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 137
    #[inline(always)]
    pub fn hlen137(&self) -> HLEN137_R {
        HLEN137_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_68")
            .field("hcode136", &self.hcode136())
            .field("hlen136", &self.hlen136())
            .field("hcode137", &self.hcode137())
            .field("hlen137", &self.hlen137())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 136
    #[inline(always)]
    pub fn hcode136(&mut self) -> HCODE136_W<'_, HUFFENC_AC0_68rs> {
        HCODE136_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 136
    #[inline(always)]
    pub fn hlen136(&mut self) -> HLEN136_W<'_, HUFFENC_AC0_68rs> {
        HLEN136_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 137
    #[inline(always)]
    pub fn hcode137(&mut self) -> HCODE137_W<'_, HUFFENC_AC0_68rs> {
        HCODE137_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 137
    #[inline(always)]
    pub fn hlen137(&mut self) -> HLEN137_W<'_, HUFFENC_AC0_68rs> {
        HLEN137_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_68::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_68::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_68)*/
pub struct HUFFENC_AC0_68rs;
impl crate::RegisterSpec for HUFFENC_AC0_68rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_68::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_68rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_68::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_68rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_68 to value 0
impl crate::Resettable for HUFFENC_AC0_68rs {}
