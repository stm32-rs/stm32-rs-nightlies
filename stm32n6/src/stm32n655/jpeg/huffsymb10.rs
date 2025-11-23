///Register `HUFFSYMB10` reader
pub type R = crate::R<HUFFSYMB10rs>;
///Register `HUFFSYMB10` writer
pub type W = crate::W<HUFFSYMB10rs>;
///Field `DATA40` reader - Data 40
pub type DATA40_R = crate::FieldReader;
///Field `DATA40` writer - Data 40
pub type DATA40_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA41` reader - Data 41
pub type DATA41_R = crate::FieldReader;
///Field `DATA41` writer - Data 41
pub type DATA41_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA42` reader - Data 42
pub type DATA42_R = crate::FieldReader;
///Field `DATA42` writer - Data 42
pub type DATA42_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA43` reader - Data 43
pub type DATA43_R = crate::FieldReader;
///Field `DATA43` writer - Data 43
pub type DATA43_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 40
    #[inline(always)]
    pub fn data40(&self) -> DATA40_R {
        DATA40_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 41
    #[inline(always)]
    pub fn data41(&self) -> DATA41_R {
        DATA41_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 42
    #[inline(always)]
    pub fn data42(&self) -> DATA42_R {
        DATA42_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 43
    #[inline(always)]
    pub fn data43(&self) -> DATA43_R {
        DATA43_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB10")
            .field("data40", &self.data40())
            .field("data41", &self.data41())
            .field("data42", &self.data42())
            .field("data43", &self.data43())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 40
    #[inline(always)]
    pub fn data40(&mut self) -> DATA40_W<'_, HUFFSYMB10rs> {
        DATA40_W::new(self, 0)
    }
    ///Bits 8:15 - Data 41
    #[inline(always)]
    pub fn data41(&mut self) -> DATA41_W<'_, HUFFSYMB10rs> {
        DATA41_W::new(self, 8)
    }
    ///Bits 16:23 - Data 42
    #[inline(always)]
    pub fn data42(&mut self) -> DATA42_W<'_, HUFFSYMB10rs> {
        DATA42_W::new(self, 16)
    }
    ///Bits 24:31 - Data 43
    #[inline(always)]
    pub fn data43(&mut self) -> DATA43_W<'_, HUFFSYMB10rs> {
        DATA43_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB10)*/
pub struct HUFFSYMB10rs;
impl crate::RegisterSpec for HUFFSYMB10rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb10::R`](R) reader structure
impl crate::Readable for HUFFSYMB10rs {}
///`write(|w| ..)` method takes [`huffsymb10::W`](W) writer structure
impl crate::Writable for HUFFSYMB10rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB10 to value 0
impl crate::Resettable for HUFFSYMB10rs {}
