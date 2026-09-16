///Register `HUFFSYMB57` reader
pub type R = crate::R<HUFFSYMB57rs>;
///Register `HUFFSYMB57` writer
pub type W = crate::W<HUFFSYMB57rs>;
///Field `DATA228` reader - Data 228
pub type DATA228_R = crate::FieldReader;
///Field `DATA228` writer - Data 228
pub type DATA228_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA229` reader - Data 229
pub type DATA229_R = crate::FieldReader;
///Field `DATA229` writer - Data 229
pub type DATA229_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA230` reader - Data 230
pub type DATA230_R = crate::FieldReader;
///Field `DATA230` writer - Data 230
pub type DATA230_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA231` reader - Data 231
pub type DATA231_R = crate::FieldReader;
///Field `DATA231` writer - Data 231
pub type DATA231_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 228
    #[inline(always)]
    pub fn data228(&self) -> DATA228_R {
        DATA228_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 229
    #[inline(always)]
    pub fn data229(&self) -> DATA229_R {
        DATA229_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 230
    #[inline(always)]
    pub fn data230(&self) -> DATA230_R {
        DATA230_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 231
    #[inline(always)]
    pub fn data231(&self) -> DATA231_R {
        DATA231_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB57")
            .field("data228", &self.data228())
            .field("data229", &self.data229())
            .field("data230", &self.data230())
            .field("data231", &self.data231())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 228
    #[inline(always)]
    pub fn data228(&mut self) -> DATA228_W<'_, HUFFSYMB57rs> {
        DATA228_W::new(self, 0)
    }
    ///Bits 8:15 - Data 229
    #[inline(always)]
    pub fn data229(&mut self) -> DATA229_W<'_, HUFFSYMB57rs> {
        DATA229_W::new(self, 8)
    }
    ///Bits 16:23 - Data 230
    #[inline(always)]
    pub fn data230(&mut self) -> DATA230_W<'_, HUFFSYMB57rs> {
        DATA230_W::new(self, 16)
    }
    ///Bits 24:31 - Data 231
    #[inline(always)]
    pub fn data231(&mut self) -> DATA231_W<'_, HUFFSYMB57rs> {
        DATA231_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb57::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb57::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFSYMB57)*/
pub struct HUFFSYMB57rs;
impl crate::RegisterSpec for HUFFSYMB57rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb57::R`](R) reader structure
impl crate::Readable for HUFFSYMB57rs {}
///`write(|w| ..)` method takes [`huffsymb57::W`](W) writer structure
impl crate::Writable for HUFFSYMB57rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB57 to value 0
impl crate::Resettable for HUFFSYMB57rs {}
