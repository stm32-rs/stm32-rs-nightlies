///Register `HUFFENC_AC0_73` reader
pub type R = crate::R<HUFFENC_AC0_73rs>;
///Register `HUFFENC_AC0_73` writer
pub type W = crate::W<HUFFENC_AC0_73rs>;
///Field `HCODE146` reader - Huffman code 146
pub type HCODE146_R = crate::FieldReader;
///Field `HCODE146` writer - Huffman code 146
pub type HCODE146_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN146` reader - Huffman length 146
pub type HLEN146_R = crate::FieldReader;
///Field `HLEN146` writer - Huffman length 146
pub type HLEN146_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE147` reader - Huffman code 147
pub type HCODE147_R = crate::FieldReader;
///Field `HCODE147` writer - Huffman code 147
pub type HCODE147_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN147` reader - Huffman length 147
pub type HLEN147_R = crate::FieldReader;
///Field `HLEN147` writer - Huffman length 147
pub type HLEN147_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 146
    #[inline(always)]
    pub fn hcode146(&self) -> HCODE146_R {
        HCODE146_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 146
    #[inline(always)]
    pub fn hlen146(&self) -> HLEN146_R {
        HLEN146_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 147
    #[inline(always)]
    pub fn hcode147(&self) -> HCODE147_R {
        HCODE147_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 147
    #[inline(always)]
    pub fn hlen147(&self) -> HLEN147_R {
        HLEN147_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_73")
            .field("hcode146", &self.hcode146())
            .field("hlen146", &self.hlen146())
            .field("hcode147", &self.hcode147())
            .field("hlen147", &self.hlen147())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 146
    #[inline(always)]
    pub fn hcode146(&mut self) -> HCODE146_W<'_, HUFFENC_AC0_73rs> {
        HCODE146_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 146
    #[inline(always)]
    pub fn hlen146(&mut self) -> HLEN146_W<'_, HUFFENC_AC0_73rs> {
        HLEN146_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 147
    #[inline(always)]
    pub fn hcode147(&mut self) -> HCODE147_W<'_, HUFFENC_AC0_73rs> {
        HCODE147_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 147
    #[inline(always)]
    pub fn hlen147(&mut self) -> HLEN147_W<'_, HUFFENC_AC0_73rs> {
        HLEN147_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_73::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_73::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFENC_AC0_73)*/
pub struct HUFFENC_AC0_73rs;
impl crate::RegisterSpec for HUFFENC_AC0_73rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_73::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_73rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_73::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_73rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_73 to value 0
impl crate::Resettable for HUFFENC_AC0_73rs {}
