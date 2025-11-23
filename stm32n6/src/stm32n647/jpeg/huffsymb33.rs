///Register `HUFFSYMB33` reader
pub type R = crate::R<HUFFSYMB33rs>;
///Register `HUFFSYMB33` writer
pub type W = crate::W<HUFFSYMB33rs>;
///Field `DATA132` reader - Data 132
pub type DATA132_R = crate::FieldReader;
///Field `DATA132` writer - Data 132
pub type DATA132_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA133` reader - Data 133
pub type DATA133_R = crate::FieldReader;
///Field `DATA133` writer - Data 133
pub type DATA133_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA134` reader - Data 134
pub type DATA134_R = crate::FieldReader;
///Field `DATA134` writer - Data 134
pub type DATA134_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA135` reader - Data 135
pub type DATA135_R = crate::FieldReader;
///Field `DATA135` writer - Data 135
pub type DATA135_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 132
    #[inline(always)]
    pub fn data132(&self) -> DATA132_R {
        DATA132_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 133
    #[inline(always)]
    pub fn data133(&self) -> DATA133_R {
        DATA133_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 134
    #[inline(always)]
    pub fn data134(&self) -> DATA134_R {
        DATA134_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 135
    #[inline(always)]
    pub fn data135(&self) -> DATA135_R {
        DATA135_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB33")
            .field("data132", &self.data132())
            .field("data133", &self.data133())
            .field("data134", &self.data134())
            .field("data135", &self.data135())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 132
    #[inline(always)]
    pub fn data132(&mut self) -> DATA132_W<'_, HUFFSYMB33rs> {
        DATA132_W::new(self, 0)
    }
    ///Bits 8:15 - Data 133
    #[inline(always)]
    pub fn data133(&mut self) -> DATA133_W<'_, HUFFSYMB33rs> {
        DATA133_W::new(self, 8)
    }
    ///Bits 16:23 - Data 134
    #[inline(always)]
    pub fn data134(&mut self) -> DATA134_W<'_, HUFFSYMB33rs> {
        DATA134_W::new(self, 16)
    }
    ///Bits 24:31 - Data 135
    #[inline(always)]
    pub fn data135(&mut self) -> DATA135_W<'_, HUFFSYMB33rs> {
        DATA135_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb33::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb33::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFSYMB33)*/
pub struct HUFFSYMB33rs;
impl crate::RegisterSpec for HUFFSYMB33rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb33::R`](R) reader structure
impl crate::Readable for HUFFSYMB33rs {}
///`write(|w| ..)` method takes [`huffsymb33::W`](W) writer structure
impl crate::Writable for HUFFSYMB33rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB33 to value 0
impl crate::Resettable for HUFFSYMB33rs {}
